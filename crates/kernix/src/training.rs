//! Parameters, reproducible initialization, explicit optimizers and gradient clipping.
//!
//! ```
//! use kernix::{autograd::Tape, training::{Parameter,Sgd}, Kahen,Order,Tensor};
//! let mut weight=Parameter::new(Tensor::from_vec(vec![2.0f64],Kahen::new([1])?,Order::C)?)?;
//! let mut tape=Tape::new();let w=weight.track(&mut tape)?;
//! let square=tape.mul(&w,&w)?;let loss=tape.sum(&square)?;
//! let gradients=tape.backward(&loss)?;
//! Sgd::new(0.1)?.step(&mut [(&mut weight,gradients.get(&w)?.unwrap())])?;
//! assert!((weight.value().storage_slice()[0]-1.6).abs()<1e-14);
//! # Ok::<(),Box<dyn std::error::Error>>(())
//! ```
use crate::{
    FloatMath, Kahen, OpsError, Order, ShapeError, Tensor, TensorError,
    autograd::{AutogradError, Tape, TrainFloat, Variable},
};
use alloc::{rc::Rc, vec, vec::Vec};
use core::mem::align_of;
use thiserror::Error;

/// Parameter, optimizer or initialization validation failure.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum TrainingError {
    /// Differentiation or checked value validation failed.
    #[error(transparent)]
    Autograd(#[from] AutogradError),
    /// Shape validation failed.
    #[error(transparent)]
    Shape(#[from] ShapeError),
    /// Tensor construction failed.
    #[error(transparent)]
    Tensor(#[from] TensorError),
    /// A portable numerical operation failed.
    #[error(transparent)]
    Ops(#[from] OpsError),
    /// A parameter or hyperparameter violates its documented domain.
    #[error("invalid training parameter: {0}")]
    Invalid(&'static str),
    /// A gradient, moment or candidate parameter update is not finite.
    #[error("nonfinite gradient, moment or parameter update")]
    NonFinite,
}
pub(crate) fn finite<T: TrainFloat>(values: &[T]) -> Result<(), TrainingError> {
    if values.iter().any(|x| !FloatMath::is_finite(*x)) {
        Err(TrainingError::NonFinite)
    } else {
        Ok(())
    }
}

/// A finite C-contiguous trainable tensor with stable optimizer identity.
/// Clone creates an independent parameter identity, even if values are identical.
/// Read access is immutable; replacement and optimizer updates preserve shape.
pub struct Parameter<T: TrainFloat> {
    pub(crate) value: Tensor<T>,
    pub(crate) owner: Rc<()>,
}
impl<T: TrainFloat> Clone for Parameter<T> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            owner: Rc::new(()),
        }
    }
}
impl<T: TrainFloat> Parameter<T> {
    /// Adopt a finite tensor, materializing logical C order if necessary. Invalid
    /// values return NonFinite before a parameter can reach an optimizer.
    pub fn new(value: Tensor<T>) -> Result<Self, TrainingError> {
        let value = if value.layout().is_c_contiguous()
            && value.origin() == 0
            && value.storage_len() == value.numel()
        {
            value
        } else {
            value.to_owned_contiguous(Order::C)?
        };
        finite(value.storage_slice())?;
        Ok(Self {
            value,
            owner: Rc::new(()),
        })
    }
    /// Borrow the current finite parameter value for inference or serialization.
    pub fn value(&self) -> &Tensor<T> {
        &self.value
    }
    /// Replace a value with identical dimensions, preserving optimizer identity.
    /// Validation happens before mutation. To reset optimizer history use a new Parameter.
    pub fn replace(&mut self, value: Tensor<T>) -> Result<(), TrainingError> {
        if value.shape() != self.value.shape() {
            return Err(TrainingError::Invalid(
                "parameter replacement shape mismatch",
            ));
        }
        self.value = Self::new(value)?.value;
        Ok(())
    }
    /// Snapshot current values into a trainable leaf, reusing the same leaf for this
    /// parameter on repeated calls within one tape generation. This correctly sums
    /// shared/recurrent parameter gradients. Later optimizer updates do not alter the
    /// graph snapshot: use a new/cleared tape to observe updated parameters.
    pub fn track(&self, tape: &mut Tape<T>) -> Result<Variable, AutogradError> {
        tape.track_parameter(&self.owner, &self.value)
    }
    /// Xavier-uniform weights with dimensions `[out,in,...kernel]`. Fan counts include
    /// kernel volume. Rank must be at least two with positive fan counts; dimensions
    /// and fan arithmetic are checked. Initialization is reproducible for a given seed.
    pub fn xavier(dims: impl AsRef<[usize]>, rng: &mut SeededRng) -> Result<Self, TrainingError> {
        let dims = dims.as_ref();
        Kahen::new(dims)?;
        if dims.len() < 2 {
            return Err(TrainingError::Invalid("Xavier requires rank at least two"));
        }
        let volume = Kahen::new(&dims[2..])?.numel();
        let fans = dims[0]
            .checked_add(dims[1])
            .and_then(|x| x.checked_mul(volume))
            .filter(|&x| x > 0)
            .ok_or(TrainingError::Invalid(
                "positive representable Xavier fan sum required",
            ))?;
        let bound = FloatMath::sqrt(6.0 / fans as f64);
        Self::new(rng.uniform(dims, T::from_f64(-bound), T::from_f64(bound))?)
    }
}
/// Deterministic SplitMix64 initialization stream (not a cryptographic RNG).
/// Integer transitions are identical across supported architectures. Float rounding
/// follows the requested dtype; no operating-system entropy is needed.
#[derive(Debug, Clone)]
pub struct SeededRng {
    state: u64,
}
impl SeededRng {
    /// Start a reproducible stream from an explicit seed.
    pub const fn new(seed: u64) -> Self {
        Self { state: seed }
    }
    /// Generate the next 64-bit value using wrapping SplitMix64 transitions.
    pub fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9e3779b97f4a7c15);
        let mut x = self.state;
        x = (x ^ (x >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        x = (x ^ (x >> 27)).wrapping_mul(0x94d049bb133111eb);
        x ^ (x >> 31)
    }
    /// Allocate finite uniform samples in [low,high) using 53 random fraction bits.
    /// Rounding to f32 can reach the rounded upper endpoint. Equal bounds fill a
    /// constant. Reject nonfinite/reversed bounds or unrepresentable interval width.
    pub fn uniform<T: TrainFloat>(
        &mut self,
        dims: impl AsRef<[usize]>,
        low: T,
        high: T,
    ) -> Result<Tensor<T>, TrainingError> {
        let (low, high) = (low.to_f64(), high.to_f64());
        if !low.is_finite() || !high.is_finite() || high < low || !(high - low).is_finite() {
            return Err(TrainingError::Invalid(
                "finite ordered uniform bounds required",
            ));
        }
        let tensor = Tensor::from_fn(Kahen::new(dims)?, Order::C, align_of::<T>(), |_| {
            let fraction = (self.next_u64() >> 11) as f64 * (1.0 / 9007199254740992.0);
            T::from_f64(low + (high - low) * fraction)
        })?;
        finite(tensor.storage_slice())?;
        Ok(tensor)
    }
}
fn validate<T: TrainFloat>(
    parameter: &Parameter<T>,
    gradient: &Tensor<T>,
) -> Result<(), TrainingError> {
    if parameter.value.shape() != gradient.shape() {
        return Err(TrainingError::Invalid(
            "gradient shape must exactly match parameter",
        ));
    }
    for &value in gradient.iter(Order::C)? {
        if !FloatMath::is_finite(value) {
            return Err(TrainingError::NonFinite);
        }
    }
    Ok(())
}
/// Plain stochastic gradient descent with an explicit positive finite learning rate.
/// No hidden momentum or weight decay is applied. Use one step for a batch of
/// parameter-gradient pairs; all candidate updates validate before any are committed.
pub struct Sgd<T: TrainFloat> {
    learning_rate: T,
}
impl<T: TrainFloat> Sgd<T> {
    /// Create SGD with a finite positive learning rate. See the module training example.
    pub fn new(learning_rate: T) -> Result<Self, TrainingError> {
        if !FloatMath::is_finite(learning_rate) || learning_rate <= T::zero() {
            return Err(TrainingError::Invalid(
                "positive finite learning rate required",
            ));
        }
        Ok(Self { learning_rate })
    }
    /// Apply simultaneous checked updates. Mismatched/nonfinite gradients or
    /// unrepresentable candidates leave every parameter unchanged.
    pub fn step(
        &self,
        updates: &mut [(&mut Parameter<T>, &Tensor<T>)],
    ) -> Result<(), TrainingError> {
        let mut candidates = Vec::with_capacity(updates.len());
        for (p, g) in updates.iter() {
            validate(p, g)?;
            let next = crate::zip_map(&p.value, *g, |p, g| *p - self.learning_rate * *g)?;
            finite(next.storage_slice())?;
            candidates.push(next);
        }
        for ((p, _), next) in updates.iter_mut().zip(candidates) {
            p.value = next;
        }
        Ok(())
    }
}
/// Adam hyperparameters; nonzero weight decay selects decoupled AdamW updates.
#[derive(Debug, Clone, Copy)]
pub struct AdamOptions<T: TrainFloat> {
    /// Positive finite step size; default 0.001.
    pub learning_rate: T,
    /// First-moment exponential retention in [0,1); default 0.9.
    pub beta1: T,
    /// Second-moment retention in [0,1); default 0.999.
    pub beta2: T,
    /// Positive finite denominator epsilon; default 1e-8.
    pub epsilon: T,
    /// Nonnegative finite decoupled weight decay; default zero.
    pub weight_decay: T,
}
impl<T: TrainFloat> Default for AdamOptions<T> {
    fn default() -> Self {
        Self {
            learning_rate: T::from_f64(0.001),
            beta1: T::from_f64(0.9),
            beta2: T::from_f64(0.999),
            epsilon: T::from_f64(1e-8),
            weight_decay: T::zero(),
        }
    }
}
#[derive(Clone)]
pub(crate) struct AdamState<T> {
    pub(crate) owner: Rc<()>,
    pub(crate) m: Vec<T>,
    pub(crate) v: Vec<T>,
    pub(crate) step: u64,
    pub(crate) beta1_power: T,
    pub(crate) beta2_power: T,
}
/// Adam/AdamW with first/second moments keyed by stable parameter identity.
/// State is independent per parameter, including update count and bias correction.
/// Cloned parameters do not share moments. All step candidates validate before commit.
pub struct Adam<T: TrainFloat> {
    pub(crate) options: AdamOptions<T>,
    pub(crate) states: Vec<AdamState<T>>,
}
impl<T: TrainFloat> Adam<T> {
    /// Construct standard Adam defaults with an explicit learning rate.
    pub fn new(learning_rate: T) -> Result<Self, TrainingError> {
        Self::from_options(AdamOptions {
            learning_rate,
            ..Default::default()
        })
    }
    /// Validate all hyperparameters. No fast-math or implicit dtype promotion is used.
    pub fn from_options(options: AdamOptions<T>) -> Result<Self, TrainingError> {
        let o = &options;
        if [o.learning_rate, o.beta1, o.beta2, o.epsilon, o.weight_decay]
            .iter()
            .any(|x| !FloatMath::is_finite(*x))
            || o.learning_rate <= T::zero()
            || o.epsilon <= T::zero()
            || o.weight_decay < T::zero()
            || o.beta1 < T::zero()
            || o.beta1 >= T::one()
            || o.beta2 < T::zero()
            || o.beta2 >= T::one()
        {
            return Err(TrainingError::Invalid("invalid Adam hyperparameter domain"));
        }
        Ok(Self {
            options,
            states: Vec::new(),
        })
    }
    /// Inspect the immutable hyperparameters; construct a new optimizer to change policy.
    pub fn options(&self) -> AdamOptions<T> {
        self.options
    }
    /// Number of parameter identities with allocated moments.
    pub fn state_count(&self) -> usize {
        self.states.len()
    }
    /// Discard all moments and per-parameter update counts, retaining hyperparameters.
    pub fn reset(&mut self) {
        self.states.clear();
    }
    /// Apply a checked batch of updates. The update is
    /// p - lr*(m_hat/(sqrt(v_hat)+epsilon) + weight_decay*p).
    /// Moment overflow, nonfinite gradients, shape errors and counter overflow
    /// leave all parameters and moments unchanged. State lookup is O(parameter count).
    pub fn step(
        &mut self,
        updates: &mut [(&mut Parameter<T>, &Tensor<T>)],
    ) -> Result<(), TrainingError> {
        let mut pending = Vec::with_capacity(updates.len());
        let o = self.options;
        for (parameter, gradient) in updates.iter() {
            validate(parameter, gradient)?;
            let position = self
                .states
                .iter()
                .position(|s| Rc::ptr_eq(&s.owner, &parameter.owner));
            let mut state = position
                .map(|i| self.states[i].clone())
                .unwrap_or_else(|| AdamState {
                    owner: parameter.owner.clone(),
                    m: vec![T::zero(); parameter.value.numel()],
                    v: vec![T::zero(); parameter.value.numel()],
                    step: 0,
                    beta1_power: T::one(),
                    beta2_power: T::one(),
                });
            state.step = state
                .step
                .checked_add(1)
                .ok_or(TrainingError::Invalid("Adam update counter overflow"))?;
            state.beta1_power = state.beta1_power * o.beta1;
            state.beta2_power = state.beta2_power * o.beta2;
            let mut next = Vec::with_capacity(parameter.value.numel());
            // Gradients can have arbitrary validated layouts: materialize logical order.
            let logical = gradient.to_owned_contiguous(Order::C)?;
            for (i, (&p, &g)) in parameter
                .value
                .storage_slice()
                .iter()
                .zip(logical.storage_slice())
                .enumerate()
            {
                state.m[i] = o.beta1 * state.m[i] + (T::one() - o.beta1) * g;
                state.v[i] = o.beta2 * state.v[i] + (T::one() - o.beta2) * g * g;
                let mh = state.m[i] / (T::one() - state.beta1_power);
                let vh = state.v[i] / (T::one() - state.beta2_power);
                next.push(
                    p - o.learning_rate
                        * (mh / (FloatMath::sqrt(vh) + o.epsilon) + o.weight_decay * p),
                );
            }
            finite(&state.m)?;
            finite(&state.v)?;
            finite(&next)?;
            pending.push((
                position,
                state,
                Tensor::from_vec(next, parameter.value.shape().clone(), Order::C)?,
            ));
        }
        for ((parameter, _), (position, state, value)) in updates.iter_mut().zip(pending) {
            parameter.value = value;
            if let Some(i) = position {
                self.states[i] = state
            } else {
                self.states.push(state);
            }
        }
        Ok(())
    }
}
/// Clip a list of owned canonical C-order gradients to a global L2 norm, returning the
/// original norm (which may be infinity if unrepresentable). Scaled sum-of-squares
/// avoids intermediate overflow. Validate every gradient and positive finite limit
/// before mutation. No element buffer is allocated; logical backing gaps are untouched.
pub fn clip_grad_norm<T: TrainFloat>(
    gradients: &mut [Tensor<T>],
    max_norm: T,
) -> Result<T, TrainingError> {
    if !FloatMath::is_finite(max_norm) || max_norm <= T::zero() {
        return Err(TrainingError::Invalid(
            "positive finite clip limit required",
        ));
    }
    let mut scale = T::zero();
    let mut ssq = T::one();
    for tensor in gradients.iter() {
        // Require canonical output gradients so physical gaps can never be modified.
        if !tensor.layout().is_c_contiguous()
            || tensor.origin() != 0
            || tensor.storage_len() != tensor.numel()
        {
            return Err(TrainingError::Invalid(
                "clip requires owned canonical C-order gradients",
            ));
        }
        finite(tensor.storage_slice())?;
        for &x in tensor.storage_slice() {
            let x = FloatMath::abs(x);
            if x != T::zero() {
                if scale < x {
                    let r = scale / x;
                    ssq = T::one() + ssq * r * r;
                    scale = x;
                } else {
                    let r = x / scale;
                    ssq += r * r;
                }
            }
        }
    }
    if scale == T::zero() {
        return Ok(T::zero());
    }
    let root = FloatMath::sqrt(ssq);
    let norm = scale * root;
    let ratio = (max_norm / scale) / root;
    if ratio < T::one() {
        for tensor in gradients {
            for value in tensor.storage_slice_mut() {
                *value = (*value / scale) * (max_norm / root);
            }
        }
    }
    Ok(norm)
}
