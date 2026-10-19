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
pub fn equal<T: PartialEq, A: TensorInput<T>, B: TensorInput<T>>(
    left: &A,
    right: &B,
) -> Result<Tensor<bool>, OpsError> {
    zip_map(left, right, |a, b| a == b)
}

/// Elementwise inequality.
pub fn not_equal<T: PartialEq, A: TensorInput<T>, B: TensorInput<T>>(
    left: &A,
    right: &B,
) -> Result<Tensor<bool>, OpsError> {
    zip_map(left, right, |a, b| a != b)
}

macro_rules! comparison {
    ($name:ident, $op:tt) => {
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

pub fn logical_not<A: TensorInput<bool>>(source: &A) -> Result<Tensor<bool>, OpsError> {
    map(source, |value| !*value)
}

pub fn logical_and<A: TensorInput<bool>, B: TensorInput<bool>>(
    left: &A,
    right: &B,
) -> Result<Tensor<bool>, OpsError> {
    zip_map(left, right, |a, b| *a && *b)
}

pub fn logical_or<A: TensorInput<bool>, B: TensorInput<bool>>(
    left: &A,
    right: &B,
) -> Result<Tensor<bool>, OpsError> {
    zip_map(left, right, |a, b| *a || *b)
}

pub fn logical_xor<A: TensorInput<bool>, B: TensorInput<bool>>(
    left: &A,
    right: &B,
) -> Result<Tensor<bool>, OpsError> {
    zip_map(left, right, |a, b| *a ^ *b)
}

/// Selects one of two values at each logical coordinate. All three operands
/// are broadcast together; `false` selects `no`, `true` selects `yes`.
/// The output is a newly allocated C-contiguous tensor.
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
