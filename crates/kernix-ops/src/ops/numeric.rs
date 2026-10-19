use core::ops::Div;

/// Additive identity. Semantics of integer overflow follow Rust's native operators.
pub trait Zero {
    fn zero() -> Self;
}

/// Multiplicative identity.
pub trait One {
    fn one() -> Self;
}

macro_rules! identities {
    ($($t:ty),+ $(,)?) => {$(
        impl Zero for $t { fn zero() -> Self { 0 as $t } }
        impl One for $t { fn one() -> Self { 1 as $t } }
    )+};
}
identities!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64
);

/// Floating scalar that can divide by a reduction count. Conversion of very
/// large `usize` counts follows normal float rounding rules.
pub trait FloatElement: Zero + Copy + Div<Output = Self> {
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
