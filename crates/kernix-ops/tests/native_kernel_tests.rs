//! Native/ordered GEMM safety and error-bound contracts, plus stable reductions.
use kernix_core::{Order, shape::Kahen, tensor::Tensor};
use kernix_ops::{
    MatmulElement, MatrixInput, matmul, matmul_reference, sum_all, sum_compensated_all,
    sum_sequential_all,
};
fn tensor(m: usize, n: usize) -> Tensor<f64> {
    Tensor::from_vec(
        (0..m * n)
            .map(|i| ((i * 7 % 37) as f64 - 18.0) / 17.0)
            .collect(),
        Kahen::new([m, n]).unwrap(),
        Order::C,
    )
    .unwrap()
}
#[test]
fn safe_native_hooks_reject_invalid_lengths_without_writes() {
    let mut out = [7.0f64; 4];
    assert!(!f64::try_dense_matmul(&[1.0], &[2.0], &mut out, 2, 2, 2));
    assert_eq!(out, [7.0; 4]);
    assert!(MatrixInput::<f64>::new(&[1.0], [2, 2], [2, 1], 0).is_err());
    let a = MatrixInput::new(&[1.0f64], [1, 1], [isize::MIN, isize::MIN], 0).unwrap();
    let b = MatrixInput::new(&[2.0f64], [1, 1], [0, 0], 0).unwrap();
    let mut out = [0.0];
    assert!(f64::try_strided_matmul(a, b, &mut out));
    assert_eq!(out, [2.0]);
}
#[test]
fn native_matches_reference_error_bounds_for_tails_transposes_reversals_and_zeros() {
    for (m, k, n) in [(17, 31, 19), (1, 1, 1), (4, 0, 3), (0, 7, 3), (3, 7, 0)] {
        let a = tensor(m, k);
        let b = tensor(k, n);
        let actual = matmul(&a, &b).unwrap();
        let expected = matmul_reference(&a, &b).unwrap();
        for (&x, &y) in actual.storage_slice().iter().zip(expected.storage_slice()) {
            assert!((x - y).abs() < 1e-13 * (1.0 + y.abs()));
        }
    }
    let a = tensor(9, 7);
    let b = tensor(9, 5);
    let transposed = a.transpose().unwrap();
    let reversed = b.reverse_axis(0).unwrap();
    let actual = matmul(&transposed, &reversed).unwrap();
    let expected = matmul_reference(&transposed, &reversed).unwrap();
    for (&x, &y) in actual.storage_slice().iter().zip(expected.storage_slice()) {
        assert!((x - y).abs() < 1e-13 * (1.0 + y.abs()));
    }
}
#[test]
fn ordered_mode_is_explicit_and_compensated_sum_recovers_cancellation() {
    let cpu = kernix_ops::execution::CpuExecutor::default().with_ordered_matmul(true);
    assert!(cpu.ordered_matmul());
    let a = tensor(7, 11);
    let b = tensor(11, 5);
    assert_eq!(
        cpu.matmul(&a, &b).unwrap().storage_slice(),
        matmul_reference(&a, &b).unwrap().storage_slice()
    );
    let a = Tensor::from_vec(
        vec![1e16f64, 1.0, -1e16],
        Kahen::new([3]).unwrap(),
        Order::C,
    )
    .unwrap();
    assert_eq!(sum_sequential_all(&a).unwrap(), 0.0);
    assert_eq!(sum_compensated_all(&a).unwrap(), 1.0);
    assert!(sum_all(&a).unwrap().is_finite());
}

