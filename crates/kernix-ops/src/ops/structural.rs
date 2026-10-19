//! Allocating structural transformations with checked dimension arithmetic.
use alloc::{vec, vec::Vec};
use core::{mem::align_of, ops::AddAssign};
use kernix_core::{layout::Order, shape::Kahen, tensor::Tensor};

use super::{
    OpsError, TensorInput, Zero, broadcast::next_c, error::flatten_build_error,
    indexed::normalized_axis,
};

/// Partition an axis into exact `lengths`. Zero-length partitions are valid;
/// `sum(lengths)` must equal the source axis length.
pub fn split_axis<T: Clone, A: TensorInput<T>>(
    source: &A,
    axis: isize,
    lengths: &[usize],
) -> Result<Vec<Tensor<T>>, OpsError> {
    let dims = source.dims();
    let axis = normalized_axis(dims.len(), axis)?;
    let mut total = 0usize;
    for &len in lengths {
        total = total
            .checked_add(len)
            .ok_or(OpsError::DimensionOverflow { axis })?;
    }
    if total != dims[axis] {
        return Err(OpsError::SplitSizesMismatch {
            axis,
            expected: dims[axis],
            actual: total,
        });
    }
    let mut start = 0usize;
    let mut result = Vec::with_capacity(lengths.len());
    let mut input_coords = vec![0; dims.len()];
    for &len in lengths {
        let mut out_dims = dims.to_vec();
        out_dims[axis] = len;
        let part =
            Tensor::<T>::try_from_fn(Kahen::new(out_dims)?, Order::C, align_of::<T>(), |coords| {
                input_coords.copy_from_slice(coords);
                input_coords[axis] += start;
                Ok::<T, OpsError>(source.get(&input_coords)?.clone())
            })
            .map_err(flatten_build_error)?;
        result.push(part);
        start += len; // prefix validated above
    }
    Ok(result)
}

/// Split into consecutive parts of at most `chunk_size` on one axis.
/// An empty axis produces no parts; `chunk_size=0` is rejected.
pub fn chunk_axis<T: Clone, A: TensorInput<T>>(
    source: &A,
    axis: isize,
    chunk_size: usize,
) -> Result<Vec<Tensor<T>>, OpsError> {
    let axis_index = normalized_axis(source.dims().len(), axis)?;
    if chunk_size == 0 {
        return Err(OpsError::InvalidParameter {
            operation: "chunk_axis",
            parameter: "chunk_size",
        });
    }
    let mut remaining = source.dims()[axis_index];
    let mut lengths = Vec::new();
    while remaining > 0 {
        let current = remaining.min(chunk_size);
        lengths.push(current);
        remaining -= current;
    }
    split_axis(source, axis, &lengths)
}

/// Tile the source across dimensions. Repeats are right-aligned; missing
/// leading repeats are 1, while excess repeat axes prepend singleton dims.
/// Unlike `repeat_axis`, an entire block is repeated.
pub fn tile<T: Clone, A: TensorInput<T>>(
    source: &A,
    repeats: &[usize],
) -> Result<Tensor<T>, OpsError> {
    let input = source.dims();
    let rank = input.len().max(repeats.len());
    let leading_shape = rank - input.len();
    let leading_reps = rank - repeats.len();
    let mut dims = Vec::with_capacity(rank);
    for i in 0..rank {
        let dim = if i < leading_shape {
            1
        } else {
            input[i - leading_shape]
        };
        let repeat = if i < leading_reps {
            1
        } else {
            repeats[i - leading_reps]
        };
        dims.push(
            dim.checked_mul(repeat)
                .ok_or(OpsError::DimensionOverflow { axis: i })?,
        );
    }
    let shape = Kahen::new(dims)?;
    let mut from = vec![0; input.len()];
    Tensor::<T>::try_from_fn(shape, Order::C, align_of::<T>(), |coords| {
        for (i, &dim) in input.iter().enumerate() {
            from[i] = coords[leading_shape + i] % dim;
        }
        Ok::<T, OpsError>(source.get(&from)?.clone())
    })
    .map_err(flatten_build_error)
}

/// Circular shift of one axis. Positive shifts move values toward increasing
/// logical indices. Works even for `isize::MIN` and dimensions > isize::MAX.
pub fn roll_axis<T: Clone, A: TensorInput<T>>(
    source: &A,
    axis: isize,
    shift: isize,
) -> Result<Tensor<T>, OpsError> {
    let dims = source.dims();
    let axis = normalized_axis(dims.len(), axis)?;
    let len = dims[axis];
    let by = if len == 0 {
        0
    } else {
        shift.unsigned_abs() % len
    };
    let mut from = vec![0; dims.len()];
    Tensor::<T>::try_from_fn(Kahen::new(dims)?, Order::C, align_of::<T>(), |coords| {
        from.copy_from_slice(coords);
        let index = coords[axis];
        from[axis] = if shift >= 0 {
            if index >= by {
                index - by
            } else {
                len - (by - index)
            }
        } else if index >= len - by {
            index - (len - by)
        } else {
            index + by
        };
        Ok::<T, OpsError>(source.get(&from)?.clone())
    })
    .map_err(flatten_build_error)
}

