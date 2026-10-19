//! Integration regressions for tensor tests.
use core::mem::align_of;

use kernix_core::{
    layout::{Layout, Order},
    shape::{Kahen, Kotei},
    storage::{CpuStorage, StorageBuffer},
    tensor::{Tensor, TensorBuildError, TensorError, TensorView, TensorViewMut},
};

fn shape(dims: impl AsRef<[usize]>) -> Kahen {
    Kahen::new(dims).unwrap()
}

#[test]
fn vec_adoption_is_zero_copy() {
    let values = vec![10u32, 20, 30, 40, 50, 60];
    let pointer = values.as_ptr();
    let tensor = Tensor::from_vec(values, shape([2, 3]), Order::C).unwrap();
    assert_eq!(tensor.storage_slice().as_ptr(), pointer);
    assert_eq!(tensor.shape().dims(), &[2, 3]);
    assert_eq!(tensor.strides(), &[3, 1]);
    assert_eq!(*tensor.get(&[1, 2]).unwrap(), 60);
    assert_eq!(
        tensor.as_contiguous_slice(Order::C).unwrap(),
        &[10, 20, 30, 40, 50, 60]
    );
}

#[test]
fn rejects_wrong_element_count() {
    let error = Tensor::from_vec(vec![1, 2, 3], shape([2, 2]), Order::C).unwrap_err();
    assert_eq!(
        error,
        TensorError::ElementCountMismatch {
            expected: 4,
            actual: 3
        }
    );
}

#[test]
fn fixed_rank_is_preserved_in_tensor_and_view() {
    let tensor = Tensor::from_vec(
        vec![0, 1, 2, 3, 4, 5],
        Kotei::<2>::new([2, 3]).unwrap(),
        Order::C,
    )
    .unwrap();
    let view = tensor.transpose().unwrap();
    assert_eq!(view.shape().dims(), &[3, 2]);
    assert_eq!(*view.get(&[2, 1]).unwrap(), 5);
    assert_eq!(tensor.shape().dims(), &[2, 3]);
}

#[test]
fn f_order_maps_coordinates_correctly() {
    let tensor = Tensor::from_vec(vec![1, 2, 3, 4, 5, 6], shape([2, 3]), Order::F).unwrap();
    assert_eq!(tensor.strides(), &[1, 2]);
    assert_eq!(*tensor.get(&[0, 0]).unwrap(), 1);
    assert_eq!(*tensor.get(&[1, 0]).unwrap(), 2);
    assert_eq!(*tensor.get(&[0, 1]).unwrap(), 3);
    assert_eq!(
        tensor.as_contiguous_slice(Order::F).unwrap(),
        &[1, 2, 3, 4, 5, 6]
    );
    assert!(tensor.as_contiguous_slice(Order::C).is_err());
}

#[test]
fn scalar_uses_one_element_and_empty_indices() {
    let tensor = Tensor::from_vec(vec![42], Kahen::scalar(), Order::C).unwrap();
    assert_eq!(tensor.numel(), 1);
    assert_eq!(tensor.rank(), 0);
    assert_eq!(*tensor.get(&[]).unwrap(), 42);
    assert_eq!(
        tensor.iter(Order::F).unwrap().copied().collect::<Vec<_>>(),
        vec![42]
    );
}

#[test]
fn empty_tensor_has_zero_elements() {
    let tensor = Tensor::<u32>::from_vec(vec![], shape([2, 0, 3]), Order::C).unwrap();
    assert!(tensor.is_empty());
    assert_eq!(tensor.numel(), 0);
    assert!(tensor.iter(Order::C).unwrap().next().is_none());
    assert!(tensor.as_contiguous_slice(Order::C).unwrap().is_empty());
    assert!(tensor.get(&[0, 0, 0]).is_err());
}

#[test]
fn arbitrary_storage_span_is_validated() {
    let storage = CpuStorage::try_from_vec(vec![10u8, 20]).unwrap();
    let layout = Layout::from_strides(shape([2]), [2], 0).unwrap();
    assert!(Tensor::from_host_storage(storage, layout).is_err());
}

#[test]
fn negative_stride_is_zero_copy() {
    let storage = CpuStorage::try_from_vec(vec![10, 20, 30, 40]).unwrap();
    let layout = Layout::from_strides(shape([4]), [-1], 3).unwrap();
    let tensor = Tensor::from_host_storage(storage, layout).unwrap();
    assert_eq!(*tensor.get(&[0]).unwrap(), 40);
    assert_eq!(*tensor.get(&[3]).unwrap(), 10);
    assert_eq!(
        tensor.iter(Order::C).unwrap().copied().collect::<Vec<_>>(),
        vec![40, 30, 20, 10]
    );
    assert!(tensor.as_contiguous_slice(Order::C).is_err());
}

