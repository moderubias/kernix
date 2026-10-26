//! Safe scalar dispatch for optimized contiguous CPU matrix kernels.
use super::Zero;
use core::ops::{AddAssign, Mul};

/// Scalar capability for matrix multiplication. Custom scalar types can use an
/// empty implementation to retain the portable ordered fallback. f32/f64 supply
/// checked native CPU kernels; integer implementations retain native arithmetic.
///
/// ```
/// fn accepts_matrix_scalar<T:kernix_ops::MatmulElement>() {}
/// accepts_matrix_scalar::<f64>();
/// accepts_matrix_scalar::<i32>();
/// ```
pub trait MatmulElement: Zero + Clone + AddAssign<Self> + Mul<Output = Self> {
    /// Whether this scalar provides native CPU GEMM capabilities.
    const HAS_NATIVE_MATMUL: bool = false;
    /// Optionally multiply two validated signed-stride matrices. Validate output
    /// length and contraction before any kernel call. False means no values written.
    fn try_strided_matmul(
        _a: MatrixInput<'_, Self>,
        _b: MatrixInput<'_, Self>,
        _out: &mut [Self],
    ) -> bool {
        false
    }
    /// Optionally write a dense C-order product into disjoint output storage.
    /// Return false without writing if unsupported. This is a safe method: an
    /// implementation must validate all lengths/dimensions before any unsafe kernel.
    /// Returning true means all m*n output values were initialized. CPU feature
    /// dispatch, packing and fused arithmetic may change floating last bits.
    /// Built-in floating kernels may allocate packing scratch. Large native products
    /// on x86-64/std may retain a thread-local slab until thread exit. Use
    /// `matmul_reference` or an ordered `CpuExecutor` to avoid native packing.
    fn try_dense_matmul(
        _a: &[Self],
        _b: &[Self],
        _out: &mut [Self],
        _m: usize,
        _k: usize,
        _n: usize,
    ) -> bool {
        false
    }
}
macro_rules! integer {($($ty:ty),*)=>{$(impl MatmulElement for $ty {})*};}
integer!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize
);
macro_rules! floating {
    ($ty:ident,$kernel:ident,$native:path,$columns:expr) => {
        impl MatmulElement for $ty {
            const HAS_NATIVE_MATMUL: bool = true;
            fn try_strided_matmul(
                a: MatrixInput<'_, Self>,
                b: MatrixInput<'_, Self>,
                out: &mut [Self],
            ) -> bool {
                let (m, k, n) = (a.dims[0], a.dims[1], b.dims[1]);
                if k != b.dims[0] || m.checked_mul(n) != Some(out.len()) {
                    return false;
                }
                if m == 0 || n == 0 {
                    return true;
                }
                if k == 0 {
                    out.fill(0.0);
                    return true;
                }
                let Ok(ns) = isize::try_from(n) else {
                    return false;
                };
                #[cfg(all(feature = "std", target_arch = "x86_64", not(miri)))]
                if m >= 32
                    && k >= 32
                    && n >= 32
                    && m.saturating_mul(k).saturating_mul(n) >= 64 * 64 * 64
                    // Panels benefit from this packing policy on tested x86 hosts.
                    // General positive-stride views are enabled on Windows only:
                    // the broader policy regressed Linux transpose-128 latency.
                    && (cfg!(target_os = "windows") || m.min(k).min(n) == 32)
                    && a.strides.iter().all(|&s| s > 0)
                    && b.strides.iter().all(|&s| s > 0)
                    && gemm_common::simd::V3::try_new().is_some()
                {
                    // SAFETY: MatrixInput validated both complete byte spans;
                    // output has the checked exact C-order extent. Positive strides
                    // support transpose/F-order/gapped inputs without materializing.
                    // Native feature dispatch is checked; serial overwriting C^T=B^T A^T.
                    unsafe {
                        $native()(
                            n,
                            m,
                            k,
                            out.as_mut_ptr(),
                            ns,
                            1,
                            false,
                            b.data.as_ptr().add(b.origin),
                            b.strides[0],
                            b.strides[1],
                            a.data.as_ptr().add(a.origin),
                            a.strides[0],
                            a.strides[1],
                            0.0,
                            1.0,
                            false,
                            false,
                            false,
                            gemm_common::Parallelism::None,
                        );
                    }
                    return true;
                }
                // SAFETY: MatrixInput constructors validated the full reachable element
                // and byte spans, including negative/zero strides. Both nonempty origins
                // address live allocation elements. All raw kernel reads stay within
                // those spans. Rust's exclusive output borrow is disjoint from shared
                // inputs, and exact checked output length bounds nonaliasing C strides.
                unsafe {
                    matrixmultiply::$kernel(
                        m,
                        k,
                        n,
                        1.0,
                        a.data.as_ptr().add(a.origin),
                        a.strides[0],
                        a.strides[1],
                        b.data.as_ptr().add(b.origin),
                        b.strides[0],
                        b.strides[1],
                        0.0,
                        out.as_mut_ptr(),
                        ns,
                        1,
                    );
                }
                true
            }
            fn try_dense_matmul(
                a: &[Self],
                b: &[Self],
                out: &mut [Self],
                m: usize,
                k: usize,
                n: usize,
            ) -> bool {
                if m.checked_mul(k) != Some(a.len())
                    || k.checked_mul(n) != Some(b.len())
                    || m.checked_mul(n) != Some(out.len())
                {
                    return false;
                }
                if m == 0 || n == 0 {
                    return true;
                }
                if k == 0 {
                    out.fill(0.0);
                    return true;
                }
                let (Ok(ks), Ok(ns)) = (isize::try_from(k), isize::try_from(n)) else {
                    return false;
                };
                #[cfg(all(feature = "std", target_arch = "x86_64", not(miri)))]
                if (4..=32).contains(&m)
                    && (4..=64).contains(&k)
                    && ($columns..=16).contains(&n)
                    && n % $columns == 0
                    && std::arch::is_x86_feature_detected!("avx2")
                    && std::arch::is_x86_feature_detected!("fma")
                {
                    // SAFETY: exact slice lengths checked above; the bounded dense
                    // kernel writes every output once and checks vector/row tails.
                    // Both required CPU features were detected on this host.
                    unsafe { super::small_gemm::$ty(a, b, out, m, k, n) };
                    return true;
                }
                #[cfg(all(feature = "std", target_arch = "x86_64", not(miri)))]
                if m >= 64 && k >= 64 && n >= 64 && gemm_common::simd::V3::try_new().is_some() {
                    // SAFETY: exact checked slice extents and representable strides
                    // above bound every access, with disjoint C-order output. The
                    // engine's complete SIMD feature check includes feature-unified
                    // builds. Compute C^T = B^T*A^T in column-major orientation;
                    // strides are (column,row), with no materialized transpose.
                    // read_dst=false overwrites output; gemm uses alpha=0, beta=1.
                    // Explicit serial execution also prevents nested worker pools.
                    unsafe {
                        $native()(
                            n,
                            m,
                            k,
                            out.as_mut_ptr(),
                            ns,
                            1,
                            false,
                            b.as_ptr(),
                            ns,
                            1,
                            a.as_ptr(),
                            ks,
                            1,
                            0.0,
                            1.0,
                            false,
                            false,
                            false,
                            gemm_common::Parallelism::None,
                        );
                    }
                    return true;
                }
                #[cfg(all(
                    feature = "std",
                    target_arch = "aarch64",
                    not(target_os = "macos"),
                    not(miri)
                ))]
                if m >= 64 && k >= 64 && n >= 64 && std::arch::is_aarch64_feature_detected!("neon")
                {
                    // SAFETY: validated extents/strides above, disjoint output,
                    // and runtime NEON support. Explicit table avoids AMX feature unification.
                    unsafe {
                        super::neon::$ty(a, b, out, m, k, n);
                    }
                    return true;
                }
                // SAFETY: safe slices own/borrow initialized valid extents; checked exact
                // products bound every accessed element. C strides are (k,1)/(n,1),
                // representable as isize. The exclusive output slice cannot alias either
                // shared input. Nonzero m,n,k imply all pointers address live elements.
                // beta=0 overwrites a non-overlapping C-order output. Backend runtime
                // feature detection selects only instructions available on this CPU.
                unsafe {
                    matrixmultiply::$kernel(
                        m,
                        k,
                        n,
                        1.0,
                        a.as_ptr(),
                        ks,
                        1,
                        b.as_ptr(),
                        ns,
                        1,
                        0.0,
                        out.as_mut_ptr(),
                        ns,
                        1,
                    );
                }
                true
            }
        }
    };
}
floating!(f32, sgemm, gemm_f32::gemm::f32::get_gemm_fn, 16);
floating!(f64, dgemm, gemm_f64::gemm::f64::get_gemm_fn, 8);

