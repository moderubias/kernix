//! Identity construction validates before generation and needs no Clone/Default.
use kernix_core::{
    Order,
    scalar::{One, Zero},
    tensor::Tensor,
};
use std::sync::atomic::{AtomicUsize, Ordering};

#[test]
fn numeric_order_scalar_empty_high_rank_and_inference() {
    let a: Tensor<f32> = Tensor::zeros([2, 3]).unwrap();
    assert_eq!(a.storage_slice(), &[0.0; 6]);
    assert!(a.layout().is_c_contiguous());
    assert_eq!(a.strides(), &[3, 1]);
    let ones = Tensor::<i16>::ones([2, 3]).unwrap();
    assert_eq!(ones.storage_slice(), &[1; 6]);
    assert_eq!(Tensor::<u8>::zeros([]).unwrap().get(&[]).unwrap(), &0);
    assert_eq!(Tensor::<f64>::ones([]).unwrap().storage_slice(), &[1.0]);
    assert_eq!(Tensor::<f64>::zeros([2, 0, 3]).unwrap().numel(), 0);
    assert_eq!(
        Tensor::<u8>::ones([1, 1, 1, 1, 1, 1, 1, 2])
            .unwrap()
            .shape()
            .rank(),
        8
    );
    assert_eq!(ones.as_contiguous_slice(Order::C).unwrap(), &[1; 6]);
}

#[test]
fn non_clone_custom_identities_are_generated_after_validation() {
    #[derive(Debug, PartialEq)]
    struct Scalar(i64); // No Clone, Default or arithmetic traits.
    static CALLS: AtomicUsize = AtomicUsize::new(0);
    impl Zero for Scalar {
        fn zero() -> Self {
            CALLS.fetch_add(1, Ordering::SeqCst);
            Self(0)
        }
    }
    impl One for Scalar {
        fn one() -> Self {
            CALLS.fetch_add(1, Ordering::SeqCst);
            Self(1)
        }
    }
    assert!(Tensor::<Scalar>::zeros([usize::MAX, 2]).is_err());
    assert!(Tensor::<Scalar>::ones([usize::MAX]).is_err());
    assert_eq!(CALLS.load(Ordering::SeqCst), 0);
    assert_eq!(Tensor::<Scalar>::zeros([2, 0]).unwrap().numel(), 0);
    assert_eq!(CALLS.load(Ordering::SeqCst), 0);
    let a = Tensor::<Scalar>::zeros([2]).unwrap();
    let b = Tensor::<Scalar>::ones([]).unwrap();
    assert_eq!(CALLS.load(Ordering::SeqCst), 3);
    assert_eq!(a.storage_slice(), &[Scalar(0), Scalar(0)]);
    assert_eq!(b.storage_slice(), &[Scalar(1)]);
}

#[test]
fn zero_sized_elements_and_resource_overflow_remain_errors() {
    struct Empty;
    impl Zero for Empty {
        fn zero() -> Self {
            panic!("validation must happen first")
        }
    }
    impl One for Empty {
        fn one() -> Self {
            panic!("validation must happen first")
        }
    }
    assert!(Tensor::<Empty>::zeros([0]).is_err());
    assert!(Tensor::<Empty>::ones([]).is_err());
    assert!(Tensor::<u64>::zeros([isize::MAX as usize]).is_err());
    assert!(Tensor::<f64>::ones([usize::MAX, 2]).is_err());
}
