// Kernix numerics-softmax overlay 04 (applied by tools/kernix-update-04/apply.py)
use alloc::{vec, vec::Vec};
use core::{
    mem::align_of,
    ops::{AddAssign, MulAssign},
};
use kernix_core::{layout::Order, shape::Kahen, tensor::Tensor};

use super::{FloatElement, One, OpsError, TensorInput, Zero, broadcast::next_c};

pub(super) fn axes_mask(rank: usize, axes: &[isize]) -> Result<Vec<bool>, OpsError> {
    let mut mask = vec![false; rank];
    for &axis in axes {
        let normalized = if axis < 0 {
            rank.checked_add_signed(axis)
        } else {
            usize::try_from(axis).ok().filter(|&a| a < rank)
        };
        let Some(axis) = normalized.filter(|&a| a < rank) else {
            return Err(OpsError::AxisOutOfBounds { axis, rank });
        };
        if mask[axis] {
            return Err(OpsError::DuplicateAxis { axis });
        }
        mask[axis] = true;
    }
    Ok(mask)
}

pub(super) fn reduced_dims(dims: &[usize], mask: &[bool], keepdims: bool) -> Vec<usize> {
    let mut out = Vec::with_capacity(dims.len());
    for (&dim, &reduce) in dims.iter().zip(mask) {
        if reduce {
            if keepdims {
                out.push(1);
            }
        } else {
            out.push(dim);
        }
    }
    out
}

pub(super) fn project_reduction(
    coords: &[usize],
    mask: &[bool],
    keepdims: bool,
    output: &mut [usize],
) {
    let mut dst = 0;
    for (axis, &reduce) in mask.iter().enumerate() {
        if reduce {
            if keepdims {
                output[dst] = 0;
                dst += 1;
            }
        } else {
            output[dst] = coords[axis];
            dst += 1;
        }
    }
}

pub(super) fn reduce_axes<T, A, F>(
    source: &A,
    axes: &[isize],
    keepdims: bool,
    identity: T,
    mut accumulate: F,
) -> Result<Tensor<T>, OpsError>
where
    T: Clone,
    A: TensorInput<T>,
    F: FnMut(&mut T, T),
{
    let input_shape = Kahen::new(source.dims())?;
    let mask = axes_mask(source.dims().len(), axes)?;
    let output_shape = Kahen::new(reduced_dims(source.dims(), &mask, keepdims))?;
    let mut output = Tensor::filled(output_shape, Order::C, align_of::<T>(), identity)?;
    let mut source_coords = vec![0; source.dims().len()];
    let mut dest_coords = vec![0; output.rank()];
    for _ in 0..input_shape.numel() {
        project_reduction(&source_coords, &mask, keepdims, &mut dest_coords);
        let value = source.get(&source_coords)?.clone();
        accumulate(output.get_mut(&dest_coords)?, value);
        next_c(&mut source_coords, source.dims());
    }
    Ok(output)
}

/// Sum along selected axes. Empty reductions return additive identity (zero).
/// Empty `axes` means no reduction, and rank-0 source is preserved.
///
/// # Examples
///
/// ```
/// use kernix_core::{Order, shape::Kahen, tensor::Tensor};
/// use kernix_ops::sum_axes;
/// let mut a = Tensor::from_vec(vec![1.0,2.0,3.0,4.0], Kahen::new([2,2])?, Order::C)?;
/// let result = sum_axes(&a, &[-1], true)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn sum_axes<T, A>(source: &A, axes: &[isize], keepdims: bool) -> Result<Tensor<T>, OpsError>
where
    T: Zero + Clone + AddAssign<T>,
    A: TensorInput<T>,
{
    reduce_axes(source, axes, keepdims, T::zero(), |acc, val| *acc += val)
}

