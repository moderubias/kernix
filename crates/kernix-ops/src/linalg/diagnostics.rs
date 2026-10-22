//! Explicit one-norm conditioning and working-precision refinement.
use super::{LinalgError, LinalgFloat, Lu, check_finite, copy_matrix, max_abs, rhs, square};
use crate::{FloatMath, TensorInput};
use alloc::{vec, vec::Vec};
use kernix_core::{Order, tensor::Tensor};

/// Reciprocal one-norm condition number, `1/(||A||_1 ||inverse(A)||_1)`.
///
/// Globally scales the finite square input, factors with exact-zero pivot rejection,
/// then solves every identity column. This is an O(n^3) calculation with O(n^2)
/// factors and O(n) column workspace, not a cheap norm estimator or a two-norm/SVD
/// condition number. Empty matrices return one; exactly singular factors return
/// zero. Required unrepresentable solves return NonFinite; arbitrary exponent
/// ranges are not guaranteed. Reciprocal underflow may return zero.
///
/// ```
/// use kernix_core::tensor::Tensor;
/// let a=Tensor::from_values([2,2],vec![1.0f64,0.0,0.0,0.25])?;
/// assert_eq!(kernix_ops::linalg::reciprocal_condition_number_one(&a)?,0.25);
/// # Ok::<(),kernix_ops::linalg::LinalgError>(())
/// ```
pub fn reciprocal_condition_number_one<T: LinalgFloat, A: TensorInput<T>>(
    input: &A,
) -> Result<T, LinalgError> {
    let mut a = copy_matrix(input)?;
    let n = square(&a)?;
    if n == 0 {
        return Ok(T::one());
    }
    let scale = max_abs(a.storage_slice());
    if scale == T::zero() {
        return Ok(T::zero());
    }
    for value in a.storage_slice_mut() {
        *value = *value / scale;
    }
    let mut norm = T::zero();
    for col in 0..n {
        let mut sum = T::zero();
        for row in 0..n {
            sum += FloatMath::abs(a.storage_slice()[row * n + col]);
        }
        if sum > norm {
            norm = sum;
        }
    }
    let lu = match Lu::factor_with_tolerance(&a, T::zero()) {
        Ok(lu) => lu,
        Err(LinalgError::Singular { .. }) => return Ok(T::zero()),
        Err(error) => return Err(error),
    };
    let mut unit = Tensor::<T>::zeros([n])?;
    let mut best = (T::zero(), T::zero());
    for col in 0..n {
        unit.storage_slice_mut()[col] = T::one();
        let x = lu.solve(&unit)?;
        unit.storage_slice_mut()[col] = T::zero();
        let mut candidate = (T::zero(), T::zero());
        for &v in x.storage_slice() {
            add_magnitude(&mut candidate, FloatMath::abs(v));
        }
        let greater = if best.0 == T::zero() {
            true
        } else if candidate.0 <= best.0 {
            candidate.1 * (candidate.0 / best.0) > best.1
        } else {
            candidate.1 > best.1 * (best.0 / candidate.0)
        };
        if greater {
            best = candidate;
        }
    }
    let value = ((T::one() / norm) / best.1) / best.0;
    if !FloatMath::is_finite(value) {
        return Err(LinalgError::NonFinite);
    }
    // Roundoff may put a theoretically unit upper bound just above one.
    Ok(if value > T::one() { T::one() } else { value })
}

/// One-norm condition number calculated by globally scaled LU and n column solves.
/// Costs O(n^3), allocates factors/column workspace, and never forms a full inverse.
/// Singular matrices or a condition number outside the finite dtype range return
/// NonFinite. Use reciprocal_condition_number_one when zero is the useful diagnostic.
/// Empty matrices return one. This is not a two-norm/SVD condition number.
///
/// ```
/// let a=kernix_core::tensor::Tensor::from_values([2,2],vec![1.0f32,0.0,0.0,0.25])?;
/// assert_eq!(kernix_ops::linalg::condition_number_one(&a)?,4.0);
/// # Ok::<(),kernix_ops::linalg::LinalgError>(())
/// ```
pub fn condition_number_one<T: LinalgFloat, A: TensorInput<T>>(
    input: &A,
) -> Result<T, LinalgError> {
    let reciprocal = reciprocal_condition_number_one(input)?;
    if reciprocal == T::zero() {
        return Err(LinalgError::NonFinite);
    }
    let value = T::one() / reciprocal;
    if !FloatMath::is_finite(value) {
        return Err(LinalgError::NonFinite);
    }
    Ok(value)
}

// A positive sum represented as scale*sum without overflowing that product.
fn add_magnitude<T: LinalgFloat>((scale, sum): &mut (T, T), value: T) {
    if value == T::zero() {
        return;
    }
    if value > *scale {
        *sum = T::one() + *sum * (*scale / value);
        *scale = value;
    } else {
        *sum += value / *scale;
    }
}

/// Reason a working-precision iterative refinement stopped.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum RefinementTermination {
    /// All componentwise backward errors are within the requested tolerance.
    Converged,
    /// A correction failed to strictly reduce the worst backward error; it was rejected.
    Stagnated,
    /// The caller's correction budget was exhausted.
    IterationLimit,
}

/// Best accepted solution and explicit diagnostics from solve_refined.
/// A returned solution need not have converged; inspect termination/backward_error.
pub struct RefinedSolution<T: LinalgFloat> {
    solution: Tensor<T>,
    backward_error: T,
    iterations: usize,
    termination: RefinementTermination,
}
impl<T: LinalgFloat> RefinedSolution<T> {
    /// Borrow the best accepted solution, in the same shape as the RHS.
    pub fn solution(&self) -> &Tensor<T> {
        &self.solution
    }
    /// Consume the diagnostics and move out the owned solution without copying.
    pub fn into_solution(self) -> Tensor<T> {
        self.solution
    }
    /// Worst componentwise relative backward error over every row and RHS column.
    pub fn backward_error(&self) -> T {
        self.backward_error
    }
    /// Number of attempted correction solves, including a rejected stagnating step.
    pub fn iterations(&self) -> usize {
        self.iterations
    }
    /// Explicit stopping reason; stagnation/iteration limits are not convergence.
    pub fn termination(&self) -> RefinementTermination {
        self.termination
    }
}

