//! Equivalence of serial/parallel and strided CPU kernel paths.
use kernix_core::{Order, shape::Kahen, tensor::Tensor};
use kernix_ops::{TensorInput, execution::CpuExecutor, log_softmax, matmul, softmax};
struct Indexed<'a>(&'a Tensor<f64>);
impl TensorInput<f64> for Indexed<'_> {
    fn dims(&self) -> &[usize] {
        self.0.shape().dims()
    }
    fn get(&self, i: &[usize]) -> Result<&f64, kernix_core::tensor::TensorError> {
        self.0.get(i)
    }
}
fn tensor(m: usize, n: usize) -> Tensor<f64> {
    Tensor::from_vec(
        (0..m * n)
            .map(|i| ((i * 13 % 47) as f64 - 23.0) / 11.0)
            .collect(),
        Kahen::new([m, n]).unwrap(),
        Order::C,
    )
    .unwrap()
}
#[test]
fn locality_optimization_preserves_scalar_accumulation_bits() {
    for (m, k, n) in [(3, 7, 5), (0, 2, 3), (3, 0, 5), (2, 3, 0), (17, 31, 19)] {
        let a = tensor(m, k);
        let b = tensor(k, n);
        let fast = matmul(&a, &b).unwrap();
        let slow = matmul(&Indexed(&a), &Indexed(&b)).unwrap();
        assert_eq!(fast.storage_slice(), slow.storage_slice());
    }
}
#[test]
fn contiguous_softmax_matches_indexed_path_exactly() {
    let a = tensor(7, 11);
    assert_eq!(
        softmax(&a, -1).unwrap().storage_slice(),
        softmax(&Indexed(&a), -1).unwrap().storage_slice()
    );
    assert_eq!(
        log_softmax(&a, -1).unwrap().storage_slice(),
        log_softmax(&Indexed(&a), -1).unwrap().storage_slice()
    );
    let mut bad = a.clone();
    *bad.get_mut(&[3, 7]).unwrap() = f64::NAN;
    assert_eq!(
        softmax(&bad, -1).unwrap_err(),
        softmax(&Indexed(&bad), -1).unwrap_err()
    );
}
#[test]
fn explicit_serial_executor_and_bad_settings() {
    assert!(CpuExecutor::new(0).is_err());
    let a = tensor(3, 4);
    let b = tensor(4, 2);
    assert_eq!(
        CpuExecutor::new(1)
            .unwrap()
            .matmul(&a, &b)
            .unwrap()
            .storage_slice(),
        matmul(&a, &b).unwrap().storage_slice()
    );
}
#[cfg(feature = "parallel")]
#[test]
fn private_pool_keeps_output_bits_and_arbitrary_layout_fallback() {
    let cpu = CpuExecutor::new(2).unwrap();
    assert_eq!(cpu.threads(), 2);
    let a = tensor(17, 31);
    let b = tensor(31, 19);
    assert_eq!(
        cpu.matmul(&a, &b).unwrap().storage_slice(),
        matmul(&Indexed(&a), &Indexed(&b)).unwrap().storage_slice()
    );
    let t = a.transpose().unwrap();
    assert_eq!(
        cpu.matmul(&t, &a).unwrap().storage_slice(),
        matmul(&t, &a).unwrap().storage_slice()
    );
}

#[test]
fn reusable_output_keeps_allocation_and_errors_do_not_write() {
    let cpu = CpuExecutor::default();
    let a = tensor(3, 4);
    let b = tensor(4, 2);
    let mut out = tensor(3, 2);
    let ptr = out.storage_slice().as_ptr();
    cpu.matmul_into(&a, &b, &mut out).unwrap();
    assert_eq!(out.storage_slice().as_ptr(), ptr);
    assert_eq!(out.storage_slice(), matmul(&a, &b).unwrap().storage_slice());
    let before = out.storage_slice().to_vec();
    assert!(cpu.matmul_into(&a, &a, &mut out).is_err());
    assert_eq!(out.storage_slice(), before);
}
