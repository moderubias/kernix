//! Explicit Apple Accelerate execution; enabled only by the `accelerate` feature.
//!
//! Requires macOS 15+, an Apple SDK with BLAS thread control, and f32/f64 C-contiguous
//! rank-two inputs. The portable CPU default is unchanged. Each call requests one
//! BLAS thread and restores the calling thread's prior policy. No input conversion,
//! hidden fallback or device transfer occurs. Native workspace allocation is managed
//! by Accelerate. This backend does not currently record an autodiff operation.
use crate::{OpsError, TensorInput, execution::MatmulDevice};
use alloc::vec::Vec;
use kernix_core::{
    device::{Cpu, Device},
    shape::Kahen,
    storage::{CpuBuffer, StorageError},
    tensor::Tensor,
};

/// An explicit single-thread Apple Accelerate backend for dense f32/f64 matrices.
///
/// Construction fails on unsupported systems instead of choosing a different backend.
/// Output allocation is fallible; accepted inputs remain borrowed and unchanged.
#[derive(Debug, Clone, Copy)]
pub struct AccelerateExecutor {
    _private: (),
}
impl AccelerateExecutor {
    /// Whether this system accepts Accelerate's single-thread policy (macOS 15+).
    /// Both requesting and restoring the thread-local policy are checked;
    /// an SDK refusal returns false.
    pub fn is_available() -> bool {
        #[cfg(target_os = "macos")]
        {
            // SAFETY: SDK bridge has no pointer parameters and checks OS availability.
            unsafe { ffi::kernix_accelerate_available() == 0 }
        }
        #[cfg(not(target_os = "macos"))]
        {
            false
        }
    }
    /// Select single-thread Accelerate explicitly, or return an availability error.
    pub fn new() -> Result<Self, OpsError> {
        if Self::is_available() {
            Ok(Self { _private: () })
        } else {
            Err(unavailable())
        }
    }
    /// Allocate and multiply C-contiguous rank-two f32/f64 tensors.
    ///
    /// Other ranks/layouts are rejected, without implicit copying or CPU fallback.
    /// Dimensions passed to the SDK must fit a signed 32-bit integer.
    pub fn matmul<T>(&self, left: &Tensor<T>, right: &Tensor<T>) -> Result<Tensor<T>, OpsError>
    where
        Self: MatmulDevice<T, Buffer = CpuBuffer<T>, KernelError = OpsError>,
    {
        <Self as MatmulDevice<T>>::matmul(self, left, right)
    }
}
impl<T: Clone> Device<T> for AccelerateExecutor {
    type Buffer = CpuBuffer<T>;
    type Error = StorageError;
    fn name(&self) -> &str {
        "apple-accelerate-single-thread"
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
fn unavailable() -> OpsError {
    OpsError::InvalidParameter {
        operation: "AccelerateExecutor",
        parameter: "macOS 15+ with available single-thread BLAS is required",
    }
}
fn invalid(parameter: &'static str) -> OpsError {
    OpsError::InvalidParameter {
        operation: "AccelerateExecutor::matmul",
        parameter,
    }
}
macro_rules! scalar {
    ($ty:ty,$kernel:ident) => {
        impl MatmulDevice<$ty> for AccelerateExecutor {
            type KernelError = OpsError;
            fn matmul(
                &self,
                left: &Tensor<$ty>,
                right: &Tensor<$ty>,
            ) -> Result<Tensor<$ty>, OpsError> {
                let (a, b) = (left.dims(), right.dims());
                if a.len() != 2 || b.len() != 2 {
                    return Err(invalid("rank-two inputs required"));
                }
                let (m, k, n) = (a[0], a[1], b[1]);
                if k != b[0] {
                    return Err(OpsError::ContractDimensionMismatch {
                        left: k,
                        right: b[0],
                    });
                }
                let av = left
                    .as_c_contiguous_slice()
                    .ok_or_else(|| invalid("C-contiguous left input required"))?;
                let bv = right
                    .as_c_contiguous_slice()
                    .ok_or_else(|| invalid("C-contiguous right input required"))?;
                let shape = Kahen::new([m, n])?;
                if m == 0 || n == 0 || k == 0 {
                    return Ok(Tensor::<$ty>::zeros(shape)?);
                }
                let dims = [m, n, k].map(i32::try_from);
                let [Ok(mi), Ok(ni), Ok(ki)] = dims else {
                    return Err(invalid("dimensions exceed signed 32-bit BLAS limits"));
                };
                let mut output = Tensor::<$ty>::zeros(shape)?;
                #[cfg(target_os = "macos")]
                {
                    // SAFETY: Tensor validated extents and C-contiguous capabilities give
                    // exact m*k/k*n initialized input lengths and leading dimensions.
                    // Exclusive owned output of m*n elements cannot alias either input.
                    // Positive dimensions fit the C bridge ABI; the SDK supplies enum and
                    // BLAS ABI declarations. Bridge verifies single threading and restores
                    // the caller's original thread-local policy before returning.
                    let status = unsafe {
                        ffi::$kernel(
                            mi,
                            ni,
                            ki,
                            av.as_ptr(),
                            bv.as_ptr(),
                            output.storage_slice_mut().as_mut_ptr(),
                        )
                    };
                    if status != 0 {
                        return Err(unavailable());
                    }
                    Ok(output)
                }
                #[cfg(not(target_os = "macos"))]
                {
                    let _ = (mi, ni, ki, av, bv, &mut output);
                    Err(unavailable())
                }
            }
        }
    };
}
scalar!(f32, kernix_accelerate_f32);
scalar!(f64, kernix_accelerate_f64);

#[cfg(target_os = "macos")]
mod ffi {
    unsafe extern "C" {
        pub(super) fn kernix_accelerate_available() -> core::ffi::c_int;
        pub(super) fn kernix_accelerate_f32(
            m: i32,
            n: i32,
            k: i32,
            a: *const f32,
            b: *const f32,
            out: *mut f32,
        ) -> core::ffi::c_int;
        pub(super) fn kernix_accelerate_f64(
            m: i32,
            n: i32,
            k: i32,
            a: *const f64,
            b: *const f64,
            out: *mut f64,
        ) -> core::ffi::c_int;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kernix_core::Order;
    #[test]
    fn validation_and_empty_products_do_not_enter_the_sdk() {
        let cpu = AccelerateExecutor { _private: () };
        let vector = Tensor::<f64>::ones([2]).unwrap();
        assert!(cpu.matmul(&vector, &vector).is_err());
        let mismatch = Tensor::<f64>::ones([2, 3]).unwrap();
        assert!(cpu.matmul(&mismatch, &mismatch).is_err());
        let f = Tensor::from_fn(Kahen::new([2, 2]).unwrap(), Order::F, 8, |_| 1.0f64).unwrap();
        assert!(cpu.matmul(&f, &f).is_err());
        let a = Tensor::<f64>::zeros([isize::MAX as usize, 0]).unwrap();
        let b = Tensor::<f64>::zeros([0, 0]).unwrap();
        assert_eq!(cpu.matmul(&a, &b).unwrap().numel(), 0);
        #[cfg(not(target_os = "macos"))]
        {
            let a = Tensor::<f32>::ones([2, 2]).unwrap();
            assert!(cpu.matmul(&a, &a).is_err());
            let a = Tensor::<f64>::ones([2, 2]).unwrap();
            assert!(cpu.matmul(&a, &a).is_err());
        }
    }
    #[cfg(target_os = "macos")]
    #[test]
    fn sdk_restores_the_calling_threads_original_policy() {
        unsafe extern "C" {
            fn kernix_accelerate_threading() -> core::ffi::c_int;
        }
        fn check() {
            // SAFETY: pointer-free SDK query checks OS availability.
            let before = unsafe { kernix_accelerate_threading() };
            let Ok(cpu) = AccelerateExecutor::new() else {
                return;
            };
            let a = Tensor::<f64>::ones([128, 128]).unwrap();
            assert_eq!(
                cpu.matmul(&a, &a).unwrap().storage_slice(),
                &[128.; 128 * 128]
            );
            // SAFETY: same pointer-free query on the same calling thread.
            assert_eq!(unsafe { kernix_accelerate_threading() }, before);
        }
        check();
        std::thread::spawn(check).join().unwrap();
    }
}
