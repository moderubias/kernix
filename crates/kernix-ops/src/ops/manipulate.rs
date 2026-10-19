//! Allocating tensor manipulation on *logical* coordinates. These reference
//! implementations accept arbitrary valid host layouts, including F-order,
//! negative/zero strides and nonzero origins.
use alloc::{vec, vec::Vec};
use core::mem::align_of;
use kernix_core::{layout::Order, shape::Kahen, tensor::Tensor};

use super::{OpsError, TensorInput, error::flatten_build_error};

fn axis_of(rank: usize, axis: isize) -> Result<usize, OpsError> {
    rank.checked_add_signed(axis)
        .filter(|_| axis < 0)
        .or_else(|| {
            if axis >= 0 {
                usize::try_from(axis).ok()
            } else {
                None
            }
        })
        .filter(|&idx| idx < rank)
        .ok_or(OpsError::AxisOutOfBounds { axis, rank })
}

/// Concatenates one or more input tensors along `axis`, allocating fresh
/// C-contiguous storage. Other dimensions must match exactly. Inputs may
/// come from different Tensor/TensorView types via `TensorInput` trait objects.
///
/// # Examples
///
/// ```
/// use kernix_core::{Order, shape::Kahen, tensor::Tensor};
/// use kernix_ops::concat;
/// let mut a = Tensor::from_vec(vec![1.0,2.0,3.0,4.0], Kahen::new([2,2])?, Order::C)?;
/// let result = concat(&[&a,&a],0)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn concat<T: Clone>(
    inputs: &[&dyn TensorInput<T>],
    axis: isize,
) -> Result<Tensor<T>, OpsError> {
    let Some(first) = inputs.first() else {
        return Err(OpsError::EmptyInputSequence {
            operation: "concat",
        });
    };
    let rank = first.dims().len();
    let axis = axis_of(rank, axis)?;
    let mut output_dims = first.dims().to_vec();
    let mut cumulative = Vec::with_capacity(inputs.len());
    let mut end = 0usize;
    for input in inputs {
        let dims = input.dims();
        if dims.len() != rank {
            return Err(OpsError::DimensionMismatch {
                axis: rank,
                expected: rank,
                actual: dims.len(),
            });
        }
        for (i, (&actual, &expected)) in dims.iter().zip(first.dims()).enumerate() {
            if i != axis && actual != expected {
                return Err(OpsError::DimensionMismatch {
                    axis: i,
                    expected,
                    actual,
                });
            }
        }
        end = end
            .checked_add(dims[axis])
            .ok_or(OpsError::DimensionOverflow { axis })?;
        cumulative.push(end);
    }
    output_dims[axis] = end;
    let shape = Kahen::new(output_dims)?;
    let mut source_coords = vec![0; rank];
    Tensor::<T>::try_from_fn(shape, Order::C, align_of::<T>(), |coords| {
        let output_index = coords[axis];
        let part = cumulative.partition_point(|&boundary| boundary <= output_index);
        let previous = if part == 0 { 0 } else { cumulative[part - 1] };
        source_coords.copy_from_slice(coords);
        source_coords[axis] = output_index - previous;
        Ok::<T, OpsError>(inputs[part].get(&source_coords)?.clone())
    })
    .map_err(flatten_build_error)
}

/// Stacks tensors of identical shape along a *new* axis. Negative axes are
/// relative to the output rank: `-1` inserts the last axis.
///
/// # Examples
///
/// ```
/// use kernix_core::{Order, shape::Kahen, tensor::Tensor};
/// use kernix_ops::stack;
/// let mut a = Tensor::from_vec(vec![1.0,2.0,3.0,4.0], Kahen::new([2,2])?, Order::C)?;
/// let result = stack(&[&a,&a],0)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn stack<T: Clone>(inputs: &[&dyn TensorInput<T>], axis: isize) -> Result<Tensor<T>, OpsError> {
    let Some(first) = inputs.first() else {
        return Err(OpsError::EmptyInputSequence { operation: "stack" });
    };
    let rank = first.dims().len();
    let out_rank = rank
        .checked_add(1)
        .ok_or(OpsError::DimensionOverflow { axis: rank })?;
    let axis = axis_of(out_rank, axis)?;
    for input in inputs {
        if input.dims().len() != rank {
            return Err(OpsError::DimensionMismatch {
                axis: rank,
                expected: rank,
                actual: input.dims().len(),
            });
        }
        for (i, (&actual, &expected)) in input.dims().iter().zip(first.dims()).enumerate() {
            if actual != expected {
                return Err(OpsError::DimensionMismatch {
                    axis: i,
                    expected,
                    actual,
                });
            }
        }
    }
    let mut dims = first.dims().to_vec();
    dims.insert(axis, inputs.len());
    let shape = Kahen::new(dims)?;
    let mut source_coords = vec![0; rank];
    Tensor::<T>::try_from_fn(shape, Order::C, align_of::<T>(), |coords| {
        source_coords[..axis].copy_from_slice(&coords[..axis]);
        source_coords[axis..].copy_from_slice(&coords[axis + 1..]);
        Ok::<T, OpsError>(inputs[coords[axis]].get(&source_coords)?.clone())
    })
    .map_err(flatten_build_error)
}

