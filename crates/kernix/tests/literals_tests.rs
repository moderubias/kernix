//! Downstream macro hygiene, ownership and fallible construction contracts.
use kernix::prelude::*;

#[test]
fn literals_compose_with_linear_algebra() -> Result<(), Box<dyn std::error::Error>> {
    let a = matrix![4.0f64, 1.0; 1.0, 3.0;]?;
    let b = tensor![1.0, 2.0,]?;
    let x = solve(&a, &b)?;
    assert!(residual_norm(&a, &x, &b)? < 1e-14);
    assert_eq!(shape![2, 2,]?.dims(), a.shape().dims());
    let scalar = full!(7;)?;
    assert_eq!(scalar.shape().rank(), 0);
    assert_eq!(scalar.get(&[])?, &7);
    assert_eq!(tensor![9; 3]?.storage_slice(), &[9; 3]);
    Ok(())
}

#[test]
fn constructors_handle_empty_and_high_rank_shapes() -> Result<(), TensorError> {
    let empty: Tensor<f64> = tensor![]?;
    assert_eq!(empty.shape().dims(), &[0]);
    let empty = full!(0u8; 2, 0, 3)?;
    assert_eq!(empty.numel(), 0);
    let rows: [[i32; 0]; 2] = [[], []];
    assert_eq!(Tensor::from_rows(rows)?.shape().dims(), &[2, 0]);
    let rows: [[i32; 3]; 0] = [];
    assert_eq!(Tensor::from_rows(rows)?.shape().dims(), &[0, 3]);
    let high = full!(1; 1, 1, 1, 1, 1, 1, 1, 2)?;
    assert_eq!(high.shape().rank(), 8);
    assert_eq!(high.storage_slice(), &[1, 1]);
    Ok(())
}

#[test]
fn inputs_are_evaluated_once_and_elements_are_moved() -> Result<(), TensorError> {
    #[derive(Debug, PartialEq)]
    struct Owned(u32); // Deliberately neither Clone nor Copy.
    let mut calls = 0;
    let mut next = || {
        calls += 1;
        Owned(calls)
    };
    let a = matrix![next(), next(); next(), next()]?;
    assert_eq!(calls, 4);
    assert_eq!(a.storage_slice(), &[Owned(1), Owned(2), Owned(3), Owned(4)]);
    let v = tensor![Owned(5), Owned(6)]?;
    assert_eq!(v.storage_slice(), &[Owned(5), Owned(6)]);
    let mut calls = 0;
    let a = full!({ calls += 1; String::from("x") }; { calls += 1; 2 }, 3)?;
    assert_eq!(calls, 2);
    assert_eq!(a.numel(), 6);
    let values = vec![1, 2, 3, 4];
    let address = values.as_ptr();
    let a = tensor!(shape = [2, 2]; values = values)?;
    assert_eq!(a.storage_slice().as_ptr(), address);
    Ok(())
}

#[test]
fn invalid_shapes_and_counts_are_errors_without_panics() {
    assert!(shape![usize::MAX, 2].is_err());
    assert!(full!(0u8; usize::MAX, 2).is_err());
    assert!(full!(0u64; isize::MAX as usize).is_err());
    assert!(matches!(
        tensor!(shape = [2, 3]; values = vec![1, 2]),
        Err(TensorError::ElementCountMismatch {
            expected: 6,
            actual: 2
        })
    ));
    assert!(tensor![()].is_err());
    assert!(full!(() ; 0).is_err());
}

#[test]
fn qualified_macros_need_no_imports_and_do_not_capture_names() {
    let count = 4;
    let values = 7;
    let a = kernix::tensor![values; count].unwrap();
    assert_eq!(a.storage_slice(), &[7; 4]);
    assert_eq!(values, 7);
    assert_eq!(count, 4);
    let a = kernix::matrix![1, 2; 3, 4].unwrap();
    assert_eq!(a.storage_slice(), &[1, 2, 3, 4]);
    assert_eq!(kernix::shape![].unwrap().numel(), 1);
}
