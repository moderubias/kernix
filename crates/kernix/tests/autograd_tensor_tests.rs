//! Vector/batched VJPs, axis reductions and ordinary sequence training.
use kernix::{
    OpsError, Tensor,
    autograd::{AutogradError, Tape},
};

fn values(dims: &[usize], offset: usize) -> Tensor<f64> {
    let count = kernix::Kahen::new(dims).unwrap().numel();
    Tensor::from_values(
        dims,
        (0..count)
            .map(|i| ((i * 7 + offset) % 19) as f64 / 13.0 - 0.6)
            .collect(),
    )
    .unwrap()
}
fn seed(dims: &[usize]) -> Tensor<f64> {
    values(dims, 3)
}
fn near(a: f64, b: f64) {
    assert!(
        (a - b).abs() <= 3e-7 * (1.0 + a.abs() + b.abs()),
        "analytic {a}, oracle {b}"
    );
}
fn objective(value: &Tensor<f64>, seed: &Tensor<f64>) -> f64 {
    assert_eq!(value.shape(), seed.shape());
    value
        .storage_slice()
        .iter()
        .zip(seed.storage_slice())
        .map(|(&a, &b)| a * b)
        .sum()
}
fn matmul_fd(ad: &[usize], bd: &[usize], expected: &[usize]) {
    let a = values(ad, 1);
    let b = values(bd, 5);
    let seed = seed(expected);
    let mut tape = Tape::new();
    let av = tape.variable(a.clone()).unwrap();
    let bv = tape.variable(b.clone()).unwrap();
    let y = tape.matmul(&av, &bv).unwrap();
    assert_eq!(tape.value(&y).unwrap().shape().dims(), expected);
    let g = tape.backward_with(&y, &seed).unwrap();
    for (side, input, variable) in [(false, &a, &av), (true, &b, &bv)] {
        let dx = g.get(variable).unwrap().unwrap();
        assert_eq!(dx.shape(), input.shape());
        for i in 0..input.numel() {
            let mut hi = input.clone();
            let mut lo = input.clone();
            let h = 1e-6;
            hi.storage_slice_mut()[i] += h;
            lo.storage_slice_mut()[i] -= h;
            let (yh, yl) = if side {
                (
                    kernix::matmul(&a, &hi).unwrap(),
                    kernix::matmul(&a, &lo).unwrap(),
                )
            } else {
                (
                    kernix::matmul(&hi, &b).unwrap(),
                    kernix::matmul(&lo, &b).unwrap(),
                )
            };
            near(
                dx.storage_slice()[i],
                (objective(&yh, &seed) - objective(&yl, &seed)) / (2.0 * h),
            );
        }
    }
}
#[test]
fn matmul_vector_matrix_and_broadcast_batch_finite_differences() {
    for (a, b, out) in [
        (&[3][..], &[3][..], &[][..]),
        (&[3], &[3, 2], &[2]),
        (&[2, 3], &[3], &[2]),
        (&[3], &[2, 3, 2], &[2, 2]),
        (&[2, 2, 3], &[3], &[2, 2]),
        (&[2, 1, 2, 3], &[1, 3, 3, 2], &[2, 3, 2, 2]),
        (&[2, 1, 2, 3], &[3, 3, 2], &[2, 3, 2, 2]),
        (&[2, 3], &[4, 3, 2], &[4, 2, 2]),
        (&[4, 2, 3], &[3, 2], &[4, 2, 2]),
        (&[1, 1, 1, 2, 3], &[2, 3, 2], &[1, 1, 2, 2, 2]),
    ] {
        matmul_fd(a, b, out);
    }
}
#[test]
fn matmul_zero_extents_have_exact_shaped_zero_gradients() {
    for (ad, bd, out) in [
        (&[1, 2, 3][..], &[0, 3, 4][..], &[0, 2, 4][..]),
        (&[0, 2, 3], &[1, 3, 4], &[0, 2, 4]),
        (&[3], &[0, 3, 4], &[0, 4]),
        (&[0, 2, 3], &[3], &[0, 2]),
        (&[2, 0], &[0, 3], &[2, 3]),
        (&[0, 3], &[3, 2], &[0, 2]),
        (&[2, 3], &[3, 0], &[2, 0]),
        (&[0], &[0], &[]),
        (&[0], &[2, 0, 3], &[2, 3]),
        (&[2, 3, 0], &[0], &[2, 3]),
    ] {
        let mut t = Tape::new();
        let a = t.variable(values(ad, 1)).unwrap();
        let b = t.variable(values(bd, 2)).unwrap();
        let y = t.matmul(&a, &b).unwrap();
        assert_eq!(t.value(&y).unwrap().shape().dims(), out);
        let loss = t.sum(&y).unwrap();
        let g = t.backward(&loss).unwrap();
        for (v, dims) in [(&a, ad), (&b, bd)] {
            let dx = g.get(v).unwrap().unwrap();
            assert_eq!(dx.shape().dims(), dims);
            assert!(dx.storage_slice().iter().all(|&x| x == 0.0));
        }
    }
}
#[test]
fn matmul_shared_dag_repeated_backward_and_seeded_dot() {
    let mut t = Tape::new();
    let x = t.variable(values(&[2, 2, 2], 1)).unwrap();
    let product = t.matmul(&x, &x).unwrap();
    let y = t.add(&product, &product).unwrap();
    let loss = t.sum(&y).unwrap();
    let first = t.backward(&loss).unwrap();
    let second = t.backward(&loss).unwrap();
    let xdata = t.value(&x).unwrap().storage_slice();
    for batch in 0..2 {
        for i in 0..2 {
            for j in 0..2 {
                let a = &xdata[batch * 4..batch * 4 + 4];
                let expected = 2.0 * (a[j * 2] + a[j * 2 + 1] + a[i] + a[2 + i]);
                near(
                    first.get(&x).unwrap().unwrap().storage_slice()[batch * 4 + i * 2 + j],
                    expected,
                );
            }
        }
    }
    assert_eq!(
        first.get(&x).unwrap().unwrap().storage_slice(),
        second.get(&x).unwrap().unwrap().storage_slice()
    );
    let mut t = Tape::new();
    let x = t
        .variable(Tensor::from_values([3], vec![1.0, 2.0, 3.0]).unwrap())
        .unwrap();
    let dot = t.matmul(&x, &x).unwrap();
    let g = t
        .backward_with(&dot, &Tensor::full([], 2.0).unwrap())
        .unwrap();
    assert_eq!(
        g.get(&x).unwrap().unwrap().storage_slice(),
        &[4.0, 8.0, 12.0]
    );
}

