use kernix_core::layout::{Layout, LayoutError, LayoutShape, Order, Overlap, StorageSpan};
use kernix_core::shape::{Kahen, Kotei};

fn dyn_layout(dims: impl AsRef<[usize]>, order: Order) -> Layout<Kahen> {
    Layout::contiguous(Kahen::new(dims).unwrap(), order).unwrap()
}

#[test]
fn c_and_f_contiguous_strides() {
    let c = dyn_layout([2, 3, 4], Order::C);
    let f = dyn_layout([2, 3, 4], Order::F);
    assert_eq!(c.strides(), &[12, 4, 1]);
    assert_eq!(f.strides(), &[1, 2, 6]);
    assert_eq!(c.origin(), 0);
    assert!(c.is_c_contiguous());
    assert!(!c.is_f_contiguous());
    assert!(f.is_f_contiguous());
    assert!(!f.is_c_contiguous());
    assert_eq!(c.span(), Some(StorageSpan { start: 0, end: 24 }));
    assert_eq!(c.required_storage_len(), 24);
    assert!(c.is_dense());
    assert_eq!(c.overlap(), Overlap::NonOverlapping);
}

#[test]
fn fixed_rank_keeps_array_strides() {
    let fixed = Kotei::<3>::new([2, 3, 4]).unwrap();
    let layout = Layout::contiguous(fixed, Order::C).unwrap();
    assert_eq!(layout.strides(), &[12, 4, 1]);
    let zeroed: [isize; 3] = layout.shape().zeroed_strides();
    assert_eq!(zeroed, [0; 3]);
}

#[test]
fn scalars_and_empty_shapes() {
    let scalar = dyn_layout([], Order::C);
    assert_eq!(scalar.strides(), &[]);
    assert_eq!(scalar.offset_of(&[]), Ok(0));
    assert_eq!(scalar.span(), Some(StorageSpan { start: 0, end: 1 }));
    assert!(scalar.is_contiguous());

    let empty = dyn_layout([2, 0, 4], Order::C);
    assert_eq!(empty.strides(), &[0, 4, 1]);
    assert_eq!(empty.span(), None);
    assert_eq!(empty.required_storage_len(), 0);
    assert!(empty.is_c_contiguous() && empty.is_f_contiguous());
    assert_eq!(
        empty.offset_of(&[0, 0, 0]),
        Err(LayoutError::IndexOutOfBounds {
            axis: 1,
            index: 0,
            dim: 0,
        })
    );
    assert_eq!(dyn_layout([2, 0, 4], Order::F).strides(), &[1, 2, 0]);
    assert_eq!(dyn_layout([0], Order::C).strides(), &[1]);
}

#[test]
fn canonical_stride_overflow_is_not_shape_overflow() {
    let huge = Kahen::new([usize::MAX]).unwrap();
    assert_eq!(Layout::contiguous(huge, Order::C).unwrap().strides(), &[1]);
    let empty = Kahen::new([0, usize::MAX, 2]).unwrap();
    assert!(matches!(
        Layout::contiguous(empty, Order::C),
        Err(LayoutError::StrideOverflow { .. })
    ));
}

#[test]
fn offset_of_checks_logical_bounds() {
    let l = dyn_layout([2, 3, 4], Order::C);
    assert_eq!(l.offset_of(&[1, 2, 3]), Ok(23));
    assert!(matches!(
        l.offset_of(&[1, 2]),
        Err(LayoutError::IndexRankMismatch { .. })
    ));
    assert!(matches!(
        l.offset_of(&[2, 0, 0]),
        Err(LayoutError::IndexOutOfBounds { .. })
    ));
    assert_eq!(
        dyn_layout([2, 3, 4], Order::F).offset_of(&[1, 2, 3]),
        Ok(23)
    );
    assert_eq!(dyn_layout([2, 3, 4], Order::F).offset_of(&[1, 0, 0]), Ok(1));
}

