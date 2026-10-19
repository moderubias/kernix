// Kernix numerics-softmax overlay 04 (applied by tools/kernix-update-04/apply.py)
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
    /// Accurate log(1+x), including x close to zero.
    fn ln_1p(self) -> Self {
        Self::ln(Self::one() + self)
    }
    /// Accurate exp(x)-1, including x close to zero.
    fn exp_m1(self) -> Self {
        Self::exp(self) - Self::one()
    }
    /// Compute the natural exponential, following IEEE nonfinite/domain behavior.
    fn exp(self) -> Self;
    /// Compute the natural logarithm; zero gives negative infinity, negatives give NaN.
    fn ln(self) -> Self;
    /// Compute the principal square root; negative finite values give NaN.
    fn sqrt(self) -> Self;
    /// Compute hyperbolic tangent, with limits -1 and +1 at infinities.
    fn tanh(self) -> Self;
    /// Compute the Gaussian error function using the portable scalar implementation.
    fn erf(self) -> Self;
    /// Return absolute magnitude, preserving NaN propagation and making negative zero positive.
    fn abs(self) -> Self;
    /// Test for IEEE NaN; tensor overload allocates a Boolean mask in C order.
    fn is_nan(self) -> bool;
    /// Test that a value is neither infinity nor NaN; tensor overload returns a Boolean mask.
    fn is_finite(self) -> bool;
}

impl FloatMath for f32 {
    fn exp(self) -> Self {
        libm::expf(self)
    }
    fn ln(self) -> Self {
        libm::logf(self)
    }
    fn ln_1p(self) -> Self {
        libm::log1pf(self)
    }
    fn exp_m1(self) -> Self {
        libm::expm1f(self)
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
    fn ln_1p(self) -> Self {
        libm::log1p(self)
    }
    fn exp_m1(self) -> Self {
        libm::expm1(self)
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
        /// Allocate an elementwise portable floating-point mathematical function.
        /// Inputs are read in logical order; IEEE domain and nonfinite behavior applies.
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

/// Test for IEEE NaN; tensor overload allocates a Boolean mask in C order.
///
/// # Examples
///
/// ```
/// use kernix_core::{Order, shape::Kahen, tensor::Tensor};
/// use kernix_ops::is_nan;
/// let mut a = Tensor::from_vec(vec![1.0,2.0,3.0,4.0], Kahen::new([2,2])?, Order::C)?;
/// let result = is_nan(&a)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn is_nan<T: FloatMath, A: TensorInput<T>>(source: &A) -> Result<Tensor<bool>, OpsError> {
    map(source, |value| FloatMath::is_nan(*value))
}
/// Test that a value is neither infinity nor NaN; tensor overload returns a Boolean mask.
///
/// # Examples
///
/// ```
/// use kernix_core::{Order, shape::Kahen, tensor::Tensor};
/// use kernix_ops::is_finite;
/// let mut a = Tensor::from_vec(vec![1.0,2.0,3.0,4.0], Kahen::new([2,2])?, Order::C)?;
/// let result = is_finite(&a)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn is_finite<T: FloatMath, A: TensorInput<T>>(source: &A) -> Result<Tensor<bool>, OpsError> {
    map(source, |value| FloatMath::is_finite(*value))
}

/// ReLU: NaNs propagate; finite negative values become positive zero.
///
/// # Examples
///
/// ```
/// use kernix_core::{Order, shape::Kahen, tensor::Tensor};
/// use kernix_ops::relu;
/// let mut a = Tensor::from_vec(vec![1.0,2.0,3.0,4.0], Kahen::new([2,2])?, Order::C)?;
/// let result = relu(&a)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
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

/// Allocate stable sigmoid values: 1/(1+exp(-x)).
/// Separate sign branches avoid exponential overflow. Infinities map to zero/one; NaNs propagate.
///
/// # Examples
///
/// ```
/// use kernix_core::{Order, shape::Kahen, tensor::Tensor};
/// use kernix_ops::sigmoid;
/// let mut a = Tensor::from_vec(vec![1.0,2.0,3.0,4.0], Kahen::new([2,2])?, Order::C)?;
/// let result = sigmoid(&a)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn sigmoid<T: FloatMath, A: TensorInput<T>>(source: &A) -> Result<Tensor<T>, OpsError> {
    map(source, |value| scalar_sigmoid(*value))
}