#[test]
fn broadcast_and_zero_stride_views_share_storage() {
    let tensor = Tensor::from_vec(vec![7, 8, 9], shape([1, 3]), Order::C).unwrap();
    let view = tensor.broadcast_to([4, 3]).unwrap();
    assert_eq!(view.shape().dims(), &[4, 3]);
    assert_eq!(view.strides(), &[0, 1]);
    assert_eq!(*view.get(&[3, 2]).unwrap(), 9);
    assert_eq!(
        view.storage_slice().as_ptr(),
        tensor.storage_slice().as_ptr()
    );
}

#[test]
fn overlapping_mutable_views_only_allow_sequential_mut_refs() {
    let mut tensor = Tensor::from_vec(vec![11i32], shape([1]), Order::C).unwrap();
    {
        let mut view = tensor.view_mut().unwrap().broadcast_to([3]).unwrap();
        *view.get_mut(&[0]).unwrap() += 1;
        *view.get_mut(&[2]).unwrap() += 10;
        assert_eq!(*view.get(&[1]).unwrap(), 22);
    }
    assert_eq!(tensor.storage_slice(), &[22]);
}

#[test]
fn independent_mutable_view_from_slice_preserves_aliasing() {
    let mut data = [1, 2, 3];
    let layout = Layout::from_strides(shape([3]), [-1], 2).unwrap();
    let mut view = TensorViewMut::from_slice(&mut data, layout).unwrap();
    *view.get_mut(&[0]).unwrap() = 9;
    assert_eq!(*view.get(&[2]).unwrap(), 1);
    assert_eq!(data, [1, 2, 9]);
}

#[test]
fn permutation_and_transposition_do_not_copy() {
    let tensor = Tensor::from_vec((0..24).collect::<Vec<_>>(), shape([2, 3, 4]), Order::C).unwrap();
    let view = tensor.permute([2, 0, 1]).unwrap();
    assert_eq!(view.shape().dims(), &[4, 2, 3]);
    assert_eq!(*view.get(&[3, 1, 2]).unwrap(), 23);
    assert_eq!(
        view.storage_slice().as_ptr(),
        tensor.storage_slice().as_ptr()
    );
}

#[test]
fn slice_supports_reverse_and_steps() {
    let tensor = Tensor::from_vec(vec![0, 1, 2, 3, 4], shape([5]), Order::C).unwrap();
    let view = tensor.slice_axis(0, 4, 3, -2).unwrap();
    assert_eq!(
        view.iter(Order::C).copied().collect::<Vec<_>>(),
        vec![4, 2, 0]
    );
    assert_eq!(*tensor.get(&[0]).unwrap(), 0);
}

#[test]
fn reshape_view_is_zero_copy_when_possible() {
    let tensor = Tensor::from_vec(vec![0, 1, 2, 3, 4, 5], shape([2, 3]), Order::C).unwrap();
    let view = tensor.reshape_view([3, 2], Order::C).unwrap();
    assert_eq!(view.strides(), &[2, 1]);
    assert_eq!(*view.get(&[2, 1]).unwrap(), 5);
    assert_eq!(
        view.storage_slice().as_ptr(),
        tensor.storage_slice().as_ptr()
    );
}

#[test]
fn reshape_rejects_noncontiguous_request() {
    let tensor = Tensor::from_vec(vec![0, 1, 2, 3, 4, 5], shape([2, 3]), Order::C).unwrap();
    let gapped = tensor.slice_axis(1, 0, 2, 2).unwrap();
    assert!(gapped.reshape_view([4], Order::C).is_err());
}

#[test]
fn contiguous_slice_is_in_logical_order_only_when_proven() {
    let tensor = Tensor::from_vec(vec![0, 1, 2, 3, 4, 5], shape([2, 3]), Order::C).unwrap();
    let tr = tensor.transpose().unwrap();
    assert!(tr.as_contiguous_slice(Order::C).is_err());
    assert_eq!(
        tr.as_contiguous_slice(Order::F).unwrap(),
        &[0, 1, 2, 3, 4, 5]
    );
}

#[test]
fn materialization_from_strided_view_respects_logical_order() {
    let tensor = Tensor::from_vec(vec![1, 2, 3, 4], shape([4]), Order::C).unwrap();
    let reverse = tensor.reverse_axis(0).unwrap();
    let copy = reverse.to_owned_contiguous(Order::C).unwrap();
    assert_eq!(copy.storage_slice(), &[4, 3, 2, 1]);
    assert_ne!(
        copy.storage_slice().as_ptr(),
        tensor.storage_slice().as_ptr()
    );
    assert_eq!(*tensor.get(&[0]).unwrap(), 1);
}

