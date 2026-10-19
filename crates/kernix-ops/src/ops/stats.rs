//! Population/sample variance and standard deviation via Welford's online
//! recurrence, avoiding the cancellation of E[x*x] - E[x]*E[x].
use alloc::vec;
use core::mem::align_of;
use kernix_core::{layout::Order, shape::Kahen, tensor::Tensor};

use super::{
    OpsError, TensorInput,
    broadcast::next_c,
    elementwise::map,
    math::FloatMath,
    reduce::{axes_mask, project_reduction, reduced_dims},
};

#[derive(Clone, Copy)]
struct State<T> {
    count: usize,
    mean: T,
    m2: T,
}

impl<T: FloatMath> State<T> {
    fn new() -> Self {
        Self {
            count: 0,
            mean: T::zero(),
            m2: T::zero(),
        }
    }

    fn update(&mut self, value: T) {
        self.count += 1;
        let delta = value - self.mean;
        self.mean = self.mean + delta / T::from_count(self.count);
        let delta2 = value - self.mean;
        self.m2 = self.m2 + delta * delta2;
    }

    fn variance(&self, correction: usize) -> Result<T, OpsError> {
        if self.count <= correction {
            return Err(OpsError::InvalidDegreesOfFreedom {
                count: self.count,
                correction,
            });
        }
        let value = self.m2 / T::from_count(self.count - correction);
        Ok(if value < T::zero() { T::zero() } else { value })
    }
}

/// Variance of all logical values using Welford's recurrence.
/// `correction=0` means population variance; `correction=1` is Bessel's
/// correction. If `count <= correction`, returns a structured error.
pub fn variance_all<T: FloatMath, A: TensorInput<T>>(
    source: &A,
    correction: usize,
) -> Result<T, OpsError> {
    let shape = Kahen::new(source.dims())?;
    let mut state = State::new();
    let mut coords = vec![0; source.dims().len()];
    for _ in 0..shape.numel() {
        state.update(*source.get(&coords)?);
        next_c(&mut coords, source.dims());
    }
    state.variance(correction)
}

pub fn std_all<T: FloatMath, A: TensorInput<T>>(
    source: &A,
    correction: usize,
) -> Result<T, OpsError> {
    Ok(FloatMath::sqrt(variance_all(source, correction)?))
}

/// Variance over zero or more axes. Empty axes preserves the input shape and
/// produces zeros for `correction=0`. Invalid degrees of freedom are rejected
/// only if the output contains logical elements (empty outputs stay empty).
pub fn variance_axes<T: FloatMath, A: TensorInput<T>>(
    source: &A,
    axes: &[isize],
    keepdims: bool,
    correction: usize,
) -> Result<Tensor<T>, OpsError> {
    let source_shape = Kahen::new(source.dims())?;
    let mask = axes_mask(source.dims().len(), axes)?;
    let output_shape = Kahen::new(reduced_dims(source.dims(), &mask, keepdims))?;
    let mut states = Tensor::filled(
        output_shape,
        Order::C,
        align_of::<State<T>>(),
        State::<T>::new(),
    )?;
    let mut coords = vec![0; source.dims().len()];
    let mut dest = vec![0; states.rank()];
    for _ in 0..source_shape.numel() {
        project_reduction(&coords, &mask, keepdims, &mut dest);
        states.get_mut(&dest)?.update(*source.get(&coords)?);
        next_c(&mut coords, source.dims());
    }
    // Use a fallible initializer to preserve exact OpsError for invalid ddof.
    super::elementwise::try_map(&states, |state| state.variance(correction))
}

pub fn std_axes<T: FloatMath, A: TensorInput<T>>(
    source: &A,
    axes: &[isize],
    keepdims: bool,
    correction: usize,
) -> Result<Tensor<T>, OpsError> {
    let variances = variance_axes(source, axes, keepdims, correction)?;
    map(&variances, |value| FloatMath::sqrt(*value))
}