fn residual<T: LinalgFloat>(
    a: &[T],
    x: &[T],
    b: &[T],
    n: usize,
    columns: usize,
) -> Result<(Vec<T>, T), LinalgError> {
    let mut residual = vec![T::zero(); b.len()];
    let mut worst = T::zero();
    for row in 0..n {
        for col in 0..columns {
            let mut value = b[row * columns + col];
            let mut correction = T::zero();
            let mut denominator = (T::zero(), T::zero());
            add_magnitude(&mut denominator, FloatMath::abs(value));
            for k in 0..n {
                let product = a[row * n + k] * x[k * columns + col];
                if !FloatMath::is_finite(product) {
                    return Err(LinalgError::NonFinite);
                }
                add_magnitude(&mut denominator, FloatMath::abs(product));
                let term = -product;
                let next = value + term;
                if !FloatMath::is_finite(next) {
                    return Err(LinalgError::NonFinite);
                }
                correction += if FloatMath::abs(value) >= FloatMath::abs(term) {
                    (value - next) + term
                } else {
                    (term - next) + value
                };
                value = next;
            }
            value += correction;
            if !FloatMath::is_finite(value) {
                return Err(LinalgError::NonFinite);
            }
            residual[row * columns + col] = value;
            let error = if denominator.0 == T::zero() {
                T::zero()
            } else {
                (FloatMath::abs(value) / denominator.0) / denominator.1
            };
            if !FloatMath::is_finite(error) {
                return Err(LinalgError::NonFinite);
            }
            if error > worst {
                worst = error;
            }
        }
    }
    Ok((residual, worst))
}

/// Solve a square system, then refine with the same LU and compensated residuals.
///
/// Tolerance must be finite in [0,1); max_iterations bounds attempted corrections.
/// RHS is `[n]` or `[n,k]`. The diagnostic is the worst componentwise ratio
/// `abs(b-A*x)/(abs(b)+abs(A)*abs(x))`, with zero/zero defined as zero. Positive
/// denominator sums are scaled to avoid overflow. Residual products, corrections
/// and factors must remain representable. All arithmetic stays in T: this is not
/// mixed/extra precision, a certified forward-error bound, or a convergence promise.
///
/// Original A and LU are retained (O(n^2)); each attempt allocates O(n*k) residual,
/// correction and candidate buffers. Cost is O(n^3) factorization plus O(n^2*k)
/// per attempt. A step that does not reduce the worst backward error is rejected;
/// return termination explicitly distinguishes convergence, stagnation and budget.
/// Default LU pivot rejection applies; ill-conditioned systems may fail early.
///
/// ```
/// use kernix_core::tensor::Tensor;
/// use kernix_ops::linalg::{solve_refined,RefinementTermination};
/// let a=Tensor::from_values([2,2],vec![4.0f64,1.0,1.0,3.0])?;
/// let b=Tensor::from_values([2],vec![1.0,2.0])?;
/// let result=solve_refined(&a,&b,5,1e-14)?;
/// assert_eq!(result.termination(),RefinementTermination::Converged);
/// assert!(result.backward_error()<=1e-14);
/// # Ok::<(),kernix_ops::linalg::LinalgError>(())
/// ```
pub fn solve_refined<T: LinalgFloat, A: TensorInput<T>, B: TensorInput<T>>(
    input: &A,
    b: &B,
    max_iterations: usize,
    tolerance: T,
) -> Result<RefinedSolution<T>, LinalgError> {
    if !FloatMath::is_finite(tolerance) || tolerance < T::zero() || tolerance >= T::one() {
        return Err(LinalgError::Invalid(
            "refinement tolerance must be finite in [0,1)",
        ));
    }
    let a = copy_matrix(input)?;
    let n = square(&a)?;
    let (source, columns, shape) = rhs(b, n)?;
    let b = Tensor::from_vec(source, shape, Order::C)?;
    let lu = Lu::factor(&a)?;
    let mut solution = lu.solve(&b)?;
    let (mut r, mut error) = residual(
        a.storage_slice(),
        solution.storage_slice(),
        b.storage_slice(),
        n,
        columns,
    )?;
    let mut iterations = 0;
    let mut termination = RefinementTermination::IterationLimit;
    while error > tolerance && iterations < max_iterations {
        iterations += 1;
        let correction = lu.solve(&Tensor::from_vec(r, b.shape().clone(), Order::C)?)?;
        let mut candidate =
            Tensor::from_values(solution.shape().dims(), solution.storage_slice().to_vec())?;
        for (x, &d) in candidate
            .storage_slice_mut()
            .iter_mut()
            .zip(correction.storage_slice())
        {
            *x += d;
        }
        check_finite(candidate.storage_slice())?;
        let (next_r, next_error) = residual(
            a.storage_slice(),
            candidate.storage_slice(),
            b.storage_slice(),
            n,
            columns,
        )?;
        if next_error >= error {
            termination = RefinementTermination::Stagnated;
            break;
        }
        solution = candidate;
        error = next_error;
        r = next_r;
    }
    if error <= tolerance {
        termination = RefinementTermination::Converged;
    }
    Ok(RefinedSolution {
        solution,
        backward_error: error,
        iterations,
        termination,
    })
}
