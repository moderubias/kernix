//! Integration regressions for ops hardening tests.
use kernix_core::{layout::Order, shape::Kahen, tensor::Tensor};
use kernix_ops::{
    OpsError, checked_add, checked_div, checked_mul, checked_neg, checked_rem, checked_sub,
    max_all, max_axes, min_all, min_axes, try_map, try_map_indexed, try_zip_map,
    try_zip_map_indexed,
};

fn tensor<T>(dims: &[usize], data: Vec<T>) -> Tensor<T> {
    Tensor::from_vec(data, Kahen::new(dims).unwrap(), Order::C).unwrap()
}

fn arithmetic(operation: &'static str, indices: &[usize]) -> OpsError {
    OpsError::Arithmetic {
        operation,
        indices: indices.to_vec(),
    }
}

#[test]
fn checked_add_broadcast_reports_output_coordinates() {
    let a = tensor(&[2, 1], vec![250_u8, 4]);
    let b = tensor(&[3], vec![1_u8, 6, 10]);
    assert_eq!(checked_add(&a, &b).unwrap_err(), arithmetic("add", &[0, 1]));
}

#[test]
fn checked_arithmetic_handles_successful_broadcast() {
    let a = tensor(&[2, 1], vec![20_i16, 30]);
    let b = tensor(&[3], vec![1_i16, 2, 3]);
    assert_eq!(
        checked_add(&a, &b).unwrap().storage_slice(),
        &[21, 22, 23, 31, 32, 33]
    );
    assert_eq!(
        checked_sub(&a, &b).unwrap().storage_slice(),
        &[19, 18, 17, 29, 28, 27]
    );
    assert_eq!(
        checked_mul(&a, &b).unwrap().storage_slice(),
        &[20, 40, 60, 30, 60, 90]
    );
}

#[test]
fn checked_sub_detects_unsigned_underflow() {
    let a = tensor(&[1], vec![0_u8]);
    let b = tensor(&[1], vec![1_u8]);
    assert_eq!(checked_sub(&a, &b).unwrap_err(), arithmetic("sub", &[0]));
}

#[test]
fn checked_mul_detects_signed_overflow() {
    let a = tensor(&[1], vec![100_i8]);
    let b = tensor(&[1], vec![2_i8]);
    assert_eq!(checked_mul(&a, &b).unwrap_err(), arithmetic("mul", &[0]));
}

#[test]
fn checked_div_detects_zero_divisor_without_panicking() {
    let a = tensor(&[2], vec![8_i32, 10]);
    let b = tensor(&[2], vec![2_i32, 0]);
    assert_eq!(checked_div(&a, &b).unwrap_err(), arithmetic("div", &[1]));
}

#[test]
fn checked_div_detects_signed_min_overflow() {
    let a = tensor(&[1], vec![i8::MIN]);
    let b = tensor(&[1], vec![-1_i8]);
    assert_eq!(checked_div(&a, &b).unwrap_err(), arithmetic("div", &[0]));
    assert_eq!(checked_rem(&a, &b).unwrap_err(), arithmetic("rem", &[0]));
}

#[test]
fn checked_neg_covers_unsigned_and_signed_boundaries() {
    let positive = tensor(&[], vec![1_u8]);
    assert_eq!(checked_neg(&positive).unwrap_err(), arithmetic("neg", &[]));
    let min = tensor(&[1], vec![i32::MIN]);
    assert_eq!(checked_neg(&min).unwrap_err(), arithmetic("neg", &[0]));
    let values = tensor(&[3], vec![0_i32, -7, 8]);
    assert_eq!(checked_neg(&values).unwrap().storage_slice(), &[0, 7, -8]);
}

#[test]
fn try_map_returns_original_error() {
    let a = tensor(&[3], vec![1_u8, 2, 3]);
    let result = try_map(&a, |x| {
        if *x == 2 {
            Err(arithmetic("user", &[1]))
        } else {
            Ok(*x)
        }
    });
    assert_eq!(result.unwrap_err(), arithmetic("user", &[1]));
}

#[test]
fn try_zip_map_is_fallible_and_broadcasts() {
    let a = tensor(&[1, 2], vec![10_i32, 20]);
    let b = tensor(&[2, 1], vec![1_i32, 2]);
    let result = try_zip_map(&a, &b, |x, y| Ok(*x - *y)).unwrap();
    assert_eq!(result.shape().dims(), &[2, 2]);
    assert_eq!(result.storage_slice(), &[9, 19, 8, 18]);
}

#[test]
fn indexed_map_reports_logical_coordinates() {
    let a = tensor(&[2, 2], vec![0_u8, 1, 2, 3]);
    let result = try_map_indexed(&a, |v, index| {
        Ok(u32::from(*v) + index[0] as u32 * 10 + index[1] as u32)
    })
    .unwrap();
    assert_eq!(result.storage_slice(), &[0, 2, 12, 14]);
}

#[test]
fn indexed_zip_uses_broadcasted_output_index() {
    let a = tensor(&[], vec![3_i32]);
    let b = tensor(&[2], vec![5_i32, 7]);
    let result = try_zip_map_indexed(&a, &b, |x, y, i| Ok(*x + *y + i[0] as i32)).unwrap();
    assert_eq!(result.storage_slice(), &[8, 11]);
}

#[test]
fn fallible_callbacks_are_not_called_for_empty_output() {
    let a = tensor(&[0, 3], Vec::<i16>::new());
    let mut calls = 0;
    let out = try_map(&a, |_v| {
        calls += 1;
        Ok(999_i16)
    })
    .unwrap();
    assert_eq!(calls, 0);
    assert_eq!(out.shape().dims(), &[0, 3]);
    assert!(out.storage_slice().is_empty());
}

#[test]
fn min_and_max_propagate_nan_regardless_of_position() {
    for values in [
        [f32::NAN, 1.0, 3.0],
        [1.0, f32::NAN, 3.0],
        [1.0, 3.0, f32::NAN],
    ] {
        let x = tensor(&[3], values.to_vec());
        assert!(min_all(&x).unwrap().unwrap().is_nan());
        assert!(max_all(&x).unwrap().unwrap().is_nan());
    }
}

#[test]
fn axis_extrema_propagate_nan_only_within_affected_group() {
    let x = tensor(&[2, 3], vec![2_f64, f64::NAN, 1.0, 5.0, 4.0, 3.0]);
    let min = min_axes(&x, &[1], false).unwrap();
    let max = max_axes(&x, &[1], false).unwrap();
    assert!(min.storage_slice()[0].is_nan());
    assert_eq!(min.storage_slice()[1], 3.0);
    assert!(max.storage_slice()[0].is_nan());
    assert_eq!(max.storage_slice()[1], 5.0);
}
