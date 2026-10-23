//! Pivoted reconstruction, rank policy and minimum-norm independent oracles.
use kernix_core::{Order, shape::Kahen, tensor::Tensor};
use kernix_ops::{
    TensorInput,
    linalg::{LinalgError, PivotedQr, Qr},
};

fn family(m: usize, n: usize, rank: usize) -> Tensor<f64> {
    let u: Vec<f64> = (0..m * rank)
        .map(|v| {
            let (i, k) = (v / rank, v % rank);
            (if i == k { 3. } else { 0. }) + ((i * 7 + k * 3) % 11) as f64 / 16. - 5. / 16.
        })
        .collect();
    Tensor::from_values(
        [m, n],
        (0..m * n)
            .map(|v| {
                let (i, j) = (v / n, v % n);
                if j < rank {
                    u[i * rank + j]
                } else {
                    let k = (j - rank) % rank;
                    u[i * rank + k] + 0.5 * u[i * rank + (k + 1) % rank]
                }
            })
            .collect(),
    )
    .unwrap()
}
fn reconstruct<A: TensorInput<f64>>(a: &A, qr: &PivotedQr<f64>) {
    let (m, n) = (a.dims()[0], a.dims()[1]);
    let thin = m.min(n);
    let mut p = qr.permutation().to_vec();
    p.sort_unstable();
    assert_eq!(p, (0..n).collect::<Vec<_>>());
    for i in 0..m {
        for j in 0..n {
            let expected = *a.get(&[i, qr.permutation()[j]]).unwrap();
            let value: f64 = (0..thin)
                .map(|k| qr.q().storage_slice()[i * thin + k] * qr.r().storage_slice()[k * n + j])
                .sum();
            assert!((value - expected).abs() < 2e-12 * (1. + expected.abs()));
        }
    }
    for i in 0..thin {
        for j in 0..thin {
            let dot: f64 = (0..m)
                .map(|k| {
                    qr.q().storage_slice()[k * thin + i] * qr.q().storage_slice()[k * thin + j]
                })
                .sum();
            assert!((dot - if i == j { 1. } else { 0. }).abs() < 2e-12);
        }
    }
    for i in 0..thin {
        for j in 0..i.min(n) {
            assert_eq!(qr.r().storage_slice()[i * n + j], 0.);
        }
    }
}

#[test]
fn rectangular_reconstruction_rank_order_layouts_and_pivot_orientation() {
    for (m, n, r) in [
        (1, 1, 1),
        (7, 3, 3),
        (3, 7, 3),
        (9, 7, 3),
        (5, 9, 2),
        (12, 8, 5),
    ] {
        let a = family(m, n, r);
        let saved = a.storage_slice().to_vec();
        let qr = PivotedQr::factor_with_tolerance(&a, 1e-12).unwrap();
        assert_eq!(qr.rank(), r);
        assert_eq!(qr.relative_tolerance(), 1e-12);
        reconstruct(&a, &qr);
        assert_eq!(a.storage_slice(), saved);
        let f = Tensor::from_fn(Kahen::new([m, n]).unwrap(), Order::F, 8, |i| {
            saved[i[0] * n + i[1]]
        })
        .unwrap();
        reconstruct(&f, &PivotedQr::factor(&f).unwrap());
        let reverse = a.reverse_axis(0).unwrap();
        reconstruct(&reverse, &PivotedQr::factor(&reverse).unwrap());
    }
    let a = Tensor::from_values([2, 3], vec![1.0, 0., 10., 0., 2., 0.]).unwrap();
    let qr = PivotedQr::factor(&a).unwrap();
    assert_eq!(qr.permutation()[0], 2);
    assert_eq!(qr.permutation()[1], 1);
    reconstruct(&a, &qr);
    let repeated = Tensor::<f64>::ones([3, 3]).unwrap();
    assert_eq!(PivotedQr::factor(&repeated).unwrap().permutation()[0], 0);
}

