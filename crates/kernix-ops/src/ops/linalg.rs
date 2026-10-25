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
/// Contiguous f32/f64 inputs use runtime-dispatched packed CPU kernels per matrix
/// with no external BLAS requirement. Validated rank-two host views can also
/// use native strided kernels, without materializing a contiguous input tensor.
/// Floating accumulation/FMA order may differ from matmul_reference. Other
/// geometry uses the checked portable fallback. Integer scalar arithmetic retains native overflow semantics.
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
pub fn matmul<T: super::MatmulElement, A: TensorInput<T>, B: TensorInput<T>>(
    left: &A,
    right: &B,
) -> Result<Tensor<T>, OpsError> {
    matmul_impl(
        left,
        right,
        if T::HAS_NATIVE_MATMUL {
            Some((T::try_dense_matmul, T::try_strided_matmul))
        } else {
            None
        },
    )
}

/// Matrix multiplication with increasing-k scalar accumulation and no fused-multiply
/// policy changes. Supports custom Zero/Clone/AddAssign/Mul scalars and the same
/// vector/batch/stride API as matmul. Useful as a numerical reference and for exact
/// compatibility with Kernix's original operation order. O(batch*m*n*k).
///
/// ```
/// use kernix_core::{Order,shape::Kahen,tensor::Tensor};
/// let a=Tensor::from_vec(vec![1.0f64,2.0,3.0,4.0],Kahen::new([2,2])?,Order::C)?;
/// assert_eq!(kernix_ops::matmul_reference(&a,&a)?.storage_slice(),&[7.0,10.0,15.0,22.0]);
/// # Ok::<(),Box<dyn std::error::Error>>(())
/// ```
pub fn matmul_reference<T, A, B>(left: &A, right: &B) -> Result<Tensor<T>, OpsError>
where
    T: Zero + Clone + AddAssign<T> + Mul<Output = T>,
    A: TensorInput<T>,
    B: TensorInput<T>,
{
    matmul_impl(left, right, None)
}

