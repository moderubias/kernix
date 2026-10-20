//! Validated shapes, affine layouts, opaque storage and checked tensor views.
//!
//! See [`mod@shape`] for pure geometry and [`mod@tensor`] for owned data and borrowed views.
#![cfg_attr(not(feature = "std"), no_std)]

pub mod layout;
/// Pure shape geometry, independent of storage, strides and devices.
pub mod shape;
extern crate alloc;

// Macro expansion must also work when downstream crates do not import alloc.
#[doc(hidden)]
pub mod __private {
    pub use alloc::vec;
}

pub mod storage;

pub use layout::{Layout, LayoutError, LayoutShape, Order};

pub mod tensor;

pub mod device;