#[test]
fn negative_stride_with_positive_origin() {
    let rev = Layout::from_strides(Kahen::new([4]).unwrap(), [-1], 3).unwrap();
    assert_eq!(rev.span(), Some(StorageSpan { start: 0, end: 4 }));
    assert_eq!(rev.offset_of(&[0]), Ok(3));
    assert_eq!(rev.offset_of(&[3]), Ok(0));
    assert!(!rev.is_c_contiguous());
    assert!(rev.is_dense());
    assert_eq!(rev.overlap(), Overlap::NonOverlapping);
    assert!(matches!(
        Layout::from_strides(Kahen::new([4]).unwrap(), [-1], 2),
        Err(LayoutError::NegativeStorageOffset)
    ));
}

#[test]
fn positive_strides_with_gaps() {
    let l = Layout::from_strides(Kahen::new([2, 2]).unwrap(), [3, 1], 2).unwrap();
    assert_eq!(l.span(), Some(StorageSpan { start: 2, end: 7 }));
    assert_eq!(l.offset_of(&[1, 1]), Ok(6));
    assert_eq!(l.required_storage_len(), 7);
    assert_eq!(l.overlap(), Overlap::NonOverlapping);
    assert!(!l.is_dense());
    assert!(matches!(
        l.validate_storage_len(6),
        Err(LayoutError::StorageTooSmall { .. })
    ));
    assert!(l.validate_storage_len(7).is_ok());
}

#[test]
fn zero_strides_and_overlap() {
    let l = Layout::from_strides(Kahen::new([10, 3]).unwrap(), [0, 1], 0).unwrap();
    assert_eq!(l.span(), Some(StorageSpan { start: 0, end: 3 }));
    assert_eq!(l.offset_of(&[9, 2]), Ok(2));
    assert_eq!(l.overlap(), Overlap::Overlapping);
    assert!(!l.is_dense());
}

#[test]
fn same_rank_permute_keeps_shape_type() {
    let l = Layout::contiguous(Kotei::<3>::new([2, 3, 4]).unwrap(), Order::C).unwrap();
    let p = l.permute([2, 0, 1]).unwrap();
    assert_eq!(p.shape().dims(), &[4, 2, 3]);
    assert_eq!(p.strides(), &[1, 12, 4]);
    assert_eq!(p.offset_of(&[3, 1, 2]), Ok(23));
    assert_eq!(p.overlap(), Overlap::NonOverlapping);
    assert!(p.is_dense());
    assert!(!p.is_f_contiguous());
    assert_eq!(p.permute([1, 2, 0]).unwrap(), l);
    assert!(p.permute([0, 0, 1]).is_err());
}

#[test]
fn swapping_axes_is_involutive() {
    let l = dyn_layout([2, 3, 4], Order::C);
    let swapped = l.swap_axes(0, 2).unwrap();
    assert_eq!(swapped.shape().dims(), &[4, 3, 2]);
    assert_eq!(swapped.strides(), &[1, 4, 12]);
    assert_eq!(swapped.swap_axes(0, 2).unwrap(), l);
}

#[test]
fn reverse_axis_is_involutive() {
    let l = dyn_layout([2, 3], Order::C);
    let rev = l.reverse_axis(1).unwrap();
    assert_eq!(rev.shape().dims(), &[2, 3]);
    assert_eq!(rev.strides(), &[3, -1]);
    assert_eq!(rev.origin(), 2);
    assert_eq!(rev.offset_of(&[1, 0]), Ok(5));
    assert_eq!(rev.offset_of(&[1, 2]), Ok(3));
    assert_eq!(rev.reverse_axis(1).unwrap(), l);
}

