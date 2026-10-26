use super::{CscView, CsrMatrix, SparseError, finite, pointers};
use crate::{FloatMath, MatmulElement, TensorInput};
use alloc::{collections::BinaryHeap, vec::Vec};
use core::cmp::Reverse;
use kernix_core::tensor::Tensor;

pub(super) fn validate<'a, T: FloatMath, A: TensorInput<T>>(
    input: &'a A,
    dims: &[usize],
) -> Result<Option<&'a [T]>, SparseError> {
    if input.dims() != dims {
        return Err(SparseError::Invalid("dense operand shape mismatch"));
    }
    let count = if dims.contains(&0) {
        0
    } else {
        dims.iter()
            .try_fold(1usize, |p, &d| p.checked_mul(d))
            .ok_or(SparseError::SizeOverflow)?
    };
    if let Some(data) = input.as_c_contiguous_slice() {
        if data.len() != count {
            return Err(SparseError::Invalid(
                "contiguous capability length mismatch",
            ));
        }
        finite(data)?;
        return Ok(Some(data));
    }
    if count == 0 {
        return Ok(None);
    }
    if dims.len() == 1 {
        for i in 0..dims[0] {
            if !input.get(&[i])?.is_finite() {
                return Err(SparseError::NonFinite);
            }
        }
    } else {
        for i in 0..dims[0] {
            for j in 0..dims[1] {
                if !input.get(&[i, j])?.is_finite() {
                    return Err(SparseError::NonFinite);
                }
            }
        }
    }
    Ok(None)
}
impl<T: FloatMath + MatmulElement> CsrMatrix<T> {
    /// Multiply by a dense vector in O(columns + rows + stored entries), including
    /// finite-input validation. Only the owned output is allocated. Strides work.
    pub fn matvec<A: TensorInput<T>>(&self, rhs: &A) -> Result<Tensor<T>, SparseError> {
        finite(&self.values)?;
        let contiguous = validate(rhs, &[self.shape[1]])?;
        let mut out = Tensor::<T>::zeros([self.shape[0]])?;
        for (row, value) in out.storage_slice_mut().iter_mut().enumerate() {
            let mut sum = T::zero();
            for p in self.ptr[row]..self.ptr[row + 1] {
                let x = if let Some(v) = contiguous {
                    v[self.indices[p]]
                } else {
                    *rhs.get(&[self.indices[p]])?
                };
                sum += self.values[p] * x;
            }
            *value = sum;
        }
        finite(out.storage_slice())?;
        Ok(out)
    }
    /// Sparse × dense, without densifying this matrix. Returns owned C-order
    /// output. O(stored entries * output columns), plus input/output validation.
    pub fn matmul_dense<A: TensorInput<T>>(&self, rhs: &A) -> Result<Tensor<T>, SparseError> {
        spmm(self, rhs, false, true)
    }
    /// Sparse × sparse using a row-stream heap merge. Scratch is bounded by the
    /// largest left row, not the number of result columns. Accumulation visits
    /// contraction indices in increasing order. Output exact zeros are omitted.
    /// Fill-in is fallible but unlimited; use matmul_with_limit for an explicit cap.
    pub fn matmul(&self, rhs: &Self) -> Result<Self, SparseError> {
        self.matmul_with_limit(rhs, usize::MAX)
    }
    /// Multiply CSR matrices with a strict cap on stored output entries. On every
    /// failure both inputs remain unchanged. O(product contributions * log(left
    /// row length)); no dense accumulator or implicit format conversion.
    pub fn matmul_with_limit(&self, rhs: &Self, limit: usize) -> Result<Self, SparseError> {
        if self.shape[1] != rhs.shape[0] {
            return Err(SparseError::Invalid("contraction dimension mismatch"));
        }
        finite(&self.values)?;
        finite(&rhs.values)?;
        let mut ptr = pointers(self.shape[0])?;
        let mut indices = Vec::new();
        let mut values = Vec::new();
        let width = self.ptr.windows(2).map(|r| r[1] - r[0]).max().unwrap_or(0);
        let mut heap = BinaryHeap::new();
        heap.try_reserve_exact(width)?;
        for row in 0..self.shape[0] {
            for pa in self.ptr[row]..self.ptr[row + 1] {
                let k = self.indices[pa];
                let pb = rhs.ptr[k];
                let end = rhs.ptr[k + 1];
                if pb < end {
                    heap.push(Reverse((rhs.indices[pb], pa, pb, end)));
                }
            }
            let mut current = None;
            let mut sum = T::zero();
            while let Some(Reverse((col, pa, pb, end))) = heap.pop() {
                if current != Some(col) {
                    if let Some(old) = current {
                        append(&mut indices, &mut values, old, sum, limit)?;
                    }
                    current = Some(col);
                    sum = T::zero();
                }
                sum += self.values[pa] * rhs.values[pb];
                if pb + 1 < end {
                    heap.push(Reverse((rhs.indices[pb + 1], pa, pb + 1, end)));
                }
            }
            if let Some(col) = current {
                append(&mut indices, &mut values, col, sum, limit)?;
            }
            ptr[row + 1] = values.len();
        }
        Self::from_parts([self.shape[0], rhs.shape[1]], ptr, indices, values)
    }
}
fn append<T: FloatMath>(
    indices: &mut Vec<usize>,
    values: &mut Vec<T>,
    col: usize,
    value: T,
    limit: usize,
) -> Result<(), SparseError> {
    if !value.is_finite() {
        return Err(SparseError::NonFinite);
    }
    if value != T::zero() {
        if values.len() >= limit {
            return Err(SparseError::FillInLimit { limit });
        }
        indices.try_reserve(1)?;
        values.try_reserve(1)?;
        indices.push(col);
        values.push(value);
    }
    Ok(())
}
impl<T: FloatMath + MatmulElement> CscView<'_, T> {
    /// Multiply the borrowed transpose by a finite dense vector, with no input copy.
    pub fn matvec<A: TensorInput<T>>(&self, rhs: &A) -> Result<Tensor<T>, SparseError> {
        let s = self.source;
        finite(&s.values)?;
        let flat = validate(rhs, &[s.shape[0]])?;
        let mut out = Tensor::<T>::zeros([s.shape[1]])?;
        let y = out.storage_slice_mut();
        for col in 0..s.shape[0] {
            let x = if let Some(v) = flat {
                v[col]
            } else {
                *rhs.get(&[col])?
            };
            for p in s.ptr[col]..s.ptr[col + 1] {
                y[s.indices[p]] += s.values[p] * x;
            }
        }
        finite(y)?;
        Ok(out)
    }
    /// Multiply the borrowed transpose by dense RHS columns without a transpose
    /// allocation. The result is an owned C-order tensor.
    pub fn matmul_dense<A: TensorInput<T>>(&self, rhs: &A) -> Result<Tensor<T>, SparseError> {
        spmm(self.source, rhs, true, true)
    }
}

