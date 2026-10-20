//! Allocation-inclusive dynamic construction; identical values and owned outputs.
use kernix::prelude::*;
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
    serde_json::json!({"p50_ns":samples[7],"p95_ns":samples[14],"samples_ns":samples})
}

fn main() {
    let a = full!(1.0f64;64,64).unwrap();
    assert_eq!(
        a.storage_slice(),
        ndarray::Array2::from_elem((64, 64), 1.0)
            .as_slice()
            .unwrap()
    );
    let a = matrix![1.0f64,2.0,3.0;4.0,5.0,6.0;7.0,8.0,9.0].unwrap();
    let na = ndarray::arr2(&[[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]);
    assert_eq!(a.storage_slice(), na.as_slice().unwrap());
    let mut results = serde_json::Map::new();
    results.insert(
        "create_full_64x64_f64/kernix".into(),
        measure(|| {
            black_box(full!(black_box(1.0f64);64,64).unwrap());
        }),
    );
    results.insert(
        "create_full_64x64_f64/ndarray".into(),
        measure(|| {
            black_box(ndarray::Array2::from_elem((64, 64), black_box(1.0f64)));
        }),
    );
    results.insert(
        "create_matrix_3x3_f64/kernix".into(),
        measure(|| {
            let x = black_box(1.0f64);
            black_box(matrix![x,2.0,3.0;4.0,5.0,6.0;7.0,8.0,9.0].unwrap());
        }),
    );
    results.insert(
        "create_matrix_3x3_f64/ndarray".into(),
        measure(|| {
            let x = black_box(1.0f64);
            black_box(ndarray::arr2(&[
                [x, 2.0, 3.0],
                [4.0, 5.0, 6.0],
                [7.0, 8.0, 9.0],
            ]));
        }),
    );
    println!("{}", serde_json::Value::Object(results));
}
