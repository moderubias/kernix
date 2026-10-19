use core::mem::align_of;

use super::{Tensor, TensorBuildError, TensorError, TensorIter, index::contiguous_range};
use crate::{
    layout::{Layout, LayoutError, LayoutShape, Order},
    shape::Kahen,
    storage::CpuBuffer,
};

/// Immutable borrowed tensor view; layout metadata is owned, elements are not.
/// Arbitrary positive, negative and zero strides are supported.
pub struct TensorView<'a, T, S: LayoutShape = Kahen> {
    elements: &'a [T],
    layout: Layout<S>,
}

/// Exclusive borrowed tensor view. `get_mut(&mut self)` allows zero-stride
/// layouts without exposing simultaneous aliases of the same element.
pub struct TensorViewMut<'a, T, S: LayoutShape = Kahen> {
    elements: &'a mut [T],
    layout: Layout<S>,
}

impl<'a, T, S: LayoutShape> TensorView<'a, T, S> {
    /// Bind a borrowed initialized host slice to a validated layout; reject an
    /// insufficient storage span or invalid CPU byte extent. No element copies.
    pub fn from_slice(elements: &'a [T], layout: Layout<S>) -> Result<Self, TensorError> {
        layout.validate_cpu_storage::<T>(elements.len())?;
        Ok(Self { elements, layout })
    }

    /// Borrow the validated affine layout. Strides are measured in elements. O(1).
    pub fn layout(&self) -> &Layout<S> {
        &self.layout
    }
    /// Borrow the validated shape without copying dimensions. O(1).
    pub fn shape(&self) -> &S {
        self.layout.shape()
    }
    /// Borrow signed element strides in axis order. Zero denotes broadcasting. O(1).
    pub fn strides(&self) -> &[isize] {
        self.layout.strides()
    }
    /// Return the number of axes. Scalars have rank zero. O(1).
    pub fn rank(&self) -> usize {
        self.layout.rank()
    }
    /// Return the logical element count. Scalars contain one element; any zero
    /// dimension makes the count zero. Shape validation prevents product overflow.
    pub fn numel(&self) -> usize {
        self.layout.numel()
    }
    /// Return whether there are no logical elements. A scalar is not empty.
    pub fn is_empty(&self) -> bool {
        self.layout.is_empty()
    }
    /// Borrow the entire physical host backing, including gaps outside a view.
    pub fn storage_slice(&self) -> &[T] {
        self.elements
    }

    /// Borrow one element at checked logical coordinates. Rank or bounds mismatches
    /// return an error. Signed/zero strides are respected. O(rank).
    pub fn get(&self, indices: &[usize]) -> Result<&T, TensorError> {
        let offset = self.layout.offset_of(indices)?;
        // Fully checked by from_slice and maintained by every transformation.
        Ok(&self.elements[offset])
    }

    /// Borrow logical elements as one slice in the requested C/F order.
    /// Returns NotContiguous when the affine mapping cannot provide that order. No allocation.
    pub fn as_contiguous_slice(&self, order: Order) -> Result<&[T], TensorError> {
        let range = contiguous_range(&self.layout, order)?;
        Ok(&self.elements[range])
    }

