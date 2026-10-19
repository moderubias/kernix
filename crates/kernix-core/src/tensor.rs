//! Owned, validated N-D tensors and checked, zero-copy CPU views.
//!
//! `Tensor` owns Storage and Layout; operations belong in `kernix-ops`.
//! A backend implementing only `StorageBuffer` need not expose host pointers.

mod error;
mod index;
mod iter;
mod owned;
mod view;

pub use error::{TensorBuildError, TensorError};
pub use iter::TensorIter;
pub use owned::Tensor;
pub use view::{TensorView, TensorViewMut};