/// Product along selected axes. Empty reductions return multiplicative identity.
///
/// # Examples
///
/// ```
/// use kernix_core::{Order, shape::Kahen, tensor::Tensor};
/// use kernix_ops::prod_axes;
/// let mut a = Tensor::from_vec(vec![1.0,2.0,3.0,4.0], Kahen::new([2,2])?, Order::C)?;
/// let result = prod_axes(&a, &[-1], true)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn prod_axes<T, A>(source: &A, axes: &[isize], keepdims: bool) -> Result<Tensor<T>, OpsError>
where
    T: One + Clone + MulAssign<T>,
    A: TensorInput<T>,
{
    reduce_axes(source, axes, keepdims, T::one(), |acc, val| *acc *= val)
}

/// Sum of all elements, including zero for an empty tensor. Contiguous inputs
/// use eight-lane pairwise summation for speed and reduced rounding-error growth.
/// Strided inputs use sequential logical order, so float last bits may differ by
/// layout. Use sum_sequential_all for fixed order or sum_compensated_all for cancellation.
///
/// # Examples
///
/// ```
/// use kernix_core::{Order, shape::Kahen, tensor::Tensor};
/// use kernix_ops::sum_all;
/// let mut a = Tensor::from_vec(vec![1.0,2.0,3.0,4.0], Kahen::new([2,2])?, Order::C)?;
/// let result = sum_all(&a)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn sum_all<T, A>(source: &A) -> Result<T, OpsError>
where
    T: Zero + AddAssign<T> + Clone,
    A: TensorInput<T>,
{
    let shape = Kahen::new(source.dims())?;
    if let Some(values) = source.as_c_contiguous_slice() {
        if values.len() == shape.numel() {
            return Ok(pairwise_sum(values));
        }
    }
    let mut coords = vec![0; source.dims().len()];
    let mut acc = T::zero();
    for _ in 0..shape.numel() {
        acc += source.get(&coords)?.clone();
        next_c(&mut coords, source.dims());
    }
    Ok(acc)
}

/// Product of all elements, including one for an empty tensor.
///
/// # Examples
///
/// ```
/// use kernix_core::{Order, shape::Kahen, tensor::Tensor};
/// use kernix_ops::prod_all;
/// let mut a = Tensor::from_vec(vec![1.0,2.0,3.0,4.0], Kahen::new([2,2])?, Order::C)?;
/// let result = prod_all(&a)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn prod_all<T, A>(source: &A) -> Result<T, OpsError>
where
    T: One + MulAssign<T> + Clone,
    A: TensorInput<T>,
{
    let shape = Kahen::new(source.dims())?;
    if let Some(values) = source.as_c_contiguous_slice() {
        if values.len() == shape.numel() {
            let mut acc = T::one();
            for value in values {
                acc *= value.clone();
            }
            return Ok(acc);
        }
    }
    let mut coords = vec![0; source.dims().len()];
    let mut acc = T::one();
    for _ in 0..shape.numel() {
        acc *= source.get(&coords)?.clone();
        next_c(&mut coords, source.dims());
    }
    Ok(acc)
}

/// Minimum of all elements. `None` means empty input. For floats, any NaN
/// propagates to the output, independent of its position in the tensor.
///
/// # Examples
///
/// ```
/// use kernix_core::{Order, shape::Kahen, tensor::Tensor};
/// use kernix_ops::min_all;
/// let mut a = Tensor::from_vec(vec![1.0,2.0,3.0,4.0], Kahen::new([2,2])?, Order::C)?;
/// let result = min_all(&a)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn min_all<T, A>(source: &A) -> Result<Option<T>, OpsError>
where
    T: PartialOrd + Clone,
    A: TensorInput<T>,
{
    extreme_all(source, |value, current| value < current)
}

/// Maximum of all elements. See `min_all` for NaN propagation.
///
/// # Examples
///
/// ```
/// use kernix_core::{Order, shape::Kahen, tensor::Tensor};
/// use kernix_ops::max_all;
/// let mut a = Tensor::from_vec(vec![1.0,2.0,3.0,4.0], Kahen::new([2,2])?, Order::C)?;
/// let result = max_all(&a)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn max_all<T, A>(source: &A) -> Result<Option<T>, OpsError>
where
    T: PartialOrd + Clone,
    A: TensorInput<T>,
{
    extreme_all(source, |value, current| value > current)
}

