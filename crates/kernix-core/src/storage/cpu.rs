use alloc::{
    alloc::{Layout as AllocLayout, alloc, dealloc},
    boxed::Box,
    vec::Vec,
};
use core::{
    fmt,
    mem::{align_of, size_of},
    ptr::NonNull,
    slice,
};

use super::{
    StorageBuildError, StorageError,
    buffer::{HostBuffer, Storage, StorageBuffer},
};

/// CPU-owned elements, stored either in a moved `Vec<T>` (zero-copy adoption)
/// or in an explicitly aligned allocation.
pub struct CpuBuffer<T> {
    backing: Backing<T>,
}

enum Backing<T> {
    Vec(Vec<T>),
    Aligned(AlignedAllocation<T>),
}

/// Exactly `initialized` valid values of `T` occupy the beginning of `ptr`.
/// The allocation and its `AllocLayout` are exclusively owned by this object.
/// Construction and movement between buffers never copy `T` as raw bytes.
struct AlignedAllocation<T> {
    ptr: NonNull<T>,
    initialized: usize,
    allocation: Option<AllocLayout>,
    alignment: usize,
}

// SAFETY: the pointer exclusively owns its allocation. Access to mutable
// elements requires &mut self; T's own Send/Sync constraints still apply.
unsafe impl<T: Send> Send for AlignedAllocation<T> {}
// SAFETY: &AlignedAllocation only gives shared access to T elements.
unsafe impl<T: Sync> Sync for AlignedAllocation<T> {}

fn validate_len<T>(len: usize) -> Result<usize, StorageError> {
    let element_size = size_of::<T>();
    if element_size == 0 {
        return Err(StorageError::ZeroSizedElementType);
    }
    let bytes = len
        .checked_mul(element_size)
        .ok_or(StorageError::CapacityOverflow { len, element_size })?;
    if bytes > isize::MAX as usize {
        return Err(StorageError::CapacityOverflow { len, element_size });
    }
    Ok(bytes)
}

fn checked_layout<T>(
    len: usize,
    alignment: usize,
) -> Result<(usize, Option<AllocLayout>), StorageError> {
    let bytes = validate_len::<T>(len)?;
    let minimum = align_of::<T>();
    if alignment < minimum || !alignment.is_power_of_two() {
        return Err(StorageError::InvalidAlignment {
            requested: alignment,
            minimum,
        });
    }
    if len == 0 {
        return Ok((bytes, None));
    }
    let layout = AllocLayout::from_size_align(bytes, alignment).map_err(|_| {
        StorageError::CapacityOverflow {
            len,
            element_size: size_of::<T>(),
        }
    })?;
    Ok((bytes, Some(layout)))
}

impl<T> AlignedAllocation<T> {
    fn allocate(len: usize, alignment: usize) -> Result<Self, StorageError> {
        let (bytes, allocation) = checked_layout::<T>(len, alignment)?;
        let ptr = if let Some(layout) = allocation {
            // SAFETY: nonzero valid allocation layout. A null pointer denotes
            // allocation failure and is never exposed as a reference.
            let raw = unsafe { alloc(layout) };
            NonNull::new(raw.cast::<T>())
                .ok_or(StorageError::AllocationFailed { bytes, alignment })?
        } else {
            // Even an empty slice requires a non-null, T-aligned pointer.
            NonNull::dangling()
        };
        Ok(Self {
            ptr,
            initialized: 0,
            allocation,
            alignment,
        })
    }

    fn push(&mut self, value: T) {
        // SAFETY: called only once per slot in [0, allocated_len), in increasing
        // order, by loops bounded by the requested allocation length.
        unsafe {
            self.ptr.as_ptr().add(self.initialized).write(value);
        }
        self.initialized += 1;
    }

    fn as_slice(&self) -> &[T] {
        // SAFETY: initialized tracks precisely the written prefix; allocation
        // is live, correctly aligned, and length has passed byte-extent checks.
        unsafe { slice::from_raw_parts(self.ptr.as_ptr(), self.initialized) }
    }

    fn as_mut_slice(&mut self) -> &mut [T] {
        // SAFETY: same as as_slice, and &mut self ensures exclusive access.
        unsafe { slice::from_raw_parts_mut(self.ptr.as_ptr(), self.initialized) }
    }

    fn into_vec(mut self) -> Vec<T> {
        let len = self.initialized;
        let mut result = Vec::with_capacity(len);
        for i in 0..len {
            // SAFETY: each initialized value is moved exactly once. No drop
            // of its former location occurs after initialized is reset.
            result.push(unsafe { self.ptr.as_ptr().add(i).read() });
        }
        self.initialized = 0;
        result
    }
}

impl<T> Drop for AlignedAllocation<T> {
    fn drop(&mut self) {
        // SAFETY: the prefix contains exactly `initialized` valid values.
        unsafe {
            core::ptr::drop_in_place(core::ptr::slice_from_raw_parts_mut(
                self.ptr.as_ptr(),
                self.initialized,
            ));
        }
        if let Some(layout) = self.allocation {
            // SAFETY: ptr originated from alloc(layout) and is deallocated
            // exactly once after its initialized contents are dropped.
            unsafe {
                dealloc(self.ptr.as_ptr().cast::<u8>(), layout);
            }
        }
    }
}