/// Validated borrowed rank-two host matrix, including signed/broadcast strides.
/// Storage and metadata remain independent of devices. This capability allows
/// optimized CPU kernels to read transpose/reverse views without materializing them.
#[derive(Debug)]
pub struct MatrixInput<'a, T> {
    data: &'a [T],
    dims: [usize; 2],
    strides: [isize; 2],
    origin: usize,
}
impl<'a, T> MatrixInput<'a, T> {
    /// Validate dimensions, element/byte span and signed strides against host storage.
    /// Singleton strides are normalized to zero because they never affect indexing.
    ///
    /// ```
    /// let matrix=kernix_ops::MatrixInput::new(&[1.0f64,2.0,3.0,4.0],[2,2],[2,1],0)?;
    /// assert_eq!(matrix.dims(),[2,2]);
    /// # Ok::<(),kernix_ops::OpsError>(())
    /// ```
    pub fn new(
        data: &'a [T],
        dims: [usize; 2],
        mut strides: [isize; 2],
        origin: usize,
    ) -> Result<Self, super::OpsError> {
        let shape = kernix_core::shape::Kahen::new(dims)?;
        let layout = kernix_core::Layout::from_strides(shape, strides, origin)
            .map_err(kernix_core::tensor::TensorError::from)?;
        layout
            .validate_cpu_storage::<T>(data.len())
            .map_err(kernix_core::tensor::TensorError::from)?;
        for axis in 0..2 {
            if dims[axis] <= 1 {
                strides[axis] = 0;
            }
        }
        Ok(Self {
            data,
            dims,
            strides,
            origin,
        })
    }
    /// Row/column counts of the logical matrix.
    pub fn dims(&self) -> [usize; 2] {
        self.dims
    }
    /// Signed element strides, with irrelevant singleton strides normalized to zero.
    pub fn strides(&self) -> [isize; 2] {
        self.strides
    }
    /// Physical backing slice, which can include gaps outside logical elements.
    pub fn storage(&self) -> &'a [T] {
        self.data
    }
    /// Physical element offset for the logical origin.
    pub fn origin(&self) -> usize {
        self.origin
    }
}
