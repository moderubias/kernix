//! Private complete orthogonal reduction of an upper trapezoidal matrix.
use super::{LinalgError, LinalgFloat, check_finite, max_abs, reflector_dot};
use crate::FloatMath;
use kernix_core::tensor::Tensor;

/// `[R11 R12] = [T 0] Z`, with Z represented by row reflectors.
/// Only the diagonal head and columns rank..n participate in each reflector.
pub(super) struct Rz<T: LinalgFloat> {
    rows: Tensor<T>,
    tau: Tensor<T>,
}

impl<T: LinalgFloat> Rz<T> {
    pub(super) fn factor(
        input: &[T],
        rank: usize,
        n: usize,
        scale: T,
    ) -> Result<Self, LinalgError> {
        debug_assert!(rank < n && rank != 0 && scale > T::zero());
        let mut rows = Tensor::<T>::zeros([rank, n])?;
        for (out, &value) in rows.storage_slice_mut().iter_mut().zip(input) {
            *out = value / scale;
        }
        let mut tau = Tensor::<T>::zeros([rank])?;
        for i in (0..rank).rev() {
            let (earlier, current) = rows.storage_slice_mut().split_at_mut(i * n);
            let row = &mut current[..n];
            let tail_scale = max_abs(&row[rank..]);
            if tail_scale == T::zero() {
                continue;
            }
            let alpha = row[i];
            let scale = if FloatMath::abs(alpha) > tail_scale {
                FloatMath::abs(alpha)
            } else {
                tail_scale
            };
            // Keep norm components separate: physical subnormal norm rounding
            // must not corrupt the dimensionless Householder vector.
            let head = alpha / scale;
            let mut sum = head * head;
            for &x in &row[rank..] {
                let x = x / scale;
                sum += x * x;
            }
            let unit = FloatMath::sqrt(sum);
            let sign = if alpha >= T::zero() {
                T::one()
            } else {
                -T::one()
            };
            let ratio = head / unit;
            let denominator = ratio + sign;
            let t = T::one() + sign * ratio;
            for x in &mut row[rank..] {
                *x = ((*x / scale) / unit) / denominator;
            }
            row[i] = -sign * (scale * unit);
            tau.storage_slice_mut()[i] = t;
            for previous in earlier.chunks_exact_mut(n) {
                let dot = reflector_dot(previous[i], &row[rank..], &previous[rank..]);
                let d = t * dot;
                if FloatMath::is_finite(d) {
                    previous[i] = previous[i] - d;
                    for (x, &v) in previous[rank..].iter_mut().zip(&row[rank..]) {
                        *x = *x - v * d;
                    }
                } else {
                    let second = (t - T::one()) * dot;
                    previous[i] = (previous[i] - dot) - second;
                    for (x, &v) in previous[rank..].iter_mut().zip(&row[rank..]) {
                        *x = (*x - v * dot) - v * second;
                    }
                }
            }
        }
        check_finite(rows.storage_slice())?;
        check_finite(tau.storage_slice())?;
        Ok(Self { rows, tau })
    }

    pub(super) fn solve(
        &self,
        projected: &mut [T],
        columns: usize,
        permutation: &[usize],
        output: &mut [T],
        scale: T,
    ) -> Result<(), LinalgError> {
        let (rank, n) = (self.tau.numel(), permutation.len());
        let rows = self.rows.storage_slice();
        for i in (0..rank).rev() {
            let diagonal = rows[i * n + i];
            if diagonal == T::zero() {
                return Err(LinalgError::Singular { pivot: i });
            }
            for c in 0..columns {
                let mut value = projected[i * columns + c];
                for j in i + 1..rank {
                    value = value - rows[i * n + j] * projected[j * columns + c];
                }
                projected[i * columns + c] = value / diagonal;
            }
            output[permutation[i] * columns..(permutation[i] + 1) * columns]
                .copy_from_slice(&projected[i * columns..(i + 1) * columns]);
        }
        check_finite(projected)?;
        // Z^T [w;0]: apply H_0 first. Work directly in the final permutation,
        // avoiding a second n-by-RHS solution buffer or an explicit Z.
        for i in 0..rank {
            let tau = self.tau.storage_slice()[i];
            if tau == T::zero() {
                continue;
            }
            let reflector = &rows[i * n + rank..(i + 1) * n];
            for c in 0..columns {
                let index = permutation[i] * columns + c;
                let mut dot = output[index];
                for (&original, &v) in permutation[rank..].iter().zip(reflector) {
                    dot += v * output[original * columns + c];
                }
                let d = tau * dot;
                if FloatMath::is_finite(d) {
                    output[index] = output[index] - d;
                    for (&original, &v) in permutation[rank..].iter().zip(reflector) {
                        let x = &mut output[original * columns + c];
                        *x = *x - v * d;
                    }
                } else {
                    let second = (tau - T::one()) * dot;
                    output[index] = (output[index] - dot) - second;
                    for (&original, &v) in permutation[rank..].iter().zip(reflector) {
                        let x = &mut output[original * columns + c];
                        *x = (*x - v * dot) - v * second;
                    }
                }
            }
        }
        // Delay RHS rescaling until after all orthogonal transformations.
        for x in output.iter_mut() {
            *x = *x / scale;
        }
        check_finite(output)
    }
}
