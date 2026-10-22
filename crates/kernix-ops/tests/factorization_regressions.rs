//! Factorization ordering, geometry, scales and memory ownership regressions.
use kernix_core::{Order, tensor::Tensor};
use kernix_ops::{
    linalg::{Cholesky, LinalgError, Qr},
    matmul_reference,
};

fn spd(n: usize, scale: f64) -> Tensor<f64> {
    Tensor::from_values(
        [n, n],
        (0..n * n)
            .map(|i| {
                let (r, c) = (i / n, i % n);
                scale
                    * (if r == c {
                        4.0
                    } else {
                        0.1 / (1 + r.abs_diff(c)) as f64
                    })
            })
            .collect(),
    )
    .unwrap()
}
fn ordered(a: &[f64], n: usize) -> Vec<f64> {
    let mut l = vec![0.; a.len()];
    for i in 0..n {
        for j in 0..=i {
            let mut v = a[i * n + j];
            for k in 0..j {
                v -= l[i * n + k] * l[j * n + k];
            }
            l[i * n + j] = if i == j { v.sqrt() } else { v / l[j * n + j] };
        }
    }
    l
}
#[test]
fn cholesky_layouts_scales_branch_boundaries_and_input_ownership() {
    for n in [0, 1, 7, 31, 64, 96, 97, 128] {
        for scale in [1e-200, 1., 1e200] {
            let a = spd(n, scale);
            let original = a.storage_slice().to_vec();
            let factor = Cholesky::factor(&a).unwrap();
            assert_eq!(a.storage_slice(), original);
            if n <= 96 {
                assert_eq!(factor.lower().storage_slice(), ordered(&original, n));
            }
            let product =
                matmul_reference(factor.lower(), &factor.lower().transpose().unwrap()).unwrap();
            for (&x, &y) in product.storage_slice().iter().zip(&original) {
                assert!(((x - y) / scale).abs() < 1e-12);
            }
            for i in 0..n {
                for j in i + 1..n {
                    assert_eq!(factor.lower().storage_slice()[i * n + j], 0.);
                }
            }
        }
    }
    let a = spd(13, 1.);
    let f = a.to_owned_contiguous(Order::F).unwrap();
    let reverse = a.reverse_axis(0).unwrap().reverse_axis(1).unwrap();
    assert_eq!(
        Cholesky::factor(&f).unwrap().lower().storage_slice(),
        Cholesky::factor(&a).unwrap().lower().storage_slice()
    );
    assert!(Cholesky::factor(&reverse).is_ok());
}

#[test]
fn cholesky_f32_reconstruction_and_rejection() {
    for n in [1, 31, 64, 96, 97, 128] {
        let a = spd(n, 1.);
        let a = Tensor::from_values(
            [n, n],
            a.storage_slice().iter().map(|x| *x as f32).collect(),
        )
        .unwrap();
        let l = Cholesky::factor(&a).unwrap();
        let product = matmul_reference(l.lower(), &l.lower().transpose().unwrap()).unwrap();
        for (&x, &y) in product.storage_slice().iter().zip(a.storage_slice()) {
            assert!((x - y).abs() < 1e-5 * (1. + y.abs()));
        }
    }
    for bad in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        let a = Tensor::from_values([2, 2], vec![1., bad, 0., 1.]).unwrap();
        assert!(matches!(Cholesky::factor(&a), Err(LinalgError::NonFinite)));
    }
    for d in [0., -1.] {
        let a = Tensor::from_values([2, 2], vec![1., 0., 0., d]).unwrap();
        assert!(matches!(
            Cholesky::factor(&a),
            Err(LinalgError::NotPositiveDefinite { pivot: 1 })
        ));
    }
}

#[test]
fn empty_qr_huge_dimensions_do_not_iterate_over_empty_axes() {
    for dims in [[0, isize::MAX as usize], [isize::MAX as usize, 0], [0, 0]] {
        let a = Tensor::<f64>::zeros(dims).unwrap();
        let q = Qr::factor(&a).unwrap();
        assert_eq!(q.q().shape().dims(), [dims[0], 0]);
        assert_eq!(q.r().shape().dims(), [0, dims[1]]);
        assert_eq!(q.q().numel(), 0);
        assert_eq!(q.r().numel(), 0);
    }
}

#[test]
fn qr_rejects_malformed_capability_and_impossible_shape_before_access() {
    struct Invalid {
        dims: [usize; 2],
    }
    impl kernix_ops::TensorInput<f64> for Invalid {
        fn dims(&self) -> &[usize] {
            &self.dims
        }
        fn get(&self, _: &[usize]) -> Result<&f64, kernix_core::tensor::TensorError> {
            panic!("invalid capability must be rejected before indexed access")
        }
        fn as_c_contiguous_slice(&self) -> Option<&[f64]> {
            Some(&[1.])
        }
    }
    for dims in [[0, 1], [2, 2], [usize::MAX, 2]] {
        assert!(Qr::factor(&Invalid { dims }).is_err());
    }
}
