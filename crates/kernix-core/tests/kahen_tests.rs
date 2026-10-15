use kernix_core::shape::{Kahen, ShapeError};

#[test]
fn kahen_new_accepts_scalar_shape() {
    assert!(Kahen::new([]).is_ok());
}

#[test]
fn kahen_new_accepts_zero_sized_shape() {
    assert!(Kahen::new([0]).is_ok());
}

#[test]
fn kahen_new_accepts_normal_sized_shape() {
    assert!(Kahen::new([2, 3, 4]).is_ok());
}

#[test]
fn kahen_new_accepts_usize_max_sized_shape() {
    assert!(Kahen::new([usize::MAX]).is_ok());
}

#[test]
fn kahen_new_accepts_usize_max_mul_one_shape() {
    assert!(Kahen::new([usize::MAX, 1]).is_ok());
}

#[test]
fn kahen_new_rejects_overflow() {
    let result = Kahen::new([usize::MAX, 2]);

    assert_eq!(
        result,
        Err(ShapeError::ElementCountOverflow {
            axis: 1,
            accumulated: usize::MAX,
            dim: 2,
        })
    );
}

#[test]
fn kahen_new_accepts_usize_max_mul_zero_shape() {
    assert!(Kahen::new([usize::MAX, 2, 0]).is_ok());
}

#[test]
fn kahen_utils_scalar_dims_returns_scalar_sized_slice() {
    assert_eq!(Kahen::scalar().dims(), []);
}

#[test]
fn kahen_utils_scalar_rank_is_zero() {
    assert_eq!(Kahen::scalar().rank(), 0);
}

#[test]
fn kahen_utils_scalar_numel_returns_one() {
    assert_eq!(Kahen::scalar().numel(), 1);
}

#[test]
fn kahen_utils_new_normal_sized_dims_correct() {
    assert_eq!(Kahen::new([2, 3, 4]).unwrap().dims(), [2, 3, 4]);
}

#[test]
fn kahen_utils_new_normal_sized_rank_equals_three() {
    assert_eq!(Kahen::new([2, 3, 4]).unwrap().rank(), 3);
}

#[test]
fn kahen_utils_new_numel_equals_24() {
    assert_eq!(Kahen::new([2, 3, 4]).unwrap().numel(), 24);
}

#[test]
fn kahen_utils_new_zero_sized_numel_zero() {
    assert_eq!(Kahen::new([0]).unwrap().numel(), 0);
}

#[test]
fn kahen_utils_normal_sized_with_zero_numel_zero() {
    assert_eq!(Kahen::new([2, 0, 5]).unwrap().numel(), 0);
}

#[test]
fn kahen_utils_new_usized_max_mul_zero_numel_zero() {
    assert_eq!(Kahen::new([usize::MAX, 2, 0]).unwrap().numel(), 0);
}

#[test]
fn kahen_utils_scalar_dim_returns_err() {
    assert!(Kahen::scalar().dim(0).is_err());
}

#[test]
fn permute_preserves_requested_order() {
    let shape = Kahen::new([2, 3, 4]).unwrap();

    let result = shape.permute([2, 0, 1]).unwrap();

    assert_eq!(result.dims(), &[4, 2, 3]);
}

#[test]
fn permute_rejects_duplicate_axes() {
    let shape = Kahen::new([2, 3, 4]).unwrap();

    assert!(matches!(
        shape.permute([0, 0, 1]),
        Err(ShapeError::DuplicateAxis { axis: 0 })
    ));
}

#[test]
fn squeeze_rejects_unsigned_overflow() {
    let shape = Kahen::new([2, 1]).unwrap();

    assert!(shape.squeeze_axis(usize::MAX).is_err());
}

use kernix_core::shape::Kotei;

#[test]
fn kotei_to_kahen_preserves_dims() {
    let fixed = Kotei::<3>::new([2, 3, 4]).unwrap();

    let dynamic = Kahen::from(fixed);

    assert_eq!(dynamic.dims(), &[2, 3, 4],);
}

#[test]
fn kahen_to_kotei_preserves_dims() {
    let dynamic = Kahen::new([2, 3, 4]).unwrap();

    let fixed = Kotei::<3>::try_from(&dynamic).unwrap();

    assert_eq!(fixed.dims(), &[2, 3, 4],);
}

#[test]
fn kahen_to_kotei_rejects_wrong_rank() {
    let dynamic = Kahen::new([2, 3, 4]).unwrap();

    assert!(matches!(
        Kotei::<2>::try_from(&dynamic),
        Err(ShapeError::RankMismatch {
            expected: 2,
            actual: 3,
        }),
    ));
}

#[test]
fn cross_type_broadcast_works() {
    let dynamic = Kahen::new([2, 1]).unwrap();

    let fixed = Kotei::<2>::new([1, 3]).unwrap();

    assert_eq!(dynamic.broadcast(&fixed).unwrap().dims(), &[2, 3],);
}
