//! Integration regressions for shape extras 04.
use kernix_core::shape::{Kahen, Kotei, ShapeError};

#[test]
fn squeeze_only_removes_singletons_and_preserves_numel() {
    let original = Kahen::new([1, 0, 1, 5, 1]).unwrap();
    let squeezed = original.squeeze();
    assert_eq!(squeezed.dims(), &[0, 5]);
    assert_eq!(squeezed.numel(), original.numel());
    assert_eq!(Kahen::new([1, 1]).unwrap().squeeze().dims(), &[]);
    assert_eq!(Kahen::scalar().squeeze(), Kahen::scalar());
}

#[test]
fn unsqueeze_and_swap_axes_preserve_values_and_validate_indices() {
    let shape = Kahen::new([2, 3, 4]).unwrap();
    assert_eq!(shape.unsqueeze(1).unwrap().dims(), &[2, 1, 3, 4]);
    let swapped = shape.swap_axes(0, 2).unwrap();
    assert_eq!(swapped.dims(), &[4, 3, 2]);
    assert_eq!(swapped.swap_axes(0, 2).unwrap(), shape);
    assert!(matches!(
        shape.swap_axes(3, 0),
        Err(ShapeError::AxisPositionOutOfBounds { .. })
    ));
}

#[test]
fn fixed_rank_swap_axes_remains_fixed_rank() {
    let shape = Kotei::<3>::new([2, 3, 4]).unwrap();
    let swapped: Kotei<3> = shape.swap_axes(0, 2).unwrap();
    assert_eq!(swapped.dims_array(), &[4, 3, 2]);
    assert_eq!(shape.swap_axes(0, 0).unwrap(), shape);
    assert!(shape.swap_axes(1, usize::MAX).is_err());
}

#[test]
fn broadcasting_zero_with_singleton_stays_zero() {
    let empty = Kahen::new([0, 3]).unwrap();
    let singleton = Kahen::new([1, 3]).unwrap();
    assert_eq!(empty.broadcast(&singleton).unwrap().dims(), &[0, 3]);
    assert_eq!(singleton.broadcast(&empty).unwrap().dims(), &[0, 3]);
    assert_eq!(empty.broadcast(&singleton).unwrap().numel(), 0);
}
