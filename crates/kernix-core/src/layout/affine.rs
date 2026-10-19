use core::mem::size_of;
use smallvec::SmallVec;

use super::geometry::{self, Overlap, StorageSpan};
use super::{LayoutError, LayoutShape, Order};
use crate::shape::{Kahen, ShapeError};

/// Validated affine mapping from N-dimensional indices to *element* indices
/// relative to a storage allocation's beginning.
///
/// This type contains neither a pointer nor a device. Binding it to storage
/// additionally requires byte extent, allocation provenance, dtype/alignment,
/// and backend-specific checks. All fields are private.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Layout<S: LayoutShape = Kahen> {
    shape: S,
    strides: S::Strides,
    origin: usize,
    span: Option<StorageSpan>,
}

impl<S: LayoutShape> Layout<S> {
    /// Canonical C- or F-ordered strides. May fail even when `shape` is valid,
    /// because strides are stored in `isize` (including for an empty tensor).
    pub fn contiguous(shape: S, order: Order) -> Result<Self, LayoutError> {
        let mut strides = shape.zeroed_strides();
        geometry::contiguous_strides(shape.dims(), strides.as_mut(), order)?;
        Self::from_parts(shape, strides, 0)
    }

    /// Build from a shape-specific owned stride representation.
    pub fn from_parts(shape: S, strides: S::Strides, origin: usize) -> Result<Self, LayoutError> {
        let span = geometry::checked_span(shape.dims(), strides.as_ref(), origin)?;
        Ok(Self {
            shape,
            strides,
            origin,
            span,
        })
    }

    /// Build from an arbitrary borrowed stride slice.
    pub fn from_strides(
        shape: S,
        strides: impl AsRef<[isize]>,
        origin: usize,
    ) -> Result<Self, LayoutError> {
        let src = strides.as_ref();
        if src.len() != shape.rank() {
            return Err(LayoutError::StrideRankMismatch {
                expected: shape.rank(),
                actual: src.len(),
            });
        }
        let mut owned = shape.zeroed_strides();
        owned.as_mut().copy_from_slice(src);
        Self::from_parts(shape, owned, origin)
    }

    /// Borrow the validated shape without copying dimensions. O(1).
    pub fn shape(&self) -> &S {
        &self.shape
    }
    /// Borrow signed element strides in axis order. Zero denotes broadcasting. O(1).
    pub fn strides(&self) -> &[isize] {
        self.strides.as_ref()
    }
    /// Return the physical element offset of logical index [0, ...]. O(1).
    pub fn origin(&self) -> usize {
        self.origin
    }
    /// Return the number of axes. Scalars have rank zero. O(1).
    pub fn rank(&self) -> usize {
        self.shape.rank()
    }
    /// Return the logical element count. Scalars contain one element; any zero
    /// dimension makes the count zero. Shape validation prevents product overflow.
    pub fn numel(&self) -> usize {
        self.shape.numel()
    }
    /// Return whether there are no logical elements. A scalar is not empty.
    pub fn is_empty(&self) -> bool {
        self.shape.is_zero_sized()
    }
    /// All storage element indices potentially addressed; `None` if empty.
    pub fn span(&self) -> Option<StorageSpan> {
        self.span
    }
    /// Return the lowest reachable storage index, or None for an empty layout.
    pub fn min_offset(&self) -> Option<usize> {
        self.span.map(|s| s.start)
    }
    /// Return the highest reachable storage index, or None for an empty layout.
    pub fn max_offset(&self) -> Option<usize> {
        self.span.map(|s| s.end - 1)
    }
    /// Required allocation length in elements; the span may begin above zero.
    pub fn required_storage_len(&self) -> usize {
        self.span.map_or(0, |span| span.end)
    }

