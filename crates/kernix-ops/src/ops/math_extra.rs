//! Stable elementary functions and attention-oriented softmax variants.
//!
//! These portable `no_std` reference kernels intentionally use checked logical
//! indexing. They work for arbitrary validated C/F, reversed and broadcast views.
//! Backends may replace execution while preserving the public numerical contract.

use alloc::vec;
use core::mem::align_of;
use kernix_core::{layout::Order, shape::Kahen, tensor::Tensor};

use super::{
    OpsError, TensorInput,
    broadcast::next_c,
    elementwise::{map, zip_map},
    error::flatten_build_error,
    indexed::normalized_axis,
    math::{FloatMath, scalar_softplus},
};

/// Scalar trigonometric and hypot primitives supported by `libm` in `no_std`.
pub trait FloatTrig: FloatMath {
    /// Compute sine in radians; infinite arguments produce NaN.
    fn sin(self) -> Self;
    /// Compute cosine in radians; infinite arguments produce NaN.
    fn cos(self) -> Self;
    /// Compute the signed angle in radians for coordinates (self, x), respecting signed zeros.
    fn atan2(self, x: Self) -> Self;
    /// Compute sqrt(self*self + x*x) with scaling to reduce intermediate overflow.
    fn hypot(self, x: Self) -> Self;
}

impl FloatTrig for f32 {
    fn sin(self) -> Self {
        #[cfg(all(feature = "std", not(feature = "portable-math")))]
        {
            f32::sin(self)
        }
        #[cfg(any(not(feature = "std"), feature = "portable-math"))]
        {
            libm::sinf(self)
        }
    }
    fn cos(self) -> Self {
        #[cfg(all(feature = "std", not(feature = "portable-math")))]
        {
            f32::cos(self)
        }
        #[cfg(any(not(feature = "std"), feature = "portable-math"))]
        {
            libm::cosf(self)
        }
    }
    fn atan2(self, x: Self) -> Self {
        #[cfg(all(feature = "std", not(feature = "portable-math")))]
        {
            f32::atan2(self, x)
        }
        #[cfg(any(not(feature = "std"), feature = "portable-math"))]
        {
            libm::atan2f(self, x)
        }
    }
    fn hypot(self, x: Self) -> Self {
        #[cfg(all(feature = "std", not(feature = "portable-math")))]
        {
            f32::hypot(self, x)
        }
        #[cfg(any(not(feature = "std"), feature = "portable-math"))]
        {
            libm::hypotf(self, x)
        }
    }
}

impl FloatTrig for f64 {
    fn sin(self) -> Self {
        #[cfg(all(feature = "std", not(feature = "portable-math")))]
        {
            f64::sin(self)
        }
        #[cfg(any(not(feature = "std"), feature = "portable-math"))]
        {
            libm::sin(self)
        }
    }
    fn cos(self) -> Self {
        #[cfg(all(feature = "std", not(feature = "portable-math")))]
        {
            f64::cos(self)
        }
        #[cfg(any(not(feature = "std"), feature = "portable-math"))]
        {
            libm::cos(self)
        }
    }
    fn atan2(self, x: Self) -> Self {
        #[cfg(all(feature = "std", not(feature = "portable-math")))]
        {
            f64::atan2(self, x)
        }
        #[cfg(any(not(feature = "std"), feature = "portable-math"))]
        {
            libm::atan2(self, x)
        }
    }
    fn hypot(self, x: Self) -> Self {
        #[cfg(all(feature = "std", not(feature = "portable-math")))]
        {
            f64::hypot(self, x)
        }
        #[cfg(any(not(feature = "std"), feature = "portable-math"))]
        {
            libm::hypot(self, x)
        }
    }
}

/// Elementwise accurate `ln(1+x)` for small `x`.
///
/// # Examples
///
/// ```
/// use kernix_core::{Order, shape::Kahen, tensor::Tensor};
/// use kernix_ops::log1p;
/// let mut a = Tensor::from_vec(vec![1.0,2.0,3.0,4.0], Kahen::new([2,2])?, Order::C)?;
/// let result = log1p(&a)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn log1p<T: FloatMath, A: TensorInput<T>>(source: &A) -> Result<Tensor<T>, OpsError> {
    map(source, |x| FloatMath::ln_1p(*x))
}

