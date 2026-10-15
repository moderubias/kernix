mod axis;
mod broadcast;
mod error;
mod kahen;
mod kotei;
mod traits;
mod utils;

pub use error::ShapeError;
pub use kahen::Kahen;
pub use kotei::Kotei;
pub use traits::Shape;

pub(crate) const INLINE_RANK: usize = 6;