#[test]
fn dense_f64_backend_boundaries_rectangles_and_reused_output() {
    for (m, k, n) in [
        (63, 64, 64),
        (64, 63, 64),
        (64, 64, 63),
        (64, 64, 64),
        (65, 67, 69),
        (96, 65, 80),
        (128, 128, 128),
        (129, 127, 131),
    ] {
        let a = tensor(m, k);
        let b = tensor(k, n);
        let expected = matmul_reference(&a, &b).unwrap();
        let actual = matmul(&a, &b).unwrap();
        let mut reused = Tensor::full([m, n], f64::NAN).unwrap();
        let pointer = reused.storage_slice().as_ptr();
        for _ in 0..2 {
            kernix_ops::execution::CpuExecutor::default()
                .matmul_into(&a, &b, &mut reused)
                .unwrap();
            assert_eq!(pointer, reused.storage_slice().as_ptr());
            for ((&x, &y), &z) in actual
                .storage_slice()
                .iter()
                .zip(expected.storage_slice())
                .zip(reused.storage_slice())
            {
                assert!(
                    (x - y).abs() < 1e-12 * (1.0 + y.abs()),
                    "{m}/{k}/{n}: {x} vs {y}"
                );
                assert_eq!(x, z);
            }
        }
    }
}

#[test]
fn dense_f64_native_dispatch_keeps_batched_and_ordered_contracts() {
    let a = Tensor::from_values(
        [2, 64, 65],
        (0..2 * 64 * 65).map(|i| (i % 7) as f64 - 3.0).collect(),
    )
    .unwrap();
    let b = Tensor::from_values(
        [1, 65, 67],
        (0..65 * 67).map(|i| (i % 11) as f64 - 5.0).collect(),
    )
    .unwrap();
    assert_eq!(
        matmul(&a, &b).unwrap().storage_slice(),
        matmul_reference(&a, &b).unwrap().storage_slice()
    );
    let a = tensor(65, 67);
    let b = tensor(67, 69);
    let expected = matmul_reference(&a, &b).unwrap();
    let ordered = kernix_ops::execution::CpuExecutor::default().with_ordered_matmul(true);
    assert_eq!(
        ordered.matmul(&a, &b).unwrap().storage_slice(),
        expected.storage_slice()
    );
    let mut out = Tensor::zeros([65, 69]).unwrap();
    ordered.matmul_into(&a, &b, &mut out).unwrap();
    assert_eq!(out.storage_slice(), expected.storage_slice());
}

#[test]
fn dense_f32_native_boundaries_rectangles_and_reuse_match_f64_oracle() {
    for (m, k, n) in [
        (63, 64, 65),
        (64, 63, 65),
        (65, 64, 63),
        (64, 64, 64),
        (65, 67, 69),
        (128, 128, 128),
        (129, 127, 131),
    ] {
        let av: Vec<f32> = (0..m * k)
            .map(|i| ((i * 7 % 37) as f32 - 18.0) / 17.0)
            .collect();
        let bv: Vec<f32> = (0..k * n)
            .map(|i| ((i * 11 % 29) as f32 - 14.0) / 13.0)
            .collect();
        let a = Tensor::from_values([m, k], av.clone()).unwrap();
        let b = Tensor::from_values([k, n], bv.clone()).unwrap();
        let actual = matmul(&a, &b).unwrap();
        let mut reused = Tensor::full([m, n], f32::NAN).unwrap();
        let ptr = reused.storage_slice().as_ptr();
        for _ in 0..2 {
            kernix_ops::execution::CpuExecutor::default()
                .matmul_into(&a, &b, &mut reused)
                .unwrap();
            assert_eq!(ptr, reused.storage_slice().as_ptr());
            assert_eq!(actual.storage_slice(), reused.storage_slice());
        }
        for i in 0..m {
            for j in 0..n {
                let expected: f64 = (0..k)
                    .map(|p| f64::from(av[i * k + p]) * f64::from(bv[p * n + j]))
                    .sum();
                assert!(
                    (f64::from(actual.storage_slice()[i * n + j]) - expected).abs()
                        < 1e-5 * (1.0 + expected.abs()),
                    "{m}/{k}/{n}, {i}/{j}"
                );
            }
        }
    }
}

#[test]
fn dense_f32_native_broadcast_batches_keep_exact_integer_products() {
    let a = Tensor::from_values(
        [2, 64, 65],
        (0..2 * 64 * 65).map(|i| (i % 7) as f32 - 3.0).collect(),
    )
    .unwrap();
    let b = Tensor::from_values(
        [1, 65, 67],
        (0..65 * 67).map(|i| (i % 11) as f32 - 5.0).collect(),
    )
    .unwrap();
    assert_eq!(
        matmul(&a, &b).unwrap().storage_slice(),
        matmul_reference(&a, &b).unwrap().storage_slice()
    );
}

