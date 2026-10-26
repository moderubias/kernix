//! Compressed rank-two matrices and automatic dense/CSR execution.
//!
//! Existing `Tensor` storage and numerical signatures are unchanged. [`Matrix`]
//! selects storage once at construction, then dispatches multiplication without
//! converting inputs. Automatic selection minimizes payload bytes, not a universal
//! timing estimate. Computation requires finite real scalars; exact zeros may be
//! omitted, including negative zero. No approximate pruning or dtype promotion.
//! Sparse products can fill in: use [`Matrix::matmul_with_limit`] to bound output
//! entries. All constructors/operations allocate fallibly. Sparse algebra is not
//! recorded by Tape; converting to a dense tensor is explicit.
mod kernels;
mod matrix;
mod storage;

use alloc::collections::TryReserveError;
use alloc::vec::Vec;
pub use matrix::{Matrix, MatrixStorage};
pub use storage::{CscView, CsrMatrix};

/// Checked sparse construction, geometry, resource or arithmetic failure.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum SparseError {
    /// A compressed representation violates its documented invariants.
    #[error("invalid sparse representation: {0}")]
    Invalid(&'static str),
    /// A coordinate lies outside the logical matrix.
    #[error("index {index} outside axis {axis} of size {dim}")]
    Index {
        /// Zero-based axis.
        axis: usize,
        /// Invalid coordinate.
        index: usize,
        /// Axis length.
        dim: usize,
    },
    /// Tensor construction or indexing failed.
    #[error(transparent)]
    Tensor(#[from] kernix_core::tensor::TensorError),
    /// Dense execution or shape checking failed.
    #[error(transparent)]
    Dense(#[from] crate::OpsError),
    /// A vector allocation could not be satisfied.
    #[error("sparse allocation failed: {0}")]
    Allocation(#[from] TryReserveError),
    /// A compressed extent or byte count overflowed usize.
    #[error("sparse size overflow")]
    SizeOverflow,
    /// An input or required result was not finite.
    #[error("sparse algebra requires finite inputs and results")]
    NonFinite,
    /// Sparse multiplication would exceed the caller's stored-entry budget.
    #[error("sparse product exceeds stored-entry budget {limit}")]
    FillInLimit {
        /// Maximum permitted output entries.
        limit: usize,
    },
}

fn reserve<T>(len: usize) -> Result<Vec<T>, SparseError> {
    let mut out = Vec::new();
    out.try_reserve_exact(len)?;
    Ok(out)
}
fn pointers(rows: usize) -> Result<Vec<usize>, SparseError> {
    let len = rows.checked_add(1).ok_or(SparseError::SizeOverflow)?;
    let mut out = reserve(len)?;
    out.resize(len, 0);
    Ok(out)
}
fn payload<T>(rows: usize, entries: usize) -> Result<usize, SparseError> {
    rows.checked_add(1)
        .and_then(|n| n.checked_mul(core::mem::size_of::<usize>()))
        .and_then(|n| {
            entries
                .checked_mul(core::mem::size_of::<T>())
                .and_then(|v| n.checked_add(v))
        })
        .and_then(|n| {
            entries
                .checked_mul(core::mem::size_of::<usize>())
                .and_then(|v| n.checked_add(v))
        })
        .ok_or(SparseError::SizeOverflow)
}
fn finite<T: crate::FloatMath>(values: &[T]) -> Result<(), SparseError> {
    if values.iter().all(|&v| v.is_finite()) {
        Ok(())
    } else {
        Err(SparseError::NonFinite)
    }
}
