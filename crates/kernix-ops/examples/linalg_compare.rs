//! General dense linear algebra comparisons with nalgebra (same mathematical outputs).
use kernix_core::{Order, shape::Kahen, tensor::Tensor};
use kernix_ops::linalg::{Cholesky, Lu, Qr, residual_norm};
use nalgebra::{DMatrix, DVector};
use std::{
    hint::black_box,
    time::{Duration, Instant},
};
fn measure(mut f: impl FnMut()) -> serde_json::Value {
    for _ in 0..5 {
        f();
    }
    let mut v = Vec::new();
    for _ in 0..15 {
        let now = Instant::now();
        let mut count = 0;
        while now.elapsed() < Duration::from_millis(25) {
            f();
            count += 1;
        }
        v.push(now.elapsed().as_secs_f64() * 1e9 / count as f64);
    }
    v.sort_by(f64::total_cmp);
    serde_json::json!({"p50_ns":v[7],"p95_ns":v[14],"samples_ns":v})
}
fn main() {
    let n: usize = 64;
    let av: Vec<f64> = (0..n * n)
        .map(|i| {
            let row = i / n;
            let col = i % n;
            0.01 / (1.0 + row.abs_diff(col) as f64) + if row == col { 4.0 } else { 0.0 }
        })
        .collect();
    let bv: Vec<f64> = (0..n).map(|i| i as f64 / n as f64).collect();
    let a = Tensor::from_vec(av.clone(), Kahen::new([n, n]).unwrap(), Order::C).unwrap();
    let b = Tensor::from_vec(bv.clone(), Kahen::new([n]).unwrap(), Order::C).unwrap();
    let na = DMatrix::from_row_slice(n, n, &av);
    let nb = DVector::from_vec(bv);
    let x = Lu::factor(&a).unwrap().solve(&b).unwrap();
    assert!(residual_norm(&a, &x, &b).unwrap() < 1e-12);
    let nx = na.clone().lu().solve(&nb).unwrap();
    for (&x, &y) in x.storage_slice().iter().zip(nx.iter()) {
        assert!((x - y).abs() < 1e-12);
    }
    let mut results = serde_json::Map::new();
    results.insert(
        "solve_64_f64/kernix".into(),
        measure(|| {
            black_box(Lu::factor(&a).unwrap().solve(&b).unwrap());
        }),
    );
    results.insert(
        "solve_64_f64/nalgebra".into(),
        measure(|| {
            black_box(na.clone().lu().solve(&nb).unwrap());
        }),
    );
    let c = Cholesky::factor(&a).unwrap();
    let nc = na.clone().cholesky().unwrap().l();
    for row in 0..n {
        for col in 0..n {
            assert!((c.lower().storage_slice()[row * n + col] - nc[(row, col)]).abs() < 1e-12);
        }
    }
    results.insert(
        "cholesky_64_f64/kernix".into(),
        measure(|| {
            black_box(Cholesky::factor(&a).unwrap());
        }),
    );
    results.insert(
        "cholesky_64_f64/nalgebra".into(),
        measure(|| {
            black_box(na.clone().cholesky().unwrap().l());
        }),
    );
    for (m, n) in [(8, 4), (32, 16), (96, 32), (256, 64)] {
        let values: Vec<f64> = (0..m * n)
            .map(|i| ((i * 13 % 47) as f64 - 23.0) / 47.0 + if i / n == i % n { 3.0 } else { 0.0 })
            .collect();
        let a = Tensor::from_vec(values.clone(), Kahen::new([m, n]).unwrap(), Order::C).unwrap();
        let na = DMatrix::from_row_slice(m, n, &values);
        let q = Qr::factor(&a).unwrap();
        let reconstruction = kernix_ops::matmul(q.q(), q.r()).unwrap();
        for (&x, &y) in reconstruction.storage_slice().iter().zip(a.storage_slice()) {
            assert!((x - y).abs() < 1e-12);
        }
        results.insert(
            format!("qr_{m}x{n}_f64/kernix"),
            measure(|| {
                black_box(Qr::factor(&a).unwrap());
            }),
        );
        results.insert(
            format!("qr_{m}x{n}_f64/nalgebra"),
            measure(|| {
                let qr = na.clone().qr();
                black_box((qr.q(), qr.r()));
            }),
        );
    }
    println!("{}", serde_json::Value::Object(results));
}
