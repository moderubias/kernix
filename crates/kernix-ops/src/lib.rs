#![cfg_attr(not(feature = "std"), no_std)]
//! CPU reference implementations of Kernix N-D operations.
//!
//! Operations use logical indexing, so C/F layouts, negative strides,
//! broadcasting, and borrowed views have identical mathematical semantics.
//! Results use freshly allocated C-contiguous host tensors. An accelerated
//! kernel planner can replace execution without changing the public API.

extern crate alloc;

mod ops;
pub use ops::*;

/// Common tensor types and traits for concise CPU-operation applications.
/// This intentionally does not glob-import hundreds of operations.
pub mod prelude {
    pub use crate::{FloatMath, TensorInput, TensorOutput};
    pub use kernix_core::{
        layout::{Layout, Order},
        shape::{Kahen, Kotei, Shape},
        tensor::{Tensor, TensorView, TensorViewMut},
    };
}

pub mod execution;

pub mod neural;
pub mod vision;

pub mod linalg;

pub use linalg::{
    Cholesky, LinalgError, LinalgFloat, Lu, Qr, determinant, inverse, least_squares, residual_norm,
    solve,
};