    /// Test whether any stride is negative, including strides on singleton axes.
    pub fn has_negative_strides(&self) -> bool {
        self.strides().iter().any(|&s| s < 0)
    }
    /// Test whether any stride is zero, including strides on singleton axes.
    pub fn has_zero_strides(&self) -> bool {
        self.strides().contains(&0)
    }
    /// Test whether an axis of length greater than one has stride +1 or -1.
    pub fn has_unit_stride(&self) -> bool {
        self.shape
            .dims()
            .iter()
            .zip(self.strides())
            .any(|(&dim, &s)| dim > 1 && s.unsigned_abs() == 1)
    }
    /// Test row-major contiguity, ignoring singleton strides; empty layouts are contiguous.
    pub fn is_c_contiguous(&self) -> bool {
        geometry::is_contiguous(self.shape.dims(), self.strides(), Order::C)
    }
    /// Test column-major contiguity, ignoring singleton strides; empty layouts are contiguous.
    pub fn is_f_contiguous(&self) -> bool {
        geometry::is_contiguous(self.shape.dims(), self.strides(), Order::F)
    }
    /// Test contiguity in either C or F order.
    pub fn is_contiguous(&self) -> bool {
        self.is_c_contiguous() || self.is_f_contiguous()
    }
    /// Conservatively classify whether different logical indices may share storage.
    /// Unknown does not authorize independently mutable references.
    pub fn overlap(&self) -> Overlap {
        geometry::overlap(self.shape.dims(), self.strides(), self.span, self.numel())
    }
    /// Dense means distinct logical elements cover precisely one hole-free
    /// storage interval. `Unknown` overlap never counts as dense.
    pub fn is_dense(&self) -> bool {
        self.is_empty()
            || (self.overlap() == Overlap::NonOverlapping
                && self.span.is_some_and(|s| s.len() == self.numel()))
    }

    /// Compute an element offset; checks rank, indices and arithmetic.
    pub fn offset_of(&self, indices: &[usize]) -> Result<usize, LayoutError> {
        if indices.len() != self.rank() {
            return Err(LayoutError::IndexRankMismatch {
                expected: self.rank(),
                actual: indices.len(),
            });
        }
        let mut position = i128::try_from(self.origin).map_err(|_| LayoutError::AddressOverflow)?;
        for (axis, ((&index, &dim), &stride)) in indices
            .iter()
            .zip(self.shape.dims())
            .zip(self.strides())
            .enumerate()
        {
            if index >= dim {
                return Err(LayoutError::IndexOutOfBounds { axis, index, dim });
            }
            let index = i128::try_from(index).map_err(|_| LayoutError::AddressOverflow)?;
            let delta = index
                .checked_mul(stride as i128)
                .ok_or(LayoutError::AddressOverflow)?;
            position = position
                .checked_add(delta)
                .ok_or(LayoutError::AddressOverflow)?;
        }
        usize::try_from(position).map_err(|_| LayoutError::AddressOverflow)
    }

    /// Element-count bound only; dtype/device constraints are separate.
    pub fn validate_storage_len(&self, available: usize) -> Result<(), LayoutError> {
        let required = self.required_storage_len();
        if available < required {
            return Err(LayoutError::StorageTooSmall {
                required,
                available,
            });
        }
        Ok(())
    }

    /// Minimal CPU-side byte-length checks. Must still be paired with actual
    /// allocation provenance and alignment checks at storage binding.
    pub fn validate_cpu_storage<T>(&self, available: usize) -> Result<(), LayoutError> {
        let bytes_per_element = size_of::<T>();
        if bytes_per_element == 0 {
            return Err(LayoutError::ZeroSizedElementType);
        }
        self.validate_storage_len(available)?;
        let byte_len = available
            .checked_mul(bytes_per_element)
            .ok_or(LayoutError::StorageByteExtentOverflow)?;
        if byte_len > isize::MAX as usize {
            return Err(LayoutError::StorageByteExtentOverflow);
        }
        Ok(())
    }

    fn axis(&self, axis: usize) -> Result<(), LayoutError> {
        if axis >= self.rank() {
            Err(LayoutError::AxisOutOfBounds {
                axis,
                rank: self.rank(),
            })
        } else {
            Ok(())
        }
    }

    fn same_rank_layout(
        &self,
        dims: &[usize],
        strides: &[isize],
        origin: usize,
    ) -> Result<Self, LayoutError> {
        let shape = self.shape.with_same_rank_dims(dims)?;
        Self::from_strides(shape, strides, origin)
    }

    /// Permutation is specified by source axis for each output axis.
    pub fn permute(&self, axes: impl AsRef<[usize]>) -> Result<Self, LayoutError> {
        let axes = axes.as_ref();
        if axes.len() != self.rank() {
            return Err(LayoutError::Shape(ShapeError::RankMismatch {
                expected: self.rank(),
                actual: axes.len(),
            }));
        }
        let mut seen: SmallVec<[bool; 6]> = core::iter::repeat_n(false, self.rank()).collect();
        let mut dims: SmallVec<[usize; 6]> = SmallVec::with_capacity(self.rank());
        let mut strides: SmallVec<[isize; 6]> = SmallVec::with_capacity(self.rank());
        for &axis in axes {
            self.axis(axis)?;
            if seen[axis] {
                return Err(LayoutError::Shape(ShapeError::DuplicateAxis { axis }));
            }
            seen[axis] = true;
            dims.push(self.shape.dims()[axis]);
            strides.push(self.strides()[axis]);
        }
        self.same_rank_layout(&dims, &strides, self.origin)
    }

