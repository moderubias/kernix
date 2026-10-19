use kernix_core::{
    layout::{Layout, Order},
    shape::{Kahen, Kotei},
    storage::CpuStorage,
    tensor::Tensor,
};
use kernix_ops::{
    OpsError, add, div, map, matmul, max_all, mean_all, mean_axes, min_all, mul, neg, prod_all,
    prod_axes, rem, sub, sum_all, sum_axes, zip_map,
};

fn shape(dims: impl AsRef<[usize]>) -> Kahen {
    Kahen::new(dims).unwrap()
}
fn tensor<T>(dims: impl AsRef<[usize]>, values: Vec<T>) -> Tensor<T> {
    Tensor::from_vec(values, shape(dims), Order::C).unwrap()
}

#[test]
fn basic_arithmetic_and_division() {
    let x = tensor([2], vec![10i32, 20]);
    let y = tensor([2], vec![2i32, 5]);
    assert_eq!(add(&x, &y).unwrap().storage_slice(), &[12, 25]);
    assert_eq!(sub(&x, &y).unwrap().storage_slice(), &[8, 15]);
    assert_eq!(mul(&x, &y).unwrap().storage_slice(), &[20, 100]);
    assert_eq!(div(&x, &y).unwrap().storage_slice(), &[5, 4]);
    assert_eq!(rem(&x, &y).unwrap().storage_slice(), &[0, 0]);
    assert_eq!(neg(&x).unwrap().storage_slice(), &[-10, -20]);
}

#[test]
fn map_supports_changing_dtype() {
    let x = tensor([2, 2], vec![1u8, 2, 3, 4]);
    let y = map(&x, |v| u16::from(*v) * 10).unwrap();
    assert_eq!(y.storage_slice(), &[10u16, 20, 30, 40]);
}

#[test]
fn zip_map_allows_different_operand_types() {
    let x = tensor([2], vec![1u8, 2]);
    let y = tensor([2], vec![10u16, 20]);
    let z = zip_map(&x, &y, |a, b| u16::from(*a) + *b).unwrap();
    assert_eq!(z.storage_slice(), &[11u16, 22]);
}

#[test]
fn multidimensional_broadcast() {
    let x = tensor([2, 1, 3], (1i32..=6).collect());
    let y = tensor([1, 4, 1], vec![10, 20, 30, 40]);
    let z = add(&x, &y).unwrap();
    assert_eq!(z.shape().dims(), &[2, 4, 3]);
    assert_eq!(*z.get(&[0, 0, 0]).unwrap(), 11);
    assert_eq!(*z.get(&[0, 3, 2]).unwrap(), 43);
    assert_eq!(*z.get(&[1, 2, 1]).unwrap(), 35);
}

#[test]
fn singleton_and_zero_broadcast_produces_empty() {
    let x = tensor([0, 3], Vec::<i32>::new());
    let y = tensor([1, 3], vec![1i32, 2, 3]);
    let z = add(&x, &y).unwrap();
    assert_eq!(z.shape().dims(), &[0, 3]);
    assert!(z.storage_slice().is_empty());
}

#[test]
fn incompatible_broadcast_is_error() {
    let x = tensor([2], vec![1i32, 2]);
    let y = tensor([3], vec![1i32, 2, 3]);
    assert_eq!(
        add(&x, &y).unwrap_err(),
        OpsError::BroadcastIncompatible {
            axis: 0,
            left: 2,
            right: 3
        }
    );
}

#[test]
fn scalar_and_nd_broadcast() {
    let scalar = tensor([], vec![3i32]);
    let x = tensor([2, 2], vec![1, 2, 3, 4]);
    let y = mul(&scalar, &x).unwrap();
    assert_eq!(y.shape().dims(), &[2, 2]);
    assert_eq!(y.storage_slice(), &[3, 6, 9, 12]);
    let z = add(&scalar, &scalar).unwrap();
    assert_eq!(z.shape().dims(), &[]);
    assert_eq!(z.storage_slice(), &[6]);
}

#[test]
fn fixed_rank_input_keeps_generic_read_contract() {
    let x = Tensor::from_vec(
        vec![1i32, 2, 3, 4],
        Kotei::<2>::new([2, 2]).unwrap(),
        Order::C,
    )
    .unwrap();
    let out = add(&x, &x).unwrap();
    assert_eq!(out.storage_slice(), &[2, 4, 6, 8]);
}

