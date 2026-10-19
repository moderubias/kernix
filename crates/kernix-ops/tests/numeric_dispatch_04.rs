//! Regression cases for numerics update 04 and the optional contiguous path.
use kernix_core::{
    layout::Order,
    shape::Kahen,
    tensor::{Tensor, TensorError},
};
use kernix_ops::{
    TensorInput, atan2, cos, dot, expm1, hypot, log1p, logaddexp, logsigmoid, matmul, mean_all,
    mean_axes, mish, rsqrt, sin, softplus,
};

fn tensor<T>(dims: &[usize], values: Vec<T>) -> Tensor<T> {
    Tensor::from_vec(values, Kahen::new(dims).unwrap(), Order::C).unwrap()
}

fn close(a: f64, b: f64, tol: f64) {
    assert!(
        (a - b).abs() <= tol,
        "got {a:?}, expected {b:?} within {tol}"
    );
}

#[test]
fn softplus_retains_small_exponent_in_f32() {
    let x = tensor(&[1], vec![-20.0f32]);
    let result = softplus(&x).unwrap();
    let value = *result.get(&[0]).unwrap() as f64;
    assert!(value > 0.0, "softplus(-20) was rounded to zero");
    close(value, 2.061153620314381e-9, 1e-15);
}

#[test]
fn mean_all_and_axes_avoid_intermediate_overflow() {
    let x = tensor(&[2, 2], vec![3.0e38f32; 4]);
    let average = mean_all(&x).unwrap();
    assert!(average.is_finite());
    close(average as f64 / 3.0e38, 1.0, 1e-6);
    let by_axis = mean_axes(&x, &[1], false).unwrap();
    assert_eq!(by_axis.shape().dims(), &[2]);
    for i in 0..2 {
        let avg = *by_axis.get(&[i]).unwrap();
        assert!(avg.is_finite());
        close(avg as f64 / 3.0e38, 1.0, 1e-6);
    }
}

#[test]
fn cancellation_sensitive_special_functions() {
    let x = tensor(&[2], vec![1.0e-12f64, -1.0e-12]);
    let ln = log1p(&x).unwrap();
    let ex = expm1(&x).unwrap();
    close(*ln.get(&[0]).unwrap(), 1e-12 - 0.5e-24, 1e-25);
    close(*ex.get(&[1]).unwrap(), -1e-12 + 0.5e-24, 1e-25);
}

#[test]
fn logaddexp_broadcast_infinities_and_nan() {
    let a = tensor(&[2, 1], vec![f64::NEG_INFINITY, 0.0]);
    let b = tensor(&[1, 3], vec![f64::NEG_INFINITY, 0.0, f64::INFINITY]);
    let z = logaddexp(&a, &b).unwrap();
    assert_eq!(z.shape().dims(), &[2, 3]);
    assert_eq!(*z.get(&[0, 0]).unwrap(), f64::NEG_INFINITY);
    assert_eq!(*z.get(&[0, 1]).unwrap(), 0.0);
    assert_eq!(*z.get(&[0, 2]).unwrap(), f64::INFINITY);
    close(*z.get(&[1, 1]).unwrap(), core::f64::consts::LN_2, 1e-15);
    let nan = tensor(&[], vec![f64::NAN]);
    assert!(logaddexp(&nan, &nan).unwrap().get(&[]).unwrap().is_nan());
}

#[test]
fn extra_activations_including_infinite_limits() {
    let x = tensor(&[3], vec![f64::NEG_INFINITY, -20.0, 0.0]);
    let y = logsigmoid(&x).unwrap();
    assert_eq!(*y.get(&[0]).unwrap(), f64::NEG_INFINITY);
    close(*y.get(&[2]).unwrap(), -core::f64::consts::LN_2, 1e-15);
    let z = mish(&x).unwrap();
    assert_eq!(*z.get(&[0]).unwrap(), 0.0);
    assert!(z.get(&[1]).unwrap().is_finite());
    let r = rsqrt(&tensor(&[1], vec![4.0f64])).unwrap();
    close(*r.get(&[0]).unwrap(), 0.5, 1e-15);
}

