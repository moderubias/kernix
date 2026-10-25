//! Independent coordinate oracles at both sides of the small-kernel boundaries.
use kernix_core::tensor::Tensor;
use kernix_ops::{MatmulElement, matmul, matmul_reference};

#[cfg(all(feature = "std", target_arch = "x86_64", not(miri)))]
mod allocations {
    use super::*;
    use std::alloc::{GlobalAlloc, Layout, System};
    use std::cell::Cell;
    std::thread_local! {
        static COUNT: Cell<Option<usize>> = const { Cell::new(None) };
    }
    struct Counting;
    // SAFETY: all allocation/deallocation requests delegate unchanged to System.
    // The thread-local counter never allocates and does not inspect allocation data.
    unsafe impl GlobalAlloc for Counting {
        unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
            let _ = COUNT.try_with(|c| c.set(c.get().map(|n| n + 1)));
            // SAFETY: delegate the caller's valid layout to the system allocator.
            unsafe { System.alloc(layout) }
        }
        unsafe fn dealloc(&self, p: *mut u8, layout: Layout) {
            // SAFETY: delegate the original allocation pointer and layout.
            unsafe { System.dealloc(p, layout) }
        }
        unsafe fn realloc(&self, p: *mut u8, layout: Layout, size: usize) -> *mut u8 {
            let _ = COUNT.try_with(|c| c.set(c.get().map(|n| n + 1)));
            // SAFETY: delegate the caller's valid reallocation request.
            unsafe { System.realloc(p, layout, size) }
        }
    }
    #[global_allocator]
    static ALLOCATOR: Counting = Counting;

    #[test]
    fn small_native_hooks_need_no_heap_scratch_even_on_first_use() {
        if !std::arch::is_x86_feature_detected!("avx2")
            || !std::arch::is_x86_feature_detected!("fma")
        {
            return;
        }
        let a = [0.5; 16 * 32];
        let b = [0.25; 32 * 16];
        let mut out64 = [f64::NAN; 16 * 16];
        let mut out32 = [f32::NAN; 16 * 16];
        let af = a.map(|v| v as f32);
        let bf = b.map(|v| v as f32);
        COUNT.with(|c| c.set(Some(0)));
        let ok64 = f64::try_dense_matmul(&a, &b, &mut out64, 16, 32, 16);
        let ok32 = f32::try_dense_matmul(&af, &bf, &mut out32, 16, 32, 16);
        let count = COUNT.with(|c| c.replace(None));
        assert!(ok64 && ok32);
        assert_eq!(count, Some(0));
        assert_eq!(out64, [4.0; 16 * 16]);
        assert_eq!(out32, [4.0; 16 * 16]);
    }
}

macro_rules! boundaries {
    ($name:ident, $ty:ty) => {
        #[test]
        fn $name() {
            for m in [0, 1, 3, 4, 5, 7, 8, 15, 16, 17, 31, 32, 33] {
                for k in [0, 1, 3, 4, 5, 31, 32, 63, 64, 65] {
                    for n in [0, 1, 7, 8, 9, 15, 16, 17, 24, 31, 32, 33] {
                        // Offset slices exercise unaligned loads. Dyadic data
                        // makes the coordinate oracle exact in both dtypes.
                        let av: Vec<$ty> = (0..m * k + 1)
                            .map(|p| ((p * 7 % 29) as $ty - 14.0) / 8.0)
                            .collect();
                        let bv: Vec<$ty> = (0..k * n + 1)
                            .map(|p| ((p * 11 % 37) as $ty - 18.0) / 16.0)
                            .collect();
                        let (a, b) = (&av[1..], &bv[1..]);
                        let mut out = vec![<$ty>::NAN; m * n + 2];
                        for _ in 0..2 {
                            assert!(<$ty>::try_dense_matmul(
                                a,
                                b,
                                &mut out[1..1 + m * n],
                                m,
                                k,
                                n
                            ));
                            for i in 0..m {
                                for j in 0..n {
                                    let expected: f64 = (0..k)
                                        .map(|p| f64::from(a[i * k + p]) * f64::from(b[p * n + j]))
                                        .sum();
                                    assert_eq!(
                                        f64::from(out[1 + i * n + j]),
                                        expected,
                                        "{m}/{k}/{n} at {i}/{j}"
                                    );
                                }
                            }
                            assert!(out[0].is_nan() && out[m * n + 1].is_nan());
                        }
                    }
                }
            }
        }
    };
}
boundaries!(f64_coordinate_boundaries_and_unaligned_reused_output, f64);
boundaries!(f32_coordinate_boundaries_and_unaligned_reused_output, f32);

#[test]
fn small_broadcast_batches_and_ordered_execution_remain_consistent() {
    for batch in [1, 4, 16, 64] {
        let a = Tensor::from_values(
            [batch, 17, 32],
            (0..batch * 17 * 32)
                .map(|p| (p % 13) as f64 / 8.0)
                .collect(),
        )
        .unwrap();
        let b = Tensor::from_values(
            [1, 32, 16],
            (0..32 * 16).map(|p| (p % 17) as f64 / 16.0).collect(),
        )
        .unwrap();
        let expected = matmul_reference(&a, &b).unwrap();
        assert_eq!(
            matmul(&a, &b).unwrap().storage_slice(),
            expected.storage_slice()
        );
        let ordered = kernix_ops::execution::CpuExecutor::default().with_ordered_matmul(true);
        assert_eq!(
            ordered.matmul(&a, &b).unwrap().storage_slice(),
            expected.storage_slice()
        );
    }
}

#[test]
fn malformed_extents_fail_before_writing_in_both_dtypes() {
    macro_rules! check {
        ($ty:ty) => {{
            for (m, k, n) in [(4, 4, 16), (usize::MAX, 2, 16), (4, usize::MAX, 16)] {
                let mut out = [7.0 as $ty; 64];
                assert!(!<$ty>::try_dense_matmul(
                    &[1.0; 16], &[2.0; 63], &mut out, m, k, n
                ));
                assert_eq!(out, [7.0; 64]);
            }
        }};
    }
    check!(f32);
    check!(f64);
}

#[test]
fn small_non_dyadic_products_respect_scaled_error_bounds() {
    macro_rules! check {
        ($ty:ty, $scales:expr, $budget:expr) => {
            for scale in $scales {
                let (m, k, n) = (31, 63, 32);
                let a: Vec<$ty> = (0..m * k)
                    .map(|p| ((p * 7 % 37) as $ty - 18.0) / 17.0 * scale)
                    .collect();
                let b: Vec<$ty> = (0..k * n)
                    .map(|p| ((p * 11 % 29) as $ty - 14.0) / 13.0 * scale)
                    .collect();
                let mut out = vec![<$ty>::NAN; m * n];
                assert!(<$ty>::try_dense_matmul(&a, &b, &mut out, m, k, n));
                for i in 0..m {
                    for j in 0..n {
                        let products =
                            (0..k).map(|p| f64::from(a[i * k + p]) * f64::from(b[p * n + j]));
                        let norm: f64 = products.clone().map(f64::abs).sum();
                        let expected: f64 = products.sum();
                        let actual = f64::from(out[i * n + j]);
                        assert!(actual.is_finite());
                        assert!(
                            (actual - expected).abs() <= $budget * norm,
                            "scale {scale}, {i}/{j}"
                        );
                    }
                }
            }
        };
    }
    check!(f64, [1e-100f64, 1.0, 1e100], 1e-13);
    check!(f32, [1e-10f32, 1.0, 1e10], 2e-6);
}
