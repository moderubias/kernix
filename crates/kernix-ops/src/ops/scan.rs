//! Inclusive prefix scans across one logical axis.
use alloc::vec;
use core::{
    mem::align_of,
    ops::{AddAssign, MulAssign},
};
use kernix_core::{layout::Order, shape::Kahen, tensor::Tensor};

use super::{One, OpsError, TensorInput, Zero, broadcast::next_c};

fn normalize_axis(rank: usize, axis: isize) -> Result<usize, OpsError> {
    let result = if axis < 0 {
        rank.checked_add_signed(axis)
    } else {
        usize::try_from(axis).ok()
    };
    result
        .filter(|&a| a < rank)
        .ok_or(OpsError::AxisOutOfBounds { axis, rank })
}

fn scan<T, A, F>(
    source: &A,
    axis: isize,
    identity: T,
    mut combine: F,
) -> Result<Tensor<T>, OpsError>
where
    T: Clone,
    A: TensorInput<T>,
    F: FnMut(&mut T, T),
{
    let dims = source.dims();
    let axis = normalize_axis(dims.len(), axis)?;
    let shape = Kahen::new(dims)?;
    let mut output = Tensor::filled(shape, Order::C, align_of::<T>(), identity.clone())?;
    if output.shape().numel() == 0 {
        return Ok(output);
    }
    let mut outer_dims = dims.to_vec();
    outer_dims.remove(axis);
    let outer_shape = Kahen::new(&outer_dims)?;
    let mut outer = vec![0; outer_dims.len()];
    let mut index = vec![0; dims.len()];
    for _ in 0..outer_shape.numel() {
        index[..axis].copy_from_slice(&outer[..axis]);
        index[axis + 1..].copy_from_slice(&outer[axis..]);
        let mut accumulator = identity.clone();
        for position in 0..dims[axis] {
            index[axis] = position;
            combine(&mut accumulator, source.get(&index)?.clone());
            *output.get_mut(&index)? = accumulator.clone();
        }
        next_c(&mut outer, &outer_dims);
    }
    Ok(output)
}

/// Inclusive cumulative sum on one axis, in forward logical index order.
///
/// # Examples
///
/// ```
/// use kernix_core::{Order, shape::Kahen, tensor::Tensor};
/// use kernix_ops::cumsum_axis;
/// let mut a = Tensor::from_vec(vec![1.0,2.0,3.0,4.0], Kahen::new([2,2])?, Order::C)?;
/// let result = cumsum_axis(&a, -1)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn cumsum_axis<T, A>(source: &A, axis: isize) -> Result<Tensor<T>, OpsError>
where
    T: Zero + Clone + AddAssign<T>,
    A: TensorInput<T>,
{
    scan(source, axis, T::zero(), |acc, value| *acc += value)
}

/// Inclusive cumulative product on one axis, in forward logical index order.
///
/// # Examples
///
/// ```
/// use kernix_core::{Order, shape::Kahen, tensor::Tensor};
/// use kernix_ops::cumprod_axis;
/// let mut a = Tensor::from_vec(vec![1.0,2.0,3.0,4.0], Kahen::new([2,2])?, Order::C)?;
/// let result = cumprod_axis(&a, -1)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn cumprod_axis<T, A>(source: &A, axis: isize) -> Result<Tensor<T>, OpsError>
where
    T: One + Clone + MulAssign<T>,
    A: TensorInput<T>,
{
    scan(source, axis, T::one(), |acc, value| *acc *= value)
}
