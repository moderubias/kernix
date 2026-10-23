//! Architecture-specific GEMM entry points with a fixed NEON kernel selection.
use gemm_common::{Parallelism, simd::Scalar};

/// Caller validates exact extents, strides, nonaliasing and runtime NEON support.
pub(super) unsafe fn f32(a: &[f32], b: &[f32], out: &mut [f32], m: usize, k: usize, n: usize) {
    use gemm_f32::microkernel::neon::f32::{H_M, H_N, H_UKR, MR_DIV_N, NR, UKR};
    // SAFETY: caller supplies initialized validated dense matrices with exclusive
    // output and representable strides; only the runtime-guarded NEON table is used.
    unsafe {
        gemm_common::gemm::gemm_basic_generic::<
            Scalar,
            f32,
            4,
            { MR_DIV_N * 4 },
            NR,
            MR_DIV_N,
            H_M,
            H_N,
        >(
            Scalar,
            n,
            m,
            k,
            out.as_mut_ptr(),
            n as isize,
            1,
            false,
            b.as_ptr(),
            n as isize,
            1,
            a.as_ptr(),
            k as isize,
            1,
            0.0,
            1.0,
            false,
            false,
            false,
            |a, b, c| a * b + c,
            &UKR,
            &H_UKR,
            false,
            Parallelism::None,
        );
    }
}

/// Caller validates exact extents, strides, nonaliasing and runtime NEON support.
pub(super) unsafe fn f64(a: &[f64], b: &[f64], out: &mut [f64], m: usize, k: usize, n: usize) {
    use gemm_f64::microkernel::neon::f64::{H_M, H_N, H_UKR, MR_DIV_N, NR, UKR};
    // SAFETY: caller supplies initialized validated dense matrices with exclusive
    // output and representable strides; only the runtime-guarded NEON table is used.
    unsafe {
        gemm_common::gemm::gemm_basic_generic::<
            Scalar,
            f64,
            2,
            { MR_DIV_N * 2 },
            NR,
            MR_DIV_N,
            H_M,
            H_N,
        >(
            Scalar,
            n,
            m,
            k,
            out.as_mut_ptr(),
            n as isize,
            1,
            false,
            b.as_ptr(),
            n as isize,
            1,
            a.as_ptr(),
            k as isize,
            1,
            0.0,
            1.0,
            false,
            false,
            false,
            |a, b, c| a * b + c,
            &UKR,
            &H_UKR,
            false,
            Parallelism::None,
        );
    }
}
