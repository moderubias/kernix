//! Exact scalar-order substitution and independent multiple-RHS solve oracles.
use kernix_core::tensor::Tensor;
use kernix_ops::linalg::{Cholesky, Lu, Qr};
#[test]
fn float32_many_rhs_preserve_scaled_solutions() {
    let n = 33;
    for scale in [1e-12f32, 1.0, 1e12] {
        let a = Tensor::from_values(
            [n, n],
            (0..n * n)
                .map(|p| {
                    scale
                        * if p / n == p % n {
                            4.0
                        } else if (p / n).abs_diff(p % n) == 1 {
                            0.125
                        } else {
                            0.0
                        }
                })
                .collect(),
        )
        .unwrap();
        let chol = Cholesky::factor(&a).unwrap();
        let lu = Lu::factor(&a).unwrap();
        let qr = Qr::factor(&a).unwrap();
        for cols in [5, 17] {
            let expected: Vec<f32> = (0..n * cols)
                .map(|p| ((p * 7 % 23) as f32 - 11.0) / 16.0)
                .collect();
            let b = Tensor::from_values(
                [n, cols],
                (0..n * cols)
                    .map(|p| {
                        (0..n)
                            .map(|k| {
                                a.storage_slice()[p / cols * n + k] * expected[k * cols + p % cols]
                            })
                            .sum()
                    })
                    .collect(),
            )
            .unwrap();
            for x in [
                chol.solve(&b).unwrap(),
                lu.solve(&b).unwrap(),
                qr.solve_least_squares(&b).unwrap(),
            ] {
                for (&actual, &target) in x.storage_slice().iter().zip(&expected) {
                    assert!((actual - target).abs() < 2e-6, "{scale}/{cols}");
                }
            }
        }
    }
}
#[test]
fn reused_factors_solve_many_rhs_with_independent_known_solutions() {
    for n in [1usize, 7, 32, 65] {
        for cols in [0, 1, 4, 5, 16, 33] {
            let a = Tensor::from_values(
                [n, n],
                (0..n * n)
                    .map(|p| {
                        if p / n == p % n {
                            4.0f64
                        } else if (p / n).abs_diff(p % n) == 1 {
                            0.125
                        } else {
                            0.0
                        }
                    })
                    .collect(),
            )
            .unwrap();
            let expected: Vec<f64> = (0..n * cols)
                .map(|p| ((p * 7 % 23) as f64 - 11.0) / 16.0)
                .collect();
            let b = Tensor::from_values(
                [n, cols],
                (0..n * cols)
                    .map(|p| {
                        (0..n)
                            .map(|k| {
                                a.storage_slice()[p / cols * n + k] * expected[k * cols + p % cols]
                            })
                            .sum()
                    })
                    .collect(),
            )
            .unwrap();
            let chol = Cholesky::factor(&a).unwrap();
            let lu = Lu::factor(&a).unwrap();
            let qr = Qr::factor(&a).unwrap();
            for x in [
                chol.solve(&b).unwrap(),
                lu.solve(&b).unwrap(),
                qr.solve_least_squares(&b).unwrap(),
            ] {
                for (&actual, &target) in x.storage_slice().iter().zip(&expected) {
                    assert!((actual - target).abs() < 3e-12, "n{n} rhs{cols}");
                }
            }
            // Reversal forces logical RHS copying, while source values stay unchanged.
            let reversed = b.reverse_axis(1).unwrap();
            let canonical = reversed.to_owned_contiguous(kernix_core::Order::C).unwrap();
            assert_eq!(
                chol.solve(&reversed).unwrap().storage_slice(),
                chol.solve(&canonical).unwrap().storage_slice()
            );
        }
    }
}
#[test]
fn rowwise_cholesky_matches_sequential_reference_bit_for_bit() {
    let n = 65usize;
    let cols = 17;
    let a = Tensor::from_values(
        [n, n],
        (0..n * n)
            .map(|p| {
                if p / n == p % n {
                    4.0f64
                } else {
                    0.01 / (1.0 + (p / n).abs_diff(p % n) as f64)
                }
            })
            .collect(),
    )
    .unwrap();
    let factor = Cholesky::factor(&a).unwrap();
    let b = Tensor::from_values(
        [n, cols],
        (0..n * cols)
            .map(|p| (p % 29) as f64 / 17.0 - 0.5)
            .collect(),
    )
    .unwrap();
    let l = factor.lower().storage_slice();
    let mut expected = b.storage_slice().to_vec();
    for row in 0..n {
        for j in 0..cols {
            let mut value = expected[row * cols + j];
            for k in 0..row {
                value -= l[row * n + k] * expected[k * cols + j];
            }
            expected[row * cols + j] = value / l[row * n + row];
        }
    }
    for row in (0..n).rev() {
        for j in 0..cols {
            let mut value = expected[row * cols + j];
            for k in row + 1..n {
                value -= l[k * n + row] * expected[k * cols + j];
            }
            expected[row * cols + j] = value / l[row * n + row];
        }
    }
    assert_eq!(factor.solve(&b).unwrap().storage_slice(), expected);
}
