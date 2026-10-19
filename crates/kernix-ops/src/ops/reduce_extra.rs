//! Boolean reductions, nonzero counts and indices of extrema.
use alloc::vec;
use core::mem::align_of;
use kernix_core::{layout::Order, shape::Kahen, tensor::Tensor};

use super::{
    OpsError, TensorInput, Zero,
    broadcast::next_c,
    elementwise::map,
    error::flatten_build_error,
    reduce::{reduce_axes, sum_axes},
};

pub fn any_all<A: TensorInput<bool>>(source: &A) -> Result<bool, OpsError> {
    let shape = Kahen::new(source.dims())?;
    let mut coords = vec![0; source.dims().len()];
    for _ in 0..shape.numel() {
        if *source.get(&coords)? {
            return Ok(true);
        }
        next_c(&mut coords, source.dims());
    }
    Ok(false)
}

pub fn all_all<A: TensorInput<bool>>(source: &A) -> Result<bool, OpsError> {
    let shape = Kahen::new(source.dims())?;
    let mut coords = vec![0; source.dims().len()];
    for _ in 0..shape.numel() {
        if !*source.get(&coords)? {
            return Ok(false);
        }
        next_c(&mut coords, source.dims());
    }
    Ok(true)
}

/// Logical OR over `axes`. `false` is the identity for an empty reduction.
pub fn any_axes<A: TensorInput<bool>>(
    source: &A,
    axes: &[isize],
    keepdims: bool,
) -> Result<Tensor<bool>, OpsError> {
    reduce_axes(source, axes, keepdims, false, |acc, value| *acc |= value)
}

/// Logical AND over `axes`. `true` is the identity for an empty reduction.
pub fn all_axes<A: TensorInput<bool>>(
    source: &A,
    axes: &[isize],
    keepdims: bool,
) -> Result<Tensor<bool>, OpsError> {
    reduce_axes(source, axes, keepdims, true, |acc, value| *acc &= value)
}

/// Number of nonzero logical elements. NaN counts as nonzero for floats.
pub fn count_nonzero_all<T: Zero + PartialEq, A: TensorInput<T>>(
    source: &A,
) -> Result<usize, OpsError> {
    let shape = Kahen::new(source.dims())?;
    let mut coords = vec![0; source.dims().len()];
    let mut count = 0;
    for _ in 0..shape.numel() {
        if source.get(&coords)? != &T::zero() {
            count += 1;
        }
        next_c(&mut coords, source.dims());
    }
    Ok(count)
}

pub fn count_nonzero_axes<T: Zero + PartialEq, A: TensorInput<T>>(
    source: &A,
    axes: &[isize],
    keepdims: bool,
) -> Result<Tensor<usize>, OpsError> {
    let indicators = map(source, |value| usize::from(value != &T::zero()))?;
    sum_axes(&indicators, axes, keepdims)
}

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

/// Match the extrema policy in `reduce.rs`: an IEEE NaN propagates and the
/// first encountered NaN wins; equal finite values keep their first index.
fn preferable<T: PartialOrd>(candidate: &T, current: &T, minimum: bool) -> bool {
    if current.partial_cmp(current).is_none() {
        return false;
    }
    if candidate.partial_cmp(candidate).is_none() {
        return true;
    }
    if minimum {
        candidate < current
    } else {
        candidate > current
    }
}

fn arg_extreme_all<T: PartialOrd, A: TensorInput<T>>(
    source: &A,
    minimum: bool,
) -> Result<Option<usize>, OpsError> {
    let shape = Kahen::new(source.dims())?;
    let mut coords = vec![0; source.dims().len()];
    let mut best: Option<(&T, usize)> = None;
    for index in 0..shape.numel() {
        let candidate = source.get(&coords)?;
        if best.is_none_or(|(current, _)| preferable(candidate, current, minimum)) {
            best = Some((candidate, index));
        }
        next_c(&mut coords, source.dims());
    }
    Ok(best.map(|(_, index)| index))
}

/// Flat C-order index of the first minimum; None for an empty tensor.
pub fn argmin_all<T: PartialOrd, A: TensorInput<T>>(source: &A) -> Result<Option<usize>, OpsError> {
    arg_extreme_all(source, true)
}

/// Flat C-order index of the first maximum; None for an empty tensor.
pub fn argmax_all<T: PartialOrd, A: TensorInput<T>>(source: &A) -> Result<Option<usize>, OpsError> {
    arg_extreme_all(source, false)
}

fn arg_extreme_axis<T: PartialOrd, A: TensorInput<T>>(
    source: &A,
    axis: isize,
    keepdims: bool,
    minimum: bool,
) -> Result<Tensor<usize>, OpsError> {
    let dims = source.dims();
    let axis = normalize_axis(dims.len(), axis)?;
    let mut out_dims = dims.to_vec();
    if keepdims {
        out_dims[axis] = 1;
    } else {
        out_dims.remove(axis);
    }
    let output_shape = Kahen::new(out_dims)?;
    if dims[axis] == 0 && output_shape.numel() != 0 {
        return Err(OpsError::EmptyReduction);
    }
    let mut from = vec![0; dims.len()];
    Tensor::<usize>::try_from_fn(output_shape, Order::C, align_of::<usize>(), |coords| {
        for i in 0..dims.len() {
            from[i] = if i == axis {
                0
            } else if keepdims || i < axis {
                coords[i]
            } else {
                coords[i - 1]
            };
        }
        let mut best_value: Option<&T> = None;
        let mut best_index = 0;
        for index in 0..dims[axis] {
            from[axis] = index;
            let candidate = source.get(&from)?;
            if best_value.is_none_or(|current| preferable(candidate, current, minimum)) {
                best_value = Some(candidate);
                best_index = index;
            }
        }
        Ok::<usize, OpsError>(best_index)
    })
    .map_err(flatten_build_error)
}

/// Argmin along one axis. NaNs and ties use first logical index semantics.
pub fn argmin_axis<T: PartialOrd, A: TensorInput<T>>(
    source: &A,
    axis: isize,
    keepdims: bool,
) -> Result<Tensor<usize>, OpsError> {
    arg_extreme_axis(source, axis, keepdims, true)
}

pub fn argmax_axis<T: PartialOrd, A: TensorInput<T>>(
    source: &A,
    axis: isize,
    keepdims: bool,
) -> Result<Tensor<usize>, OpsError> {
    arg_extreme_axis(source, axis, keepdims, false)
}