/// SiLU / Swish activation: `x * sigmoid(x)`.
///
/// # Examples
///
/// ```
/// use kernix_core::{Order, shape::Kahen, tensor::Tensor};
/// use kernix_ops::silu;
/// let mut a = Tensor::from_vec(vec![1.0,2.0,3.0,4.0], Kahen::new([2,2])?, Order::C)?;
/// let result = silu(&a)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn silu<T: FloatMath, A: TensorInput<T>>(source: &A) -> Result<Tensor<T>, OpsError> {
    map(source, |value| {
        if *value < T::zero() && !FloatMath::is_finite(*value) {
            T::zero()
        } else {
            *value * scalar_sigmoid(*value)
        }
    })
}

/// GELU using the exact Gaussian CDF formulation (`erf`), rather than a tanh
/// approximation. Accuracy depends on the portable libm implementation.
///
/// # Examples
///
/// ```
/// use kernix_core::{Order, shape::Kahen, tensor::Tensor};
/// use kernix_ops::gelu;
/// let mut a = Tensor::from_vec(vec![1.0,2.0,3.0,4.0], Kahen::new([2,2])?, Order::C)?;
/// let result = gelu(&a)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn gelu<T: FloatMath, A: TensorInput<T>>(source: &A) -> Result<Tensor<T>, OpsError> {
    let half = T::one() / T::from_count(2);
    let root_two = FloatMath::sqrt(T::from_count(2));
    map(source, |value| {
        half * *value * (T::one() + FloatMath::erf(*value / root_two))
    })
}

/// Allocate x for nonnegative x and negative_slope*x otherwise.
/// NaNs propagate. Slope uses the element dtype with no implicit promotion.
///
/// # Examples
///
/// ```
/// use kernix_core::{Order, shape::Kahen, tensor::Tensor};
/// use kernix_ops::leaky_relu;
/// let mut a = Tensor::from_vec(vec![1.0,2.0,3.0,4.0], Kahen::new([2,2])?, Order::C)?;
/// let result = leaky_relu(&a, 0.1)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
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
pub(crate) fn scalar_softplus<T: FloatMath>(value: T) -> T {
    if value > T::zero() {
        value + FloatMath::ln_1p(FloatMath::exp(-value))
    } else {
        FloatMath::ln_1p(FloatMath::exp(value))
    }
}

/// Allocate stable ln(1+exp(x)), using log1p and a sign-dependent branch.
/// Avoids exponential overflow for finite inputs and preserves small positive values.
///
/// # Examples
///
/// ```
/// use kernix_core::{Order, shape::Kahen, tensor::Tensor};
/// use kernix_ops::softplus;
/// let mut a = Tensor::from_vec(vec![1.0,2.0,3.0,4.0], Kahen::new([2,2])?, Order::C)?;
/// let result = softplus(&a)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn softplus<T: FloatMath, A: TensorInput<T>>(source: &A) -> Result<Tensor<T>, OpsError> {
    map(source, |value| scalar_softplus(*value))
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

/// Normalize finite logits along a signed axis using max-shifted exponentials.
/// Reject invalid axes or nonfinite inputs with their logical coordinates. Empty
/// tensors produce empty outputs. Allocates C-order output; takes O(numel*rank).
///
/// # Examples
///
/// ```
/// use kernix_core::{Order, shape::Kahen, tensor::Tensor};
/// use kernix_ops::softmax;
/// let mut a = Tensor::from_vec(vec![1.0,2.0,3.0,4.0], Kahen::new([2,2])?, Order::C)?;
/// let result = softmax(&a, -1)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn softmax<T: FloatMath, A: TensorInput<T>>(
    source: &A,
    axis: isize,
) -> Result<Tensor<T>, OpsError> {
    normalized_exp(source, axis, false)
}

/// Compute log probabilities directly from max-shifted finite logits.
/// Avoids taking log of rounded-zero probabilities. Invalid axes or nonfinite
/// inputs return errors; empty tensors return empty C-order outputs.
///
/// # Examples
///
/// ```
/// use kernix_core::{Order, shape::Kahen, tensor::Tensor};
/// use kernix_ops::log_softmax;
/// let mut a = Tensor::from_vec(vec![1.0,2.0,3.0,4.0], Kahen::new([2,2])?, Order::C)?;
/// let result = log_softmax(&a, -1)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn log_softmax<T: FloatMath, A: TensorInput<T>>(
    source: &A,
    axis: isize,
) -> Result<Tensor<T>, OpsError> {
    normalized_exp(source, axis, true)
}
