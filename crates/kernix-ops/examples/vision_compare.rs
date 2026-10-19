//! Fused RGB preprocessing and convolution comparisons, with explicit reference algorithms.
use kernix_core::{Order, shape::Kahen, tensor::Tensor};
use kernix_ops::{
    neural::{Conv2dOptions, conv2d},
    vision::RgbFrame,
};
use ndarray::{Array3, Array4};
use std::{
    hint::black_box,
    time::{Duration, Instant},
};
fn measure(mut operation: impl FnMut()) -> serde_json::Value {
    for _ in 0..5 {
        operation();
    }
    let mut v = Vec::new();
    for _ in 0..15 {
        let start = Instant::now();
        let mut n = 0;
        while start.elapsed() < Duration::from_millis(25) {
            operation();
            n += 1;
        }
        v.push(start.elapsed().as_secs_f64() * 1e9 / n as f64);
    }
    v.sort_by(f64::total_cmp);
    serde_json::json!({"p50_ns":v[7],"p95_ns":v[14],"samples_ns":v})
}
fn ndarray_resize(a: &Array3<u8>) -> Array4<f32> {
    let mut out = Array4::zeros((1, 3, 224, 224));
    for y in 0..224 {
        let sy = ((y as f64 + 0.5) * 1080.0 / 224.0 - 0.5).clamp(0.0, 1079.0);
        let y0 = sy as usize;
        let y1 = (y0 + 1).min(1079);
        let fy = sy - y0 as f64;
        for x in 0..224 {
            let sx = ((x as f64 + 0.5) * 1920.0 / 224.0 - 0.5).clamp(0.0, 1919.0);
            let x0 = sx as usize;
            let x1 = (x0 + 1).min(1919);
            let fx = sx - x0 as f64;
            for c in 0..3 {
                let top = a[[y0, x0, c]] as f64 * (1.0 - fx) + a[[y0, x1, c]] as f64 * fx;
                let bottom = a[[y1, x0, c]] as f64 * (1.0 - fx) + a[[y1, x1, c]] as f64 * fx;
                out[[0, c, y, x]] = ((top * (1.0 - fy) + bottom * fy) / 255.0) as f32;
            }
        }
    }
    out
}
fn ndarray_conv(a: &Array4<f32>, w: &Array4<f32>) -> Array4<f32> {
    let mut out = Array4::zeros((1, 8, 30, 30));
    for co in 0..8 {
        for y in 0..30 {
            for x in 0..30 {
                let mut sum = 0.0;
                for ci in 0..3 {
                    for ky in 0..3 {
                        for kx in 0..3 {
                            sum += a[[0, ci, y + ky, x + kx]] * w[[co, ci, ky, kx]];
                        }
                    }
                }
                out[[0, co, y, x]] = sum;
            }
        }
    }
    out
}
fn main() {
    let pixels: Vec<u8> = (0..1920 * 1080 * 3).map(|i| (i % 251) as u8).collect();
    let frame = RgbFrame::new(&pixels, 1920, 1080).unwrap();
    let array = Array3::from_shape_vec((1080, 1920, 3), pixels.clone()).unwrap();
    let actual = frame.resize_nchw([224, 224], [0.0; 3], [1.0; 3]).unwrap();
    let expected = ndarray_resize(&array);
    assert_eq!(actual.storage_slice(), expected.as_slice().unwrap());
    let mut results = serde_json::Map::new();
    results.insert(
        "rgb1080_to224_f32/kernix".into(),
        measure(|| {
            black_box(frame.resize_nchw([224, 224], [0.0; 3], [1.0; 3]).unwrap());
        }),
    );
    results.insert(
        "rgb1080_to224_f32/ndarray".into(),
        measure(|| {
            black_box(ndarray_resize(&array));
        }),
    );
    let av: Vec<f32> = (0..3 * 32 * 32)
        .map(|i| ((i % 31) as f32 - 15.0) / 31.0)
        .collect();
    let wv: Vec<f32> = (0..8 * 3 * 3 * 3)
        .map(|i| ((i % 13) as f32 - 6.0) / 13.0)
        .collect();
    let a = Tensor::from_vec(av.clone(), Kahen::new([1, 3, 32, 32]).unwrap(), Order::C).unwrap();
    let w = Tensor::from_vec(wv.clone(), Kahen::new([8, 3, 3, 3]).unwrap(), Order::C).unwrap();
    let na = Array4::from_shape_vec((1, 3, 32, 32), av).unwrap();
    let nw = Array4::from_shape_vec((8, 3, 3, 3), wv).unwrap();
    assert_eq!(
        conv2d(&a, &w, Default::default()).unwrap().storage_slice(),
        ndarray_conv(&na, &nw).as_slice().unwrap()
    );
    results.insert(
        "conv_1x3x32x32_8x3x3x3_f32/kernix".into(),
        measure(|| {
            black_box(conv2d(&a, &w, Conv2dOptions::default()).unwrap());
        }),
    );
    results.insert(
        "conv_1x3x32x32_8x3x3x3_f32/ndarray".into(),
        measure(|| {
            black_box(ndarray_conv(&na, &nw));
        }),
    );
    println!("{}", serde_json::Value::Object(results));
}
