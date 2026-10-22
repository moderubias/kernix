//! Independent one-norm diagnostics and working-precision refinement contracts.
use kernix_core::tensor::Tensor;
use kernix_ops::linalg::{
    LinalgError, RefinementTermination, condition_number_one, reciprocal_condition_number_one,
    solve_refined,
};

#[test]
fn condition_diagonal_rotated_scaled_singular_and_empty() {
    for scale in [1e-300f64, 1., 1e300] {
        let a = Tensor::from_values([2, 2], vec![scale, 0., 0., scale * 0.125]).unwrap();
        assert!((condition_number_one(&a).unwrap() - 8.).abs() < 1e-12);
        assert!((reciprocal_condition_number_one(&a).unwrap() - 0.125).abs() < 1e-14);
        let a = Tensor::from_values(
            [2, 2],
            vec![scale * 0.6, -scale * 0.008, scale * 0.8, scale * 0.006],
        )
        .unwrap();
        let expected = 1.4 * 80.6;
        assert!((condition_number_one(&a).unwrap() - expected).abs() < 1e-11);
        let transposed = a.transpose().unwrap();
        assert!((condition_number_one(&transposed).unwrap() - (0.806 * 140.)).abs() < 1e-11);
    }
    let huge = Tensor::from_values([2, 2], vec![f64::MAX, f64::MAX, f64::MAX, -f64::MAX]).unwrap();
    assert!((condition_number_one(&huge).unwrap() - 2.).abs() < 1e-14);
    for v in [vec![0.; 4], vec![1.; 4]] {
        let a = Tensor::from_values([2, 2], v).unwrap();
        assert_eq!(reciprocal_condition_number_one(&a).unwrap(), 0.);
        assert!(matches!(
            condition_number_one(&a),
            Err(LinalgError::NonFinite)
        ));
    }
    assert_eq!(
        condition_number_one(&Tensor::<f64>::zeros([0, 0]).unwrap()).unwrap(),
        1.
    );
    let tiny = Tensor::from_values([2, 2], vec![1., 0., 0., f64::from_bits(1)]).unwrap();
    assert!(matches!(
        reciprocal_condition_number_one(&tiny),
        Err(LinalgError::NonFinite)
    ));
}

#[test]
fn condition_f32_matches_independent_explicit_inverse_and_validates_geometry() {
    let a =
        Tensor::from_values([3, 3], vec![0.1f32, 0.2, -0.1, 0.3, 2., 0.1, -0.4, 0.7, 1.]).unwrap();
    let matrix = nalgebra::DMatrix::from_row_slice(
        3,
        3,
        &a.storage_slice()
            .iter()
            .map(|&x| f64::from(x))
            .collect::<Vec<_>>(),
    );
    let inverse = matrix.clone().try_inverse().unwrap();
    let norm = |m: &nalgebra::DMatrix<f64>| {
        (0..3)
            .map(|j| (0..3).map(|i| m[(i, j)].abs()).sum::<f64>())
            .fold(0., f64::max)
    };
    let expected = norm(&matrix) * norm(&inverse);
    assert!((f64::from(condition_number_one(&a).unwrap()) - expected).abs() < 1e-5 * expected);
    assert!(condition_number_one(&Tensor::<f64>::ones([2, 3]).unwrap()).is_err());
    for bad in [f64::NAN, f64::INFINITY] {
        assert!(matches!(
            condition_number_one(&Tensor::from_values([1, 1], vec![bad]).unwrap()),
            Err(LinalgError::NonFinite)
        ));
    }
}

