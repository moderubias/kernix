//! Left-looking Cholesky with bounded Schur workspace and borrowed matrix panels.
use super::{LinalgError, LinalgFloat};
use crate::{FloatMath, MatrixInput};
use kernix_core::tensor::Tensor;

const BLOCK: usize = 16;
const ROWS: usize = 64;

// Complete finite/symmetry validation precedes this function. Only completed
// columns are overwritten, so every current entry still holds its original A.
// This also preserves the ordered range retry without a second full input copy.
pub(super) fn factor<T: LinalgFloat>(a: &mut [T], n: usize) -> Result<(), LinalgError> {
    let mut workspace = Tensor::<T>::zeros([ROWS, BLOCK])?;
    let scratch = workspace.storage_slice_mut();
    for start in (0..n).step_by(BLOCK) {
        let end = (start + BLOCK).min(n);
        let width = end - start;
        schur(a, n, start, width, start, width, start, scratch)?;
        for row in start..end {
            for col in start..=row {
                let original = a[row * n + col];
                let mut value = original - scratch[(row - start) * width + col - start];
                for k in start..col {
                    value = value - a[row * n + k] * a[col * n + k];
                }
                if !FloatMath::is_finite(value) || (row == col && value <= T::zero()) {
                    value = ordered(a, n, row, col, original);
                }
                if row == col {
                    if value <= T::zero() {
                        return Err(LinalgError::NotPositiveDefinite { pivot: row });
                    }
                    a[row * n + col] = FloatMath::sqrt(value);
                } else {
                    a[row * n + col] = value / a[col * n + col];
                }
            }
        }
        // Each off-diagonal panel is a rectangular GEMM update followed by a
        // small right triangular solve. No transpose or full Schur copy exists.
        for first in (end..n).step_by(ROWS) {
            let rows = ROWS.min(n - first);
            schur(a, n, first, rows, start, width, start, scratch)?;
            for row in first..first + rows {
                for col in start..end {
                    let original = a[row * n + col];
                    let mut value = original - scratch[(row - first) * width + col - start];
                    for k in start..col {
                        value = value - a[row * n + k] * a[col * n + k];
                    }
                    if !FloatMath::is_finite(value) {
                        value = ordered(a, n, row, col, original);
                    }
                    a[row * n + col] = value / a[col * n + col];
                }
            }
        }
    }
    Ok(())
}

fn ordered<T: LinalgFloat>(a: &[T], n: usize, row: usize, col: usize, mut value: T) -> T {
    for k in 0..col {
        value = value - a[row * n + k] * a[col * n + k];
    }
    value
}

#[allow(clippy::too_many_arguments)]
fn schur<T: LinalgFloat>(
    a: &[T],
    n: usize,
    row: usize,
    rows: usize,
    col: usize,
    cols: usize,
    depth: usize,
    scratch: &mut [T],
) -> Result<(), LinalgError> {
    let output = &mut scratch[..rows * cols];
    if depth == 0 {
        output.fill(T::zero());
        return Ok(());
    }
    let stride = isize::try_from(n).map_err(|_| LinalgError::Invalid("matrix stride overflow"))?;
    let left = MatrixInput::new(a, [rows, depth], [stride, 1], row * n)?;
    let right = MatrixInput::new(a, [depth, cols], [1, stride], col * n)?;
    if !T::try_strided_matmul(left, right, output) {
        // Custom capabilities may decline the safe kernel request.
        for i in 0..rows {
            for j in 0..cols {
                let mut sum = T::zero();
                for k in 0..depth {
                    sum += a[(row + i) * n + k] * a[(col + j) * n + k];
                }
                output[i * cols + j] = sum;
            }
        }
    }
    Ok(())
}
