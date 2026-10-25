//! Bounded, allocation-free outer-product kernels for small dense products.
//!
//! Unlike the general GEMM engine these reuse rows of B directly, without packing.
//! Dispatch stays in matmul_element: large/strided/ordered products keep their engines.
use core::arch::x86_64::*;

macro_rules! kernel {
    ($ty:ident, $lanes:expr, $zero:ident, $load:ident, $broadcast:ident, $fma:ident, $store:ident) => {
        #[target_feature(enable = "avx2,fma")]
        pub(super) unsafe fn $ty(
            a: &[$ty],
            b: &[$ty],
            out: &mut [$ty],
            m: usize,
            k: usize,
            n: usize,
        ) {
            // SIMD intrinsics are unsafe on the Rust 1.86 MSRV.
            #[allow(unused_unsafe)]
            #[inline]
            #[target_feature(enable = "avx2,fma")]
            unsafe fn tile<const ROWS: usize>(
                a: &[$ty],
                b: &[$ty],
                out: &mut [$ty],
                k: usize,
                n: usize,
                first: usize,
            ) {
                let columns = n / (2 * $lanes) * (2 * $lanes);
                for j in (0..columns).step_by(2 * $lanes) {
                    let mut acc = [unsafe { $zero() }; ROWS];
                    let mut high = [unsafe { $zero() }; ROWS];
                    for p in 0..k {
                        // SAFETY: j+2*lanes<=n and p<k. All slices have the exact
                        // extents validated by dispatch; loads may be unaligned.
                        let bv = unsafe { $load(b.as_ptr().add(p * n + j)) };
                        let bh = unsafe { $load(b.as_ptr().add(p * n + j + $lanes)) };
                        for r in 0..ROWS {
                            let av = unsafe { $broadcast(a[(first + r) * k + p]) };
                            acc[r] = unsafe { $fma(av, bv, acc[r]) };
                            high[r] = unsafe { $fma(av, bh, high[r]) };
                        }
                    }
                    for r in 0..ROWS {
                        // SAFETY: first+ROWS<=m; vectors stay in this row and
                        // output is disjoint from inputs. No previous C is read.
                        unsafe {
                            $store(out.as_mut_ptr().add((first + r) * n + j), acc[r]);
                            $store(out.as_mut_ptr().add((first + r) * n + j + $lanes), high[r]);
                        };
                    }
                }
                // Dispatch accepts only complete vector column tiles. Row tails
                // use the const-generic specializations below, without overreads.
            }
            let full = m / 4 * 4;
            for first in (0..full).step_by(4) {
                // SAFETY: full tiles and the following tail partition the rows.
                unsafe { tile::<4>(a, b, out, k, n, first) };
            }
            // SAFETY: the selected specialization covers exactly the remaining
            // rows. The target features are inherited from the checked entry.
            unsafe {
                match m - full {
                    1 => tile::<1>(a, b, out, k, n, full),
                    2 => tile::<2>(a, b, out, k, n, full),
                    3 => tile::<3>(a, b, out, k, n, full),
                    _ => {}
                }
            }
        }
    };
}
kernel!(
    f64,
    4,
    _mm256_setzero_pd,
    _mm256_loadu_pd,
    _mm256_set1_pd,
    _mm256_fmadd_pd,
    _mm256_storeu_pd
);
kernel!(
    f32,
    8,
    _mm256_setzero_ps,
    _mm256_loadu_ps,
    _mm256_set1_ps,
    _mm256_fmadd_ps,
    _mm256_storeu_ps
);
