//! Allocation-inclusive, single-thread comparative CPU benchmarks.
use kernix_core::{Order, shape::Kahen, tensor::Tensor};
use kernix_ops::{add, matmul, softmax, sum_all};
use ndarray::{Array1, Array2, Axis};
use std::{
    hint::black_box,
    time::{Duration, Instant},
};

fn measure(mut operation: impl FnMut()) -> serde_json::Value {
    for _ in 0..8 {
        operation();
    }
    let mut samples = Vec::new();
    for _ in 0..15 {
        let start = Instant::now();
        let mut count = 0;
        while start.elapsed() < Duration::from_millis(25) {
            operation();
            count += 1;
        }
        samples.push(start.elapsed().as_secs_f64() * 1e9 / count as f64);
    }
    samples.sort_by(f64::total_cmp);
    serde_json::json!({"p50_ns": samples[7], "p95_ns": samples[14], "samples_ns": samples})
}
fn tensor(values: Vec<f64>, dims: &[usize]) -> Tensor<f64> {
    Tensor::from_vec(values, Kahen::new(dims).unwrap(), Order::C).unwrap()
}
fn main() {
    let mut results = serde_json::Map::new();
    let v: Vec<f64> = (0..65536).map(|i| (i % 31) as f64 / 31.0).collect();
    let a = tensor(v.clone(), &[65536]);
    let b = tensor(v.clone(), &[65536]);
    let na = Array1::from_vec(v.clone());
    let nb = na.clone();
    results.insert(
        "add_65536_f64/kernix".into(),
        measure(|| {
            black_box(add(&a, &b).unwrap());
        }),
    );
    results.insert(
        "add_65536_f64/ndarray".into(),
        measure(|| {
            black_box(&na + &nb);
        }),
    );
    results.insert(
        "sum_65536_f64/kernix".into(),
        measure(|| {
            black_box(sum_all(&a).unwrap());
        }),
    );
    results.insert(
        "sum_65536_f64/ndarray".into(),
        measure(|| {
            black_box(na.sum());
        }),
    );
    for size in [32, 128] {
        let vals: Vec<f64> = (0..size * size).map(|i| (i % 13) as f64 / 13.0).collect();
        let a = tensor(vals.clone(), &[size, size]);
        let b = tensor(vals.clone(), &[size, size]);
        let na = Array2::from_shape_vec((size, size), vals).unwrap();
        let nb = na.clone();
        let reference = na.dot(&nb);
        let actual = matmul(&a, &b).unwrap();
        for (x, y) in actual.storage_slice().iter().zip(reference.iter()) {
            assert!((x - y).abs() <= 1e-11 * (1.0 + y.abs()));
        }
        results.insert(
            format!("matmul_{size}_f64/kernix"),
            measure(|| {
                black_box(matmul(&a, &b).unwrap());
            }),
        );
        results.insert(
            format!("matmul_{size}_f64/ndarray"),
            measure(|| {
                black_box(na.dot(&nb));
            }),
        );
    }
    let vals: Vec<f64> = (0..256 * 64).map(|i| (i % 41) as f64 / 4.0).collect();
    let a = tensor(vals.clone(), &[256, 64]);
    let na = Array2::from_shape_vec((256, 64), vals).unwrap();
    results.insert(
        "softmax_256x64_f64/kernix".into(),
        measure(|| {
            black_box(softmax(&a, -1).unwrap());
        }),
    );
    results.insert(
        "softmax_256x64_f64/ndarray".into(),
        measure(|| {
            let mut out = na.clone();
            for mut row in out.axis_iter_mut(Axis(0)) {
                let max = row.iter().copied().fold(f64::NEG_INFINITY, f64::max);
                row.mapv_inplace(|x| (x - max).exp());
                let sum = row.sum();
                row.mapv_inplace(|x| x / sum);
            }
            black_box(out);
        }),
    );
    println!("{}", serde_json::Value::Object(results));
}