    fn with_layout<U: LayoutShape>(
        self,
        layout: Layout<U>,
    ) -> Result<TensorView<'a, T, U>, TensorError> {
        layout.validate_cpu_storage::<T>(self.elements.len())?;
        Ok(TensorView {
            elements: self.elements,
            layout,
        })
    }

    /// Reorder axes according to a complete permutation.
    /// Returns an error for duplicate, missing or out-of-range axes. O(rank).
    pub fn permute(self, axes: impl AsRef<[usize]>) -> Result<Self, TensorError> {
        let layout = self.layout.permute(axes)?;
        self.with_layout(layout)
    }

    /// Swap two existing axes without moving any stored elements; invalid axes return an error.
    pub fn swap_axes(self, left: usize, right: usize) -> Result<Self, TensorError> {
        let layout = self.layout.swap_axes(left, right)?;
        self.with_layout(layout)
    }

    /// Reverse axis order using metadata without moving elements.
    pub fn transpose(self) -> Result<Self, TensorError> {
        let layout = self.layout.transpose()?;
        self.with_layout(layout)
    }

    /// Reverse logical traversal along an axis using a negative stride.
    /// Returns an error for an invalid axis or unrepresentable address arithmetic.
    pub fn reverse_axis(self, axis: usize) -> Result<Self, TensorError> {
        let layout = self.layout.reverse_axis(axis)?;
        self.with_layout(layout)
    }

    /// Create a view using explicit source start, output length and nonzero
    /// signed step. Invalid axes, bounds or arithmetic return an error; no element copies.
    pub fn slice_axis(
        self,
        axis: usize,
        start: usize,
        len: usize,
        step: isize,
    ) -> Result<Self, TensorError> {
        let layout = self.layout.slice_axis(axis, start, len, step)?;
        self.with_layout(layout)
    }

    /// Insert a singleton axis at a position in 0..=rank.
    /// Returns an axis error outside that range. Element count is unchanged. O(rank).
    pub fn insert_axis(self, axis: usize) -> Result<TensorView<'a, T, Kahen>, TensorError> {
        let layout = self.layout.insert_axis(axis)?;
        self.with_layout(layout)
    }

    /// Remove a singleton axis, returning an error for an invalid axis or a
    /// non-singleton dimension. Element count is unchanged. O(rank).
    pub fn squeeze_axis(self, axis: usize) -> Result<TensorView<'a, T, Kahen>, TensorError> {
        let layout = self.layout.squeeze_axis(axis)?;
        self.with_layout(layout)
    }

    /// Create a view with missing/singleton axes expanded through zero strides.
    /// Invalid directional broadcasting returns an error; no element copies.
    pub fn broadcast_to(
        self,
        dims: impl AsRef<[usize]>,
    ) -> Result<TensorView<'a, T, Kahen>, TensorError> {
        let layout = self.layout.broadcast_to(dims)?;
        self.with_layout(layout)
    }

    /// Reinterpret logical traversal using a new shape and C/F order without
    /// copying. Element count must match and stride chunks must permit a view, otherwise error.
    pub fn reshape_view(
        self,
        dims: impl AsRef<[usize]>,
        order: Order,
    ) -> Result<TensorView<'a, T, Kahen>, TensorError> {
        let layout = self.layout.reshape_view(dims, order)?;
        self.with_layout(layout)
    }

    /// Iterate logical elements in C/F order, respecting gaps, negative and zero
    /// strides. The iterator borrows data and owns a copy of layout metadata.
    pub fn iter(&self, order: Order) -> TensorIter<'_, T, S> {
        TensorIter::new(self.elements, self.layout.clone(), order)
            .expect("a validated tensor view always yields a valid iterator")
    }

    /// Materialize exactly the logical elements, in C- or F-order, copying
    /// values by `Clone` rather than copying raw object bytes.
    pub fn to_owned_contiguous(
        &self,
        order: Order,
    ) -> Result<Tensor<T, Kahen, CpuBuffer<T>>, TensorError>
    where
        T: Clone,
    {
        let shape = Kahen::new(self.shape().dims()).map_err(LayoutError::from)?;
        let result = Tensor::<T, Kahen, CpuBuffer<T>>::try_from_fn(
            shape,
            order,
            align_of::<T>(),
            |indices| self.get(indices).cloned(),
        );
        result.map_err(|error| match error {
            TensorBuildError::Tensor(error) => error,
            TensorBuildError::Initialize { error, .. } => error,
        })
    }
}

impl<'a, T, S: LayoutShape> Clone for TensorView<'a, T, S> {
    fn clone(&self) -> Self {
        Self {
            elements: self.elements,
            layout: self.layout.clone(),
        }
    }
}

impl<'a, T, S: LayoutShape> TensorViewMut<'a, T, S> {
    /// Bind a borrowed initialized host slice to a validated layout; reject an
    /// insufficient storage span or invalid CPU byte extent. No element copies.
    pub fn from_slice(elements: &'a mut [T], layout: Layout<S>) -> Result<Self, TensorError> {
        layout.validate_cpu_storage::<T>(elements.len())?;
        Ok(Self { elements, layout })
    }

    /// Borrow the validated affine layout. Strides are measured in elements. O(1).
    pub fn layout(&self) -> &Layout<S> {
        &self.layout
    }
    /// Borrow the validated shape without copying dimensions. O(1).
    pub fn shape(&self) -> &S {
        self.layout.shape()
    }
    /// Borrow signed element strides in axis order. Zero denotes broadcasting. O(1).
    pub fn strides(&self) -> &[isize] {
        self.layout.strides()
    }
    /// Return the number of axes. Scalars have rank zero. O(1).
    pub fn rank(&self) -> usize {
        self.layout.rank()
    }
    /// Return the logical element count. Scalars contain one element; any zero
    /// dimension makes the count zero. Shape validation prevents product overflow.
    pub fn numel(&self) -> usize {
        self.layout.numel()
    }
    /// Return whether there are no logical elements. A scalar is not empty.
    pub fn is_empty(&self) -> bool {
        self.layout.is_empty()
    }

    /// Borrow one element at checked logical coordinates. Rank or bounds mismatches
    /// return an error. Signed/zero strides are respected. O(rank).
    pub fn get(&self, indices: &[usize]) -> Result<&T, TensorError> {
        let offset = self.layout.offset_of(indices)?;
        Ok(&self.elements[offset])
    }

    /// Each mutable reference lives only for the exclusive borrow of `self`.
    /// Even two different logical indices may map to one physical element.
    pub fn get_mut(&mut self, indices: &[usize]) -> Result<&mut T, TensorError> {
        let offset = self.layout.offset_of(indices)?;
        Ok(&mut self.elements[offset])
    }

