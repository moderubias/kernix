//! Orthogonal updates and checked hyperbolic downdates of a lower factor.
use super::{LinalgError, LinalgFloat, check_finite};
use crate::{FloatMath, TensorInput};
use kernix_core::tensor::Tensor;

pub(super) fn modify<T: LinalgFloat, V: TensorInput<T>>(
    lower: &Tensor<T>,
    vector: &V,
    downdate: bool,
) -> Result<Tensor<T>, LinalgError> {
    let n = lower.shape().dims()[0];
    if vector.dims() != [n] {
        return Err(LinalgError::Invalid(
            "rank-one modification requires a length-n vector",
        ));
    }
    let contiguous = vector.as_c_contiguous_slice();
    if let Some(values) = contiguous {
        if values.len() != n {
            return Err(LinalgError::Invalid(
                "contiguous capability length mismatch",
            ));
        }
        check_finite(values)?;
    }
    let mut work = Tensor::<T>::zeros([n])?.into_storage_vec();
    if let Some(values) = contiguous {
        work.copy_from_slice(values);
    } else {
        for (i, value) in work.iter_mut().enumerate() {
            *value = *vector.get(&[i])?;
        }
        check_finite(&work)?;
    }
    let mut output = Tensor::<T>::zeros([n, n])?;
    let l = output.storage_slice_mut();
    // Keep working columns contiguous for the rotation sweep; transpose to the
    // public C-order lower factor only once, without another n² allocation.
    let original = lower.storage_slice();
    for first_row in (0..n).step_by(8) {
        for first_col in (0..n).step_by(16) {
            for col in first_col..(first_col + 16).min(n) {
                for row in first_row.max(col)..(first_row + 8).min(n) {
                    l[col * n + row] = original[row * n + col];
                }
            }
        }
    }
    for k in 0..n {
        let x = work[k];
        if x == T::zero() {
            continue;
        }
        let d = l[k * n + k];
        let magnitude = FloatMath::abs(x);
        let (r, c, s) = if downdate {
            if magnitude >= d {
                return Err(LinalgError::NotPositiveDefinite { pivot: k });
            }
            let ratio = x / d;
            // (d-|x|)/d retains the positive gap even if |x|/d rounds to one.
            let c = FloatMath::sqrt(((d - magnitude) / d) * (T::one() + magnitude / d));
            (d * c, c, ratio)
        } else {
            let scale = if d > magnitude { d } else { magnitude };
            let ds = d / scale;
            let xs = x / scale;
            let r = scale * FloatMath::sqrt(ds * ds + xs * xs);
            (r, d / r, x / r)
        };
        if !FloatMath::is_finite(r) || !FloatMath::is_finite(c) || !FloatMath::is_finite(s) {
            return Err(LinalgError::NonFinite);
        }
        if r <= T::zero() {
            return Err(LinalgError::NotPositiveDefinite { pivot: k });
        }
        l[k * n + k] = r;
        let column = &mut l[k * n + k + 1..(k + 1) * n];
        let tail = &mut work[k + 1..];
        if downdate {
            let reciprocal = T::one() / c;
            if FloatMath::is_finite(reciprocal) {
                for (value, x) in column.iter_mut().zip(tail.iter_mut()) {
                    let old = *value;
                    let current = *x;
                    *value = (old - s * current) * reciprocal;
                    *x = (current - s * old) * reciprocal;
                }
            } else {
                for (value, x) in column.iter_mut().zip(tail.iter_mut()) {
                    let old = *value;
                    let current = *x;
                    *value = (old - s * current) / c;
                    *x = (current - s * old) / c;
                }
            }
        } else {
            for (value, x) in column.iter_mut().zip(tail.iter_mut()) {
                let old = *value;
                let current = *x;
                *value = c * old + s * current;
                *x = c * current - s * old;
            }
        }
        check_finite(&l[k * n + k + 1..(k + 1) * n])?;
        check_finite(&work[k + 1..])?;
    }
    for first_row in (0..n).step_by(8) {
        for first_col in (0..=first_row).step_by(8) {
            for row in first_row..(first_row + 8).min(n) {
                for col in first_col..(first_col + 8).min(row) {
                    l.swap(row * n + col, col * n + row);
                }
            }
        }
    }
    check_finite(l)?;
    Ok(output)
}
