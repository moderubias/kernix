use super::{CsrMatrix, SparseError, finite, kernels, payload, pointers, reserve};
use crate::{FloatMath, MatmulElement, TensorInput};
use alloc::vec::Vec;
use kernix_core::tensor::Tensor;

/// Selected physical representation. Selection never changes the element dtype.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MatrixStorage {
    /// Existing affine dense CPU tensor, retaining its original layout.
    Dense,
    /// Canonical compressed sparse rows.
    Csr,
}
#[derive(Debug)]
enum Representation<T> {
    Dense(Tensor<T>),
    Csr(CsrMatrix<T>),
}
/// A finite real rank-two matrix with storage-aware arithmetic.
///
/// [`Self::from_tensor`] and [`Self::from_rows`] choose the smaller payload once.
/// Explicit [`Self::dense`] / [`Self::sparse`] constructors keep caller control.
/// Dense×dense uses existing Kernix GEMM; mixed products return dense results;
/// CSR×CSR returns CSR, with no implicit input conversion/recompression.
/// This wrapper does not alter existing Tensor APIs or record gradients.
///
/// ```
/// use kernix_ops::{Matrix, MatrixStorage};
/// let a=Matrix::from_rows([[2.0f64,0.0,0.0,0.0],[0.0,3.0,0.0,0.0]])?;
/// assert_eq!(a.storage_kind(),MatrixStorage::Csr);
/// let b=Matrix::from_rows([[1.0],[2.0],[3.0],[4.0]])?;
/// let y=a.matmul(&b)?.into_dense()?;
/// assert_eq!(y.storage_slice(),&[2.0,6.0]);
/// # Ok::<(),Box<dyn std::error::Error>>(())
/// ```
#[derive(Debug)]
pub struct Matrix<T> {
    repr: Representation<T>,
}
impl<T> Matrix<T> {
    /// Logical rank-two dimensions. No allocation occurs.
    pub fn dims(&self) -> [usize; 2] {
        match &self.repr {
            Representation::Dense(d) => [d.shape().dims()[0], d.shape().dims()[1]],
            Representation::Csr(s) => s.shape,
        }
    }
    /// Inspect the selected representation without triggering conversion.
    pub fn storage_kind(&self) -> MatrixStorage {
        match self.repr {
            Representation::Dense(_) => MatrixStorage::Dense,
            Representation::Csr(_) => MatrixStorage::Csr,
        }
    }
    /// Borrow a dense representation if already present; never densifies.
    pub fn as_dense(&self) -> Option<&Tensor<T>> {
        if let Representation::Dense(d) = &self.repr {
            Some(d)
        } else {
            None
        }
    }
    /// Borrow CSR if already present; never scans or compresses dense data.
    pub fn as_sparse(&self) -> Option<&CsrMatrix<T>> {
        if let Representation::Csr(s) = &self.repr {
            Some(s)
        } else {
            None
        }
    }
    /// Initialized backing payload bytes, excluding capacity/metadata/allocator
    /// overhead. Dense includes any retained padding; CSR includes indices.
    pub fn payload_bytes(&self) -> usize {
        match &self.repr {
            Representation::Dense(d) => core::mem::size_of_val(d.storage_slice()),
            Representation::Csr(s) => s.payload_bytes(),
        }
    }
}
impl<T: FloatMath + MatmulElement> Matrix<T> {
    /// Keep a dense tensor and its layout/backing allocation. Checks rank and
    /// finite logical values once; NaN padding outside the layout is ignored.
    pub fn dense(tensor: Tensor<T>) -> Result<Self, SparseError> {
        if tensor.rank() != 2 {
            return Err(SparseError::Invalid("Matrix requires rank two"));
        }
        kernels::validate(&tensor, tensor.shape().dims())?;
        Ok(Self {
            repr: Representation::Dense(tensor),
        })
    }
    /// Keep canonical CSR, validating finite stored values once. No data copy.
    pub fn sparse(matrix: CsrMatrix<T>) -> Result<Self, SparseError> {
        finite(&matrix.values)?;
        Ok(Self {
            repr: Representation::Csr(matrix),
        })
    }
    /// Automatically choose the smaller initialized payload. Scans exact zeros
    /// in O(rows*columns) once; if dense wins its buffer/layout is retained.
    /// Compression copies only nonzero scalars into fallibly allocated buffers.
    /// Signed zero is normalized when omitted. This is a memory policy, not a
    /// promise that sparse execution is faster at every density.
    pub fn from_tensor(tensor: Tensor<T>) -> Result<Self, SparseError> {
        if tensor.rank() != 2 {
            return Err(SparseError::Invalid("Matrix requires rank two"));
        }
        if tensor.numel() == 0 {
            return Ok(Self {
                repr: Representation::Dense(tensor),
            });
        }
        let dims = [tensor.shape().dims()[0], tensor.shape().dims()[1]];
        let nnz = inspect(&tensor, dims)?;
        if payload::<T>(dims[0], nnz)
            .is_ok_and(|bytes| bytes < core::mem::size_of_val(tensor.storage_slice()))
        {
            Ok(Self {
                repr: Representation::Csr(compress(&tensor, dims, nnz)?),
            })
        } else {
            Ok(Self {
                repr: Representation::Dense(tensor),
            })
        }
    }
    /// Construct runtime dimensions/values, then select storage once. Ownership
    /// transfers to the result; dense selection retains the original Vec payload.
    pub fn from_values(dims: [usize; 2], values: Vec<T>) -> Result<Self, SparseError> {
        Self::from_tensor(Tensor::from_values(dims, values)?)
    }
    /// Construct from natural rows, then select storage once. Allocation is
    /// fallible; unlike collecting an iterator, no hidden infallible Vec growth.
    pub fn from_rows<const R: usize, const C: usize>(
        rows: [[T; C]; R],
    ) -> Result<Self, SparseError> {
        let count = R.checked_mul(C).ok_or(SparseError::SizeOverflow)?;
        let mut values = reserve(count)?;
        for row in rows {
            values.extend(row);
        }
        Self::from_tensor(Tensor::from_values([R, C], values)?)
    }
    /// Assemble sparse COO coordinates with ordered duplicate addition. Finite
    /// validation is shared with explicit sparse construction. No densification.
    pub fn from_triplets(
        shape: [usize; 2],
        entries: Vec<(usize, usize, T)>,
    ) -> Result<Self, SparseError> {
        Self::sparse(CsrMatrix::from_triplets(shape, entries)?)
    }
    /// Multiply, dispatching once on both representations. Mixed inputs return
    /// dense output; two sparse inputs use a bounded-workspace heap merge.
    pub fn matmul(&self, rhs: &Self) -> Result<Self, SparseError> {
        self.matmul_with_limit(rhs, usize::MAX)
    }
    /// Multiply with an explicit sparse-output stored-entry limit. The limit
    /// applies to CSR×CSR only; dense output still uses checked tensor allocation.
    pub fn matmul_with_limit(&self, rhs: &Self, limit: usize) -> Result<Self, SparseError> {
        if self.dims()[1] != rhs.dims()[0] {
            return Err(SparseError::Invalid("contraction dimension mismatch"));
        }
        match (&self.repr, &rhs.repr) {
            (Representation::Dense(a), Representation::Dense(b)) => {
                Self::dense(crate::matmul(a, b)?)
            }
            (Representation::Csr(a), Representation::Dense(b)) => Ok(Self {
                repr: Representation::Dense(kernels::spmm(a, b, false, false)?),
            }),
            (Representation::Dense(a), Representation::Csr(b)) => Ok(Self {
                repr: Representation::Dense(kernels::dense_sparse(a, b)?),
            }),
            (Representation::Csr(a), Representation::Csr(b)) => Ok(Self {
                repr: Representation::Csr(a.matmul_with_limit(b, limit)?),
            }),
        }
    }
    /// Multiply by a finite dense vector. Result shape is `[rows]` in either format.
    pub fn matvec<A: TensorInput<T>>(&self, rhs: &A) -> Result<Tensor<T>, SparseError> {
        match &self.repr {
            Representation::Csr(s) => s.matvec(rhs),
            Representation::Dense(d) => {
                kernels::validate(rhs, &[self.dims()[1]])?;
                let out = crate::matmul(d, rhs)?;
                finite(out.storage_slice())?;
                Ok(out)
            }
        }
    }
    /// Explicit densification. An existing dense tensor moves out without copying;
    /// CSR allocates rows*columns and checks size/resource errors before writing.
    pub fn into_dense(self) -> Result<Tensor<T>, SparseError> {
        match self.repr {
            Representation::Dense(d) => Ok(d),
            Representation::Csr(s) => s.to_dense(),
        }
    }
    /// Explicit exact-zero compression. Existing CSR moves out without copying.
    pub fn into_csr(self) -> Result<CsrMatrix<T>, SparseError> {
        match self.repr {
            Representation::Csr(s) => Ok(s),
            Representation::Dense(d) => CsrMatrix::from_tensor(&d),
        }
    }
}
impl<T: FloatMath + MatmulElement> CsrMatrix<T> {
    /// Explicitly compress exact zeros from any checked rank-two host input.
    /// Supports fixed-rank tensors and signed-stride views without a dense copy.
    /// Two logical passes allocate only CSR buffers; negative zero is omitted.
    pub fn from_tensor<A: TensorInput<T>>(input: &A) -> Result<Self, SparseError> {
        let input_dims = input.dims();
        if input_dims.len() != 2 {
            return Err(SparseError::Invalid("CSR compression requires rank two"));
        }
        let dims = [input_dims[0], input_dims[1]];
        let nnz = inspect(input, dims)?;
        compress(input, dims, nnz)
    }
}
fn inspect<T: FloatMath, A: TensorInput<T>>(
    input: &A,
    dims: [usize; 2],
) -> Result<usize, SparseError> {
    let count = if dims.contains(&0) {
        0
    } else {
        dims[0]
            .checked_mul(dims[1])
            .ok_or(SparseError::SizeOverflow)?
    };
    let mut nnz = 0;
    if let Some(data) = input.as_c_contiguous_slice() {
        if data.len() != count {
            return Err(SparseError::Invalid(
                "contiguous capability length mismatch",
            ));
        }
        for &value in data {
            if !value.is_finite() {
                return Err(SparseError::NonFinite);
            }
            nnz += usize::from(value != T::zero());
        }
    } else if count != 0 {
        for row in 0..dims[0] {
            for col in 0..dims[1] {
                let value = *input.get(&[row, col])?;
                if !value.is_finite() {
                    return Err(SparseError::NonFinite);
                }
                nnz += usize::from(value != T::zero());
            }
        }
    }
    Ok(nnz)
}
fn compress<T: FloatMath, A: TensorInput<T>>(
    dense: &A,
    shape: [usize; 2],
    nnz: usize,
) -> Result<CsrMatrix<T>, SparseError> {
    let mut ptr = pointers(shape[0])?;
    let mut indices = reserve(nnz)?;
    let mut values = reserve(nnz)?;
    let flat = dense.as_c_contiguous_slice();
    let count = if shape.contains(&0) {
        0
    } else {
        shape[0]
            .checked_mul(shape[1])
            .ok_or(SparseError::SizeOverflow)?
    };
    if flat.is_some_and(|slice| slice.len() != count) {
        return Err(SparseError::Invalid(
            "contiguous capability length mismatch",
        ));
    }
    if !shape.contains(&0) {
        for row in 0..shape[0] {
            for col in 0..shape[1] {
                let value = if let Some(v) = flat {
                    v[row * shape[1] + col]
                } else {
                    *dense.get(&[row, col])?
                };
                if !value.is_finite() {
                    return Err(SparseError::NonFinite);
                }
                if value != T::zero() {
                    if values.len() >= nnz {
                        return Err(SparseError::Invalid("input changed during compression"));
                    }
                    indices.push(col);
                    values.push(value);
                }
            }
            ptr[row + 1] = values.len();
        }
    }
    if values.len() != nnz {
        return Err(SparseError::Invalid("input changed during compression"));
    }
    CsrMatrix::from_parts(shape, ptr, indices, values)
}