#[test]
fn tiled_convolution_matches_ordered_reference_across_tiles_and_geometry() {
    use kernix_ops::neural::{Conv2dOptions, conv2d, conv2d_reference};
    for options in [
        Conv2dOptions::default(),
        Conv2dOptions {
            stride: [2, 1],
            padding: [2, 1],
            dilation: [2, 1],
            groups: 2,
        },
    ] {
        let channels = 4;
        let input = Tensor::from_vec(
            (0..2 * channels * 17 * 19)
                .map(|i| ((i * 7 % 31) as f64 - 15.0) / 31.0)
                .collect(),
            Kahen::new([2, channels, 17, 19]).unwrap(),
            Order::C,
        )
        .unwrap();
        let weight = Tensor::from_vec(
            (0..6 * (channels / options.groups) * 3 * 2)
                .map(|i| ((i * 11 % 29) as f64 - 14.0) / 29.0)
                .collect(),
            Kahen::new([6, channels / options.groups, 3, 2]).unwrap(),
            Order::C,
        )
        .unwrap();
        let actual = conv2d(&input, &weight, options).unwrap();
        let expected = conv2d_reference(&input, &weight, options).unwrap();
        assert_eq!(actual.shape(), expected.shape());
        for (&x, &y) in actual.storage_slice().iter().zip(expected.storage_slice()) {
            assert!((x - y).abs() < 1e-13 * (1.0 + y.abs()), "{x} vs {y}");
        }
        let input = input.reverse_axis(3).unwrap();
        let actual = conv2d(&input, &weight, options).unwrap();
        let expected = conv2d_reference(&input, &weight, options).unwrap();
        assert_eq!(actual.storage_slice(), expected.storage_slice());
    }
}

#[test]
fn large_positive_gapped_strides_match_coordinate_oracles_in_both_dtypes() {
    macro_rules! check {
        ($ty:ty) => {{
            for (m, k, n) in [
                (63usize, 65usize, 67usize),
                (64, 64, 64),
                (65, 67, 69),
                (128, 128, 128),
            ] {
                let ar = 2 * k + 5;
                let bc = 3 * k + 7;
                let mut a = vec![<$ty>::NAN; 7 + (m - 1) * ar + 2 * (k - 1) + 1];
                let mut b = vec![<$ty>::NAN; 3 + 3 * (k - 1) + (n - 1) * bc + 1];
                for i in 0..m {
                    for p in 0..k {
                        a[7 + i * ar + 2 * p] = ((i * 7 + p * 3) % 23) as $ty / 16. - 0.5;
                    }
                }
                for p in 0..k {
                    for j in 0..n {
                        b[3 + 3 * p + j * bc] = ((p * 5 + j * 11) % 13) as $ty / 8. - 0.5;
                    }
                }
                let mut out = vec![<$ty>::NAN; m * n + 2];
                let aa = MatrixInput::new(&a, [m, k], [ar as isize, 2], 7).unwrap();
                let bb = MatrixInput::new(&b, [k, n], [3, bc as isize], 3).unwrap();
                assert!(<$ty>::try_strided_matmul(aa, bb, &mut out[1..1 + m * n]));
                for i in 0..m {
                    for j in 0..n {
                        let expected: f64 = (0..k)
                            .map(|p| {
                                f64::from(a[7 + i * ar + 2 * p]) * f64::from(b[3 + 3 * p + j * bc])
                            })
                            .sum();
                        assert_eq!(
                            f64::from(out[1 + i * n + j]),
                            expected,
                            "{m}/{k}/{n} {i}/{j}"
                        );
                    }
                }
                assert!(out[0].is_nan() && out[m * n + 1].is_nan());
            }
        }};
    }
    check!(f64);
    check!(f32);
}