/// Elementwise accurate `exp(x)-1` for small `x`.
///
/// # Examples
///
/// ```
/// use kernix_core::{Order, shape::Kahen, tensor::Tensor};
/// use kernix_ops::expm1;
/// let mut a = Tensor::from_vec(vec![1.0,2.0,3.0,4.0], Kahen::new([2,2])?, Order::C)?;
/// let result = expm1(&a)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn expm1<T: FloatMath, A: TensorInput<T>>(source: &A) -> Result<Tensor<T>, OpsError> {
    map(source, |x| FloatMath::exp_m1(*x))
}

/// Stable `log(1 - exp(x))` on x <= 0, including values close to zero.
/// Values above zero produce NaN according to the real-domain restriction.
fn scalar_log1mexp<T: FloatMath>(x: T) -> T {
    let cutoff = -FloatMath::ln(T::from_count(2));
    if x < cutoff {
        FloatMath::ln_1p(-FloatMath::exp(x))
    } else {
        FloatMath::ln(-FloatMath::exp_m1(x))
    }
}

/// Allocate stable log(1-exp(x)) for x <= 0.
/// Zero maps to negative infinity, positive inputs to NaN, and negative infinity to zero.
///
/// # Examples
///
/// ```
/// use kernix_core::{Order, shape::Kahen, tensor::Tensor};
/// use kernix_ops::log1mexp;
/// let mut a = Tensor::from_vec(vec![1.0,2.0,3.0,4.0], Kahen::new([2,2])?, Order::C)?;
/// let result = log1mexp(&a)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn log1mexp<T: FloatMath, A: TensorInput<T>>(source: &A) -> Result<Tensor<T>, OpsError> {
    map(source, |x| scalar_log1mexp(*x))
}

/// Stable `log(exp(a)-exp(b))`, defined for a >= b (otherwise NaN).
/// Equal finite values return -infinity; b=-infinity gives a.
///
/// # Examples
///
/// ```
/// use kernix_core::{Order, shape::Kahen, tensor::Tensor};
/// use kernix_ops::logdiffexp;
/// let mut a = Tensor::from_vec(vec![1.0,2.0,3.0,4.0], Kahen::new([2,2])?, Order::C)?;
/// let result = logdiffexp(&a, &a)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn logdiffexp<T: FloatMath, A: TensorInput<T>, B: TensorInput<T>>(
    left: &A,
    right: &B,
) -> Result<Tensor<T>, OpsError> {
    zip_map(left, right, |a, b| {
        if *a == *b {
            return if *a > T::zero() && !FloatMath::is_finite(*a) {
                T::zero() / T::zero() // inf - inf is undefined
            } else {
                -T::one() / T::zero()
            };
        }
        if !FloatMath::is_finite(*b) && *b < T::zero() {
            return *a;
        }
        *a + scalar_log1mexp(*b - *a)
    })
}

/// Generalized softplus `ln(1+exp(beta*x))/beta`, beta > 0 and finite.
/// Uses `log1p` and stable branches to avoid overflow for finite x.
///
/// # Examples
///
/// ```
/// use kernix_core::{Order, shape::Kahen, tensor::Tensor};
/// use kernix_ops::softplus_beta;
/// let mut a = Tensor::from_vec(vec![1.0,2.0,3.0,4.0], Kahen::new([2,2])?, Order::C)?;
/// let result = softplus_beta(&a, 2.0)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn softplus_beta<T: FloatMath, A: TensorInput<T>>(
    source: &A,
    beta: T,
) -> Result<Tensor<T>, OpsError> {
    if !FloatMath::is_finite(beta) || beta <= T::zero() {
        return Err(OpsError::InvalidParameter {
            operation: "softplus_beta",
            parameter: "beta (must be finite and > 0)",
        });
    }
    map(source, |x| {
        if *x > T::zero() {
            *x + FloatMath::ln_1p(FloatMath::exp(-(beta * *x))) / beta
        } else {
            FloatMath::ln_1p(FloatMath::exp(beta * *x)) / beta
        }
    })
}

