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
    pub fn from_slice(elements: &'a [T], layout: Layout<S>) -> Result<Self, TensorError> {
        layout.validate_cpu_storage::<T>(elements.len())?;
        Ok(Self { elements, layout })
    }

    pub fn layout(&self) -> &Layout<S> {
        &self.layout
    }
    pub fn shape(&self) -> &S {
        self.layout.shape()
    }
    pub fn strides(&self) -> &[isize] {
        self.layout.strides()
    }
    pub fn rank(&self) -> usize {
        self.layout.rank()
    }
    pub fn numel(&self) -> usize {
        self.layout.numel()
    }
    pub fn is_empty(&self) -> bool {
        self.layout.is_empty()
    }
    pub fn storage_slice(&self) -> &[T] {
        self.elements
    }

    pub fn get(&self, indices: &[usize]) -> Result<&T, TensorError> {
        let offset = self.layout.offset_of(indices)?;
        // Fully checked by from_slice and maintained by every transformation.
        Ok(&self.elements[offset])
    }

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

    pub fn permute(self, axes: impl AsRef<[usize]>) -> Result<Self, TensorError> {
        let layout = self.layout.permute(axes)?;
        self.with_layout(layout)
    }

    pub fn swap_axes(self, left: usize, right: usize) -> Result<Self, TensorError> {
        let layout = self.layout.swap_axes(left, right)?;
        self.with_layout(layout)
    }

    pub fn transpose(self) -> Result<Self, TensorError> {
        let layout = self.layout.transpose()?;
        self.with_layout(layout)
    }

    pub fn reverse_axis(self, axis: usize) -> Result<Self, TensorError> {
        let layout = self.layout.reverse_axis(axis)?;
        self.with_layout(layout)
    }

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

    pub fn insert_axis(self, axis: usize) -> Result<TensorView<'a, T, Kahen>, TensorError> {
        let layout = self.layout.insert_axis(axis)?;
        self.with_layout(layout)
    }

    pub fn squeeze_axis(self, axis: usize) -> Result<TensorView<'a, T, Kahen>, TensorError> {
        let layout = self.layout.squeeze_axis(axis)?;
        self.with_layout(layout)
    }

    pub fn broadcast_to(
        self,
        dims: impl AsRef<[usize]>,
    ) -> Result<TensorView<'a, T, Kahen>, TensorError> {
        let layout = self.layout.broadcast_to(dims)?;
        self.with_layout(layout)
    }

    pub fn reshape_view(
        self,
        dims: impl AsRef<[usize]>,
        order: Order,
    ) -> Result<TensorView<'a, T, Kahen>, TensorError> {
        let layout = self.layout.reshape_view(dims, order)?;
        self.with_layout(layout)
    }

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
    pub fn from_slice(elements: &'a mut [T], layout: Layout<S>) -> Result<Self, TensorError> {
        layout.validate_cpu_storage::<T>(elements.len())?;
        Ok(Self { elements, layout })
    }

    pub fn layout(&self) -> &Layout<S> {
        &self.layout
    }
    pub fn shape(&self) -> &S {
        self.layout.shape()
    }
    pub fn strides(&self) -> &[isize] {
        self.layout.strides()
    }
    pub fn rank(&self) -> usize {
        self.layout.rank()
    }
    pub fn numel(&self) -> usize {
        self.layout.numel()
    }
    pub fn is_empty(&self) -> bool {
        self.layout.is_empty()
    }

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

    pub fn as_contiguous_slice(&self, order: Order) -> Result<&[T], TensorError> {
        let range = contiguous_range(&self.layout, order)?;
        Ok(&self.elements[range])
    }

    pub fn as_contiguous_mut_slice(&mut self, order: Order) -> Result<&mut [T], TensorError> {
        let range = contiguous_range(&self.layout, order)?;
        Ok(&mut self.elements[range])
    }

    pub fn as_read_only(&self) -> TensorView<'_, T, S> {
        TensorView {
            elements: &*self.elements,
            layout: self.layout.clone(),
        }
    }

    pub fn iter(&self, order: Order) -> TensorIter<'_, T, S> {
        TensorIter::new(&*self.elements, self.layout.clone(), order)
            .expect("a validated mutable tensor view always yields a valid iterator")
    }

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

    pub fn swap_axes(self, left: usize, right: usize) -> Result<Self, TensorError> {
        let layout = self.layout.swap_axes(left, right)?;
        self.with_layout(layout)
    }

    pub fn transpose(self) -> Result<Self, TensorError> {
        let layout = self.layout.transpose()?;
        self.with_layout(layout)
    }

    pub fn reverse_axis(self, axis: usize) -> Result<Self, TensorError> {
        let layout = self.layout.reverse_axis(axis)?;
        self.with_layout(layout)
    }

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

    pub fn insert_axis(self, axis: usize) -> Result<TensorViewMut<'a, T, Kahen>, TensorError> {
        let layout = self.layout.insert_axis(axis)?;
        self.with_layout(layout)
    }

    pub fn squeeze_axis(self, axis: usize) -> Result<TensorViewMut<'a, T, Kahen>, TensorError> {
        let layout = self.layout.squeeze_axis(axis)?;
        self.with_layout(layout)
    }

    pub fn broadcast_to(
        self,
        dims: impl AsRef<[usize]>,
    ) -> Result<TensorViewMut<'a, T, Kahen>, TensorError> {
        let layout = self.layout.broadcast_to(dims)?;
        self.with_layout(layout)
    }

    pub fn reshape_view(
        self,
        dims: impl AsRef<[usize]>,
        order: Order,
    ) -> Result<TensorViewMut<'a, T, Kahen>, TensorError> {
        let layout = self.layout.reshape_view(dims, order)?;
        self.with_layout(layout)
    }
}
