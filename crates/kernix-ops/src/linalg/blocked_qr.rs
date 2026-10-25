//! Forward compact-WY panels, using the existing checked scalar kernel capability.
use super::{LinalgError, LinalgFloat, check_finite, reflector_dot};
use crate::MatrixInput;
use alloc::vec::Vec;
use kernix_core::tensor::Tensor;

pub(super) const BLOCK: usize = 32;

pub(super) struct Workspace<T: LinalgFloat> {
    v: Vec<T>,
    w: Vec<T>,
    cache: Vec<T>,
    m: usize,
    start: usize,
    width: usize,
}

impl<T: LinalgFloat> Workspace<T> {
    pub(super) fn new(m: usize, n: usize) -> Result<Self, LinalgError> {
        Ok(Self {
            v: Tensor::<T>::zeros([BLOCK, m])?.into_storage_vec(),
            cache: Tensor::<T>::zeros([n.div_ceil(BLOCK), BLOCK, BLOCK])?.into_storage_vec(),
            w: Tensor::<T>::zeros([BLOCK, n])?.into_storage_vec(),
            m,
            start: 0,
            width: 0,
        })
    }

    // H_start ... H_end-1 = I - V T V^T, with upper triangular T.
    pub(super) fn prepare(
        &mut self,
        packed: &[T],
        tau: &[T],
        start: usize,
        end: usize,
        reuse: bool,
    ) -> bool {
        self.start = start;
        self.width = end - start;
        for j in 0..self.width {
            let pivot = start + j;
            let column = &mut self.v[j * self.m..(j + 1) * self.m];
            column[start..pivot].fill(T::zero());
            column[pivot] = T::one();
            column[pivot + 1..]
                .copy_from_slice(&packed[pivot * self.m + pivot + 1..(pivot + 1) * self.m]);
        }
        let cache_offset = start / BLOCK * BLOCK * BLOCK;
        if reuse {
            return check_finite(&self.cache[cache_offset..cache_offset + BLOCK * BLOCK]).is_ok();
        }
        let t = &mut self.cache[cache_offset..cache_offset + BLOCK * BLOCK];
        t.fill(T::zero());
        let stride = self.m as isize;
        let gram = if let (Ok(vt), Ok(v)) = (
            MatrixInput::new(&self.v, [self.width, self.m - start], [stride, 1], start),
            MatrixInput::new(&self.v, [self.m - start, self.width], [1, stride], start),
        ) {
            T::try_strided_matmul(vt, v, &mut self.w[..self.width * self.width])
                && check_finite(&self.w[..self.width * self.width]).is_ok()
        } else {
            false
        };
        for j in 0..self.width {
            let pivot = start + j;
            // The Gram upper triangle is still needed by later columns.
            // Its already-consumed lower row can hold z without another buffer.
            for i in 0..j {
                let a = &self.v[i * self.m..(i + 1) * self.m];
                let b = &self.v[j * self.m..(j + 1) * self.m];
                self.w[j * self.width + i] = -tau[pivot]
                    * if gram {
                        self.w[i * self.width + j]
                    } else {
                        reflector_dot(a[pivot], &a[pivot + 1..], &b[pivot + 1..])
                    };
            }
            for i in 0..j {
                let mut value = T::zero();
                for l in i..j {
                    value += t[i * BLOCK + l] * self.w[j * self.width + l];
                }
                t[i * BLOCK + j] = value;
            }
            t[j * BLOCK + j] = tau[pivot];
        }
        check_finite(t).is_ok()
    }

    // Produce V op(T) V^T A in disjoint scratch. The caller commits only after
    // every subtraction is finite; unsupported/range-limited kernels can decline
    // without changing A, then individual reflectors remain a valid fallback.
    pub(super) fn product(
        &mut self,
        input: MatrixInput<'_, T>,
        columns: usize,
        transpose: bool,
        product: &mut [T],
    ) -> Result<bool, LinalgError> {
        let (rows, width) = (self.m - self.start, self.width);
        let stride =
            isize::try_from(self.m).map_err(|_| LinalgError::Invalid("QR stride overflow"))?;
        let v_transpose = MatrixInput::new(&self.v, [width, rows], [stride, 1], self.start)?;
        let w = &mut self.w[..width * columns];
        if !T::try_strided_matmul(v_transpose, input, w) || check_finite(w).is_err() {
            return Ok(false);
        }
        let offset = self.start / BLOCK * BLOCK * BLOCK;
        let t = &self.cache[offset..offset + BLOCK * BLOCK];
        for step in 0..width {
            let i = if transpose { width - step - 1 } else { step };
            let (before, from_i) = w.split_at_mut(i * columns);
            let (row, after) = from_i.split_at_mut(columns);
            for value in row.iter_mut() {
                *value = *value * t[i * BLOCK + i];
            }
            if transpose {
                for (j, source) in before.chunks_exact(columns).enumerate() {
                    let alpha = t[j * BLOCK + i];
                    for (value, &other) in row.iter_mut().zip(source) {
                        *value += alpha * other;
                    }
                }
            } else {
                for (j, source) in after.chunks_exact(columns).enumerate() {
                    let alpha = t[i * BLOCK + i + 1 + j];
                    for (value, &other) in row.iter_mut().zip(source) {
                        *value += alpha * other;
                    }
                }
            }
        }
        if check_finite(w).is_err() {
            return Ok(false);
        }
        let v = MatrixInput::new(&self.v, [rows, width], [1, stride], self.start)?;
        let w = MatrixInput::new(w, [width, columns], [columns as isize, 1], 0)?;
        Ok(T::try_strided_matmul(v, w, &mut product[..rows * columns]))
    }
}

pub(super) fn commit<T: LinalgFloat>(
    data: &mut [T],
    m: usize,
    start: usize,
    first_col: usize,
    columns: usize,
    product: &mut [T],
) -> bool {
    let rows = m - start;
    for first_row in (0..rows).step_by(8) {
        for first_col_tile in (0..columns).step_by(16) {
            for col in first_col_tile..(first_col_tile + 16).min(columns) {
                for row in first_row..(first_row + 8).min(rows) {
                    let index = row * columns + col;
                    product[index] = data[(first_col + col) * m + start + row] - product[index];
                }
            }
        }
    }
    if check_finite(&product[..rows * columns]).is_err() {
        return false;
    }
    for first_row in (0..rows).step_by(8) {
        for first_col_tile in (0..columns).step_by(16) {
            for col in first_col_tile..(first_col_tile + 16).min(columns) {
                for row in first_row..(first_row + 8).min(rows) {
                    data[(first_col + col) * m + start + row] = product[row * columns + col];
                }
            }
        }
    }
    true
}