#[test]
fn trigonometry_hypot_and_atan2_broadcast() {
    let y = tensor(&[2, 1], vec![0.0f64, 3.0]);
    let x = tensor(&[1, 2], vec![4.0f64, -4.0]);
    let h = hypot(&y, &x).unwrap();
    assert_eq!(h.shape().dims(), &[2, 2]);
    close(*h.get(&[1, 0]).unwrap(), 5.0, 1e-14);
    let angle = atan2(&y, &x).unwrap();
    close(*angle.get(&[1, 0]).unwrap(), (3.0f64).atan2(4.0), 1e-14);
    let angles = tensor(&[2], vec![0.0f64, core::f64::consts::FRAC_PI_2]);
    close(*sin(&angles).unwrap().get(&[1]).unwrap(), 1.0, 1e-15);
    close(*cos(&angles).unwrap().get(&[0]).unwrap(), 1.0, 1e-15);
}

// The optional fast path is intentionally tested separately from the Tensor
// implementation: calling get() is a failure, so a passing test shows direct
// C-contiguous access was actually selected.
struct ContiguousOnly<'a> {
    dims: &'a [usize],
    values: &'a [f64],
}
impl TensorInput<f64> for ContiguousOnly<'_> {
    fn dims(&self) -> &[usize] {
        self.dims
    }
    fn get(&self, _indices: &[usize]) -> Result<&f64, TensorError> {
        panic!("contiguous operation unexpectedly used indexed fallback")
    }
    fn as_c_contiguous_slice(&self) -> Option<&[f64]> {
        Some(self.values)
    }
}

#[test]
fn contiguous_matrix_mult_uses_direct_memory() {
    let a = ContiguousOnly {
        dims: &[2, 3],
        values: &[1., 2., 3., 4., 5., 6.],
    };
    let b = ContiguousOnly {
        dims: &[3, 2],
        values: &[7., 8., 9., 10., 11., 12.],
    };
    let product = matmul(&a, &b).unwrap();
    assert_eq!(product.shape().dims(), &[2, 2]);
    assert_eq!(product.storage_slice(), &[58.0, 64.0, 139.0, 154.0]);
}

#[test]
fn contiguous_dot_uses_direct_memory() {
    let a = ContiguousOnly {
        dims: &[3],
        values: &[1., 2., 3.],
    };
    let b = ContiguousOnly {
        dims: &[3],
        values: &[4., 5., 6.],
    };
    assert_eq!(dot(&a, &b).unwrap(), 32.0);
}

#[test]
fn noncontiguous_matrix_falls_back_to_checked_indexing() {
    let a = tensor(&[2, 3], vec![1., 2., 3., 4., 5., 6.]);
    let b = tensor(&[3, 2], vec![7., 8., 9., 10., 11., 12.]);
    let reversed = a.reverse_axis(1).unwrap();
    let out = matmul(&reversed, &b).unwrap();
    assert_eq!(out.storage_slice(), &[50.0, 56.0, 131.0, 146.0]);
}

#[test]
fn masked_softmax_broadcasts_mask_and_preserves_probabilities() {
    use kernix_ops::{masked_log_softmax, masked_softmax};
    let x = tensor(
        &[2, 3],
        vec![1000.0f64, 1001.0, f64::NAN, -1000.0, 1.0, f64::INFINITY],
    );
    let mask = tensor(&[1, 3], vec![true, true, false]);
    let result = masked_softmax(&x, &mask, -1).unwrap();
    let expected = 1.0 / (1.0 + core::f64::consts::E);
    close(*result.get(&[0, 0]).unwrap(), expected, 1e-15);
    close(*result.get(&[0, 1]).unwrap(), 1.0 - expected, 1e-15);
    assert_eq!(*result.get(&[0, 2]).unwrap(), 0.0);
    assert_eq!(*result.get(&[1, 2]).unwrap(), 0.0);
    let log = masked_log_softmax(&x, &mask, -1).unwrap();
    assert_eq!(*log.get(&[0, 2]).unwrap(), f64::NEG_INFINITY);
    close(log.get(&[0, 0]).unwrap().exp(), expected, 1e-15);
}