/// Stable `log(exp(a) + exp(b))`; handles both infinities and IEEE NaNs.
fn scalar_logaddexp<T: FloatMath>(a: T, b: T) -> T {
    if FloatMath::is_nan(a) || FloatMath::is_nan(b) {
        return a + b;
    }
    if a == b {
        return if FloatMath::is_finite(a) {
            a + FloatMath::ln(T::from_count(2))
        } else {
            a // +inf/+inf or -inf/-inf
        };
    }
    let (hi, lo) = if a > b { (a, b) } else { (b, a) };
    if !FloatMath::is_finite(hi) || !FloatMath::is_finite(lo) {
        return hi;
    }
    hi + FloatMath::ln_1p(FloatMath::exp(lo - hi))
}

/// Stable, broadcasting binary log-add-exp; unlike ordinary addition it
/// computes in the log domain and never exponentiates a large positive value.
///
/// # Examples
///
/// ```
/// use kernix_core::{Order, shape::Kahen, tensor::Tensor};
/// use kernix_ops::logaddexp;
/// let mut a = Tensor::from_vec(vec![1.0,2.0,3.0,4.0], Kahen::new([2,2])?, Order::C)?;
/// let result = logaddexp(&a, &a)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn logaddexp<T: FloatMath, A: TensorInput<T>, B: TensorInput<T>>(
    left: &A,
    right: &B,
) -> Result<Tensor<T>, OpsError> {
    zip_map(left, right, |a, b| scalar_logaddexp(*a, *b))
}

/// `log(sigmoid(x)) = -softplus(-x)`, avoiding underflow in `sigmoid(x)`.
///
/// # Examples
///
/// ```
/// use kernix_core::{Order, shape::Kahen, tensor::Tensor};
/// use kernix_ops::logsigmoid;
/// let mut a = Tensor::from_vec(vec![1.0,2.0,3.0,4.0], Kahen::new([2,2])?, Order::C)?;
/// let result = logsigmoid(&a)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn logsigmoid<T: FloatMath, A: TensorInput<T>>(source: &A) -> Result<Tensor<T>, OpsError> {
    map(source, |x| -scalar_softplus(-*x))
}

/// Mish activation, `x * tanh(softplus(x))`. The `-inf` limit is zero.
///
/// # Examples
///
/// ```
/// use kernix_core::{Order, shape::Kahen, tensor::Tensor};
/// use kernix_ops::mish;
/// let mut a = Tensor::from_vec(vec![1.0,2.0,3.0,4.0], Kahen::new([2,2])?, Order::C)?;
/// let result = mish(&a)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn mish<T: FloatMath, A: TensorInput<T>>(source: &A) -> Result<Tensor<T>, OpsError> {
    map(source, |x| {
        if *x < T::zero() && !FloatMath::is_finite(*x) {
            T::zero()
        } else {
            *x * FloatMath::tanh(scalar_softplus(*x))
        }
    })
}

/// Reciprocal square root (negative inputs follow IEEE floating-point rules).
///
/// # Examples
///
/// ```
/// use kernix_core::{Order, shape::Kahen, tensor::Tensor};
/// use kernix_ops::rsqrt;
/// let mut a = Tensor::from_vec(vec![1.0,2.0,3.0,4.0], Kahen::new([2,2])?, Order::C)?;
/// let result = rsqrt(&a)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn rsqrt<T: FloatMath, A: TensorInput<T>>(source: &A) -> Result<Tensor<T>, OpsError> {
    map(source, |x| T::one() / FloatMath::sqrt(*x))
}

/// Compute sine in radians; infinite arguments produce NaN.
///
/// # Examples
///
/// ```
/// use kernix_core::{Order, shape::Kahen, tensor::Tensor};
/// use kernix_ops::sin;
/// let mut a = Tensor::from_vec(vec![1.0,2.0,3.0,4.0], Kahen::new([2,2])?, Order::C)?;
/// let result = sin(&a)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn sin<T: FloatTrig, A: TensorInput<T>>(source: &A) -> Result<Tensor<T>, OpsError> {
    map(source, |x| FloatTrig::sin(*x))
}

