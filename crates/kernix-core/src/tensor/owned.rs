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

    /// Wrap backend storage; tensor construction additionally validates the reachable layout span.
    pub fn from_buffer(buffer: B, layout: Layout<S>) -> Result<Self, TensorError> {
        Self::from_storage(Storage::from_buffer(buffer), layout)
    }

    /// Consume the tensor and return its storage and layout without copying.
    pub fn into_parts(self) -> (Storage<T, B>, Layout<S>) {
        (self.storage, self.layout)
    }

    /// Borrow the storage owner. Layout-sensitive mutation is available through tensor views.
    pub fn storage(&self) -> &Storage<T, B> {
        &self.storage
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
    /// Return the physical element offset of logical index [0, ...]. O(1).
    pub fn origin(&self) -> usize {
        self.layout.origin()
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
    /// Return physical backing length, which can exceed the logical element count.
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

    /// Swap axis metadata in place, moving no elements. Invalid axes return an error.
    pub fn swap_axes_in_place(&mut self, left: usize, right: usize) -> Result<(), TensorError> {
        let layout = self.layout.swap_axes(left, right)?;
        self.set_layout(layout)
    }

    /// Reverse axis order in place, moving no elements.
    pub fn transpose_in_place(&mut self) -> Result<(), TensorError> {
        let layout = self.layout.transpose()?;
        self.set_layout(layout)
    }

    /// Reverse one axis in place using signed stride metadata; invalid axes
    /// or unrepresentable addresses return an error.
    pub fn reverse_axis_in_place(&mut self, axis: usize) -> Result<(), TensorError> {
        let layout = self.layout.reverse_axis(axis)?;
        self.set_layout(layout)
    }

    /// Replace an axis with start/length/signed-step slice metadata.
    /// Reject a zero step, out-of-range source indices or overflowing address arithmetic.
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

    /// Borrow one element at checked logical coordinates. Rank or bounds mismatches
    /// return an error. Signed/zero strides are respected. O(rank).
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

    /// Exclusively borrow one element at checked logical coordinates. The borrow
    /// is tied to self, preserving safety even for overlapping layouts. O(rank).
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
    /// Exclusively borrow the entire physical host backing. Does not change shape or layout.
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

    /// Exclusively borrow logical elements as a contiguous C/F-ordered slice.
    /// Returns an error for noncontiguous layouts. No allocation.
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

    /// Iterate logical elements in C/F order, respecting gaps, negative and zero
    /// strides. The iterator borrows data and owns a copy of layout metadata.
    pub fn iter(&self, order: Order) -> Result<TensorIter<'_, T, S>, TensorError> {
        TensorIter::new(self.storage.as_slice(), self.layout.clone(), order)
    }

    /// All view transformations leave the allocation in place.
    pub fn permute(&self, axes: impl AsRef<[usize]>) -> Result<TensorView<'_, T, S>, TensorError> {
        self.view()?.permute(axes)
    }

    /// Swap two existing axes without moving any stored elements; invalid axes return an error.
    pub fn swap_axes(
        &self,
        left: usize,
        right: usize,
    ) -> Result<TensorView<'_, T, S>, TensorError> {
        self.view()?.swap_axes(left, right)
    }

    /// Reverse axis order using metadata without moving elements.
    pub fn transpose(&self) -> Result<TensorView<'_, T, S>, TensorError> {
        self.view()?.transpose()
    }

    /// Reverse logical traversal along an axis using a negative stride.
    /// Returns an error for an invalid axis or unrepresentable address arithmetic.
    pub fn reverse_axis(&self, axis: usize) -> Result<TensorView<'_, T, S>, TensorError> {
        self.view()?.reverse_axis(axis)
    }

    /// Create a view using explicit source start, output length and nonzero
    /// signed step. Invalid axes, bounds or arithmetic return an error; no element copies.
    pub fn slice_axis(
        &self,
        axis: usize,
        start: usize,
        len: usize,
        step: isize,
    ) -> Result<TensorView<'_, T, S>, TensorError> {
        self.view()?.slice_axis(axis, start, len, step)
    }

    /// Insert a singleton axis at a position in 0..=rank.
    /// Returns an axis error outside that range. Element count is unchanged. O(rank).
    pub fn insert_axis(&self, axis: usize) -> Result<TensorView<'_, T, Kahen>, TensorError> {
        self.view()?.insert_axis(axis)
    }

    /// Remove a singleton axis, returning an error for an invalid axis or a
    /// non-singleton dimension. Element count is unchanged. O(rank).
    pub fn squeeze_axis(&self, axis: usize) -> Result<TensorView<'_, T, Kahen>, TensorError> {
        self.view()?.squeeze_axis(axis)
    }

    /// Create a view with missing/singleton axes expanded through zero strides.
    /// Invalid directional broadcasting returns an error; no element copies.
    pub fn broadcast_to(
        &self,
        dims: impl AsRef<[usize]>,
    ) -> Result<TensorView<'_, T, Kahen>, TensorError> {
        self.view()?.broadcast_to(dims)
    }

    /// Reinterpret logical traversal using a new shape and C/F order without
    /// copying. Element count must match and stride chunks must permit a view, otherwise error.
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

    /// Adopt a boxed slice with matching element count and canonical strides.
    /// Reject invalid layout, zero-sized element types and address extent overflow.
    pub fn from_box(values: Box<[T]>, shape: S, order: Order) -> Result<Self, TensorError> {
        Self::from_vec(values.into_vec(), shape, order)
    }

    /// Construct from physical values in the requested order and alignment.
    /// Length must equal numel; alignment must be a power of two at least `align_of::<T>()`.
    /// Reallocation may be required; invalid extent, layout or allocation returns an error.
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

    /// Allocate and initialize elements using a fallible callback. On the first error,
    /// return its physical index and clean up the initialized prefix. No partial tensor escapes.
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

    /// Allocate a canonical tensor initialized by cloning one value.
    /// Returns layout, alignment, extent or allocation errors; takes O(numel).
    pub fn filled(shape: S, order: Order, alignment: usize, value: T) -> Result<Self, TensorError>
    where
        T: Clone,
    {
        let layout = Layout::contiguous(shape, order)?;
        let storage = Storage::<T, CpuBuffer<T>>::try_filled(layout.numel(), alignment, value)?;
        Self::from_host_storage(storage, layout)
    }

    /// Allocate a canonical tensor initialized with T::default().
    /// Returns layout, alignment, extent or allocation errors; takes O(numel).
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
