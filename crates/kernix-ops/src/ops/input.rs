use kernix_core::{
    layout::LayoutShape,
    storage::{HostBuffer, StorageBuffer},
    tensor::{Tensor, TensorError, TensorView, TensorViewMut},
};

/// Read-only host tensor contract for CPU reference kernels.
///
/// This is deliberately *not* implemented for opaque device buffers:
/// GPU execution must use device kernels, not synthetic host pointers.
pub trait TensorInput<T> {
    fn dims(&self) -> &[usize];
    fn get(&self, indices: &[usize]) -> Result<&T, TensorError>;
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
}

impl<T, S: LayoutShape> TensorInput<T> for TensorView<'_, T, S> {
    fn dims(&self) -> &[usize] {
        self.shape().dims()
    }
    fn get(&self, indices: &[usize]) -> Result<&T, TensorError> {
        TensorView::get(self, indices)
    }
}

impl<T, S: LayoutShape> TensorInput<T> for TensorViewMut<'_, T, S> {
    fn dims(&self) -> &[usize] {
        self.shape().dims()
    }
    fn get(&self, indices: &[usize]) -> Result<&T, TensorError> {
        TensorViewMut::get(self, indices)
    }
}