impl<T> CpuBuffer<T> {
    /// Adopt a Vec with no element copies, moves, or new allocations.
    pub fn try_from_vec(values: Vec<T>) -> Result<Self, StorageError> {
        validate_len::<T>(values.len())?;
        Ok(Self {
            backing: Backing::Vec(values),
        })
    }

    /// Adopt a boxed slice without copying its elements.
    pub fn try_from_box(values: Box<[T]>) -> Result<Self, StorageError> {
        Self::try_from_vec(values.into_vec())
    }

    /// Adopt a Vec, reallocating with the requested alignment if required.
    /// Existing Vec allocations are preserved when natural alignment suffices.
    pub fn try_from_vec_aligned(values: Vec<T>, alignment: usize) -> Result<Self, StorageError> {
        let len = values.len();
        checked_layout::<T>(len, alignment)?;
        if alignment == align_of::<T>() {
            return Self::try_from_vec(values);
        }
        let mut buffer = AlignedAllocation::<T>::allocate(len, alignment)?;
        for value in values {
            buffer.push(value);
        }
        Ok(Self {
            backing: Backing::Aligned(buffer),
        })
    }

    /// Construct elements directly in an aligned allocation.
    /// If `init` panics, already-initialized elements are dropped exactly once.
    pub fn try_from_fn<F>(len: usize, alignment: usize, mut init: F) -> Result<Self, StorageError>
    where
        F: FnMut(usize) -> T,
    {
        let mut buffer = AlignedAllocation::<T>::allocate(len, alignment)?;
        for index in 0..len {
            buffer.push(init(index));
        }
        Ok(Self {
            backing: Backing::Aligned(buffer),
        })
    }

    /// Fallible initializer; any successfully constructed prefix is cleaned up
    /// on the first error (also on panic).
    pub fn try_from_fn_fallible<E, F>(
        len: usize,
        alignment: usize,
        mut init: F,
    ) -> Result<Self, StorageBuildError<E>>
    where
        F: FnMut(usize) -> Result<T, E>,
    {
        let mut buffer = AlignedAllocation::<T>::allocate(len, alignment)?;
        for index in 0..len {
            let value =
                init(index).map_err(|error| StorageBuildError::Initialize { index, error })?;
            buffer.push(value);
        }
        Ok(Self {
            backing: Backing::Aligned(buffer),
        })
    }

    /// Allocate aligned storage initialized by cloning a value.
    /// Reject invalid alignment, zero-sized types, extent overflow or allocation failure.
    pub fn try_filled(len: usize, alignment: usize, value: T) -> Result<Self, StorageError>
    where
        T: Clone,
    {
        Self::try_from_fn(len, alignment, |_| value.clone())
    }

    /// Allocate aligned storage initialized with T::default().
    /// Reject invalid alignment, zero-sized types, extent overflow or allocation failure.
    pub fn try_default(len: usize, alignment: usize) -> Result<Self, StorageError>
    where
        T: Default,
    {
        Self::try_from_fn(len, alignment, |_| T::default())
    }

    /// Return the number of stored elements (or addressed span elements for a span).
    pub fn len(&self) -> usize {
        self.as_slice().len()
    }
    /// Return whether there are no logical elements. A scalar is not empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Guaranteed alignment of the live allocation (for zero length, a
    /// requirement for any subsequently allocated element, not a pointer claim).
    pub fn alignment(&self) -> usize {
        match &self.backing {
            Backing::Vec(_) => align_of::<T>(),
            Backing::Aligned(a) => a.alignment,
        }
    }

    /// Test live pointer alignment; reject zero/non-power-of-two alignments.
    /// Empty storage satisfies every valid alignment without making a pointer guarantee.
    pub fn is_aligned_to(&self, alignment: usize) -> bool {
        if alignment == 0 || !alignment.is_power_of_two() {
            return false;
        }
        if self.is_empty() {
            return true;
        }
        (self.as_slice().as_ptr() as usize) % alignment == 0
    }

    /// Borrow the initialized host elements in physical order, not logical tensor order.
    pub fn as_slice(&self) -> &[T] {
        match &self.backing {
            Backing::Vec(v) => v.as_slice(),
            Backing::Aligned(a) => a.as_slice(),
        }
    }

    /// Exclusively borrow initialized host elements in physical order.
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        match &mut self.backing {
            Backing::Vec(v) => v.as_mut_slice(),
            Backing::Aligned(a) => a.as_mut_slice(),
        }
    }

    /// Return a Vec. This is zero-copy for a Vec backing; for an aligned
    /// allocation elements are moved into Vec-owned storage.
    pub fn into_vec(self) -> Vec<T> {
        match self.backing {
            Backing::Vec(v) => v,
            Backing::Aligned(a) => a.into_vec(),
        }
    }
}

