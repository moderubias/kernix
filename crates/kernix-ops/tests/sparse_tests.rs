//! Independent dense oracles, representation dispatch and sparse resource contracts.
use kernix_core::{Order, tensor::Tensor};
use kernix_ops::{CsrMatrix, Matrix, MatrixStorage, SparseError, matmul_reference};

fn matrix(m: usize, n: usize, shift: usize) -> Tensor<f64> {
    Tensor::from_values(
        [m, n],
        (0..m * n)
            .map(|p| {
                if (p * 7 + shift) % 11 < 2 {
                    (p % 13) as f64 / 8. - 0.5
                } else {
                    0.
                }
            })
            .collect(),
    )
    .unwrap()
}
fn assert_close(a: &Tensor<f64>, b: &Tensor<f64>) {
    assert_eq!(a.shape().dims(), b.shape().dims());
    for (&x, &y) in a.storage_slice().iter().zip(b.storage_slice()) {
        assert!((x - y).abs() < 2e-12 * (1. + y.abs()), "{x} vs {y}");
    }
}
#[test]
fn raw_csr_adopts_nonclone_values_and_rejects_malformed_buffers() {
    #[derive(Debug, PartialEq)]
    struct Move(i32);
    let ptr = vec![0, 1, 1, 2];
    let idx = vec![1, 0];
    let vals = vec![Move(3), Move(7)];
    let addresses = (ptr.as_ptr(), idx.as_ptr(), vals.as_ptr());
    let s = CsrMatrix::from_parts([3, 2], ptr, idx, vals).unwrap();
    assert_eq!(
        (
            s.row_offsets().as_ptr(),
            s.column_indices().as_ptr(),
            s.values().as_ptr()
        ),
        addresses
    );
    assert_eq!(s.get(0, 1).unwrap(), Some(&Move(3)));
    assert!(s.get(1, 1).unwrap().is_none());
    assert!(s.get(3, 0).is_err());
    assert!(s.get(0, 2).is_err());
    let t = s.transpose();
    assert_eq!(t.dims(), [2, 3]);
    assert_eq!(t.values().as_ptr(), addresses.2);
    assert_eq!(t.column_offsets().as_ptr(), addresses.0);
    assert_eq!(t.row_indices().as_ptr(), addresses.1);
    assert_eq!(t.get(1, 0).unwrap(), Some(&Move(3)));
    assert!(core::ptr::eq(t.transpose(), &s));
    for (p, i, v) in [
        (vec![1, 1, 1], vec![0], vec![1.]),
        (vec![0, 2, 1], vec![0], vec![1.]),
        (vec![0, 1, 2], vec![0, 3], vec![1., 2.]),
        (vec![0, 2, 2], vec![0, 0], vec![1., 2.]),
        (vec![0, 2, 2], vec![1, 0], vec![1., 2.]),
        (vec![0, 0], vec![], vec![]),
    ] {
        assert!(CsrMatrix::from_parts([2, 2], p, i, v).is_err());
    }
    assert!(matches!(
        CsrMatrix::<f64>::from_parts([usize::MAX, 0], vec![], vec![], vec![]),
        Err(SparseError::SizeOverflow)
    ));
}
#[test]
fn coo_folds_duplicates_in_original_order_and_moves_custom_elements() {
    let s = CsrMatrix::from_triplets(
        [2, 3],
        vec![
            (1, 2, 7.),
            (0, 1, 1e16),
            (0, 0, 3.),
            (0, 1, 1.),
            (0, 1, -1e16),
        ],
    )
    .unwrap();
    assert_eq!(s.row_offsets(), &[0, 2, 3]);
    assert_eq!(s.column_indices(), &[0, 1, 2]);
    assert_eq!(s.values(), &[3., 0., 7.]);
    #[derive(Debug, PartialEq)]
    struct Move(i32);
    impl core::ops::AddAssign for Move {
        fn add_assign(&mut self, r: Self) {
            self.0 += r.0;
        }
    }
    let a = CsrMatrix::from_triplets([1, 1], vec![(0, 0, Move(3)), (0, 0, Move(5))]).unwrap();
    assert_eq!(a.values(), &[Move(8)]);
    assert!(CsrMatrix::from_triplets([0, 3], vec![(0, 1, 2.)]).is_err());
    assert!(CsrMatrix::from_triplets([2, 0], vec![(1, 0, 2.)]).is_err());
}
#[test]
fn auto_storage_preserves_dense_buffer_and_uses_exact_zero_memory_policy() {
    let dense = Tensor::from_values([3, 4], (1..=12).map(|v| v as f64).collect()).unwrap();
    let pointer = dense.storage_slice().as_ptr();
    let d = Matrix::from_tensor(dense).unwrap();
    assert_eq!(d.storage_kind(), MatrixStorage::Dense);
    assert_eq!(d.as_dense().unwrap().storage_slice().as_ptr(), pointer);
    assert_eq!(d.into_dense().unwrap().storage_slice().as_ptr(), pointer);
    let a = Matrix::from_rows([[2.0f64, 0., 0., 0.], [0., 3., 0., 0.]]).unwrap();
    assert_eq!(a.storage_kind(), MatrixStorage::Csr);
    assert_eq!(a.payload_bytes(), 16 + 5 * core::mem::size_of::<usize>());
    assert_eq!(
        a.into_dense().unwrap().storage_slice(),
        &[2., 0., 0., 0., 0., 3., 0., 0.]
    );
    let tiny = Matrix::from_rows([[0.0f64]]).unwrap();
    assert_eq!(tiny.storage_kind(), MatrixStorage::Dense);
    let near = Matrix::from_rows([[1e-300f64, 0., 0., 0.], [0., 0., 0., 0.]]).unwrap();
    assert_eq!(near.as_sparse().unwrap().nnz(), 1);
    assert_eq!(near.into_dense().unwrap().storage_slice()[0], 1e-300);
    let f = Tensor::from_values(
        [4, 7],
        (0..28).map(|p| if p % 9 == 0 { 1. } else { 0. }).collect(),
    )
    .unwrap();
    let reversed = f
        .reverse_axis(1)
        .unwrap()
        .to_owned_contiguous(Order::F)
        .unwrap();
    let original = reversed.to_owned_contiguous(Order::C).unwrap();
    let result = Matrix::from_tensor(reversed).unwrap().into_dense().unwrap();
    assert_eq!(result.storage_slice(), original.storage_slice());
    let empty =
        Matrix::from_tensor(Tensor::<f64>::zeros([0, isize::MAX as usize]).unwrap()).unwrap();
    assert_eq!(empty.storage_kind(), MatrixStorage::Dense);
}
#[test]
fn all_four_storage_products_and_signed_stride_vectors_match_independent_oracles() {
    for (m, k, n) in [
        (0, 3, 4),
        (3, 0, 4),
        (3, 4, 0),
        (1, 1, 1),
        (3, 5, 4),
        (7, 9, 13),
        (17, 31, 9),
        (65, 33, 7),
    ] {
        let a = matrix(m, k, 1);
        let b = matrix(k, n, 2);
        let expected = matmul_reference(&a, &b).unwrap();
        for sparse_a in [false, true] {
            for sparse_b in [false, true] {
                let a = if sparse_a {
                    Matrix::sparse(Matrix::dense(a.clone()).unwrap().into_csr().unwrap()).unwrap()
                } else {
                    Matrix::dense(a.clone()).unwrap()
                };
                let b = if sparse_b {
                    Matrix::sparse(Matrix::dense(b.clone()).unwrap().into_csr().unwrap()).unwrap()
                } else {
                    Matrix::dense(b.clone()).unwrap()
                };
                let output = a.matmul(&b).unwrap();
                assert_eq!(
                    output.storage_kind(),
                    if sparse_a && sparse_b {
                        MatrixStorage::Csr
                    } else {
                        MatrixStorage::Dense
                    }
                );
                assert_close(&output.into_dense().unwrap(), &expected);
            }
        }
        let v = Tensor::from_values([k], (0..k).map(|i| i as f64 / 16. - 0.5).collect()).unwrap();
        let v = v.reverse_axis(0).unwrap();
        let target = matmul_reference(&a, &v).unwrap();
        let s = Matrix::dense(a).unwrap().into_csr().unwrap();
        assert_close(&s.matvec(&v).unwrap(), &target);
    }
}
#[test]
fn transpose_csc_and_reversed_dense_rhs_remain_zero_copy_and_correct() {
    let a = matrix(7, 9, 3);
    let s = Matrix::dense(a.clone()).unwrap().into_csr().unwrap();
    let view = s.transpose();
    let b = matrix(7, 5, 4);
    let b = b.reverse_axis(0).unwrap();
    let expected = matmul_reference(&a.transpose().unwrap(), &b).unwrap();
    assert_close(&view.matmul_dense(&b).unwrap(), &expected);
    let v = Tensor::from_values([7], vec![0.25; 7]).unwrap();
    assert_close(
        &view.matvec(&v).unwrap(),
        &matmul_reference(&a.transpose().unwrap(), &v).unwrap(),
    );
    let b = matrix(9, 5, 2);
    let b = b.reverse_axis(1).unwrap();
    assert_close(
        &s.matmul_dense(&b).unwrap(),
        &matmul_reference(&a, &b).unwrap(),
    );
}
#[test]
fn sparse_products_do_not_allocate_a_column_sized_accumulator_and_limit_fill_in() {
    let a = CsrMatrix::from_triplets([1, 1], vec![(0, 0, 2.0f64)]).unwrap();
    let b = CsrMatrix::from_triplets([1, usize::MAX], vec![(0, usize::MAX - 1, 3.)]).unwrap();
    let result = a.matmul_with_limit(&b, 1).unwrap();
    assert_eq!(result.dims(), [1, usize::MAX]);
    assert_eq!(result.nnz(), 1);
    assert_eq!(result.column_indices(), &[usize::MAX - 1]);
    assert_eq!(result.values(), &[6.]);
    assert!(result.to_dense().is_err());
    assert!(matches!(
        a.matmul_with_limit(&b, 0),
        Err(SparseError::FillInLimit { limit: 0 })
    ));
    let a = CsrMatrix::from_triplets([1, 2], vec![(0, 0, 1.), (0, 1, 1.)]).unwrap();
    let b = CsrMatrix::from_triplets([2, 1], vec![(0, 0, 1.), (1, 0, -1.)]).unwrap();
    assert_eq!(a.matmul_with_limit(&b, 0).unwrap().nnz(), 0);
}
#[test]
fn finite_policy_errors_and_shape_validation_are_representation_independent() {
    for bad in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert!(matches!(
            Matrix::from_rows([[bad]]),
            Err(SparseError::NonFinite)
        ));
        assert!(matches!(
            Matrix::dense(Tensor::from_rows([[bad]]).unwrap()),
            Err(SparseError::NonFinite)
        ));
        let s = CsrMatrix::from_triplets([1, 1], vec![(0, 0, bad)]).unwrap();
        assert!(matches!(Matrix::sparse(s), Err(SparseError::NonFinite)));
    }
    let s = CsrMatrix::from_triplets([1, 2], vec![(0, 0, 1.)]).unwrap();
    let rhs = Tensor::from_values([2], vec![1., f64::NAN]).unwrap();
    assert!(matches!(s.matvec(&rhs), Err(SparseError::NonFinite)));
    let s = CsrMatrix::from_triplets([1, 1], vec![(0, 0, f64::MAX)]).unwrap();
    let rhs = Tensor::from_values([1], vec![2.]).unwrap();
    assert!(matches!(s.matvec(&rhs), Err(SparseError::NonFinite)));
    assert!(s.matvec(&Tensor::<f64>::zeros([1, 1]).unwrap()).is_err());
    assert!(Matrix::from_tensor(Tensor::<f64>::zeros([2]).unwrap()).is_err());
}
#[test]
fn float32_storage_and_products_keep_dtype_and_error_budget() {
    let a = Matrix::from_rows([[1.0f32, 0., 0., 0.], [0., 0., 0., 2.]]).unwrap();
    assert_eq!(a.storage_kind(), MatrixStorage::Dense); // CSR indices cost more for this tiny f32 fixture.
    let b = Matrix::from_rows([[2.0f32], [3.], [4.], [5.]]).unwrap();
    let out: Tensor<f32> = a.matmul(&b).unwrap().into_dense().unwrap();
    assert_eq!(out.storage_slice(), &[2., 10.]);
}

