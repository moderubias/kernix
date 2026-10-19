//! Advanced CPU indexing. Indices refer to *logical* tensor coordinates,
//! independent of the source's physical layout. Operations allocate output.
use alloc::{vec, vec::Vec};
use core::mem::align_of;
use kernix_core::{layout::Order, shape::Kahen, tensor::Tensor};

use super::{OpsError, TensorInput, broadcast::next_c, error::flatten_build_error};

pub(super) fn normalized_axis(rank: usize, axis: isize) -> Result<usize, OpsError> {
    let normalized = if axis < 0 {
        rank.checked_add_signed(axis)
    } else {
        usize::try_from(axis).ok()
    };
    normalized
        .filter(|&index| index < rank)
        .ok_or(OpsError::AxisOutOfBounds { axis, rank })
}

/// NumPy-like `take`: replace the selected source axis with all dimensions
/// of `indices`. A scalar index removes that axis. Duplicate indices are
/// allowed. Every index is validated even if other axes make output empty.
pub fn take_axis<T: Clone, A: TensorInput<T>, I: TensorInput<usize>>(
    source: &A,
    axis: isize,
    indices: &I,
) -> Result<Tensor<T>, OpsError> {
    let src_dims = source.dims();
    let axis = normalized_axis(src_dims.len(), axis)?;
    let index_shape = Kahen::new(indices.dims())?;
    let mut index_coords = vec![0; index_shape.rank()];
    for _ in 0..index_shape.numel() {
        let index = *indices.get(&index_coords)?;
        if index >= src_dims[axis] {
            return Err(OpsError::IndexOutOfBounds {
                axis,
                index,
                dim: src_dims[axis],
            });
        }
        next_c(&mut index_coords, indices.dims());
    }
    let mut output_dims = Vec::with_capacity(src_dims.len() - 1 + indices.dims().len());
    output_dims.extend_from_slice(&src_dims[..axis]);
    output_dims.extend_from_slice(indices.dims());
    output_dims.extend_from_slice(&src_dims[axis + 1..]);
    let shape = Kahen::new(output_dims)?;
    let mut src_coords = vec![0; src_dims.len()];
    let index_rank = indices.dims().len();
    Tensor::<T>::try_from_fn(shape, Order::C, align_of::<T>(), |coords| {
        src_coords[..axis].copy_from_slice(&coords[..axis]);
        src_coords[axis] = *indices.get(&coords[axis..axis + index_rank])?;
        src_coords[axis + 1..].copy_from_slice(&coords[axis + index_rank..]);
        Ok::<T, OpsError>(source.get(&src_coords)?.clone())
    })
    .map_err(flatten_build_error)
}

/// Applies `updates` at the specified positions on `axis`, returning a NEW
/// tensor. The update shape equals `source.shape` except for its selected axis,
/// whose length must equal `indices.len()`. Duplicate destination indices
/// are resolved deterministically: the last update in logical C-order wins.
pub fn scatter_axis<T: Clone, A: TensorInput<T>, U: TensorInput<T>>(
    source: &A,
    axis: isize,
    indices: &[usize],
    updates: &U,
) -> Result<Tensor<T>, OpsError> {
    let dims = source.dims();
    let axis = normalized_axis(dims.len(), axis)?;
    if updates.dims().len() != dims.len() {
        return Err(OpsError::DimensionMismatch {
            axis: dims.len(),
            expected: dims.len(),
            actual: updates.dims().len(),
        });
    }
    for (i, &dim) in dims.iter().enumerate() {
        let expected = if i == axis { indices.len() } else { dim };
        if updates.dims()[i] != expected {
            return Err(OpsError::DimensionMismatch {
                axis: i,
                expected,
                actual: updates.dims()[i],
            });
        }
    }
    for &index in indices {
        if index >= dims[axis] {
            return Err(OpsError::IndexOutOfBounds {
                axis,
                index,
                dim: dims[axis],
            });
        }
    }
    let mut output =
        Tensor::<T>::try_from_fn(Kahen::new(dims)?, Order::C, align_of::<T>(), |coords| {
            Ok::<T, OpsError>(source.get(coords)?.clone())
        })
        .map_err(flatten_build_error)?;
    let update_shape = Kahen::new(updates.dims())?;
    let mut update_coords = vec![0; dims.len()];
    let mut dest_coords = vec![0; dims.len()];
    for _ in 0..update_shape.numel() {
        dest_coords.copy_from_slice(&update_coords);
        dest_coords[axis] = indices[update_coords[axis]];
        *output.get_mut(&dest_coords)? = updates.get(&update_coords)?.clone();
        next_c(&mut update_coords, updates.dims());
    }
    Ok(output)
}

/// Flattens values selected by a broadcast-compatible boolean mask into a
/// one-dimensional C-order tensor. A mask can broadcast to the source shape;
/// it must not expand the source into a larger shape.
pub fn masked_select<T: Clone, A: TensorInput<T>, M: TensorInput<bool>>(
    source: &A,
    mask: &M,
) -> Result<Tensor<T>, OpsError> {
    let dims = source.dims();
    let rank = dims.len();
    let mdims = mask.dims();
    if mdims.len() > rank {
        return Err(OpsError::InPlaceBroadcastIncompatible);
    }
    let lead = rank - mdims.len();
    for (i, &dim) in mdims.iter().enumerate() {
        if dim != 1 && dim != dims[lead + i] {
            return Err(OpsError::BroadcastIncompatible {
                axis: lead + i,
                left: dims[lead + i],
                right: dim,
            });
        }
        // Dimension 1 can only expand to zero when the output is already zero.
    }
    let shape = Kahen::new(dims)?;
    let mut coords = vec![0; rank];
    let mut mask_coords = vec![0; mdims.len()];
    let mut values = Vec::new();
    for _ in 0..shape.numel() {
        for (j, &dim) in mdims.iter().enumerate() {
            mask_coords[j] = if dim == 1 { 0 } else { coords[lead + j] };
        }
        if *mask.get(&mask_coords)? {
            values.push(source.get(&coords)?.clone());
        }
        next_c(&mut coords, dims);
    }
    let n = values.len();
    Ok(Tensor::from_vec(values, Kahen::new([n])?, Order::C)?)
}

/// C-order coordinates of nonzero values, with output shape `[count, rank]`.
/// Scalars use rank zero, so a true scalar has shape `[1, 0]` and no data.
pub fn nonzero<T: PartialEq + super::Zero, A: TensorInput<T>>(
    source: &A,
) -> Result<Tensor<usize>, OpsError> {
    let shape = Kahen::new(source.dims())?;
    let mut coords = vec![0; shape.rank()];
    let mut result = Vec::new();
    let mut count = 0usize;
    for _ in 0..shape.numel() {
        if source.get(&coords)? != &T::zero() {
            result.extend_from_slice(&coords);
            count += 1;
        }
        next_c(&mut coords, source.dims());
    }
    Ok(Tensor::from_vec(
        result,
        Kahen::new([count, shape.rank()])?,
        Order::C,
    )?)
}