/// Reads elements at the supplied indices on one axis, in the supplied order.
/// Duplicate indices are allowed. Bounds are checked even if the resulting
/// tensor has no elements due to a zero-sized *other* axis.
///
/// # Examples
///
/// ```
/// use kernix_core::{Order, shape::Kahen, tensor::Tensor};
/// use kernix_ops::gather_axis;
/// let mut a = Tensor::from_vec(vec![1.0,2.0,3.0,4.0], Kahen::new([2,2])?, Order::C)?;
/// let result = gather_axis(&a,-1,&[1,0])?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn gather_axis<T: Clone, A: TensorInput<T>>(
    source: &A,
    axis: isize,
    indices: &[usize],
) -> Result<Tensor<T>, OpsError> {
    let rank = source.dims().len();
    let axis = axis_of(rank, axis)?;
    let dim = source.dims()[axis];
    for &index in indices {
        if index >= dim {
            return Err(OpsError::IndexOutOfBounds { axis, index, dim });
        }
    }
    let mut dims = source.dims().to_vec();
    dims[axis] = indices.len();
    let shape = Kahen::new(dims)?;
    let mut from = vec![0; rank];
    Tensor::<T>::try_from_fn(shape, Order::C, align_of::<T>(), |coords| {
        from.copy_from_slice(coords);
        from[axis] = indices[coords[axis]];
        Ok::<T, OpsError>(source.get(&from)?.clone())
    })
    .map_err(flatten_build_error)
}

/// Repeats each element `times` along an existing axis (akin to
/// NumPy's `repeat`, not `tile`). A repeat count of zero makes that axis empty.
///
/// # Examples
///
/// ```
/// use kernix_core::{Order, shape::Kahen, tensor::Tensor};
/// use kernix_ops::repeat_axis;
/// let mut a = Tensor::from_vec(vec![1.0,2.0,3.0,4.0], Kahen::new([2,2])?, Order::C)?;
/// let result = repeat_axis(&a,0,2)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn repeat_axis<T: Clone, A: TensorInput<T>>(
    source: &A,
    axis: isize,
    times: usize,
) -> Result<Tensor<T>, OpsError> {
    let rank = source.dims().len();
    let axis = axis_of(rank, axis)?;
    let mut dims = source.dims().to_vec();
    dims[axis] = dims[axis]
        .checked_mul(times)
        .ok_or(OpsError::DimensionOverflow { axis })?;
    let shape = Kahen::new(dims)?;
    let mut from = vec![0; rank];
    Tensor::<T>::try_from_fn(shape, Order::C, align_of::<T>(), |coords| {
        from.copy_from_slice(coords);
        // If times == 0 there are no output elements; the closure is not called.
        from[axis] = coords[axis] / times;
        Ok::<T, OpsError>(source.get(&from)?.clone())
    })
    .map_err(flatten_build_error)
}

/// Constant padding on each axis: `[(before, after), ...]` with one entry per
/// axis. Rank-zero (scalar) padding accepts only an empty `padding` list.
///
/// # Examples
///
/// ```
/// use kernix_core::{Order, shape::Kahen, tensor::Tensor};
/// use kernix_ops::pad_constant;
/// let mut a = Tensor::from_vec(vec![1.0,2.0,3.0,4.0], Kahen::new([2,2])?, Order::C)?;
/// let result = pad_constant(&a,&[(1,1),(0,0)],0.0)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn pad_constant<T: Clone, A: TensorInput<T>>(
    source: &A,
    padding: &[(usize, usize)],
    value: T,
) -> Result<Tensor<T>, OpsError> {
    let dims = source.dims();
    if padding.len() != dims.len() {
        return Err(OpsError::PaddingRankMismatch {
            expected: dims.len(),
            actual: padding.len(),
        });
    }
    let mut new_dims = Vec::with_capacity(dims.len());
    for (axis, (&dim, &(before, after))) in dims.iter().zip(padding).enumerate() {
        let total = dim
            .checked_add(before)
            .and_then(|v| v.checked_add(after))
            .ok_or(OpsError::DimensionOverflow { axis })?;
        new_dims.push(total);
    }
    let shape = Kahen::new(new_dims)?;
    let mut from = vec![0; dims.len()];
    Tensor::<T>::try_from_fn(shape, Order::C, align_of::<T>(), |coords| {
        for axis in 0..dims.len() {
            let (before, _) = padding[axis];
            if coords[axis] < before || coords[axis] - before >= dims[axis] {
                return Ok::<T, OpsError>(value.clone());
            }
            from[axis] = coords[axis] - before;
        }
        Ok::<T, OpsError>(source.get(&from)?.clone())
    })
    .map_err(flatten_build_error)
}
