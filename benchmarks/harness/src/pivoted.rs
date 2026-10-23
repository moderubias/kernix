//! Pivoted QR and rank-deficient minimum-norm problem comparisons.
use super::{measure, qr_error};
use kernix::Tensor;
use serde_json::{Value, json};

pub fn factor(mode: &str, library: &str, m: usize, n: usize) -> Value {
    let values: Vec<f64> = (0..m * n)
        .map(|i| ((i * 13 % 47) as f64 - 23.) / 47. + if i / n == i % n { 3. } else { 0. })
        .collect();
    let check = |q: &dyn Fn(usize, usize) -> f64, r: &dyn Fn(usize, usize) -> f64, p: &[usize]| {
        let mut sorted = p.to_vec();
        sorted.sort_unstable();
        assert_eq!(sorted, (0..n).collect::<Vec<_>>());
        let permuted: Vec<f64> = (0..m * n).map(|i| values[(i / n) * n + p[i % n]]).collect();
        qr_error(m, n, &permuted, q, r)
    };
    match library {
        "kernix" => {
            let a = Tensor::from_values([m, n], values.clone()).unwrap();
            measure(
                mode,
                || kernix::PivotedQr::factor(&a).unwrap(),
                |out| {
                    check(
                        &|i, j| out.q().storage_slice()[i * n + j],
                        &|i, j| out.r().storage_slice()[i * n + j],
                        out.permutation(),
                    )
                },
            )
        }
        "nalgebra" => {
            let a = nalgebra::DMatrix::from_row_slice(m, n, &values);
            measure(
                mode,
                || {
                    let qr = a.clone().col_piv_qr();
                    let mut indices =
                        nalgebra::DMatrix::from_row_slice(1, n, &(0..n).collect::<Vec<_>>());
                    qr.p().permute_columns(&mut indices);
                    (qr.q(), qr.r(), indices)
                },
                |out| {
                    check(
                        &|i, j| out.0[(i, j)],
                        &|i, j| out.1[(i, j)],
                        out.2.as_slice(),
                    )
                },
            )
        }
        "faer" => {
            let a = faer::MatRef::from_row_major_slice(&values, m, n);
            measure(
                mode,
                || {
                    let qr = a.col_piv_qr();
                    (
                        qr.compute_thin_Q(),
                        qr.thin_R().to_owned(),
                        qr.P().arrays().0.to_vec(),
                    )
                },
                |out| check(&|i, j| out.0[(i, j)], &|i, j| out.1[(i, j)], &out.2),
            )
        }
        _ => panic!("unsupported pivoted QR comparator"),
    }
}

fn fixture(m: usize, n: usize, r: usize) -> Vec<f64> {
    let u: Vec<f64> = (0..m * r)
        .map(|v| {
            let (i, k) = (v / r, v % r);
            (if i == k { 3. } else { 0. }) + ((i * 7 + k * 3) % 11) as f64 / 16. - 5. / 16.
        })
        .collect();
    (0..m * n)
        .map(|v| {
            let (i, j) = (v / n, v % n);
            if j < r {
                u[i * r + j]
            } else {
                let k = (j - r) % r;
                u[i * r + k] + 0.5 * u[i * r + (k + 1) % r]
            }
        })
        .collect()
}
fn solve_error(
    m: usize,
    n: usize,
    r: usize,
    a: &[f64],
    b: &[f64],
    get: impl Fn(usize, usize) -> f64,
) -> Value {
    // Analytic null vectors of [U,U*C], plus least-squares stationarity.
    let mut null_error = 0f64;
    let mut normal_error = 0f64;
    for j in 0..4 {
        for free in r..n {
            let k = (free - r) % r;
            null_error =
                null_error.max((get(free, j) - get(k, j) - 0.5 * get((k + 1) % r, j)).abs());
        }
        let residual: Vec<f64> = (0..m)
            .map(|i| (0..n).map(|k| a[i * n + k] * get(k, j)).sum::<f64>() - b[i * 4 + j])
            .collect();
        for col in 0..n {
            normal_error = normal_error.max(
                (0..m)
                    .map(|i| a[i * n + col] * residual[i])
                    .sum::<f64>()
                    .abs(),
            );
        }
    }
    assert!(
        null_error < 2e-10 && normal_error < 2e-10,
        "null={null_error}, normal={normal_error}"
    );
    json!({"null_space_orthogonality_max_absolute_error":null_error,"normal_residual_max_absolute_error":normal_error,"budget":2e-10})
}
pub fn minimum_norm(mode: &str, library: &str, m: usize, n: usize, r: usize) -> Value {
    let values = fixture(m, n, r);
    let b: Vec<f64> = (0..m * 4)
        .map(|i| ((i * 11 % 29) as f64 - 14.) / 8.)
        .collect();
    match library {
        "kernix" => {
            let a = Tensor::from_values([m, n], values.clone()).unwrap();
            let rhs = Tensor::from_values([m, 4], b.clone()).unwrap();
            measure(
                mode,
                || {
                    kernix::PivotedQr::factor_with_tolerance(&a, 1e-12)
                        .unwrap()
                        .solve_least_squares(&rhs)
                        .unwrap()
                },
                |out| solve_error(m, n, r, &values, &b, |i, j| out.storage_slice()[i * 4 + j]),
            )
        }
        "nalgebra" => {
            let a = nalgebra::DMatrix::from_row_slice(m, n, &values);
            let rhs = nalgebra::DMatrix::from_row_slice(m, 4, &b);
            measure(
                mode,
                || a.clone().svd(true, true).solve(&rhs, 1e-10).unwrap(),
                |out| solve_error(m, n, r, &values, &b, |i, j| out[(i, j)]),
            )
        }
        _ => panic!("unsupported minimum-norm comparator"),
    }
}
#[cfg(test)]
mod tests {
    #[test]
    #[cfg(not(feature = "allocation-counting"))]
    fn independent_pivoted_workers_validate_permutation_and_minimum_norm() {
        faer::set_global_parallelism(faer::Par::Seq);
        for library in ["kernix", "nalgebra", "faer"] {
            super::factor("first", library, 17, 7);
        }
        for library in ["kernix", "nalgebra"] {
            super::minimum_norm("first", library, 64, 32, 16);
            super::minimum_norm("first", library, 32, 64, 32);
        }
    }
    #[test]
    #[should_panic(expected = "normal=")]
    #[cfg(not(feature = "allocation-counting"))]
    fn incorrect_finite_solution_cannot_receive_a_benchmark_score() {
        let a = super::fixture(12, 8, 4);
        super::solve_error(12, 8, 4, &a, &[1.; 48], |_, _| 0.);
    }
}