/// Compute cosine in radians; infinite arguments produce NaN.
///
/// # Examples
///
/// ```
/// use kernix_core::{Order, shape::Kahen, tensor::Tensor};
/// use kernix_ops::cos;
/// let mut a = Tensor::from_vec(vec![1.0,2.0,3.0,4.0], Kahen::new([2,2])?, Order::C)?;
/// let result = cos(&a)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn cos<T: FloatTrig, A: TensorInput<T>>(source: &A) -> Result<Tensor<T>, OpsError> {
    map(source, |x| FloatTrig::cos(*x))
}

/// IEEE-style `atan2(y, x)`, with NumPy-style broadcasting.
///
/// # Examples
///
/// ```
/// use kernix_core::{Order, shape::Kahen, tensor::Tensor};
/// use kernix_ops::atan2;
/// let mut a = Tensor::from_vec(vec![1.0,2.0,3.0,4.0], Kahen::new([2,2])?, Order::C)?;
/// let result = atan2(&a, &a)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn atan2<T: FloatTrig, A: TensorInput<T>, B: TensorInput<T>>(
    y: &A,
    x: &B,
) -> Result<Tensor<T>, OpsError> {
    zip_map(y, x, |a, b| FloatTrig::atan2(*a, *b))
}

/// Overflow/underflow-resistant `hypot(x, y)`, with broadcasting.
///
/// # Examples
///
/// ```
/// use kernix_core::{Order, shape::Kahen, tensor::Tensor};
/// use kernix_ops::hypot;
/// let mut a = Tensor::from_vec(vec![1.0,2.0,3.0,4.0], Kahen::new([2,2])?, Order::C)?;
/// let result = hypot(&a, &a)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn hypot<T: FloatTrig, A: TensorInput<T>, B: TensorInput<T>>(
    x: &A,
    y: &B,
) -> Result<Tensor<T>, OpsError> {
    zip_map(x, y, |a, b| FloatTrig::hypot(*a, *b))
}

// Inputs are finite and temperature is positive. Subtract first for nearby
// logits; divide first only when opposite-sign subtraction overflowed.
fn scaled_difference<T: FloatMath>(value: T, maximum: T, temperature: T) -> T {
    let difference = value - maximum;
    if FloatMath::is_finite(difference) {
        difference / temperature
    } else {
        value / temperature - maximum / temperature
    }
}

fn mask_allows(
    mask: Option<&dyn TensorInput<bool>>,
    coords: &[usize],
    mask_coords: &mut [usize],
) -> Result<bool, OpsError> {
    let Some(mask) = mask else {
        return Ok(true);
    };
    let dims = mask.dims();
    let leading = coords.len() - dims.len();
    for (i, &dim) in dims.iter().enumerate() {
        mask_coords[i] = if dim == 1 { 0 } else { coords[leading + i] };
    }
    Ok(*mask.get(mask_coords)?)
}

