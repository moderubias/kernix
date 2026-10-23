//! Independent analytic SPD families and block-boundary validation.
use kernix_core::{Order, shape::Kahen, tensor::Tensor};
use kernix_ops::linalg::{Cholesky, LinalgError};

fn spd(n: usize, condition: f64, scale: f64) -> Tensor<f64> {
    let delta = n as f64 / (condition - 1.);
    Tensor::from_values(
        [n, n],
        (0..n * n)
            .map(|p| {
                let (i, j) = (p / n, p % n);
                let u = if (i + j) % 2 == 0 { 1. } else { -1. };
                (u + if i == j { delta } else { 0. }) * scale
            })
            .collect(),
    )
    .unwrap()
}
#[test]
fn blocks_match_closed_form_and_independent_reconstruction() {
    for n in [127usize, 128, 191, 192, 193, 255, 256, 257, 383, 512] {
        for condition in [2., 1e4, 1e8, 1e12] {
            let a = spd(n, condition, 1.);
            let saved = a.storage_slice().to_vec();
            let ch = Cholesky::factor(&a).unwrap();
            let l = ch.lower().storage_slice();
            let d = n as f64 / (condition - 1.);
            for i in 0..n {
                for j in 0..n {
                    let expected = if j > i {
                        0.
                    } else if i == j {
                        (d * (d + j as f64 + 1.) / (d + j as f64)).sqrt()
                    } else {
                        let sign = if (i + j) % 2 == 0 { 1. } else { -1. };
                        sign * d.sqrt() / ((d + j as f64) * (d + j as f64 + 1.)).sqrt()
                    };
                    assert!(
                        (l[i * n + j] - expected).abs() < 2e-7 * (1. + expected.abs()),
                        "n={n} kappa={condition} ({i},{j})"
                    );
                }
            }
            for i in [0, 15, 16, 31, 32, n - 1] {
                for j in [0, 15, 16, 31, 32, n - 1] {
                    let product: f64 = (0..=i.min(j)).map(|k| l[i * n + k] * l[j * n + k]).sum();
                    assert!(
                        (product - saved[i * n + j]).abs() < 3e-12 * (1. + saved[i * n + j].abs()),
                        "n={n} kappa={condition}"
                    );
                }
            }
            assert_eq!(a.storage_slice(), saved);
        }
    }
}
#[test]
fn block_scaled_f32_f64_and_arbitrary_layouts() {
    let n = 193;
    for scale in [1e-300, 1., 1e300] {
        let a = spd(n, 1e4, scale);
        let ch = Cholesky::factor(&a).unwrap();
        let d = n as f64 / (1e4 - 1.);
        let expected = (d * (d + n as f64) / (d + n as f64 - 1.)).sqrt();
        assert!((ch.lower().storage_slice()[n * n - 1] / scale.sqrt() - expected).abs() < 1e-9);
    }
    for scale in [1e-20f32, 1., 1e20] {
        let a = spd(n, 1e3, 1.);
        let f = Tensor::from_values(
            [n, n],
            a.storage_slice()
                .iter()
                .map(|&v| v as f32 * scale)
                .collect(),
        )
        .unwrap();
        let ch = Cholesky::factor(&f).unwrap();
        let l = ch.lower().storage_slice();
        for (i, j) in [(0, 0), (16, 16), (191, 192), (192, 192)] {
            let r: f64 = (0..=i.min(j))
                .map(|k| {
                    f64::from(l[i * n + k] / scale.sqrt()) * f64::from(l[j * n + k] / scale.sqrt())
                })
                .sum();
            assert!((r - a.storage_slice()[i * n + j]).abs() < 2e-4);
        }
    }
    let a = spd(n, 1e4, 1.);
    let f = Tensor::from_fn(Kahen::new([n, n]).unwrap(), Order::F, 8, |c| {
        a.storage_slice()[c[0] * n + c[1]]
    })
    .unwrap();
    let expected = Cholesky::factor(&a).unwrap();
    let other = Cholesky::factor(&f).unwrap();
    assert_eq!(
        expected.lower().storage_slice(),
        other.lower().storage_slice()
    );
    let reversed = f.reverse_axis(0).unwrap();
    let reversed = reversed.reverse_axis(1).unwrap();
    assert!(Cholesky::factor(&reversed).is_ok());
}
#[test]
fn validation_precedes_block_work_and_pivot_indices_remain_global() {
    let n = 257;
    for pivot in [0, 15, 16, 191, 192, 193, 256] {
        let mut a = Tensor::<f64>::zeros([n, n]).unwrap();
        for i in 0..n {
            a.storage_slice_mut()[i * n + i] = if i == pivot { -1. } else { 1. };
        }
        assert!(
            matches!(Cholesky::factor(&a),Err(LinalgError::NotPositiveDefinite{pivot:p}) if p==pivot)
        );
    }
    let mut a = spd(n, 2., 1.);
    a.storage_slice_mut()[n - 1] = 100.;
    assert!(matches!(
        Cholesky::factor(&a),
        Err(LinalgError::NotSymmetric {
            row: 256,
            column: 0
        })
    ));
    for value in [f64::NAN, f64::INFINITY] {
        let mut a = spd(n, 2., 1.);
        a.storage_slice_mut()[n - 1] = value;
        assert!(matches!(Cholesky::factor(&a), Err(LinalgError::NonFinite)));
    }
}
