#![cfg_attr(not(feature = "std"), no_std)]
//! Kernix: type-safe N-dimensional tensor primitives for Rust.
//!
//! The facade exports the Shape, Layout, Storage and Tensor layers.
//! CPU numerical operations currently live in the sibling `kernix-ops` crate;
//! they are not silently coupled to a specific backend through this facade.

pub use kernix_core::{layout, shape, storage, tensor};

pub use kernix_core::layout::{Layout, LayoutError, LayoutShape, Order};
pub use kernix_core::shape::{Kahen, Kotei, Shape, ShapeError};
pub use kernix_core::storage::{CpuBuffer, CpuStorage, Storage, StorageBuffer, StorageError};
pub use kernix_core::tensor::{Tensor, TensorBuildError, TensorError, TensorView, TensorViewMut};
