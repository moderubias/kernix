use super::{SparseError, payload, pointers, reserve};
use alloc::vec::Vec;
use core::ops::AddAssign;
use kernix_core::tensor::Tensor;

/// Canonical compressed sparse rows. Indices are sorted and unique in each row.
///
/// Raw construction adopts buffers without cloning elements or requiring numeric
/// traits. Explicit stored zeros are permitted. No affine HostBuffer/TensorInput
/// implementation pretends missing entries are ordinary dense storage.
#[derive(Debug)]
pub struct CsrMatrix<T> {
    pub(super) shape: [usize; 2],
    pub(super) ptr: Vec<usize>,
    pub(super) indices: Vec<usize>,
    pub(super) values: Vec<T>,
}
impl<T> CsrMatrix<T> {
    /// Adopt already canonical CSR buffers without copying their payloads.
    /// Offsets must start at zero, end at values.len(), and be monotone. Column
    /// indices must be in bounds and strictly increasing within every row.
    pub fn from_parts(
        shape: [usize; 2],
        row_offsets: Vec<usize>,
        column_indices: Vec<usize>,
        values: Vec<T>,
    ) -> Result<Self, SparseError> {
        let len = shape[0].checked_add(1).ok_or(SparseError::SizeOverflow)?;
        if row_offsets.len() != len
            || column_indices.len() != values.len()
            || row_offsets.first() != Some(&0)
            || row_offsets.last() != Some(&values.len())
        {
            return Err(SparseError::Invalid(
                "offset/index/value lengths or endpoints",
            ));
        }
        for row in row_offsets.windows(2) {
            if row[0] > row[1] || row[1] > values.len() {
                return Err(SparseError::Invalid("nonmonotone row offsets"));
            }
            let columns = &column_indices[row[0]..row[1]];
            if columns.iter().any(|&col| col >= shape[1])
                || columns.windows(2).any(|w| w[0] >= w[1])
            {
                return Err(SparseError::Invalid(
                    "columns must be sorted, unique and in bounds",
                ));
            }
        }
        payload::<T>(shape[0], values.len())?;
        Ok(Self {
            shape,
            ptr: row_offsets,
            indices: column_indices,
            values,
        })
    }
    /// Assemble COO triplets. Duplicate coordinates are added in original input
    /// order; sorting uses an ordinal tie-break and no hidden stable-sort buffer.
    /// Moves non-Clone elements. Uses O(input entries + rows) fallible workspace.
    /// Explicit zeros, including cancellation results, remain stored.
    pub fn from_triplets(
        shape: [usize; 2],
        entries: Vec<(usize, usize, T)>,
    ) -> Result<Self, SparseError>
    where
        T: AddAssign<T>,
    {
        let mut ptr = pointers(shape[0])?;
        let mut sorted = reserve(entries.len())?;
        for (ordinal, (row, col, value)) in entries.into_iter().enumerate() {
            if row >= shape[0] {
                return Err(SparseError::Index {
                    axis: 0,
                    index: row,
                    dim: shape[0],
                });
            }
            if col >= shape[1] {
                return Err(SparseError::Index {
                    axis: 1,
                    index: col,
                    dim: shape[1],
                });
            }
            sorted.push((row, col, ordinal, value));
        }
        sorted.sort_unstable_by_key(|v| (v.0, v.1, v.2));
        let unique = usize::from(!sorted.is_empty())
            + sorted
                .windows(2)
                .filter(|w| (w[0].0, w[0].1) != (w[1].0, w[1].1))
                .count();
        let mut indices = reserve(unique)?;
        let mut values: Vec<T> = reserve(unique)?;
        let mut previous = None;
        for (row, col, _, value) in sorted {
            if previous == Some((row, col)) {
                *values
                    .last_mut()
                    .ok_or(SparseError::Invalid("missing COO accumulator"))? += value;
            } else {
                ptr[row + 1] += 1;
                indices.push(col);
                values.push(value);
                previous = Some((row, col));
            }
        }
        for row in 1..ptr.len() {
            ptr[row] += ptr[row - 1];
        }
        Self::from_parts(shape, ptr, indices, values)
    }
    /// Logical rows and columns; logical element count need not fit usize.
    pub fn dims(&self) -> [usize; 2] {
        self.shape
    }
    /// Number of stored entries, including explicit stored zeros.
    pub fn nnz(&self) -> usize {
        self.values.len()
    }
    /// Borrow row offsets. No allocation or copy.
    pub fn row_offsets(&self) -> &[usize] {
        &self.ptr
    }
    /// Borrow canonical column indices. No allocation or copy.
    pub fn column_indices(&self) -> &[usize] {
        &self.indices
    }
    /// Borrow stored values. No allocation or copy.
    pub fn values(&self) -> &[T] {
        &self.values
    }
    /// Payload bytes for initialized offsets/indices/values, excluding capacity,
    /// metadata and allocator overhead. Not a process-RSS measurement.
    pub fn payload_bytes(&self) -> usize {
        payload::<T>(self.shape[0], self.nnz()).expect("validated CSR extent")
    }
    /// Consume into original shape and owned buffers without cloning elements.
    pub fn into_parts(self) -> ([usize; 2], Vec<usize>, Vec<usize>, Vec<T>) {
        (self.shape, self.ptr, self.indices, self.values)
    }
    /// Checked lookup. None denotes a structural zero, not an invalid coordinate.
    /// Binary search costs O(log(stored entries in the row)).
    pub fn get(&self, row: usize, col: usize) -> Result<Option<&T>, SparseError> {
        if row >= self.shape[0] {
            return Err(SparseError::Index {
                axis: 0,
                index: row,
                dim: self.shape[0],
            });
        }
        if col >= self.shape[1] {
            return Err(SparseError::Index {
                axis: 1,
                index: col,
                dim: self.shape[1],
            });
        }
        let start = self.ptr[row];
        let columns = &self.indices[start..self.ptr[row + 1]];
        Ok(columns
            .binary_search(&col)
            .ok()
            .map(|p| &self.values[start + p]))
    }
    /// Borrow a CSC view of the transpose in O(1), sharing all three buffers.
    pub fn transpose(&self) -> CscView<'_, T> {
        CscView { source: self }
    }
    /// Explicitly allocate a dense C-order matrix, checking logical size before
    /// allocation. Only this conversion requires cloning stored generic values.
    pub fn to_dense(&self) -> Result<Tensor<T>, SparseError>
    where
        T: crate::Zero + Clone,
    {
        let mut result = Tensor::<T>::zeros(self.shape)?;
        if result.numel() == 0 {
            return Ok(result);
        }
        let data = result.storage_slice_mut();
        for row in 0..self.shape[0] {
            for p in self.ptr[row]..self.ptr[row + 1] {
                data[row * self.shape[1] + self.indices[p]] = self.values[p].clone();
            }
        }
        Ok(result)
    }
}
/// A zero-copy CSC view of a CSR transpose. Values/indices/offsets are borrowed.
#[derive(Debug, Clone, Copy)]
pub struct CscView<'a, T> {
    pub(super) source: &'a CsrMatrix<T>,
}
impl<'a, T> CscView<'a, T> {
    /// Logical rows and columns of the transposed matrix.
    pub fn dims(&self) -> [usize; 2] {
        [self.source.shape[1], self.source.shape[0]]
    }
    /// Borrow column offsets, identical to the source row-offset buffer.
    pub fn column_offsets(&self) -> &'a [usize] {
        &self.source.ptr
    }
    /// Borrow row indices, identical to the source column-index buffer.
    pub fn row_indices(&self) -> &'a [usize] {
        &self.source.indices
    }
    /// Borrow unchanged values.
    pub fn values(&self) -> &'a [T] {
        &self.source.values
    }
    /// Transposing back borrows the original CSR matrix.
    pub fn transpose(&self) -> &'a CsrMatrix<T> {
        self.source
    }
    /// Checked lookup into the transpose, without searching other columns.
    pub fn get(&self, row: usize, col: usize) -> Result<Option<&'a T>, SparseError> {
        let dims = self.dims();
        if row >= dims[0] {
            return Err(SparseError::Index {
                axis: 0,
                index: row,
                dim: dims[0],
            });
        }
        if col >= dims[1] {
            return Err(SparseError::Index {
                axis: 1,
                index: col,
                dim: dims[1],
            });
        }
        self.source.get(col, row)
    }
}
