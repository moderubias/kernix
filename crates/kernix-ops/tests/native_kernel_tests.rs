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
