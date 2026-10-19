//! Validated shapes, affine layouts, opaque storage and checked tensor views.
//!
//! See [`shape`] for pure geometry and [`tensor`] for owned data and borrowed views.
#![cfg_attr(not(feature = "std"), no_std)]

pub mod layout;
/// Pure shape geometry, independent of storage, strides and devices.
pub mod shape;
extern crate alloc;

pub mod storage;

pub use layout::{Layout, LayoutError, LayoutShape, Order};

pub mod tensor;

pub mod device;
