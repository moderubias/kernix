// Kernix numerics-softmax overlay 04 (applied by tools/kernix-update-04/apply.py)
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
///
/// # Examples
///
/// ```
/// use kernix_core::{Order, shape::Kahen, tensor::Tensor};
/// use kernix_ops::matmul;
/// let mut a = Tensor::from_vec(vec![1.0,2.0,3.0,4.0], Kahen::new([2,2])?, Order::C)?;
/// let result = matmul(&a, &a)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
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
    if output.numel() == 0 {
        return Ok(Tensor::from_vec(Vec::new(), output, Order::C)?);
    }

    // Fast direct-address CPU path for ordinary row-major rank-2 tensors.
    // Accumulate k in increasing order for each output while traversing contiguous
    // columns: locality improves without reassociating each scalar sum. Arbitrary strides, batches and custom inputs
    // retain the fully general checked-indexing fallback below.
    if a.len() == 2 && b.len() == 2 {
        if let (Some(a_data), Some(b_data)) =
            (left.as_c_contiguous_slice(), right.as_c_contiguous_slice())
        {
            if m.checked_mul(k) == Some(a_data.len()) && k.checked_mul(n) == Some(b_data.len()) {
                let mut values = vec![T::zero(); output.numel()];
                for row in 0..m {
                    let out_row = &mut values[row * n..(row + 1) * n];
                    for inner in 0..k {
                        let a = &a_data[row * k + inner];
                        let b_row = &b_data[inner * n..(inner + 1) * n];
                        for (out, b) in out_row.iter_mut().zip(b_row) {
                            *out += a.clone() * b.clone();
                        }
                    }
                }
                return Ok(Tensor::from_vec(values, output, Order::C)?);
            }
        }
    }
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