/// Numerically stable normalized exponential with optional attention mask.
///
/// A mask is broadcast *to the input*, never allowed to enlarge the output.
/// Masked values are not read or checked for finiteness, and get probability
/// zero (or log-probability -infinity). A nonempty, fully masked reduction row
/// is an error rather than silently returning NaNs. Empty outputs are valid.
fn normalized_exp_advanced<T: FloatMath, A: TensorInput<T>>(
    source: &A,
    axis: isize,
    temperature: T,
    mask: Option<&dyn TensorInput<bool>>,
    logarithmic: bool,
    operation: &'static str,
) -> Result<Tensor<T>, OpsError> {
    if !FloatMath::is_finite(temperature) || temperature <= T::zero() {
        return Err(OpsError::InvalidParameter {
            operation,
            parameter: "temperature (must be finite and > 0)",
        });
    }
    let dims = source.dims();
    let axis = normalized_axis(dims.len(), axis)?;
    let shape = Kahen::new(dims)?;
    if let Some(mask) = mask {
        let mdims = mask.dims();
        if mdims.len() > dims.len() {
            return Err(OpsError::InPlaceBroadcastIncompatible);
        }
        let lead = dims.len() - mdims.len();
        for (i, &dim) in mdims.iter().enumerate() {
            if dim != 1 && dim != dims[lead + i] {
                return Err(OpsError::BroadcastIncompatible {
                    axis: lead + i,
                    left: dims[lead + i],
                    right: dim,
                });
            }
        }
    }
    let mut output = Tensor::filled(shape, Order::C, align_of::<T>(), T::zero())?;
    if output.numel() == 0 {
        return Ok(output);
    }
    let mut outer_dims = dims.to_vec();
    outer_dims.remove(axis);
    let outer_shape = Kahen::new(&outer_dims)?;
    let mut outer = vec![0; outer_dims.len()];
    let mut index = vec![0; dims.len()];
    let mut mask_coords = vec![0; mask.map_or(0, |m| m.dims().len())];
    let masked_log_value = -T::one() / T::zero();
    for _ in 0..outer_shape.numel() {
        index[..axis].copy_from_slice(&outer[..axis]);
        index[axis + 1..].copy_from_slice(&outer[axis..]);
        let mut maximum: Option<T> = None;
        for position in 0..dims[axis] {
            index[axis] = position;
            if !mask_allows(mask, &index, &mut mask_coords)? {
                continue;
            }
            let value = *source.get(&index)?;
            if !FloatMath::is_finite(value) {
                return Err(OpsError::NonFiniteInput {
                    operation,
                    indices: index.clone(),
                });
            }
            if maximum.is_none_or(|prev| value > prev) {
                maximum = Some(value);
            }
        }
        let maximum = maximum.ok_or(OpsError::InvalidParameter {
            operation,
            parameter: "fully masked reduction axis",
        })?;
        let mut sum = T::zero();
        for position in 0..dims[axis] {
            index[axis] = position;
            if mask_allows(mask, &index, &mut mask_coords)? {
                sum = sum
                    + FloatMath::exp(scaled_difference(
                        *source.get(&index)?,
                        maximum,
                        temperature,
                    ));
            }
        }
        // At least one unmasked maximum contributes exp(0) == 1.
        let log_sum = if logarithmic {
            Some(FloatMath::ln(sum))
        } else {
            None
        };
        for position in 0..dims[axis] {
            index[axis] = position;
            let value = if mask_allows(mask, &index, &mut mask_coords)? {
                let delta = scaled_difference(*source.get(&index)?, maximum, temperature);
                if let Some(log_sum) = log_sum {
                    delta - log_sum
                } else {
                    FloatMath::exp(delta) / sum
                }
            } else if logarithmic {
                masked_log_value
            } else {
                T::zero()
            };
            *output.get_mut(&index)? = value;
        }
        next_c(&mut outer, &outer_dims);
    }
    Ok(output)
}

/// Softmax with a strictly positive, finite temperature. Normalization uses
/// `(x - max(x)) / temperature` to avoid overflowing `x / temperature`.
///
/// # Examples
///
/// ```
/// use kernix_core::{Order, shape::Kahen, tensor::Tensor};
/// use kernix_ops::softmax_temperature;
/// let mut a = Tensor::from_vec(vec![1.0,2.0,3.0,4.0], Kahen::new([2,2])?, Order::C)?;
/// let result = softmax_temperature(&a,-1,0.5)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn softmax_temperature<T: FloatMath, A: TensorInput<T>>(
    source: &A,
    axis: isize,
    temperature: T,
) -> Result<Tensor<T>, OpsError> {
    normalized_exp_advanced(
        source,
        axis,
        temperature,
        None,
        false,
        "softmax_temperature",
    )
}

/// Numerically stable temperature-scaled log-softmax.
///
/// # Examples
///
/// ```
/// use kernix_core::{Order, shape::Kahen, tensor::Tensor};
/// use kernix_ops::log_softmax_temperature;
/// let mut a = Tensor::from_vec(vec![1.0,2.0,3.0,4.0], Kahen::new([2,2])?, Order::C)?;
/// let result = log_softmax_temperature(&a,-1,0.5)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn log_softmax_temperature<T: FloatMath, A: TensorInput<T>>(
    source: &A,
    axis: isize,
    temperature: T,
) -> Result<Tensor<T>, OpsError> {
    normalized_exp_advanced(
        source,
        axis,
        temperature,
        None,
        true,
        "log_softmax_temperature",
    )
}

