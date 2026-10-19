use alloc::{vec, vec::Vec};
use core::{
    mem::align_of,
    ops::{AddAssign, Mul},
};
use kernix_core::{layout::Order, shape::Kahen, tensor::Tensor};

use super::{OpsError, TensorInput, Zero, broadcast::broadcast_shape, error::flatten_build_error};

/// NumPy-style general `matmul` (including vector/matrix and batched matmul).
///
/// Rank-1 × rank-1 produces a scalar; rank-1 × rank-N and rank-N × rank-1
/// remove the corresponding singleton matrix dimension. All batch axes
/// broadcast right-aligned. Zero contracted dimension yields additive zero.
/// The implementation is the portable CPU reference O(batch*m*n*k) kernel;
/// it does not imply BLAS/SIMD acceleration.
pub fn matmul<T, A, B>(left: &A, right: &B) -> Result<Tensor<T>, OpsError>
where
    T: Zero + Clone + AddAssign<T> + Mul<Output = T>,
    A: TensorInput<T>,
    B: TensorInput<T>,
{
    let a = left.dims();
    let b = right.dims();
    if a.is_empty() {
        return Err(OpsError::InvalidRank {
            operation: "matmul",
            minimum: 1,
            actual: 0,
        });
    }
    if b.is_empty() {
        return Err(OpsError::InvalidRank {
            operation: "matmul",
            minimum: 1,
            actual: 0,
        });
    }
    let a_vector = a.len() == 1;
    let b_vector = b.len() == 1;
    let a_batch_rank = if a_vector { 0 } else { a.len() - 2 };
    let b_batch_rank = if b_vector { 0 } else { b.len() - 2 };
    let m = if a_vector { 1 } else { a[a.len() - 2] };
    let k = a[a.len() - 1];
    let kb = if b_vector { b[0] } else { b[b.len() - 2] };
    let n = if b_vector { 1 } else { b[b.len() - 1] };
    if k != kb {
        return Err(OpsError::ContractDimensionMismatch { left: k, right: kb });
    }

    let batch = broadcast_shape(&a[..a_batch_rank], &b[..b_batch_rank])?;
    let mut out_dims =
        Vec::with_capacity(batch.len() + usize::from(!a_vector) + usize::from(!b_vector));
    out_dims.extend_from_slice(&batch);
    if !a_vector {
        out_dims.push(m);
    }
    if !b_vector {
        out_dims.push(n);
    }
    let output = Kahen::new(out_dims)?;
    let batch_rank = batch.len();
    let mut ai = vec![0; a.len()];
    let mut bi = vec![0; b.len()];

    Tensor::<T>::try_from_fn(output, Order::C, align_of::<T>(), |coords| {
        for (i, &dim) in a[..a_batch_rank].iter().enumerate() {
            let out_index = coords[batch_rank - a_batch_rank + i];
            ai[i] = if dim == 1 { 0 } else { out_index };
        }
        for (i, &dim) in b[..b_batch_rank].iter().enumerate() {
            let out_index = coords[batch_rank - b_batch_rank + i];
            bi[i] = if dim == 1 { 0 } else { out_index };
        }
        let row = if a_vector { 0 } else { coords[batch_rank] };
        let col = if b_vector {
            0
        } else {
            coords[batch_rank + usize::from(!a_vector)]
        };
        if !a_vector {
            ai[a.len() - 2] = row;
        }
        if !b_vector {
            bi[b.len() - 1] = col;
        }
        let mut value = T::zero();
        for inner in 0..k {
            ai[a.len() - 1] = inner;
            if b_vector {
                bi[0] = inner;
            } else {
                bi[b.len() - 2] = inner;
            }
            value += left.get(&ai)?.clone() * right.get(&bi)?.clone();
        }
        Ok::<T, OpsError>(value)
    })
    .map_err(flatten_build_error)
}
