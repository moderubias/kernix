use super::OpsError;
use alloc::vec::Vec;

/// NumPy-compatible right-aligned shape broadcasting.
/// In particular 0 with 1 produces 0, not 1.
pub(crate) fn broadcast_shape(left: &[usize], right: &[usize]) -> Result<Vec<usize>, OpsError> {
    let rank = left.len().max(right.len());
    let mut result = Vec::with_capacity(rank);
    for axis in 0..rank {
        let a = if axis + left.len() >= rank {
            left[axis + left.len() - rank]
        } else {
            1
        };
        let b = if axis + right.len() >= rank {
            right[axis + right.len() - rank]
        } else {
            1
        };
        let dim = if a == b || b == 1 {
            a
        } else if a == 1 {
            b
        } else {
            return Err(OpsError::BroadcastIncompatible {
                axis,
                left: a,
                right: b,
            });
        };
        result.push(dim);
    }
    Ok(result)
}

/// Populate a source coordinate buffer from a full broadcasted index.
pub(crate) fn project_broadcast(out: &[usize], source_dims: &[usize], coords: &mut [usize]) {
    debug_assert!(out.len() >= source_dims.len());
    debug_assert_eq!(source_dims.len(), coords.len());
    let offset = out.len() - source_dims.len();
    for (i, &dim) in source_dims.iter().enumerate() {
        coords[i] = if dim == 1 { 0 } else { out[offset + i] };
    }
}

/// Advance coordinates in logical C-order. Never invoked on a zero dimension.
pub(crate) fn next_c(coords: &mut [usize], dims: &[usize]) {
    for (coord, &dim) in coords.iter_mut().zip(dims).rev() {
        *coord += 1;
        if *coord < dim {
            return;
        }
        *coord = 0;
    }
}