/// Keep the first self-unordered value (NaN for IEEE floats) and propagate
/// a newly observed unordered value. For partially ordered user types,
/// incomparable *distinct* values otherwise retain the first-wins policy.
fn choose_extreme<T: PartialOrd, F: Fn(&T, &T) -> bool>(
    candidate: &T,
    current: &T,
    better: &F,
) -> bool {
    if current.partial_cmp(current).is_none() {
        return false;
    }
    if candidate.partial_cmp(candidate).is_none() {
        return true;
    }
    better(candidate, current)
}

fn extreme_all<T, A, F>(source: &A, better: F) -> Result<Option<T>, OpsError>
where
    T: PartialOrd + Clone,
    A: TensorInput<T>,
    F: Fn(&T, &T) -> bool,
{
    let shape = Kahen::new(source.dims())?;
    let mut coords = vec![0; source.dims().len()];
    let mut result: Option<T> = None;
    for _ in 0..shape.numel() {
        let value = source.get(&coords)?;
        if result
            .as_ref()
            .is_none_or(|cur| choose_extreme(value, cur, &better))
        {
            result = Some(value.clone());
        }
        next_c(&mut coords, source.dims());
    }
    Ok(result)
}

/// Floating mean of every element. Empty source is an explicit error.
///
/// # Examples
///
/// ```
/// use kernix_core::{Order, shape::Kahen, tensor::Tensor};
/// use kernix_ops::mean_all;
/// let mut a = Tensor::from_vec(vec![1.0,2.0,3.0,4.0], Kahen::new([2,2])?, Order::C)?;
/// let result = mean_all(&a)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn mean_all<T, A>(source: &A) -> Result<T, OpsError>
where
    T: FloatElement + Clone + AddAssign<T>,
    A: TensorInput<T>,
{
    let shape = Kahen::new(source.dims())?;
    if shape.numel() == 0 {
        return Err(OpsError::EmptyReduction);
    }
    // Divide before accumulation. This avoids overflow of a large finite
    // intermediate sum when the final mean is representable. This is not a
    // compensated summation guarantee for all possible input sequences.
    let divisor = T::from_count(shape.numel());
    if let Some(values) = source.as_c_contiguous_slice() {
        if values.len() == shape.numel() {
            let mut mean = T::zero();
            for value in values {
                mean += *value / divisor;
            }
            return Ok(mean);
        }
    }
    let mut coords = vec![0; source.dims().len()];
    let mut mean = T::zero();
    for _ in 0..shape.numel() {
        mean += *source.get(&coords)? / divisor;
        next_c(&mut coords, source.dims());
    }
    Ok(mean)
}

/// Floating mean along axes. Empty reduced extents produce an error if the
/// output would contain elements; a zero-sized result is valid and empty.
///
/// # Examples
///
/// ```
/// use kernix_core::{Order, shape::Kahen, tensor::Tensor};
/// use kernix_ops::mean_axes;
/// let mut a = Tensor::from_vec(vec![1.0,2.0,3.0,4.0], Kahen::new([2,2])?, Order::C)?;
/// let result = mean_axes(&a, &[-1], true)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn mean_axes<T, A>(source: &A, axes: &[isize], keepdims: bool) -> Result<Tensor<T>, OpsError>
where
    T: FloatElement + Clone + AddAssign<T>,
    A: TensorInput<T>,
{
    let mask = axes_mask(source.dims().len(), axes)?;
    let output_shape = Kahen::new(reduced_dims(source.dims(), &mask, keepdims))?;
    let count = if source
        .dims()
        .iter()
        .zip(&mask)
        .any(|(&dim, &reduced)| reduced && dim == 0)
    {
        0 // A zero factor dominates even if the other dimensions are enormous.
    } else {
        source
            .dims()
            .iter()
            .zip(&mask)
            .filter(|(_, reduced)| **reduced)
            .try_fold(1usize, |acc, (&dim, _)| acc.checked_mul(dim))
            .ok_or(OpsError::ReductionCountOverflow)?
    };
    if count == 0 && output_shape.numel() != 0 {
        return Err(OpsError::EmptyReduction);
    }
    if count == 0 {
        // Only possible for an empty output (nonempty outputs error above).
        return sum_axes(source, axes, keepdims);
    }
    let divisor = T::from_count(count);
    reduce_axes(source, axes, keepdims, T::zero(), |acc, value| {
        *acc += value / divisor;
    })
}