    /// Swap two existing axes without moving any stored elements; invalid axes return an error.
    pub fn swap_axes(&self, left: usize, right: usize) -> Result<Self, LayoutError> {
        self.axis(left)?;
        self.axis(right)?;
        let mut dims: SmallVec<[usize; 6]> = SmallVec::from_slice(self.shape.dims());
        let mut strides: SmallVec<[isize; 6]> = SmallVec::from_slice(self.strides());
        dims.swap(left, right);
        strides.swap(left, right);
        self.same_rank_layout(&dims, &strides, self.origin)
    }

    /// Two-dimensional transpose. Use `swap_axes` or `permute` for N-D tensors.
    pub fn transpose(&self) -> Result<Self, LayoutError> {
        if self.rank() != 2 {
            return Err(LayoutError::Shape(ShapeError::RankMismatch {
                expected: 2,
                actual: self.rank(),
            }));
        }
        self.swap_axes(0, 1)
    }

    /// Reverse logical traversal along an axis using a negative stride.
    /// Returns an error for an invalid axis or unrepresentable address arithmetic.
    pub fn reverse_axis(&self, axis: usize) -> Result<Self, LayoutError> {
        self.axis(axis)?;
        let dim = self.shape.dims()[axis];
        // Singleton and empty dimensions cannot change the addressed elements.
        if dim <= 1 || self.is_empty() {
            return Ok(self.clone());
        }
        let mut strides: SmallVec<[isize; 6]> = SmallVec::from_slice(self.strides());
        let old_stride = strides[axis];
        strides[axis] = old_stride
            .checked_neg()
            .ok_or(LayoutError::StrideOverflow { axis })?;
        let displacement = i128::try_from(dim - 1)
            .map_err(|_| LayoutError::AddressOverflow)?
            .checked_mul(old_stride as i128)
            .ok_or(LayoutError::AddressOverflow)?;
        let origin = i128::try_from(self.origin)
            .map_err(|_| LayoutError::AddressOverflow)?
            .checked_add(displacement)
            .ok_or(LayoutError::AddressOverflow)?;
        let origin = usize::try_from(origin).map_err(|_| LayoutError::AddressOverflow)?;
        self.same_rank_layout(self.shape.dims(), &strides, origin)
    }

    /// `start` is the first *source* index, `len` is the number of output
    /// elements, and `step != 0` is the signed increment in source indices.
    /// Unlike Python's slice syntax there are no implicit end/default rules.
    pub fn slice_axis(
        &self,
        axis: usize,
        start: usize,
        len: usize,
        step: isize,
    ) -> Result<Self, LayoutError> {
        self.axis(axis)?;
        if step == 0 {
            return Err(LayoutError::ZeroSliceStep);
        }
        let dim = self.shape.dims()[axis];
        let invalid = || LayoutError::InvalidSlice {
            axis,
            start,
            len,
            step,
            dim,
        };
        if len == 0 {
            if start > dim {
                return Err(invalid());
            }
        } else {
            if start >= dim {
                return Err(invalid());
            }
            let steps = i128::try_from(len - 1).map_err(|_| invalid())?;
            let last = i128::try_from(start)
                .map_err(|_| invalid())?
                .checked_add(steps.checked_mul(step as i128).ok_or_else(invalid)?)
                .ok_or_else(invalid)?;
            if last < 0 || last >= i128::try_from(dim).map_err(|_| invalid())? {
                return Err(invalid());
            }
        }
        let mut dims: SmallVec<[usize; 6]> = SmallVec::from_slice(self.shape.dims());
        dims[axis] = len;
        let mut strides: SmallVec<[isize; 6]> = SmallVec::from_slice(self.strides());
        // Values on a dimension of length <=1 never participate in indexing.
        strides[axis] = if len <= 1 {
            0
        } else {
            strides[axis]
                .checked_mul(step)
                .ok_or(LayoutError::StrideOverflow { axis })?
        };
        let origin = if dims.contains(&0) {
            self.origin
        } else {
            let delta = i128::try_from(start)
                .map_err(|_| LayoutError::AddressOverflow)?
                .checked_mul(self.strides()[axis] as i128)
                .ok_or(LayoutError::AddressOverflow)?;
            let next = i128::try_from(self.origin)
                .map_err(|_| LayoutError::AddressOverflow)?
                .checked_add(delta)
                .ok_or(LayoutError::AddressOverflow)?;
            usize::try_from(next).map_err(|_| LayoutError::AddressOverflow)?
        };
        self.same_rank_layout(&dims, &strides, origin)
    }

