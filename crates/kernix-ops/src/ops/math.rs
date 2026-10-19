//! Portable floating-point kernels with explicitly documented IEEE semantics.
//! `libm` permits use on `no_std` targets. Outputs use C-contiguous CPU storage.
use alloc::vec;
use core::{
    mem::align_of,
    ops::{Add, Mul, Neg, Sub},
};
use kernix_core::{layout::Order, shape::Kahen, tensor::Tensor};

use super::{FloatElement, One, OpsError, TensorInput, broadcast::next_c, elementwise::map};

/// Math operations for floating elements. In addition to the reference
/// kernels, acceleration backends may implement these with device intrinsics.
pub trait FloatMath:
    FloatElement
    + One
    + PartialOrd
    + PartialEq
    + Copy
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Neg<Output = Self>
{
    fn exp(self) -> Self;
    fn ln(self) -> Self;
    fn sqrt(self) -> Self;
    fn tanh(self) -> Self;
    fn erf(self) -> Self;
    fn abs(self) -> Self;
    fn is_nan(self) -> bool;
    fn is_finite(self) -> bool;
}

impl FloatMath for f32 {
    fn exp(self) -> Self {
        libm::expf(self)
    }
    fn ln(self) -> Self {
        libm::logf(self)
    }
    fn sqrt(self) -> Self {
        libm::sqrtf(self)
    }
    fn tanh(self) -> Self {
        libm::tanhf(self)
    }
    fn erf(self) -> Self {
        libm::erff(self)
    }
    fn abs(self) -> Self {
        libm::fabsf(self)
    }
    fn is_nan(self) -> bool {
        f32::is_nan(self)
    }
    fn is_finite(self) -> bool {
        f32::is_finite(self)
    }
}
impl FloatMath for f64 {
    fn exp(self) -> Self {
        libm::exp(self)
    }
    fn ln(self) -> Self {
        libm::log(self)
    }
    fn sqrt(self) -> Self {
        libm::sqrt(self)
    }
    fn tanh(self) -> Self {
        libm::tanh(self)
    }
    fn erf(self) -> Self {
        libm::erf(self)
    }
    fn abs(self) -> Self {
        libm::fabs(self)
    }
    fn is_nan(self) -> bool {
        f64::is_nan(self)
    }
    fn is_finite(self) -> bool {
        f64::is_finite(self)
    }
}

macro_rules! float_unary {
    ($name:ident, $method:ident) => {
        pub fn $name<T: FloatMath, A: TensorInput<T>>(source: &A) -> Result<Tensor<T>, OpsError> {
            map(source, |value| FloatMath::$method(*value))
        }
    };
}
float_unary!(exp, exp);
float_unary!(ln, ln);
float_unary!(sqrt, sqrt);
float_unary!(tanh, tanh);
float_unary!(abs, abs);

pub fn is_nan<T: FloatMath, A: TensorInput<T>>(source: &A) -> Result<Tensor<bool>, OpsError> {
    map(source, |value| FloatMath::is_nan(*value))
}
pub fn is_finite<T: FloatMath, A: TensorInput<T>>(source: &A) -> Result<Tensor<bool>, OpsError> {
    map(source, |value| FloatMath::is_finite(*value))
}

/// ReLU: NaNs propagate; finite negative values become positive zero.
pub fn relu<T: FloatMath, A: TensorInput<T>>(source: &A) -> Result<Tensor<T>, OpsError> {
    map(source, |value| {
        if FloatMath::is_nan(*value) || *value > T::zero() {
            *value
        } else {
            T::zero()
        }
    })
}

/// Stable sigmoid, avoiding overflow of `exp(-x)` for very negative values.
fn scalar_sigmoid<T: FloatMath>(x: T) -> T {
    if x >= T::zero() {
        T::one() / (T::one() + FloatMath::exp(-x))
    } else {
        let e = FloatMath::exp(x);
        e / (T::one() + e)
    }
}

pub fn sigmoid<T: FloatMath, A: TensorInput<T>>(source: &A) -> Result<Tensor<T>, OpsError> {
    map(source, |value| scalar_sigmoid(*value))
}

/// SiLU / Swish activation: `x * sigmoid(x)`.
pub fn silu<T: FloatMath, A: TensorInput<T>>(source: &A) -> Result<Tensor<T>, OpsError> {
    map(source, |value| *value * scalar_sigmoid(*value))
}

