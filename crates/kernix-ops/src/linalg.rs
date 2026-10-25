//! Dense numerical linear algebra: pivoted LU, Cholesky, Householder QR and solves.
//!
//! ```
//! use kernix_core::{Order,shape::Kahen,tensor::Tensor};
//! use kernix_ops::linalg::{solve,Lu,Cholesky,Qr};
//! let a=Tensor::from_vec(vec![4.0f64,1.0,1.0,3.0],Kahen::new([2,2])?,Order::C)?;
//! let b=Tensor::from_vec(vec![1.0,2.0],Kahen::new([2])?,Order::C)?;
//! let x=solve(&a,&b)?;
//! assert!((x.storage_slice()[0]-1.0/11.0).abs()<1e-14);
//! let lu=Lu::factor(&a)?;assert!((lu.determinant()?-11.0).abs()<1e-14);
//! let chol=Cholesky::factor(&a)?;let other=chol.solve(&b)?;
//! assert!((other.storage_slice()[1]-7.0/11.0).abs()<1e-14);
//! let qr=Qr::factor(&a)?;assert_eq!(qr.q().shape().dims(),&[2,2]);
//! # Ok::<(),Box<dyn std::error::Error>>(())
//! ```
use crate::{FloatMath, MatmulElement, OpsError, TensorInput};
use alloc::{vec, vec::Vec};
use core::mem::align_of;
use kernix_core::{
    Order,
    shape::{Kahen, ShapeError},
    tensor::{Tensor, TensorError},
};
use thiserror::Error;

mod pivoted_qr;
pub use pivoted_qr::PivotedQr;
mod blocked_cholesky;
mod blocked_qr;
mod cholesky_update;
mod rz;
mod triangular;

mod diagnostics;
pub use diagnostics::{
    RefinedSolution, RefinementTermination, condition_number_one, reciprocal_condition_number_one,
    solve_refined,
};

