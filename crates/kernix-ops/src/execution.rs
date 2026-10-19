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
use crate::{OpsError, TensorInput, Zero};
#[cfg(feature = "parallel")]
use alloc::vec;
use alloc::vec::Vec;
use core::ops::{AddAssign, Mul};
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
    /// Number of explicitly configured workers; one denotes serial execution.
    pub fn threads(&self) -> usize {
        #[cfg(feature = "parallel")]
        if let Some(pool) = &self.pool {
            return pool.current_num_threads();
        }
        1
    }
    /// Allocate matrix multiplication output with deterministic scalar accumulation
    /// order per element. The contiguous rank-two path can distribute rows across
    /// workers; all other validated layouts use portable reference matmul.
    pub fn matmul<T, A, B>(&self, left: &A, right: &B) -> Result<Tensor<T>, OpsError>
    where
        T: Zero + Clone + AddAssign<T> + Mul<Output = T> + Send + Sync,
        A: TensorInput<T> + Sync,
        B: TensorInput<T> + Sync,
    {
        #[cfg(feature = "parallel")]
        if let Some(pool) = &self.pool {
            let a = left.dims();
            let b = right.dims();
            if a.len() == 2 && b.len() == 2 && a[1] == b[0] {
                if let (Some(av), Some(bv)) =
                    (left.as_c_contiguous_slice(), right.as_c_contiguous_slice())
                {
                    let (m, k, n) = (a[0], a[1], b[1]);
                    let shape = Kahen::new([m, n])?;
                    if m.checked_mul(k) == Some(av.len())
                        && k.checked_mul(n) == Some(bv.len())
                        && n > 0
                    {
                        use rayon::prelude::*;
                        let mut values = vec![T::zero(); shape.numel()];
                        pool.install(|| {
                            values.par_chunks_mut(n).enumerate().for_each(|(row, out)| {
                                for inner in 0..k {
                                    for (dst, b) in
                                        out.iter_mut().zip(&bv[inner * n..(inner + 1) * n])
                                    {
                                        *dst += av[row * k + inner].clone() * b.clone();
                                    }
                                }
                            })
                        });
                        return Ok(Tensor::from_vec(values, shape, kernix_core::Order::C)?);
                    }
                }
            }
        }
        crate::matmul(left, right)
    }
    /// Write rank-two C-contiguous matmul into an existing output allocation.
    ///
    /// No tensor or temporary element buffer is allocated. All shape/contiguity
    /// checks happen before writes. Output shape must be `[m,n]`; per-element
    /// accumulation follows increasing k. Scalar operator panics retain Rust
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
        T: Zero + Clone + AddAssign<T> + Mul<Output = T> + Send + Sync,
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
        let write_row = |(row, out): (usize, &mut [T])| {
            out.fill(T::zero());
            for inner in 0..k {
                for (dst, b) in out.iter_mut().zip(&bv[inner * n..(inner + 1) * n]) {
                    *dst += av[row * k + inner].clone() * b.clone();
                }
            }
        };
        #[cfg(feature = "parallel")]
        if let Some(pool) = &self.pool {
            use rayon::prelude::*;
            pool.install(|| values.par_chunks_mut(n).enumerate().for_each(write_row));
            return Ok(());
        }
        values.chunks_mut(n).enumerate().for_each(write_row);
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
    T: Zero + Clone + AddAssign<T> + Mul<Output = T> + Send + Sync,
{
    type KernelError = OpsError;
    fn matmul(&self, left: &Tensor<T>, right: &Tensor<T>) -> Result<Tensor<T>, OpsError> {
        CpuExecutor::matmul(self, left, right)
    }
}
