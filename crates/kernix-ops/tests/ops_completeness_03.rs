//! Integration regressions for ops completeness 03.
use kernix_core::{
    layout::Order,
    shape::{Kahen, Kotei},
    tensor::Tensor,
};
use kernix_ops::{
    OpsError, SortDirection, argsort_axis, cast, chunk_axis, dot, fold_axis, l2_norm_all,
    l2_norm_axis, logsumexp_all, logsumexp_axis, masked_select, nonzero, outer, roll_axis,
    scatter_axis, sort_axis, split_axis, take_axis, tensordot, tile, topk_axis, trace, try_cast,
    unfold_axis,
};

fn t<T>(dims: impl AsRef<[usize]>, values: Vec<T>) -> Tensor<T> {
    Tensor::from_vec(values, Kahen::new(dims).unwrap(), Order::C).unwrap()
}

#[test]
fn take_axis_replaces_axis_with_matrix_of_indices() {
    let src = t([2, 3], vec![10i32, 11, 12, 20, 21, 22]);
    let indices = t([2, 2], vec![2usize, 0, 1, 1]);
    let out = take_axis(&src, -1, &indices).unwrap();
    assert_eq!(out.shape().dims(), &[2, 2, 2]);
    assert_eq!(out.storage_slice(), &[12, 10, 11, 11, 22, 20, 21, 21]);
}

#[test]
fn take_scalar_indices_remove_selected_axis() {
    let src = t([2, 3], vec![1i32, 2, 3, 4, 5, 6]);
    let index = t([], vec![1usize]);
    let out = take_axis(&src, 1, &index).unwrap();
    assert_eq!(out.shape().dims(), &[2]);
    assert_eq!(out.storage_slice(), &[2, 5]);
}

#[test]
fn take_checks_bad_indices_even_if_nonselected_axis_empty() {
    let src = t([0, 2], Vec::<i32>::new());
    let indices = t([2], vec![1usize, 2]);
    assert!(matches!(
        take_axis(&src, 1, &indices),
        Err(OpsError::IndexOutOfBounds {
            axis: 1,
            index: 2,
            dim: 2
        })
    ));
}

#[test]
fn scatter_repeats_last_update_and_keeps_source_unchanged() {
    let original = t([2, 3], vec![0i32, 1, 2, 3, 4, 5]);
    let updates = t([2, 2], vec![10, 11, 20, 21]);
    let result = scatter_axis(&original, -1, &[2, 2], &updates).unwrap();
    assert_eq!(result.storage_slice(), &[0, 1, 11, 3, 4, 21]);
    assert_eq!(original.storage_slice(), &[0, 1, 2, 3, 4, 5]);
}

#[test]
fn scatter_rejects_oob_indices_on_empty_other_axes() {
    let source = t([0, 3], Vec::<i32>::new());
    let updates = t([0, 1], Vec::<i32>::new());
    assert!(matches!(
        scatter_axis(&source, 1, &[3], &updates),
        Err(OpsError::IndexOutOfBounds { .. })
    ));
}

#[test]
fn masked_select_broadcasts_and_flattens_logical_order() {
    let src = t([2, 3], vec![1i32, 2, 3, 4, 5, 6]);
    let reversed = src.reverse_axis(1).unwrap();
    let mask = t([3], vec![true, false, true]);
    let result = masked_select(&reversed, &mask).unwrap();
    assert_eq!(result.shape().dims(), &[4]);
    assert_eq!(result.storage_slice(), &[3, 1, 6, 4]);
}

#[test]
fn nonzero_supports_scalar_and_empty_tensors() {
    let src = t([2, 2], vec![0i32, 1, 2, 0]);
    let idx = nonzero(&src).unwrap();
    assert_eq!(idx.shape().dims(), &[2, 2]);
    assert_eq!(idx.storage_slice(), &[0, 1, 1, 0]);
    let yes = t([], vec![9i32]);
    let scalar = nonzero(&yes).unwrap();
    assert_eq!(scalar.shape().dims(), &[1, 0]);
    assert_eq!(scalar.storage_slice(), &[]);
    let zero = t([0, 4], Vec::<i32>::new());
    assert_eq!(nonzero(&zero).unwrap().shape().dims(), &[0, 2]);
}