/// Floating element capability for finite dense decompositions (f32/f64 provided).
pub trait LinalgFloat: FloatMath + MatmulElement {
    /// Distance from one to the next representable value in this dtype.
    const EPSILON: Self;
}
impl LinalgFloat for f32 {
    const EPSILON: Self = f32::EPSILON;
}
impl LinalgFloat for f64 {
    const EPSILON: Self = f64::EPSILON;
}
/// Checked decomposition/solve failure, with a failing pivot when relevant.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum LinalgError {
    /// Shape geometry validation failed.
    #[error(transparent)]
    Shape(#[from] ShapeError),
    /// Tensor construction or indexing failed.
    #[error(transparent)]
    Tensor(#[from] TensorError),
    /// A numerical tensor operation failed.
    #[error(transparent)]
    Ops(#[from] OpsError),
    /// Rank, dimensions or tolerance violate this operation's contract.
    #[error("invalid linear algebra parameter: {0}")]
    Invalid(&'static str),
    /// A pivot is zero or below the caller's relative cutoff.
    #[error("singular/rank-deficient matrix at pivot {pivot}")]
    Singular {
        /// Zero-based diagonal pivot index.
        pivot: usize,
    },
    /// A symmetric matrix has a nonpositive Cholesky pivot.
    #[error("matrix is not positive definite at pivot {pivot}")]
    NotPositiveDefinite {
        /// Zero-based diagonal pivot index.
        pivot: usize,
    },
    /// Opposite-triangle values differ beyond the scaled symmetry tolerance.
    #[error("matrix is not symmetric at ({row},{column})")]
    NotSymmetric {
        /// Row of the offending off-diagonal value.
        row: usize,
        /// Column of the offending off-diagonal value.
        column: usize,
    },
    /// Input or a required intermediate/result is outside the finite dtype range.
    #[error("nonfinite input or unrepresentable linear algebra intermediate")]
    NonFinite,
}
fn check_finite<T: FloatMath>(data: &[T]) -> Result<(), LinalgError> {
    if data.iter().any(|x| !FloatMath::is_finite(*x)) {
        Err(LinalgError::NonFinite)
    } else {
        Ok(())
    }
}
fn copy_matrix<T: LinalgFloat, A: TensorInput<T>>(a: &A) -> Result<Tensor<T>, LinalgError> {
    if a.dims().len() != 2 {
        return Err(LinalgError::Invalid("rank-two matrix required"));
    }
    let a = if let Some(values) = a.as_c_contiguous_slice() {
        Tensor::from_vec(values.to_vec(), Kahen::new(a.dims())?, Order::C)?
    } else {
        crate::map(a, |v| *v)?
    };
    check_finite(a.storage_slice())?;
    Ok(a)
}
fn square<T: LinalgFloat>(a: &Tensor<T>) -> Result<usize, LinalgError> {
    let d = a.shape().dims();
    if d[0] != d[1] {
        Err(LinalgError::Invalid("square matrix required"))
    } else {
        Ok(d[0])
    }
}
fn max_abs<T: LinalgFloat>(data: &[T]) -> T {
    data.iter().fold(T::zero(), |m, &v| {
        let v = FloatMath::abs(v);
        if v > m { v } else { m }
    })
}
// Independent lanes remove the long dependency chain in reflector dot products.
// If regrouping overflows, retry the original ordered arithmetic before rejecting
// the factorization. Ordinary rounding can differ; decomposition tolerances apply.
fn reflector_dot<T: LinalgFloat>(head: T, v: &[T], x: &[T]) -> T {
    let mut sums = [T::zero(); 4];
    let mut vs = v.chunks_exact(4);
    let mut xs = x.chunks_exact(4);
    for (v, x) in vs.by_ref().zip(xs.by_ref()) {
        for lane in 0..4 {
            sums[lane] += v[lane] * x[lane];
        }
    }
    let mut dot = head + ((sums[0] + sums[1]) + (sums[2] + sums[3]));
    for (&v, &x) in vs.remainder().iter().zip(xs.remainder()) {
        dot += v * x;
    }
    if !FloatMath::is_finite(dot) {
        dot = head;
        for (&v, &x) in v.iter().zip(x) {
            dot += v * x;
        }
    }
    dot
}

fn apply_reflector<T: LinalgFloat>(column: &mut [T], reflector: &[T], tau: T) {
    let dot = reflector_dot(column[0], reflector, &column[1..]);
    let d = tau * dot;
    if FloatMath::is_finite(d) {
        column[0] = column[0] - d;
        for (&v, x) in reflector.iter().zip(&mut column[1..]) {
            *x = *x - v * d;
        }
    } else {
        // tau lies in [1,2]. Splitting its product retains representable results
        // when tau*dot would overflow, e.g. reflecting a column near f64::MAX.
        let second = (tau - T::one()) * dot;
        column[0] = (column[0] - dot) - second;
        for (&v, x) in reflector.iter().zip(&mut column[1..]) {
            *x = (*x - v * dot) - v * second;
        }
    }
}

fn rhs<T: LinalgFloat, B: TensorInput<T>>(
    b: &B,
    rows: usize,
) -> Result<(Vec<T>, usize, Kahen), LinalgError> {
    let dims = b.dims();
    if !(dims.len() == 1 || dims.len() == 2) || dims[0] != rows {
        return Err(LinalgError::Invalid(
            "RHS must have shape [rows] or [rows,columns]",
        ));
    }
    let shape = Kahen::new(dims)?;
    let columns = if dims.len() == 1 { 1 } else { dims[1] };
    if let Some(values) = b.as_c_contiguous_slice() {
        check_finite(values)?;
        return Ok((values.to_vec(), columns, shape));
    }
    let mut values = Vec::with_capacity(shape.numel());
    if shape.numel() != 0 {
        for row in 0..rows {
            for column in 0..columns {
                values.push(if dims.len() == 1 {
                    *b.get(&[row])?
                } else {
                    *b.get(&[row, column])?
                });
            }
        }
    }
    check_finite(&values)?;
    Ok((values, columns, shape))
}
/// Allocate a C-order floating identity matrix. The empty 0x0 identity is valid.
///
/// ```
/// assert_eq!(kernix_ops::linalg::eye::<f64>(2)?.storage_slice(),&[1.0,0.0,0.0,1.0]);
/// # Ok::<(),kernix_ops::linalg::LinalgError>(())
/// ```
pub fn eye<T: LinalgFloat>(n: usize) -> Result<Tensor<T>, LinalgError> {
    Ok(Tensor::from_fn(
        Kahen::new([n, n])?,
        Order::C,
        align_of::<T>(),
        |i| if i[0] == i[1] { T::one() } else { T::zero() },
    )?)
}
/// Reusable partial-pivoted square LU: P*A = L*U, with unit diagonal L.
/// Inputs are copied once into packed C-order factors. Pivot cutoff is relative
/// to the maximum absolute input entry; it is a rejection policy, not a condition estimate.
pub struct Lu<T: LinalgFloat> {
    packed: Tensor<T>,
    permutation: Vec<usize>,
    odd: bool,
}
impl<T: LinalgFloat> Lu<T> {
    /// Factor a finite square matrix using partial pivoting and relative cutoff
    /// epsilon*max(n,1). Near-singular matrices can be rejected; use factor_with_tolerance
    /// to choose an explicit policy. See the module's solve/determinant example.
    pub fn factor<A: TensorInput<T>>(input: &A) -> Result<Self, LinalgError> {
        let n = input.dims().first().copied().unwrap_or(0);
        Self::factor_with_tolerance(input, T::EPSILON * T::from_count(n.max(1)))
    }
    /// Factor with finite relative pivot cutoff in [0,1). Zero rejects exact zero
    /// pivots only; callers remain responsible for interpreting ill-conditioned results.
    /// O(n^3) time and O(n^2) copied storage. Arbitrary input strides are respected.
    pub fn factor_with_tolerance<A: TensorInput<T>>(
        input: &A,
        relative_cutoff: T,
    ) -> Result<Self, LinalgError> {
        if !FloatMath::is_finite(relative_cutoff)
            || relative_cutoff < T::zero()
            || relative_cutoff >= T::one()
        {
            return Err(LinalgError::Invalid(
                "relative pivot cutoff must be in [0,1)",
            ));
        }
        let mut packed = copy_matrix(input)?;
        let n = square(&packed)?;
        let cutoff = relative_cutoff * max_abs(packed.storage_slice());
        let mut permutation: Vec<usize> = (0..n).collect();
        let mut odd = false;
        let data = packed.storage_slice_mut();
        for k in 0..n {
            let mut pivot = k;
            for row in k + 1..n {
                if FloatMath::abs(data[row * n + k]) > FloatMath::abs(data[pivot * n + k]) {
                    pivot = row;
                }
            }
            if FloatMath::abs(data[pivot * n + k]) <= cutoff {
                return Err(LinalgError::Singular { pivot: k });
            }
            if pivot != k {
                for j in 0..n {
                    data.swap(k * n + j, pivot * n + j);
                }
                permutation.swap(k, pivot);
                odd = !odd;
            }
            for row in k + 1..n {
                let multiplier = data[row * n + k] / data[k * n + k];
                let (before, tail) = data.split_at_mut(row * n);
                let pivot_row = &before[k * n..(k + 1) * n];
                let row = &mut tail[..n];
                row[k] = multiplier;
                for (value, &p) in row[k + 1..].iter_mut().zip(&pivot_row[k + 1..]) {
                    *value = *value - multiplier * p;
                }
            }
        }
        check_finite(data)?;
        Ok(Self {
            packed,
            permutation,
            odd,
        })
    }
    /// Borrow packed lower/upper factors: diagonal belongs to U; L diagonal is implicit one.
    pub fn packed(&self) -> &Tensor<T> {
        &self.packed
    }
    /// Source row at each row of P*A. Permutation storage is immutable.
    pub fn permutation(&self) -> &[usize] {
        &self.permutation
    }
    /// Solve for one or many RHS columns using the existing factors. RHS shapes are
    /// `[n]` or `[n,k]`; output retains that shape. O(n^2*k), no explicit inverse needed.
    pub fn solve<B: TensorInput<T>>(&self, b: &B) -> Result<Tensor<T>, LinalgError> {
        let n = self.permutation.len();
        let (source, columns, shape) = rhs(b, n)?;
        let mut out = vec![T::zero(); source.len()];
        if !out.is_empty() {
            for row in 0..n {
                out[row * columns..(row + 1) * columns].copy_from_slice(
                    &source[self.permutation[row] * columns..(self.permutation[row] + 1) * columns],
                );
            }
            let a = self.packed.storage_slice();
            if columns <= 4 {
                for row in 0..n {
                    for j in 0..columns {
                        let mut value = out[row * columns + j];
                        for k in 0..row {
                            value = value - a[row * n + k] * out[k * columns + j];
                        }
                        out[row * columns + j] = value;
                    }
                }
                for row in (0..n).rev() {
                    for j in 0..columns {
                        let mut value = out[row * columns + j];
                        for k in row + 1..n {
                            value = value - a[row * n + k] * out[k * columns + j];
                        }
                        out[row * columns + j] = value / a[row * n + row];
                    }
                }
            } else {
                triangular::solve(&mut out, n, columns, true, true, |i, j| a[i * n + j]);
                triangular::solve(&mut out, n, columns, false, false, |i, j| a[i * n + j]);
            }
        }
        check_finite(&out)?;
        Ok(Tensor::from_vec(out, shape, Order::C)?)
    }
    /// Compute the determinant from pivot parity and U diagonal. Empty determinant
    /// is one; an unrepresentable product returns NonFinite. Prefer slogdet for scale.
    pub fn determinant(&self) -> Result<T, LinalgError> {
        let n = self.permutation.len();
        let mut value = if self.odd { -T::one() } else { T::one() };
        for i in 0..n {
            value = value * self.packed.storage_slice()[i * n + i];
        }
        if !FloatMath::is_finite(value) {
            return Err(LinalgError::NonFinite);
        }
        Ok(value)
    }
    /// Return determinant sign (+/-1) and log(abs(det)) without multiplying pivots.
    /// Empty determinant has sign one and log magnitude zero.
    pub fn slogdet(&self) -> (T, T) {
        let n = self.permutation.len();
        let mut sign = if self.odd { -T::one() } else { T::one() };
        let mut log = T::zero();
        for i in 0..n {
            let d = self.packed.storage_slice()[i * n + i];
            if d < T::zero() {
                sign = -sign;
            }
            log += FloatMath::ln(FloatMath::abs(d));
        }
        (sign, log)
    }
    /// Solve against the identity to allocate an inverse. Prefer solve for ordinary
    /// linear systems: forming an inverse costs additional work/storage.
    pub fn inverse(&self) -> Result<Tensor<T>, LinalgError> {
        self.solve(&eye(self.permutation.len())?)
    }
}
/// Solve a finite square linear system using reusable partial-pivoted LU internally.
/// RHS shapes are `[n]` or `[n,k]`. See the module's worked 2x2 example.
pub fn solve<T: LinalgFloat, A: TensorInput<T>, B: TensorInput<T>>(
    a: &A,
    b: &B,
) -> Result<Tensor<T>, LinalgError> {
    Lu::factor(a)?.solve(b)
}
/// Compute a square matrix inverse through pivoted solves. Empty inverse is 0x0.
/// Singular/near-singular matrices return an explicit pivot error.
pub fn inverse<T: LinalgFloat, A: TensorInput<T>>(a: &A) -> Result<Tensor<T>, LinalgError> {
    Lu::factor(a)?.inverse()
}
/// Determinant with exact-zero pivot rejection (no near-singular cutoff). Singular
/// matrices return zero. Products outside the finite dtype range return an error.
pub fn determinant<T: LinalgFloat, A: TensorInput<T>>(a: &A) -> Result<T, LinalgError> {
    match Lu::factor_with_tolerance(a, T::zero()) {
        Ok(lu) => lu.determinant(),
        Err(LinalgError::Singular { .. }) => Ok(T::zero()),
        Err(error) => Err(error),
    }
}
/// Lower Cholesky factor L for a finite symmetric positive-definite matrix A=L*L^T.
/// Opposite triangles must agree within epsilon*max(n,1)*max(abs(A)); within that
/// tolerance the lower triangle is authoritative. No hidden diagonal jitter is added.
pub struct Cholesky<T: LinalgFloat> {
    lower: Tensor<T>,
}
impl<T: LinalgFloat> Cholesky<T> {
    /// Factor a finite symmetric positive-definite matrix. Nonpositive pivots and
    /// nonsymmetric inputs return contextual errors. O(n^3) time, O(n^2) storage.
    pub fn factor<A: TensorInput<T>>(input: &A) -> Result<Self, LinalgError> {
        let mut a = copy_matrix(input)?;
        let n = square(&a)?;
        let data = a.storage_slice();
        let tol = (T::EPSILON * T::from_count(n.max(1))) * max_abs(data);
        for row in 0..n {
            for col in 0..row {
                if FloatMath::abs(data[row * n + col] - data[col * n + row]) > tol {
                    return Err(LinalgError::NotSymmetric { row, column: col });
                }
            }
        }
        // Reuse the checked input copy as output. Large native-capable matrices
        // use bounded left-looking block updates; smaller matrices retain their
        // column/row kernels. Every range retry retains the original current entry.
        let data = a.storage_slice_mut();
        if n >= 192 && T::HAS_NATIVE_MATMUL {
            blocked_cholesky::factor(data, n)?;
        } else if n <= 96 && n != 0 {
            let mut workspace = Tensor::<T>::zeros([n])?;
            let column = workspace.storage_slice_mut();
            for k in 0..n {
                let value = data[k * n + k];
                if value <= T::zero() {
                    return Err(LinalgError::NotPositiveDefinite { pivot: k });
                }
                let diagonal = FloatMath::sqrt(value);
                data[k * n + k] = diagonal;
                for row in k + 1..n {
                    column[row] = data[row * n + k] / diagonal;
                    data[row * n + k] = column[row];
                }
                for row in k + 1..n {
                    let multiplier = column[row];
                    for (value, &other) in data[row * n + k + 1..row * n + row + 1]
                        .iter_mut()
                        .zip(&column[k + 1..row + 1])
                    {
                        *value = *value - multiplier * other;
                    }
                }
            }
        } else {
            for row in 0..n {
                let (previous, rest) = data.split_at_mut(row * n);
                let current = &mut rest[..n];
                for col in 0..=row {
                    let other = if row == col {
                        &current[..col]
                    } else {
                        &previous[col * n..col * n + col]
                    };
                    let original = current[col];
                    let mut value = original - reflector_dot(T::zero(), &current[..col], other);
                    if !FloatMath::is_finite(value) || (row == col && value <= T::zero()) {
                        value = original;
                        for (&x, &y) in current[..col].iter().zip(other) {
                            value = value - x * y;
                        }
                    }
                    if row == col {
                        if value <= T::zero() {
                            return Err(LinalgError::NotPositiveDefinite { pivot: row });
                        }
                        current[col] = FloatMath::sqrt(value);
                    } else {
                        current[col] = value / previous[col * n + col];
                    }
                }
            }
        }
        if n != 0 {
            for (i, row) in data.chunks_exact_mut(n).enumerate() {
                row[i + 1..].fill(T::zero());
            }
        }
        check_finite(data)?;
        Ok(Self { lower: a })
    }
    /// Borrow the C-order lower triangular factor (upper triangle is exactly zero).
    pub fn lower(&self) -> &Tensor<T> {
        &self.lower
    }
    /// Factor `A + x*x^T` from this factor of A, in O(n²) time.
    ///
    /// Returns a new factor; this factor and x remain unchanged, including on
    /// failure. x must be a finite rank-one tensor of length n; strided vectors
    /// are supported. Allocates an n×n factor and an n-element rotation workspace.
    /// Rotations avoid explicitly forming A or squaring large entries of x.
    /// Required intermediates and the resulting factor must remain finite.
    ///
    /// ```
    /// use kernix_core::tensor::Tensor;
    /// use kernix_ops::linalg::Cholesky;
    /// let a=Tensor::from_rows([[2.0f64,0.0],[0.0,3.0]])?;
    /// let factor=Cholesky::factor(&a)?;
    /// let updated=factor.rank_one_update(&Tensor::from_values([2],vec![1.0,0.0])?)?;
    /// let x=updated.solve(&Tensor::from_values([2],vec![3.0,6.0])?)?;
    /// assert!((x.storage_slice()[0]-1.0).abs()<1e-14);
    /// assert!((x.storage_slice()[1]-2.0).abs()<1e-14);
    /// # Ok::<(),Box<dyn std::error::Error>>(())
    /// ```
    pub fn rank_one_update<V: TensorInput<T>>(&self, x: &V) -> Result<Self, LinalgError> {
        Ok(Self {
            lower: cholesky_update::modify(&self.lower, x, false)?,
        })
    }
    /// Factor `A - x*x^T` in O(n²), rejecting nonpositive pivots explicitly.
    ///
    /// Returns a new factor and leaves both inputs unchanged on every error.
    /// x must be a finite vector of length n. Allocates an n×n output and
    /// n-element workspace. Near the positive-definite boundary, rounding and
    /// conditioning can limit accuracy or cause rejection; no jitter is added.
    /// Nonfinite or unrepresentable intermediates return `LinalgError::NonFinite`.
    pub fn rank_one_downdate<V: TensorInput<T>>(&self, x: &V) -> Result<Self, LinalgError> {
        Ok(Self {
            lower: cholesky_update::modify(&self.lower, x, true)?,
        })
    }
    /// Solve one or multiple RHS columns using L followed by L^T. O(n^2*k).
    pub fn solve<B: TensorInput<T>>(&self, b: &B) -> Result<Tensor<T>, LinalgError> {
        let n = self.lower.shape().dims()[0];
        let (mut out, columns, shape) = rhs(b, n)?;
        let l = self.lower.storage_slice();
        if columns <= 4 {
            if !out.is_empty() {
                for row in 0..n {
                    for j in 0..columns {
                        let mut value = out[row * columns + j];
                        for k in 0..row {
                            value = value - l[row * n + k] * out[k * columns + j];
                        }
                        out[row * columns + j] = value / l[row * n + row];
                    }
                }
                for row in (0..n).rev() {
                    for j in 0..columns {
                        let mut value = out[row * columns + j];
                        for k in row + 1..n {
                            value = value - l[k * n + row] * out[k * columns + j];
                        }
                        out[row * columns + j] = value / l[row * n + row];
                    }
                }
            }
        } else {
            triangular::solve(&mut out, n, columns, true, false, |i, j| l[i * n + j]);
            triangular::solve(&mut out, n, columns, false, false, |i, j| l[j * n + i]);
        }
        check_finite(&out)?;
        Ok(Tensor::from_vec(out, shape, Order::C)?)
    }
}
#[inline(always)]
fn qr_column<T: LinalgFloat>(
    packed: &mut [T],
    m: usize,
    k: usize,
    target_end: usize,
    tau: &mut T,
) -> Result<(), LinalgError> {
    let mut s = T::zero();
    let mut ssq = T::one();
    for &value in &packed[k * m + k..(k + 1) * m] {
        let v = FloatMath::abs(value);
        if v != T::zero() {
            if s < v {
                let ratio = s / v;
                ssq = T::one() + ssq * ratio * ratio;
                s = v;
            } else {
                let ratio = v / s;
                ssq += ratio * ratio;
            }
        }
    }
    if s == T::zero() {
        return Ok(());
    }
    let unit_norm = FloatMath::sqrt(ssq);
    let norm = s * unit_norm;
    if !FloatMath::is_finite(norm) {
        return Err(LinalgError::NonFinite);
    }
    let a0 = packed[k * m + k];
    let sign = if a0 >= T::zero() { T::one() } else { -T::one() };
    // A rounded subnormal norm can corrupt a dimensionless reflector.
    let weak_norm = norm * T::EPSILON == T::zero();
    let ratio = if weak_norm {
        (a0 / s) / unit_norm
    } else {
        a0 / norm
    };
    let denom = ratio + sign;
    *tau = T::one() + sign * ratio;
    if weak_norm {
        for value in &mut packed[k * m + k + 1..(k + 1) * m] {
            *value = ((*value / s) / unit_norm) / denom;
        }
    } else {
        for value in &mut packed[k * m + k + 1..(k + 1) * m] {
            *value = (*value / norm) / denom;
        }
    }
    packed[k * m + k] = -sign * norm;
    let (done, remaining) = packed[..target_end * m].split_at_mut((k + 1) * m);
    let reflector = &done[k * m + k + 1..(k + 1) * m];
    for column in remaining.chunks_exact_mut(m) {
        apply_reflector(&mut column[k..], reflector, *tau);
    }
    Ok(())
}

/// Thin Householder QR: Q has shape `[m,min(m,n)]`, R `[min(m,n),n]`.
/// Orthogonal reflectors are preferable to normal equations for least-squares accuracy.
/// Input is copied; finite intermediate/output checks reject unrepresentable ranges.
/// Reflector dot products use independent accumulation lanes. Rounding can differ
/// from sequential arithmetic; no bitwise agreement with previous releases is promised.
pub struct Qr<T: LinalgFloat> {
    q: Tensor<T>,
    r: Tensor<T>,
    cutoff: T,
}
impl<T: LinalgFloat> Qr<T> {
    /// Factor a finite rectangular matrix using stable-sign Householder reflectors.
    /// Zero columns are supported. O(m*n*min(m,n)) time; thin Q/R are materialized.
    pub fn factor<A: TensorInput<T>>(input: &A) -> Result<Self, LinalgError> {
        let dims = input.dims();
        if dims.len() != 2 {
            return Err(LinalgError::Invalid("rank-two matrix required"));
        }
        let shape = Kahen::new(dims)?;
        let (m, n) = (dims[0], dims[1]);
        let rank = m.min(n);
        let contiguous = input.as_c_contiguous_slice();
        if let Some(values) = contiguous {
            if values.len() != shape.numel() {
                return Err(LinalgError::Invalid(
                    "contiguous capability length mismatch",
                ));
            }
        }
        if rank == 0 {
            // Avoid dimension-sized loops on valid empty [0,isize::MAX] inputs.
            return Ok(Self {
                q: Tensor::zeros([m, 0])?,
                r: Tensor::zeros([0, n])?,
                cutoff: T::zero(),
            });
        }
        // Pack directly from the validated capability, without a temporary C copy.
        let mut packed = Tensor::<T>::zeros([n, m])?.into_storage_vec();
        if let Some(values) = contiguous {
            check_finite(values)?;
            for (col, column) in packed.chunks_exact_mut(m).enumerate() {
                for (row, value) in column.iter_mut().enumerate() {
                    *value = values[row * n + col];
                }
            }
        } else {
            for (col, column) in packed.chunks_exact_mut(m).enumerate() {
                for (row, value) in column.iter_mut().enumerate() {
                    *value = *input.get(&[row, col])?;
                }
            }
            check_finite(&packed)?;
        }
        let scale = max_abs(&packed);
        let cutoff = (T::EPSILON * T::from_count(m.max(n).max(1))) * scale;
        let mut tau = vec![T::zero(); rank];
        let blocked = T::HAS_NATIVE_MATMUL && m >= 192 && n >= 128 && m >= n;
        let mut workspace = if blocked {
            Some(blocked_qr::Workspace::new(m, n)?)
        } else {
            None
        };
        let mut q_output = if blocked {
            Tensor::<T>::zeros([m, rank])?.into_storage_vec()
        } else {
            Vec::new()
        };
        let block = if blocked { blocked_qr::BLOCK } else { 1 };
        for start in (0..rank).step_by(block) {
            let end = (start + block).min(rank);
            for (k, tau_k) in tau.iter_mut().enumerate().take(end).skip(start) {
                qr_column(&mut packed, m, k, if blocked { end } else { n }, tau_k)?;
            }
            if let Some(work) = workspace.as_mut() {
                let prepared = work.prepare(&packed, &tau, start, end, false);
                if end == n {
                    continue;
                }
                let input = crate::MatrixInput::new(
                    &packed,
                    [m - start, n - end],
                    [1, m as isize],
                    end * m + start,
                )?;
                let applied = prepared
                    && work.product(input, n - end, true, &mut q_output)?
                    && blocked_qr::commit(&mut packed, m, start, end, n - end, &mut q_output);
                if !applied {
                    for (k, &tau_k) in tau.iter().enumerate().take(end).skip(start) {
                        let (done, remaining) = packed.split_at_mut(end * m);
                        let reflector = &done[k * m + k + 1..(k + 1) * m];
                        for column in remaining.chunks_exact_mut(m) {
                            apply_reflector(&mut column[k..], reflector, tau_k);
                        }
                    }
                }
            }
        }
        check_finite(&packed)?;
        let mut q = vec![T::zero(); Kahen::new([m, rank])?.numel()];
        for i in 0..rank {
            q[i * m + i] = T::one();
        }
        for start in (0..rank).step_by(block).rev() {
            let end = (start + block).min(rank);
            let mut applied = false;
            if let Some(work) = workspace.as_mut() {
                let prepared = work.prepare(&packed, &tau, start, end, true);
                let input = crate::MatrixInput::new(
                    &q,
                    [m - start, rank - start],
                    [1, m as isize],
                    start * m + start,
                )?;
                applied = prepared
                    && work.product(input, rank - start, false, &mut q_output)?
                    && blocked_qr::commit(&mut q, m, start, start, rank - start, &mut q_output);
            }
            if !applied {
                for k in (start..end).rev() {
                    let reflector = &packed[k * m + k + 1..(k + 1) * m];
                    for column in q.chunks_exact_mut(m).skip(k) {
                        apply_reflector(&mut column[k..], reflector, tau[k]);
                    }
                }
            }
        }
        drop(workspace);
        let mut r = vec![T::zero(); Kahen::new([rank, n])?.numel()];
        for row in 0..rank {
            for col in row..n {
                r[row * n + col] = packed[col * m + row];
            }
        }
        let q_columns = q;
        let mut q = if blocked {
            q_output
        } else {
            vec![T::zero(); q_columns.len()]
        };
        for first_row in (0..m).step_by(8) {
            for first_col in (0..rank).step_by(16) {
                for col in first_col..(first_col + 16).min(rank) {
                    for row in first_row..(first_row + 8).min(m) {
                        q[row * rank + col] = q_columns[col * m + row];
                    }
                }
            }
        }
        check_finite(&q)?;
        check_finite(&r)?;
        Ok(Self {
            q: Tensor::from_vec(q, Kahen::new([m, rank])?, Order::C)?,
            r: Tensor::from_vec(r, Kahen::new([rank, n])?, Order::C)?,
            cutoff,
        })
    }
    /// Borrow thin orthonormal Q. No allocation occurs in this inspector.
    pub fn q(&self) -> &Tensor<T> {
        &self.q
    }
    /// Borrow upper trapezoidal R. No allocation occurs in this inspector.
    pub fn r(&self) -> &Tensor<T> {
        &self.r
    }
    /// Solve a full-column-rank least-squares system with m>=n. Under-determined or
    /// rank-deficient systems fail explicitly; use PivotedQr for minimum-norm solving.
    /// RHS shapes are `[m]` or `[m,k]`; output is `[n]` or `[n,k]`.
    pub fn solve_least_squares<B: TensorInput<T>>(&self, b: &B) -> Result<Tensor<T>, LinalgError> {
        let (m, n) = (self.q.shape().dims()[0], self.r.shape().dims()[1]);
        if m < n {
            return Err(LinalgError::Invalid(
                "least squares requires rows >= columns",
            ));
        }
        rhs(b, m)?;
        for i in 0..n {
            if FloatMath::abs(self.r.storage_slice()[i * n + i]) <= self.cutoff {
                return Err(LinalgError::Singular { pivot: i });
            }
        }
        let mut out = crate::matmul(&self.q.transpose()?, b)?;
        let columns = if b.dims().len() == 1 { 1 } else { b.dims()[1] };
        let r = self.r.storage_slice();
        let values = out.storage_slice_mut();
        if columns <= 4 {
            if !values.is_empty() {
                for row in (0..n).rev() {
                    for j in 0..columns {
                        let mut value = values[row * columns + j];
                        for k in row + 1..n {
                            value = value - r[row * n + k] * values[k * columns + j];
                        }
                        values[row * columns + j] = value / r[row * n + row];
                    }
                }
            }
        } else {
            triangular::solve(values, n, columns, false, false, |i, j| r[i * n + j]);
        }
        check_finite(values)?;
        Ok(out)
    }
}
/// Rectangular full-column-rank least squares using Householder QR, avoiding A^T*A.
/// Under-determined/rank-deficient systems return errors; see Qr::solve_least_squares.
pub fn least_squares<T: LinalgFloat, A: TensorInput<T>, B: TensorInput<T>>(
    a: &A,
    b: &B,
) -> Result<Tensor<T>, LinalgError> {
    Qr::factor(a)?.solve_least_squares(b)
}
/// Stable L2 norm of A*x-b for explicitly matching output/RHS shapes. This is an
/// absolute residual, not a condition number or a guarantee of small forward error.
pub fn residual_norm<T: LinalgFloat, A: TensorInput<T>, X: TensorInput<T>, B: TensorInput<T>>(
    a: &A,
    x: &X,
    b: &B,
) -> Result<T, LinalgError> {
    let product = crate::matmul(a, x)?;
    if product.shape().dims() != b.dims() {
        return Err(LinalgError::Invalid("residual RHS shape mismatch"));
    }
    Ok(crate::l2_norm_all(&crate::sub(&product, b)?)?)
}

#[cfg(test)]
mod reflector_tests {
    #[test]
    fn regrouping_overflow_retries_ordered_cancellation() {
        let x = [
            f64::MAX,
            -f64::MAX,
            f64::MAX,
            -f64::MAX,
            f64::MAX,
            -f64::MAX,
            f64::MAX,
            -f64::MAX,
        ];
        assert_eq!(super::reflector_dot(0.0, &[1.0; 8], &x), 0.0);
    }
}
