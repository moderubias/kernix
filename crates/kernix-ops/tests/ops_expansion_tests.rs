//! Integration regressions for ops expansion tests.
use kernix_core::{
    layout::{Layout, Order},
    shape::{Kahen, Kotei},
    storage::CpuStorage,
    tensor::Tensor,
};
use kernix_ops::{
    OpsError, TensorInput, abs, all_all, all_axes, any_all, any_axes, argmax_all, argmax_axis,
    argmin_all, argmin_axis, clamp, concat, count_nonzero_all, count_nonzero_axes, cumprod_axis,
    cumsum_axis, equal, exp, gather_axis, gelu, greater, greater_equal, is_finite, is_nan,
    leaky_relu, less, less_equal, ln, log_softmax, logical_and, logical_not, logical_or,
    logical_xor, not_equal, pad_constant, relu, repeat_axis, sigmoid, silu, softmax, softplus,
    sqrt, stack, tanh, where_select,
};

fn t<T>(dims: impl AsRef<[usize]>, values: Vec<T>) -> Tensor<T> {
    Tensor::from_vec(values, Kahen::new(dims).unwrap(), Order::C).unwrap()
}

#[test]
fn comparisons_broadcast_all_six_relations() {
    let a = t([2, 1], vec![2i32, 4]);
    let b = t([3], vec![2i32, 3, 4]);
    assert_eq!(
        equal(&a, &b).unwrap().storage_slice(),
        &[true, false, false, false, false, true]
    );
    assert_eq!(
        not_equal(&a, &b).unwrap().storage_slice(),
        &[false, true, true, true, true, false]
    );
    assert_eq!(
        less(&a, &b).unwrap().storage_slice(),
        &[false, true, true, false, false, false]
    );
    assert_eq!(
        less_equal(&a, &b).unwrap().storage_slice(),
        &[true, true, true, false, false, true]
    );
    assert_eq!(
        greater(&a, &b).unwrap().storage_slice(),
        &[false, false, false, true, true, false]
    );
    assert_eq!(
        greater_equal(&a, &b).unwrap().storage_slice(),
        &[true, false, false, true, true, true]
    );
}

#[test]
fn comparisons_follow_ieee_nan_semantics() {
    let a = t([2], vec![f32::NAN, 1.]);
    let b = t([2], vec![f32::NAN, 2.]);
    assert_eq!(equal(&a, &b).unwrap().storage_slice(), &[false, false]);
    assert_eq!(not_equal(&a, &b).unwrap().storage_slice(), &[true, true]);
    assert_eq!(less(&a, &b).unwrap().storage_slice(), &[false, true]);
}

#[test]
fn logical_operators_support_broadcasting() {
    let a = t([2, 1], vec![true, false]);
    let b = t([2], vec![true, false]);
    assert_eq!(
        logical_and(&a, &b).unwrap().storage_slice(),
        &[true, false, false, false]
    );
    assert_eq!(
        logical_or(&a, &b).unwrap().storage_slice(),
        &[true, true, true, false]
    );
    assert_eq!(
        logical_xor(&a, &b).unwrap().storage_slice(),
        &[false, true, true, false]
    );
    assert_eq!(logical_not(&a).unwrap().storage_slice(), &[false, true]);
}

#[test]
fn where_select_broadcasts_three_inputs_and_short_circuits() {
    let mask = t([2, 1], vec![true, false]);
    let yes = t([1, 3], vec![10i32, 20, 30]);
    let no = t([], vec![-1i32]);
    let y = where_select(&mask, &yes, &no).unwrap();
    assert_eq!(y.shape().dims(), &[2, 3]);
    assert_eq!(y.storage_slice(), &[10, 20, 30, -1, -1, -1]);
}

#[test]
fn zero_one_broadcast_is_empty_even_in_where_select() {
    let mask = t([0, 2], Vec::<bool>::new());
    let yes = t([1, 2], vec![1i32, 2]);
    let no = t([], vec![0]);
    let result = where_select(&mask, &yes, &no).unwrap();
    assert_eq!(result.shape().dims(), &[0, 2]);
    assert!(result.storage_slice().is_empty());
}

