#![cfg_attr(not(feature = "std"), no_std)]
//! Kernix: type-safe N-dimensional tensor primitives for Rust.
//!
//! The facade exports the Shape, Layout, Storage and Tensor layers.
//! Portable numerical functions are available in [`ops`] and at the crate root.
//! No system BLAS or GPU runtime is required.

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