/// Attention-style boolean masked softmax (`true` = include). A fully masked
/// nonempty row returns `OpsError::InvalidParameter`.
///
/// # Examples
///
/// ```
/// use kernix_core::{Order, shape::Kahen, tensor::Tensor};
/// use kernix_ops::masked_softmax;
/// let mut a = Tensor::from_vec(vec![1.0,2.0,3.0,4.0], Kahen::new([2,2])?, Order::C)?;
/// let mask = Tensor::from_vec(vec![true, false], Kahen::new([2])?, Order::C)?;
/// let result = masked_softmax(&a,&mask,-1)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn masked_softmax<T: FloatMath, A: TensorInput<T>, M: TensorInput<bool>>(
    source: &A,
    mask: &M,
    axis: isize,
) -> Result<Tensor<T>, OpsError> {
    normalized_exp_advanced(source, axis, T::one(), Some(mask), false, "masked_softmax")
}

/// Masked log-softmax (`false` positions yield negative infinity).
///
/// # Examples
///
/// ```
/// use kernix_core::{Order, shape::Kahen, tensor::Tensor};
/// use kernix_ops::masked_log_softmax;
/// let mut a = Tensor::from_vec(vec![1.0,2.0,3.0,4.0], Kahen::new([2,2])?, Order::C)?;
/// let mask = Tensor::from_vec(vec![true, false], Kahen::new([2])?, Order::C)?;
/// let result = masked_log_softmax(&a,&mask,-1)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn masked_log_softmax<T: FloatMath, A: TensorInput<T>, M: TensorInput<bool>>(
    source: &A,
    mask: &M,
    axis: isize,
) -> Result<Tensor<T>, OpsError> {
    normalized_exp_advanced(
        source,
        axis,
        T::one(),
        Some(mask),
        true,
        "masked_log_softmax",
    )
}

/// Fused masking and temperature scaling without an intermediate tensor.
///
/// # Examples
///
/// ```
/// use kernix_core::{Order, shape::Kahen, tensor::Tensor};
/// use kernix_ops::masked_softmax_temperature;
/// let mut a = Tensor::from_vec(vec![1.0,2.0,3.0,4.0], Kahen::new([2,2])?, Order::C)?;
/// let mask = Tensor::from_vec(vec![true, false], Kahen::new([2])?, Order::C)?;
/// let result = masked_softmax_temperature(&a,&mask,-1,0.5)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn masked_softmax_temperature<T: FloatMath, A: TensorInput<T>, M: TensorInput<bool>>(
    source: &A,
    mask: &M,
    axis: isize,
    temperature: T,
) -> Result<Tensor<T>, OpsError> {
    normalized_exp_advanced(
        source,
        axis,
        temperature,
        Some(mask),
        false,
        "masked_softmax_temperature",
    )
}

/// Fused masked temperature-scaled log-softmax.
///
/// # Examples
///
/// ```
/// use kernix_core::{Order, shape::Kahen, tensor::Tensor};
/// use kernix_ops::masked_log_softmax_temperature;
/// let mut a = Tensor::from_vec(vec![1.0,2.0,3.0,4.0], Kahen::new([2,2])?, Order::C)?;
/// let mask = Tensor::from_vec(vec![true, false], Kahen::new([2])?, Order::C)?;
/// let result = masked_log_softmax_temperature(&a,&mask,-1,0.5)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn masked_log_softmax_temperature<T: FloatMath, A: TensorInput<T>, M: TensorInput<bool>>(
    source: &A,
    mask: &M,
    axis: isize,
    temperature: T,
) -> Result<Tensor<T>, OpsError> {
    normalized_exp_advanced(
        source,
        axis,
        temperature,
        Some(mask),
        true,
        "masked_log_softmax_temperature",
    )
}