#[test]
fn clamp_rejects_invalid_bounds_and_preserves_nan() {
    let x = t([4], vec![-2f32, 0., 5., f32::NAN]);
    let y = clamp(&x, -1., 2.).unwrap();
    assert_eq!(&y.storage_slice()[..3], &[-1., 0., 2.]);
    assert!(y.storage_slice()[3].is_nan());
    assert_eq!(clamp(&x, 2., -1.).unwrap_err(), OpsError::InvalidBounds);
    assert_eq!(
        clamp(&x, f32::NAN, 1.).unwrap_err(),
        OpsError::InvalidBounds
    );
}

#[test]
fn concat_handles_arbitrary_views_and_negative_axis() {
    let a = t([2, 2], vec![1i32, 2, 3, 4]);
    let b = t([2, 2], vec![5i32, 6, 7, 8]);
    let reversed = b.reverse_axis(1).unwrap();
    let pieces: [&dyn TensorInput<i32>; 2] = [&a, &reversed];
    let joined = concat(&pieces, -1).unwrap();
    assert_eq!(joined.shape().dims(), &[2, 4]);
    assert_eq!(joined.storage_slice(), &[1, 2, 6, 5, 3, 4, 8, 7]);
}

#[test]
fn concat_disallows_rank_and_dimension_mismatches() {
    let a = t([2, 2], vec![1i32; 4]);
    let b = t([2, 3], vec![1i32; 6]);
    let inputs: [&dyn TensorInput<i32>; 2] = [&a, &b];
    assert!(matches!(
        concat(&inputs, 0),
        Err(OpsError::DimensionMismatch { axis: 1, .. })
    ));
    let nothing: [&dyn TensorInput<i32>; 0] = [];
    assert!(matches!(
        concat(&nothing, 0),
        Err(OpsError::EmptyInputSequence { .. })
    ));
}

#[test]
fn concat_can_copy_empty_inputs() {
    let a = t([0, 2], Vec::<u8>::new());
    let b = t([1, 2], vec![4u8, 5]);
    let joined = concat(&[&a, &b], 0).unwrap();
    assert_eq!(joined.shape().dims(), &[1, 2]);
    assert_eq!(joined.storage_slice(), &[4, 5]);
}

#[test]
fn stack_supports_scalars_last_axis_and_empty_inputs() {
    let a = t([], vec![10i32]);
    let b = t([], vec![20i32]);
    let x = stack(&[&a, &b], 0).unwrap();
    assert_eq!(x.shape().dims(), &[2]);
    assert_eq!(x.storage_slice(), &[10, 20]);
    let a = t([2], vec![1i32, 2]);
    let b = t([2], vec![3i32, 4]);
    let y = stack(&[&a, &b], -1).unwrap();
    assert_eq!(y.shape().dims(), &[2, 2]);
    assert_eq!(y.storage_slice(), &[1, 3, 2, 4]);
}

#[test]
fn gather_checks_indices_even_when_output_empty() {
    let a = t([0, 3], Vec::<i32>::new());
    assert!(matches!(
        gather_axis(&a, 1, &[4]),
        Err(OpsError::IndexOutOfBounds {
            axis: 1,
            index: 4,
            dim: 3
        })
    ));
    let y = gather_axis(&a, 1, &[2, 0]).unwrap();
    assert_eq!(y.shape().dims(), &[0, 2]);
}

#[test]
fn gather_preserves_duplicate_and_ordered_indices_on_f_order() {
    let a = Tensor::from_vec(
        vec![1i32, 4, 2, 5, 3, 6],
        Kahen::new([2, 3]).unwrap(),
        Order::F,
    )
    .unwrap();
    let result = gather_axis(&a, -1, &[2, 0, 2]).unwrap();
    assert_eq!(result.shape().dims(), &[2, 3]);
    assert_eq!(result.storage_slice(), &[3, 1, 3, 6, 4, 6]);
}