type StridedKernel<T> = fn(super::MatrixInput<'_, T>, super::MatrixInput<'_, T>, &mut [T]) -> bool;
type DenseKernel<T> = fn(&[T], &[T], &mut [T], usize, usize, usize) -> bool;
fn matmul_impl<T, A, B>(
    left: &A,
    right: &B,
    kernel: Option<(DenseKernel<T>, StridedKernel<T>)>,
) -> Result<Tensor<T>, OpsError>
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
                let mut result = Tensor::<T>::zeros(output.dims())?;
                let values = result.storage_slice_mut();
                if !kernel.is_some_and(|kernel| kernel.0(a_data, b_data, values, m, k, n)) {
                    dense_matmul(a_data, b_data, values, m, k, n, true);
                }
                return Ok(result);
            }
        }
    }
    if a.len() == 2 && b.len() == 2 {
        if let Some((_, strided)) = kernel {
            if let (Some(a), Some(b)) = (left.as_matrix(), right.as_matrix()) {
                let mut result = Tensor::<T>::zeros(output.dims())?;
                if strided(a, b, result.storage_slice_mut()) {
                    return Ok(result);
                }
            }
        }
    }
    if let Some((dense, _)) = kernel {
        if let (Some(av), Some(bv)) = (left.as_c_contiguous_slice(), right.as_c_contiguous_slice())
        {
            // Validate advertised extents before slicing even custom TensorInput implementations.
            if av.len() == Kahen::new(a)?.numel() && bv.len() == Kahen::new(b)?.numel() {
                let mk = m
                    .checked_mul(k)
                    .ok_or(OpsError::DimensionOverflow { axis: a.len() - 1 })?;
                let kn = k
                    .checked_mul(n)
                    .ok_or(OpsError::DimensionOverflow { axis: b.len() - 1 })?;
                // Nonempty output has positive batch/m/n and a checked product.
                let mn = m * n;
                let mut result = Tensor::<T>::zeros(output.dims())?;
                for (index, out) in result.storage_slice_mut().chunks_exact_mut(mn).enumerate() {
                    let ai = batch_index(index, &batch, &a[..a_batch_rank]) * mk;
                    let bi = batch_index(index, &batch, &b[..b_batch_rank]) * kn;
                    let am = &av[ai..ai + mk];
                    let bm = &bv[bi..bi + kn];
                    if !dense(am, bm, out, m, k, n) {
                        dense_matmul(am, bm, out, m, k, n, true);
                    }
                }
                return Ok(result);
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

// Map a row-major output batch to a broadcast operand's row-major matrix number.
// Called only for nonempty output: batch dimensions are positive, and every
// operand batch subproduct is bounded by the checked output batch product.
fn batch_index(mut index: usize, batch: &[usize], operand: &[usize]) -> usize {
    let lead = batch.len() - operand.len();
    let mut output = 0;
    let mut stride = 1;
    for axis in (0..operand.len()).rev() {
        let coord = index % batch[lead + axis];
        index /= batch[lead + axis];
        if operand[axis] != 1 {
            output += coord * stride;
        }
        stride *= operand[axis];
    }
    output
}

// Four-by-four register tiles preserve increasing-k order for each output.
// Packed panels are optional so allocation-reusing execution can remain allocation-free.
pub(crate) fn dense_matmul<T>(
    a: &[T],
    b: &[T],
    out: &mut [T],
    m: usize,
    k: usize,
    n: usize,
    pack: bool,
) where
    T: Zero + Clone + AddAssign<T> + Mul<Output = T>,
{
    out.fill(T::zero());
    let rows = m / 4 * 4;
    let cols = n / 4 * 4;
    let mut panel = if pack && rows > 0 && cols > 0 {
        vec![T::zero(); k * 4]
    } else {
        Vec::new()
    };
    for col in (0..cols).step_by(4) {
        if !panel.is_empty() {
            for p in 0..k {
                panel[p * 4..p * 4 + 4].clone_from_slice(&b[p * n + col..p * n + col + 4]);
            }
        }
        for row in (0..rows).step_by(4) {
            if panel.is_empty() {
                tile::<T, false>(a, b, out, row, col, k, n);
            } else {
                tile::<T, true>(a, &panel, out, row, col, k, n);
            }
        }
    }
    for row in 0..m {
        for col in cols..n {
            let mut value = T::zero();
            for p in 0..k {
                value += a[row * k + p].clone() * b[p * n + col].clone();
            }
            out[row * n + col] = value;
        }
    }
    for row in rows..m {
        for col in 0..cols {
            let mut value = T::zero();
            for p in 0..k {
                value += a[row * k + p].clone() * b[p * n + col].clone();
            }
            out[row * n + col] = value;
        }
    }
}
#[inline]
fn tile<T, const PACKED: bool>(
    a: &[T],
    b: &[T],
    out: &mut [T],
    row: usize,
    col: usize,
    k: usize,
    n: usize,
) where
    T: Zero + Clone + AddAssign<T> + Mul<Output = T>,
{
    let arows: [&[T]; 4] = core::array::from_fn(|r| &a[(row + r) * k..(row + r + 1) * k]);
    let mut acc: [[T; 4]; 4] = core::array::from_fn(|_| core::array::from_fn(|_| T::zero()));
    for (p, _) in arows[0].iter().enumerate() {
        let start = if PACKED { p * 4 } else { p * n + col };
        let br = &b[start..start + 4];
        for r in 0..4 {
            let av = &arows[r][p];
            acc[r][0] += av.clone() * br[0].clone();
            acc[r][1] += av.clone() * br[1].clone();
            acc[r][2] += av.clone() * br[2].clone();
            acc[r][3] += av.clone() * br[3].clone();
        }
    }
    for (r, values) in acc.into_iter().enumerate() {
        out[(row + r) * n + col..(row + r) * n + col + 4].clone_from_slice(&values);
    }
}