#[test]
fn softmax_temperature_preserves_extreme_logits() {
    use kernix_ops::{log_softmax_temperature, softmax_temperature};
    let x = tensor(&[1, 2], vec![1000.0f64, 1001.0]);
    let at_two = softmax_temperature(&x, -1, 2.0).unwrap();
    let probability = 1.0 / (1.0 + (-0.5f64).exp());
    close(*at_two.get(&[0, 1]).unwrap(), probability, 1e-15);
    let log = log_softmax_temperature(&x, 1, 2.0).unwrap();
    close(log.get(&[0, 1]).unwrap().exp(), probability, 1e-15);
    assert!(softmax_temperature(&x, 1, 0.0).is_err());
    assert!(softmax_temperature(&x, 1, -1.0).is_err());
    assert!(softmax_temperature(&x, 1, f64::NAN).is_err());
    assert!(softmax_temperature(&x, 1, f64::INFINITY).is_err());
    // Divide-before-subtract would overflow in this case; subtract first.
    let huge = tensor(&[2], vec![2.0e38f32, 2.0e38]);
    let y = softmax_temperature(&huge, -1, 0.001f32).unwrap();
    assert_eq!(y.storage_slice(), &[0.5, 0.5]);
}

#[test]
fn fully_masked_rows_error_except_vacuous_outputs() {
    use kernix_ops::{OpsError, masked_softmax, masked_softmax_temperature};
    let x = tensor(&[1, 2], vec![1.0f64, 2.0]);
    let mask = tensor(&[2], vec![false, false]);
    assert!(matches!(
        masked_softmax(&x, &mask, -1),
        Err(OpsError::InvalidParameter { .. })
    ));
    assert!(matches!(
        masked_softmax_temperature(&x, &mask, -1, 0.1),
        Err(OpsError::InvalidParameter { .. })
    ));
    let empty: Tensor<f64> = tensor(&[0, 2], vec![]);
    let result = masked_softmax(&empty, &mask, -1).unwrap();
    assert_eq!(result.shape().dims(), &[0, 2]);
    assert_eq!(result.numel(), 0);
}

#[test]
fn online_logsumexp_matches_reference_and_observes_empty_behavior() {
    use kernix_ops::{logsumexp_all, logsumexp_axis, logsumexp_online_all, logsumexp_online_axis};
    let x = tensor(&[3, 2], vec![1000.0f64, -1000.0, 1001.0, 0.0, 999.0, 800.0]);
    close(
        logsumexp_online_all(&x).unwrap(),
        logsumexp_all(&x).unwrap(),
        1e-12,
    );
    let online = logsumexp_online_axis(&x, 0, true).unwrap();
    let reference = logsumexp_axis(&x, 0, true).unwrap();
    assert_eq!(online.shape().dims(), &[1, 2]);
    for col in 0..2 {
        close(
            *online.get(&[0, col]).unwrap(),
            *reference.get(&[0, col]).unwrap(),
            1e-12,
        );
    }
    let zero: Tensor<f64> = tensor(&[0, 2], vec![]);
    assert!(logsumexp_online_all(&zero).is_err());
    assert!(logsumexp_online_axis(&zero, 0, false).is_err());
    let empty = logsumexp_online_axis(&zero, 1, true).unwrap();
    assert_eq!(empty.shape().dims(), &[0, 1]);
}

#[test]
fn logaddexp_large_equal_logits_is_stable() {
    let a = tensor(&[1], vec![-1000.0f64]);
    let sum = logaddexp(&a, &a).unwrap();
    close(
        *sum.get(&[0]).unwrap(),
        -1000.0 + core::f64::consts::LN_2,
        1e-13,
    );
}

#[test]
fn online_softmax_matches_reference_and_preserves_mass() {
    use kernix_ops::{
        log_softmax, log_softmax_online, softmax, softmax_online, softmax_online_temperature,
    };
    let input = tensor(
        &[2, 3],
        vec![1000.0f64, 1001.0, 999.0, -1200.0, -1201.0, -1199.0],
    );
    let online = softmax_online(&input, -1).unwrap();
    let ordinary = softmax(&input, -1).unwrap();
    for row in 0..2 {
        let mut total = 0.0;
        for col in 0..3 {
            let got = *online.get(&[row, col]).unwrap();
            let expected = *ordinary.get(&[row, col]).unwrap();
            close(got, expected, 1e-15);
            total += got;
        }
        close(total, 1.0, 1e-14);
    }
    let online_log = log_softmax_online(&input, 1).unwrap();
    let reference_log = log_softmax(&input, 1).unwrap();
    for row in 0..2 {
        for col in 0..3 {
            close(
                *online_log.get(&[row, col]).unwrap(),
                *reference_log.get(&[row, col]).unwrap(),
                1e-14,
            );
        }
    }
    let scaled = softmax_online_temperature(&input, -1, 0.5).unwrap();
    close(
        *scaled.get(&[0, 1]).unwrap()
            + *scaled.get(&[0, 0]).unwrap()
            + *scaled.get(&[0, 2]).unwrap(),
        1.0,
        1e-14,
    );
}