#[test]
fn failed_shared_gradient_accumulation_does_not_mutate_primal_or_retained_results() {
    let mut tape = Tape::new();
    let x = tape
        .variable(Tensor::<f64>::zeros([2, 3]).unwrap())
        .unwrap();
    let large = tape
        .constant(Tensor::<f64>::full([2, 3], f64::MAX * 0.75).unwrap())
        .unwrap();
    let a = tape.mul(&x, &large).unwrap();
    let b = tape.mul(&x, &large).unwrap();
    let y = tape.add(&a, &b).unwrap();
    let bad = tape.sum(&y).unwrap();
    let safe = tape.sum(&x).unwrap();
    let retained = tape.backward(&safe).unwrap();
    assert!(tape.backward(&bad).is_err());
    assert_eq!(tape.value(&x).unwrap().storage_slice(), &[0.; 6]);
    assert_eq!(retained.get(&x).unwrap().unwrap().storage_slice(), &[1.; 6]);
    let fresh = tape.backward(&safe).unwrap();
    assert_eq!(fresh.get(&x).unwrap().unwrap().storage_slice(), &[1.; 6]);
}
#[test]
fn matmul_transposed_reversed_inputs_and_constants() {
    let mut a = values(&[2, 2, 3], 1);
    a.reverse_axis_in_place(2).unwrap();
    let mut b = values(&[2, 4, 3], 2);
    b.swap_axes_in_place(1, 2).unwrap();
    let mut t = Tape::new();
    let av = t.variable(a.clone()).unwrap();
    let bv = t.constant(b.clone()).unwrap();
    let y = t.matmul(&av, &bv).unwrap();
    let expected = kernix::matmul(&a, &b).unwrap();
    for (&a, &b) in t
        .value(&y)
        .unwrap()
        .storage_slice()
        .iter()
        .zip(expected.storage_slice())
    {
        near(a, b);
    }
    let out = t.permute(&y, &[1, 0, 2]).unwrap();
    let loss = t.sum(&out).unwrap();
    let g = t.backward(&loss).unwrap();
    assert!(g.get(&bv).unwrap().is_none());
    let expected = kernix::matmul(
        &Tensor::ones([2, 2, 4]).unwrap(),
        &b.swap_axes(1, 2).unwrap(),
    )
    .unwrap();
    for (&a, &b) in g
        .get(&av)
        .unwrap()
        .unwrap()
        .storage_slice()
        .iter()
        .zip(expected.storage_slice())
    {
        near(a, b);
    }
}
#[test]
fn matmul_errors_preserve_handle_and_shape_contracts() {
    let mut t = Tape::new();
    let x = t.variable(values(&[2, 3], 1)).unwrap();
    let scalar = t.variable(values(&[], 0)).unwrap();
    let wrong = t.variable(values(&[4, 2], 2)).unwrap();
    let batch = t.variable(values(&[3, 3, 2], 3)).unwrap();
    let count = t.len();
    assert!(matches!(
        t.matmul(&x, &scalar),
        Err(AutogradError::Ops(OpsError::InvalidRank { .. }))
    ));
    assert!(matches!(
        t.matmul(&x, &wrong),
        Err(AutogradError::Ops(
            OpsError::ContractDimensionMismatch { .. }
        ))
    ));
    assert_eq!(t.len(), count);
    let y = t.matmul(&x, &batch).unwrap();
    assert!(matches!(t.backward(&y), Err(AutogradError::ExpectedScalar)));
    assert!(matches!(
        t.backward_with(&y, &values(&[2, 2], 0)),
        Err(AutogradError::GradientShape)
    ));
    assert!(
        t.backward_with(&y, &Tensor::full([3, 2, 2], f64::NAN).unwrap())
            .is_err()
    );
    let mut other = Tape::new();
    let foreign = other.variable(values(&[3], 0)).unwrap();
    assert!(matches!(
        t.matmul(&x, &foreign),
        Err(AutogradError::ForeignVariable)
    ));
    t.clear();
    assert!(matches!(
        t.matmul(&x, &x),
        Err(AutogradError::ForeignVariable)
    ));
}
fn reduction_fd(axes: &[isize], keepdims: bool, mean: bool) {
    let input = values(&[2, 3, 2], 4);
    let mut t = Tape::new();
    let x = t.variable(input.clone()).unwrap();
    let y = if mean {
        t.mean_axes(&x, axes, keepdims)
    } else {
        t.sum_axes(&x, axes, keepdims)
    }
    .unwrap();
    let s = seed(t.value(&y).unwrap().shape().dims());
    let g = t.backward_with(&y, &s).unwrap();
    for i in 0..input.numel() {
        let mut hi = input.clone();
        let mut lo = input.clone();
        hi.storage_slice_mut()[i] += 1e-6;
        lo.storage_slice_mut()[i] -= 1e-6;
        let op = |a: &Tensor<f64>| {
            if mean {
                kernix::mean_axes(a, axes, keepdims)
            } else {
                kernix::sum_axes(a, axes, keepdims)
            }
        };
        near(
            g.get(&x).unwrap().unwrap().storage_slice()[i],
            (objective(&op(&hi).unwrap(), &s) - objective(&op(&lo).unwrap(), &s)) / 2e-6,
        );
    }
}
#[test]
fn axis_reductions_finite_differences_and_axis_errors() {
    for axes in [&[][..], &[0], &[-1], &[2, 0], &[-1, -3, -2]] {
        for keep in [false, true] {
            for mean in [false, true] {
                reduction_fd(axes, keep, mean);
            }
        }
    }
    let mut t = Tape::new();
    let x = t.variable(values(&[2, 3], 0)).unwrap();
    let count = t.len();
    for axes in [&[2][..], &[-3], &[0, -2]] {
        assert!(t.sum_axes(&x, axes, false).is_err());
        assert!(t.mean_axes(&x, axes, true).is_err());
    }
    assert_eq!(t.len(), count);
}
#[test]
fn axis_reductions_scalar_empty_and_zero_broadcast() {
    for dims in [&[][..], &[0, 3], &[2, 0, 3]] {
        let mut t = Tape::new();
        let x = t.variable(values(dims, 0)).unwrap();
        let identity = t.mean_axes(&x, &[], false).unwrap();
        let loss = t.sum(&identity).unwrap();
        let g = t.backward(&loss).unwrap();
        assert_eq!(g.get(&x).unwrap().unwrap().shape().dims(), dims);
        assert!(
            g.get(&x)
                .unwrap()
                .unwrap()
                .storage_slice()
                .iter()
                .all(|&x| x == 1.0)
        );
    }
    let mut t = Tape::new();
    let x = t.variable(values(&[2, 0, 3], 0)).unwrap();
    assert!(t.mean_axes(&x, &[1], false).is_err());
    let s = t.sum_axes(&x, &[1], false).unwrap();
    assert_eq!(t.value(&s).unwrap().storage_slice(), &[0.0; 6]);
    let g = t.backward_with(&s, &Tensor::ones([2, 3]).unwrap()).unwrap();
    assert!(g.get(&x).unwrap().unwrap().is_empty());
    let empty = t.mean_axes(&x, &[0], false).unwrap();
    let loss = t.sum(&empty).unwrap();
    assert!(
        t.backward(&loss)
            .unwrap()
            .get(&x)
            .unwrap()
            .unwrap()
            .is_empty()
    );
}
#[test]
fn linear_record_accepts_vectors_batches_and_shared_parameters() {
    use kernix::{nn::Linear, training::SeededRng};
    let layer = Linear::<f64>::new(3, 2, &mut SeededRng::new(9)).unwrap();
    for dims in [&[3][..], &[2, 3], &[2, 4, 3], &[2, 0, 3]] {
        let input = values(dims, 5);
        let expected = layer.forward(&input).unwrap();
        let mut t = Tape::new();
        let x = t.variable(input).unwrap();
        let first = layer.record(&mut t, &x).unwrap();
        let second = layer.record(&mut t, &x).unwrap();
        for (&a, &b) in t
            .value(&first.output)
            .unwrap()
            .storage_slice()
            .iter()
            .zip(expected.storage_slice())
        {
            near(a, b);
        }
        let total = t.add(&first.output, &second.output).unwrap();
        let loss = t.sum(&total).unwrap();
        let g = t.backward(&loss).unwrap();
        assert!(core::ptr::eq(
            g.get(&first.weight).unwrap().unwrap(),
            g.get(&second.weight).unwrap().unwrap()
        ));
        let rows = if expected.is_empty() {
            0
        } else {
            expected.numel() / 2
        };
        for &dx in g.get(&first.bias).unwrap().unwrap().storage_slice() {
            near(dx, (rows * 2) as f64);
        }
        assert_eq!(g.get(&x).unwrap().unwrap().shape().dims(), dims);
    }
}