/// Streaming-normalizer recurrence used by online softmax and log-sum-exp.
/// The running maximum is in unscaled logit units: this avoids prematurely
/// overflowing `logit / temperature` when temperature is very small.
fn online_update_temperature<T: FloatMath>(running: &mut Option<(T, T)>, value: T, temperature: T) {
    match running {
        None => *running = Some((value, T::one())),
        Some((maximum, sum)) if value > *maximum => {
            *sum =
                *sum * FloatMath::exp(scaled_difference(*maximum, value, temperature)) + T::one();
            *maximum = value;
        }
        Some((maximum, sum)) => {
            *sum = *sum + FloatMath::exp(scaled_difference(value, *maximum, temperature));
        }
    }
}

/// Online softmax: one source pass for running (max, exp-sum), then one pass
/// to materialize normalized output. This is a reference implementation of
/// the streaming normalization recurrence, not a FlashAttention GPU kernel.
fn normalized_exp_online<T: FloatMath, A: TensorInput<T>>(
    source: &A,
    axis: isize,
    temperature: T,
    logarithmic: bool,
) -> Result<Tensor<T>, OpsError> {
    let operation = if logarithmic {
        "log_softmax_online"
    } else {
        "softmax_online"
    };
    if !FloatMath::is_finite(temperature) || temperature <= T::zero() {
        return Err(OpsError::InvalidParameter {
            operation,
            parameter: "temperature (must be finite and > 0)",
        });
    }
    let dims = source.dims();
    let axis = normalized_axis(dims.len(), axis)?;
    let mut output = Tensor::filled(Kahen::new(dims)?, Order::C, align_of::<T>(), T::zero())?;
    if output.numel() == 0 {
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
        let mut running = None;
        for position in 0..dims[axis] {
            index[axis] = position;
            let value = *source.get(&index)?;
            if !FloatMath::is_finite(value) {
                return Err(OpsError::NonFiniteInput {
                    operation,
                    indices: index.clone(),
                });
            }
            online_update_temperature(&mut running, value, temperature);
        }
        let (maximum, sum) = running.ok_or(OpsError::EmptyReduction)?;
        let log_sum = if logarithmic {
            Some(FloatMath::ln(sum))
        } else {
            None
        };
        for position in 0..dims[axis] {
            index[axis] = position;
            let delta = scaled_difference(*source.get(&index)?, maximum, temperature);
            *output.get_mut(&index)? = if let Some(ls) = log_sum {
                delta - ls
            } else {
                FloatMath::exp(delta) / sum
            };
        }
        next_c(&mut outer, &outer_dims);
    }
    Ok(output)
}

/// Compute finite-input softmax using a streaming (maximum, exponential sum)
/// recurrence and a second pass to emit output. Reject invalid axes/nonfinite inputs.
/// No intermediate tensor is allocated beyond the C-order output and axis coordinates.
///
/// # Examples
///
/// ```
/// use kernix_core::{Order, shape::Kahen, tensor::Tensor};
/// use kernix_ops::softmax_online;
/// let mut a = Tensor::from_vec(vec![1.0,2.0,3.0,4.0], Kahen::new([2,2])?, Order::C)?;
/// let result = softmax_online(&a, -1)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn softmax_online<T: FloatMath, A: TensorInput<T>>(
    source: &A,
    axis: isize,
) -> Result<Tensor<T>, OpsError> {
    normalized_exp_online(source, axis, T::one(), false)
}

/// Compute finite-input log-softmax using a streaming normalization recurrence.
/// Reject invalid axes/nonfinite inputs; empty inputs yield empty outputs.
///
/// # Examples
///
/// ```
/// use kernix_core::{Order, shape::Kahen, tensor::Tensor};
/// use kernix_ops::log_softmax_online;
/// let mut a = Tensor::from_vec(vec![1.0,2.0,3.0,4.0], Kahen::new([2,2])?, Order::C)?;
/// let result = log_softmax_online(&a, -1)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn log_softmax_online<T: FloatMath, A: TensorInput<T>>(
    source: &A,
    axis: isize,
) -> Result<Tensor<T>, OpsError> {
    normalized_exp_online(source, axis, T::one(), true)
}

