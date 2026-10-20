//! Macros must expand without std or a caller-side alloc import.
#![no_std]
use kernix_core as math;

#[test]
fn renamed_dependency_macros_work_without_alloc_import() {
    let a = math::matrix![1.0f64, 2.0; 3.0, 4.0].unwrap();
    let b = math::tensor![1.0, 2.0].unwrap();
    let c = math::full!(0.0f64; 2, 2).unwrap();
    let d = math::shape![2, 2].unwrap();
    let e = math::tensor![1.0; 4].unwrap();
    let f = math::tensor!(shape = [2, 2]; values = a.into_storage_vec()).unwrap();
    assert_eq!(f.shape().dims(), d.dims());
    assert_eq!(b.numel(), 2);
    assert_eq!(e.numel(), c.numel());
}
