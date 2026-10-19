//! Explicit overflow-checked integer arithmetic. Native `add`, `mul`, etc.
//! retain Rust's normal operator semantics; these functions never wrap or
//! panic on division by zero or signed MIN / -1 overflow.
use kernix_core::tensor::Tensor;

use super::{
    OpsError, TensorInput,
    elementwise::{try_map_indexed, try_zip_map_indexed},
};

/// Checked arithmetic contract for primitive signed and unsigned integers.
///
/// `None` denotes overflow, division by zero, or a result that is not
/// representable in the integer type.
pub trait CheckedInteger: Copy {
    /// Return checked integer addition; None denotes overflow.
    fn checked_add(self, rhs: Self) -> Option<Self>;
    /// Return checked integer subtraction; None denotes overflow or unsigned underflow.
    fn checked_sub(self, rhs: Self) -> Option<Self>;
    /// Return checked integer multiplication; None denotes overflow.
    fn checked_mul(self, rhs: Self) -> Option<Self>;
    /// Return checked integer division; None denotes zero divisor or signed MIN / -1.
    fn checked_div(self, rhs: Self) -> Option<Self>;
    /// Return checked integer remainder; None denotes zero divisor or signed MIN % -1.
    fn checked_rem(self, rhs: Self) -> Option<Self>;
    /// Return checked integer negation; None denotes an unrepresentable result.
    fn checked_neg(self) -> Option<Self>;
}

macro_rules! impl_checked_integer {
    ($($ty:ty),+ $(,)?) => {
        $(
        impl CheckedInteger for $ty {
            fn checked_add(self, rhs: Self) -> Option<Self> { <$ty>::checked_add(self, rhs) }
            fn checked_sub(self, rhs: Self) -> Option<Self> { <$ty>::checked_sub(self, rhs) }
            fn checked_mul(self, rhs: Self) -> Option<Self> { <$ty>::checked_mul(self, rhs) }
            fn checked_div(self, rhs: Self) -> Option<Self> { <$ty>::checked_div(self, rhs) }
            fn checked_rem(self, rhs: Self) -> Option<Self> { <$ty>::checked_rem(self, rhs) }
            fn checked_neg(self) -> Option<Self> { <$ty>::checked_neg(self) }
        }
        )+
    };
}
impl_checked_integer!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize
);

macro_rules! checked_binary {
    ($name:ident, $method:ident, $label:literal) => {
        /// Computes checked integer arithmetic using NumPy broadcasting.
        /// Failing logical coordinates are available in `OpsError::Arithmetic`.
        pub fn $name<T, A, B>(left: &A, right: &B) -> Result<Tensor<T>, OpsError>
        where
            T: CheckedInteger,
            A: TensorInput<T>,
            B: TensorInput<T>,
        {
            try_zip_map_indexed(left, right, |a, b, coords| {
                (*a).$method(*b).ok_or_else(|| OpsError::Arithmetic {
                    operation: $label,
                    indices: coords.to_vec(),
                })
            })
        }
    };
}

checked_binary!(checked_add, checked_add, "add");
checked_binary!(checked_sub, checked_sub, "sub");
checked_binary!(checked_mul, checked_mul, "mul");
checked_binary!(checked_div, checked_div, "div");
checked_binary!(checked_rem, checked_rem, "rem");

/// Checked integer negation. For unsigned integers, only zero has a valid
/// negation. For signed integers, `MIN` is an overflow.
///
/// # Examples
///
/// ```
/// use kernix_core::{Order, shape::Kahen, tensor::Tensor};
/// use kernix_ops::checked_neg;
/// let mut a = Tensor::from_vec(vec![1,2,3,4], Kahen::new([2,2])?, Order::C)?;
/// let result = checked_neg(&a)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn checked_neg<T, A>(source: &A) -> Result<Tensor<T>, OpsError>
where
    T: CheckedInteger,
    A: TensorInput<T>,
{
    try_map_indexed(source, |value, coords| {
        (*value).checked_neg().ok_or_else(|| OpsError::Arithmetic {
            operation: "neg",
            indices: coords.to_vec(),
        })
    })
}
