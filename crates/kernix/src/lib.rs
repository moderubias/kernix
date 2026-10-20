#![cfg_attr(not(feature = "std"), no_std)]
//! Kernix: type-safe N-dimensional tensor primitives for Rust.
//!
//! The facade exports the Shape, Layout, Storage and Tensor layers.
//! Portable numerical functions are available in [`ops`] and at the crate root.
//! No system BLAS or GPU runtime is required.

pub use kernix_core::{full, matrix};
pub use kernix_core::{layout, shape, storage, tensor};

pub use kernix_core::layout::{Layout, LayoutError, LayoutShape, Order};
pub use kernix_core::shape::{Kahen, Kotei, Shape, ShapeError};
pub use kernix_core::storage::{CpuBuffer, CpuStorage, Storage, StorageBuffer, StorageError};
pub use kernix_core::tensor::{Tensor, TensorBuildError, TensorError, TensorView, TensorViewMut};

/// Portable tensor operations and their numerical contracts.
pub use kernix_ops as ops;
pub use kernix_ops::*;

#[cfg(feature = "config")]
pub mod config;
pub use kernix_core::device::{self, Cpu, Device};
pub use kernix_ops::execution::{CpuExecutor, MatmulDevice};

pub use kernix_ops::{neural, vision};

extern crate alloc;
pub mod autograd;
pub mod checkpoint;
pub mod training;

pub mod nn;

pub use kernix_ops::linalg;

/// Common tensor types, literal macros and numerical operations for interactive use.
///
/// ```
/// use kernix::prelude::*;
/// let a = matrix![4.0f64, 1.0; 1.0, 3.0]?;
/// let b = tensor![1.0, 2.0]?;
/// let x = solve(&a, &b)?;
/// assert!(residual_norm(&a, &x, &b)? < 1e-14);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub mod prelude {
    pub use crate::linalg::{Cholesky, LinalgError, Lu, Qr, least_squares, residual_norm, solve};
    pub use crate::{
        Kahen, Kotei, Order, Shape, Tensor, TensorError, TensorInput, TensorView, TensorViewMut,
        add, div, full, matmul, matrix, mul, shape, sub, tensor,
    };
}