#[test]
fn materialization_from_transpose_in_c_order_is_correct() {
    let tensor = Tensor::from_vec(vec![0, 1, 2, 3, 4, 5], shape([2, 3]), Order::C).unwrap();
    let tr = tensor.transpose().unwrap();
    let copied = tr.to_owned_contiguous(Order::C).unwrap();
    assert_eq!(copied.storage_slice(), &[0, 3, 1, 4, 2, 5]);
}

#[test]
fn mutable_metadata_transforms_preserve_buffer() {
    let mut tensor = Tensor::from_vec(vec![1, 2, 3, 4], shape([2, 2]), Order::C).unwrap();
    let pointer = tensor.storage_slice().as_ptr();
    tensor.transpose_in_place().unwrap();
    assert_eq!(*tensor.get(&[0, 1]).unwrap(), 3);
    *tensor.get_mut(&[1, 0]).unwrap() = 77;
    assert_eq!(tensor.storage_slice().as_ptr(), pointer);
    assert_eq!(tensor.storage_slice(), &[1, 77, 3, 4]);
    tensor.transpose_in_place().unwrap();
    assert_eq!(*tensor.get(&[0, 1]).unwrap(), 77);
}

#[test]
fn invalid_relayout_leaves_existing_layout_intact() {
    let mut tensor = Tensor::from_vec(vec![1, 2], shape([2]), Order::C).unwrap();
    let wrong = Layout::from_strides(shape([2]), [100], 0).unwrap();
    assert!(tensor.set_layout(wrong).is_err());
    assert_eq!(*tensor.get(&[1]).unwrap(), 2);
}

#[test]
fn logical_iterator_is_double_ended_and_exact_size() {
    let tensor = Tensor::from_vec(vec![1, 2, 3, 4], shape([2, 2]), Order::C).unwrap();
    let view = tensor.transpose().unwrap();
    let mut it = view.iter(Order::C);
    assert_eq!(it.len(), 4);
    assert_eq!(it.next(), Some(&1));
    assert_eq!(it.next_back(), Some(&4));
    assert_eq!(it.next(), Some(&3));
    assert_eq!(it.next_back(), Some(&2));
    assert_eq!(it.next(), None);
}

#[test]
fn c_and_f_initializer_receive_correct_logical_coordinates() {
    let c = Tensor::from_fn(shape([2, 3]), Order::C, align_of::<i32>(), |i| {
        i[0] as i32 * 10 + i[1] as i32
    })
    .unwrap();
    let f = Tensor::from_fn(shape([2, 3]), Order::F, align_of::<i32>(), |i| {
        i[0] as i32 * 10 + i[1] as i32
    })
    .unwrap();
    assert_eq!(c.storage_slice(), &[0, 1, 2, 10, 11, 12]);
    assert_eq!(f.storage_slice(), &[0, 10, 1, 11, 2, 12]);
}

#[test]
fn aligned_allocation_is_preserved() {
    let tensor = Tensor::from_vec_aligned(vec![7u64; 8], shape([8]), Order::C, 64).unwrap();
    assert!(tensor.storage().alignment() >= 64);
    assert_eq!((tensor.storage_slice().as_ptr() as usize) % 64, 0);
}

#[test]
fn defaulted_and_filled_constructors() {
    let filled = Tensor::filled(shape([2, 2]), Order::C, align_of::<u8>(), 9u8).unwrap();
    assert_eq!(filled.storage_slice(), &[9, 9, 9, 9]);
    let zeros = Tensor::<u8>::defaulted(shape([2, 2]), Order::C, align_of::<u8>()).unwrap();
    assert_eq!(zeros.storage_slice(), &[0, 0, 0, 0]);
}

#[test]
fn failed_initializer_reports_physical_index() {
    let failure =
        Tensor::<i32>::try_from_fn(shape([2, 3]), Order::F, align_of::<i32>(), |indices| {
            if indices == [0, 1] {
                Err("bad pixel")
            } else {
                Ok(indices[0] as i32)
            }
        })
        .unwrap_err();
    assert!(matches!(
        failure,
        TensorBuildError::Initialize {
            index: 2,
            error: "bad pixel"
        }
    ));
}

#[test]
fn host_binding_rejects_zero_sized_types() {
    let layout = Layout::contiguous(shape([0]), Order::C).unwrap();
    assert!(TensorView::<()>::from_slice(&[], layout).is_err());
}

#[test]
fn non_host_backend_does_not_require_cpu_pointers() {
    struct Remote {
        length: usize,
    }
    impl StorageBuffer<i32> for Remote {
        fn len(&self) -> usize {
            self.length
        }
    }
    let layout = Layout::contiguous(shape([2, 3]), Order::C).unwrap();
    let tensor = Tensor::<i32, _, Remote>::from_buffer(Remote { length: 6 }, layout).unwrap();
    assert_eq!(tensor.storage_len(), 6);
    assert_eq!(tensor.shape().dims(), &[2, 3]);
}

