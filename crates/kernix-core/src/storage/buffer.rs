use core::{
    fmt,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use super::cpu::CpuBuffer;

/// Contract for a backend-owned buffer of fully initialized `T` elements.
///
/// GPU implementers may provide only `StorageBuffer`; they are NOT required to
/// offer host pointers, blocking mappings, or synchronously accessible slices.
/// The element count is logical; backing allocation and synchronization are
/// properties of the concrete backend.
pub trait StorageBuffer<T> {
    /// Return the number of stored elements (or addressed span elements for a span).
    fn len(&self) -> usize;

    /// Return whether there are no logical elements. A scalar is not empty.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// A buffer whose initialized elements can be borrowed synchronously on CPU.
///
/// Safe references returned by these methods must follow Rust's normal aliasing
/// rules. A device buffer must not implement this trait unless it can uphold
/// those contracts for the lifetime of each borrow.
pub trait HostBuffer<T>: StorageBuffer<T> {
    /// Borrow the initialized host elements in physical order, not logical tensor order.
    fn as_slice(&self) -> &[T];
    /// Exclusively borrow initialized host elements in physical order.
    fn as_mut_slice(&mut self) -> &mut [T];
}

/// Owns backend storage without imposing a CPU pointer on device backends.
///
/// `Storage<T>` defaults to `CpuBuffer<T>`; a new backend supplies another `B`
/// implementing `StorageBuffer<T>` without changing layout or tensor metadata.
pub struct Storage<T, B: StorageBuffer<T> = CpuBuffer<T>> {
    buffer: B,
    marker: PhantomData<fn() -> T>,
}

/// CPU-owned initialized storage with checked allocation and alignment.
pub type CpuStorage<T> = Storage<T, CpuBuffer<T>>;

/// Atomic shared ownership of backend storage; mutation requires exclusive ownership.
#[cfg(target_has_atomic = "ptr")]
pub type SharedStorage<T, B = CpuBuffer<T>> = alloc::sync::Arc<Storage<T, B>>;

impl<T, B: StorageBuffer<T>> Storage<T, B> {
    /// Wrap backend storage; tensor construction additionally validates the reachable layout span.
    pub fn from_buffer(buffer: B) -> Self {
        Self {
            buffer,
            marker: PhantomData,
        }
    }

    /// Borrow the backend buffer without changing its ownership.
    pub fn buffer(&self) -> &B {
        &self.buffer
    }
    /// Exclusively borrow the backend buffer. Layout bindings must be validated
    /// again after changing the allocation or its length.
    pub fn buffer_mut(&mut self) -> &mut B {
        &mut self.buffer
    }
    /// Consume this storage wrapper and return its backend buffer without copying.
    pub fn into_buffer(self) -> B {
        self.buffer
    }
    /// Return the number of stored elements (or addressed span elements for a span).
    pub fn len(&self) -> usize {
        self.buffer.len()
    }
    /// Return whether there are no logical elements. A scalar is not empty.
    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    /// Move storage into an atomic reference-counted owner without copying elements.
    #[cfg(target_has_atomic = "ptr")]
    pub fn into_shared(self) -> SharedStorage<T, B> {
        alloc::sync::Arc::new(self)
    }
}

impl<T, B: StorageBuffer<T> + Clone> Clone for Storage<T, B> {
    fn clone(&self) -> Self {
        Self::from_buffer(self.buffer.clone())
    }
}

impl<T, B: StorageBuffer<T> + fmt::Debug> fmt::Debug for Storage<T, B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Storage")
            .field("buffer", &self.buffer)
            .finish()
    }
}

impl<T, B: HostBuffer<T>> Storage<T, B> {
    /// Borrow the initialized host elements in physical order, not logical tensor order.
    pub fn as_slice(&self) -> &[T] {
        self.buffer.as_slice()
    }
    /// Exclusively borrow initialized host elements in physical order.
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        self.buffer.as_mut_slice()
    }
}

impl<T, B: HostBuffer<T>> AsRef<[T]> for Storage<T, B> {
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T, B: HostBuffer<T>> AsMut<[T]> for Storage<T, B> {
    fn as_mut(&mut self) -> &mut [T] {
        self.as_mut_slice()
    }
}

impl<T, B: HostBuffer<T>> Deref for Storage<T, B> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<T, B: HostBuffer<T>> DerefMut for Storage<T, B> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_slice()
    }
}
