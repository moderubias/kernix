#![no_std]
//! CPU reference implementations of Kernix N-D operations.
//!
//! Operations use logical indexing, so C/F layouts, negative strides,
//! broadcasting, and borrowed views have identical mathematical semantics.
//! Results use freshly allocated C-contiguous host tensors. An accelerated
//! kernel planner can replace execution without changing the public API.

extern crate alloc;

mod ops;
pub use ops::*;
