#![no_main]
use kernix::{
    Tensor,
    linalg::{Cholesky, PivotedQr, Qr, reciprocal_condition_number_one, solve_refined},
};
use libfuzzer_sys::fuzz_target;
fuzz_target!(|bytes: &[u8]| {
    if bytes.len() < 2 {
        return;
    }
    let (m, n) = ((bytes[0] % 9) as usize, (bytes[1] % 9) as usize);
    let values: Vec<f64> = (0..m * n)
        .map(|i| (bytes.get(i + 2).copied().unwrap_or(128) as f64 - 128.) / 16.)
        .collect();
    let a = Tensor::from_values([m, n], values.clone()).unwrap();
    let qr = Qr::factor(&a).unwrap();
    let rank = m.min(n);
    let (q, r) = (qr.q().storage_slice(), qr.r().storage_slice());
    for i in 0..m {
        for j in 0..n {
            let product: f64 = (0..rank).map(|k| q[i * rank + k] * r[k * n + j]).sum();
            assert!((product - values[i * n + j]).abs() < 1e-10 * (1. + values[i * n + j].abs()));
        }
    }
    for i in 0..rank {
        for j in 0..rank {
            let product: f64 = (0..m).map(|k| q[k * rank + i] * q[k * rank + j]).sum();
            assert!((product - if i == j { 1. } else { 0. }).abs() < 1e-10);
        }
    }
    let pivoted = PivotedQr::factor_with_tolerance(&a, 1e-10).unwrap();
    for i in 0..m {
        for j in 0..n {
            let product: f64 = (0..rank)
                .map(|k| {
                    pivoted.q().storage_slice()[i * rank + k]
                        * pivoted.r().storage_slice()[k * n + j]
                })
                .sum();
            assert!((product - values[i * n + pivoted.permutation()[j]]).abs() < 1e-9);
        }
    }
    let b = Tensor::<f64>::ones([m]).unwrap();
    let minimum = pivoted.solve_least_squares(&b).unwrap();
    let basic = pivoted.solve_basic_least_squares(&b).unwrap();
    let norm = |x: &Tensor<f64>| x.storage_slice().iter().map(|x| x * x).sum::<f64>();
    assert!(norm(&minimum) <= norm(&basic) * (1. + 1e-6) + 1e-20);
    // B B^T + I is positive definite, independent of rectangular B's rank.
    let mut spd = vec![0.; m * m];
    for i in 0..m {
        for j in 0..m {
            spd[i * m + j] = (0..n)
                .map(|k| values[i * n + k] * values[j * n + k])
                .sum::<f64>()
                + if i == j { 1. } else { 0. };
        }
    }
    let a = Tensor::from_values([m, m], spd).unwrap();
    let ch = Cholesky::factor(&a).unwrap();
    let l = ch.lower().storage_slice();
    for i in 0..m {
        for j in 0..m {
            let product: f64 = (0..=i.min(j)).map(|k| l[i * m + k] * l[j * m + k]).sum();
            assert!(
                (product - a.storage_slice()[i * m + j]).abs()
                    < 1e-10 * (1. + a.storage_slice()[i * m + j].abs())
            );
        }
    }
    let rcond = reciprocal_condition_number_one(&a).unwrap();
    assert!(rcond > 0. && rcond <= 1.);
    let b = Tensor::<f64>::ones([m]).unwrap();
    let initial = solve_refined(&a, &b, 0, 0.).unwrap();
    let refined = solve_refined(&a, &b, 3, 0.).unwrap();
    assert!(refined.backward_error() <= initial.backward_error());
});