#[test]
fn compression_accepts_fixed_shapes_views_and_checks_custom_capabilities() {
    use kernix_ops::TensorInput;
    let a = matrix(5, 7, 2);
    let view = a.reverse_axis(0).unwrap();
    let csr = CsrMatrix::from_tensor(&view).unwrap();
    assert_close(
        &csr.to_dense().unwrap(),
        &view.to_owned_contiguous(Order::C).unwrap(),
    );
    let fixed = Tensor::from_vec(
        vec![0.0f64, 2., 0., 0.],
        kernix_core::shape::Kotei::new([2, 2]).unwrap(),
        Order::F,
    )
    .unwrap();
    assert_eq!(
        CsrMatrix::from_tensor(&fixed)
            .unwrap()
            .to_dense()
            .unwrap()
            .storage_slice(),
        &[0., 0., 2., 0.]
    );
    let t = csr.transpose();
    assert!(matches!(
        t.get(7, 0),
        Err(SparseError::Index {
            axis: 0,
            index: 7,
            dim: 7
        })
    ));
    assert!(matches!(
        t.get(0, 5),
        Err(SparseError::Index {
            axis: 1,
            index: 5,
            dim: 5
        })
    ));
    struct Bad;
    impl TensorInput<f64> for Bad {
        fn dims(&self) -> &[usize] {
            &[2, 3]
        }
        fn get(&self, _: &[usize]) -> Result<&f64, kernix_core::tensor::TensorError> {
            panic!("checked slice capability must be used")
        }
        fn as_c_contiguous_slice(&self) -> Option<&[f64]> {
            Some(&[0.])
        }
    }
    assert!(CsrMatrix::from_tensor(&Bad).is_err());
    assert!(csr.matmul_dense(&Bad).is_err());
}