/// GELU using the exact Gaussian CDF formulation (`erf`), rather than a tanh
/// approximation. Accuracy depends on the portable libm implementation.
pub fn gelu<T: FloatMath, A: TensorInput<T>>(source: &A) -> Result<Tensor<T>, OpsError> {
    let half = T::one() / T::from_count(2);
    let root_two = FloatMath::sqrt(T::from_count(2));
    map(source, |value| {
        half * *value * (T::one() + FloatMath::erf(*value / root_two))
    })
}

pub fn leaky_relu<T: FloatMath, A: TensorInput<T>>(
    source: &A,
    negative_slope: T,
) -> Result<Tensor<T>, OpsError> {
    map(source, |value| {
        if FloatMath::is_nan(*value) || *value >= T::zero() {
            *value
        } else {
            *value * negative_slope
        }
    })
}

/// Numerically stable softplus, avoiding `exp(x)` overflow for large x.
pub fn softplus<T: FloatMath, A: TensorInput<T>>(source: &A) -> Result<Tensor<T>, OpsError> {
    map(source, |value| {
        if *value > T::zero() {
            *value + FloatMath::ln(T::one() + FloatMath::exp(-*value))
        } else {
            FloatMath::ln(T::one() + FloatMath::exp(*value))
        }
    })
}

fn normalize_axis(rank: usize, axis: isize) -> Result<usize, OpsError> {
    let normalized = if axis < 0 {
        rank.checked_add_signed(axis)
    } else {
        usize::try_from(axis).ok()
    };
    normalized
        .filter(|&a| a < rank)
        .ok_or(OpsError::AxisOutOfBounds { axis, rank })
}

/// Numerically stable softmax (or log-softmax) along a chosen axis.
/// Only *finite* input values are accepted, so the result never silently
/// turns `+infinity - +infinity` or `-infinity - -infinity` into NaNs.
/// Empty tensors return empty tensors; scalar input has no valid axis.
fn normalized_exp<T: FloatMath, A: TensorInput<T>>(
    source: &A,
    axis: isize,
    logarithmic: bool,
) -> Result<Tensor<T>, OpsError> {
    let dims = source.dims();
    let axis = normalize_axis(dims.len(), axis)?;
    let shape = Kahen::new(dims)?;
    let mut output = Tensor::filled(shape, Order::C, align_of::<T>(), T::zero())?;
    if output.shape().numel() == 0 {
        return Ok(output);
    }
    let mut outer_dims = dims.to_vec();
    outer_dims.remove(axis);
    let outer_shape = Kahen::new(&outer_dims)?;
    let mut outer = vec![0; outer_dims.len()];
    let mut index = vec![0; dims.len()];
    for _ in 0..outer_shape.numel() {
        index[..axis].copy_from_slice(&outer[..axis]);
        index[axis + 1..].copy_from_slice(&outer[axis..]);
        let mut maximum: Option<T> = None;
        for position in 0..dims[axis] {
            index[axis] = position;
            let value = *source.get(&index)?;
            if !FloatMath::is_finite(value) {
                return Err(OpsError::NonFiniteInput {
                    operation: if logarithmic {
                        "log_softmax"
                    } else {
                        "softmax"
                    },
                    indices: index.clone(),
                });
            }
            if maximum.is_none_or(|old| value > old) {
                maximum = Some(value);
            }
        }
        // Since output is nonempty, every processed axis has at least one element.
        let maximum = maximum.ok_or(OpsError::EmptyReduction)?;
        let mut sum = T::zero();
        for position in 0..dims[axis] {
            index[axis] = position;
            sum = sum + FloatMath::exp(*source.get(&index)? - maximum);
        }
        let log_sum = if logarithmic {
            Some(FloatMath::ln(sum))
        } else {
            None
        };
        for position in 0..dims[axis] {
            index[axis] = position;
            let delta = *source.get(&index)? - maximum;
            *output.get_mut(&index)? = if let Some(log_sum) = log_sum {
                delta - log_sum
            } else {
                FloatMath::exp(delta) / sum
            };
        }
        next_c(&mut outer, &outer_dims);
    }
    Ok(output)
}

pub fn softmax<T: FloatMath, A: TensorInput<T>>(
    source: &A,
    axis: isize,
) -> Result<Tensor<T>, OpsError> {
    normalized_exp(source, axis, false)
}

pub fn log_softmax<T: FloatMath, A: TensorInput<T>>(
    source: &A,
    axis: isize,
) -> Result<Tensor<T>, OpsError> {
    normalized_exp(source, axis, true)
}
