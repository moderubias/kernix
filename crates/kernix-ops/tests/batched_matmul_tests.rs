//! Native per-matrix batches retain the full reference geometry and resource checks.
use kernix_core::tensor::Tensor;
use kernix_ops::{matmul, matmul_reference};

#[test]
fn native_batches_vectors_and_broadcasts_match_ordered_reference() {
    for (a, b) in [
        (&[3][..], &[3][..]),
        (&[3], &[2, 3, 4]),
        (&[2, 4, 3], &[3]),
        (&[2, 1, 4, 3], &[1, 5, 3, 2]),
        (&[2, 1, 4, 3], &[5, 3, 2]),
        (&[4, 3], &[2, 3, 2]),
        (&[2, 4, 3], &[3, 2]),
        (&[2, 3, 1, 4, 3], &[1, 1, 5, 3, 2]),
        (&[2, 4, 0], &[0, 3]),
        (&[0], &[0]),
        (&[0, 2, 3], &[1, 3, 4]),
    ] {
        let av = Tensor::from_values(
            a,
            (0..kernix_core::shape::Kahen::new(a).unwrap().numel())
                .map(|i| i as f64 / 17.0 - 0.3)
                .collect(),
        )
        .unwrap();
        let bv = Tensor::from_values(
            b,
            (0..kernix_core::shape::Kahen::new(b).unwrap().numel())
                .map(|i| i as f64 / 13.0 - 0.2)
                .collect(),
        )
        .unwrap();
        let expected = matmul_reference(&av, &bv).unwrap();
        let actual = matmul(&av, &bv).unwrap();
        assert_eq!(actual.shape(), expected.shape());
        for (&x, &y) in actual.storage_slice().iter().zip(expected.storage_slice()) {
            assert!((x - y).abs() < 2e-13 * (1.0 + y.abs()));
        }
        let af = kernix_ops::map(&av, |x| *x as f32).unwrap();
        let bf = kernix_ops::map(&bv, |x| *x as f32).unwrap();
        let expected = matmul_reference(&af, &bf).unwrap();
        let actual = matmul(&af, &bf).unwrap();
        for (&x, &y) in actual.storage_slice().iter().zip(expected.storage_slice()) {
            assert!((x - y).abs() < 3e-6 * (1.0 + y.abs()));
        }
    }
}
#[test]
fn strided_batches_and_integer_fallback_keep_logical_values() {
    let mut a = Tensor::from_values([2, 3, 4], (0..24).map(|i| i as f64 / 17.0).collect()).unwrap();
    a.swap_axes_in_place(1, 2).unwrap();
    a.reverse_axis_in_place(2).unwrap();
    let b = Tensor::full([3, 2], 0.7).unwrap();
    assert_eq!(
        matmul(&a, &b).unwrap().storage_slice(),
        matmul_reference(&a, &b).unwrap().storage_slice()
    );
    let ai = Tensor::from_values([2, 1, 2, 3], (0..12).collect::<Vec<i32>>()).unwrap();
    let bi = Tensor::ones([3, 3, 2]).unwrap();
    assert_eq!(
        matmul(&ai, &bi).unwrap().storage_slice(),
        matmul_reference(&ai, &bi).unwrap().storage_slice()
    );
}
#[test]
fn enormous_zero_contraction_outputs_return_errors_without_capacity_panics() {
    for dims in [&[isize::MAX as usize, 0][..], &[1, isize::MAX as usize, 0]] {
        let a = Tensor::<f64>::zeros(dims).unwrap();
        let b = Tensor::<f64>::zeros([0, 2]).unwrap();
        let outcome = std::panic::catch_unwind(|| matmul(&a, &b));
        assert!(
            outcome.is_ok(),
            "fallible output validation must precede allocation"
        );
        assert!(outcome.unwrap().is_err());
    }
}
