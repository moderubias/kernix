//! Row-wise multiple-RHS substitution, preserving increasing-k scalar order.
use super::LinalgFloat;

#[inline(never)]
pub(super) fn solve<T: LinalgFloat, F: Fn(usize, usize) -> T>(
    out: &mut [T],
    n: usize,
    columns: usize,
    lower: bool,
    unit: bool,
    entry: F,
) {
    if out.is_empty() {
        return;
    }
    for step in 0..n {
        let row = if lower { step } else { n - step - 1 };
        if lower {
            let (previous, tail) = out.split_at_mut(row * columns);
            let current = &mut tail[..columns];
            for (k, other) in previous.chunks_exact(columns).enumerate() {
                let coefficient = entry(row, k);
                for (value, &x) in current.iter_mut().zip(other) {
                    *value = *value - coefficient * x;
                }
            }
            if !unit {
                let d = entry(row, row);
                for value in current {
                    *value = *value / d;
                }
            }
        } else {
            let (previous, following) = out.split_at_mut((row + 1) * columns);
            let current = &mut previous[row * columns..];
            for (offset, other) in following.chunks_exact(columns).enumerate() {
                let coefficient = entry(row, row + 1 + offset);
                for (value, &x) in current.iter_mut().zip(other) {
                    *value = *value - coefficient * x;
                }
            }
            if !unit {
                let d = entry(row, row);
                for value in current {
                    *value = *value / d;
                }
            }
        }
    }
}
