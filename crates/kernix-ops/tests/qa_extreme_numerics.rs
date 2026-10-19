//! Integration regressions for qa extreme numerics.
use kernix_core::{Order, shape::Kahen, tensor::Tensor};
use kernix_ops::{log_softmax_temperature, silu, softmax_online_temperature, softmax_temperature};

fn tensor(values: Vec<f64>) -> Tensor<f64> {
    let shape = Kahen::new([values.len()]).unwrap();
    Tensor::from_vec(values, shape, Order::C).unwrap()
}

#[test]
fn large_temperature_preserves_finite_logit_differences() {
    let x = tensor(vec![f64::MAX, -f64::MAX]);
    let expected = 1.0 / (1.0 + (-2.0_f64).exp());
    for out in [
        softmax_temperature(&x, 0, f64::MAX).unwrap(),
        softmax_online_temperature(&x, 0, f64::MAX).unwrap(),
    ] {
        assert!((out.storage_slice()[0] - expected).abs() < 1e-14);
        assert!((out.storage_slice()[1] - (1.0 - expected)).abs() < 1e-14);
    }
    let log = log_softmax_temperature(&x, 0, f64::MAX).unwrap();
    assert!((log.storage_slice()[1] - (1.0 - expected).ln()).abs() < 1e-14);
}

#[test]
fn silu_has_defined_negative_infinity_limit() {
    let out = silu(&tensor(vec![f64::NEG_INFINITY, f64::INFINITY, f64::NAN])).unwrap();
    assert_eq!(out.storage_slice()[0], 0.0);
    assert_eq!(out.storage_slice()[1], f64::INFINITY);
    assert!(out.storage_slice()[2].is_nan());
}
