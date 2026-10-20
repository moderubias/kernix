//! Independent Candle graphs validate the generalized Tape rules outside timings.
use candle_core::{Device, Tensor as C, Var};
use kernix::{Tensor, autograd::Tape};
fn data(dims: &[usize], offset: usize) -> Vec<f64> {
    (0..dims.iter().product())
        .map(|i| ((i * 7 + offset) % 19) as f64 / 13.0 - 0.6)
        .collect()
}
fn close(a: &[f64], b: &[f64]) {
    assert_eq!(a.len(), b.len());
    for (&a, &b) in a.iter().zip(b) {
        assert!(
            (a - b).abs() < 3e-11 * (1.0 + a.abs() + b.abs()),
            "Kernix {a}, Candle {b}"
        );
    }
}
#[test]
fn batched_and_vector_vjps_match_independent_candle_graphs() {
    for (ad, bd, out) in [
        (&[3][..], &[3][..], &[][..]),
        (&[3], &[3, 2], &[2]),
        (&[2, 3], &[3], &[2]),
        (&[3], &[2, 3, 2], &[2, 2]),
        (&[2, 2, 3], &[3], &[2, 2]),
        (&[2, 1, 2, 3], &[1, 3, 3, 2], &[2, 3, 2, 2]),
        (&[2, 1, 2, 3], &[3, 3, 2], &[2, 3, 2, 2]),
        (&[2, 3], &[4, 3, 2], &[4, 2, 2]),
        (&[4, 2, 3], &[3, 2], &[4, 2, 2]),
    ] {
        let a = data(ad, 1);
        let b = data(bd, 5);
        let s = data(out, 3);
        let mut t = Tape::new();
        let av = t
            .variable(Tensor::from_values(ad, a.clone()).unwrap())
            .unwrap();
        let bv = t
            .variable(Tensor::from_values(bd, b.clone()).unwrap())
            .unwrap();
        let y = t.matmul(&av, &bv).unwrap();
        let g = t
            .backward_with(&y, &Tensor::from_values(out, s.clone()).unwrap())
            .unwrap();
        let ca = Var::from_vec(a, ad, &Device::Cpu).unwrap();
        let cb = Var::from_vec(b, bd, &Device::Cpu).unwrap();
        // Candle requires matrices: reshape views encode the same documented vector promotion.
        let left = if ad.len() == 1 {
            ca.reshape((1, ad[0])).unwrap()
        } else {
            ca.as_tensor().clone()
        };
        let right = if bd.len() == 1 {
            cb.reshape((bd[0], 1)).unwrap()
        } else {
            cb.as_tensor().clone()
        };
        let cy = left.broadcast_matmul(&right).unwrap().reshape(out).unwrap();
        let loss = (&cy * &C::from_vec(s, out, &Device::Cpu).unwrap())
            .unwrap()
            .sum_all()
            .unwrap();
        let cg = loss.backward().unwrap();
        close(
            t.value(&y).unwrap().storage_slice(),
            &cy.flatten_all().unwrap().to_vec1::<f64>().unwrap(),
        );
        for (k, c) in [(&av, &ca), (&bv, &cb)] {
            close(
                g.get(k).unwrap().unwrap().storage_slice(),
                &cg.get(c)
                    .unwrap()
                    .flatten_all()
                    .unwrap()
                    .to_vec1::<f64>()
                    .unwrap(),
            );
        }
    }
}
#[test]
fn normalization_affine_vjps_match_independent_candle_graphs() {
    for rank in [1, 2, 3] {
        let dims = &[2, 2, 3][..];
        let a = data(dims, 4);
        let w = data(&[3], 7);
        let b = data(&[1, 1, 3], 9);
        let s = data(dims, 3);
        let mut t = Tape::new();
        let av = t
            .variable(Tensor::from_values(dims, a.clone()).unwrap())
            .unwrap();
        let wv = t
            .variable(Tensor::from_values([3], w.clone()).unwrap())
            .unwrap();
        let bv = t
            .variable(Tensor::from_values([1, 1, 3], b.clone()).unwrap())
            .unwrap();
        let norm = t.layer_norm(&av, rank, 1e-4).unwrap();
        let scaled = t.mul(&norm, &wv).unwrap();
        let y = t.add(&scaled, &bv).unwrap();
        let g = t
            .backward_with(&y, &Tensor::from_values(dims, s.clone()).unwrap())
            .unwrap();
        let ca = Var::from_vec(a, dims, &Device::Cpu).unwrap();
        let cw = Var::from_vec(w, 3, &Device::Cpu).unwrap();
        let cb = Var::from_vec(b, (1, 1, 3), &Device::Cpu).unwrap();
        let axes: Vec<_> = ((3 - rank)..3).collect();
        let mean = ca.mean_keepdim(axes.as_slice()).unwrap();
        let centered = ca.broadcast_sub(&mean).unwrap();
        let variance = centered
            .sqr()
            .unwrap()
            .mean_keepdim(axes.as_slice())
            .unwrap();
        let denom = (variance + 1e-4).unwrap().sqrt().unwrap();
        let cy = centered
            .broadcast_div(&denom)
            .unwrap()
            .broadcast_mul(&cw)
            .unwrap()
            .broadcast_add(&cb)
            .unwrap();
        let loss = (&cy * &C::from_vec(s, dims, &Device::Cpu).unwrap())
            .unwrap()
            .sum_all()
            .unwrap();
        let cg = loss.backward().unwrap();
        for (k, c) in [(&av, &ca), (&wv, &cw), (&bv, &cb)] {
            close(
                g.get(k).unwrap().unwrap().storage_slice(),
                &cg.get(c)
                    .unwrap()
                    .flatten_all()
                    .unwrap()
                    .to_vec1::<f64>()
                    .unwrap(),
            );
        }
    }
}
#[test]
fn axis_reduction_vjps_match_independent_candle_graphs() {
    let dims = &[2, 3, 2][..];
    for axes in [vec![0usize], vec![2, 0], vec![0, 1, 2]] {
        for mean in [false, true] {
            for keep in [false, true] {
                let a = data(dims, 4);
                let mut t = Tape::new();
                let x = t
                    .variable(Tensor::from_values(dims, a.clone()).unwrap())
                    .unwrap();
                let signed: Vec<_> = axes.iter().map(|&x| x as isize).collect();
                let y = if mean {
                    t.mean_axes(&x, &signed, keep)
                } else {
                    t.sum_axes(&x, &signed, keep)
                }
                .unwrap();
                let s = data(t.value(&y).unwrap().shape().dims(), 3);
                let seed =
                    Tensor::from_values(t.value(&y).unwrap().shape().dims(), s.clone()).unwrap();
                let g = t.backward_with(&y, &seed).unwrap();
                let c = Var::from_vec(a, dims, &Device::Cpu).unwrap();
                let cy = match (mean, keep) {
                    (false, false) => c.sum(axes.as_slice()),
                    (false, true) => c.sum_keepdim(axes.as_slice()),
                    (true, false) => c.mean(axes.as_slice()),
                    (true, true) => c.mean_keepdim(axes.as_slice()),
                }
                .unwrap();
                let loss = (&cy * &C::from_vec(s, cy.shape(), &Device::Cpu).unwrap())
                    .unwrap()
                    .sum_all()
                    .unwrap();
                let cg = loss.backward().unwrap();
                close(
                    g.get(&x).unwrap().unwrap().storage_slice(),
                    &cg.get(&c)
                        .unwrap()
                        .flatten_all()
                        .unwrap()
                        .to_vec1::<f64>()
                        .unwrap(),
                );
            }
        }
    }
}
