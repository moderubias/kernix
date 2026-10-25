//! Rank-one factor modifications, independent reconstruction and transactional failures.
use kernix_core::tensor::Tensor;
use kernix_ops::linalg::{Cholesky, LinalgError};

#[test]
fn updates_downdates_reconstruct_solve_and_preserve_inputs() {
    for n in [0usize, 1, 2, 7, 16, 33, 65] {
        let values: Vec<f64> = (0..n * n)
            .map(|p| {
                if p / n == p % n {
                    4.0
                } else {
                    0.01 / (1.0 + (p / n).abs_diff(p % n) as f64)
                }
            })
            .collect();
        let a = Tensor::from_values([n, n], values.clone()).unwrap();
        let base = Cholesky::factor(&a).unwrap();
        let saved = base.lower().storage_slice().to_vec();
        let vector =
            Tensor::from_values([n], (0..n).map(|i| (i % 7) as f64 / 32.0 - 0.125).collect())
                .unwrap();
        let reversed = vector.reverse_axis(0).unwrap();
        for subtract in [false, true] {
            let updated = if subtract {
                base.rank_one_downdate(&reversed)
            } else {
                base.rank_one_update(&reversed)
            }
            .unwrap();
            let l = updated.lower().storage_slice();
            let u: Vec<f64> = (0..n).map(|i| *reversed.get(&[i]).unwrap()).collect();
            let expected: Vec<f64> = (0..n * n)
                .map(|p| {
                    values[p]
                        + if subtract {
                            -u[p / n] * u[p % n]
                        } else {
                            u[p / n] * u[p % n]
                        }
                })
                .collect();
            for i in 0..n {
                for j in 0..n {
                    let value: f64 = (0..=i.min(j)).map(|k| l[i * n + k] * l[j * n + k]).sum();
                    assert!(
                        (value - expected[i * n + j]).abs()
                            < 2e-12 * (1.0 + expected[i * n + j].abs()),
                        "{n} {i}/{j}"
                    );
                    if j > i {
                        assert_eq!(l[i * n + j], 0.0);
                    }
                }
            }
            let b = Tensor::from_values(
                [n],
                (0..n)
                    .map(|i| expected[i * n..(i + 1) * n].iter().sum())
                    .collect(),
            )
            .unwrap();
            for &x in updated.solve(&b).unwrap().storage_slice() {
                assert!((x - 1.0).abs() < 2e-12);
            }
            let rebuilt =
                Cholesky::factor(&Tensor::from_values([n, n], expected).unwrap()).unwrap();
            for (&x, &y) in l.iter().zip(rebuilt.lower().storage_slice()) {
                assert!((x - y).abs() < 2e-12);
            }
        }
        assert_eq!(base.lower().storage_slice(), saved);
        assert_eq!(a.storage_slice(), values);
    }
}
#[test]
fn float32_updates_are_scale_aware() {
    for scale in [1e-10f32, 1.0, 1e10] {
        let n = 17;
        let a = Tensor::from_values(
            [n, n],
            (0..n * n)
                .map(|p| {
                    if p / n == p % n {
                        4.0 * scale * scale
                    } else {
                        0.0
                    }
                })
                .collect(),
        )
        .unwrap();
        let base = Cholesky::factor(&a).unwrap();
        let u = Tensor::from_values(
            [n],
            (0..n)
                .map(|i| ((i % 5) as f32 - 2.0) * scale / 32.0)
                .collect(),
        )
        .unwrap();
        for subtract in [false, true] {
            let result = if subtract {
                base.rank_one_downdate(&u)
            } else {
                base.rank_one_update(&u)
            }
            .unwrap();
            let l = result.lower().storage_slice();
            for i in 0..n {
                for j in 0..n {
                    let actual: f64 = (0..=i.min(j))
                        .map(|k| f64::from(l[i * n + k] / scale) * f64::from(l[j * n + k] / scale))
                        .sum();
                    let outer = f64::from(u.storage_slice()[i] / scale)
                        * f64::from(u.storage_slice()[j] / scale);
                    let expected = f64::from(a.storage_slice()[i * n + j])
                        / f64::from(scale).powi(2)
                        + if subtract { -outer } else { outer };
                    assert!(
                        (actual - expected).abs() < 1e-5,
                        "{scale}/{i}/{j}: {actual} {expected}"
                    );
                }
            }
        }
    }
}
#[test]
fn failed_downdates_and_nonfinite_vectors_leave_factor_unchanged() {
    let a = Tensor::from_rows([[1.0f64, 0.0], [0.0, 1.0]]).unwrap();
    let base = Cholesky::factor(&a).unwrap();
    let saved = base.lower().storage_slice().to_vec();
    for (v, pivot) in [([2.0, 0.0], 0), ([0.5, 1.0], 1), ([0.0, 1.0], 1)] {
        let u = Tensor::from_values([2], v.to_vec()).unwrap();
        assert!(
            matches!(base.rank_one_downdate(&u),Err(LinalgError::NotPositiveDefinite{pivot:p}) if p==pivot)
        );
        assert_eq!(base.lower().storage_slice(), saved);
    }
    for bad in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        let u = Tensor::from_values([2], vec![bad, 0.0]).unwrap();
        assert!(matches!(
            base.rank_one_update(&u),
            Err(LinalgError::NonFinite)
        ));
        assert!(matches!(
            base.rank_one_downdate(&u),
            Err(LinalgError::NonFinite)
        ));
    }
    assert!(
        base.rank_one_update(&Tensor::<f64>::zeros([1, 2]).unwrap())
            .is_err()
    );
    assert!(
        base.rank_one_downdate(&Tensor::<f64>::zeros([3]).unwrap())
            .is_err()
    );
    assert_eq!(base.lower().storage_slice(), saved);
}
#[test]
fn rotations_avoid_squared_overflow_and_retain_small_downdate_gaps() {
    let base = Cholesky::factor(&Tensor::from_rows([[1e-280f64]]).unwrap()).unwrap();
    let huge = base
        .rank_one_update(&Tensor::from_values([1], vec![1e200]).unwrap())
        .unwrap();
    assert_eq!(huge.lower().storage_slice(), &[1e200]);
    let base = Cholesky::factor(&Tensor::from_rows([[2.25f64]]).unwrap()).unwrap();
    let d = 1.5f64;
    let x = f64::from_bits(d.to_bits() - 1);
    let tiny = base
        .rank_one_downdate(&Tensor::from_values([1], vec![x]).unwrap())
        .unwrap();
    let expected = ((d - x) * (d + x)).sqrt();
    assert!((tiny.lower().storage_slice()[0] / expected - 1.0).abs() < 5e-15);
}
#[test]
fn repeated_updates_and_reverse_downdates_recover_the_original() {
    let n = 12;
    let a = Tensor::from_values(
        [n, n],
        (0..n * n)
            .map(|p| if p / n == p % n { 4.0f64 } else { 0.0 })
            .collect(),
    )
    .unwrap();
    let original = Cholesky::factor(&a).unwrap();
    let mut factor = Cholesky::factor(&a).unwrap();
    let vectors: Vec<_> = (0..64)
        .map(|i| {
            Tensor::from_values(
                [n],
                (0..n)
                    .map(|j| (((i * 7 + j * 3) % 11) as f64 - 5.0) / 64.0)
                    .collect(),
            )
            .unwrap()
        })
        .collect();
    for v in &vectors {
        factor = factor.rank_one_update(v).unwrap();
    }
    for v in vectors.iter().rev() {
        factor = factor.rank_one_downdate(v).unwrap();
    }
    for (&x, &y) in factor
        .lower()
        .storage_slice()
        .iter()
        .zip(original.lower().storage_slice())
    {
        assert!((x - y).abs() < 1e-13);
    }
}