#[test]
fn layer_norm_input_and_affine_vjps_finite_differences() {
    for normalized_rank in [1, 2, 3] {
        let a = values(&[2, 2, 3], 6);
        let scale = values(&[3], 8);
        let bias = values(&[1, 1, 3], 11);
        let s = seed(a.shape().dims());
        let mut t = Tape::new();
        let x = t.variable(a.clone()).unwrap();
        let w = t.variable(scale.clone()).unwrap();
        let b = t.variable(bias.clone()).unwrap();
        let norm = t.layer_norm(&x, normalized_rank, 1e-4).unwrap();
        let scaled = t.mul(&norm, &w).unwrap();
        let out = t.add(&scaled, &b).unwrap();
        let g = t.backward_with(&out, &s).unwrap();
        let eval = |x: &Tensor<f64>, w: &Tensor<f64>, b: &Tensor<f64>| {
            let norm = kernix::neural::layer_norm(x, normalized_rank, 1e-4).unwrap();
            objective(
                &kernix::add(&kernix::mul(&norm, w).unwrap(), b).unwrap(),
                &s,
            )
        };
        for (side, input, var) in [(0, &a, &x), (1, &scale, &w), (2, &bias, &b)] {
            for i in 0..input.numel() {
                let mut hi = input.clone();
                let mut lo = input.clone();
                hi.storage_slice_mut()[i] += 1e-6;
                lo.storage_slice_mut()[i] -= 1e-6;
                let delta = match side {
                    0 => eval(&hi, &scale, &bias) - eval(&lo, &scale, &bias),
                    1 => eval(&a, &hi, &bias) - eval(&a, &lo, &bias),
                    _ => eval(&a, &scale, &hi) - eval(&a, &scale, &lo),
                };
                near(
                    g.get(var).unwrap().unwrap().storage_slice()[i],
                    delta / 2e-6,
                );
            }
        }
        let fresh = t.backward_with(&out, &s).unwrap();
        assert_eq!(
            g.get(&x).unwrap().unwrap().storage_slice(),
            fresh.get(&x).unwrap().unwrap().storage_slice()
        );
    }
}
#[test]
fn layer_norm_constant_singleton_empty_and_f32() {
    for n in [1, 3, 7, 15] {
        let input = Tensor::full([2, n], 1.3f64).unwrap();
        let gradient = Tensor::full([2, n], 1.7).unwrap();
        let normalized = kernix::neural::layer_norm(&input, 1, 1e-300).unwrap();
        assert!(normalized.storage_slice().iter().all(|&x| x == 0.0));
        let dx = kernix::neural::layer_norm_backward(&input, &gradient, 1, 1e-300).unwrap();
        assert!(dx.storage_slice().iter().all(|&x| x == 0.0));
    }
    let input = Tensor::full([2, 3], 1.3f32).unwrap();
    let g = Tensor::from_values([2, 3], vec![0.0f32, 1.0, 2.0, 3.0, 4.0, 5.0]).unwrap();
    let dx = kernix::neural::layer_norm_backward(&input, &g, 1, 0.25).unwrap();
    assert_eq!(dx.storage_slice(), &[-2.0, 0.0, 2.0, -2.0, 0.0, 2.0]);
    for dims in [&[0, 3][..], &[2, 0, 3], &[0, 0]] {
        let mut t = Tape::new();
        let x = t.variable(values(dims, 0)).unwrap();
        let norm = t.layer_norm(&x, 1, 1e-5).unwrap();
        let loss = t.sum(&norm).unwrap();
        assert!(
            t.backward(&loss)
                .unwrap()
                .get(&x)
                .unwrap()
                .unwrap()
                .is_empty()
        );
    }
    let mut t = Tape::new();
    let x = t.variable(values(&[2, 0], 0)).unwrap();
    assert!(t.layer_norm(&x, 1, 1e-5).is_err());
}
#[test]
fn layer_norm_extreme_statistics_errors_and_checked_parameters() {
    use kernix::neural::{layer_norm, layer_norm_backward};
    let input = values(&[2, 3], 0);
    let gradient = Tensor::ones([2, 3]).unwrap();
    for rank in [0, 3, usize::MAX] {
        assert!(layer_norm(&input, rank, 1e-5).is_err());
    }
    for eps in [0.0, -1.0, f64::NAN, f64::INFINITY] {
        assert!(layer_norm(&input, 1, eps).is_err());
    }
    assert!(layer_norm(&values(&[], 0), 1, 1e-5).is_err());
    assert!(layer_norm_backward(&input, &Tensor::ones([3]).unwrap(), 1, 1e-5).is_err());
    for v in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert!(layer_norm(&Tensor::full([3], v).unwrap(), 1, 1e-5).is_err());
        assert!(layer_norm_backward(&input, &Tensor::full([2, 3], v).unwrap(), 1, 1e-5).is_err());
    }
    // Previously variance overflow could turn finite, nonconstant data into zeros.
    assert!(
        layer_norm(
            &Tensor::from_values([2], vec![-1e200, 1e200]).unwrap(),
            1,
            1e-5
        )
        .is_err()
    );
    // variance + epsilon overflows here, while its square root is representable.
    let wide = Tensor::from_values([2], vec![-6e153, 6e153]).unwrap();
    let out = layer_norm(&wide, 1, 1.5e308).unwrap();
    let expected = 6e153 / (1.86f64.sqrt() * 1e154);
    near(out.storage_slice()[0], -expected);
    near(out.storage_slice()[1], expected);
    let reversed = input.reverse_axis(1).unwrap();
    let seed = gradient.reverse_axis(1).unwrap();
    assert_eq!(
        layer_norm_backward(&reversed, &seed, 1, 1e-5)
            .unwrap()
            .shape(),
        input.shape()
    );
}
#[test]
fn normalized_batched_model_trains_without_custom_gradients() {
    use kernix::{
        nn::Linear,
        training::{SeededRng, Sgd},
    };
    let mut rng = SeededRng::new(27);
    let mut hidden = Linear::<f64>::new(3, 4, &mut rng).unwrap();
    let mut head = Linear::<f64>::new(4, 2, &mut rng).unwrap();
    let input = values(&[2, 3, 3], 2);
    let target = Tensor::from_values(
        [2, 3, 2],
        input
            .storage_slice()
            .chunks(3)
            .flat_map(|x| {
                [
                    0.3 * x[0] - 0.2 * x[1] + 0.1 * x[2],
                    -0.4 * x[0] + 0.1 * x[1],
                ]
            })
            .collect(),
    )
    .unwrap();
    let evaluate = |hidden: &Linear<f64>, head: &Linear<f64>| {
        let h = kernix::tanh(
            &kernix::neural::layer_norm(&hidden.forward(&input).unwrap(), 1, 1e-4).unwrap(),
        )
        .unwrap();
        kernix::neural::mse_loss(&head.forward(&h).unwrap(), &target).unwrap()
    };
    let initial = evaluate(&hidden, &head);
    for _ in 0..120 {
        let mut t = Tape::new();
        let x = t.constant(input.clone()).unwrap();
        let target = t.constant(target.clone()).unwrap();
        let h = hidden.record(&mut t, &x).unwrap();
        let norm = t.layer_norm(&h.output, 1, 1e-4).unwrap();
        let activation = t.tanh(&norm).unwrap();
        let out = head.record(&mut t, &activation).unwrap();
        let loss = t.mse(&out.output, &target).unwrap();
        let g = t.backward(&loss).unwrap();
        Sgd::new(0.05)
            .unwrap()
            .step(&mut [
                (&mut hidden.weight, g.get(&h.weight).unwrap().unwrap()),
                (&mut hidden.bias, g.get(&h.bias).unwrap().unwrap()),
                (&mut head.weight, g.get(&out.weight).unwrap().unwrap()),
                (&mut head.bias, g.get(&out.bias).unwrap().unwrap()),
            ])
            .unwrap();
    }
    let final_loss = evaluate(&hidden, &head);
    assert!(
        final_loss < initial * 0.15,
        "initial {initial}, final {final_loss}"
    );
}