#[test]
fn f_order_reads_logical_elements() {
    let x = Tensor::from_vec(vec![1i32, 2, 3, 4, 5, 6], shape([2, 3]), Order::F).unwrap();
    let y = map(&x, |v| v * 10).unwrap();
    assert_eq!(y.storage_slice(), &[10, 30, 50, 20, 40, 60]);
}

#[test]
fn negative_stride_and_transpose_use_logical_order() {
    let x = tensor([2, 3], vec![0i32, 1, 2, 3, 4, 5]);
    let transposed = x.transpose().unwrap();
    let y = add(&transposed, &transposed).unwrap();
    assert_eq!(y.shape().dims(), &[3, 2]);
    assert_eq!(y.storage_slice(), &[0, 6, 2, 8, 4, 10]);
    let reversed = x.reverse_axis(1).unwrap();
    let z = map(&reversed, |v| *v).unwrap();
    assert_eq!(z.storage_slice(), &[2, 1, 0, 5, 4, 3]);
}

#[test]
fn aliasing_broadcast_view_reads_correctly() {
    let x = tensor([1, 3], vec![1i32, 2, 3]);
    let view = x.broadcast_to([4, 3]).unwrap();
    let y = add(&view, &view).unwrap();
    assert_eq!(y.shape().dims(), &[4, 3]);
    assert_eq!(y.storage_slice(), &[2, 4, 6, 2, 4, 6, 2, 4, 6, 2, 4, 6]);
}

#[test]
fn arbitrary_origin_and_gapped_strides() {
    let layout = Layout::from_strides(shape([2, 2]), [4, 2], 1).unwrap();
    let x = Tensor::from_host_storage(
        CpuStorage::try_from_vec(vec![0i32, 1, 2, 3, 4, 5, 6, 7]).unwrap(),
        layout,
    )
    .unwrap();
    let y = map(&x, |v| *v).unwrap();
    assert_eq!(y.storage_slice(), &[1, 3, 5, 7]);
}

#[test]
fn sum_product_and_empty_identity() {
    let x = tensor([2, 2], vec![1i32, 2, 3, 4]);
    assert_eq!(sum_all(&x).unwrap(), 10);
    assert_eq!(prod_all(&x).unwrap(), 24);
    let e = tensor([0, 3], Vec::<i32>::new());
    assert_eq!(sum_all(&e).unwrap(), 0);
    assert_eq!(prod_all(&e).unwrap(), 1);
}

#[test]
fn sum_axes_and_keepdims() {
    let x = tensor([2, 3], vec![1i32, 2, 3, 4, 5, 6]);
    let a = sum_axes(&x, &[0], false).unwrap();
    assert_eq!(a.shape().dims(), &[3]);
    assert_eq!(a.storage_slice(), &[5, 7, 9]);
    let b = sum_axes(&x, &[-1], true).unwrap();
    assert_eq!(b.shape().dims(), &[2, 1]);
    assert_eq!(b.storage_slice(), &[6, 15]);
    let c = sum_axes(&x, &[0, 1], false).unwrap();
    assert_eq!(c.shape().dims(), &[]);
    assert_eq!(c.storage_slice(), &[21]);
}

#[test]
fn sum_non_reduced_axes_returns_copy() {
    let x = tensor([2, 2], vec![1i32, 2, 3, 4]);
    let y = sum_axes(&x, &[], true).unwrap();
    assert_eq!(y.shape().dims(), &[2, 2]);
    assert_eq!(y.storage_slice(), &[1, 2, 3, 4]);
}

#[test]
fn negative_axis_duplicate_axis_and_bad_axis() {
    let x = tensor([2, 2], vec![1i32, 2, 3, 4]);
    assert_eq!(
        sum_axes(&x, &[0, -2], false).unwrap_err(),
        OpsError::DuplicateAxis { axis: 0 }
    );
    assert_eq!(
        sum_axes(&x, &[2], false).unwrap_err(),
        OpsError::AxisOutOfBounds { axis: 2, rank: 2 }
    );
    assert_eq!(
        sum_axes(&x, &[-3], false).unwrap_err(),
        OpsError::AxisOutOfBounds { axis: -3, rank: 2 }
    );
}