#[test]
fn minimum_norm_differs_from_basic_and_matches_analytic_null_space() {
    let a = Tensor::from_values([2, 3], vec![1.0f64, 0., 1., 0., 1., 1.]).unwrap();
    let b = Tensor::ones([2]).unwrap();
    let qr = PivotedQr::factor(&a).unwrap();
    let x = qr.solve_least_squares(&b).unwrap();
    let basic = qr.solve_basic_least_squares(&b).unwrap();
    for (&actual, expected) in x.storage_slice().iter().zip([1. / 3., 1. / 3., 2. / 3.]) {
        assert!((actual - expected).abs() < 1e-14);
    }
    assert!(
        x.storage_slice().iter().map(|x| x * x).sum::<f64>()
            < basic.storage_slice().iter().map(|x| x * x).sum::<f64>()
    );
    assert!((-x.storage_slice()[0] - x.storage_slice()[1] + x.storage_slice()[2]).abs() < 1e-14);
    assert!(Qr::factor(&a).unwrap().solve_least_squares(&b).is_err());
    let a = Tensor::from_values([2, 3], vec![1.0f64, 2., 3., 2., 4., 6.]).unwrap();
    let b = Tensor::from_values([2], vec![3., 1.]).unwrap();
    let qr = PivotedQr::factor(&a).unwrap();
    assert_eq!(qr.rank(), 1);
    let x = qr.solve_least_squares(&b).unwrap();
    for (&actual, v) in x.storage_slice().iter().zip([1., 2., 3.]) {
        assert!((actual - v / 14.).abs() < 1e-14);
    }
}

#[test]
fn ranked_multi_rhs_agree_with_svd_residual_stationarity_and_null_space() {
    for (m, n, r) in [(9, 7, 3), (5, 9, 2), (7, 4, 4), (3, 7, 3), (8, 8, 5)] {
        let a = family(m, n, r);
        let rhs: Vec<f64> = (0..m * 3).map(|i| (i as f64 - 3.) / 8.).collect();
        let b = Tensor::from_values([m, 3], rhs.clone()).unwrap();
        let qr = PivotedQr::factor_with_tolerance(&a, 1e-12).unwrap();
        assert_eq!(qr.rank(), r);
        let x = qr.solve_least_squares(&b).unwrap();
        assert_eq!(x.shape().dims(), [n, 3]);
        let oracle = nalgebra::DMatrix::from_row_slice(m, n, a.storage_slice())
            .svd(true, true)
            .solve(&nalgebra::DMatrix::from_row_slice(m, 3, &rhs), 1e-11)
            .unwrap();
        for i in 0..n {
            for j in 0..3 {
                assert!((x.storage_slice()[i * 3 + j] - oracle[(i, j)]).abs() < 1e-11);
            }
        }
        for j in 0..3 {
            let residual: Vec<f64> = (0..m)
                .map(|i| {
                    (0..n)
                        .map(|k| a.storage_slice()[i * n + k] * x.storage_slice()[k * 3 + j])
                        .sum::<f64>()
                        - rhs[i * 3 + j]
                })
                .collect();
            for k in 0..n {
                assert!(
                    (0..m)
                        .map(|i| a.storage_slice()[i * n + k] * residual[i])
                        .sum::<f64>()
                        .abs()
                        < 2e-11
                );
            }
            for free in r..n {
                let k = (free - r) % r;
                let dot = x.storage_slice()[free * 3 + j]
                    - x.storage_slice()[k * 3 + j]
                    - 0.5 * x.storage_slice()[((k + 1) % r) * 3 + j];
                assert!(dot.abs() < 1e-11);
            }
        }
        let f = Tensor::from_fn(Kahen::new([m, 3]).unwrap(), Order::F, 8, |i| {
            rhs[i[0] * 3 + i[1]]
        })
        .unwrap();
        let transposed_rhs_solution = qr.solve_least_squares(&f).unwrap();
        for (&actual, &expected) in transposed_rhs_solution
            .storage_slice()
            .iter()
            .zip(x.storage_slice())
        {
            assert!((actual - expected).abs() < 1e-12);
        }
    }
}