    /// Add an axis of size 1; changing rank produces a dynamic-rank layout.
    pub fn insert_axis(&self, axis: usize) -> Result<Layout<Kahen>, LayoutError> {
        if axis > self.rank() {
            return Err(LayoutError::AxisOutOfBounds {
                axis,
                rank: self.rank(),
            });
        }
        let mut dims: SmallVec<[usize; 6]> = SmallVec::from_slice(self.shape.dims());
        let mut strides: SmallVec<[isize; 6]> = SmallVec::from_slice(self.strides());
        dims.insert(axis, 1);
        strides.insert(axis, 0);
        Layout::<Kahen>::from_strides(Kahen::new(dims)?, &strides, self.origin)
    }

    /// Remove a singleton axis; changing rank produces a dynamic-rank layout.
    pub fn squeeze_axis(&self, axis: usize) -> Result<Layout<Kahen>, LayoutError> {
        self.axis(axis)?;
        let dim = self.shape.dims()[axis];
        if dim != 1 {
            return Err(LayoutError::Shape(ShapeError::AxisNotSingleton {
                axis,
                dim,
            }));
        }
        let mut dims: SmallVec<[usize; 6]> = SmallVec::from_slice(self.shape.dims());
        let mut strides: SmallVec<[isize; 6]> = SmallVec::from_slice(self.strides());
        dims.remove(axis);
        strides.remove(axis);
        Layout::<Kahen>::from_strides(Kahen::new(dims)?, &strides, self.origin)
    }

    /// Broadcasting expands missing or singleton axes with zero strides.
    pub fn broadcast_to(&self, target: impl AsRef<[usize]>) -> Result<Layout<Kahen>, LayoutError> {
        let new_shape = Kahen::new(target)?;
        let dest = new_shape.dims();
        if self.rank() > dest.len() {
            return Err(LayoutError::Shape(ShapeError::RankMismatch {
                expected: self.rank(),
                actual: dest.len(),
            }));
        }
        let mut new_strides: SmallVec<[isize; 6]> = core::iter::repeat_n(0, dest.len()).collect();
        for offset in 0..self.rank() {
            let source_axis = self.rank() - 1 - offset;
            let target_axis = dest.len() - 1 - offset;
            let source_dim = self.shape.dims()[source_axis];
            let target_dim = dest[target_axis];
            new_strides[target_axis] = if source_dim == target_dim {
                self.strides()[source_axis]
            } else if source_dim == 1 {
                0
            } else {
                return Err(LayoutError::BroadcastIncompatible {
                    axis: target_axis,
                    source_dim,
                    target: target_dim,
                });
            };
        }
        Layout::<Kahen>::from_strides(new_shape, new_strides, self.origin)
    }

    /// Reshape without copying elements, if the original affine mapping can
    /// be factored into compatible stride chunks in the requested order.
    /// This also supports some non-contiguous and negative-stride views.
    pub fn reshape_view(
        &self,
        new_dims: impl AsRef<[usize]>,
        order: Order,
    ) -> Result<Layout<Kahen>, LayoutError> {
        let shape = Kahen::new(new_dims)?;
        if shape.numel() != self.numel() {
            return Err(LayoutError::ReshapeElementCountMismatch {
                old: self.numel(),
                new: shape.numel(),
            });
        }
        if shape.dims() == self.shape.dims() {
            return Layout::<Kahen>::from_strides(shape, self.strides(), self.origin);
        }
        if self.numel() <= 1 {
            // An empty or single-element tensor has no varying address.
            let strides: SmallVec<[isize; 6]> = core::iter::repeat_n(0, shape.rank()).collect();
            return Layout::<Kahen>::from_strides(shape, strides, self.origin);
        }
        let strides =
            geometry::reshape_strides(self.shape.dims(), self.strides(), shape.dims(), order)?
                .ok_or(LayoutError::NotViewReshapable)?;
        Layout::<Kahen>::from_strides(shape, strides, self.origin)
    }
}
