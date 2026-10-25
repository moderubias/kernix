//! Compact-WY reconstruction, orthogonality, scales, tails and generic decline/range fallback.
use kernix_core::{Order, tensor::Tensor};
use kernix_ops::linalg::Qr;
fn check(m: usize, n: usize, scale: f64) {
    let a = Tensor::from_values(
        [m, n],
        (0..m * n)
            .map(|p| {
                (((p * 13 % 47) as f64 - 23.0) / 47.0 + if p / n == p % n { 3.0 } else { 0.0 })
                    * scale
            })
            .collect(),
    )
    .unwrap();
    let saved = a.storage_slice().to_vec();
    let qr = Qr::factor(&a).unwrap();
    let q = qr.q().storage_slice();
    let r = qr.r().storage_slice();
    for i in 0..m {
        for j in 0..n {
            let value: f64 = (0..n).map(|k| q[i * n + k] * (r[k * n + j] / scale)).sum();
            assert!(
                (value - a.storage_slice()[i * n + j] / scale).abs() < 3e-12,
                "{m}/{n}, scale {scale} at{i}/{j}"
            );
        }
    }
    for i in 0..n {
        for j in 0..n {
            let value: f64 = (0..m).map(|k| q[k * n + i] * q[k * n + j]).sum();
            assert!(
                (value - if i == j { 1.0 } else { 0.0 }).abs() < 3e-12,
                "Gram {m}/{n} {i}/{j}"
            );
        }
    }
    assert_eq!(a.storage_slice(), saved);
    let reversed = a.reverse_axis(0).unwrap();
    let owned = reversed.to_owned_contiguous(Order::C).unwrap();
    let x = Qr::factor(&reversed).unwrap();
    let y = Qr::factor(&owned).unwrap();
    assert_eq!(x.q().storage_slice(), y.q().storage_slice());
    assert_eq!(x.r().storage_slice(), y.r().storage_slice());
}
#[test]
fn compact_wy_branch_boundaries_tails_scales_and_views() {
    for (m, n) in [
        (191, 128),
        (192, 127),
        (192, 128),
        (193, 129),
        (256, 128),
        (257, 159),
        (512, 128),
    ] {
        check(m, n, 1.0);
    }
    for scale in [1e-200, 1e200] {
        check(192, 129, scale);
    }
}
#[test]
fn zero_reflectors_and_float32_blocked_factorization() {
    let z = Qr::factor(&Tensor::<f64>::zeros([192, 128]).unwrap()).unwrap();
    for i in 0..192 {
        for j in 0..128 {
            assert_eq!(
                z.q().storage_slice()[i * 128 + j],
                if i == j { 1.0 } else { 0.0 }
            );
        }
    }
    let (m, n) = (193, 129);
    let a = Tensor::from_values(
        [m, n],
        (0..m * n)
            .map(|p| ((p * 7 % 31) as f32 - 15.0) / 32.0 + if p / n == p % n { 4.0 } else { 0.0 })
            .collect(),
    )
    .unwrap();
    let qr = Qr::factor(&a).unwrap();
    let q = qr.q().storage_slice();
    let r = qr.r().storage_slice();
    for i in 0..m {
        for j in 0..n {
            let v: f64 = (0..n)
                .map(|k| f64::from(q[i * n + k]) * f64::from(r[k * n + j]))
                .sum();
            assert!((v - f64::from(a.storage_slice()[i * n + j])).abs() < 5e-5);
        }
    }
    for i in 0..n {
        for j in 0..n {
            let v: f64 = (0..m)
                .map(|k| f64::from(q[k * n + i]) * f64::from(q[k * n + j]))
                .sum();
            assert!((v - if i == j { 1.0 } else { 0.0 }).abs() < 3e-6);
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
struct Limited<const MODE: u8>(f64);
macro_rules! op {($trait:ident,$method:ident,$op:tt)=>{impl<const M:u8> core::ops::$trait for Limited<M>{type Output=Self;fn $method(self,r:Self)->Self{Self(self.0 $op r.0)}}};}
op!(Add,add,+);
op!(Sub,sub,-);
op!(Mul,mul,*);
op!(Div,div,/);
impl<const M: u8> core::ops::Neg for Limited<M> {
    type Output = Self;
    fn neg(self) -> Self {
        Self(-self.0)
    }
}
impl<const M: u8> core::ops::AddAssign for Limited<M> {
    fn add_assign(&mut self, r: Self) {
        self.0 += r.0;
    }
}
impl<const M: u8> kernix_ops::Zero for Limited<M> {
    fn zero() -> Self {
        Self(0.0)
    }
}
impl<const M: u8> kernix_ops::One for Limited<M> {
    fn one() -> Self {
        Self(1.0)
    }
}
impl<const M: u8> kernix_ops::FloatElement for Limited<M> {
    fn from_count(n: usize) -> Self {
        Self(n as f64)
    }
}
impl<const M: u8> kernix_ops::FloatMath for Limited<M> {
    fn exp(self) -> Self {
        Self(self.0.exp())
    }
    fn ln(self) -> Self {
        Self(self.0.ln())
    }
    fn sqrt(self) -> Self {
        Self(self.0.sqrt())
    }
    fn tanh(self) -> Self {
        Self(self.0.tanh())
    }
    fn erf(self) -> Self {
        Self(self.0)
    }
    fn abs(self) -> Self {
        Self(self.0.abs())
    }
    fn is_nan(self) -> bool {
        self.0.is_nan()
    }
    fn is_finite(self) -> bool {
        self.0.is_finite()
    }
}
impl<const M: u8> kernix_ops::MatmulElement for Limited<M> {
    const HAS_NATIVE_MATMUL: bool = M != 0;
    fn try_strided_matmul(
        _: kernix_ops::MatrixInput<'_, Self>,
        _: kernix_ops::MatrixInput<'_, Self>,
        out: &mut [Self],
    ) -> bool {
        if M == 2 {
            out.fill(Self(f64::NAN));
            true
        } else {
            false
        }
    }
}
impl<const M: u8> kernix_ops::LinalgFloat for Limited<M> {
    const EPSILON: Self = Self(f64::EPSILON);
}
fn fallback<const M: u8>() -> Qr<Limited<M>> {
    Qr::factor(
        &Tensor::from_values(
            [192, 128],
            (0..192 * 128)
                .map(|p| {
                    Limited::<M>(
                        ((p * 7 % 31) as f64 - 15.0) / 32.0
                            + if p / 128 == p % 128 { 4.0 } else { 0.0 },
                    )
                })
                .collect(),
        )
        .unwrap(),
    )
    .unwrap()
}
#[test]
fn declined_and_range_limited_native_kernels_preserve_ordered_reflectors() {
    let reference = fallback::<0>();
    let declined = fallback::<1>();
    let range = fallback::<2>();
    let expected: Vec<_> = reference
        .q()
        .storage_slice()
        .iter()
        .chain(reference.r().storage_slice())
        .map(|v| v.0)
        .collect();
    let a: Vec<_> = declined
        .q()
        .storage_slice()
        .iter()
        .chain(declined.r().storage_slice())
        .map(|v| v.0)
        .collect();
    let b: Vec<_> = range
        .q()
        .storage_slice()
        .iter()
        .chain(range.r().storage_slice())
        .map(|v| v.0)
        .collect();
    assert_eq!(expected, a);
    assert_eq!(expected, b);
}
