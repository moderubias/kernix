use alloc::{boxed::Box, vec::Vec};
use core::{fmt, marker::PhantomData};
use smallvec::SmallVec;

use super::{
    TensorBuildError, TensorError, TensorIter, TensorView, TensorViewMut,
    index::{contiguous_range, unravel},
};
use crate::{
    layout::{Layout, LayoutShape, Order},
    shape::Kahen,
    storage::{CpuBuffer, HostBuffer, Storage, StorageBuffer},
};

/// An owned tensor combining a validated affine layout with backend storage.
///
/// `B` may represent host or device memory. Indexing via Rust references is
/// available only for backends explicitly implementing `HostBuffer<T>`.
///
/// `Tensor` never exposes `&mut Storage`: a caller must not resize/replace the
/// buffer behind a still-valid layout. To change ownership, use `into_parts`.
pub struct Tensor<T, S: LayoutShape = Kahen, B: StorageBuffer<T> = CpuBuffer<T>> {
    storage: Storage<T, B>,
    layout: Layout<S>,
    marker: PhantomData<fn() -> T>,
}

impl<T, S: LayoutShape, B: StorageBuffer<T>> Tensor<T, S, B> {
    /// Checks the *element* address span. A device-specific backend is
    /// responsible for byte extent, element type and synchronization.
    pub fn from_storage(storage: Storage<T, B>, layout: Layout<S>) -> Result<Self, TensorError> {
        layout.validate_storage_len(storage.len())?;
        Ok(Self {
            storage,
            layout,
            marker: PhantomData,
        })
    }

    pub fn from_buffer(buffer: B, layout: Layout<S>) -> Result<Self, TensorError> {
        Self::from_storage(Storage::from_buffer(buffer), layout)
    }

    pub fn into_parts(self) -> (Storage<T, B>, Layout<S>) {
        (self.storage, self.layout)
    }

    pub fn storage(&self) -> &Storage<T, B> {
        &self.storage
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
    pub fn origin(&self) -> usize {
        self.layout.origin()
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
    pub fn storage_len(&self) -> usize {
        self.storage.len()
    }

    /// Replace metadata without changing the buffer. On error, self is intact.
    pub fn set_layout(&mut self, layout: Layout<S>) -> Result<(), TensorError> {
        layout.validate_storage_len(self.storage.len())?;
        self.layout = layout;
        Ok(())
    }

    /// Metadata-only transformations, preserving storage and its ownership.
    pub fn permute_in_place(&mut self, axes: impl AsRef<[usize]>) -> Result<(), TensorError> {
        let layout = self.layout.permute(axes)?;
        self.set_layout(layout)
    }

    pub fn swap_axes_in_place(&mut self, left: usize, right: usize) -> Result<(), TensorError> {
        let layout = self.layout.swap_axes(left, right)?;
        self.set_layout(layout)
    }

    pub fn transpose_in_place(&mut self) -> Result<(), TensorError> {
        let layout = self.layout.transpose()?;
        self.set_layout(layout)
    }

    pub fn reverse_axis_in_place(&mut self, axis: usize) -> Result<(), TensorError> {
        let layout = self.layout.reverse_axis(axis)?;
        self.set_layout(layout)
    }

    pub fn slice_axis_in_place(
        &mut self,
        axis: usize,
        start: usize,
        len: usize,
        step: isize,
    ) -> Result<(), TensorError> {
        let layout = self.layout.slice_axis(axis, start, len, step)?;
        self.set_layout(layout)
    }
}

impl<T, S: LayoutShape, B: HostBuffer<T>> Tensor<T, S, B> {
    /// Checked CPU constructor: also verifies the full byte extent.
    pub fn from_host_storage(
        storage: Storage<T, B>,
        layout: Layout<S>,
    ) -> Result<Self, TensorError> {
        layout.validate_cpu_storage::<T>(storage.as_slice().len())?;
        Self::from_storage(storage, layout)
    }

    /// Borrow a zero-copy, read-only logical tensor view.
    pub fn view(&self) -> Result<TensorView<'_, T, S>, TensorError> {
        TensorView::from_slice(self.storage.as_slice(), self.layout.clone())
    }

    /// Borrow an exclusive zero-copy logical tensor view. Overlapping indices
    /// are safe because `get_mut` requires an exclusive borrow of the view.
    pub fn view_mut(&mut self) -> Result<TensorViewMut<'_, T, S>, TensorError> {
        let layout = self.layout.clone();
        TensorViewMut::from_slice(self.storage.as_mut_slice(), layout)
    }

