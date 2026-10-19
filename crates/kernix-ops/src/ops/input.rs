// Kernix numerics-softmax overlay 04 (applied by tools/kernix-update-04/apply.py)
use kernix_core::{
    layout::{LayoutShape, Order},
    storage::{HostBuffer, StorageBuffer},
    tensor::{Tensor, TensorError, TensorView, TensorViewMut},
};

/// Read-only host tensor contract for CPU reference kernels.
///
/// This is deliberately *not* implemented for opaque device buffers:
/// GPU execution must use device kernels, not synthetic host pointers.
pub trait TensorInput<T> {
    /// Borrow the dimension sizes in axis order, without allocating. O(1).
    fn dims(&self) -> &[usize];
    /// Borrow one element at checked logical coordinates. Rank or bounds mismatches
    /// return an error. Signed/zero strides are respected. O(rank).
    fn get(&self, indices: &[usize]) -> Result<&T, TensorError>;

    /// Optional C-order contiguous logical payload. The slice must have
    /// exactly product(dims()) elements in logical C traversal order.
    /// Device buffers and noncontiguous layouts return `None` by default.
    fn as_c_contiguous_slice(&self) -> Option<&[T]> {
        None
    }
}

impl<T, S, B> TensorInput<T> for Tensor<T, S, B>
where
    S: LayoutShape,
    B: StorageBuffer<T> + HostBuffer<T>,
{
    fn dims(&self) -> &[usize] {
        self.shape().dims()
    }
    fn get(&self, indices: &[usize]) -> Result<&T, TensorError> {
        Tensor::get(self, indices)
    }

    fn as_c_contiguous_slice(&self) -> Option<&[T]> {
        self.as_contiguous_slice(Order::C).ok()
    }
}

impl<T, S: LayoutShape> TensorInput<T> for TensorView<'_, T, S> {
    fn dims(&self) -> &[usize] {
        self.shape().dims()
    }
    fn get(&self, indices: &[usize]) -> Result<&T, TensorError> {
        TensorView::get(self, indices)
    }

    fn as_c_contiguous_slice(&self) -> Option<&[T]> {
        self.as_contiguous_slice(Order::C).ok()
    }
}

impl<T, S: LayoutShape> TensorInput<T> for TensorViewMut<'_, T, S> {
    fn dims(&self) -> &[usize] {
        self.shape().dims()
    }
    fn get(&self, indices: &[usize]) -> Result<&T, TensorError> {
        TensorViewMut::get(self, indices)
    }

    fn as_c_contiguous_slice(&self) -> Option<&[T]> {
        self.as_contiguous_slice(Order::C).ok()
    }
}
