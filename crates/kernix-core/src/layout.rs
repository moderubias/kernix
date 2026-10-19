//! Validated, device-independent affine tensor layouts.

mod affine;
mod error;
mod geometry;
mod order;
mod traits;

pub use affine::Layout;
pub use error::LayoutError;
pub use geometry::{Overlap, StorageSpan};
pub use order::Order;
pub use traits::LayoutShape;