/// Compute online softmax with a strictly positive finite temperature.
/// Scaled differences avoid intermediate overflow when possible. Reject invalid
/// temperature, axes or nonfinite logits; allocate one C-order output.
///
/// # Examples
///
/// ```
/// use kernix_core::{Order, shape::Kahen, tensor::Tensor};
/// use kernix_ops::softmax_online_temperature;
/// let mut a = Tensor::from_vec(vec![1.0,2.0,3.0,4.0], Kahen::new([2,2])?, Order::C)?;
/// let result = softmax_online_temperature(&a,-1,0.5)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn softmax_online_temperature<T: FloatMath, A: TensorInput<T>>(
    source: &A,
    axis: isize,
    temperature: T,
) -> Result<Tensor<T>, OpsError> {
    normalized_exp_online(source, axis, temperature, false)
}

/// One-pass online log-sum-exp update (Milakov/Gimelshein recurrence).
/// Finite-only, preserving Kernix's current `logsumexp_*` input contract.
fn online_update<T: FloatMath>(running: &mut Option<(T, T)>, value: T) {
    match running {
        None => *running = Some((value, T::one())),
        Some((maximum, sum)) if value > *maximum => {
            *sum = *sum * FloatMath::exp(*maximum - value) + T::one();
            *maximum = value;
        }
        Some((maximum, sum)) => {
            *sum = *sum + FloatMath::exp(value - *maximum);
        }
    }
}

/// Online (single-input-pass) log-sum-exp of all values; empty input errors.
///
/// # Examples
///
/// ```
/// use kernix_core::{Order, shape::Kahen, tensor::Tensor};
/// use kernix_ops::logsumexp_online_all;
/// let mut a = Tensor::from_vec(vec![1.0,2.0,3.0,4.0], Kahen::new([2,2])?, Order::C)?;
/// let result = logsumexp_online_all(&a)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn logsumexp_online_all<T: FloatMath, A: TensorInput<T>>(source: &A) -> Result<T, OpsError> {
    let shape = Kahen::new(source.dims())?;
    let mut coords = vec![0; shape.rank()];
    let mut running = None;
    for _ in 0..shape.numel() {
        let value = *source.get(&coords)?;
        if !FloatMath::is_finite(value) {
            return Err(OpsError::NonFiniteInput {
                operation: "logsumexp_online_all",
                indices: coords,
            });
        }
        online_update(&mut running, value);
        next_c(&mut coords, source.dims());
    }
    let (maximum, sum) = running.ok_or(OpsError::EmptyReduction)?;
    Ok(maximum + FloatMath::ln(sum))
}

/// Online one-pass log-sum-exp along one axis. Avoids rescanning each row to
/// obtain the normalization sum. `keepdims` preserves the reduced axis.
///
/// # Examples
///
/// ```
/// use kernix_core::{Order, shape::Kahen, tensor::Tensor};
/// use kernix_ops::logsumexp_online_axis;
/// let mut a = Tensor::from_vec(vec![1.0,2.0,3.0,4.0], Kahen::new([2,2])?, Order::C)?;
/// let result = logsumexp_online_axis(&a, -1, false)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn logsumexp_online_axis<T: FloatMath, A: TensorInput<T>>(
    source: &A,
    axis: isize,
    keepdims: bool,
) -> Result<Tensor<T>, OpsError> {
    let dims = source.dims();
    let axis = normalized_axis(dims.len(), axis)?;
    let mut out_dims = dims.to_vec();
    if keepdims {
        out_dims[axis] = 1;
    } else {
        out_dims.remove(axis);
    }
    let mut from = vec![0; dims.len()];
    Tensor::<T>::try_from_fn(Kahen::new(out_dims)?, Order::C, align_of::<T>(), |coords| {
        for i in 0..dims.len() {
            from[i] = if i == axis {
                0
            } else if keepdims || i < axis {
                coords[i]
            } else {
                coords[i - 1]
            };
        }
        let mut running = None;
        for position in 0..dims[axis] {
            from[axis] = position;
            let value = *source.get(&from)?;
            if !FloatMath::is_finite(value) {
                return Err(OpsError::NonFiniteInput {
                    operation: "logsumexp_online_axis",
                    indices: from.clone(),
                });
            }
            online_update(&mut running, value);
        }
        let (maximum, sum) = running.ok_or(OpsError::EmptyReduction)?;
        Ok(maximum + FloatMath::ln(sum))
    })
    .map_err(flatten_build_error)
}
