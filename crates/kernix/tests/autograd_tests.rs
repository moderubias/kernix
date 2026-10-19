//! Independent finite differences and graph lifetime checks for all differentiation rules.
use kernix::{
    Kahen, Order, Tensor,
    autograd::{AutogradError, Tape, Variable},
    neural::Conv2dOptions,
};
fn tensor(dims: &[usize], values: &[f64]) -> Tensor<f64> {
    Tensor::from_vec(values.to_vec(), Kahen::new(dims).unwrap(), Order::C).unwrap()
}
fn near(a: f64, b: f64) {
    assert!(
        (a - b).abs() < 2e-6 * (1.0 + a.abs() + b.abs()),
        "analytic {a}, finite difference {b}"
    );
}
fn unary<F>(input: Tensor<f64>, build: F)
where
    F: Fn(&mut Tape<f64>, &Variable) -> Result<Variable, AutogradError>,
{
    let evaluate = |input: Tensor<f64>| {
        let mut t = Tape::new();
        let x = t.variable(input).unwrap();
        let y = build(&mut t, &x).unwrap();
        let out = t.value(&y).unwrap();
        out.storage_slice()
            .iter()
            .enumerate()
            .map(|(i, &v)| v * (0.3 + i as f64 * 0.11))
            .sum::<f64>()
    };
    let mut t = Tape::new();
    let x = t.variable(input.clone()).unwrap();
    let y = build(&mut t, &x).unwrap();
    let out = t.value(&y).unwrap();
    let seed = Tensor::from_vec(
        (0..out.numel()).map(|i| 0.3 + i as f64 * 0.11).collect(),
        out.shape().clone(),
        Order::C,
    )
    .unwrap();
    let g = t.backward_with(&y, &seed).unwrap();
    let dx = g.get(&x).unwrap().unwrap();
    for i in 0..input.numel() {
        let mut a = input.clone();
        let mut b = input.clone();
        let h = 1e-6;
        a.storage_slice_mut()[i] += h;
        b.storage_slice_mut()[i] -= h;
        near(
            dx.storage_slice()[i],
            (evaluate(a) - evaluate(b)) / (2.0 * h),
        );
    }
}
fn binary<F>(left: Tensor<f64>, right: Tensor<f64>, build: F)
where
    F: Fn(&mut Tape<f64>, &Variable, &Variable) -> Result<Variable, AutogradError>,
{
    let evaluate = |a: Tensor<f64>, b: Tensor<f64>| {
        let mut t = Tape::new();
        let a = t.variable(a).unwrap();
        let b = t.variable(b).unwrap();
        let y = build(&mut t, &a, &b).unwrap();
        t.value(&y)
            .unwrap()
            .storage_slice()
            .iter()
            .enumerate()
            .map(|(i, &v)| v * (0.3 + i as f64 * 0.11))
            .sum::<f64>()
    };
    let mut t = Tape::new();
    let a = t.variable(left.clone()).unwrap();
    let b = t.variable(right.clone()).unwrap();
    let y = build(&mut t, &a, &b).unwrap();
    let out = t.value(&y).unwrap();
    let seed = Tensor::from_vec(
        (0..out.numel()).map(|i| 0.3 + i as f64 * 0.11).collect(),
        out.shape().clone(),
        Order::C,
    )
    .unwrap();
    let g = t.backward_with(&y, &seed).unwrap();
    for (v, input) in [(&a, &left), (&b, &right)] {
        let dx = g.get(v).unwrap().unwrap();
        for i in 0..input.numel() {
            let mut hi = input.clone();
            let mut lo = input.clone();
            let h = 1e-6;
            hi.storage_slice_mut()[i] += h;
            lo.storage_slice_mut()[i] -= h;
            let numeric = if core::ptr::eq(input, &left) {
                (evaluate(hi, right.clone()) - evaluate(lo, right.clone())) / (2.0 * h)
            } else {
                (evaluate(left.clone(), hi) - evaluate(left.clone(), lo)) / (2.0 * h)
            };
            near(dx.storage_slice()[i], numeric);
        }
    }
}
#[test]
fn all_elementwise_and_broadcast_derivatives() {
    for op in [Tape::add, Tape::sub, Tape::mul, Tape::div] {
        binary(
            tensor(&[2, 1], &[0.7, 1.1]),
            tensor(&[1, 3], &[0.4, 0.9, 1.7]),
            op,
        );
    }
    for op in [
        Tape::relu,
        Tape::tanh,
        Tape::sigmoid,
        Tape::exp,
        Tape::ln,
        Tape::sqrt,
    ] {
        unary(tensor(&[2, 2], &[0.4, 0.8, 1.2, 1.6]), op);
    }
    unary(tensor(&[3], &[-0.7, 0.3, 1.1]), Tape::relu);
}
#[test]
fn reductions_reshape_permutation_and_normalization_vjps() {
    unary(tensor(&[2, 2], &[0.4, 0.8, 1.2, 1.6]), Tape::sum);
    unary(tensor(&[2, 2], &[0.4, 0.8, 1.2, 1.6]), Tape::mean);
    unary(tensor(&[2, 2], &[0.4, 0.8, 1.2, 1.6]), |t, x| {
        t.reshape(x, [4])
    });
    unary(tensor(&[2, 2], &[0.4, 0.8, 1.2, 1.6]), |t, x| {
        t.permute(x, &[1, 0])
    });
    for axis in [0, -1] {
        unary(tensor(&[2, 2], &[0.4, 0.8, 1.2, 1.6]), |t, x| {
            t.softmax(x, axis)
        });
        unary(tensor(&[2, 2], &[0.4, 0.8, 1.2, 1.6]), |t, x| {
            t.log_softmax(x, axis)
        });
    }
}
#[test]
fn matmul_and_dense_shared_parameter_gradients() {
    binary(
        tensor(&[2, 3], &[0.2, 0.4, 0.6, 0.8, 1.0, 1.2]),
        tensor(&[3, 2], &[0.1, 0.3, 0.5, 0.7, 0.9, 1.1]),
        Tape::matmul,
    );
    let mut t = Tape::new();
    let x = t.variable(tensor(&[2], &[2.0, 3.0])).unwrap();
    let y = t.mul(&x, &x).unwrap();
    let z = t.add(&y, &x).unwrap();
    let loss = t.sum(&z).unwrap();
    let mut g = t.backward(&loss).unwrap();
    assert_eq!(g.get(&x).unwrap().unwrap().storage_slice(), &[5.0, 7.0]);
    let fresh = t.backward(&loss).unwrap();
    g.accumulate(&fresh).unwrap();
    assert_eq!(g.get(&x).unwrap().unwrap().storage_slice(), &[10.0, 14.0]);
}
#[test]
fn convolution_and_pooling_gradients() {
    binary(
        tensor(
            &[1, 1, 3, 3],
            &[0.1, 0.3, 0.7, 0.4, 0.2, 0.8, 0.9, 0.6, 0.5],
        ),
        tensor(&[1, 1, 2, 2], &[0.2, 0.4, 0.7, 0.9]),
        |t, a, b| t.conv2d(a, b, Conv2dOptions::default()),
    );
    unary(
        tensor(
            &[1, 1, 3, 3],
            &[0.1, 0.3, 0.7, 0.4, 0.2, 0.8, 0.9, 0.6, 0.5],
        ),
        |t, x| t.avg_pool2d(x, [2, 2], [1, 1]),
    );
    unary(
        tensor(
            &[1, 1, 3, 3],
            &[0.1, 0.3, 0.7, 0.4, 0.2, 0.8, 0.9, 0.6, 0.5],
        ),
        |t, x| t.max_pool2d(x, [2, 2], [1, 1]),
    );
}
#[test]
fn loss_gradients_match_independent_perturbations() {
    unary(tensor(&[2, 3], &[0.2, 0.4, 0.7, 0.9, 0.3, 0.5]), |t, x| {
        t.cross_entropy(x, &[2, 0])
    });
    binary(
        tensor(&[2, 2], &[0.2, 0.4, 0.7, 0.9]),
        tensor(&[2, 2], &[0.1, 0.3, 0.8, 0.5]),
        Tape::mse,
    );
    binary(
        tensor(&[2, 2], &[0.2, 0.4, 0.7, 0.9]),
        tensor(&[2, 2], &[0.1, 0.3, 0.8, 0.5]),
        Tape::binary_cross_entropy,
    );
}
#[test]
fn foreign_stale_handles_constants_and_detach() {
    let mut a = Tape::new();
    let mut b = Tape::new();
    let x = a.variable(tensor(&[1], &[2.0])).unwrap();
    let y = b.variable(tensor(&[1], &[3.0])).unwrap();
    assert!(a.add(&x, &y).is_err());
    let constant = a.constant(tensor(&[1], &[1.0])).unwrap();
    let disconnected = a.variable(tensor(&[1], &[3.0])).unwrap();
    let z = a.add(&x, &constant).unwrap();
    let loss = a.sum(&z).unwrap();
    let g = a.backward(&loss).unwrap();
    assert!(g.get(&constant).unwrap().is_none());
    assert!(g.get(&disconnected).unwrap().is_none());
    let state = a.detach(&z).unwrap();
    a.clear();
    assert!(a.value(&x).is_err());
    assert!(a.is_empty());
    assert!(g.get(&x).unwrap().is_some());
    let leaf = a.constant(state).unwrap();
    assert_eq!(a.value(&leaf).unwrap().storage_slice(), &[3.0]);
    assert!(g.get(&leaf).is_err());
}
#[test]
fn empty_broadcast_gradient_and_scalar_seeds() {
    let mut t = Tape::new();
    let a = t.variable(tensor(&[1, 3], &[1.0, 2.0, 3.0])).unwrap();
    let b = t.variable(tensor(&[0, 3], &[])).unwrap();
    let x = t.add(&a, &b).unwrap();
    assert!(t.backward(&x).is_err());
    let loss = t.sum(&x).unwrap();
    let g = t.backward(&loss).unwrap();
    assert_eq!(g.get(&a).unwrap().unwrap().storage_slice(), &[0.0; 3]);
    assert!(g.get(&b).unwrap().unwrap().is_empty());
}
#[test]
fn finite_domain_and_singular_derivative_errors() {
    let mut t = Tape::new();
    assert!(t.variable(tensor(&[1], &[f64::NAN])).is_err());
    let zero = t.variable(tensor(&[], &[0.0])).unwrap();
    let s = t.sqrt(&zero).unwrap();
    assert!(matches!(
        t.backward(&s),
        Err(AutogradError::NonFinite { .. })
    ));
    let large = t.variable(tensor(&[], &[1000.0])).unwrap();
    let count = t.len();
    assert!(t.exp(&large).is_err());
    assert_eq!(t.len(), count);
    let mut t = Tape::new();
    let a = t.variable(tensor(&[], &[1e200])).unwrap();
    let b = t.variable(tensor(&[], &[1e200])).unwrap();
    let y = t.div(&a, &b).unwrap();
    let g = t.backward(&y).unwrap();
    assert_eq!(g.get(&b).unwrap().unwrap().storage_slice(), &[-1e-200]);
}

#[test]
fn layer_reuse_shares_parameter_leaves_across_recurrent_steps() {
    use kernix::{nn::Linear, training::SeededRng};
    let layer = Linear::<f64>::new(2, 2, &mut SeededRng::new(3)).unwrap();
    let mut tape = Tape::new();
    let input = tape.constant(tensor(&[1, 2], &[2.0, 3.0])).unwrap();
    let a = layer.record(&mut tape, &input).unwrap();
    let b = layer.record(&mut tape, &input).unwrap();
    let total = tape.add(&a.output, &b.output).unwrap();
    let loss = tape.sum(&total).unwrap();
    let g = tape.backward(&loss).unwrap();
    assert!(core::ptr::eq(
        g.get(&a.weight).unwrap().unwrap(),
        g.get(&b.weight).unwrap().unwrap()
    ));
    assert_eq!(
        g.get(&a.weight).unwrap().unwrap().storage_slice(),
        &[4.0, 6.0, 4.0, 6.0]
    );
    assert_eq!(
        g.get(&a.bias).unwrap().unwrap().storage_slice(),
        &[2.0, 2.0]
    );
}
