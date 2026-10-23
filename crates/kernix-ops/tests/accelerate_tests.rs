//! Explicit optional vendor backend, never a substitute for portable CPU evidence.
#![cfg(feature = "accelerate")]
use kernix_core::{Order, shape::Kahen, tensor::Tensor};
use kernix_ops::accelerate::AccelerateExecutor;
#[test]
fn availability_is_explicit() {
    if !AccelerateExecutor::is_available() {
        assert!(AccelerateExecutor::new().is_err());
    } else {
        assert!(AccelerateExecutor::new().is_ok());
    }
}
#[test]
fn sdk_coordinates_layout_errors_and_empty_extents() {
    let Ok(cpu) = AccelerateExecutor::new() else {
        return;
    };
    for (m, k, n) in [
        (1, 1, 1),
        (64, 65, 67),
        (128, 128, 128),
        (129, 67, 131),
        (256, 128, 64),
        (0, 7, 3),
        (3, 0, 5),
    ] {
        let a: Vec<f64> = (0..m * k).map(|i| (i % 23) as f64 / 16. - 0.5).collect();
        let b: Vec<f64> = (0..k * n).map(|i| (i % 13) as f64 / 8. - 0.5).collect();
        let expected: Vec<f64> = (0..m * n)
            .map(|i| (0..k).map(|p| a[i / n * k + p] * b[p * n + i % n]).sum())
            .collect();
        let at = Tensor::from_values([m, k], a.clone()).unwrap();
        let bt = Tensor::from_values([k, n], b.clone()).unwrap();
        assert_eq!(cpu.matmul(&at, &bt).unwrap().storage_slice(), expected);
        let af = Tensor::from_values([m, k], a.iter().map(|&v| v as f32).collect()).unwrap();
        let bf = Tensor::from_values([k, n], b.iter().map(|&v| v as f32).collect()).unwrap();
        assert_eq!(
            cpu.matmul(&af, &bf).unwrap().storage_slice(),
            expected.iter().map(|&v| v as f32).collect::<Vec<_>>()
        );
        assert_eq!(at.storage_slice(), a);
    }
    let a = Tensor::<f64>::ones([3]).unwrap();
    assert!(cpu.matmul(&a, &a).is_err());
    let a = Tensor::<f64>::ones([2, 3]).unwrap();
    assert!(cpu.matmul(&a, &a).is_err());
    let f = Tensor::from_fn(Kahen::new([2, 2]).unwrap(), Order::F, 8, |_| 1.0f64).unwrap();
    assert!(cpu.matmul(&f, &f).is_err());
    let huge = Tensor::<f64>::zeros([isize::MAX as usize, 0]).unwrap();
    let zero = Tensor::<f64>::zeros([0, 0]).unwrap();
    assert_eq!(cpu.matmul(&huge, &zero).unwrap().numel(), 0);
}