#[test]
fn rectangular_random_patterns_and_nondyadic_values_match_scalar_reference() {
    let mut state = 73981u64;
    let mut next = || {
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
        state >> 32
    };
    for density in [1u64, 13, 49, 97] {
        for (m, k, n) in [(2, 11, 17), (13, 7, 19), (31, 33, 9)] {
            let mut data = |count| {
                (0..count)
                    .map(|_| {
                        let r = next();
                        if r % 100 < density {
                            (next() % 2001) as f64 / 997. - 1.
                        } else {
                            0.
                        }
                    })
                    .collect()
            };
            let a = Tensor::from_values([m, k], data(m * k)).unwrap();
            let b = Tensor::from_values([k, n], data(k * n)).unwrap();
            let target = matmul_reference(&a, &b).unwrap();
            let sa = CsrMatrix::from_tensor(&a).unwrap();
            let sb = CsrMatrix::from_tensor(&b).unwrap();
            assert_close(&sa.matmul(&sb).unwrap().to_dense().unwrap(), &target);
            assert_close(&sa.matmul_dense(&b).unwrap(), &target);
            assert_close(
                &Matrix::dense(a)
                    .unwrap()
                    .matmul(&Matrix::sparse(sb).unwrap())
                    .unwrap()
                    .into_dense()
                    .unwrap(),
                &target,
            );
        }
    }
}

#[test]
fn compression_rejects_changing_capabilities_and_resource_overflow_without_panics() {
    use core::cell::Cell;
    use kernix_ops::TensorInput;
    struct Changing {
        calls: Cell<usize>,
        slice: bool,
    }
    impl TensorInput<f64> for Changing {
        fn dims(&self) -> &[usize] {
            &[1, 2]
        }
        fn get(&self, _: &[usize]) -> Result<&f64, kernix_core::tensor::TensorError> {
            let n = self.calls.get();
            self.calls.set(n + 1);
            Ok(if n < 2 { &0. } else { &1. })
        }
        fn as_c_contiguous_slice(&self) -> Option<&[f64]> {
            if !self.slice {
                return None;
            }
            let n = self.calls.get();
            self.calls.set(n + 1);
            Some(if n == 0 { &[0., 0.] } else { &[0.] })
        }
    }
    for slice in [false, true] {
        assert!(matches!(
            CsrMatrix::from_tensor(&Changing {
                calls: Cell::new(0),
                slice
            }),
            Err(SparseError::Invalid(_))
        ));
    }
    assert!(matches!(
        CsrMatrix::<f64>::from_triplets([usize::MAX - 1, 0], vec![]),
        Err(SparseError::Allocation(_))
    ));
}
