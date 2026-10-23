//! Coalesced contraction keeps the portable scalar contract and indexed fallback.
use kernix_core::{
    Order,
    shape::Kahen,
    tensor::{Tensor, TensorError},
};
use kernix_ops::{TensorInput, Zero, tensordot};
use std::cell::Cell;
use std::ops::{AddAssign, Mul};

struct Indexed<'a, T>(&'a Tensor<T>);
impl<T> TensorInput<T> for Indexed<'_, T> {
    fn dims(&self) -> &[usize] {
        self.0.shape().dims()
    }
    fn get(&self, i: &[usize]) -> Result<&T, TensorError> {
        self.0.get(i)
    }
}
thread_local! {static CLONES:Cell<usize> = const { Cell::new(0) };}
#[derive(Debug, PartialEq)]
struct Scalar(i64);
impl Clone for Scalar {
    fn clone(&self) -> Self {
        CLONES.with(|n| n.set(n.get() + 1));
        Self(self.0)
    }
}
impl Zero for Scalar {
    fn zero() -> Self {
        Self(0)
    }
}
impl AddAssign for Scalar {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}
impl Mul for Scalar {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self(self.0 * rhs.0)
    }
}

#[test]
fn coalesced_custom_scalar_keeps_values_moves_and_clone_count() {
    let a = Tensor::from_values([2, 3, 4], (0..24).map(Scalar).collect()).unwrap();
    let b = Tensor::from_values([3, 4, 5], (0..60).map(Scalar).collect()).unwrap();
    CLONES.with(|n| n.set(0));
    let x = tensordot(&a, &b, &[1, 2], &[0, 1]).unwrap();
    let fast = CLONES.with(Cell::get);
    CLONES.with(|n| n.set(0));
    let y = tensordot(&Indexed(&a), &Indexed(&b), &[1, 2], &[0, 1]).unwrap();
    assert_eq!(x.storage_slice(), y.storage_slice());
    assert_eq!(fast, CLONES.with(Cell::get));
    assert_eq!(fast, 2 * 2 * 5 * 12);
    assert_eq!(a.storage_slice()[23].0, 23);
}

#[test]
fn coalesced_axes_empty_outer_scalars_and_general_fallback_keep_exact_order() {
    for (ad, bd, aa, ba) in [
        (vec![2, 3, 4], vec![3, 4, 5], vec![1, 2], vec![0, 1]),
        (vec![2, 3, 4], vec![4, 3, 5], vec![2, 1], vec![0, 1]),
        (vec![2, 0, 4], vec![0, 4, 5], vec![1, 2], vec![0, 1]),
        (vec![2, 3], vec![4], vec![], vec![]),
        (vec![], vec![], vec![], vec![]),
    ] {
        let a = Tensor::from_values(
            &ad,
            (0..ad.iter().product())
                .map(|i| (i as f64 - 3.) / 47.)
                .collect(),
        )
        .unwrap();
        let b = Tensor::from_values(
            &bd,
            (0..bd.iter().product())
                .map(|i| (i as f64 - 2.) / 13.)
                .collect(),
        )
        .unwrap();
        let x = tensordot(&a, &b, &aa, &ba).unwrap();
        let y = tensordot(&Indexed(&a), &Indexed(&b), &aa, &ba).unwrap();
        assert_eq!(x.shape().dims(), y.shape().dims());
        assert_eq!(x.storage_slice(), y.storage_slice());
    }
    let a = Tensor::from_fn(Kahen::new([2, 3]).unwrap(), Order::F, 8, |i| {
        (i[0] * 3 + i[1]) as f64
    })
    .unwrap();
    let b = Tensor::from_values([3, 2], vec![1.; 6]).unwrap();
    let reversed = a.reverse_axis(1).unwrap();
    assert_eq!(
        tensordot(&reversed, &b, &[1], &[0])
            .unwrap()
            .storage_slice(),
        &[3., 3., 12., 12.]
    );
}

#[test]
fn malformed_contiguous_capability_is_rejected_before_access() {
    struct Bad;
    impl TensorInput<f64> for Bad {
        fn dims(&self) -> &[usize] {
            &[2, 3]
        }
        fn get(&self, _: &[usize]) -> Result<&f64, TensorError> {
            panic!("invalid capability was accessed")
        }
        fn as_c_contiguous_slice(&self) -> Option<&[f64]> {
            Some(&[1.])
        }
    }
    let b = Tensor::<f64>::ones([3, 4]).unwrap();
    assert!(tensordot(&Bad, &b, &[1], &[0]).is_err());
}