#[test]
fn thresholds_scaling_f32_and_finite_validation() {
    for scale in [1e-200f64, 1., 1e200] {
        let a = Tensor::from_values([2, 2], vec![scale, 0., 0., scale * 1e-8]).unwrap();
        assert_eq!(
            PivotedQr::factor_with_tolerance(&a, 1e-6).unwrap().rank(),
            1
        );
        assert_eq!(
            PivotedQr::factor_with_tolerance(&a, 1e-10).unwrap().rank(),
            2
        );
        assert_eq!(PivotedQr::factor_with_tolerance(&a, 0.).unwrap().rank(), 2);
        let qr = PivotedQr::factor_with_tolerance(&a, 1e300).unwrap();
        assert_eq!(qr.rank(), 0);
        assert_eq!(
            qr.solve_least_squares(&Tensor::<f64>::ones([2]).unwrap())
                .unwrap()
                .storage_slice(),
            [0., 0.]
        );
    }
    for scale in [1e-30f32, 1., 1e30] {
        let a = Tensor::from_values([2, 3], vec![scale, 0., scale, 0., scale, scale]).unwrap();
        let b = Tensor::from_values([2], vec![scale, scale]).unwrap();
        let qr = PivotedQr::factor(&a).unwrap();
        assert_eq!(qr.rank(), 2);
        let x = qr.solve_least_squares(&b).unwrap();
        for (&value, expected) in x.storage_slice().iter().zip([1. / 3., 1. / 3., 2. / 3.]) {
            assert!((value - expected).abs() < 1e-5);
        }
    }
    let zero = Tensor::<f64>::zeros([2, 3]).unwrap();
    let qr = PivotedQr::factor(&zero).unwrap();
    assert_eq!(qr.rank(), 0);
    for bad in [f64::NAN, f64::INFINITY] {
        assert!(matches!(
            qr.solve_least_squares(&Tensor::from_values([2], vec![bad, 0.]).unwrap()),
            Err(LinalgError::NonFinite)
        ));
        assert!(PivotedQr::factor(&Tensor::from_values([1, 1], vec![bad]).unwrap()).is_err());
    }
    for tol in [-1., f64::NAN, f64::INFINITY] {
        assert!(PivotedQr::factor_with_tolerance(&zero, tol).is_err());
    }
    assert!(
        qr.solve_least_squares(&Tensor::<f64>::ones([3]).unwrap())
            .is_err()
    );
    assert!(PivotedQr::factor(&Tensor::<f64>::ones([3]).unwrap()).is_err());
    let unrepresentable_norm = Tensor::full([2, 1], f64::MAX).unwrap();
    assert!(matches!(
        PivotedQr::factor(&unrepresentable_norm),
        Err(LinalgError::NonFinite)
    ));
    let a = Tensor::from_values([2, 2], vec![0.6f64, -0.8, 0.8, 0.6]).unwrap();
    assert!(matches!(
        PivotedQr::factor(&a)
            .unwrap()
            .solve_least_squares(&Tensor::full([2], f64::MAX).unwrap()),
        Err(LinalgError::NonFinite)
    ));
    let huge = Tensor::from_values([1, 2], vec![f64::MAX, f64::MAX]).unwrap();
    let x = PivotedQr::factor(&huge)
        .unwrap()
        .solve_least_squares(&Tensor::<f64>::ones([1]).unwrap())
        .unwrap();
    for &v in x.storage_slice() {
        assert!((v * f64::MAX - 0.5).abs() < 1e-14);
    }
}

#[test]
fn empty_axes_zero_rhs_columns_and_resource_failures_do_not_loop() {
    for (m, n) in [(0, 0), (0, 3), (3, 0), (isize::MAX as usize, 0)] {
        let a = Tensor::<f64>::zeros([m, n]).unwrap();
        let qr = PivotedQr::factor(&a).unwrap();
        assert_eq!(qr.rank(), 0);
        let rhs = Tensor::<f64>::zeros([m, 0]).unwrap();
        assert_eq!(qr.solve_least_squares(&rhs).unwrap().shape().dims(), [n, 0]);
        assert_eq!(
            qr.solve_basic_least_squares(&rhs).unwrap().shape().dims(),
            [n, 0]
        );
    }
    let a = family(7, 5, 2);
    let qr = PivotedQr::factor(&a).unwrap();
    assert_eq!(
        qr.solve_least_squares(&Tensor::<f64>::zeros([7, 0]).unwrap())
            .unwrap()
            .numel(),
        0
    );
    struct Fake {
        dims: [usize; 2],
        cap: Option<&'static [f64]>,
    }
    impl TensorInput<f64> for Fake {
        fn dims(&self) -> &[usize] {
            &self.dims
        }
        fn get(&self, _: &[usize]) -> Result<&f64, kernix_core::tensor::TensorError> {
            panic!("invalid extent must be rejected before access")
        }
        fn as_c_contiguous_slice(&self) -> Option<&[f64]> {
            self.cap
        }
    }
    for f in [
        Fake {
            dims: [2, 2],
            cap: Some(&[0.]),
        },
        Fake {
            dims: [0, 1],
            cap: Some(&[0.]),
        },
        Fake {
            dims: [usize::MAX, 1],
            cap: None,
        },
        Fake {
            dims: [0, usize::MAX],
            cap: None,
        },
    ] {
        assert!(PivotedQr::factor(&f).is_err());
    }
    assert!(
        qr.solve_least_squares(&Fake {
            dims: [7, 1],
            cap: Some(&[1.])
        })
        .is_err()
    );
}

