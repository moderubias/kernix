//! Explicit two-worker reusable output and generic tensor contraction fixtures.
use super::measure;
use kernix::{CpuExecutor, Tensor};
use serde_json::{Value, json};
use std::cell::RefCell;

pub(super) fn parallel(mode: &str, n: usize, single: bool) -> Value {
    macro_rules! run {
        ($ty:ty) => {{
            let a: Vec<$ty> = (0..n*n).map(|i| (i%23) as $ty /16. -0.5).collect();
            let b: Vec<$ty> = (0..n*n).map(|i| (i%13) as $ty /8. -0.5).collect();
            let expected: Vec<f64> = (0..n*n).map(|p| (0..n).map(|k|
                f64::from(a[(p/n)*n+k])*f64::from(b[k*n+p%n])).sum()).collect();
            let a=Tensor::from_values([n,n],a).unwrap();
            let b=Tensor::from_values([n,n],b).unwrap();
            let output=RefCell::new(Tensor::<$ty>::zeros([n,n]).unwrap());
            let cpu=CpuExecutor::new(2).unwrap();
            measure(mode,|| cpu.matmul_into(&a,&b,&mut output.borrow_mut()).unwrap(),|_| {
                let error=output.borrow().storage_slice().iter().zip(&expected)
                    .map(|(&x,&y)|(f64::from(x)-y).abs()).fold(0.,f64::max);
                assert_eq!(error,0.);
                json!({"coordinate_max_absolute_error":error,"budget":0.,"workers":2,"reused_output":true})
            })
        }}
    }
    if single { run!(f32) } else { run!(f64) }
}

pub(super) fn contraction(mode: &str) -> Value {
    let a: Vec<f64> = (0..16 * 32 * 8)
        .map(|i| (i % 23) as f64 / 16. - 0.5)
        .collect();
    let b: Vec<f64> = (0..32 * 8 * 16)
        .map(|i| (i % 13) as f64 / 8. - 0.5)
        .collect();
    let expected: Vec<f64> = (0..16 * 16)
        .map(|p| {
            (0..256)
                .map(|k| a[(p / 16) * 256 + k] * b[k * 16 + p % 16])
                .sum()
        })
        .collect();
    let a = Tensor::from_values([16, 32, 8], a).unwrap();
    let b = Tensor::from_values([32, 8, 16], b).unwrap();
    measure(
        mode,
        || kernix::tensordot(&a, &b, &[1, 2], &[0, 1]).unwrap(),
        |out| {
            assert_eq!(out.shape().dims(), [16, 16]);
            let error = out
                .storage_slice()
                .iter()
                .zip(&expected)
                .map(|(x, y)| (x - y).abs())
                .fold(0., f64::max);
            assert_eq!(error, 0.);
            json!({"coordinate_max_absolute_error":error,"budget":0.})
        },
    )
}

#[cfg(all(test, not(feature = "allocation-counting")))]
mod tests {
    #[test]
    fn new_workers_validate_all_coordinates_before_timing() {
        super::contraction("first");
        for (n, single) in [(64, false), (128, false), (256, false), (128, true)] {
            super::parallel("first", n, single);
        }
    }
}
