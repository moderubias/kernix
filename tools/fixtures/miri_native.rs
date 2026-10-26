//! Bounded interpreter checks: SIMD selection is disabled under cfg(miri).
//! Full native dimensions remain in native_kernel_tests and actual-hardware/ASan CI.
#[cfg(test)]
mod tests {
    use kernix_ops::{MatmulElement, MatrixInput};

    macro_rules! coordinates {
        ($name:ident, $ty:ty) => {
            #[test]
            fn $name() {
                for (m, k, n) in [(3usize, 5usize, 4usize), (4, 3, 8), (5, 7, 3)] {
                    let ar = 2 * k + 3;
                    let br = 3 * n + 1;
                    let mut a = vec![<$ty>::NAN; 5 + (m - 1) * ar + 2 * (k - 1) + 1];
                    let mut b = vec![<$ty>::NAN; 7 + (k - 1) * br + 3 * (n - 1) + 1];
                    let av: Vec<$ty> = (0..m * k)
                        .map(|p| ((p * 7 % 23) as $ty - 11.) / 16.)
                        .collect();
                    let bv: Vec<$ty> = (0..k * n)
                        .map(|p| ((p * 5 % 13) as $ty - 6.) / 8.)
                        .collect();
                    for i in 0..m {
                        for p in 0..k {
                            a[5 + i * ar + 2 * p] = av[i * k + p];
                        }
                    }
                    for p in 0..k {
                        for j in 0..n {
                            b[7 + p * br + 3 * j] = bv[p * n + j];
                        }
                    }
                    let saved_a: Vec<_> = a.iter().map(|x| x.to_bits()).collect();
                    let saved_b: Vec<_> = b.iter().map(|x| x.to_bits()).collect();
                    for reverse_a in [false, true] {
                        for reverse_b in [false, true] {
                            let aa = MatrixInput::new(
                                &a,
                                [m, k],
                                [
                                    if reverse_a {
                                        -(ar as isize)
                                    } else {
                                        ar as isize
                                    },
                                    2,
                                ],
                                5 + if reverse_a { (m - 1) * ar } else { 0 },
                            )
                            .unwrap();
                            let bb = MatrixInput::new(
                                &b,
                                [k, n],
                                [br as isize, if reverse_b { -3 } else { 3 }],
                                7 + if reverse_b { 3 * (n - 1) } else { 0 },
                            )
                            .unwrap();
                            let mut out = vec![<$ty>::NAN; m * n + 2];
                            assert!(<$ty>::try_strided_matmul(aa, bb, &mut out[1..1 + m * n]));
                            for i in 0..m {
                                for j in 0..n {
                                    let expected: f64 = (0..k)
                                        .map(|p| {
                                            f64::from(
                                                av[(if reverse_a { m - 1 - i } else { i }) * k + p],
                                            ) * f64::from(
                                                bv[p * n + if reverse_b { n - 1 - j } else { j }],
                                            )
                                        })
                                        .sum();
                                    assert_eq!(f64::from(out[1 + i * n + j]), expected);
                                }
                            }
                            assert!(out[0].is_nan() && out[1 + m * n].is_nan());
                        }
                    }
                    assert_eq!(a.iter().map(|x| x.to_bits()).collect::<Vec<_>>(), saved_a);
                    assert_eq!(b.iter().map(|x| x.to_bits()).collect::<Vec<_>>(), saved_b);
                    let mut dense = vec![<$ty>::NAN; m * n];
                    assert!(<$ty>::try_dense_matmul(&av, &bv, &mut dense, m, k, n));
                    for i in 0..m {
                        for j in 0..n {
                            let expected: f64 = (0..k)
                                .map(|p| f64::from(av[i * k + p]) * f64::from(bv[p * n + j]))
                                .sum();
                            assert_eq!(f64::from(dense[i * n + j]), expected);
                        }
                    }
                }
                let a = [1.5 as $ty];
                let b = [2.0 as $ty];
                let mut out = [0.0 as $ty; 15];
                assert!(<$ty>::try_strided_matmul(
                    MatrixInput::new(&a, [3, 4], [0, 0], 0).unwrap(),
                    MatrixInput::new(&b, [4, 5], [0, 0], 0).unwrap(),
                    &mut out
                ));
                assert!(out.iter().all(|&x| x == 12.0));
            }
        };
    }
    coordinates!(
        float64_gapped_signed_broadcast_dense_and_canary_oracles,
        f64
    );
    coordinates!(
        float32_gapped_signed_broadcast_dense_and_canary_oracles,
        f32
    );

    #[test]
    fn empty_invalid_and_aliasing_inputs_keep_checked_output_contracts() {
        let empty = MatrixInput::new(&[], [3, 0], [isize::MIN, 0], 0).unwrap();
        let rhs = MatrixInput::new(&[], [0, 5], [0, isize::MIN], 0).unwrap();
        let mut out = [f64::NAN; 15];
        assert!(f64::try_strided_matmul(empty, rhs, &mut out));
        assert_eq!(out, [0.0; 15]);
        let one = [2.0f64];
        let empty = MatrixInput::new(&[], [0, 4], [0, 0], 0).unwrap();
        let rhs = MatrixInput::new(&one, [4, 5], [0, 0], 0).unwrap();
        assert!(f64::try_strided_matmul(empty, rhs, &mut []));
        let lhs = MatrixInput::new(&one, [2, 3], [0, 0], 0).unwrap();
        let wrong = MatrixInput::new(&one, [4, 2], [0, 0], 0).unwrap();
        let mut guard = [7.0; 4];
        assert!(!f64::try_strided_matmul(lhs, wrong, &mut guard));
        assert_eq!(guard, [7.0; 4]);
        assert!(!f64::try_dense_matmul(&one, &one, &mut guard, 2, 2, 2));
        assert_eq!(guard, [7.0; 4]);
        assert!(MatrixInput::new(&one, [2, 2], [isize::MAX, isize::MAX], 0).is_err());
        let x = [1.0, 2.0, 3.0, 4.0];
        let a = MatrixInput::new(&x, [2, 2], [2, 1], 0).unwrap();
        let b = MatrixInput::new(&x, [2, 2], [2, 1], 0).unwrap();
        assert!(f64::try_strided_matmul(a, b, &mut guard));
        assert_eq!(guard, [7.0, 10.0, 15.0, 22.0]);
    }
}
