//! QR correctness across reflector tails, layouts, scales and an independent oracle.
use kernix_core::{Order, shape::Kahen, tensor::Tensor};
use kernix_ops::{
    linalg::{LinalgError, Qr},
    matmul,
};
use nalgebra::{DMatrix, DVector};
use proptest::prelude::*;

fn check_qr(a: &Tensor<f64>, scale: f64) {
    let qr = Qr::factor(a).unwrap();
    let (m, n) = (a.shape().dims()[0], a.shape().dims()[1]);
    let rank = m.min(n);
    let product = matmul(qr.q(), qr.r()).unwrap();
    for row in 0..m {
        for col in 0..n {
            let original = *a.get(&[row, col]).unwrap() / scale;
            let actual = *product.get(&[row, col]).unwrap() / scale;
            assert!(
                (actual - original).abs() < 2e-12 * (1.0 + original.abs()),
                "{m}x{n} ({row},{col}): {actual} vs {original}"
            );
        }
    }
    let gram = matmul(&qr.q().transpose().unwrap(), qr.q()).unwrap();
    for row in 0..rank {
        for col in 0..rank {
            let target = if row == col { 1.0 } else { 0.0 };
            assert!((gram.get(&[row, col]).unwrap() - target).abs() < 2e-12);
        }
    }
}

#[test]
fn reflector_tails_scales_layouts_and_zero_columns() {
    for (m, n) in [
        (0, 0),
        (0, 7),
        (7, 0),
        (1, 1),
        (1, 9),
        (9, 1),
        (5, 3),
        (8, 5),
        (17, 9),
        (32, 17),
        (9, 17),
        (96, 32),
    ] {
        for scale in [1e-200, 1.0, 1e200] {
            let values: Vec<f64> = (0..m * n)
                .map(|i| {
                    (((i * 13 % 47) as f64 - 23.0) / 47.0 + if i / n == i % n { 3.0 } else { 0.0 })
                        * scale
                })
                .collect();
            for order in [Order::C, Order::F] {
                let a =
                    Tensor::from_vec(values.clone(), Kahen::new([m, n]).unwrap(), order).unwrap();
                check_qr(&a, scale);
                // Copy the transposed logical contents for the helper; direct view
                // factorization must agree without assuming contiguous row storage.
                let view = a.transpose().unwrap();
                let expected = view.to_owned_contiguous(Order::C).unwrap();
                let direct = Qr::factor(&view).unwrap();
                let canonical = Qr::factor(&expected).unwrap();
                assert_eq!(direct.q().storage_slice(), canonical.q().storage_slice());
                assert_eq!(direct.r().storage_slice(), canonical.r().storage_slice());
            }
        }
    }
    let zeros = Tensor::full([9, 5], 0.0f64).unwrap();
    check_qr(&zeros, 1.0);
}

#[test]
fn large_reflector_product_uses_representable_overflow_fallback() {
    let a = Tensor::from_rows([[f64::MAX, f64::MAX], [0.0, 0.0]]).unwrap();
    let qr = Qr::factor(&a).unwrap();
    assert_eq!(qr.r().storage_slice(), &[-f64::MAX, -f64::MAX, 0.0, 0.0]);
    check_qr(&a, f64::MAX);
    for bad in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        let a = Tensor::from_rows([[1.0, bad], [2.0, 3.0]]).unwrap();
        assert!(matches!(Qr::factor(&a), Err(LinalgError::NonFinite)));
    }
}

#[test]
fn float32_reconstruction_and_orthogonality() {
    let (m, n) = (37, 13);
    let a = Tensor::from_values(
        [m, n],
        (0..m * n)
            .map(|i| (i * 17 % 53) as f32 / 53.0 - 0.5)
            .collect(),
    )
    .unwrap();
    let qr = Qr::factor(&a).unwrap();
    let product = matmul(qr.q(), qr.r()).unwrap();
    for (&x, &y) in product.storage_slice().iter().zip(a.storage_slice()) {
        assert!((x - y).abs() < 2e-6);
    }
    let gram = matmul(&qr.q().transpose().unwrap(), qr.q()).unwrap();
    for row in 0..n {
        for col in 0..n {
            let expected = if row == col { 1.0 } else { 0.0 };
            assert!((gram.get(&[row, col]).unwrap() - expected).abs() < 2e-6);
        }
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(32))]
    #[test]
    fn seeded_rectangular_solutions_match_nalgebra(m in 2usize..24,n in 1usize..12,seed in 1usize..97) {
        let m=m.max(n);
        let values:Vec<f64>=(0..m*n).map(|i| ((i*seed+13)%101) as f64/101.0-0.5+if i/n==i%n {3.0} else {0.0}).collect();
        let b:Vec<f64>=(0..m).map(|i| ((i*7+seed)%31) as f64/31.0).collect();
        let na = DMatrix::from_row_slice(m,n,&values);
        let nb = DVector::from_vec(b.clone());
        let independent = na.clone().qr();
        let qt_b = independent.q().transpose()*nb;
        let expected = independent.r().solve_upper_triangular(&qt_b).unwrap();
        let a=Tensor::from_values([m,n],values).unwrap();
        check_qr(&a,1.0);
        let x=Qr::factor(&a).unwrap().solve_least_squares(&Tensor::from_values([m],b).unwrap()).unwrap();
        for (&x,&y) in x.storage_slice().iter().zip(expected.iter()) {
            prop_assert!((x-y).abs() < 2e-12*(1.0+y.abs()));
        }
    }
}