impl<T> StorageBuffer<T> for CpuBuffer<T> {
    fn len(&self) -> usize {
        self.len()
    }
}

impl<T> HostBuffer<T> for CpuBuffer<T> {
    fn as_slice(&self) -> &[T] {
        self.as_slice()
    }
    fn as_mut_slice(&mut self) -> &mut [T] {
        self.as_mut_slice()
    }
}

impl<T: Clone> Clone for CpuBuffer<T> {
    fn clone(&self) -> Self {
        match &self.backing {
            Backing::Vec(v) => Self {
                backing: Backing::Vec(v.clone()),
            },
            Backing::Aligned(a) => {
                Self::try_from_fn(a.initialized, a.alignment, |i| a.as_slice()[i].clone())
                    .expect("failed to allocate when cloning CPU storage")
            }
        }
    }
}

impl<T> fmt::Debug for CpuBuffer<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CpuBuffer")
            .field("len", &self.len())
            .field("alignment", &self.alignment())
            .finish_non_exhaustive()
    }
}

impl<T> Storage<T, CpuBuffer<T>> {
    /// Adopt initialized Vec elements without copies; reject zero-sized element types
    /// and CPU byte extents exceeding isize::MAX.
    pub fn try_from_vec(values: Vec<T>) -> Result<Self, StorageError> {
        CpuBuffer::try_from_vec(values).map(Self::from_buffer)
    }
    /// Adopt a boxed slice without copies, checking element type and CPU byte extent.
    pub fn try_from_box(values: Box<[T]>) -> Result<Self, StorageError> {
        CpuBuffer::try_from_box(values).map(Self::from_buffer)
    }
    /// Adopt physical values, moving into an aligned allocation when needed.
    /// Reject invalid alignment/extent or allocation failure; runs in O(len) when moving.
    pub fn try_from_vec_aligned(values: Vec<T>, alignment: usize) -> Result<Self, StorageError> {
        CpuBuffer::try_from_vec_aligned(values, alignment).map(Self::from_buffer)
    }
    /// Allocate and initialize elements using a fallible callback. On the first error,
    /// return its physical index and clean up the initialized prefix. No partial tensor escapes.
    pub fn try_from_fn<F>(len: usize, alignment: usize, init: F) -> Result<Self, StorageError>
    where
        F: FnMut(usize) -> T,
    {
        CpuBuffer::try_from_fn(len, alignment, init).map(Self::from_buffer)
    }
    /// Allocate aligned storage with a fallible per-slot initializer.
    /// Return the first error and drop the initialized prefix exactly once, including on panic.
    pub fn try_from_fn_fallible<E, F>(
        len: usize,
        alignment: usize,
        init: F,
    ) -> Result<Self, StorageBuildError<E>>
    where
        F: FnMut(usize) -> Result<T, E>,
    {
        CpuBuffer::try_from_fn_fallible(len, alignment, init).map(Self::from_buffer)
    }
    /// Allocate aligned storage initialized by cloning a value.
    /// Reject invalid alignment, zero-sized types, extent overflow or allocation failure.
    pub fn try_filled(len: usize, alignment: usize, value: T) -> Result<Self, StorageError>
    where
        T: Clone,
    {
        CpuBuffer::try_filled(len, alignment, value).map(Self::from_buffer)
    }
    /// Allocate aligned storage initialized with T::default().
    /// Reject invalid alignment, zero-sized types, extent overflow or allocation failure.
    pub fn try_default(len: usize, alignment: usize) -> Result<Self, StorageError>
    where
        T: Default,
    {
        CpuBuffer::try_default(len, alignment).map(Self::from_buffer)
    }
    /// Consume storage into physical-order Vec elements. Vec-backed storage is moved
    /// without copying; explicitly aligned storage moves elements into a new Vec allocation.
    pub fn into_vec(self) -> Vec<T> {
        self.into_buffer().into_vec()
    }
    /// Return the allocation alignment guarantee in bytes. For empty allocations,
    /// this records the requirement rather than guaranteeing an over-aligned pointer.
    pub fn alignment(&self) -> usize {
        self.buffer().alignment()
    }
    /// Test live pointer alignment; reject zero/non-power-of-two alignments.
    /// Empty storage satisfies every valid alignment without making a pointer guarantee.
    pub fn is_aligned_to(&self, alignment: usize) -> bool {
        self.buffer().is_aligned_to(alignment)
    }
}

impl<T> TryFrom<Vec<T>> for Storage<T, CpuBuffer<T>> {
    type Error = StorageError;
    fn try_from(values: Vec<T>) -> Result<Self, Self::Error> {
        Self::try_from_vec(values)
    }
}

impl<T> TryFrom<Box<[T]>> for Storage<T, CpuBuffer<T>> {
    type Error = StorageError;
    fn try_from(values: Box<[T]>) -> Result<Self, Self::Error> {
        Self::try_from_box(values)
    }
}
