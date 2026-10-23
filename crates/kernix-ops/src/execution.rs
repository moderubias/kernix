//! Explicit CPU scheduling and backend execution capabilities.
//!
//! ```
//! use kernix_core::{Order, shape::Kahen, tensor::Tensor};
//! use kernix_ops::execution::CpuExecutor;
//! let cpu = CpuExecutor::new(1)?;
//! let a = Tensor::from_vec(vec![1.0f32,2.0,3.0,4.0],Kahen::new([2,2])?,Order::C)?;
//! let out = cpu.matmul(&a,&a)?;
//! assert_eq!(out.storage_slice(), &[7.0,10.0,15.0,22.0]);
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
use crate::{OpsError, TensorInput};
use alloc::vec::Vec;
use kernix_core::{
    device::{Cpu, Device},
    shape::Kahen,
    storage::{CpuBuffer, StorageError},
    tensor::Tensor,
};

/// Optional matrix multiplication capability for a device's opaque tensor buffers.
///
/// Backends implement this independently of transfers. Shapes/layouts remain device
/// agnostic; a successful result belongs to the receiving execution context.
pub trait MatmulDevice<T>: Device<T> {
    /// Checked kernel, layout, ownership, allocation or synchronization failure.
    type KernelError: core::error::Error;
    /// Multiply device tensors using NumPy matmul rank/batch semantics.
    fn matmul(
        &self,
        left: &Tensor<T, Kahen, Self::Buffer>,
        right: &Tensor<T, Kahen, Self::Buffer>,
    ) -> Result<Tensor<T, Kahen, Self::Buffer>, Self::KernelError>;
}