#[test]
fn refinement_reports_budget_stagnation_and_never_accepts_worse_backward_error() {
    let mut improvements = 0;
    let mut stagnations = 0;
    for n in 2..9 {
        let a = Tensor::from_values(
            [n, n],
            (0..n * n)
                .map(|i| ((i * 7 % 17) as f32 - 8.) / 13. + if i / n == i % n { 3. } else { 0. })
                .collect(),
        )
        .unwrap();
        let b = Tensor::from_values([n, 2], (0..n * 2).map(|i| (i as f32 - 3.) / 7.).collect())
            .unwrap();
        let initial = solve_refined(&a, &b, 0, 0.).unwrap();
        let result = solve_refined(&a, &b, 8, 0.).unwrap();
        assert!(result.backward_error() <= initial.backward_error());
        assert!(result.iterations() <= 8);
        assert_eq!(result.solution().shape().dims(), [n, 2]);
        improvements += usize::from(result.backward_error() < initial.backward_error());
        stagnations += usize::from(result.termination() == RefinementTermination::Stagnated);
        if initial.backward_error() > 0. {
            assert_eq!(initial.termination(), RefinementTermination::IterationLimit);
        }
        let easy = solve_refined(&a, &b, 4, 1e-5).unwrap();
        assert_eq!(easy.termination(), RefinementTermination::Converged);
        assert!(easy.backward_error() <= 1e-5);
        let moved = easy.into_solution();
        assert_eq!(moved.shape().dims(), [n, 2]);
    }
    assert!(improvements > 0);
    assert!(stagnations > 0);
}

#[test]
fn refinement_componentwise_metric_scaling_empty_and_invalid_inputs() {
    let huge = Tensor::from_values([1, 1], vec![f64::MAX]).unwrap();
    let b = Tensor::from_values([1], vec![f64::MAX]).unwrap();
    let result = solve_refined(&huge, &b, 2, 1e-14).unwrap();
    assert_eq!(result.solution().storage_slice(), [1.]);
    assert_eq!(result.backward_error(), 0.);
    let a = Tensor::<f64>::zeros([0, 0]).unwrap();
    let b = Tensor::<f64>::zeros([0, isize::MAX as usize]).unwrap();
    let r = solve_refined(&a, &b, 3, 0.).unwrap();
    assert_eq!(r.solution().shape().dims(), [0, isize::MAX as usize]);
    assert_eq!(r.termination(), RefinementTermination::Converged);
    for tol in [-1., 1., f64::NAN, f64::INFINITY] {
        assert!(solve_refined(&a, &b, 3, tol).is_err());
    }
    let bad = Tensor::<f64>::zeros([2, 2]).unwrap();
    assert!(matches!(
        solve_refined(&bad, &Tensor::ones([2]).unwrap(), 1, 1e-12),
        Err(LinalgError::Singular { .. })
    ));
    assert!(solve_refined(&huge, &Tensor::<f64>::ones([2]).unwrap(), 2, 1e-12).is_err());
}

#[test]
fn controlled_condition_dense_family_matches_independent_one_norm() {
    let h = [
        [1., 1., 1., 1.],
        [1., -1., 1., -1.],
        [1., 1., -1., -1.],
        [1., -1., -1., 1.],
    ];
    for condition in [1.0f64, 1e4, 1e8, 1e12] {
        let diagonal = [
            1.,
            condition.powf(-1. / 3.),
            condition.powf(-2. / 3.),
            1. / condition,
        ];
        let mut values = vec![0.; 16];
        for i in 0..4 {
            for j in 0..4 {
                values[i * 4 + j] = (0..4)
                    .map(|k| h[i][k] * diagonal[k] * h[(j + 1) % 4][k] / 4.)
                    .sum();
            }
        }
        let matrix = nalgebra::DMatrix::from_row_slice(4, 4, &values);
        let norm = |a: &nalgebra::DMatrix<f64>| {
            (0..4)
                .map(|j| (0..4).map(|i| a[(i, j)].abs()).sum::<f64>())
                .fold(0., f64::max)
        };
        // Dynamic 4x4 try_inverse uses an unstable closed formula here;
        // use independent pivoted solves plus the known spectral family instead.
        let inverse = matrix
            .clone()
            .lu()
            .solve(&nalgebra::DMatrix::identity(4, 4))
            .unwrap();
        let expected = norm(&matrix) * norm(&inverse);
        let a = Tensor::from_values([4, 4], values).unwrap();
        let actual = condition_number_one(&a).unwrap();
        assert!((actual / condition - 1.).abs() < 2e-4);
        assert!(
            (actual / expected - 1.).abs() < 2e-4,
            "condition={condition}, actual={actual}, oracle={expected}"
        );
        let b = Tensor::<f64>::ones([4]).unwrap();
        let initial = solve_refined(&a, &b, 0, 0.).unwrap();
        let refined = solve_refined(&a, &b, 5, 0.).unwrap();
        assert!(refined.backward_error() <= initial.backward_error());
    }
}
