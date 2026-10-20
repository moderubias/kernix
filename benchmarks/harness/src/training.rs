//! Stage-specific eager training costs. Fixtures/oracles are outside timing.
use super::measure;
use candle_core::{Device, Tensor as C, Var};
use kernix::{
    Tensor,
    autograd::{Tape, Variable},
    training::{Parameter, Sgd},
};
use serde_json::{Value, json};
const EPS: f64 = 1e-3;
struct Fixture {
    x: Vec<f64>,
    w: Vec<f64>,
    b: Vec<f64>,
    target: Vec<f64>,
}
impl Fixture {
    fn new() -> Self {
        Self {
            x: (0..64)
                .map(|i| ((i * 7 % 31) as f64 - 15.0) / 31.0)
                .collect(),
            w: (0..32)
                .map(|i| ((i * 11 % 29) as f64 - 14.0) / 29.0)
                .collect(),
            b: (0..4).map(|i| i as f64 / 17.0 - 0.1).collect(),
            target: (0..16)
                .map(|i| ((i * 13 % 23) as f64 - 11.0) / 23.0)
                .collect(),
        }
    }
    fn loss(&self, w: &[f64], b: &[f64]) -> f64 {
        let mut loss = 0.0;
        for row in 0..8 {
            let mut y = [0.0; 4];
            for j in 0..4 {
                y[j] = b[j]
                    + (0..8)
                        .map(|k| self.x[row * 8 + k] * w[k * 4 + j])
                        .sum::<f64>();
            }
            let mean = y.iter().sum::<f64>() / 4.0;
            let variance = y.iter().map(|x| (x - mean) * (x - mean)).sum::<f64>() / 4.0;
            for (output, j) in [0, 2].into_iter().enumerate() {
                let d = ((y[j] - mean) / (variance + EPS).sqrt()).tanh()
                    - self.target[row * 2 + output];
                loss += d * d / 16.0;
            }
        }
        loss
    }
    fn gradients(&self) -> (Vec<f64>, Vec<f64>) {
        let h = 1e-6;
        let mut w = self.w.clone();
        let mut dw = vec![0.0; 32];
        for i in 0..32 {
            w[i] += h;
            let hi = self.loss(&w, &self.b);
            w[i] -= 2.0 * h;
            let lo = self.loss(&w, &self.b);
            w[i] = self.w[i];
            dw[i] = (hi - lo) / (2.0 * h);
        }
        let mut b = self.b.clone();
        let mut db = vec![0.0; 4];
        for i in 0..4 {
            b[i] += h;
            let hi = self.loss(&self.w, &b);
            b[i] -= 2.0 * h;
            let lo = self.loss(&self.w, &b);
            b[i] = self.b[i];
            db[i] = (hi - lo) / (2.0 * h);
        }
        (dw, db)
    }
}
fn check_scalar(actual: f64, expected: f64) -> Value {
    let error = (actual - expected).abs();
    assert!(error < 2e-12);
    json!({"loss_absolute_error":error,"budget":2e-12})
}
fn check_gradients(dw: &[f64], db: &[f64], expected: &(Vec<f64>, Vec<f64>)) -> Value {
    let mut error = 0.0f64;
    for (&x, &y) in dw
        .iter()
        .chain(db)
        .zip(expected.0.iter().chain(&expected.1))
    {
        error = error.max((x - y).abs());
    }
    assert!(error < 3e-8);
    json!({"gradient_max_absolute_error":error,"central_difference_step":1e-6,"budget":3e-8})
}
struct KFixture {
    x: Tensor<f64>,
    w: Tensor<f64>,
    b: Tensor<f64>,
    target: Tensor<f64>,
}
impl KFixture {
    fn new(f: &Fixture) -> Self {
        Self {
            x: Tensor::from_values([2, 4, 8], f.x.clone()).unwrap(),
            w: Tensor::from_values([8, 4], f.w.clone()).unwrap(),
            b: Tensor::from_values([4], f.b.clone()).unwrap(),
            target: Tensor::from_values([2, 4, 2], f.target.clone()).unwrap(),
        }
    }
    fn forward(&self) -> f64 {
        let y = kernix::add(&kernix::matmul(&self.x, &self.w).unwrap(), &self.b).unwrap();
        let y = kernix::tanh(&kernix::neural::layer_norm(&y, 1, EPS).unwrap()).unwrap();
        kernix::neural::mse_loss(&y.slice_axis(2, 0, 2, 2).unwrap(), &self.target).unwrap()
    }
    fn record(&self) -> KGraph {
        let mut tape = Tape::new();
        let x = tape.constant(self.x.clone()).unwrap();
        let w = tape.variable(self.w.clone()).unwrap();
        let b = tape.variable(self.b.clone()).unwrap();
        let target = tape.constant(self.target.clone()).unwrap();
        let y = tape.matmul(&x, &w).unwrap();
        let y = tape.add(&y, &b).unwrap();
        let y = tape.layer_norm(&y, 1, EPS).unwrap();
        let y = tape.tanh(&y).unwrap();
        let y = tape.slice_axis(&y, 2, 0, 2, 2).unwrap();
        let loss = tape.mse(&y, &target).unwrap();
        KGraph { tape, w, b, loss }
    }
}
struct KGraph {
    tape: Tape<f64>,
    w: Variable,
    b: Variable,
    loss: Variable,
}
struct CFixture {
    x: C,
    w: C,
    b: C,
    target: C,
    indices: C,
}
struct CGraph {
    loss: C,
    w: Var,
    b: Var,
}
impl CFixture {
    fn new(f: &Fixture) -> Self {
        Self {
            x: C::from_vec(f.x.clone(), (2, 4, 8), &Device::Cpu).unwrap(),
            w: C::from_vec(f.w.clone(), (8, 4), &Device::Cpu).unwrap(),
            b: C::from_vec(f.b.clone(), 4, &Device::Cpu).unwrap(),
            target: C::from_vec(f.target.clone(), (2, 4, 2), &Device::Cpu).unwrap(),
            indices: C::new(&[0u32, 2], &Device::Cpu).unwrap(),
        }
    }
    fn loss(x: &C, w: &C, b: &C, target: &C, indices: &C) -> C {
        let y = x.broadcast_matmul(w).unwrap().broadcast_add(b).unwrap();
        let mean = y.mean_keepdim(2).unwrap();
        let centered = y.broadcast_sub(&mean).unwrap();
        let variance = centered.sqr().unwrap().mean_keepdim(2).unwrap();
        let denom = (variance + EPS).unwrap().sqrt().unwrap();
        let y = centered.broadcast_div(&denom).unwrap().tanh().unwrap();
        y.index_select(indices, 2)
            .unwrap()
            .sub(target)
            .unwrap()
            .sqr()
            .unwrap()
            .mean_all()
            .unwrap()
    }
    fn forward(&self) -> C {
        Self::loss(&self.x, &self.w, &self.b, &self.target, &self.indices)
    }
    fn record(&self) -> CGraph {
        let x = self.x.copy().unwrap();
        let w = Var::from_tensor(&self.w).unwrap();
        let b = Var::from_tensor(&self.b).unwrap();
        let target = self.target.copy().unwrap();
        CGraph {
            loss: Self::loss(&x, &w, &b, &target, &self.indices),
            w,
            b,
        }
    }
}
pub(super) fn run(workload: &str, library: &str, mode: &str) -> Value {
    let f = Fixture::new();
    let expected_loss = f.loss(&f.w, &f.b);
    let expected_gradients = f.gradients();
    if library == "kernix" {
        let input = KFixture::new(&f);
        match workload {
            "training_forward_2x4x8x4_f64" => measure(
                mode,
                || input.forward(),
                |out| check_scalar(*out, expected_loss),
            ),
            "training_record_2x4x8x4_f64" => measure(
                mode,
                || input.record(),
                |out| {
                    check_scalar(
                        out.tape.value(&out.loss).unwrap().storage_slice()[0],
                        expected_loss,
                    )
                },
            ),
            "training_backward_2x4x8x4_f64" => {
                let graph = input.record();
                measure(
                    mode,
                    || graph.tape.backward(&graph.loss).unwrap(),
                    |out| {
                        check_gradients(
                            out.get(&graph.w).unwrap().unwrap().storage_slice(),
                            out.get(&graph.b).unwrap().unwrap().storage_slice(),
                            &expected_gradients,
                        )
                    },
                )
            }
            "training_sgd_2x4x8x4_f64" => {
                let graph = input.record();
                let gradients = graph.tape.backward(&graph.loss).unwrap();
                let dw = gradients.get(&graph.w).unwrap().unwrap();
                let db = gradients.get(&graph.b).unwrap().unwrap();
                check_gradients(dw.storage_slice(), db.storage_slice(), &expected_gradients);
                let mut w = Parameter::new(input.w).unwrap();
                let mut b = Parameter::new(input.b).unwrap();
                let sgd = Sgd::new(1e-3).unwrap();
                measure(
                    mode,
                    || {
                        sgd.step(&mut [(&mut w, dw), (&mut b, db)]).unwrap();
                        (w.value().storage_slice()[0], b.value().storage_slice()[0])
                    },
                    |&(w, b)| {
                        assert!((w - (f.w[0] - 1e-3 * expected_gradients.0[0])).abs() < 3e-11);
                        assert!((b - (f.b[0] - 1e-3 * expected_gradients.1[0])).abs() < 3e-11);
                        json!({"first_update_budget":3e-11,"gradient_budget":3e-8,"fixed_gradient":true})
                    },
                )
            }
            _ => panic!("unknown training stage"),
        }
    } else if library == "candle" {
        let input = CFixture::new(&f);
        match workload {
            "training_forward_2x4x8x4_f64" => measure(
                mode,
                || input.forward(),
                |out| check_scalar(out.to_scalar::<f64>().unwrap(), expected_loss),
            ),
            "training_record_2x4x8x4_f64" => measure(
                mode,
                || input.record(),
                |out| check_scalar(out.loss.to_scalar::<f64>().unwrap(), expected_loss),
            ),
            "training_backward_2x4x8x4_f64" => {
                let graph = input.record();
                measure(
                    mode,
                    || graph.loss.backward().unwrap(),
                    |out| {
                        check_gradients(
                            &out.get(&graph.w)
                                .unwrap()
                                .flatten_all()
                                .unwrap()
                                .to_vec1::<f64>()
                                .unwrap(),
                            &out.get(&graph.b).unwrap().to_vec1::<f64>().unwrap(),
                            &expected_gradients,
                        )
                    },
                )
            }
            _ => panic!("unknown Candle training stage"),
        }
    } else {
        panic!("training comparison requires an actual autodiff implementation")
    }
}

#[cfg(all(test, not(feature = "allocation-counting")))]
mod tests {
    #[test]
    fn every_training_stage_validates_against_the_scalar_oracle() {
        for stage in ["forward", "record", "backward", "sgd"] {
            for library in ["kernix", "candle"] {
                if stage == "sgd" && library == "candle" {
                    continue;
                }
                super::run(&format!("training_{stage}_2x4x8x4_f64"), library, "first");
            }
        }
    }
}
