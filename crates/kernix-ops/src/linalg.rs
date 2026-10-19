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
        let a = copy_matrix(input)?;
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
        let mut lower = vec![T::zero(); a.numel()];
        for row in 0..n {
            for col in 0..=row {
                let mut value = data[row * n + col];
                for (&x, &y) in lower[row * n..row * n + col]
                    .iter()
                    .zip(&lower[col * n..col * n + col])
                {
                    value = value - x * y;
                }
                if row == col {
                    if value <= T::zero() {
                        return Err(LinalgError::NotPositiveDefinite { pivot: row });
                    }
                    lower[row * n + col] = FloatMath::sqrt(value);
                } else {
                    lower[row * n + col] = value / lower[col * n + col];
                }
            }
        }
        check_finite(&lower)?;
        Ok(Self {
            lower: Tensor::from_vec(lower, Kahen::new([n, n])?, Order::C)?,
        })
    }
    /// Borrow the C-order lower triangular factor (upper triangle is exactly zero).
    pub fn lower(&self) -> &Tensor<T> {
        &self.lower
    }
    /// Solve one or multiple RHS columns using L followed by L^T. O(n^2*k).
    pub fn solve<B: TensorInput<T>>(&self, b: &B) -> Result<Tensor<T>, LinalgError> {
        let n = self.lower.shape().dims()[0];
        let (mut out, columns, shape) = rhs(b, n)?;
        let l = self.lower.storage_slice();
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
        check_finite(&out)?;
        Ok(Tensor::from_vec(out, shape, Order::C)?)
    }
}
/// Thin Householder QR: Q has shape `[m,min(m,n)]`, R `[min(m,n),n]`.
/// Orthogonal reflectors are preferable to normal equations for least-squares accuracy.
/// Input is copied; finite intermediate/output checks reject unrepresentable ranges.
pub struct Qr<T: LinalgFloat> {
    q: Tensor<T>,
    r: Tensor<T>,
    cutoff: T,
}
impl<T: LinalgFloat> Qr<T> {
    /// Factor a finite rectangular matrix using stable-sign Householder reflectors.
    /// Zero columns are supported. O(m*n*min(m,n)) time; thin Q/R are materialized.
    pub fn factor<A: TensorInput<T>>(input: &A) -> Result<Self, LinalgError> {
        let a = copy_matrix(input)?;
        let dims = a.shape().dims();
        let (m, n) = (dims[0], dims[1]);
        let rank = m.min(n);
        // Reflectors traverse columns; pack column-major to keep those scans contiguous.
        let input = a.storage_slice();
        let mut packed = vec![T::zero(); input.len()];
        for col in 0..n {
            for row in 0..m {
                packed[col * m + row] = input[row * n + col];
            }
        }
        let scale = max_abs(&packed);
        let cutoff = (T::EPSILON * T::from_count(m.max(n).max(1))) * scale;
        let mut tau = vec![T::zero(); rank];
        for k in 0..rank {
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
                continue;
            }
            let norm = s * FloatMath::sqrt(ssq);
            if !FloatMath::is_finite(norm) {
                return Err(LinalgError::NonFinite);
            }
            let a0 = packed[k * m + k];
            let sign = if a0 >= T::zero() { T::one() } else { -T::one() };
            let ratio = a0 / norm;
            let denom = ratio + sign;
            tau[k] = T::one() + sign * ratio;
            for value in &mut packed[k * m + k + 1..(k + 1) * m] {
                *value = (*value / norm) / denom;
            }
            packed[k * m + k] = -sign * norm;
            let (done, remaining) = packed.split_at_mut((k + 1) * m);
            let reflector = &done[k * m + k + 1..(k + 1) * m];
            for column in remaining.chunks_exact_mut(m) {
                let mut dot = column[k];
                for (&v, &x) in reflector.iter().zip(&column[k + 1..]) {
                    dot += v * x;
                }
                let second = (tau[k] - T::one()) * dot;
                column[k] = (column[k] - dot) - second;
                for (&v, x) in reflector.iter().zip(&mut column[k + 1..]) {
                    *x = (*x - v * dot) - v * second;
                }
            }
        }
        check_finite(&packed)?;
        let mut q = vec![T::zero(); Kahen::new([m, rank])?.numel()];
        for i in 0..rank {
            q[i * m + i] = T::one();
        }
        for k in (0..rank).rev() {
            let reflector = &packed[k * m + k + 1..(k + 1) * m];
            for column in q.chunks_exact_mut(m) {
                let mut dot = column[k];
                for (&v, &x) in reflector.iter().zip(&column[k + 1..]) {
                    dot += v * x;
                }
                let d = tau[k] * dot;
                column[k] = column[k] - d;
                for (&v, x) in reflector.iter().zip(&mut column[k + 1..]) {
                    *x = *x - v * d;
                }
            }
        }
        let mut r = vec![T::zero(); Kahen::new([rank, n])?.numel()];
        for row in 0..rank {
            for col in row..n {
                r[row * n + col] = packed[col * m + row];
            }
        }
        let q_columns = q;
        let mut q = vec![T::zero(); q_columns.len()];
        for col in 0..rank {
            for row in 0..m {
                q[row * rank + col] = q_columns[col * m + row];
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
    /// rank-deficient systems fail explicitly; minimum-norm/SVD solving is future work.
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
