//! Prepared-factor solves and owned rank-one modifications; setup is outside timing.
use super::{chol_error, measure};
use faer::prelude::*;
use kernix::{Cholesky, Lu, Tensor};
use serde_json::{Value, json};

pub(super) fn solve(mode: &str, library: &str, n: usize, columns: usize, lu: bool) -> Value {
    let values: Vec<f64> = (0..n * n)
        .map(|p| {
            if p / n == p % n {
                4.0
            } else if (p / n).abs_diff(p % n) == 1 {
                0.125
            } else {
                0.0
            }
        })
        .collect();
    let expected: Vec<f64> = (0..n * columns)
        .map(|p| ((p * 7 % 23) as f64 - 11.0) / 16.0)
        .collect();
    let rhs: Vec<f64> = (0..n * columns)
        .map(|p| {
            let i = p / columns;
            let j = p % columns;
            4.0 * expected[p]
                + if i > 0 {
                    0.125 * expected[(i - 1) * columns + j]
                } else {
                    0.0
                }
                + if i + 1 < n {
                    0.125 * expected[(i + 1) * columns + j]
                } else {
                    0.0
                }
        })
        .collect();
    let check = |get: &dyn Fn(usize, usize) -> f64| {
        let mut error = 0.0f64;
        for i in 0..n {
            for j in 0..columns {
                error = error.max((get(i, j) - expected[i * columns + j]).abs());
            }
        }
        assert!(error < 2e-11);
        json!({"known_solution_max_absolute_error":error,"budget":2e-11,"prepared_factor":true,"rhs_columns":columns})
    };
    match library {
        "kernix" => {
            let a = Tensor::from_values([n, n], values).unwrap();
            let b = Tensor::from_values([n, columns], rhs).unwrap();
            if lu {
                let factor = Lu::factor(&a).unwrap();
                measure(
                    mode,
                    || factor.solve(&b).unwrap(),
                    |out| check(&|i, j| out.storage_slice()[i * columns + j]),
                )
            } else {
                let factor = Cholesky::factor(&a).unwrap();
                measure(
                    mode,
                    || factor.solve(&b).unwrap(),
                    |out| check(&|i, j| out.storage_slice()[i * columns + j]),
                )
            }
        }
        "nalgebra" => {
            let a = nalgebra::DMatrix::from_row_slice(n, n, &values);
            let b = nalgebra::DMatrix::from_row_slice(n, columns, &rhs);
            if lu {
                let factor = a.lu();
                measure(
                    mode,
                    || factor.solve(&b).unwrap(),
                    |out| check(&|i, j| out[(i, j)]),
                )
            } else {
                let factor = a.cholesky().unwrap();
                measure(mode, || factor.solve(&b), |out| check(&|i, j| out[(i, j)]))
            }
        }
        "faer" => {
            let a = faer::MatRef::from_row_major_slice(&values, n, n);
            let b = faer::MatRef::from_row_major_slice(&rhs, n, columns);
            if lu {
                let factor = a.partial_piv_lu();
                measure(mode, || factor.solve(b), |out| check(&|i, j| out[(i, j)]))
            } else {
                let factor = a.llt(faer::Side::Lower).unwrap();
                measure(mode, || factor.solve(b), |out| check(&|i, j| out[(i, j)]))
            }
        }
        _ => panic!("unsupported factor-solve library"),
    }
}

pub(super) fn modification(mode: &str, library: &str, n: usize, subtract: bool) -> Value {
    let values: Vec<f64> = (0..n * n)
        .map(|p| {
            0.01 / (1.0 + (p / n).abs_diff(p % n) as f64) + if p / n == p % n { 4.0 } else { 0.0 }
        })
        .collect();
    let vector: Vec<f64> = (0..n).map(|i| ((i % 7) as f64 - 2.0) / 128.0).collect();
    let modified: Vec<f64> = (0..n * n)
        .map(|p| {
            values[p]
                + if subtract {
                    -vector[p / n] * vector[p % n]
                } else {
                    vector[p / n] * vector[p % n]
                }
        })
        .collect();
    match library {
        "kernix" => {
            #[cfg(feature = "legacy-rank-updates")]
            panic!("registry baseline has no rank-one modification API; worker must be omitted");
            #[cfg(not(feature = "legacy-rank-updates"))]
            {
                let factor =
                    Cholesky::factor(&Tensor::from_values([n, n], values).unwrap()).unwrap();
                let v = Tensor::from_values([n], vector).unwrap();
                measure(
                    mode,
                    || {
                        if subtract {
                            factor.rank_one_downdate(&v).unwrap()
                        } else {
                            factor.rank_one_update(&v).unwrap()
                        }
                    },
                    |out| chol_error(n, &modified, |i, j| out.lower().storage_slice()[i * n + j]),
                )
            }
        }
        "refactor" => {
            let a = Tensor::from_values([n, n], modified.clone()).unwrap();
            measure(
                mode,
                || Cholesky::factor(&a).unwrap(),
                |out| chol_error(n, &modified, |i, j| out.lower().storage_slice()[i * n + j]),
            )
        }
        "nalgebra" => {
            let factor = nalgebra::DMatrix::from_row_slice(n, n, &values)
                .cholesky()
                .unwrap();
            let v = nalgebra::DVector::from_vec(vector);
            measure(
                mode,
                || {
                    let mut out = factor.clone();
                    out.rank_one_update(&v, if subtract { -1.0 } else { 1.0 });
                    out
                },
                |out| {
                    let l = out.l();
                    chol_error(n, &modified, |i, j| l[(i, j)])
                },
            )
        }
        _ => panic!("unsupported rank-one modification comparator"),
    }
}

pub(super) fn transpose(mode: &str, single: bool) -> Value {
    macro_rules! run{($ty:ty)=>{{
        let n=128;let av:Vec<$ty>=(0..n*n).map(|p|(p%23)as $ty/16.-0.5).collect();let bv:Vec<$ty>=(0..n*n).map(|p|(p%13)as $ty/8.-0.5).collect();
        let expected:Vec<f64>=(0..n*n).map(|p|(0..n).map(|k|f64::from(av[k*n+p/n])*f64::from(bv[k*n+p%n])).sum()).collect();
        let a=Tensor::from_values([n,n],av).unwrap();let a=a.transpose().unwrap();let b=Tensor::from_values([n,n],bv).unwrap();
        measure(mode,||kernix::matmul(&a,&b).unwrap(),|out|{let error=out.storage_slice().iter().zip(&expected).map(|(&v,&x)|(f64::from(v)-x).abs()).fold(0.0,f64::max);assert_eq!(error,0.0);json!({"coordinate_max_absolute_error":error,"budget":0.0,"borrowed_transpose":true})})
    }}}
    if single { run!(f32) } else { run!(f64) }
}

#[cfg(all(
    test,
    not(feature = "allocation-counting"),
    not(feature = "legacy-rank-updates")
))]
mod tests {
    #[test]
    fn extended_workers_validate_all_outputs_before_timing() {
        for library in ["kernix", "nalgebra", "faer"] {
            super::solve("first", library, 16, 8, false);
            super::solve("first", library, 16, 8, true);
        }
        for library in ["kernix", "nalgebra", "refactor"] {
            super::modification("first", library, 17, false);
            super::modification("first", library, 17, true);
        }
        super::transpose("first", false);
        super::transpose("first", true);
    }
}