#[test]
fn log1mexp_and_logdiffexp_avoid_cancellation() {
    use kernix_ops::{log1mexp, logdiffexp};
    let x = tensor(&[1], vec![-1.0e-12_f64]);
    let r = log1mexp(&x).unwrap();
    close(*r.get(&[0]).unwrap(), (1.0e-12_f64).ln(), 1e-12);
    let a = tensor(&[1], vec![0.0_f64]);
    let b = tensor(&[1], vec![-1.0e-12_f64]);
    let d = logdiffexp(&a, &b).unwrap();
    close(*d.get(&[0]).unwrap(), (1.0e-12_f64).ln(), 1e-12);
    assert_eq!(
        *logdiffexp(&a, &a).unwrap().get(&[0]).unwrap(),
        f64::NEG_INFINITY
    );
}

#[test]
fn parameterized_softplus_preserves_limits_and_rejects_invalid_beta() {
    use kernix_ops::softplus_beta;
    let x = tensor(&[2], vec![-20.0f32, 20.0]);
    let y = softplus_beta(&x, 2.0).unwrap();
    assert!(*y.get(&[0]).unwrap() > 0.0);
    close(*y.get(&[1]).unwrap() as f64, 20.0, 1e-6);
    assert!(softplus_beta(&x, 0.0).is_err());
    assert!(softplus_beta(&x, f32::NAN).is_err());
}

#[test]
fn contiguous_matmul_zero_output_returns_without_iterating_huge_outer_axis() {
    let a = ContiguousOnly {
        dims: &[usize::MAX, 0],
        values: &[],
    };
    let b = ContiguousOnly {
        dims: &[0, 0],
        values: &[],
    };
    let product = matmul(&a, &b).unwrap();
    assert_eq!(product.shape().dims(), &[usize::MAX, 0]);
    assert_eq!(product.numel(), 0);
}

#[test]
fn contiguous_elementwise_and_reductions_use_direct_memory() {
    use kernix_ops::{add, map, mean_all, prod_all, sum_all, zip_map};
    let a = ContiguousOnly {
        dims: &[3],
        values: &[1.0, 2.0, 3.0],
    };
    let b = ContiguousOnly {
        dims: &[3],
        values: &[4.0, 5.0, 6.0],
    };
    assert_eq!(
        map(&a, |x| *x + 1.0).unwrap().storage_slice(),
        &[2.0, 3.0, 4.0]
    );
    assert_eq!(
        zip_map(&a, &b, |x, y| *x * *y).unwrap().storage_slice(),
        &[4.0, 10.0, 18.0]
    );
    assert_eq!(add(&a, &b).unwrap().storage_slice(), &[5.0, 7.0, 9.0]);
    close(sum_all(&a).unwrap(), 6.0, 0.0);
    close(prod_all(&a).unwrap(), 6.0, 0.0);
    close(mean_all(&a).unwrap(), 2.0, 1e-15);
}

#[test]
fn noncontiguous_elementwise_and_reductions_preserve_logical_order() {
    use kernix_ops::{add, mean_all, sum_all};
    let a = tensor(&[2, 3], vec![1.0f64, 2.0, 3.0, 4.0, 5.0, 6.0]);
    let backward = a.reverse_axis(1).unwrap();
    let result = add(&backward, &a).unwrap();
    assert_eq!(result.storage_slice(), &[4.0, 4.0, 4.0, 10.0, 10.0, 10.0]);
    close(sum_all(&backward).unwrap(), 21.0, 0.0);
    close(mean_all(&backward).unwrap(), 3.5, 1e-15);
}
