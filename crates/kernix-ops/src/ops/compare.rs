//! Elementwise comparisons, Boolean algebra, and conditional selection.
//! All binary/ternary operations obey right-aligned NumPy broadcasting.
use alloc::vec;
use core::mem::align_of;
use kernix_core::{layout::Order, shape::Kahen, tensor::Tensor};

use super::{
    OpsError, TensorInput,
    broadcast::{broadcast_shape, project_broadcast},
    elementwise::{map, zip_map},
    error::flatten_build_error,
};

/// Elementwise equality (including ordinary IEEE NaN != NaN semantics).
///
/// # Examples
///
/// ```
/// use kernix_core::{Order, shape::Kahen, tensor::Tensor};
/// use kernix_ops::equal;
/// let mut a = Tensor::from_vec(vec![1.0,2.0,3.0,4.0], Kahen::new([2,2])?, Order::C)?;
/// let result = equal(&a, &a)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn equal<T: PartialEq, A: TensorInput<T>, B: TensorInput<T>>(
    left: &A,
    right: &B,
) -> Result<Tensor<bool>, OpsError> {
    zip_map(left, right, |a, b| a == b)
}

/// Elementwise inequality.
///
/// # Examples
///
/// ```
/// use kernix_core::{Order, shape::Kahen, tensor::Tensor};
/// use kernix_ops::not_equal;
/// let mut a = Tensor::from_vec(vec![1.0,2.0,3.0,4.0], Kahen::new([2,2])?, Order::C)?;
/// let result = not_equal(&a, &a)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn not_equal<T: PartialEq, A: TensorInput<T>, B: TensorInput<T>>(
    left: &A,
    right: &B,
) -> Result<Tensor<bool>, OpsError> {
    zip_map(left, right, |a, b| a != b)
}

macro_rules! comparison {
    ($name:ident, $op:tt) => {
        /// Allocate an elementwise ordered comparison with trailing-axis broadcasting.
        /// IEEE comparisons with NaNs are false. Results are C-order Boolean values.
        pub fn $name<T: PartialOrd, A: TensorInput<T>, B: TensorInput<T>>(
            left: &A, right: &B,
        ) -> Result<Tensor<bool>, OpsError> {
            zip_map(left, right, |a, b| a $op b)
        }
    };
}
comparison!(less, <);
comparison!(less_equal, <=);
comparison!(greater, >);
comparison!(greater_equal, >=);

/// Elementwise Boolean negation, allocating a C-order Boolean tensor.
///
/// # Examples
///
/// ```
/// use kernix_core::{Order, shape::Kahen, tensor::Tensor};
/// use kernix_ops::logical_not;
/// let mut a = Tensor::from_vec(vec![true,false,true,true], Kahen::new([2,2])?, Order::C)?;
/// let result = logical_not(&a)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn logical_not<A: TensorInput<bool>>(source: &A) -> Result<Tensor<bool>, OpsError> {
    map(source, |value| !*value)
}

/// Elementwise Boolean conjunction with trailing-axis broadcasting.
/// Incompatible shapes, invalid reads or allocation failures return an error.
///
/// # Examples
///
/// ```
/// use kernix_core::{Order, shape::Kahen, tensor::Tensor};
/// use kernix_ops::logical_and;
/// let mut a = Tensor::from_vec(vec![true,false,true,true], Kahen::new([2,2])?, Order::C)?;
/// let result = logical_and(&a,&a)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn logical_and<A: TensorInput<bool>, B: TensorInput<bool>>(
    left: &A,
    right: &B,
) -> Result<Tensor<bool>, OpsError> {
    zip_map(left, right, |a, b| *a && *b)
}