    pub fn get(&self, indices: &[usize]) -> Result<&T, TensorError> {
        let offset = self.layout.offset_of(indices)?;
        self.storage.as_slice().get(offset).ok_or_else(|| {
            crate::layout::LayoutError::StorageTooSmall {
                required: offset + 1,
                available: self.storage.as_slice().len(),
            }
            .into()
        })
    }

    pub fn get_mut(&mut self, indices: &[usize]) -> Result<&mut T, TensorError> {
        let offset = self.layout.offset_of(indices)?;
        let available = self.storage.as_slice().len();
        self.storage.as_mut_slice().get_mut(offset).ok_or_else(|| {
            crate::layout::LayoutError::StorageTooSmall {
                required: offset + 1,
                available,
            }
            .into()
        })
    }

    /// Physical storage, not logical row-major tensor contents.
    pub fn storage_slice(&self) -> &[T] {
        self.storage.as_slice()
    }
    pub fn storage_slice_mut(&mut self) -> &mut [T] {
        self.storage.as_mut_slice()
    }

    /// Logical elements as one contiguous borrowed slice in the selected order.
    pub fn as_contiguous_slice(&self, order: Order) -> Result<&[T], TensorError> {
        let range = contiguous_range(&self.layout, order)?;
        self.storage.as_slice().get(range).ok_or_else(|| {
            crate::layout::LayoutError::StorageTooSmall {
                required: self.layout.required_storage_len(),
                available: self.storage.as_slice().len(),
            }
            .into()
        })
    }

    pub fn as_contiguous_mut_slice(&mut self, order: Order) -> Result<&mut [T], TensorError> {
        let range = contiguous_range(&self.layout, order)?;
        let available = self.storage.as_slice().len();
        let required = self.layout.required_storage_len();
        self.storage.as_mut_slice().get_mut(range).ok_or_else(|| {
            crate::layout::LayoutError::StorageTooSmall {
                required,
                available,
            }
            .into()
        })
    }

    pub fn iter(&self, order: Order) -> Result<TensorIter<'_, T, S>, TensorError> {
        TensorIter::new(self.storage.as_slice(), self.layout.clone(), order)
    }

    /// All view transformations leave the allocation in place.
    pub fn permute(&self, axes: impl AsRef<[usize]>) -> Result<TensorView<'_, T, S>, TensorError> {
        self.view()?.permute(axes)
    }

    pub fn swap_axes(
        &self,
        left: usize,
        right: usize,
    ) -> Result<TensorView<'_, T, S>, TensorError> {
        self.view()?.swap_axes(left, right)
    }

    pub fn transpose(&self) -> Result<TensorView<'_, T, S>, TensorError> {
        self.view()?.transpose()
    }

    pub fn reverse_axis(&self, axis: usize) -> Result<TensorView<'_, T, S>, TensorError> {
        self.view()?.reverse_axis(axis)
    }

    pub fn slice_axis(
        &self,
        axis: usize,
        start: usize,
        len: usize,
        step: isize,
    ) -> Result<TensorView<'_, T, S>, TensorError> {
        self.view()?.slice_axis(axis, start, len, step)
    }

    pub fn insert_axis(&self, axis: usize) -> Result<TensorView<'_, T, Kahen>, TensorError> {
        self.view()?.insert_axis(axis)
    }

    pub fn squeeze_axis(&self, axis: usize) -> Result<TensorView<'_, T, Kahen>, TensorError> {
        self.view()?.squeeze_axis(axis)
    }

    pub fn broadcast_to(
        &self,
        dims: impl AsRef<[usize]>,
    ) -> Result<TensorView<'_, T, Kahen>, TensorError> {
        self.view()?.broadcast_to(dims)
    }

    pub fn reshape_view(
        &self,
        dims: impl AsRef<[usize]>,
        order: Order,
    ) -> Result<TensorView<'_, T, Kahen>, TensorError> {
        self.view()?.reshape_view(dims, order)
    }

    /// Copies values in logical order, respecting arbitrary strides.
    pub fn to_owned_contiguous(&self, order: Order) -> Result<Tensor<T, Kahen>, TensorError>
    where
        T: Clone,
    {
        self.view()?.to_owned_contiguous(order)
    }
}

