use alloc::{vec, vec::Vec};
use core::{
    mem::align_of,
    ops::{AddAssign, MulAssign},
};
use kernix_core::{layout::Order, shape::Kahen, tensor::Tensor};

use super::{FloatElement, One, OpsError, TensorInput, Zero, broadcast::next_c, elementwise::map};

fn axes_mask(rank: usize, axes: &[isize]) -> Result<Vec<bool>, OpsError> {
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

fn reduced_dims(dims: &[usize], mask: &[bool], keepdims: bool) -> Vec<usize> {
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

fn project_reduction(coords: &[usize], mask: &[bool], keepdims: bool, output: &mut [usize]) {
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

fn reduce_axes<T, A, F>(
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
pub fn sum_axes<T, A>(source: &A, axes: &[isize], keepdims: bool) -> Result<Tensor<T>, OpsError>
where
    T: Zero + Clone + AddAssign<T>,
    A: TensorInput<T>,
{
    reduce_axes(source, axes, keepdims, T::zero(), |acc, val| *acc += val)
}

/// Product along selected axes. Empty reductions return multiplicative identity.
pub fn prod_axes<T, A>(source: &A, axes: &[isize], keepdims: bool) -> Result<Tensor<T>, OpsError>
where
    T: One + Clone + MulAssign<T>,
    A: TensorInput<T>,
{
    reduce_axes(source, axes, keepdims, T::one(), |acc, val| *acc *= val)
}

/// Sum of all elements, including zero for an empty tensor.
pub fn sum_all<T, A>(source: &A) -> Result<T, OpsError>
where
    T: Zero + AddAssign<T> + Clone,
    A: TensorInput<T>,
{
    let shape = Kahen::new(source.dims())?;
    let mut coords = vec![0; source.dims().len()];
    let mut acc = T::zero();
    for _ in 0..shape.numel() {
        acc += source.get(&coords)?.clone();
        next_c(&mut coords, source.dims());
    }
    Ok(acc)
}

/// Product of all elements, including one for an empty tensor.
pub fn prod_all<T, A>(source: &A) -> Result<T, OpsError>
where
    T: One + MulAssign<T> + Clone,
    A: TensorInput<T>,
{
    let shape = Kahen::new(source.dims())?;
    let mut coords = vec![0; source.dims().len()];
    let mut acc = T::one();
    for _ in 0..shape.numel() {
        acc *= source.get(&coords)?.clone();
        next_c(&mut coords, source.dims());
    }
    Ok(acc)
}

/// Minimum of all elements. `None` means empty input. Unordered values such
/// as NaN are skipped for comparisons (but an initial NaN stays the result).
pub fn min_all<T, A>(source: &A) -> Result<Option<T>, OpsError>
where
    T: PartialOrd + Clone,
    A: TensorInput<T>,
{
    extreme_all(source, |value, current| value < current)
}

/// Maximum of all elements. See `min_all` for NaN comparison semantics.
pub fn max_all<T, A>(source: &A) -> Result<Option<T>, OpsError>
where
    T: PartialOrd + Clone,
    A: TensorInput<T>,
{
    extreme_all(source, |value, current| value > current)
}

fn extreme_all<T, A, F>(source: &A, better: F) -> Result<Option<T>, OpsError>
where
    T: Clone,
    A: TensorInput<T>,
    F: Fn(&T, &T) -> bool,
{
    let shape = Kahen::new(source.dims())?;
    let mut coords = vec![0; source.dims().len()];
    let mut result: Option<T> = None;
    for _ in 0..shape.numel() {
        let value = source.get(&coords)?;
        if result.as_ref().is_none_or(|cur| better(value, cur)) {
            result = Some(value.clone());
        }
        next_c(&mut coords, source.dims());
    }
    Ok(result)
}

/// Floating mean of every element. Empty source is an explicit error.
pub fn mean_all<T, A>(source: &A) -> Result<T, OpsError>
where
    T: FloatElement + Clone + AddAssign<T>,
    A: TensorInput<T>,
{
    let shape = Kahen::new(source.dims())?;
    if shape.numel() == 0 {
        return Err(OpsError::EmptyReduction);
    }
    Ok(sum_all(source)? / T::from_count(shape.numel()))
}

/// Floating mean along axes. Empty reduced extents produce an error if the
/// output would contain elements; a zero-sized result is valid and empty.
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
    let sum = sum_axes(source, axes, keepdims)?;
    if count == 0 {
        return Ok(sum);
    } // empty output, no division performed
    map(&sum, |value| *value / T::from_count(count))
}

/// Minimum along selected axes, or `EmptyReduction` where no reduced elements
/// exist for a nonempty output. NaN comparisons follow `min_all` semantics.
pub fn min_axes<T, A>(source: &A, axes: &[isize], keepdims: bool) -> Result<Tensor<T>, OpsError>
where
    T: PartialOrd + Clone,
    A: TensorInput<T>,
{
    extreme_axes(source, axes, keepdims, |value, current| value < current)
}

/// Maximum along selected axes, with the same empty/NaN policy as `min_axes`.
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
    T: Clone,
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
        if dest.as_ref().is_none_or(|current| better(value, current)) {
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
