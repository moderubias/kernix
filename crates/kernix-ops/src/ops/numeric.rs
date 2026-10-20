use core::ops::Div;

// Preserve all existing public operation-crate trait paths and implementations.
pub use kernix_core::scalar::{One, Zero};

/// Floating scalar that can divide by a reduction count. Conversion of very
/// large `usize` counts follows normal float rounding rules.
pub trait FloatElement: Zero + Copy + Div<Output = Self> {
    /// Convert a usize reduction count to this floating dtype, with normal rounding.
    fn from_count(count: usize) -> Self;
}
impl FloatElement for f32 {
    fn from_count(count: usize) -> Self {
        count as f32
    }
}
impl FloatElement for f64 {
    fn from_count(count: usize) -> Self {
        count as f64
    }
}