impl<T, S: LayoutShape> Tensor<T, S, CpuBuffer<T>> {
    /// Element count must exactly match the logical size. No copy when
    /// adopting a Vec; strides come from the requested canonical order.
    pub fn from_vec(values: Vec<T>, shape: S, order: Order) -> Result<Self, TensorError> {
        let expected = shape.numel();
        if values.len() != expected {
            return Err(TensorError::ElementCountMismatch {
                expected,
                actual: values.len(),
            });
        }
        let layout = Layout::contiguous(shape, order)?;
        let storage = Storage::<T, CpuBuffer<T>>::try_from_vec(values)?;
        Self::from_host_storage(storage, layout)
    }

    pub fn from_box(values: Box<[T]>, shape: S, order: Order) -> Result<Self, TensorError> {
        Self::from_vec(values.into_vec(), shape, order)
    }

    pub fn from_vec_aligned(
        values: Vec<T>,
        shape: S,
        order: Order,
        alignment: usize,
    ) -> Result<Self, TensorError> {
        let expected = shape.numel();
        if values.len() != expected {
            return Err(TensorError::ElementCountMismatch {
                expected,
                actual: values.len(),
            });
        }
        let layout = Layout::contiguous(shape, order)?;
        let storage = Storage::<T, CpuBuffer<T>>::try_from_vec_aligned(values, alignment)?;
        Self::from_host_storage(storage, layout)
    }

    /// Build in physical order; initializer receives *logical* N-D indices.
    pub fn from_fn<F>(
        shape: S,
        order: Order,
        alignment: usize,
        mut init: F,
    ) -> Result<Self, TensorError>
    where
        F: FnMut(&[usize]) -> T,
    {
        let layout = Layout::contiguous(shape, order)?;
        let mut coords: SmallVec<[usize; 6]> = core::iter::repeat_n(0, layout.rank()).collect();
        let storage =
            Storage::<T, CpuBuffer<T>>::try_from_fn(layout.numel(), alignment, |linear| {
                unravel(linear, layout.shape().dims(), order, coords.as_mut_slice());
                init(coords.as_slice())
            })?;
        Self::from_host_storage(storage, layout)
    }

    pub fn try_from_fn<E, F>(
        shape: S,
        order: Order,
        alignment: usize,
        mut init: F,
    ) -> Result<Self, TensorBuildError<E>>
    where
        F: FnMut(&[usize]) -> Result<T, E>,
    {
        let layout = Layout::contiguous(shape, order).map_err(TensorError::from)?;
        let mut coords: SmallVec<[usize; 6]> = core::iter::repeat_n(0, layout.rank()).collect();
        let storage = Storage::<T, CpuBuffer<T>>::try_from_fn_fallible(
            layout.numel(),
            alignment,
            |linear| {
                unravel(linear, layout.shape().dims(), order, coords.as_mut_slice());
                init(coords.as_slice())
            },
        )?;
        Self::from_host_storage(storage, layout).map_err(TensorBuildError::from)
    }

    pub fn filled(shape: S, order: Order, alignment: usize, value: T) -> Result<Self, TensorError>
    where
        T: Clone,
    {
        let layout = Layout::contiguous(shape, order)?;
        let storage = Storage::<T, CpuBuffer<T>>::try_filled(layout.numel(), alignment, value)?;
        Self::from_host_storage(storage, layout)
    }

    pub fn defaulted(shape: S, order: Order, alignment: usize) -> Result<Self, TensorError>
    where
        T: Default,
    {
        let layout = Layout::contiguous(shape, order)?;
        let storage = Storage::<T, CpuBuffer<T>>::try_default(layout.numel(), alignment)?;
        Self::from_host_storage(storage, layout)
    }

    /// Moves physical backing contents. This is NOT logical-order flattening
    /// for sliced/transposed tensors. Use `to_owned_contiguous` for that.
    pub fn into_storage_vec(self) -> Vec<T> {
        self.storage.into_vec()
    }
}

impl<T, S: LayoutShape, B: StorageBuffer<T> + Clone> Clone for Tensor<T, S, B> {
    fn clone(&self) -> Self {
        Self {
            storage: self.storage.clone(),
            layout: self.layout.clone(),
            marker: PhantomData,
        }
    }
}

impl<T, S: LayoutShape + fmt::Debug, B: StorageBuffer<T>> fmt::Debug for Tensor<T, S, B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Tensor")
            .field("layout", &self.layout)
            .field("storage_len", &self.storage.len())
            .finish_non_exhaustive()
    }
}
