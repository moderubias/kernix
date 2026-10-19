//! Integration regressions for shape contract.
use kernix_core::shape::{Kahen, Kotei, ShapeError};
use proptest::prelude::*;

#[test]
fn scalar_is_const_and_has_one_element() {
    const SCALAR: Kahen = Kahen::scalar();
    assert_eq!(SCALAR.numel(), 1);
}
#[test]
fn removing_zero_revalidates_exposed_product() {
    let shape = Kahen::new([usize::MAX, 2, 0]).unwrap();
    assert!(matches!(
        shape.remove_axis(2),
        Err(ShapeError::ElementCountOverflow { axis: 1, .. })
    ));
    assert!(shape.remove_axis(usize::MAX).is_err());
    assert!(Kahen::scalar().remove_axis(0).is_err());
}
#[test]
fn negative_axis_bounds_include_minimum() {
    let shape = Kahen::new([2, 3]).unwrap();
    assert_eq!(shape.axis(-2), Ok(0));
    for axis in [-3, 2, isize::MIN, isize::MAX] {
        assert!(shape.axis(axis).is_err());
    }
}
#[test]
fn conversions_and_display() {
    let dynamic = Kahen::try_from(vec![2, 3]).unwrap();
    let fixed = Kotei::<2>::try_from(dynamic.dims()).unwrap();
    assert_eq!(format!("{fixed}"), "[2, 3]");
    assert_eq!(format!("{dynamic}"), "[2, 3]");
    assert_eq!(fixed.unsqueeze(1).unwrap().squeeze().dims(), &[2, 3]);
}
#[test]
fn broadcast_contract_is_directional_and_zero_aware() {
    let zero = Kahen::new([0, 3]).unwrap();
    let unit = Kahen::new([1, 3]).unwrap();
    assert!(unit.can_broadcast_to(&zero));
    assert!(!zero.can_broadcast_to(&unit));
    assert_eq!(zero.broadcast(&unit).unwrap().dims(), &[0, 3]);
    assert_eq!(
        Kahen::new([2, 1])
            .unwrap()
            .broadcast(&Kahen::new([1, 3]).unwrap())
            .unwrap()
            .numel(),
        6
    );
}
#[cfg(feature = "serde")]
#[test]
fn serde_validates_invariants_and_fixed_rank() {
    let s = Kahen::new([2, 0, 3]).unwrap();
    let json = serde_json::to_string(&s).unwrap();
    assert_eq!(json, "[2,0,3]");
    assert_eq!(serde_json::from_str::<Kahen>(&json).unwrap(), s);
    assert_eq!(
        serde_json::from_str::<Kotei<3>>(&json).unwrap().dims(),
        s.dims()
    );
    assert!(serde_json::from_str::<Kotei<2>>(&json).is_err());
    assert!(serde_json::from_str::<Kahen>("[-1]").is_err());
    assert!(serde_json::from_str::<Kahen>(&format!("[{},2]", usize::MAX)).is_err());
}
proptest! {
    #[test]
    fn insertion_removal_and_permutation_preserve_shape(dims in prop::collection::vec(0usize..8, 0..12), seed in any::<usize>()) {
        let s = Kahen::new(dims).unwrap();
        let axis=seed%(s.rank()+1);
        prop_assert_eq!(s.insert_axis(axis).unwrap().remove_axis(axis).unwrap(),s.clone());
        let rev: Vec<usize> = (0..s.rank()).rev().collect();
        let p=s.permute(&rev).unwrap();
        prop_assert_eq!(p.numel(),s.numel());
        prop_assert_eq!(p.permute(&rev).unwrap(),s);
    }
    #[test]
    fn broadcasting_is_commutative(a in prop::collection::vec(0usize..8,0..8), b in prop::collection::vec(0usize..8,0..8)) {
        let a=Kahen::new(a).unwrap(); let b=Kahen::new(b).unwrap();
        match (a.broadcast(&b),b.broadcast(&a)) {
            (Ok(x),Ok(y)) => {prop_assert_eq!(&x,&y);prop_assert!(a.can_broadcast_to(&x));prop_assert!(b.can_broadcast_to(&x));},
            (Err(_),Err(_))=>(),
            _=>prop_assert!(false),
        }
    }
}
