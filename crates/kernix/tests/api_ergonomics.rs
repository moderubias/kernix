//! Executable ordinary workflows; explicit costs and existing literal semantics.
use kernix::{CpuExecutor, prelude::*};
type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

#[test]
fn vector_matrix_arithmetic_and_broadcasting() -> Result {
    let a = matrix![1.0f32,2.0,3.0;4.0,5.0,6.0]?;
    let bias = tensor![0.5, 1.0, -0.5]?;
    let output = mul(&add(&a, &bias)?, &tensor![2.0]?)?;
    assert_eq!(output.storage_slice(), &[3.0, 6.0, 5.0, 9.0, 12.0, 11.0]);
    Ok(())
}
#[test]
fn higher_rank_creation_and_zero_copy_reshape() -> Result {
    let volume = Tensor::from_values([2, 2, 2], (1..=8).collect())?;
    let matrix = volume.reshape_view([4, 2], Order::C)?;
    assert_eq!(matrix.get(&[3, 1])?, &8);
    assert_eq!(
        matrix.storage_slice().as_ptr(),
        volume.storage_slice().as_ptr()
    );
    Ok(())
}
#[test]
fn batched_forward_matmul() -> Result {
    let a = Tensor::<f32>::ones([2, 2, 3])?;
    let b = Tensor::<f32>::ones([3, 2])?;
    let c = matmul(&a, &b)?;
    assert_eq!(c.shape().dims(), &[2, 2, 2]);
    assert_eq!(c.storage_slice(), &[3.0; 8]);
    Ok(())
}
#[test]
fn statistical_reductions() -> Result {
    let a = matrix![1.0f64,2.0,3.0;4.0,5.0,6.0]?;
    assert_eq!(
        kernix::mean_axes(&a, &[0], false)?.storage_slice(),
        &[2.5, 3.5, 4.5]
    );
    assert_eq!(kernix::mean_all(&a)?, 3.5);
    assert!((kernix::variance_all(&a, 0)? - 35.0 / 12.0).abs() < 1e-14);
    Ok(())
}
#[test]
fn linear_system_with_residual() -> Result {
    let a = matrix![4.0f64,1.0;1.0,3.0]?;
    let b = tensor![1.0, 2.0]?;
    let x = solve(&a, &b)?;
    assert!(residual_norm(&a, &x, &b)? < 1e-14);
    Ok(())
}
#[test]
fn small_neural_forward_pass() -> Result {
    let mut rng = kernix::training::SeededRng::new(7);
    let hidden = kernix::nn::Linear::<f32>::new(3, 4, &mut rng)?;
    let output = kernix::nn::Linear::<f32>::new(4, 2, &mut rng)?;
    let input = Tensor::<f32>::ones([5, 3])?;
    let prediction = output.forward(&kernix::relu(&hidden.forward(&input)?)?)?;
    assert_eq!(prediction.shape().dims(), &[5, 2]);
    Ok(())
}
#[test]
fn ordinary_autodiff_parameter_update() -> Result {
    use kernix::{
        autograd::Tape,
        training::{Parameter, Sgd},
    };
    let x = matrix![1.0f64, 1.0]?;
    let target = Tensor::<f64>::zeros([1, 1])?;
    let mut weight = Parameter::new(Tensor::full([2, 1], 0.5)?)?;
    let before = kernix::neural::mse_loss(&matmul(&x, weight.value())?, &target)?;
    let mut tape = Tape::new();
    let input = tape.constant(x.clone())?;
    let w = weight.track(&mut tape)?;
    let expected = tape.constant(target.clone())?;
    let prediction = tape.matmul(&input, &w)?;
    let loss = tape.mse(&prediction, &expected)?;
    let gradients = tape.backward(&loss)?;
    Sgd::new(0.1)?.step(&mut [(&mut weight, gradients.get(&w)?.unwrap())])?;
    let after = kernix::neural::mse_loss(&matmul(&x, weight.value())?, &target)?;
    assert!(after < before);
    for &value in weight.value().storage_slice() {
        assert!((value - 0.3).abs() < 1e-14);
    }
    Ok(())
}
#[test]
fn transposed_and_reversed_views_share_storage() -> Result {
    let a = matrix![1,2,3;4,5,6]?;
    let transposed = a.transpose()?;
    let reversed = a.slice_axis(1, 2, 3, -1)?;
    assert_eq!(transposed.get(&[2, 1])?, &6);
    assert_eq!(reversed.get(&[0, 0])?, &3);
    assert_eq!(
        transposed.storage_slice().as_ptr(),
        a.storage_slice().as_ptr()
    );
    assert_eq!(
        reversed.storage_slice().as_ptr(),
        a.storage_slice().as_ptr()
    );
    Ok(())
}
#[test]
fn explicit_layout_policy_and_reusable_output() -> Result {
    let column_major = Tensor::from_vec(vec![1.0f64, 3.0, 2.0, 4.0], shape![2, 2]?, Order::F)?;
    assert_eq!(column_major.get(&[0, 1])?, &2.0);
    // matmul_into requires C-order; this conversion is an explicit one-time copy.
    let a = column_major.to_owned_contiguous(Order::C)?;
    let mut out = Tensor::<f64>::zeros([2, 2])?;
    let pointer = out.storage_slice().as_ptr();
    let cpu = CpuExecutor::default().with_ordered_matmul(true);
    cpu.matmul_into(&a, &a, &mut out)?;
    assert_eq!(out.storage_slice(), &[7.0, 10.0, 15.0, 22.0]);
    assert_eq!(out.storage_slice().as_ptr(), pointer);
    Ok(())
}
#[test]
fn array_elements_and_legacy_scalar_trait_paths_are_preserved() -> Result {
    let pixels = tensor![[255u8, 0, 0], [0, 255, 0]]?;
    assert_eq!(pixels.shape().dims(), &[2]);
    assert_eq!(pixels.get(&[1])?, &[0, 255, 0]);
    let matrix_of_arrays = matrix![[1, 2], [3, 4]]?;
    assert_eq!(matrix_of_arrays.shape().dims(), &[1, 2]);
    struct Custom(i32);
    impl kernix::ops::Zero for Custom {
        fn zero() -> Self {
            Self(0)
        }
    }
    impl kernix::One for Custom {
        fn one() -> Self {
            Self(1)
        }
    }
    let pairs = tensor![[Custom(1), Custom(2)], [Custom(3), Custom(4)]]?;
    assert_eq!(pairs.shape().dims(), &[2]);
    assert_eq!(pairs.get(&[1])?[0].0, 3);
    assert_eq!(Tensor::<Custom>::zeros([])?.get(&[])?.0, 0);
    assert_eq!(Tensor::<Custom>::ones([])?.get(&[])?.0, 1);
    Ok(())
}
#[test]
fn macro_arguments_are_evaluated_once_in_existing_order() -> Result {
    let trace = std::cell::RefCell::new(Vec::new());
    let a = full!({trace.borrow_mut().push("value");3};{trace.borrow_mut().push("dimension");2})?;
    assert_eq!(*trace.borrow(), ["dimension", "value"]);
    trace.borrow_mut().clear();
    let b = tensor![{trace.borrow_mut().push("value");3};{trace.borrow_mut().push("count");2}]?;
    assert_eq!(*trace.borrow(), ["count", "value"]);
    assert_eq!(a.storage_slice(), b.storage_slice());
    Ok(())
}