#[test]
fn repeat_and_zero_times() {
    let a = t([2, 2], vec![1i32, 2, 3, 4]);
    let x = repeat_axis(&a, 0, 2).unwrap();
    assert_eq!(x.shape().dims(), &[4, 2]);
    assert_eq!(x.storage_slice(), &[1, 2, 1, 2, 3, 4, 3, 4]);
    let empty = repeat_axis(&a, -1, 0).unwrap();
    assert_eq!(empty.shape().dims(), &[2, 0]);
}

#[test]
fn pad_constant_handles_empty_source_and_scalar() {
    let empty = t([0], Vec::<i32>::new());
    let padded = pad_constant(&empty, &[(2, 1)], 7).unwrap();
    assert_eq!(padded.storage_slice(), &[7, 7, 7]);
    let scalar = t([], vec![42i32]);
    let same = pad_constant(&scalar, &[], 0).unwrap();
    assert_eq!(same.storage_slice(), &[42]);
}

#[test]
fn pad_constant_two_axes() {
    let a = t([2, 2], vec![1i32, 2, 3, 4]);
    let result = pad_constant(&a, &[(1, 0), (1, 1)], -1).unwrap();
    assert_eq!(result.shape().dims(), &[3, 4]);
    assert_eq!(
        result.storage_slice(),
        &[-1, -1, -1, -1, -1, 1, 2, -1, -1, 3, 4, -1]
    );
}

#[test]
fn boolean_reductions_have_identity_on_empty() {
    let a = t([0, 3], Vec::<bool>::new());
    assert!(!any_all(&a).unwrap());
    assert!(all_all(&a).unwrap());
    assert_eq!(
        any_axes(&a, &[0], false).unwrap().storage_slice(),
        &[false, false, false]
    );
    assert_eq!(
        all_axes(&a, &[0], false).unwrap().storage_slice(),
        &[true, true, true]
    );
}

#[test]
fn any_all_axes_accept_negative_and_keepdims() {
    let a = t([2, 2], vec![false, true, true, true]);
    assert!(any_all(&a).unwrap());
    assert!(!all_all(&a).unwrap());
    assert_eq!(
        any_axes(&a, &[-1], true).unwrap().storage_slice(),
        &[true, true]
    );
    assert_eq!(
        all_axes(&a, &[0], false).unwrap().storage_slice(),
        &[false, true]
    );
}

#[test]
fn count_nonzero_supports_strided_and_nan() {
    let a = t([2, 2], vec![0f64, 1., f64::NAN, 0.]);
    assert_eq!(count_nonzero_all(&a).unwrap(), 2);
    let y = count_nonzero_axes(&a, &[1], false).unwrap();
    assert_eq!(y.storage_slice(), &[1, 1]);
}

#[test]
fn argmin_argmax_use_logical_indices_and_first_ties() {
    let a = t([2, 3], vec![3i32, 1, 1, 9, 9, 2]);
    assert_eq!(argmin_all(&a).unwrap(), Some(1));
    assert_eq!(argmax_all(&a).unwrap(), Some(3));
    assert_eq!(argmin_axis(&a, 1, false).unwrap().storage_slice(), &[1, 2]);
    assert_eq!(
        argmax_axis(&a, 0, true).unwrap().storage_slice(),
        &[1, 1, 1]
    );
}

#[test]
fn argmin_nan_policy_and_empty() {
    let a = t([3], vec![10f64, f64::NAN, -99.]);
    assert_eq!(argmin_all(&a).unwrap(), Some(1));
    assert_eq!(argmax_all(&a).unwrap(), Some(1));
    let empty = t([0], Vec::<i32>::new());
    assert_eq!(argmin_all(&empty).unwrap(), None);
    assert_eq!(argmax_all(&empty).unwrap(), None);
    assert_eq!(
        argmin_axis(&empty, 0, false).unwrap_err(),
        OpsError::EmptyReduction
    );
}

