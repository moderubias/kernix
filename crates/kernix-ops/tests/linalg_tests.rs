//! Decomposition reconstruction, orthogonality, residuals and singularity contracts.
use kernix_core::{Order, shape::Kahen, tensor::Tensor};
use kernix_ops::{l2_norm_all, linalg::*, matmul, sub};
fn tensor(dims: &[usize], v: &[f64]) -> Tensor<f64> {
    Tensor::from_vec(v.to_vec(), Kahen::new(dims).unwrap(), Order::C).unwrap()
}
fn near(a: f64, b: f64) {
    assert!(
        (a - b).abs() <= 2e-12 * (1.0 + a.abs() + b.abs()),
        "{a} vs {b}"
    );
}
#[test]
fn pivoted_lu_solves_reconstructs_and_inverts() {
    let a = tensor(&[3, 3], &[0.0, 2.0, 1.0, 1.0, 1.0, 0.0, 2.0, 0.0, 1.0]);
    let b = tensor(&[3], &[7.0, 3.0, 5.0]);
    let lu = Lu::factor(&a).unwrap();
    let x = lu.solve(&b).unwrap();
    for (&v, e) in x.storage_slice().iter().zip([1.0, 2.0, 3.0]) {
        near(v, e);
    }
    assert_eq!(lu.permutation()[0], 2);
    near(lu.determinant().unwrap(), -4.0);
    let mut l = vec![0.0; 9];
    let mut u = vec![0.0; 9];
    let data = lu.packed().storage_slice();
    for i in 0..3 {
        for j in 0..3 {
            if i == j {
                l[i * 3 + j] = 1.0;
            } else if i > j {
                l[i * 3 + j] = data[i * 3 + j];
            }
            if i <= j {
                u[i * 3 + j] = data[i * 3 + j];
            }
        }
    }
    let product = matmul(&tensor(&[3, 3], &l), &tensor(&[3, 3], &u)).unwrap();
    for i in 0..3 {
        for j in 0..3 {
            near(
                product.storage_slice()[i * 3 + j],
                a.storage_slice()[lu.permutation()[i] * 3 + j],
            );
        }
    }
    let identity = matmul(&a, &inverse(&a).unwrap()).unwrap();
    for i in 0..3 {
        for j in 0..3 {
            near(
                identity.storage_slice()[i * 3 + j],
                if i == j { 1.0 } else { 0.0 },
            );
        }
    }
    near(residual_norm(&a, &x, &b).unwrap(), 0.0);
}
#[test]
fn multiple_rhs_and_scale_aware_singularity_policy() {
    let a = tensor(&[2, 2], &[4.0, 1.0, 1.0, 3.0]);
    let b = eye::<f64>(2).unwrap();
    let x = solve(&a, &b).unwrap();
    let product = matmul(&a, &x).unwrap();
    for (&x, &y) in product.storage_slice().iter().zip(b.storage_slice()) {
        near(x, y);
    }
    let tiny = tensor(&[2, 2], &[4e-200, 1e-200, 1e-200, 3e-200]);
    let rhs = tensor(&[2], &[1e-200, 2e-200]);
    let x = solve(&tiny, &rhs).unwrap();
    near(x.storage_slice()[0], 1.0 / 11.0);
    let ill = tensor(&[2, 2], &[1.0, 0.0, 0.0, f64::EPSILON / 4.0]);
    assert!(matches!(
        Lu::factor(&ill),
        Err(LinalgError::Singular { pivot: 1 })
    ));
    assert!(Lu::factor_with_tolerance(&ill, 0.0).is_ok());
    assert!(Lu::factor_with_tolerance(&ill, -1.0).is_err());
}
#[test]
fn cholesky_factor_solve_symmetry_and_nonpositive_errors() {
    let a = tensor(&[2, 2], &[4.0, 1.0, 1.0, 3.0]);
    let b = tensor(&[2], &[1.0, 2.0]);
    let c = Cholesky::factor(&a).unwrap();
    let reconstructed = matmul(c.lower(), &c.lower().transpose().unwrap()).unwrap();
    for (&x, &y) in reconstructed.storage_slice().iter().zip(a.storage_slice()) {
        near(x, y);
    }
    let x = c.solve(&b).unwrap();
    near(residual_norm(&a, &x, &b).unwrap(), 0.0);
    assert!(matches!(
        Cholesky::factor(&tensor(&[2, 2], &[1.0, 2.0, 2.0, 1.0])),
        Err(LinalgError::NotPositiveDefinite { pivot: 1 })
    ));
    assert!(matches!(
        Cholesky::factor(&tensor(&[2, 2], &[1.0, 2.0, 3.0, 4.0])),
        Err(LinalgError::NotSymmetric { .. })
    ));
}
#[test]
fn rectangular_qr_reconstruction_orthogonality_and_least_squares() {
    let a = tensor(&[3, 2], &[1.0, 0.0, 1.0, 1.0, 1.0, 2.0]);
    let qr = Qr::factor(&a).unwrap();
    let product = matmul(qr.q(), qr.r()).unwrap();
    for (&x, &y) in product.storage_slice().iter().zip(a.storage_slice()) {
        near(x, y);
    }
    let orthogonal = matmul(&qr.q().transpose().unwrap(), qr.q()).unwrap();
    for (&x, &y) in orthogonal
        .storage_slice()
        .iter()
        .zip([1.0, 0.0, 0.0, 1.0].iter())
    {
        near(x, y);
    }
    let b = tensor(&[3], &[1.0, 2.0, 2.0]);
    let x = least_squares(&a, &b).unwrap();
    near(x.storage_slice()[0], 7.0 / 6.0);
    near(x.storage_slice()[1], 0.5);
    near(residual_norm(&a, &x, &b).unwrap(), (1.0 / 6.0_f64).sqrt());
    let residual = sub(&matmul(&a, &x).unwrap(), &b).unwrap();
    let normal = matmul(&a.transpose().unwrap(), &residual).unwrap();
    near(l2_norm_all(&normal).unwrap(), 0.0);
    let wide = tensor(&[2, 3], &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    let q = Qr::factor(&wide).unwrap();
    assert_eq!(q.q().shape().dims(), &[2, 2]);
    let reconstructed = matmul(q.q(), q.r()).unwrap();
    for (&x, &y) in reconstructed
        .storage_slice()
        .iter()
        .zip(wide.storage_slice())
    {
        near(x, y);
    }
    assert!(q.solve_least_squares(&tensor(&[2], &[1.0, 2.0])).is_err());
}
#[test]
fn empty_nonfinite_singular_and_large_logdet() {
    let zero = tensor(&[0, 0], &[]);
    assert_eq!(Lu::factor(&zero).unwrap().determinant().unwrap(), 1.0);
    assert_eq!(Cholesky::factor(&zero).unwrap().lower().numel(), 0);
    assert_eq!(Qr::factor(&zero).unwrap().q().numel(), 0);
    let singular = tensor(&[2, 2], &[1.0, 2.0, 2.0, 4.0]);
    assert_eq!(determinant(&singular).unwrap(), 0.0);
    assert!(Lu::factor(&singular).is_err());
    assert!(
        Qr::factor(&singular)
            .unwrap()
            .solve_least_squares(&tensor(&[2], &[1.0, 2.0]))
            .is_err()
    );
    assert!(Lu::factor(&tensor(&[1, 1], &[f64::NAN])).is_err());
    assert!(Lu::factor(&tensor(&[2, 1], &[1.0, 2.0])).is_err());
    let large = tensor(&[2, 2], &[1e300, 0.0, 0.0, 1e300]);
    let lu = Lu::factor(&large).unwrap();
    assert!(lu.determinant().is_err());
    let (sign, log) = lu.slogdet();
    assert_eq!(sign, 1.0);
    near(log, 2.0 * 1e300_f64.ln());
    let extreme = tensor(&[2, 1], &[f64::MAX, 0.0]);
    assert!(Qr::factor(&extreme).is_ok());
}
#[test]
fn strided_matrix_and_float32_solve() {
    let a = tensor(&[2, 2], &[4.0, 1.0, 1.0, 3.0]);
    let transposed = a.transpose().unwrap();
    near(
        Lu::factor(&transposed).unwrap().determinant().unwrap(),
        11.0,
    );
    let a = Tensor::from_vec(
        vec![4.0f32, 1.0, 1.0, 3.0],
        Kahen::new([2, 2]).unwrap(),
        Order::F,
    )
    .unwrap();
    let b = Tensor::from_vec(vec![1.0f32, 2.0], Kahen::new([2]).unwrap(), Order::C).unwrap();
    let x = solve(&a, &b).unwrap();
    assert!(residual_norm(&a, &x, &b).unwrap() < 2e-6);
}