/// Sliding windows on one axis, with an additional window-index axis inserted
/// immediately after it. `window`, `step`, `dilation` must be positive.
/// Output dimension on the chosen axis is `1 + (dim - span) / step` when
/// `dim >= span`, otherwise 0; `span = 1 + (window-1)*dilation`.
pub fn unfold_axis<T: Clone, A: TensorInput<T>>(
    source: &A,
    axis: isize,
    window: usize,
    step: usize,
    dilation: usize,
) -> Result<Tensor<T>, OpsError> {
    let dims = source.dims();
    let axis = normalized_axis(dims.len(), axis)?;
    if window == 0 {
        return Err(OpsError::InvalidParameter {
            operation: "unfold_axis",
            parameter: "window",
        });
    }
    if step == 0 {
        return Err(OpsError::InvalidParameter {
            operation: "unfold_axis",
            parameter: "step",
        });
    }
    if dilation == 0 {
        return Err(OpsError::InvalidParameter {
            operation: "unfold_axis",
            parameter: "dilation",
        });
    }
    let span = (window - 1)
        .checked_mul(dilation)
        .and_then(|n| n.checked_add(1))
        .ok_or(OpsError::DimensionOverflow { axis })?;
    let count = if dims[axis] < span {
        0
    } else {
        1 + (dims[axis] - span) / step
    };
    let mut out_dims = dims.to_vec();
    out_dims[axis] = count;
    out_dims.insert(axis + 1, window);
    let mut from = vec![0; dims.len()];
    Tensor::<T>::try_from_fn(Kahen::new(out_dims)?, Order::C, align_of::<T>(), |coords| {
        from[..axis].copy_from_slice(&coords[..axis]);
        from[axis] = coords[axis] * step + coords[axis + 1] * dilation;
        from[axis + 1..].copy_from_slice(&coords[axis + 2..]);
        Ok::<T, OpsError>(source.get(&from)?.clone())
    })
    .map_err(flatten_build_error)
}

/// Overlap-add inverse-style folding of windows created by `unfold_axis`.
/// The axis to fold has size `number_of_windows`; the axis immediately after
/// has size `window`. Output extent is supplied explicitly. Any positions not
/// covered by windows are filled with `T::zero()`. Duplicate contributions
/// are summed in logical C-order (native arithmetic semantics).
pub fn fold_axis<T, A>(
    windows: &A,
    axis: isize,
    output_len: usize,
    step: usize,
    dilation: usize,
) -> Result<Tensor<T>, OpsError>
where
    T: Zero + Clone + AddAssign<T>,
    A: TensorInput<T>,
{
    let dims = windows.dims();
    let axis = normalized_axis(dims.len(), axis)?;
    if axis + 1 >= dims.len() {
        return Err(OpsError::InvalidRank {
            operation: "fold_axis (window dimension)",
            minimum: axis + 2,
            actual: dims.len(),
        });
    }
    if step == 0 {
        return Err(OpsError::InvalidParameter {
            operation: "fold_axis",
            parameter: "step",
        });
    }
    if dilation == 0 {
        return Err(OpsError::InvalidParameter {
            operation: "fold_axis",
            parameter: "dilation",
        });
    }
    if dims[axis + 1] == 0 {
        return Err(OpsError::InvalidParameter {
            operation: "fold_axis",
            parameter: "window",
        });
    }
    let span = (dims[axis + 1] - 1)
        .checked_mul(dilation)
        .and_then(|n| n.checked_add(1))
        .ok_or(OpsError::DimensionOverflow { axis })?;
    let count = if output_len < span {
        0
    } else {
        1 + (output_len - span) / step
    };
    if dims[axis] != count {
        return Err(OpsError::DimensionMismatch {
            axis,
            expected: count,
            actual: dims[axis],
        });
    }
    let mut out_dims = dims.to_vec();
    out_dims[axis] = output_len;
    out_dims.remove(axis + 1);
    let mut output = Tensor::filled(Kahen::new(out_dims)?, Order::C, align_of::<T>(), T::zero())?;
    let shape = Kahen::new(dims)?;
    let mut coords = vec![0; dims.len()];
    let mut dest = vec![0; dims.len() - 1];
    for _ in 0..shape.numel() {
        dest[..axis].copy_from_slice(&coords[..axis]);
        dest[axis] = coords[axis] * step + coords[axis + 1] * dilation;
        dest[axis + 1..].copy_from_slice(&coords[axis + 2..]);
        *output.get_mut(&dest)? += windows.get(&coords)?.clone();
        next_c(&mut coords, dims);
    }
    Ok(output)
}