/// Elementwise Boolean disjunction with trailing-axis broadcasting.
/// Incompatible shapes, invalid reads or allocation failures return an error.
///
/// # Examples
///
/// ```
/// use kernix_core::{Order, shape::Kahen, tensor::Tensor};
/// use kernix_ops::logical_or;
/// let mut a = Tensor::from_vec(vec![true,false,true,true], Kahen::new([2,2])?, Order::C)?;
/// let result = logical_or(&a,&a)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn logical_or<A: TensorInput<bool>, B: TensorInput<bool>>(
    left: &A,
    right: &B,
) -> Result<Tensor<bool>, OpsError> {
    zip_map(left, right, |a, b| *a || *b)
}

/// Elementwise exclusive OR with trailing-axis broadcasting.
/// Incompatible shapes, invalid reads or allocation failures return an error.
///
/// # Examples
///
/// ```
/// use kernix_core::{Order, shape::Kahen, tensor::Tensor};
/// use kernix_ops::logical_xor;
/// let mut a = Tensor::from_vec(vec![true,false,true,true], Kahen::new([2,2])?, Order::C)?;
/// let result = logical_xor(&a,&a)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn logical_xor<A: TensorInput<bool>, B: TensorInput<bool>>(
    left: &A,
    right: &B,
) -> Result<Tensor<bool>, OpsError> {
    zip_map(left, right, |a, b| *a ^ *b)
}

/// Selects one of two values at each logical coordinate. All three operands
/// are broadcast together; `false` selects `no`, `true` selects `yes`.
/// The output is a newly allocated C-contiguous tensor.
///
/// # Examples
///
/// ```
/// use kernix_core::{Order, shape::Kahen, tensor::Tensor};
/// use kernix_ops::where_select;
/// let mut a = Tensor::from_vec(vec![1.0,2.0,3.0,4.0], Kahen::new([2,2])?, Order::C)?;
/// let mask = Tensor::from_vec(vec![true, false], Kahen::new([2])?, Order::C)?;
/// let result = where_select(&mask,&a,&a)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn where_select<T: Clone, C, A, B>(
    condition: &C,
    yes: &A,
    no: &B,
) -> Result<Tensor<T>, OpsError>
where
    C: TensorInput<bool>,
    A: TensorInput<T>,
    B: TensorInput<T>,
{
    let ab = broadcast_shape(yes.dims(), no.dims())?;
    let dims = broadcast_shape(&ab, condition.dims())?;
    let shape = Kahen::new(&dims)?;
    let mut ci = vec![0; condition.dims().len()];
    let mut ai = vec![0; yes.dims().len()];
    let mut bi = vec![0; no.dims().len()];
    Tensor::<T>::try_from_fn(shape, Order::C, align_of::<T>(), |coords| {
        project_broadcast(coords, condition.dims(), &mut ci);
        if *condition.get(&ci)? {
            project_broadcast(coords, yes.dims(), &mut ai);
            Ok::<T, OpsError>(yes.get(&ai)?.clone())
        } else {
            project_broadcast(coords, no.dims(), &mut bi);
            Ok::<T, OpsError>(no.get(&bi)?.clone())
        }
    })
    .map_err(flatten_build_error)
}

/// Clamps values between inclusive scalar bounds. Unordered bounds (NaN) and
/// `minimum > maximum` are rejected. A NaN in the input propagates unchanged.
///
/// # Examples
///
/// ```
/// use kernix_core::{Order, shape::Kahen, tensor::Tensor};
/// use kernix_ops::clamp;
/// let mut a = Tensor::from_vec(vec![1.0,2.0,3.0,4.0], Kahen::new([2,2])?, Order::C)?;
/// let result = clamp(&a, 0.0, 3.0)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn clamp<T: PartialOrd + Clone, A: TensorInput<T>>(
    source: &A,
    minimum: T,
    maximum: T,
) -> Result<Tensor<T>, OpsError> {
    if minimum.partial_cmp(&maximum).is_none_or(|ord| ord.is_gt()) {
        return Err(OpsError::InvalidBounds);
    }
    map(source, |value| {
        if value < &minimum {
            minimum.clone()
        } else if value > &maximum {
            maximum.clone()
        } else {
            value.clone()
        }
    })
}