/// CPU execution context with a private, explicitly sized thread pool when enabled.
///
/// One thread is the default and works with no_std+alloc. More threads require
/// `parallel`; no global Rayon settings are modified. Parallel matmul is currently
/// restricted to C-contiguous rank-two inputs; other layouts use the serial fallback.
#[derive(Clone, Default)]
pub struct CpuExecutor {
    ordered_matmul: bool,
    #[cfg(feature = "parallel")]
    pool: Option<alloc::sync::Arc<rayon::ThreadPool>>,
}
impl CpuExecutor {
    /// Build a CPU executor with exactly the requested number of worker threads.
    /// Zero is invalid; requests above one require `parallel`. Thread creation errors
    /// return InvalidParameter rather than silently selecting a different policy.
    pub fn new(threads: usize) -> Result<Self, OpsError> {
        if threads == 0 {
            return Err(invalid_threads());
        }
        if threads == 1 {
            return Ok(Self::default());
        }
        #[cfg(feature = "parallel")]
        {
            Ok(Self {
                ordered_matmul: false,
                pool: Some(alloc::sync::Arc::new(
                    rayon::ThreadPoolBuilder::new()
                        .num_threads(threads)
                        .build()
                        .map_err(|_| invalid_threads())?,
                )),
            })
        }
        #[cfg(not(feature = "parallel"))]
        Err(invalid_threads())
    }
    /// Select strictly increasing-k matrix accumulation instead of native packed
    /// floating kernels. Ordered mode also avoids packing allocation in matmul_into.
    pub fn with_ordered_matmul(mut self, ordered: bool) -> Self {
        self.ordered_matmul = ordered;
        self
    }
    /// Whether native floating GEMM dispatch is disabled for this context.
    pub fn ordered_matmul(&self) -> bool {
        self.ordered_matmul
    }
    /// Number of explicitly configured workers; one denotes serial execution.
    pub fn threads(&self) -> usize {
        #[cfg(feature = "parallel")]
        if let Some(pool) = &self.pool {
            return pool.current_num_threads();
        }
        1
    }
    /// Allocate matrix multiplication output with native packed floating kernels,
    /// or increasing-k scalar accumulation when ordered_matmul is enabled. The contiguous rank-two path can distribute rows across
    /// workers; other validated layouts use serial native dispatch or the portable
    /// fallback, according to layout/scalar capabilities and the selected policy.
    pub fn matmul<T, A, B>(&self, left: &A, right: &B) -> Result<Tensor<T>, OpsError>
    where
        T: crate::MatmulElement + Send + Sync,
        A: TensorInput<T> + Sync,
        B: TensorInput<T> + Sync,
    {
        #[cfg(feature = "parallel")]
        if self.pool.is_some() {
            let a = left.dims();
            let b = right.dims();
            if a.len() == 2
                && b.len() == 2
                && a[1] == b[0]
                && left.as_c_contiguous_slice().is_some()
                && right.as_c_contiguous_slice().is_some()
            {
                let mut output = Tensor::filled(
                    Kahen::new([a[0], b[1]])?,
                    kernix_core::Order::C,
                    core::mem::align_of::<T>(),
                    T::zero(),
                )?;
                self.matmul_into(left, right, &mut output)?;
                return Ok(output);
            }
        }
        if self.ordered_matmul {
            crate::matmul_reference(left, right)
        } else {
            crate::matmul(left, right)
        }
    }
    /// Write rank-two C-contiguous matmul into an existing output allocation.
    ///
    /// Output tensor storage is reused. Native kernels may allocate packed workspace
    /// and retain thread-local packing memory until thread exit. Reusing output does
    /// not imply scratch-free execution. with_ordered_matmul(true) avoids
    /// element-buffer allocations. All shape/contiguity
    /// checks happen before writes. Output shape must be `[m,n]`; per-element
    /// accumulation follows the selected native/ordered policy. Scalar operator panics retain Rust
    /// semantics and can leave a partially updated output. Parallel contexts
    /// distribute independent rows. Other layouts should use [`Self::matmul`].
    ///
    /// ```
    /// use kernix_core::{Order, shape::Kahen, tensor::Tensor};
    /// use kernix_ops::execution::CpuExecutor;
    /// let a=Tensor::from_vec(vec![1.0f32,2.0,3.0,4.0],Kahen::new([2,2])?,Order::C)?;
    /// let mut out=Tensor::filled(Kahen::new([2,2])?,Order::C,4,0.0)?;
    /// CpuExecutor::default().matmul_into(&a,&a,&mut out)?;
    /// assert_eq!(out.storage_slice(),&[7.0,10.0,15.0,22.0]);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn matmul_into<T, A, B>(
        &self,
        left: &A,
        right: &B,
        output: &mut Tensor<T>,
    ) -> Result<(), OpsError>
    where
        T: crate::MatmulElement + Send + Sync,
        A: TensorInput<T> + Sync,
        B: TensorInput<T> + Sync,
    {
        let a = left.dims();
        let b = right.dims();
        let invalid = || OpsError::InvalidParameter {
            operation: "matmul_into",
            parameter: "rank-two contiguous inputs and output [m,n] required",
        };
        if a.len() != 2 || b.len() != 2 {
            return Err(invalid());
        }
        let (m, k, n) = (a[0], a[1], b[1]);
        if k != b[0] {
            return Err(OpsError::ContractDimensionMismatch {
                left: k,
                right: b[0],
            });
        }
        if output.shape().dims() != [m, n] {
            return Err(invalid());
        }
        let av = left.as_c_contiguous_slice().ok_or_else(invalid)?;
        let bv = right.as_c_contiguous_slice().ok_or_else(invalid)?;
        if m.checked_mul(k) != Some(av.len()) || k.checked_mul(n) != Some(bv.len()) {
            return Err(invalid());
        }
        let values = output.as_contiguous_mut_slice(kernix_core::Order::C)?;
        if values.is_empty() {
            return Ok(());
        }
        #[cfg(feature = "parallel")]
        if let Some(pool) = &self.pool {
            use rayon::prelude::*;
            let rows_per_chunk = m.clamp(1, 32);
            // Keep large-dense tasks eligible for the selected GEMM engine.
            // Small/skinny, ordered and portable work retains its previous split.
            #[cfg(all(feature = "std", target_arch = "x86_64", not(miri)))]
            let rows_per_chunk = if !self.ordered_matmul
                && T::HAS_NATIVE_MATMUL
                && m >= 128
                && k >= 64
                && n >= 64
                && gemm_common::simd::V3::try_new().is_some()
            {
                (m / pool.current_num_threads() / 64).max(1) * 64
            } else {
                rows_per_chunk
            };
            let chunk_size = n * rows_per_chunk;
            pool.install(|| {
                values
                    .par_chunks_mut(chunk_size)
                    .enumerate()
                    .for_each(|(chunk, out)| {
                        let row = chunk * rows_per_chunk;
                        let rows = out.len() / n;
                        let input = &av[row * k..(row + rows) * k];
                        if !self.ordered_matmul && T::try_dense_matmul(input, bv, out, rows, k, n) {
                            return;
                        }
                        crate::ops::dense_matmul(input, bv, out, rows, k, n, false);
                    })
            });
            return Ok(());
        }
        if !self.ordered_matmul && T::try_dense_matmul(av, bv, values, m, k, n) {
            return Ok(());
        }
        crate::ops::dense_matmul(av, bv, values, m, k, n, false);
        Ok(())
    }
}
fn invalid_threads() -> OpsError {
    OpsError::InvalidParameter {
        operation: "CpuExecutor::new",
        parameter: "threads (positive; >1 requires parallel and available workers)",
    }
}
impl<T: Clone> Device<T> for CpuExecutor {
    type Buffer = CpuBuffer<T>;
    type Error = StorageError;
    fn name(&self) -> &str {
        "cpu"
    }
    fn upload(&self, values: Vec<T>) -> Result<Self::Buffer, Self::Error> {
        Cpu.upload(values)
    }
    fn download(&self, buffer: &Self::Buffer) -> Result<Vec<T>, Self::Error> {
        Cpu.download(buffer)
    }
    fn synchronize(&self) -> Result<(), Self::Error> {
        Ok(())
    }
}
impl<T> MatmulDevice<T> for CpuExecutor
where
    T: crate::MatmulElement + Send + Sync,
{
    type KernelError = OpsError;
    fn matmul(&self, left: &Tensor<T>, right: &Tensor<T>) -> Result<Tensor<T>, OpsError> {
        CpuExecutor::matmul(self, left, right)
    }
}