/// Minimum along selected axes, or `EmptyReduction` where no reduced elements
/// exist for a nonempty output. NaNs propagate within each reduced group.
///
/// # Examples
///
/// ```
/// use kernix_core::{Order, shape::Kahen, tensor::Tensor};
/// use kernix_ops::min_axes;
/// let mut a = Tensor::from_vec(vec![1.0,2.0,3.0,4.0], Kahen::new([2,2])?, Order::C)?;
/// let result = min_axes(&a, &[-1], true)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn min_axes<T, A>(source: &A, axes: &[isize], keepdims: bool) -> Result<Tensor<T>, OpsError>
where
    T: PartialOrd + Clone,
    A: TensorInput<T>,
{
    extreme_axes(source, axes, keepdims, |value, current| value < current)
}

/// Maximum along selected axes, with the same empty/NaN policy as `min_axes`.
///
/// # Examples
///
/// ```
/// use kernix_core::{Order, shape::Kahen, tensor::Tensor};
/// use kernix_ops::max_axes;
/// let mut a = Tensor::from_vec(vec![1.0,2.0,3.0,4.0], Kahen::new([2,2])?, Order::C)?;
/// let result = max_axes(&a, &[-1], true)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn max_axes<T, A>(source: &A, axes: &[isize], keepdims: bool) -> Result<Tensor<T>, OpsError>
where
    T: PartialOrd + Clone,
    A: TensorInput<T>,
{
    extreme_axes(source, axes, keepdims, |value, current| value > current)
}

fn extreme_axes<T, A, F>(
    source: &A,
    axes: &[isize],
    keepdims: bool,
    better: F,
) -> Result<Tensor<T>, OpsError>
where
    T: PartialOrd + Clone,
    A: TensorInput<T>,
    F: Fn(&T, &T) -> bool,
{
    let shape = Kahen::new(source.dims())?;
    let mask = axes_mask(source.dims().len(), axes)?;
    let output_shape = Kahen::new(reduced_dims(source.dims(), &mask, keepdims))?;
    let mut output = Tensor::filled(
        output_shape.clone(),
        Order::C,
        align_of::<Option<T>>(),
        None::<T>,
    )?;
    let mut coords = vec![0; source.dims().len()];
    let mut dest_coords = vec![0; output.rank()];
    for _ in 0..shape.numel() {
        project_reduction(&coords, &mask, keepdims, &mut dest_coords);
        let value = source.get(&coords)?;
        let dest = output.get_mut(&dest_coords)?;
        if dest
            .as_ref()
            .is_none_or(|current| choose_extreme(value, current, &better))
        {
            *dest = Some(value.clone());
        }
        next_c(&mut coords, source.dims());
    }
    let values: Result<Vec<T>, OpsError> = output
        .into_storage_vec()
        .into_iter()
        .map(|value| value.ok_or(OpsError::EmptyReduction))
        .collect();
    Ok(Tensor::from_vec(values?, output_shape, Order::C)?)
}