#[test]
fn affine_slice_with_step_and_backwards() {
    let l = dyn_layout([10], Order::C);
    let evens = l.slice_axis(0, 1, 4, 2).unwrap();
    assert_eq!(evens.shape().dims(), &[4]);
    assert_eq!(evens.strides(), &[2]);
    assert_eq!(evens.origin(), 1);
    assert_eq!(evens.offset_of(&[3]), Ok(7));
    let backwards = l.slice_axis(0, 9, 5, -2).unwrap();
    assert_eq!(backwards.strides(), &[-2]);
    assert_eq!(backwards.offset_of(&[4]), Ok(1));
    assert!(l.slice_axis(0, 0, 2, 0).is_err());
    assert!(l.slice_axis(0, 0, 3, -1).is_err());
    assert!(l.slice_axis(0, 8, 2, 2).is_err());
}

#[test]
fn slicing_to_empty_is_safe() {
    let l = dyn_layout([10, 2], Order::C);
    let empty = l.slice_axis(0, 10, 0, 1).unwrap();
    assert_eq!(empty.shape().dims(), &[0, 2]);
    assert_eq!(empty.span(), None);
}

#[test]
fn broadcast_to_adds_zero_strides_without_copy() {
    let l = dyn_layout([1, 3], Order::C);
    let b = l.broadcast_to([8, 3]).unwrap();
    assert_eq!(b.strides(), &[0, 1]);
    assert_eq!(b.offset_of(&[7, 2]), Ok(2));
    assert_eq!(b.overlap(), Overlap::Overlapping);
    assert!(l.broadcast_to([2, 4]).is_err());
    let scalar = dyn_layout([], Order::C);
    let b = scalar.broadcast_to([2, 3]).unwrap();
    assert_eq!(b.strides(), &[0, 0]);
    assert_eq!(b.required_storage_len(), 1);
}

#[test]
fn zero_sized_broadcast_is_correct() {
    let l = dyn_layout([1], Order::C);
    let b = l.broadcast_to([0]).unwrap();
    assert_eq!(b.shape().dims(), &[0]);
    assert_eq!(b.strides(), &[0]);
    assert_eq!(b.span(), None);
}

#[test]
fn singleton_insert_and_squeeze() {
    let l = Layout::contiguous(Kotei::<2>::new([2, 3]).unwrap(), Order::C).unwrap();
    let inserted = l.insert_axis(1).unwrap();
    assert_eq!(inserted.shape().dims(), &[2, 1, 3]);
    assert_eq!(inserted.strides(), &[3, 0, 1]);
    let squeezed = inserted.squeeze_axis(1).unwrap();
    assert_eq!(squeezed.shape().dims(), &[2, 3]);
    assert_eq!(squeezed.strides(), &[3, 1]);
    assert!(l.squeeze_axis(0).is_err());
}

#[test]
fn dense_contiguity_ignores_singleton_strides() {
    let l = Layout::from_strides(Kahen::new([2, 1, 4]).unwrap(), [4, 999, 1], 0).unwrap();
    assert!(l.is_c_contiguous());
    assert!(!l.is_f_contiguous());
    assert!(l.is_dense());
}

#[test]
fn reshape_view_is_zero_copy_or_explicit_error() {
    let l = dyn_layout([2, 3, 4], Order::C);
    let r = l.reshape_view([6, 4], Order::C).unwrap();
    assert_eq!(r.strides(), &[4, 1]);
    assert_eq!(r.span(), l.span());
    assert_eq!(r.offset_of(&[5, 3]), Ok(23));
    assert!(l.reshape_view([5, 5], Order::C).is_err());
    let noncontig = l.permute([1, 0, 2]).unwrap();
    assert!(matches!(
        noncontig.reshape_view([6, 4], Order::C),
        Err(LayoutError::NotViewReshapable)
    ));
    assert_eq!(
        noncontig
            .reshape_view([3, 2, 4], Order::C)
            .unwrap()
            .strides(),
        noncontig.strides()
    );
}