#[test]
fn split_and_chunk_handle_uneven_and_empty_sections() {
    let src = t([5], vec![1i32, 2, 3, 4, 5]);
    let parts = split_axis(&src, -1, &[2, 0, 3]).unwrap();
    assert_eq!(parts[0].storage_slice(), &[1, 2]);
    assert!(parts[1].storage_slice().is_empty());
    assert_eq!(parts[2].storage_slice(), &[3, 4, 5]);
    let chunks = chunk_axis(&src, 0, 2).unwrap();
    assert_eq!(chunks.len(), 3);
    assert_eq!(chunks[2].storage_slice(), &[5]);
    assert!(matches!(
        chunk_axis(&src, 0, 0),
        Err(OpsError::InvalidParameter { .. })
    ));
    assert!(matches!(
        split_axis(&src, 0, &[4]),
        Err(OpsError::SplitSizesMismatch { .. })
    ));
}

#[test]
fn tile_is_block_repeat_and_broadcasts_rank() {
    let src = t([2], vec![1i32, 2]);
    let out = tile(&src, &[2, 3]).unwrap();
    assert_eq!(out.shape().dims(), &[2, 6]);
    assert_eq!(out.storage_slice(), &[1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2]);
    let scalar = t([], vec![7i32]);
    assert_eq!(
        tile(&scalar, &[2, 2]).unwrap().storage_slice(),
        &[7, 7, 7, 7]
    );
    assert_eq!(tile(&src, &[0]).unwrap().shape().dims(), &[0]);
}

#[test]
fn roll_handles_sign_and_min_integer_without_overflow() {
    let src = t([5], vec![0i32, 1, 2, 3, 4]);
    assert_eq!(
        roll_axis(&src, 0, 2).unwrap().storage_slice(),
        &[3, 4, 0, 1, 2]
    );
    assert_eq!(
        roll_axis(&src, 0, -1).unwrap().storage_slice(),
        &[1, 2, 3, 4, 0]
    );
    let by = isize::MIN;
    let src_index = by.unsigned_abs() % 5;
    let out = roll_axis(&src, 0, by).unwrap();
    assert_eq!(out.storage_slice()[0], src.storage_slice()[src_index]);
}

#[test]
fn unfold_and_fold_sliding_windows() {
    let src = t([5], vec![1i32, 2, 3, 4, 5]);
    let windows = unfold_axis(&src, 0, 3, 1, 1).unwrap();
    assert_eq!(windows.shape().dims(), &[3, 3]);
    assert_eq!(windows.storage_slice(), &[1, 2, 3, 2, 3, 4, 3, 4, 5]);
    let folded = fold_axis(&windows, 0, 5, 1, 1).unwrap();
    assert_eq!(folded.shape().dims(), &[5]);
    assert_eq!(folded.storage_slice(), &[1, 4, 9, 8, 5]);
}

#[test]
fn unfold_supports_dilation_and_invalid_parameters() {
    let src = t([7], vec![0, 1, 2, 3, 4, 5, 6]);
    let windows = unfold_axis(&src, -1, 3, 2, 2).unwrap();
    assert_eq!(windows.shape().dims(), &[2, 3]);
    assert_eq!(windows.storage_slice(), &[0, 2, 4, 2, 4, 6]);
    assert!(matches!(
        unfold_axis(&src, 0, 0, 1, 1),
        Err(OpsError::InvalidParameter { .. })
    ));
}