    /// Borrow logical elements as one slice in the requested C/F order.
    /// Returns NotContiguous when the affine mapping cannot provide that order. No allocation.
    pub fn as_contiguous_slice(&self, order: Order) -> Result<&[T], TensorError> {
        let range = contiguous_range(&self.layout, order)?;
        Ok(&self.elements[range])
    }

    /// Exclusively borrow logical elements as a contiguous C/F-ordered slice.
    /// Returns an error for noncontiguous layouts. No allocation.
    pub fn as_contiguous_mut_slice(&mut self, order: Order) -> Result<&mut [T], TensorError> {
        let range = contiguous_range(&self.layout, order)?;
        Ok(&mut self.elements[range])
    }

    /// Borrow an exclusive view as read-only for the duration of this borrow.
    pub fn as_read_only(&self) -> TensorView<'_, T, S> {
        TensorView {
            elements: &*self.elements,
            layout: self.layout.clone(),
        }
    }

    /// Iterate logical elements in C/F order, respecting gaps, negative and zero
    /// strides. The iterator borrows data and owns a copy of layout metadata.
    pub fn iter(&self, order: Order) -> TensorIter<'_, T, S> {
        TensorIter::new(&*self.elements, self.layout.clone(), order)
            .expect("a validated mutable tensor view always yields a valid iterator")
    }

    /// Consume an exclusive view and retain its remaining lifetime as a read-only view.
    pub fn into_read_only(self) -> TensorView<'a, T, S> {
        TensorView {
            elements: self.elements,
            layout: self.layout,
        }
    }

    fn with_layout<U: LayoutShape>(
        self,
        layout: Layout<U>,
    ) -> Result<TensorViewMut<'a, T, U>, TensorError> {
        layout.validate_cpu_storage::<T>(self.elements.len())?;
        Ok(TensorViewMut {
            elements: self.elements,
            layout,
        })
    }

    /// Metadata-only transformations consume the exclusive view, ensuring that
    /// there cannot be a second independently usable mutable view.
    pub fn permute(self, axes: impl AsRef<[usize]>) -> Result<Self, TensorError> {
        let layout = self.layout.permute(axes)?;
        self.with_layout(layout)
    }

    /// Swap two existing axes without moving any stored elements; invalid axes return an error.
    pub fn swap_axes(self, left: usize, right: usize) -> Result<Self, TensorError> {
        let layout = self.layout.swap_axes(left, right)?;
        self.with_layout(layout)
    }

    /// Reverse axis order using metadata without moving elements.
    pub fn transpose(self) -> Result<Self, TensorError> {
        let layout = self.layout.transpose()?;
        self.with_layout(layout)
    }

    /// Reverse logical traversal along an axis using a negative stride.
    /// Returns an error for an invalid axis or unrepresentable address arithmetic.
    pub fn reverse_axis(self, axis: usize) -> Result<Self, TensorError> {
        let layout = self.layout.reverse_axis(axis)?;
        self.with_layout(layout)
    }

    /// Create a view using explicit source start, output length and nonzero
    /// signed step. Invalid axes, bounds or arithmetic return an error; no element copies.
    pub fn slice_axis(
        self,
        axis: usize,
        start: usize,
        len: usize,
        step: isize,
    ) -> Result<Self, TensorError> {
        let layout = self.layout.slice_axis(axis, start, len, step)?;
        self.with_layout(layout)
    }

    /// Insert a singleton axis at a position in 0..=rank.
    /// Returns an axis error outside that range. Element count is unchanged. O(rank).
    pub fn insert_axis(self, axis: usize) -> Result<TensorViewMut<'a, T, Kahen>, TensorError> {
        let layout = self.layout.insert_axis(axis)?;
        self.with_layout(layout)
    }

    /// Remove a singleton axis, returning an error for an invalid axis or a
    /// non-singleton dimension. Element count is unchanged. O(rank).
    pub fn squeeze_axis(self, axis: usize) -> Result<TensorViewMut<'a, T, Kahen>, TensorError> {
        let layout = self.layout.squeeze_axis(axis)?;
        self.with_layout(layout)
    }

    /// Create a view with missing/singleton axes expanded through zero strides.
    /// Invalid directional broadcasting returns an error; no element copies.
    pub fn broadcast_to(
        self,
        dims: impl AsRef<[usize]>,
    ) -> Result<TensorViewMut<'a, T, Kahen>, TensorError> {
        let layout = self.layout.broadcast_to(dims)?;
        self.with_layout(layout)
    }

    /// Reinterpret logical traversal using a new shape and C/F order without
    /// copying. Element count must match and stride chunks must permit a view, otherwise error.
    pub fn reshape_view(
        self,
        dims: impl AsRef<[usize]>,
        order: Order,
    ) -> Result<TensorViewMut<'a, T, Kahen>, TensorError> {
        let layout = self.layout.reshape_view(dims, order)?;
        self.with_layout(layout)
    }
}