#[test]
fn numerical_rank_policy_changes_the_solved_matrix_not_the_returned_factors() {
    let a = Tensor::from_values([2, 2], vec![1.0f64, 0., 0., 1e-8]).unwrap();
    let qr = PivotedQr::factor_with_tolerance(&a, 1e-6).unwrap();
    reconstruct(&a, &qr);
    assert_eq!(qr.rank(), 1);
    let x = qr
        .solve_least_squares(&Tensor::<f64>::ones([2]).unwrap())
        .unwrap();
    assert_eq!(x.storage_slice(), [1., 0.]);
    let full = PivotedQr::factor_with_tolerance(&a, 1e-10).unwrap();
    let x = full
        .solve_least_squares(&Tensor::<f64>::ones([2]).unwrap())
        .unwrap();
    assert_eq!(x.storage_slice(), [1., 1e8]);
}

#[test]
fn right_basis_scaling_does_not_underflow_the_rhs_before_solving() {
    let a = Tensor::from_values([2, 3], vec![1e300f64, 0., 0., 0., 1., 0.]).unwrap();
    let b = Tensor::from_values([2], vec![1e300, 1e-300]).unwrap();
    let qr = PivotedQr::factor_with_tolerance(&a, 0.).unwrap();
    assert_eq!(qr.rank(), 2);
    let x = qr.solve_least_squares(&b).unwrap();
    assert!((x.storage_slice()[0] - 1.).abs() < 1e-14);
    assert!((x.storage_slice()[1] / 1e-300 - 1.).abs() < 1e-14);
    assert_eq!(x.storage_slice()[2], 0.);
}

#[test]
fn cancellation_in_partial_norms_recomputes_tiny_remaining_columns() {
    let a = Tensor::from_values(
        [4, 3],
        vec![1.0f64, 1., 1., 0., 1e-16, 0., 0., 0., 1e-20, 0., 0., 0.],
    )
    .unwrap();
    let qr = PivotedQr::factor_with_tolerance(&a, 0.).unwrap();
    assert_eq!(qr.rank(), 3);
    assert_eq!(qr.permutation(), [0, 1, 2]);
    assert!((qr.r().storage_slice()[4].abs() / 1e-16 - 1.).abs() < 1e-14);
    assert!((qr.r().storage_slice()[8].abs() / 1e-20 - 1.).abs() < 1e-14);
    reconstruct(&a, &qr);
    let a = Tensor::from_values([2, 3], vec![1e-310f64, 0., 1e-310, 0., 1e-310, 1e-310]).unwrap();
    let b = Tensor::full([2], 1e-310).unwrap();
    let qr = PivotedQr::factor(&a).unwrap();
    assert_eq!(qr.rank(), 2);
    let x = qr.solve_least_squares(&b).unwrap();
    for (&v, expected) in x.storage_slice().iter().zip([1. / 3., 1. / 3., 2. / 3.]) {
        assert!((v - expected).abs() < 1e-12);
    }
}

#[test]
fn subnormal_column_norm_rounding_must_not_destroy_orthogonality() {
    let tiny = f64::from_bits(2);
    let a = Tensor::from_values([2, 1], vec![tiny, tiny]).unwrap();
    let ordinary = Qr::factor(&a).unwrap();
    let pivoted = PivotedQr::factor(&a).unwrap();
    for (name, q) in [("ordinary", ordinary.q()), ("pivoted", pivoted.q())] {
        let norm = q.storage_slice().iter().map(|x| x * x).sum::<f64>();
        assert!((norm - 1.).abs() < 1e-14, "{name}: ||q||²={norm}");
    }
    let tiny = f32::from_bits(2);
    let a = Tensor::from_values([2, 1], vec![tiny, tiny]).unwrap();
    let ordinary = Qr::factor(&a).unwrap();
    let pivoted = PivotedQr::factor(&a).unwrap();
    for (name, q) in [("ordinary", ordinary.q()), ("pivoted", pivoted.q())] {
        let norm = q
            .storage_slice()
            .iter()
            .map(|&x| f64::from(x) * f64::from(x))
            .sum::<f64>();
        assert!((norm - 1.).abs() < 1e-6, "{name}: ||q||²={norm}");
    }
}