#[test]
fn product_axes_with_empty_contract() {
    let x = tensor([0, 3], Vec::<i32>::new());
    let y = prod_axes(&x, &[0], false).unwrap();
    assert_eq!(y.shape().dims(), &[3]);
    assert_eq!(y.storage_slice(), &[1, 1, 1]);
    let z = sum_axes(&x, &[0], false).unwrap();
    assert_eq!(z.storage_slice(), &[0, 0, 0]);
}

#[test]
fn mean_all_and_axis() {
    let x = tensor([2, 3], vec![1f64, 2., 3., 4., 5., 6.]);
    assert_eq!(mean_all(&x).unwrap(), 3.5);
    let y = mean_axes(&x, &[0], false).unwrap();
    assert_eq!(y.storage_slice(), &[2.5, 3.5, 4.5]);
    let z = mean_axes(&x, &[-1], true).unwrap();
    assert_eq!(z.shape().dims(), &[2, 1]);
    assert_eq!(z.storage_slice(), &[2.0, 5.0]);
}

#[test]
fn empty_mean_is_error_unless_output_is_empty() {
    let x = tensor([0, 3], Vec::<f64>::new());
    assert_eq!(mean_all(&x).unwrap_err(), OpsError::EmptyReduction);
    assert_eq!(
        mean_axes(&x, &[0], false).unwrap_err(),
        OpsError::EmptyReduction
    );
    let y = mean_axes(&x, &[1], false).unwrap();
    assert_eq!(y.shape().dims(), &[0]);
    assert!(y.storage_slice().is_empty());
}

#[test]
fn extrema_and_empty() {
    let x = tensor([2, 2], vec![10i32, -5, 2, 40]);
    assert_eq!(min_all(&x).unwrap(), Some(-5));
    assert_eq!(max_all(&x).unwrap(), Some(40));
    let empty = tensor([0], Vec::<i32>::new());
    assert_eq!(min_all(&empty).unwrap(), None);
    assert_eq!(max_all(&empty).unwrap(), None);
}

#[test]
fn matmul_2d() {
    let x = tensor([2, 3], vec![1i32, 2, 3, 4, 5, 6]);
    let y = tensor([3, 2], vec![7i32, 8, 9, 10, 11, 12]);
    let z = matmul(&x, &y).unwrap();
    assert_eq!(z.shape().dims(), &[2, 2]);
    assert_eq!(z.storage_slice(), &[58, 64, 139, 154]);
}

#[test]
fn matmul_vector_dot_scalar_and_outer_like() {
    let x = tensor([3], vec![1i32, 2, 3]);
    let y = tensor([3], vec![4i32, 5, 6]);
    let dot = matmul(&x, &y).unwrap();
    assert_eq!(dot.shape().dims(), &[]);
    assert_eq!(dot.storage_slice(), &[32]);
    let a = tensor([3, 1], vec![1i32, 2, 3]);
    let b = tensor([1, 2], vec![5i32, 6]);
    let outer = matmul(&a, &b).unwrap();
    assert_eq!(outer.shape().dims(), &[3, 2]);
    assert_eq!(outer.storage_slice(), &[5, 6, 10, 12, 15, 18]);
}

#[test]
fn matmul_matrix_vector_vector_matrix() {
    let matrix = tensor([2, 3], vec![1i32, 2, 3, 4, 5, 6]);
    let x = tensor([3], vec![10i32, 20, 30]);
    let y = matmul(&matrix, &x).unwrap();
    assert_eq!(y.shape().dims(), &[2]);
    assert_eq!(y.storage_slice(), &[140, 320]);
    let v = tensor([2], vec![10i32, 20]);
    let z = matmul(&v, &matrix).unwrap();
    assert_eq!(z.shape().dims(), &[3]);
    assert_eq!(z.storage_slice(), &[90, 120, 150]);
}

#[test]
fn matmul_batch_broadcast() {
    let a = tensor([2, 1, 2, 2], vec![1i32, 2, 3, 4, 5, 6, 7, 8]);
    let b = tensor([1, 3, 2, 1], vec![1i32, 2, 3, 4, 5, 6]);
    let out = matmul(&a, &b).unwrap();
    assert_eq!(out.shape().dims(), &[2, 3, 2, 1]);
    assert_eq!(
        out.storage_slice(),
        &[5, 11, 11, 25, 17, 39, 17, 23, 39, 53, 61, 83]
    );
}