#[test]
fn cumsum_cumprod_on_both_axes() {
    let a = t([2, 3], vec![1i32, 2, 3, 4, 5, 6]);
    assert_eq!(
        cumsum_axis(&a, 1).unwrap().storage_slice(),
        &[1, 3, 6, 4, 9, 15]
    );
    assert_eq!(
        cumsum_axis(&a, 0).unwrap().storage_slice(),
        &[1, 2, 3, 5, 7, 9]
    );
    assert_eq!(
        cumprod_axis(&a, -1).unwrap().storage_slice(),
        &[1, 2, 6, 4, 20, 120]
    );
}

#[test]
fn scans_skip_computation_on_empty_dimensions() {
    let x = t([usize::MAX, 0], Vec::<i32>::new());
    let y = cumsum_axis(&x, 1).unwrap();
    assert_eq!(y.shape().dims(), &[usize::MAX, 0]);
    assert!(y.storage_slice().is_empty());
}

#[test]
fn comparisons_and_gather_accept_fixed_shapes_and_gapped_layouts() {
    let fixed = Tensor::from_vec(vec![1i32, 2], Kotei::<1>::new([2]).unwrap(), Order::C).unwrap();
    assert_eq!(
        equal(&fixed, &fixed).unwrap().storage_slice(),
        &[true, true]
    );
    let layout = Layout::from_strides(Kahen::new([2]).unwrap(), [2], 1).unwrap();
    let x = Tensor::from_host_storage(
        CpuStorage::try_from_vec(vec![0i32, 7, 0, 8]).unwrap(),
        layout,
    )
    .unwrap();
    assert_eq!(
        gather_axis(&x, 0, &[1, 0]).unwrap().storage_slice(),
        &[8, 7]
    );
}

#[test]
fn math_unaries_and_nan_masks() {
    let a = t([3], vec![1f64, 4., 9.]);
    let b = sqrt(&a).unwrap();
    assert_eq!(b.storage_slice(), &[1., 2., 3.]);
    let c = ln(&exp(&a).unwrap()).unwrap();
    for (actual, expected) in c.storage_slice().iter().zip(a.storage_slice()) {
        assert!((actual - expected).abs() < 1e-12);
    }
    assert_eq!(
        abs(&t([2], vec![-3f64, 2.])).unwrap().storage_slice(),
        &[3., 2.]
    );
    let values = t([3], vec![1f64, f64::NAN, f64::INFINITY]);
    assert_eq!(
        is_nan(&values).unwrap().storage_slice(),
        &[false, true, false]
    );
    assert_eq!(
        is_finite(&values).unwrap().storage_slice(),
        &[true, false, false]
    );
}

#[test]
fn activations_have_expected_special_values() {
    let x = t([3], vec![-1000f64, 0., 1000.]);
    let s = sigmoid(&x).unwrap();
    assert_eq!(s.storage_slice(), &[0., 0.5, 1.]);
    let r = relu(&x).unwrap();
    assert_eq!(r.storage_slice(), &[0., 0., 1000.]);
    let lr = leaky_relu(&x, 0.01).unwrap();
    assert_eq!(lr.storage_slice(), &[-10., 0., 1000.]);
    let g = gelu(&t([1], vec![0f64])).unwrap();
    assert_eq!(g.storage_slice(), &[0.]);
    assert!((silu(&t([1], vec![0f64])).unwrap().storage_slice()[0]).abs() < 1e-12);
    assert_eq!(tanh(&t([1], vec![0f64])).unwrap().storage_slice(), &[0.]);
    let p = softplus(&x).unwrap();
    assert!(p.storage_slice()[0] >= 0. && p.storage_slice()[0] < 1e-100);
    assert!((p.storage_slice()[1] - core::f64::consts::LN_2).abs() < 1e-12);
    assert!((p.storage_slice()[2] - 1000.).abs() < 1e-9);
}