#[test]
fn signed_axis_slice_vjps_and_overlapping_branches() {
    let input = values(&[2, 3, 4], 5);
    for (axis, start, len, step) in [
        (0, 1, 1, 1),
        (1, 2, 3, -1),
        (2, 0, 2, 2),
        (2, 3, 2, -2),
        (1, 3, 0, isize::MIN),
    ] {
        let mut t = Tape::new();
        let x = t.variable(input.clone()).unwrap();
        let y = t.slice_axis(&x, axis, start, len, step).unwrap();
        let s = seed(t.value(&y).unwrap().shape().dims());
        let g = t.backward_with(&y, &s).unwrap();
        for i in 0..input.numel() {
            let mut hi = input.clone();
            let mut lo = input.clone();
            hi.storage_slice_mut()[i] += 1e-6;
            lo.storage_slice_mut()[i] -= 1e-6;
            let eval = |x: &Tensor<f64>| {
                let sliced = x
                    .slice_axis(axis, start, len, step)
                    .unwrap()
                    .to_owned_contiguous(kernix::Order::C)
                    .unwrap();
                objective(&sliced, &s)
            };
            near(
                g.get(&x).unwrap().unwrap().storage_slice()[i],
                (eval(&hi) - eval(&lo)) / 2e-6,
            );
        }
    }
    let mut t = Tape::new();
    let x = t.variable(values(&[5], 0)).unwrap();
    let a = t.slice_axis(&x, 0, 0, 3, 1).unwrap();
    let b = t.slice_axis(&x, 0, 4, 3, -1).unwrap();
    let a = t.sum(&a).unwrap();
    let b = t.sum(&b).unwrap();
    let loss = t.add(&a, &b).unwrap();
    let g = t.backward(&loss).unwrap();
    assert_eq!(
        g.get(&x).unwrap().unwrap().storage_slice(),
        &[1.0, 1.0, 2.0, 1.0, 1.0]
    );
}
#[test]
fn slice_errors_and_empty_inputs_preserve_checked_semantics() {
    let mut t = Tape::new();
    let x = t.variable(values(&[2, 3], 0)).unwrap();
    let count = t.len();
    for (axis, start, len, step) in [
        (2, 0, 1, 1),
        (0, 0, 1, 0),
        (0, 2, 1, 1),
        (1, 0, 2, -1),
        (1, 0, usize::MAX, 2),
    ] {
        assert!(t.slice_axis(&x, axis, start, len, step).is_err());
    }
    assert_eq!(t.len(), count);
    let empty = t.variable(values(&[2, 0, 3], 0)).unwrap();
    let y = t.slice_axis(&empty, 1, 0, 0, -1).unwrap();
    let loss = t.sum(&y).unwrap();
    assert!(
        t.backward(&loss)
            .unwrap()
            .get(&empty)
            .unwrap()
            .unwrap()
            .is_empty()
    );
    let scalar = t.variable(values(&[], 0)).unwrap();
    assert!(t.slice_axis(&scalar, 0, 0, 1, 1).is_err());
}
