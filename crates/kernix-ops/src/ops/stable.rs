//! Numerically stable floating-point reference reductions. All arithmetic
//! uses the `FloatMath` contract, suitable for no_std f32/f64 through libm.
use alloc::vec;
use core::mem::align_of;
use kernix_core::{layout::Order, shape::Kahen, tensor::Tensor};

use super::{
    OpsError, TensorInput, broadcast::next_c, error::flatten_build_error, indexed::normalized_axis,
    math::FloatMath,
};

fn check_finite<T: FloatMath>(
    value: T,
    operation: &'static str,
    coords: &[usize],
) -> Result<T, OpsError> {
    if !FloatMath::is_finite(value) {
        return Err(OpsError::NonFiniteInput {
            operation,
            indices: coords.to_vec(),
        });
    }
    Ok(value)
}

/// Log of the sum of exponentials of every element, computed with a max
/// shift. Rejects nonfinite inputs and empty logical tensors.
pub fn logsumexp_all<T: FloatMath, A: TensorInput<T>>(source: &A) -> Result<T, OpsError> {
    let shape = Kahen::new(source.dims())?;
    if shape.numel() == 0 {
        return Err(OpsError::EmptyReduction);
    }
    let mut coords = vec![0; shape.rank()];
    let mut maximum: Option<T> = None;
    for _ in 0..shape.numel() {
        let value = check_finite(*source.get(&coords)?, "logsumexp_all", &coords)?;
        if maximum.is_none_or(|m| value > m) {
            maximum = Some(value);
        }
        next_c(&mut coords, source.dims());
    }
    let maximum = maximum.ok_or(OpsError::EmptyReduction)?;
    coords.fill(0);
    let mut total = T::zero();
    for _ in 0..shape.numel() {
        total = total + FloatMath::exp(*source.get(&coords)? - maximum);
        next_c(&mut coords, source.dims());
    }
    Ok(maximum + FloatMath::ln(total))
}

/// Stable log-sum-exp along one axis. `keepdims` preserves a unit axis.
/// A zero-sized output remains valid; a nonempty output with empty reduced
/// axis returns `EmptyReduction`.
pub fn logsumexp_axis<T: FloatMath, A: TensorInput<T>>(
    source: &A,
    axis: isize,
    keepdims: bool,
) -> Result<Tensor<T>, OpsError> {
    let dims = source.dims();
    let axis = normalized_axis(dims.len(), axis)?;
    let mut output_dims = dims.to_vec();
    if keepdims {
        output_dims[axis] = 1;
    } else {
        output_dims.remove(axis);
    }
    let mut from = vec![0; dims.len()];
    Tensor::<T>::try_from_fn(
        Kahen::new(output_dims)?,
        Order::C,
        align_of::<T>(),
        |coords| {
            if dims[axis] == 0 {
                return Err(OpsError::EmptyReduction);
            }
            for i in 0..dims.len() {
                from[i] = if i == axis {
                    0
                } else if keepdims || i < axis {
                    coords[i]
                } else {
                    coords[i - 1]
                };
            }
            let mut maximum: Option<T> = None;
            for i in 0..dims[axis] {
                from[axis] = i;
                let value = check_finite(*source.get(&from)?, "logsumexp_axis", &from)?;
                if maximum.is_none_or(|m| value > m) {
                    maximum = Some(value);
                }
            }
            let max = maximum.ok_or(OpsError::EmptyReduction)?;
            let mut total = T::zero();
            for i in 0..dims[axis] {
                from[axis] = i;
                total = total + FloatMath::exp(*source.get(&from)? - max);
            }
            Ok::<T, OpsError>(max + FloatMath::ln(total))
        },
    )
    .map_err(flatten_build_error)
}

/// Stable Euclidean norm. This scaled sum-of-squares algorithm avoids the
/// overflow/underflow of `sqrt(sum(x*x))` for large/small finite values.
/// Empty tensor has norm zero. Nonfinite inputs produce an indexed error.
pub fn l2_norm_all<T: FloatMath, A: TensorInput<T>>(source: &A) -> Result<T, OpsError> {
    let shape = Kahen::new(source.dims())?;
    let mut coords = vec![0; shape.rank()];
    let mut scale = T::zero();
    let mut ssq = T::one();
    for _ in 0..shape.numel() {
        let value = FloatMath::abs(check_finite(*source.get(&coords)?, "l2_norm_all", &coords)?);
        if value != T::zero() {
            if scale < value {
                let ratio = scale / value;
                ssq = T::one() + ssq * ratio * ratio;
                scale = value;
            } else {
                let ratio = value / scale;
                ssq = ssq + ratio * ratio;
            }
        }
        next_c(&mut coords, source.dims());
    }
    Ok(scale * FloatMath::sqrt(ssq))
}

/// Stable Euclidean norm along one axis. `keepdims` is analogous to reductions.
pub fn l2_norm_axis<T: FloatMath, A: TensorInput<T>>(
    source: &A,
    axis: isize,
    keepdims: bool,
) -> Result<Tensor<T>, OpsError> {
    let dims = source.dims();
    let axis = normalized_axis(dims.len(), axis)?;
    let mut output_dims = dims.to_vec();
    if keepdims {
        output_dims[axis] = 1;
    } else {
        output_dims.remove(axis);
    }
    let mut from = vec![0; dims.len()];
    Tensor::<T>::try_from_fn(
        Kahen::new(output_dims)?,
        Order::C,
        align_of::<T>(),
        |coords| {
            for i in 0..dims.len() {
                from[i] = if i == axis {
                    0
                } else if keepdims || i < axis {
                    coords[i]
                } else {
                    coords[i - 1]
                };
            }
            let mut scale = T::zero();
            let mut ssq = T::one();
            for i in 0..dims[axis] {
                from[axis] = i;
                let value =
                    FloatMath::abs(check_finite(*source.get(&from)?, "l2_norm_axis", &from)?);
                if value != T::zero() {
                    if scale < value {
                        let ratio = scale / value;
                        ssq = T::one() + ssq * ratio * ratio;
                        scale = value;
                    } else {
                        let ratio = value / scale;
                        ssq = ssq + ratio * ratio;
                    }
                }
            }
            Ok::<T, OpsError>(scale * FloatMath::sqrt(ssq))
        },
    )
    .map_err(flatten_build_error)
}

/// Infallible element conversion using `From`. The explicit type arguments
/// avoid implicit dtype promotion; callers decide the target dtype.
pub fn cast<T: Clone, U: From<T>, A: TensorInput<T>>(source: &A) -> Result<Tensor<U>, OpsError> {
    super::elementwise::map(source, |value| U::from(value.clone()))
}

/// Checked element conversion using `TryFrom`. Failing coordinates are
/// preserved in `OpsError::Cast` rather than silently truncated.
pub fn try_cast<T: Clone, U: TryFrom<T>, A: TensorInput<T>>(
    source: &A,
) -> Result<Tensor<U>, OpsError> {
    super::elementwise::try_map_indexed(source, |value, indices| {
        U::try_from(value.clone()).map_err(|_| OpsError::Cast {
            indices: indices.to_vec(),
        })
    })
}