fn pairwise_sum<T: Zero + AddAssign<T> + Clone>(values: &[T]) -> T {
    if values.len() > 128 {
        let middle = values.len() / 2 / 8 * 8;
        let mut result = pairwise_sum(&values[..middle]);
        result += pairwise_sum(&values[middle..]);
        return result;
    }
    let mut lanes: [T; 8] = core::array::from_fn(|_| T::zero());
    let mut chunks = values.chunks_exact(8);
    for chunk in &mut chunks {
        for (lane, value) in lanes.iter_mut().zip(chunk) {
            *lane += value.clone();
        }
    }
    for (lane, value) in lanes.iter_mut().zip(chunks.remainder()) {
        *lane += value.clone();
    }
    let mut a = lanes[0].clone();
    a += lanes[1].clone();
    let mut b = lanes[2].clone();
    b += lanes[3].clone();
    let mut c = lanes[4].clone();
    c += lanes[5].clone();
    let mut d = lanes[6].clone();
    d += lanes[7].clone();
    a += b;
    c += d;
    a += c;
    a
}

/// Sum in strictly sequential logical C order, including contiguous inputs.
/// This preserves the original accumulation order when exact last-bit compatibility
/// matters. Native scalar overflow/IEEE semantics apply. O(numel*rank), O(rank) memory.
///
/// ```
/// use kernix_core::{Order,shape::Kahen,tensor::Tensor};
/// let a=Tensor::from_vec(vec![1.0f64,2.0,3.0],Kahen::new([3])?,Order::C)?;
/// assert_eq!(kernix_ops::sum_sequential_all(&a)?,6.0);
/// # Ok::<(),Box<dyn std::error::Error>>(())
/// ```
pub fn sum_sequential_all<T: Zero + AddAssign<T> + Clone, A: TensorInput<T>>(
    source: &A,
) -> Result<T, OpsError> {
    let shape = Kahen::new(source.dims())?;
    let mut sum = T::zero();
    if let Some(data) = source.as_c_contiguous_slice() {
        if data.len() == shape.numel() {
            for value in data {
                sum += value.clone();
            }
            return Ok(sum);
        }
    }
    let mut index = vec![0; shape.rank()];
    for _ in 0..shape.numel() {
        sum += source.get(&index)?.clone();
        next_c(&mut index, shape.dims());
    }
    Ok(sum)
}
/// Neumaier-compensated floating sum in logical C order. This improves many
/// cancellation-heavy sums without changing dtype. Reject nonfinite inputs and
/// overflowing intermediate sums; this is not arbitrary precision arithmetic.
///
/// ```
/// use kernix_core::{Order,shape::Kahen,tensor::Tensor};
/// let a=Tensor::from_vec(vec![1e16f64,1.0,-1e16],Kahen::new([3])?,Order::C)?;
/// assert_eq!(kernix_ops::sum_compensated_all(&a)?,1.0);
/// # Ok::<(),Box<dyn std::error::Error>>(())
/// ```
pub fn sum_compensated_all<T: super::FloatMath, A: TensorInput<T>>(
    source: &A,
) -> Result<T, OpsError> {
    let shape = Kahen::new(source.dims())?;
    let mut sum = T::zero();
    let mut correction = T::zero();
    let mut index = vec![0; shape.rank()];
    for _ in 0..shape.numel() {
        let value = *source.get(&index)?;
        let next = sum + value;
        if !super::FloatMath::is_finite(value) || !super::FloatMath::is_finite(next) {
            return Err(OpsError::NonFiniteInput {
                operation: "sum_compensated_all",
                indices: index,
            });
        }
        correction = correction
            + if super::FloatMath::abs(sum) >= super::FloatMath::abs(value) {
                (sum - next) + value
            } else {
                (value - next) + sum
            };
        sum = next;
        next_c(&mut index, shape.dims());
    }
    let result = sum + correction;
    if !super::FloatMath::is_finite(result) {
        return Err(OpsError::NonFiniteInput {
            operation: "sum_compensated_all",
            indices: index,
        });
    }
    Ok(result)
}