#[test]
fn into_parts_and_storage_vec_have_explicit_physical_semantics() {
    let tensor = Tensor::from_vec(vec![1, 2, 3, 4], shape([2, 2]), Order::C).unwrap();
    let (storage, layout) = tensor.into_parts();
    assert_eq!(layout.strides(), &[2, 1]);
    assert_eq!(storage.into_vec(), vec![1, 2, 3, 4]);
}

#[test]
fn a_view_can_be_created_over_borrowed_memory_without_allocation() {
    let data = [8u32, 9, 10];
    let layout = Layout::contiguous(shape([3]), Order::C).unwrap();
    let view = TensorView::from_slice(&data, layout).unwrap();
    assert_eq!(view.storage_slice().as_ptr(), data.as_ptr());
    assert_eq!(
        view.iter(Order::C).copied().collect::<Vec<_>>(),
        vec![8, 9, 10]
    );
}

#[test]
fn rank_changing_views_do_not_change_original_shape() {
    let tensor = Tensor::from_vec(vec![1, 2], shape([1, 2]), Order::C).unwrap();
    let sq = tensor.squeeze_axis(0).unwrap();
    assert_eq!(sq.shape().dims(), &[2]);
    let expanded = sq.insert_axis(1).unwrap();
    assert_eq!(expanded.shape().dims(), &[2, 1]);
    assert_eq!(*expanded.get(&[1, 0]).unwrap(), 2);
    assert_eq!(tensor.shape().dims(), &[1, 2]);
}

#[test]
fn transposed_mutable_view_can_update_original() {
    let mut tensor = Tensor::from_vec(vec![1, 2, 3, 4, 5, 6], shape([2, 3]), Order::C).unwrap();
    {
        let mut view = tensor.view_mut().unwrap().transpose().unwrap();
        *view.get_mut(&[2, 1]).unwrap() = 99;
        assert_eq!(view.as_read_only().get(&[2, 1]).unwrap(), &99);
    }
    assert_eq!(tensor.storage_slice(), &[1, 2, 3, 4, 5, 99]);
}

#[test]
fn exclusive_slice_exposure_requires_real_contiguity() {
    let mut tensor = Tensor::from_vec(vec![1, 2, 3, 4], shape([2, 2]), Order::C).unwrap();
    let mut view = tensor.view_mut().unwrap().transpose().unwrap();
    assert!(view.as_contiguous_mut_slice(Order::C).is_err());
    view.as_contiguous_mut_slice(Order::F).unwrap()[0] = 9;
    assert_eq!(*view.get(&[0, 0]).unwrap(), 9);
}

#[test]
fn view_mut_into_read_only_keeps_same_buffer() {
    let mut buffer = [3, 4];
    let layout = Layout::contiguous(shape([2]), Order::C).unwrap();
    let mut view = TensorViewMut::from_slice(&mut buffer, layout).unwrap();
    *view.get_mut(&[0]).unwrap() = 15;
    let read = view.into_read_only();
    assert_eq!(*read.get(&[0]).unwrap(), 15);
    assert_eq!(buffer, [15, 4]);
}

#[test]
fn failed_initializer_drops_successful_prefix() {
    use std::{cell::Cell, rc::Rc};
    struct Tracked(Rc<Cell<usize>>);
    impl Drop for Tracked {
        fn drop(&mut self) {
            self.0.set(self.0.get() + 1);
        }
    }
    let dropped = Rc::new(Cell::new(0));
    let result =
        Tensor::<Tracked>::try_from_fn(shape([5]), Order::C, align_of::<Tracked>(), |idx| {
            if idx[0] == 3 {
                Err("stop")
            } else {
                Ok(Tracked(Rc::clone(&dropped)))
            }
        });
    assert!(matches!(
        result,
        Err(TensorBuildError::Initialize { index: 3, .. })
    ));
    assert_eq!(dropped.get(), 3);
}

#[test]
fn logical_indexing_matches_constructed_values_for_small_shapes() {
    for d0 in 0..=3 {
        for d1 in 0..=3 {
            for d2 in 0..=3 {
                for order in [Order::C, Order::F] {
                    let tensor =
                        Tensor::from_fn(shape([d0, d1, d2]), order, align_of::<usize>(), |i| {
                            i[0] * 100 + i[1] * 10 + i[2]
                        })
                        .unwrap();
                    for i in 0..d0 {
                        for j in 0..d1 {
                            for k in 0..d2 {
                                assert_eq!(*tensor.get(&[i, j, k]).unwrap(), i * 100 + j * 10 + k);
                            }
                        }
                    }
                    assert_eq!(tensor.iter(order).unwrap().count(), d0 * d1 * d2);
                }
            }
        }
    }
}
