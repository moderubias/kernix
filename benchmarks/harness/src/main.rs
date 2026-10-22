//! Reproducible CPU measurements; not linked into any published Kernix package.
#![deny(unsafe_op_in_unsafe_fn)]
#[cfg(all(test, not(feature = "legacy-autograd")))]
mod gradient_checks;
mod profile;
#[cfg(not(feature = "legacy-autograd"))]
mod training;
use kernix::{
    Tensor,
    autograd::Tape,
    linalg::{Cholesky, Qr},
};
use serde_json::{Value, json};
use std::{
    hint::black_box,
    time::{Duration, Instant},
};

const CALLS: usize = 256;
fn quantile(sorted: &[f64], numerator: usize, denominator: usize) -> f64 {
    assert!(!sorted.is_empty());
    sorted[(sorted.len() * numerator)
        .div_ceil(denominator)
        .saturating_sub(1)]
}
fn distribution(samples: Vec<f64>) -> Value {
    let mut sorted = samples.clone();
    sorted.sort_by(f64::total_cmp);
    json!({"p50_ns":quantile(&sorted,50,100),"p95_ns":quantile(&sorted,95,100),"p99_ns":quantile(&sorted,99,100),
        "max_ns":sorted.last(),"samples_ns":samples})
}
fn peak_rss() -> Option<u64> {
    let status = std::fs::read_to_string("/proc/self/status").ok()?;
    let line = status.lines().find(|x| x.starts_with("VmHWM:"))?;
    Some(line.split_whitespace().nth(1)?.parse::<u64>().ok()? * 1024)
}
fn measure<R>(
    mode: &str,
    mut operation: impl FnMut() -> R,
    check: impl FnOnce(&R) -> Value,
) -> Value {
    assert!(
        mode == "resources" || !cfg!(feature = "allocation-counting"),
        "timing must use the uninstrumented binary"
    );
    let start = Instant::now();
    let first = black_box(operation());
    let first_ns = start.elapsed().as_secs_f64() * 1e9;
    let accuracy = check(&first);
    drop(first);
    if mode == "first" {
        return json!({"first_call_return_ns":first_ns,"accuracy":accuracy});
    }
    let warmup_start = Instant::now();
    let mut warmup_calls = 0usize;
    while warmup_calls < 8 || warmup_start.elapsed() < Duration::from_millis(10) {
        drop(black_box(operation()));
        warmup_calls += 1;
    }
    let warmup_ns = warmup_start.elapsed().as_nanos();
    if mode == "resources" {
        let allocations = profile::count(|| drop(black_box(operation())));
        return json!({"warmup_calls":warmup_calls,"warmup_ns":warmup_ns,"rust_allocator":allocations,"process_peak_rss_bytes":peak_rss(),"accuracy":accuracy});
    }
    let mut calls = Vec::with_capacity(CALLS);
    for _ in 0..CALLS {
        let start = Instant::now();
        drop(black_box(operation()));
        calls.push(start.elapsed().as_secs_f64() * 1e9);
    }
    let mut batches = Vec::new();
    let mut counts = Vec::new();
    for _ in 0..15 {
        let start = Instant::now();
        let mut count = 0;
        while start.elapsed() < Duration::from_millis(10) {
            drop(black_box(operation()));
            count += 1;
        }
        batches.push(start.elapsed().as_secs_f64() * 1e9 / count as f64);
        counts.push(count);
    }
    let timer: Vec<f64> = (0..CALLS)
        .map(|_| {
            let start = Instant::now();
            black_box(());
            start.elapsed().as_secs_f64() * 1e9
        })
        .collect();
    json!({"first_call_return_ns":first_ns,"warmup_calls":warmup_calls,"warmup_ns":warmup_ns,"per_call":distribution(calls),
        "batch_means":distribution(batches),"batch_call_counts":counts,
        "timer_pair":distribution(timer),"process_peak_rss_bytes":peak_rss(),"accuracy":accuracy})
}
fn gemm_error(n: usize, values: &[f64], get: impl Fn(usize, usize) -> f64) -> Value {
    let mut error = 0.0f64;
    for row in 0..n {
        for col in 0..n {
            let expected: f64 = (0..n)
                .map(|k| values[row * n + k] * values[k * n + col])
                .sum();
            let delta = (get(row, col) - expected).abs();
            assert!(delta <= 1e-11 * (1.0 + expected.abs()));
            error = error.max(delta);
        }
    }
    json!({"max_absolute_error":error,"relative_plus_absolute_budget":1e-11})
}
fn gemm(mode: &str, library: &str, n: usize) -> Value {
    let values: Vec<f64> = (0..n * n).map(|i| (i % 13) as f64 / 13.0).collect();
    match library {
        "kernix" => {
            let a = Tensor::from_values([n, n], values.clone()).unwrap();
            measure(
                mode,
                || kernix::matmul(&a, &a).unwrap(),
                |out| gemm_error(n, &values, |i, j| *out.get(&[i, j]).unwrap()),
            )
        }
        "ndarray" => {
            let a = ndarray::Array2::from_shape_vec((n, n), values.clone()).unwrap();
            measure(
                mode,
                || a.dot(&a),
                |out| gemm_error(n, &values, |i, j| out[(i, j)]),
            )
        }
        "nalgebra" => {
            let a = nalgebra::DMatrixView::from_slice_with_strides(&values, n, n, n, 1);
            measure(
                mode,
                || a * a,
                |out| gemm_error(n, &values, |i, j| out[(i, j)]),
            )
        }
        "faer" => {
            let a = faer::MatRef::from_row_major_slice(&values, n, n);
            measure(
                mode,
                || a * a,
                |out| gemm_error(n, &values, |i, j| out[(i, j)]),
            )
        }
        "candle" => {
            let a =
                candle_core::Tensor::from_vec(values.clone(), (n, n), &candle_core::Device::Cpu)
                    .unwrap();
            measure(
                mode,
                || a.matmul(&a).unwrap(),
                |out| {
                    let v = out.to_vec2::<f64>().unwrap();
                    gemm_error(n, &values, |i, j| v[i][j])
                },
            )
        }
        _ => panic!("unknown library"),
    }
}
fn gemm_f32_error(n: usize, values: &[f32], get: impl Fn(usize, usize) -> f32) -> Value {
    let mut error = 0.0f64;
    for row in 0..n {
        for col in 0..n {
            let expected: f64 = (0..n)
                .map(|k| f64::from(values[row * n + k]) * f64::from(values[k * n + col]))
                .sum();
            let delta = (f64::from(get(row, col)) - expected).abs();
            assert!(delta <= 1e-5 * (1.0 + expected.abs()));
            error = error.max(delta);
        }
    }
    json!({"max_absolute_error":error,"relative_plus_absolute_budget":1e-5})
}
fn gemm_f32(mode: &str, library: &str, n: usize) -> Value {
    let values: Vec<f32> = (0..n * n).map(|i| (i % 13) as f32 / 13.0).collect();
    match library {
        "kernix" => {
            let a = Tensor::from_values([n, n], values.clone()).unwrap();
            measure(
                mode,
                || kernix::matmul(&a, &a).unwrap(),
                |out| gemm_f32_error(n, &values, |i, j| *out.get(&[i, j]).unwrap()),
            )
        }
        "ndarray" => {
            let a = ndarray::Array2::from_shape_vec((n, n), values.clone()).unwrap();
            measure(
                mode,
                || a.dot(&a),
                |out| gemm_f32_error(n, &values, |i, j| out[(i, j)]),
            )
        }
        "nalgebra" => {
            let a = nalgebra::DMatrixView::from_slice_with_strides(&values, n, n, n, 1);
            measure(
                mode,
                || a * a,
                |out| gemm_f32_error(n, &values, |i, j| out[(i, j)]),
            )
        }
        "faer" => {
            let a = faer::MatRef::from_row_major_slice(&values, n, n);
            measure(
                mode,
                || a * a,
                |out| gemm_f32_error(n, &values, |i, j| out[(i, j)]),
            )
        }
        "candle" => {
            let a =
                candle_core::Tensor::from_vec(values.clone(), (n, n), &candle_core::Device::Cpu)
                    .unwrap();
            measure(
                mode,
                || a.matmul(&a).unwrap(),
                |out| {
                    let v = out.to_vec2::<f32>().unwrap();
                    gemm_f32_error(n, &values, |i, j| v[i][j])
                },
            )
        }
        _ => panic!("unknown library"),
    }
}
fn batched_gemm(mode: &str, library: &str) -> Value {
    let a: Vec<f64> = (0..4 * 16 * 32).map(|i| (i % 13) as f64 / 13.0).collect();
    let b: Vec<f64> = (0..32 * 16).map(|i| (i % 17) as f64 / 17.0).collect();
    let check = |get: &dyn Fn(usize, usize, usize) -> f64| {
        let mut error = 0.0f64;
        for batch in 0..4 {
            for row in 0..16 {
                for col in 0..16 {
                    let expected: f64 = (0..32)
                        .map(|k| a[(batch * 16 + row) * 32 + k] * b[k * 16 + col])
                        .sum();
                    let delta = (get(batch, row, col) - expected).abs();
                    assert!(delta < 1e-11 * (1.0 + expected.abs()));
                    error = error.max(delta);
                }
            }
        }
        json!({"max_absolute_error":error,"relative_plus_absolute_budget":1e-11})
    };
    match library {
        "kernix" => {
            let a = Tensor::from_values([4, 16, 32], a.clone()).unwrap();
            let b = Tensor::from_values([1, 32, 16], b.clone()).unwrap();
            measure(
                mode,
                || kernix::matmul(&a, &b).unwrap(),
                |out| check(&|i, j, k| *out.get(&[i, j, k]).unwrap()),
            )
        }
        "candle" => {
            let a =
                candle_core::Tensor::from_vec(a.clone(), (4, 16, 32), &candle_core::Device::Cpu)
                    .unwrap();
            let b =
                candle_core::Tensor::from_vec(b.clone(), (1, 32, 16), &candle_core::Device::Cpu)
                    .unwrap();
            measure(
                mode,
                || a.broadcast_matmul(&b).unwrap(),
                |out| {
                    let data = out.to_vec3::<f64>().unwrap();
                    check(&|i, j, k| data[i][j][k])
                },
            )
        }
        _ => panic!("unknown batched matmul library"),
    }
}
fn qr_error(
    m: usize,
    n: usize,
    values: &[f64],
    q: impl Fn(usize, usize) -> f64,
    r: impl Fn(usize, usize) -> f64,
) -> Value {
    let mut reconstruction = 0.0f64;
    let mut orthogonality = 0.0f64;
    for i in 0..m {
        for j in 0..n {
            let actual: f64 = (0..n).map(|k| q(i, k) * r(k, j)).sum();
            let error = (actual - values[i * n + j]).abs();
            assert!(error < 2e-12 * (1.0 + values[i * n + j].abs()));
            reconstruction = reconstruction.max(error);
        }
    }
    for i in 0..n {
        for j in 0..n {
            let actual: f64 = (0..m).map(|k| q(k, i) * q(k, j)).sum();
            let error = (actual - if i == j { 1.0 } else { 0.0 }).abs();
            assert!(error < 2e-12);
            orthogonality = orthogonality.max(error);
        }
    }
    json!({"reconstruction_max_absolute_error":reconstruction,"orthogonality_max_absolute_error":orthogonality,"budget":2e-12})
}
fn qr(mode: &str, library: &str, m: usize, n: usize) -> Value {
    let values: Vec<f64> = (0..m * n)
        .map(|i| ((i * 13 % 47) as f64 - 23.0) / 47.0 + if i / n == i % n { 3.0 } else { 0.0 })
        .collect();
    match library {
        "kernix" => {
            let a = Tensor::from_values([m, n], values.clone()).unwrap();
            measure(
                mode,
                || Qr::factor(&a).unwrap(),
                |out| {
                    qr_error(
                        m,
                        n,
                        &values,
                        |i, j| *out.q().get(&[i, j]).unwrap(),
                        |i, j| *out.r().get(&[i, j]).unwrap(),
                    )
                },
            )
        }
        "nalgebra" => {
            let a = nalgebra::DMatrixView::from_slice_with_strides(&values, m, n, n, 1);
            measure(
                mode,
                || {
                    let qr = a.clone_owned().qr();
                    (qr.q(), qr.r())
                },
                |out| qr_error(m, n, &values, |i, j| out.0[(i, j)], |i, j| out.1[(i, j)]),
            )
        }
        "faer" => {
            let a = faer::MatRef::from_row_major_slice(&values, m, n);
            measure(
                mode,
                || {
                    let qr = a.qr();
                    (qr.compute_thin_Q(), qr.thin_R().to_owned())
                },
                |out| qr_error(m, n, &values, |i, j| out.0[(i, j)], |i, j| out.1[(i, j)]),
            )
        }
        _ => panic!("unknown QR library"),
    }
}
fn chol_error(n: usize, values: &[f64], l: impl Fn(usize, usize) -> f64) -> Value {
    let mut error = 0.0f64;
    for i in 0..n {
        for j in 0..n {
            let actual: f64 = (0..=i.min(j)).map(|k| l(i, k) * l(j, k)).sum();
            let delta = (actual - values[i * n + j]).abs();
            assert!(delta < 2e-12 * (1.0 + values[i * n + j].abs()));
            error = error.max(delta);
        }
    }
    json!({"reconstruction_max_absolute_error":error,"budget":2e-12})
}
fn cholesky(mode: &str, library: &str, n: usize) -> Value {
    let values: Vec<f64> = (0..n * n)
        .map(|i| {
            0.01 / (1.0 + (i / n).abs_diff(i % n) as f64) + if i / n == i % n { 4.0 } else { 0.0 }
        })
        .collect();
    match library {
        "kernix" => {
            let a = Tensor::from_values([n, n], values.clone()).unwrap();
            measure(
                mode,
                || Cholesky::factor(&a).unwrap(),
                |out| chol_error(n, &values, |i, j| *out.lower().get(&[i, j]).unwrap()),
            )
        }
        "nalgebra" => {
            let a = nalgebra::DMatrixView::from_slice_with_strides(&values, n, n, n, 1);
            measure(
                mode,
                || a.clone_owned().cholesky().unwrap(),
                |out| chol_error(n, &values, |i, j| out.l_dirty()[(i, j)]),
            )
        }
        "faer" => {
            let a = faer::MatRef::from_row_major_slice(&values, n, n);
            measure(
                mode,
                || a.llt(faer::Side::Lower).unwrap(),
                |out| chol_error(n, &values, |i, j| out.L()[(i, j)]),
            )
        }
        _ => panic!("unknown Cholesky library"),
    }
}
fn softmax_error(get: impl Fn(usize, usize) -> f64) -> Value {
    let mut error = 0.0f64;
    for row in 0..256 {
        let values: Vec<f64> = (0..64)
            .map(|col| ((row * 64 + col) % 41) as f64 / 4.0)
            .collect();
        let max = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        let weights: Vec<f64> = values.iter().map(|x| (x - max).exp()).collect();
        let total: f64 = weights.iter().sum();
        for (col, weight) in weights.iter().enumerate() {
            let delta = (get(row, col) - weight / total).abs();
            assert!(delta < 2e-14);
            error = error.max(delta);
        }
    }
    json!({"max_absolute_error":error,"budget":2e-14})
}
fn softmax(mode: &str, library: &str) -> Value {
    let values: Vec<f64> = (0..256 * 64).map(|i| (i % 41) as f64 / 4.0).collect();
    match library {
        "kernix" => {
            let a = Tensor::from_values([256, 64], values).unwrap();
            measure(
                mode,
                || kernix::softmax(&a, -1).unwrap(),
                |out| softmax_error(|i, j| *out.get(&[i, j]).unwrap()),
            )
        }
        "ndarray" => {
            let a = ndarray::Array2::from_shape_vec((256, 64), values).unwrap();
            measure(
                mode,
                || {
                    let mut out = a.clone();
                    for mut row in out.rows_mut() {
                        let max = row.iter().copied().fold(f64::NEG_INFINITY, f64::max);
                        row.mapv_inplace(|x| (x - max).exp());
                        let total = row.sum();
                        row.mapv_inplace(|x| x / total);
                    }
                    out
                },
                |out| softmax_error(|i, j| out[(i, j)]),
            )
        }
        "candle" => {
            let a = candle_core::Tensor::from_vec(values, (256, 64), &candle_core::Device::Cpu)
                .unwrap();
            measure(
                mode,
                || {
                    let e = a
                        .broadcast_sub(&a.max_keepdim(1).unwrap())
                        .unwrap()
                        .exp()
                        .unwrap();
                    e.broadcast_div(&e.sum_keepdim(1).unwrap()).unwrap()
                },
                |out| {
                    let values = out.to_vec2::<f64>().unwrap();
                    softmax_error(|i, j| values[i][j])
                },
            )
        }
        _ => panic!("unknown softmax library"),
    }
}
fn backward(mode: &str, library: &str) -> Value {
    let (m, k, n) = (32, 16, 8);
    let av: Vec<f64> = (0..m * k)
        .map(|i| ((i * 7 % 31) as f64 - 15.0) / 31.0)
        .collect();
    let wv: Vec<f64> = (0..k * n)
        .map(|i| ((i * 11 % 29) as f64 - 14.0) / 29.0)
        .collect();
    let check = |gradient: &[f64]| {
        let mut error = 0.0f64;
        for i in 0..k {
            for j in 0..n {
                let expected: f64 = (0..m).map(|row| av[row * k + i]).sum();
                let delta = (gradient[i * n + j] - expected).abs();
                assert!(delta < 2e-12);
                error = error.max(delta);
            }
        }
        json!({"weight_gradient_max_absolute_error":error,"budget":2e-12})
    };
    match library {
        "kernix" => {
            let a = Tensor::from_values([m, k], av.clone()).unwrap();
            let w = Tensor::from_values([k, n], wv).unwrap();
            measure(
                mode,
                || {
                    let mut tape = Tape::new();
                    let x = tape.constant(a.clone()).unwrap();
                    let weight = tape.variable(w.clone()).unwrap();
                    let y = tape.matmul(&x, &weight).unwrap();
                    let loss = tape.sum(&y).unwrap();
                    let gradients = tape.backward(&loss).unwrap();
                    gradients.get(&weight).unwrap().unwrap().clone()
                },
                |out| check(out.storage_slice()),
            )
        }
        "candle" => {
            let a = candle_core::Tensor::from_vec(av.clone(), (m, k), &candle_core::Device::Cpu)
                .unwrap();
            let w = candle_core::Tensor::from_vec(wv, (k, n), &candle_core::Device::Cpu).unwrap();
            measure(
                mode,
                || {
                    let x = a.copy().unwrap();
                    let weight = candle_core::Var::from_tensor(&w).unwrap();
                    let loss = x.matmul(&weight).unwrap().sum_all().unwrap();
                    let gradients = loss.backward().unwrap();
                    gradients.get(&weight).unwrap().copy().unwrap()
                },
                |out| check(&out.flatten_all().unwrap().to_vec1::<f64>().unwrap()),
            )
        }
        _ => panic!("backward comparison requires an autodiff framework"),
    }
}
fn identity_constructor(mode: &str, library: &str, one: bool) -> Value {
    let value = if one { 1.0f32 } else { 0.0 };
    let check = |get: &dyn Fn(usize, usize) -> f32| {
        for i in 0..64 {
            for j in 0..64 {
                assert_eq!(get(i, j), value);
            }
        }
        json!({"max_absolute_error":0.0,"budget":0.0})
    };
    match library {
        "kernix" => measure(
            mode,
            || {
                #[cfg(feature = "legacy-constructors")]
                {
                    Tensor::full([64, 64], value).unwrap()
                }
                #[cfg(not(feature = "legacy-constructors"))]
                {
                    if one {
                        Tensor::<f32>::ones([64, 64]).unwrap()
                    } else {
                        Tensor::<f32>::zeros([64, 64]).unwrap()
                    }
                }
            },
            |out| check(&|i, j| *out.get(&[i, j]).unwrap()),
        ),
        "ndarray" => measure(
            mode,
            || {
                if one {
                    ndarray::Array2::<f32>::ones((64, 64))
                } else {
                    ndarray::Array2::<f32>::zeros((64, 64))
                }
            },
            |out| check(&|i, j| out[(i, j)]),
        ),
        "faer" => measure(
            mode,
            || {
                if one {
                    faer::Mat::<f32>::ones(64, 64)
                } else {
                    faer::Mat::<f32>::zeros(64, 64)
                }
            },
            |out| check(&|i, j| out[(i, j)]),
        ),
        "candle" => measure(
            mode,
            || {
                if one {
                    candle_core::Tensor::ones(
                        (64, 64),
                        candle_core::DType::F32,
                        &candle_core::Device::Cpu,
                    )
                    .unwrap()
                } else {
                    candle_core::Tensor::zeros(
                        (64, 64),
                        candle_core::DType::F32,
                        &candle_core::Device::Cpu,
                    )
                    .unwrap()
                }
            },
            |out| {
                let values = out.to_vec2::<f32>().unwrap();
                check(&|i, j| values[i][j])
            },
        ),
        _ => panic!("unknown constructor library"),
    }
}
fn main() {
    let args: Vec<String> = std::env::args().collect();
    assert_eq!(
        args.len(),
        4,
        "usage: harness WORKLOAD LIBRARY time|resources|first"
    );
    let (workload, library, mode) = (&*args[1], &*args[2], &*args[3]);
    assert!(["time", "resources", "first"].contains(&mode));
    faer::set_global_parallelism(faer::Par::Seq);
    let result = match workload {
        name if name.starts_with("training_") => {
            #[cfg(not(feature = "legacy-autograd"))]
            {
                training::run(name, library, mode)
            }
            #[cfg(feature = "legacy-autograd")]
            {
                let _ = name;
                panic!("legacy registry baseline does not support these Tape operations")
            }
        }
        "create_zeros_64x64_f32" => identity_constructor(mode, library, false),
        "create_ones_64x64_f32" => identity_constructor(mode, library, true),
        "matmul_batched_4x16x32x16_f64" => batched_gemm(mode, library),
        "matmul_32_f64" => gemm(mode, library, 32),
        "matmul_64_f64" => gemm(mode, library, 64),
        "matmul_128_f64" => gemm(mode, library, 128),
        "matmul_256_f64" => gemm(mode, library, 256),
        "matmul_64_f32" => gemm_f32(mode, library, 64),
        "matmul_128_f32" => gemm_f32(mode, library, 128),
        "matmul_256_f32" => gemm_f32(mode, library, 256),
        "qr_96x32_f64" => qr(mode, library, 96, 32),
        "qr_256x64_f64" => qr(mode, library, 256, 64),
        "cholesky_factor_64_f64" => cholesky(mode, library, 64),
        "cholesky_factor_128_f64" => cholesky(mode, library, 128),
        "cholesky_factor_256_f64" => cholesky(mode, library, 256),
        "softmax_256x64_f64" => softmax(mode, library),
        "matmul_sum_backward_32x16x8_f64" => backward(mode, library),
        _ => panic!("unknown workload"),
    };
    println!("{result}");
}
#[cfg(test)]
mod tests {
    #[test]
    #[cfg(not(feature = "allocation-counting"))]
    fn all_f32_gemm_workers_validate_against_the_wider_scalar_oracle() {
        faer::set_global_parallelism(faer::Par::Seq);
        for library in ["kernix", "ndarray", "nalgebra", "faer", "candle"] {
            let result = super::gemm_f32("first", library, 65);
            assert!(
                result["accuracy"]["max_absolute_error"]
                    .as_f64()
                    .unwrap()
                    .is_finite()
            );
        }
    }
    #[test]
    fn nearest_rank_quantiles_distinguish_tail_from_maximum() {
        let v: Vec<f64> = (1..=100).map(f64::from).collect();
        assert_eq!(super::quantile(&v, 50, 100), 50.0);
        assert_eq!(super::quantile(&v, 95, 100), 95.0);
        assert_eq!(super::quantile(&v, 99, 100), 99.0);
        assert_eq!(super::quantile(&[7.0], 95, 100), 7.0);
        let data = super::distribution(vec![4.0, 1.0, 3.0, 2.0]);
        assert_eq!(data["samples_ns"], serde_json::json!([4.0, 1.0, 3.0, 2.0]));
    }
}
