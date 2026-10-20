//! Minimal scalar identities shared by construction and numerical capabilities.
//!
//! These traits impose no arithmetic, Clone or Default requirement. Operations
//! add their own bounds; constructors can generate owned identities directly.

/// Additive identity. Integer arithmetic retains Rust's native overflow semantics.
///
/// Implementations should return the mathematical zero for their scalar domain.
pub trait Zero {
    /// Return the additive identity for this scalar type.
    fn zero() -> Self;
}

/// Multiplicative identity, independent of storage, layout and devices.
pub trait One {
    /// Return the multiplicative identity for this scalar type.
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
