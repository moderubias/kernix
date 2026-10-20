//! Dense normalization must retain the checked indexed implementation's scalar policy.
use core::ops::AddAssign;
use kernix_core::tensor::{Tensor, TensorError};
use kernix_ops::{
    FloatMath, TensorInput,
    neural::{layer_norm, layer_norm_backward},
};
struct Indexed<'a, T>(&'a Tensor<T>);
impl<T> TensorInput<T> for Indexed<'_, T> {
    fn dims(&self) -> &[usize] {
        self.0.shape().dims()
    }
    fn get(&self, i: &[usize]) -> Result<&T, TensorError> {
        self.0.get(i)
    }
}
fn compare<T: FloatMath + AddAssign + core::fmt::Debug>(
    x: Tensor<T>,
    g: Tensor<T>,
    rank: usize,
    eps: T,
) {
    let dense = layer_norm(&x, rank, eps);
    let indexed = layer_norm(&Indexed(&x), rank, eps);
    match (dense, indexed) {
        (Ok(a), Ok(b)) => assert_eq!(a.storage_slice(), b.storage_slice()),
        (Err(a), Err(b)) => assert_eq!(a, b),
        other => panic!("forward {other:?}"),
    }
    let dense = layer_norm_backward(&x, &g, rank, eps);
    let indexed = layer_norm_backward(&Indexed(&x), &Indexed(&g), rank, eps);
    match (dense, indexed) {
        (Ok(a), Ok(b)) => assert_eq!(a.storage_slice(), b.storage_slice()),
        (Err(a), Err(b)) => assert_eq!(a, b),
        other => panic!("VJP {other:?}"),
    }
}
#[test]
fn dense_and_indexed_paths_match_for_both_dtypes_and_tail_ranks() {
    for dims in [
        &[1][..],
        &[3],
        &[2, 3, 4],
        &[2, 1, 3, 5],
        &[1, 1, 1, 1, 1, 1, 2],
    ] {
        let count = kernix_core::shape::Kahen::new(dims).unwrap().numel();
        let x = Tensor::from_values(
            dims,
            (0..count)
                .map(|i| ((i * 7 % 19) as f64 - 9.0) / 13.0)
                .collect(),
        )
        .unwrap();
        let g = Tensor::from_values(
            dims,
            (0..count)
                .map(|i| ((i * 3 % 11) as f64 - 5.0) / 7.0)
                .collect(),
        )
        .unwrap();
        for rank in 1..=dims.len() {
            compare(x.clone(), g.clone(), rank, 1e-5);
            compare(
                kernix_ops::map(&x, |x| *x as f32).unwrap(),
                kernix_ops::map(&g, |g| *g as f32).unwrap(),
                rank,
                1e-5f32,
            );
        }
    }
}
#[test]
fn dense_constants_extremes_and_invalid_values_match_fallback() {
    for n in [1, 3, 7, 31] {
        compare(
            Tensor::full([2, n], 1.3).unwrap(),
            Tensor::full([2, n], 1.7).unwrap(),
            1,
            1e-300,
        );
    }
    compare(
        Tensor::from_values([2], vec![-6e153, 6e153]).unwrap(),
        Tensor::from_values([2], vec![0.2, -0.4]).unwrap(),
        1,
        1.5e308,
    );
    compare(
        Tensor::from_values([2], vec![-1e200, 1e200]).unwrap(),
        Tensor::ones([2]).unwrap(),
        1,
        1e-5,
    );
    for bad in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        compare(
            Tensor::full([2, 3], bad).unwrap(),
            Tensor::ones([2, 3]).unwrap(),
            1,
            1e-5,
        );
        compare(
            Tensor::ones([2, 3]).unwrap(),
            Tensor::full([2, 3], bad).unwrap(),
            1,
            1e-5,
        );
    }
    for dims in [&[0, 3][..], &[2, 0, 3], &[2, 0], &[]] {
        compare(
            Tensor::zeros(dims).unwrap(),
            Tensor::ones(dims).unwrap(),
            1,
            1e-5,
        );
    }
}