#[test]
fn sort_handles_floats_nan_zero_and_stable_ties() {
    let src = t([5], vec![f32::NAN, 2.0, -0.0, 0.0, 2.0]);
    let sorted = sort_axis(&src, 0, SortDirection::Ascending).unwrap();
    assert_eq!(&sorted.storage_slice()[..4], &[-0.0, 0.0, 2.0, 2.0]);
    assert!(sorted.storage_slice()[4].is_nan());
    let ids = argsort_axis(&src, 0, SortDirection::Descending).unwrap();
    assert_eq!(ids.storage_slice(), &[0, 1, 4, 3, 2]);
}

#[test]
fn topk_axis_keeps_first_tie_and_nonlast_axis_correct() {
    let src = t([3, 2], vec![1i32, 8, 9, 2, 9, 3]);
    let (values, positions) = topk_axis(&src, 0, 2).unwrap();
    assert_eq!(values.shape().dims(), &[2, 2]);
    assert_eq!(values.storage_slice(), &[9, 8, 9, 3]);
    assert_eq!(positions.storage_slice(), &[1, 0, 2, 2]);
    assert!(matches!(
        topk_axis(&src, 0, 4),
        Err(OpsError::IndexOutOfBounds { .. })
    ));
}

#[test]
fn sorting_fixed_rank_and_reversed_views() {
    let src = Tensor::from_vec(
        vec![3i32, 1, 2, 8, 4, 6],
        Kotei::<2>::new([2, 3]).unwrap(),
        Order::C,
    )
    .unwrap();
    let backwards = src.reverse_axis(1).unwrap();
    let sorted = sort_axis(&backwards, 1, SortDirection::Ascending).unwrap();
    assert_eq!(sorted.storage_slice(), &[1, 2, 3, 4, 6, 8]);
}

#[test]
fn dot_outer_and_trace_cover_empty_and_offset() {
    let x = t([3], vec![1i32, 2, 3]);
    let y = t([3], vec![4i32, 5, 6]);
    assert_eq!(dot(&x, &y).unwrap(), 32);
    let e = t([0], Vec::<i32>::new());
    assert_eq!(dot(&e, &e).unwrap(), 0);
    let v = t([2], vec![5i32, 7]);
    assert_eq!(
        outer(&v, &x).unwrap().storage_slice(),
        &[5, 10, 15, 7, 14, 21]
    );
    let matrix = t([2, 3], vec![1i32, 2, 3, 4, 5, 6]);
    assert_eq!(trace(&matrix, 0).unwrap(), 6);
    assert_eq!(trace(&matrix, 1).unwrap(), 8);
    assert_eq!(trace(&matrix, -1).unwrap(), 4);
    assert_eq!(trace(&matrix, 10).unwrap(), 0);
}

#[test]
fn tensordot_contracts_arbitrary_axes_and_empty_k() {
    let x = t([2, 3], vec![1i32, 2, 3, 4, 5, 6]);
    let y = t([4, 3], vec![1i32, 0, 0, 0, 1, 0, 0, 0, 1, 1, 1, 1]);
    let z = tensordot(&x, &y, &[1], &[1]).unwrap();
    assert_eq!(z.shape().dims(), &[2, 4]);
    assert_eq!(z.storage_slice(), &[1, 2, 3, 6, 4, 5, 6, 15]);
    let a = t([2, 0], Vec::<i32>::new());
    let b = t([0, 3], Vec::<i32>::new());
    let zeros = tensordot(&a, &b, &[1], &[0]).unwrap();
    assert_eq!(zeros.shape().dims(), &[2, 3]);
    assert_eq!(zeros.storage_slice(), &[0, 0, 0, 0, 0, 0]);
    assert!(matches!(
        tensordot(&x, &y, &[0], &[1]),
        Err(OpsError::ContractDimensionMismatch { .. })
    ));
}

