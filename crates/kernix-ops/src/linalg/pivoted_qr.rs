//! Column-pivoted QR and minimum-norm solves without normal equations.
use super::{LinalgError, LinalgFloat, Qr, apply_reflector, check_finite, max_abs};
use crate::{FloatMath, TensorInput};
use alloc::vec::Vec;
use kernix_core::{shape::Kahen, tensor::Tensor};

/// Thin Householder QR with greedy column pivoting: `A[:, permutation] = Q * R`.
///
/// Q is `[m,min(m,n)]`, R is `[min(m,n),n]`; factors reconstruct the original
/// matrix independently of the rank cutoff. The permutation maps factor columns
/// to original columns. Tied norm estimates select the first remaining column.
///
/// Numerical rank is the leading run of diagonal magnitudes exceeding
/// `relative_tolerance * max(abs(diag(R)))`. This is a diagonal rejection policy,
/// not an SVD rank, condition estimate or strong rank-revealing guarantee.
/// Solves use the rank-truncated matrix `Q[:, :rank] * R[:rank, :] * P^T`.
///
/// Input is copied; results are C-order. Pivot selection uses partial column-norm
/// estimates, with full scaled recomputation when the retained squared-norm
/// fraction reaches the sqrt(epsilon) reliability cutoff. Every reflector uses
/// a freshly computed norm. Factorization costs O(m*n*min(m,n) + n) time
/// and O(m*n + m*min(m,n) + n) workspace/output. Empty wide matrices still need
/// an O(n) permutation and can fail resource validation. All required values
/// must remain finite. Arithmetic/reassociation can differ from unpivoted QR.
pub struct PivotedQr<T: LinalgFloat> {
    q: Tensor<T>,
    r: Tensor<T>,
    permutation: Vec<usize>,
    rank: usize,
    relative_tolerance: T,
}

// Scaled sum of squares avoids overflow/underflow of unscaled squares.
fn column_norm<T: LinalgFloat>(column: &[T]) -> Result<T, LinalgError> {
    let scale = max_abs(column);
    if scale == T::zero() {
        return Ok(T::zero());
    }
    let mut lanes = [T::zero(); 4];
    let mut chunks = column.chunks_exact(4);
    for values in chunks.by_ref() {
        for i in 0..4 {
            let ratio = values[i] / scale;
            lanes[i] += ratio * ratio;
        }
    }
    let mut sum = (lanes[0] + lanes[1]) + (lanes[2] + lanes[3]);
    for &value in chunks.remainder() {
        let ratio = value / scale;
        sum += ratio * ratio;
    }
    let norm = scale * FloatMath::sqrt(sum);
    if !FloatMath::is_finite(norm) {
        return Err(LinalgError::NonFinite);
    }
    Ok(norm)
}

impl<T: LinalgFloat> PivotedQr<T> {
    /// Factor using relative diagonal cutoff `epsilon * max(m,n,1)`.
    /// Use factor_with_tolerance to choose a numerical-rank policy explicitly.
    ///
    /// ```
    /// use kernix_core::tensor::Tensor;
    /// use kernix_ops::linalg::PivotedQr;
    /// let a=Tensor::from_values([2,3],vec![1.0f64,0.0,1.0,0.0,1.0,1.0])?;
    /// let qr=PivotedQr::factor(&a)?;
    /// assert_eq!(qr.rank(),2);
    /// let b=Tensor::from_values([2],vec![1.0,1.0])?;
    /// let x=qr.solve_least_squares(&b)?;
    /// assert!((x.storage_slice()[2]-2.0/3.0).abs()<1e-14);
    /// # Ok::<(),kernix_ops::linalg::LinalgError>(())
    /// ```
    pub fn factor<A: TensorInput<T>>(input: &A) -> Result<Self, LinalgError> {
        let dims = input.dims();
        if dims.len() != 2 {
            return Err(LinalgError::Invalid("rank-two matrix required"));
        }
        let size = dims[0].max(dims[1]).max(1);
        Self::factor_with_tolerance(input, T::EPSILON * T::from_count(size))
    }