#[test]
fn matmul_handles_negative_stride_and_f_order() {
    let x = tensor([2, 2], vec![1i32, 2, 3, 4]);
    let xt = x.transpose().unwrap();
    let y = Tensor::from_vec(vec![5i32, 7, 6, 8], shape([2, 2]), Order::F).unwrap();
    let z = matmul(&xt, &y).unwrap();
    assert_eq!(z.storage_slice(), &[26, 30, 38, 44]);
}

#[test]
fn matmul_empty_contraction_has_zero_identity() {
    let x = tensor([2, 0], Vec::<i32>::new());
    let y = tensor([0, 3], Vec::<i32>::new());
    let result = matmul(&x, &y).unwrap();
    assert_eq!(result.shape().dims(), &[2, 3]);
    assert_eq!(result.storage_slice(), &[0, 0, 0, 0, 0, 0]);
}

#[test]
fn matmul_rejects_rank0_and_wrong_k() {
    let x = tensor([], vec![2i32]);
    let y = tensor([2], vec![1i32, 2]);
    assert_eq!(
        matmul(&x, &y).unwrap_err(),
        OpsError::InvalidRank {
            operation: "matmul",
            minimum: 1,
            actual: 0
        }
    );
    let a = tensor([2, 3], vec![0i32; 6]);
    let b = tensor([4, 2], vec![0i32; 8]);
    assert_eq!(
        matmul(&a, &b).unwrap_err(),
        OpsError::ContractDimensionMismatch { left: 3, right: 4 }
    );
}

#[test]
fn scalar_sum_preserves_single_element() {
    let x = tensor([], vec![5i32]);
    assert_eq!(sum_all(&x).unwrap(), 5);
    let out = sum_axes(&x, &[], false).unwrap();
    assert_eq!(out.shape().dims(), &[]);
    assert_eq!(out.storage_slice(), &[5]);
}

#[test]
fn in_place_broadcast_and_assignment() {
    use kernix_ops::{add_assign, assign, mul_assign};
    let mut x = tensor([2, 3], vec![1i32, 2, 3, 4, 5, 6]);
    let row = tensor([3], vec![10i32, 20, 30]);
    add_assign(&mut x, &row).unwrap();
    assert_eq!(x.storage_slice(), &[11, 22, 33, 14, 25, 36]);
    mul_assign(&mut x, &tensor([], vec![2i32])).unwrap();
    assert_eq!(x.storage_slice(), &[22, 44, 66, 28, 50, 72]);
    assign(&mut x, &row).unwrap();
    assert_eq!(x.storage_slice(), &[10, 20, 30, 10, 20, 30]);
}

#[test]
fn in_place_rejects_broadcast_destination_aliases() {
    use kernix_ops::assign;
    let mut x = tensor([1], vec![0i32]);
    let mut view = x.view_mut().unwrap().broadcast_to([3]).unwrap();
    let rhs = tensor([3], vec![4i32, 5, 6]);
    assert_eq!(
        assign(&mut view, &rhs).unwrap_err(),
        OpsError::OverlappingOutput
    );
}

#[test]
fn in_place_rejects_unprovable_overlap() {
    use kernix_ops::assign;
    let layout = Layout::from_strides(shape([2, 2]), [1, 1], 0).unwrap();
    let mut x =
        Tensor::from_host_storage(CpuStorage::try_from_vec(vec![1i32, 2, 3]).unwrap(), layout)
            .unwrap();
    let rhs = tensor([2, 2], vec![0i32, 1, 2, 3]);
    assert_eq!(
        assign(&mut x, &rhs).unwrap_err(),
        OpsError::OverlappingOutput
    );
}

#[test]
fn extrema_axis_keepdims_and_empty_behavior() {
    use kernix_ops::{max_axes, min_axes};
    let x = tensor([2, 3], vec![2i32, 9, 3, 5, 1, 8]);
    let lo = min_axes(&x, &[0], false).unwrap();
    assert_eq!(lo.storage_slice(), &[2, 1, 3]);
    let hi = max_axes(&x, &[1], true).unwrap();
    assert_eq!(hi.shape().dims(), &[2, 1]);
    assert_eq!(hi.storage_slice(), &[9, 8]);
    let empty = tensor([0, 3], Vec::<i32>::new());
    assert_eq!(
        min_axes(&empty, &[0], false).unwrap_err(),
        OpsError::EmptyReduction
    );
    assert!(
        max_axes(&empty, &[1], false)
            .unwrap()
            .storage_slice()
            .is_empty()
    );
}
