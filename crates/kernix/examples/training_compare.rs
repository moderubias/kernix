//! Allocation-inclusive dense MSE/SGD step, compared with explicit ndarray backpropagation.
use kernix::{
    Kahen, Order, Tensor,
    autograd::Tape,
    training::{Parameter, Sgd},
};
use ndarray::{Array1, Array2, Axis};
use std::{
    hint::black_box,
    time::{Duration, Instant},
};
fn tensor(v: Vec<f64>, dims: &[usize]) -> Tensor<f64> {
    Tensor::from_vec(v, Kahen::new(dims).unwrap(), Order::C).unwrap()
}
fn measure(mut f: impl FnMut()) -> serde_json::Value {
    for _ in 0..5 {
        f();
    }
    let mut v = Vec::new();
    for _ in 0..15 {
        let now = Instant::now();
        let mut n = 0;
        while now.elapsed() < Duration::from_millis(25) {
            f();
            n += 1;
        }
        v.push(now.elapsed().as_secs_f64() * 1e9 / n as f64);
    }
    v.sort_by(f64::total_cmp);
    serde_json::json!({"p50_ns":v[7],"p95_ns":v[14],"samples_ns":v})
}
fn kernix_step(
    a: &Tensor<f64>,
    initial: &Tensor<f64>,
    target: &Tensor<f64>,
) -> (Vec<f64>, Vec<f64>, f64) {
    let mut weight = Parameter::new(initial.clone()).unwrap();
    let mut bias = Parameter::new(tensor(vec![0.0; 8], &[8])).unwrap();
    let mut tape = Tape::new();
    let input = tape.constant(a.clone()).unwrap();
    let w = weight.track(&mut tape).unwrap();
    let b = bias.track(&mut tape).unwrap();
    let expected = tape.constant(target.clone()).unwrap();
    let product = tape.matmul(&input, &w).unwrap();
    let prediction = tape.add(&product, &b).unwrap();
    let loss = tape.mse(&prediction, &expected).unwrap();
    let value = tape.value(&loss).unwrap().storage_slice()[0];
    let gradients = tape.backward(&loss).unwrap();
    Sgd::new(0.01)
        .unwrap()
        .step(&mut [
            (&mut weight, gradients.get(&w).unwrap().unwrap()),
            (&mut bias, gradients.get(&b).unwrap().unwrap()),
        ])
        .unwrap();
    (
        weight.value().storage_slice().to_vec(),
        bias.value().storage_slice().to_vec(),
        value,
    )
}
fn ndarray_step(
    a: &Array2<f64>,
    initial: &Array2<f64>,
    target: &Array2<f64>,
) -> (Vec<f64>, Vec<f64>, f64) {
    let mut weight = initial.clone();
    let mut bias = Array1::zeros(8);
    let prediction = a.dot(&weight) + &bias;
    let diff = &prediction - target;
    let loss = diff.mapv(|x| x * x).sum() / 256.0;
    let gradient = diff * (2.0 / 256.0);
    let dw = a.t().dot(&gradient);
    let db = gradient.sum_axis(Axis(0));
    weight -= &(dw * 0.01);
    bias -= &(db * 0.01);
    (weight.into_raw_vec_and_offset().0, bias.to_vec(), loss)
}
fn main() {
    let av: Vec<f64> = (0..32 * 16)
        .map(|i| ((i * 7 % 31) as f64 - 15.0) / 31.0)
        .collect();
    let wv: Vec<f64> = (0..16 * 8)
        .map(|i| ((i * 11 % 29) as f64 - 14.0) / 29.0)
        .collect();
    let tv: Vec<f64> = (0..32 * 8).map(|i| ((i % 7) as f64 - 3.0) / 7.0).collect();
    let a = tensor(av.clone(), &[32, 16]);
    let w = tensor(wv.clone(), &[16, 8]);
    let target = tensor(tv.clone(), &[32, 8]);
    let na = Array2::from_shape_vec((32, 16), av).unwrap();
    let nw = Array2::from_shape_vec((16, 8), wv).unwrap();
    let nt = Array2::from_shape_vec((32, 8), tv).unwrap();
    let x = kernix_step(&a, &w, &target);
    let y = ndarray_step(&na, &nw, &nt);
    for (&x, &y) in x.0.iter().chain(&x.1).zip(y.0.iter().chain(&y.1)) {
        assert!((x - y).abs() < 1e-13);
    }
    assert!((x.2 - y.2).abs() < 1e-13);
    println!(
        "{}",
        serde_json::json!({"dense_sgd_step_32x16x8_f64/kernix":measure(||{black_box(kernix_step(&a,&w,&target));}),"dense_sgd_step_32x16x8_f64/ndarray":measure(||{black_box(ndarray_step(&na,&nw,&nt));})})
    );
}