#[test]
fn logsumexp_shift_is_stable_and_checks_nonfinite() {
    let x = t([2], vec![1000f64, 1000.]);
    let answer = logsumexp_all(&x).unwrap();
    assert!((answer - (1000.0 + core::f64::consts::LN_2)).abs() < 1e-10);
    let matrix = t([2, 2], vec![1000f64, 1000., 0., 0.]);
    let axis = logsumexp_axis(&matrix, -1, true).unwrap();
    assert_eq!(axis.shape().dims(), &[2, 1]);
    assert!((axis.storage_slice()[0] - (1000.0 + core::f64::consts::LN_2)).abs() < 1e-10);
    assert!((axis.storage_slice()[1] - core::f64::consts::LN_2).abs() < 1e-10);
    let bad = t([2], vec![1f64, f64::INFINITY]);
    assert!(matches!(
        logsumexp_all(&bad),
        Err(OpsError::NonFiniteInput { .. })
    ));
}

#[test]
fn l2_norm_scaled_avoids_overflow_and_tiny_underflow() {
    let x = t([2], vec![1.0e200f64, 1.0e200]);
    let result = l2_norm_all(&x).unwrap();
    assert!(result.is_finite());
    assert!((result / 1.0e200 - 2f64.sqrt()).abs() < 1e-12);
    let tiny = t([2], vec![1.0e-200f64, 1.0e-200]);
    assert!(l2_norm_all(&tiny).unwrap() > 0.);
    let empty = t([0], Vec::<f64>::new());
    assert_eq!(l2_norm_all(&empty).unwrap(), 0.);
}

#[test]
fn l2_norm_axis_handles_strides_and_empty_reduction() {
    let x = t([2, 2], vec![3f64, 4., 5., 12.]);
    let backwards = x.reverse_axis(1).unwrap();
    let norm = l2_norm_axis(&backwards, -1, true).unwrap();
    assert_eq!(norm.shape().dims(), &[2, 1]);
    assert_eq!(norm.storage_slice(), &[5., 13.]);
    let empty = t([2, 0], Vec::<f64>::new());
    assert_eq!(
        l2_norm_axis(&empty, 1, false).unwrap().storage_slice(),
        &[0., 0.]
    );
}

#[test]
fn cast_has_explicit_lossless_and_checked_semantics() {
    let x = t([3], vec![1u8, 2, 255]);
    let bigger: Tensor<u16> = cast(&x).unwrap();
    assert_eq!(bigger.storage_slice(), &[1, 2, 255]);
    let from = t([3], vec![4i32, 256, 1]);
    let bad: Result<Tensor<u8>, OpsError> = try_cast(&from);
    assert!(matches!(bad, Err(OpsError::Cast { indices }) if indices == vec![1]));
}

#[test]
fn zero_sized_sort_and_topk_do_not_iterate_enormous_outer_axes() {
    let empty = t([usize::MAX, 0], Vec::<i32>::new());
    assert!(
        sort_axis(&empty, 1, SortDirection::Ascending)
            .unwrap()
            .is_empty()
    );
    assert!(
        argsort_axis(&empty, 1, SortDirection::Ascending)
            .unwrap()
            .is_empty()
    );
    let (v, i) = topk_axis(&empty, 1, 0).unwrap();
    assert_eq!(v.shape().dims(), &[usize::MAX, 0]);
    assert!(i.is_empty());
}

#[test]
fn tensordot_scalar_outer_and_zero_result_shapes() {
    let scalar = t([], vec![3i32]);
    let vector = t([2], vec![5i32, 7]);
    let product = tensordot(&scalar, &vector, &[], &[]).unwrap();
    assert_eq!(product.shape().dims(), &[2]);
    assert_eq!(product.storage_slice(), &[15, 21]);
    let a = t([0, 2], Vec::<i32>::new());
    let b = t([2, 3], vec![1i32, 2, 3, 4, 5, 6]);
    let out = tensordot(&a, &b, &[1], &[0]).unwrap();
    assert_eq!(out.shape().dims(), &[0, 3]);
    assert!(out.is_empty());
}