    /// Factor with finite nonnegative relative tolerance. Zero rejects only exact
    /// zero diagonals; tolerance >=1 selects rank zero. Rank comparison uses ratios
    /// to avoid overflowing the cutoff product. No dtype conversion occurs.
    pub fn factor_with_tolerance<A: TensorInput<T>>(
        input: &A,
        tolerance: T,
    ) -> Result<Self, LinalgError> {
        if !FloatMath::is_finite(tolerance) || tolerance < T::zero() {
            return Err(LinalgError::Invalid(
                "relative rank tolerance must be finite and nonnegative",
            ));
        }
        let dims = input.dims();
        if dims.len() != 2 {
            return Err(LinalgError::Invalid("rank-two matrix required"));
        }
        let shape = Kahen::new(dims)?;
        let (m, n) = (dims[0], dims[1]);
        let thin = m.min(n);
        let contiguous = input.as_c_contiguous_slice();
        if contiguous.is_some_and(|v| v.len() != shape.numel()) {
            return Err(LinalgError::Invalid(
                "contiguous capability length mismatch",
            ));
        }
        // Validate/materialize every owned extent through fallible Tensor allocation.
        let mut q = Tensor::<T>::zeros([m, thin])?;
        let mut r = Tensor::<T>::zeros([thin, n])?;
        let mut permutation = Tensor::<usize>::zeros([n])?.into_storage_vec();
        for (i, p) in permutation.iter_mut().enumerate() {
            *p = i;
        }
        if thin == 0 {
            return Ok(Self {
                q,
                r,
                permutation,
                rank: 0,
                relative_tolerance: tolerance,
            });
        }
        let mut packed = Tensor::<T>::zeros([n, m])?.into_storage_vec();
        if let Some(values) = contiguous {
            check_finite(values)?;
            for (j, col) in packed.chunks_exact_mut(m).enumerate() {
                for (i, value) in col.iter_mut().enumerate() {
                    *value = values[i * n + j];
                }
            }
        } else {
            for (j, col) in packed.chunks_exact_mut(m).enumerate() {
                for (i, value) in col.iter_mut().enumerate() {
                    *value = *input.get(&[i, j])?;
                }
            }
            check_finite(&packed)?;
        }
        let mut tau = Tensor::<T>::zeros([thin])?.into_storage_vec();
        let mut norms = Tensor::<T>::zeros([n])?.into_storage_vec();
        let mut reference = Tensor::<T>::zeros([n])?.into_storage_vec();
        for (j, col) in packed.chunks_exact(m).enumerate() {
            norms[j] = column_norm(col)?;
            reference[j] = norms[j];
        }
        let loss_limit = FloatMath::sqrt(T::EPSILON);
        for k in 0..thin {
            let mut pivot = k;
            for j in k + 1..n {
                if norms[j] > norms[pivot] {
                    pivot = j;
                }
            }
            if pivot != k {
                for row in 0..m {
                    packed.swap(k * m + row, pivot * m + row);
                }
                permutation.swap(k, pivot);
                norms.swap(k, pivot);
                reference.swap(k, pivot);
            }
            // Estimates choose the pivot; its actual scaled norm forms the reflector.
            let norm = column_norm(&packed[k * m + k..(k + 1) * m])?;
            if norm == T::zero() {
                continue;
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
            for (offset, column) in remaining.chunks_exact_mut(m).enumerate() {
                apply_reflector(&mut column[k..], reflector, tau[k]);
                let j = k + 1 + offset;
                if norms[j] != T::zero() {
                    // Remove the eliminated component from the current estimate.
                    // Recompute when subtraction/cumulative loss becomes unreliable.
                    let ratio = FloatMath::abs(column[k]) / norms[j];
                    let retained = (T::one() + ratio) * (T::one() - ratio);
                    let relative = norms[j] / reference[j];
                    if retained <= T::zero() || retained * relative * relative <= loss_limit {
                        norms[j] = column_norm(&column[k + 1..])?;
                        reference[j] = norms[j];
                    } else {
                        norms[j] = norms[j] * FloatMath::sqrt(retained);
                        if norms[j] == T::zero() {
                            norms[j] = column_norm(&column[k + 1..])?;
                            reference[j] = norms[j];
                        }
                    }
                }
            }
        }
        check_finite(&packed)?;
        let mut columns = Tensor::<T>::zeros([thin, m])?.into_storage_vec();
        for i in 0..thin {
            columns[i * m + i] = T::one();
        }
        for k in (0..thin).rev() {
            let reflector = &packed[k * m + k + 1..(k + 1) * m];
            for column in columns.chunks_exact_mut(m).skip(k) {
                apply_reflector(&mut column[k..], reflector, tau[k]);
            }
        }
        for j in 0..thin {
            for i in 0..m {
                q.storage_slice_mut()[i * thin + j] = columns[j * m + i];
            }
        }
        let mut largest = T::zero();
        for i in 0..thin {
            for j in i..n {
                r.storage_slice_mut()[i * n + j] = packed[j * m + i];
            }
            let d = FloatMath::abs(packed[i * m + i]);
            if d > largest {
                largest = d;
            }
        }
        let rank = (0..thin)
            .take_while(|&i| {
                let diagonal = FloatMath::abs(packed[i * m + i]);
                diagonal > T::zero() && (tolerance == T::zero() || diagonal / largest > tolerance)
            })
            .count();
        check_finite(q.storage_slice())?;
        check_finite(r.storage_slice())?;
        Ok(Self {
            q,
            r,
            permutation,
            rank,
            relative_tolerance: tolerance,
        })
    }
    /// Borrow full thin Q, including columns beyond the selected numerical rank.
    pub fn q(&self) -> &Tensor<T> {
        &self.q
    }
    /// Borrow upper trapezoidal R, including rows beyond the selected rank.
    pub fn r(&self) -> &Tensor<T> {
        &self.r
    }
    /// Factor-column to original-column indices; no allocation occurs.
    pub fn permutation(&self) -> &[usize] {
        &self.permutation
    }
    /// Selected leading diagonal rank; not an SVD or condition-number estimate.
    pub fn rank(&self) -> usize {
        self.rank
    }
    /// The relative diagonal cutoff policy used by this factorization.
    pub fn relative_tolerance(&self) -> T {
        self.relative_tolerance
    }

    fn project<B: TensorInput<T>>(&self, b: &B) -> Result<Tensor<T>, LinalgError> {
        let dims = b.dims();
        if !(dims.len() == 1 || dims.len() == 2) || dims[0] != self.q.shape().dims()[0] {
            return Err(LinalgError::Invalid(
                "RHS must have shape [rows] or [rows,columns]",
            ));
        }
        let shape = Kahen::new(dims)?;
        let columns = if dims.len() == 1 { 1 } else { dims[1] };
        if let Some(values) = b.as_c_contiguous_slice() {
            if values.len() != shape.numel() {
                return Err(LinalgError::Invalid(
                    "contiguous capability length mismatch",
                ));
            }
            check_finite(values)?;
        } else if shape.numel() != 0 {
            for i in 0..dims[0] {
                for j in 0..columns {
                    let value = if dims.len() == 1 {
                        *b.get(&[i])?
                    } else {
                        *b.get(&[i, j])?
                    };
                    if !FloatMath::is_finite(value) {
                        return Err(LinalgError::NonFinite);
                    }
                }
            }
        }
        let leading = self.q.slice_axis(1, 0, self.rank, 1)?;
        let projected = crate::matmul(&leading.transpose()?, b)?;
        debug_assert_eq!(projected.numel(), self.rank * columns);
        check_finite(projected.storage_slice())?;
        Ok(projected)
    }
    fn solution_buffer(&self, b_dims: &[usize]) -> Result<Tensor<T>, LinalgError> {
        let n = self.permutation.len();
        Ok(if b_dims.len() == 1 {
            Tensor::<T>::zeros([n])?
        } else {
            Tensor::<T>::zeros([n, b_dims[1]])?
        })
    }
    /// Basic least-squares solution of the rank-truncated matrix. Permuted free
    /// coefficients are zero; this is generally NOT the minimum-norm solution.
    /// Reuses factors, costs O((m+n)*k + m*r*k + r^2*k), with borrowed RHS
    /// and allocated projection/solution workspace.
    /// RHS `[m]`/`[m,k]` returns `[n]`/`[n,k]`; finite values are required.
    pub fn solve_basic_least_squares<B: TensorInput<T>>(
        &self,
        b: &B,
    ) -> Result<Tensor<T>, LinalgError> {
        let projected = self.project(b)?;
        let n = self.permutation.len();
        let columns = if b.dims().len() == 1 { 1 } else { b.dims()[1] };
        let mut values = self.solution_buffer(b.dims())?;
        if columns != 0 {
            for row in (0..self.rank).rev() {
                for j in 0..columns {
                    let mut v = projected.storage_slice()[row * columns + j];
                    for k in row + 1..self.rank {
                        v = v - self.r.storage_slice()[row * n + k]
                            * values.storage_slice()[self.permutation[k] * columns + j];
                    }
                    values.storage_slice_mut()[self.permutation[row] * columns + j] =
                        v / self.r.storage_slice()[row * n + row];
                }
            }
        }
        check_finite(values.storage_slice())?;
        Ok(values)
    }
    /// Minimum Euclidean-norm least-squares solution of the rank-truncated matrix.
    ///
    /// For full column rank this reuses the triangular solve directly. Otherwise
    /// a second QR of scaled `R[:rank,:]^T` supplies the right orthogonal basis;
    /// no normal equations or explicit inverse are formed. This extra factorization
    /// is performed per call: O(n*r^2) time/O(n*r) memory, plus
    /// O((m+n)*r*k + (m+n)*k) validation/projection/output work.
    /// Rank zero returns zeros after validating the finite RHS. All arithmetic
    /// remains in T. Nonfinite intermediates return NonFinite even when another
    /// evaluation might be representable; underflow/rounding retain T semantics.
    /// A zero pivot in the normalized right factor returns Singular. Rank truncation
    /// is QR-based, not an optimal SVD truncation or a forward-error guarantee.
    pub fn solve_least_squares<B: TensorInput<T>>(&self, b: &B) -> Result<Tensor<T>, LinalgError> {
        let n = self.permutation.len();
        if self.rank == n {
            return self.solve_basic_least_squares(b);
        }
        let mut projected = self.project(b)?;
        let columns = if b.dims().len() == 1 { 1 } else { b.dims()[1] };
        let mut values = self.solution_buffer(b.dims())?;
        if self.rank != 0 && columns != 0 {
            let scale = max_abs(&self.r.storage_slice()[..self.rank * n]);
            let mut transpose = Tensor::<T>::zeros([n, self.rank])?;
            for j in 0..n {
                for i in 0..self.rank {
                    transpose.storage_slice_mut()[j * self.rank + i] =
                        self.r.storage_slice()[i * n + j] / scale;
                }
            }
            let right = Qr::factor(&transpose)?;
            let triangular = right.r().storage_slice();
            for row in 0..self.rank {
                let diagonal = triangular[row * self.rank + row];
                if diagonal == T::zero() {
                    return Err(LinalgError::Singular { pivot: row });
                }
                for j in 0..columns {
                    let mut v = projected.storage_slice()[row * columns + j];
                    for k in 0..row {
                        v = v - triangular[k * self.rank + row]
                            * projected.storage_slice()[k * columns + j];
                    }
                    projected.storage_slice_mut()[row * columns + j] = v / diagonal;
                }
            }
            check_finite(projected.storage_slice())?;
            let product = crate::matmul(right.q(), &projected)?;
            for (row, &original) in self.permutation.iter().enumerate() {
                for j in 0..columns {
                    values.storage_slice_mut()[original * columns + j] =
                        product.storage_slice()[row * columns + j] / scale;
                }
            }
            check_finite(values.storage_slice())?;
        }
        Ok(values)
    }
}