pub(super) fn spmm<T: FloatMath + MatmulElement, A: TensorInput<T>>(
    left: &CsrMatrix<T>,
    right: &A,
    transpose: bool,
    check: bool,
) -> Result<Tensor<T>, SparseError> {
    let (m, k) = if transpose {
        (left.shape[1], left.shape[0])
    } else {
        (left.shape[0], left.shape[1])
    };
    let dims = right.dims();
    if dims.len() != 2 || dims[0] != k {
        return Err(SparseError::Invalid(
            "rank-two RHS with matching contraction required",
        ));
    }
    let n = dims[1];
    let flat = if check {
        finite(&left.values)?;
        validate(right, &[k, n])?
    } else {
        right.as_c_contiguous_slice()
    };
    let mut out = Tensor::<T>::zeros([m, n])?;
    if out.numel() == 0 {
        return Ok(out);
    }
    let y = out.storage_slice_mut();
    for row in 0..left.shape[0] {
        for p in left.ptr[row]..left.ptr[row + 1] {
            let (i, j) = if transpose {
                (left.indices[p], row)
            } else {
                (row, left.indices[p])
            };
            let alpha = left.values[p];
            let target = &mut y[i * n..(i + 1) * n];
            if let Some(data) = flat {
                for (dst, &src) in target.iter_mut().zip(&data[j * n..(j + 1) * n]) {
                    *dst += alpha * src;
                }
            } else {
                for (col, dst) in target.iter_mut().enumerate() {
                    *dst += alpha * *right.get(&[j, col])?;
                }
            }
        }
    }
    finite(y)?;
    Ok(out)
}
pub(super) fn dense_sparse<T: FloatMath + MatmulElement>(
    left: &Tensor<T>,
    right: &CsrMatrix<T>,
) -> Result<Tensor<T>, SparseError> {
    let a = left.shape().dims();
    if a[1] != right.shape[0] {
        return Err(SparseError::Invalid("contraction dimension mismatch"));
    }
    let n = right.shape[1];
    let mut out = Tensor::<T>::zeros([a[0], n])?;
    if out.numel() == 0 {
        return Ok(out);
    }
    let flat = left.as_c_contiguous_slice();
    let y = out.storage_slice_mut();
    for i in 0..a[0] {
        for k in 0..a[1] {
            let alpha = if let Some(v) = flat {
                v[i * a[1] + k]
            } else {
                *left.get(&[i, k])?
            };
            for p in right.ptr[k]..right.ptr[k + 1] {
                y[i * n + right.indices[p]] += alpha * right.values[p];
            }
        }
    }
    finite(y)?;
    Ok(out)
}