#[test]
fn reshape_empty_avoids_large_canonical_stride_requirements() {
    let l = Layout::from_strides(Kahen::new([0, usize::MAX, 2]).unwrap(), [0, 0, 0], 0).unwrap();
    let reshaped = l.reshape_view([usize::MAX, 0], Order::C).unwrap();
    assert_eq!(reshaped.strides(), &[0, 0]);
    assert_eq!(reshaped.span(), None);
}

#[test]
fn storage_binding_rejects_oversized_byte_extents() {
    let l = dyn_layout([1], Order::C);
    assert!(l.validate_cpu_storage::<u64>(1).is_ok());
    assert!(l.validate_cpu_storage::<()>(1).is_err());
    assert!(matches!(
        l.validate_cpu_storage::<u64>(usize::MAX),
        Err(LayoutError::StorageByteExtentOverflow)
    ));
}

#[test]
fn unusual_strides_never_authorize_unproven_nonoverlap() {
    let l = Layout::from_strides(Kahen::new([2, 2]).unwrap(), [2, 2], 0).unwrap();
    assert_eq!(l.overlap(), Overlap::Overlapping);
    let l = Layout::from_strides(Kahen::new([2, 2]).unwrap(), [2, 3], 0).unwrap();
    assert_eq!(l.overlap(), Overlap::NonOverlapping);
}

#[test]
fn reshape_noncontiguous_chunks_and_reversed_chunks() {
    // Two rows separated by a one-element gap: flattening must fail, but
    // splitting its contiguous inner row is valid without copying.
    let padded = Layout::from_strides(Kahen::new([2, 6]).unwrap(), [8, 1], 0).unwrap();
    let res = padded.reshape_view([2, 2, 3], Order::C).unwrap();
    assert_eq!(res.strides(), &[8, 3, 1]);
    assert_eq!(res.offset_of(&[1, 1, 2]), Ok(13));
    assert!(matches!(
        padded.reshape_view([12], Order::C),
        Err(LayoutError::NotViewReshapable)
    ));

    let reversed = dyn_layout([2, 3], Order::C)
        .reverse_axis(0)
        .unwrap()
        .reverse_axis(1)
        .unwrap();
    assert_eq!(reversed.strides(), &[-3, -1]);
    assert_eq!(reversed.origin(), 5);
    let flat = reversed.reshape_view([6], Order::C).unwrap();
    assert_eq!(flat.strides(), &[-1]);
    assert_eq!(flat.offset_of(&[0]), Ok(5));
    assert_eq!(flat.offset_of(&[5]), Ok(0));
}

#[test]
fn two_dimensional_transpose() {
    let l = dyn_layout([2, 3], Order::C);
    let t = l.transpose().unwrap();
    assert_eq!(t.shape().dims(), &[3, 2]);
    assert_eq!(t.strides(), &[1, 3]);
    assert!(t.transpose().unwrap() == l);
    assert!(dyn_layout([3], Order::C).transpose().is_err());
}

use proptest::prelude::*;
proptest! {
    #[test]
    fn c_order_index_matches_row_major_formula(
        a in 1usize..10, b in 1usize..10, c in 1usize..10,
        i in 0usize..100, j in 0usize..100, k in 0usize..100,
    ) {
        let l = dyn_layout([a,b,c], Order::C);
        let i = i % a;
        let j = j % b;
        let k = k % c;
        prop_assert_eq!(l.offset_of(&[i,j,k]), Ok(i*b*c + j*c + k));
        prop_assert_eq!(l.reverse_axis(2).unwrap().reverse_axis(2).unwrap(), l);
    }

    #[test]
    fn permutation_and_inverse_preserve_mapping(
        a in 1usize..8, b in 1usize..8, c in 1usize..8,
    ) {
        let l = dyn_layout([a,b,c], Order::C);
        let permuted = l.permute([2,0,1]).unwrap();
        prop_assert_eq!(permuted.permute([1,2,0]).unwrap(), l);
    }
}
