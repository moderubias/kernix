#![cfg_attr(not(feature = "std"), no_std)]

pub mod layout;
pub mod shape;
extern crate alloc;

pub mod storage;

pub use layout::{Layout, LayoutError, LayoutShape, Order};

pub mod tensor;