#[test]
fn softmax_is_stable_for_large_logits_and_axis() {
    let x = t([2, 3], vec![1000f64, 1001., 1002., -1000., -1000., -1000.]);
    let y = softmax(&x, -1).unwrap();
    assert_eq!(y.shape().dims(), &[2, 3]);
    assert!((y.storage_slice()[0] - 0.09003057317).abs() < 1e-9);
    assert!((y.storage_slice()[1] - 0.24472847105).abs() < 1e-9);
    assert!((y.storage_slice()[2] - 0.66524095577).abs() < 1e-9);
    assert!((y.storage_slice()[3] - 1. / 3.).abs() < 1e-12);
    let l = log_softmax(&x, 1).unwrap();
    for (p, q) in y.storage_slice().iter().zip(l.storage_slice()) {
        assert!((*p - q.exp()).abs() < 1e-12);
    }
}

#[test]
fn softmax_works_for_f_order_and_strided_views() {
    let x = Tensor::from_vec(
        vec![0f32, 3., 1., 4., 2., 5.],
        Kahen::new([2, 3]).unwrap(),
        Order::F,
    )
    .unwrap();
    let y = softmax(&x, 0).unwrap();
    assert_eq!(y.shape().dims(), &[2, 3]);
    for column in 0..3 {
        let a = *y.get(&[0, column]).unwrap();
        let b = *y.get(&[1, column]).unwrap();
        assert!((a + b - 1.).abs() < 1e-6);
        assert!(a < b);
    }
}

#[test]
fn softmax_rejects_nonfinite_and_handles_empty() {
    let x = t([2], vec![1f64, f64::NAN]);
    assert_eq!(
        softmax(&x, 0).unwrap_err(),
        OpsError::NonFiniteInput {
            operation: "softmax",
            indices: vec![1],
        }
    );
    let z = t([0, 4], Vec::<f64>::new());
    assert_eq!(softmax(&z, 1).unwrap().shape().dims(), &[0, 4]);
    assert!(softmax(&t([], vec![0f64]), 0).is_err());
}

#[test]
fn welford_variance_and_standard_deviation_population_sample() {
    use kernix_ops::{std_all, variance_all};
    let x = t([4], vec![1f64, 2., 3., 4.]);
    assert!((variance_all(&x, 0).unwrap() - 1.25).abs() < 1e-12);
    assert!((variance_all(&x, 1).unwrap() - (5. / 3.)).abs() < 1e-12);
    assert!((std_all(&x, 0).unwrap() - 1.25f64.sqrt()).abs() < 1e-12);
    assert_eq!(
        variance_all(&x, 4).unwrap_err(),
        OpsError::InvalidDegreesOfFreedom {
            count: 4,
            correction: 4
        }
    );
}

#[test]
fn variance_axes_keepdims_empty_and_noreduction() {
    use kernix_ops::{std_axes, variance_axes};
    let x = t([2, 2], vec![1f64, 3., 2., 4.]);
    let y = variance_axes(&x, &[1], false, 0).unwrap();
    assert_eq!(y.shape().dims(), &[2]);
    assert_eq!(y.storage_slice(), &[1., 1.]);
    let z = std_axes(&x, &[0], true, 0).unwrap();
    assert_eq!(z.shape().dims(), &[1, 2]);
    assert_eq!(z.storage_slice(), &[0.5, 0.5]);
    let no_axes = variance_axes(&x, &[], false, 0).unwrap();
    assert_eq!(no_axes.shape().dims(), &[2, 2]);
    assert_eq!(no_axes.storage_slice(), &[0., 0., 0., 0.]);
    let empty = t([0, 2], Vec::<f64>::new());
    assert_eq!(
        variance_axes(&empty, &[1], false, 1)
            .unwrap()
            .shape()
            .dims(),
        &[0]
    );
    assert!(matches!(
        variance_axes(&empty, &[0], false, 0),
        Err(OpsError::InvalidDegreesOfFreedom {
            count: 0,
            correction: 0
        })
    ));
}

#[test]
fn stable_variance_avoids_naive_cancellation() {
    use kernix_ops::variance_all;
    let x = t(
        [4],
        vec![
            1_000_000_001f64,
            1_000_000_002.,
            1_000_000_003.,
            1_000_000_004.,
        ],
    );
    assert!((variance_all(&x, 0).unwrap() - 1.25).abs() < 1e-12);
}
