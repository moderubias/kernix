use core::mem::align_of;
use std::{
    cell::Cell,
    panic::{AssertUnwindSafe, catch_unwind},
    rc::Rc,
    sync::Arc,
};

use kernix_core::{
    layout::{Layout, Order},
    shape::{Kahen, Kotei},
    storage::{CpuStorage, StorageBuildError, StorageError},
};

#[test]
fn vec_adoption_preserves_allocation() {
    let values = vec![10u64, 20, 30, 40];
    let original_ptr = values.as_ptr();
    let mut storage = CpuStorage::try_from_vec(values).unwrap();
    assert_eq!(storage.as_slice().as_ptr(), original_ptr);
    storage[2] = 99;
    let values = storage.into_vec();
    assert_eq!(values.as_ptr(), original_ptr);
    assert_eq!(values, [10, 20, 99, 40]);
}

#[test]
fn boxed_slice_is_adopted_and_roundtrips() {
    let values = vec![3u32, 5, 7].into_boxed_slice();
    let p = values.as_ptr();
    let storage = CpuStorage::try_from_box(values).unwrap();
    assert_eq!(storage.as_slice().as_ptr(), p);
    assert_eq!(storage.into_vec(), vec![3, 5, 7]);
}

#[test]
fn aligned_cpu_allocation_guarantees_requested_alignment() {
    for &align in &[align_of::<f32>(), 16, 32, 64, 128, 256] {
        let mut s = CpuStorage::try_from_fn(129, align, |i| i as f32).unwrap();
        assert_eq!(s.alignment(), align);
        assert!(s.is_aligned_to(align));
        assert_eq!(s[100], 100.0);
        s[100] = -8.0;
        assert_eq!(s.as_slice()[100], -8.0);
        let vec = s.into_vec();
        assert_eq!(vec[100], -8.0);
        assert_eq!(vec.len(), 129);
    }
}

#[test]
fn vec_to_aligned_allocation_preserves_noncopy_elements() {
    let input = vec![Box::new(13i32), Box::new(42)];
    let p = input[0].as_ref() as *const i32;
    let s = CpuStorage::try_from_vec_aligned(input, 64).unwrap();
    assert_eq!(s.alignment(), 64);
    assert_eq!(s[0].as_ref() as *const i32, p);
    assert_eq!(*s[1], 42);
}

#[test]
fn vec_to_natural_alignment_stays_zero_copy() {
    let input = vec![1u64, 2, 3];
    let ptr = input.as_ptr();
    let s = CpuStorage::try_from_vec_aligned(input, align_of::<u64>()).unwrap();
    assert_eq!(s.as_slice().as_ptr(), ptr);
}

#[test]
fn initialization_helpers_and_empty_buffers() {
    let s = CpuStorage::<i32>::try_default(4, 64).unwrap();
    assert_eq!(s.as_slice(), &[0; 4]);
    let s = CpuStorage::<i32>::try_filled(3, 64, 7).unwrap();
    assert_eq!(s.as_slice(), &[7, 7, 7]);
    let s = CpuStorage::<i32>::try_from_fn(0, 64, |_| unreachable!()).unwrap();
    assert!(s.is_empty());
    assert!(s.is_aligned_to(4096));
    assert_eq!(s.into_vec(), Vec::<i32>::new());
}

#[test]
fn invalid_alignment_and_extent_rejected_before_init() {
    assert_eq!(
        CpuStorage::<u64>::try_from_fn(2, 3, |_| unreachable!()).unwrap_err(),
        StorageError::InvalidAlignment {
            requested: 3,
            minimum: 8
        },
    );
    assert_eq!(
        CpuStorage::<u64>::try_from_fn(2, 4, |_| unreachable!()).unwrap_err(),
        StorageError::InvalidAlignment {
            requested: 4,
            minimum: 8
        },
    );
    assert!(matches!(
        CpuStorage::<u64>::try_from_fn(usize::MAX, 8, |_| unreachable!()),
        Err(StorageError::CapacityOverflow { .. })
    ));
    assert_eq!(
        CpuStorage::<()>::try_from_vec(vec![()]).unwrap_err(),
        StorageError::ZeroSizedElementType,
    );
}

#[derive(Clone)]
struct Spy(Rc<Cell<usize>>);

impl Drop for Spy {
    fn drop(&mut self) {
        self.0.set(self.0.get() + 1);
    }
}

#[test]
fn initializer_panic_drops_only_written_prefix() {
    let drops = Rc::new(Cell::new(0));
    let result = catch_unwind(AssertUnwindSafe(|| {
        let _ = CpuStorage::try_from_fn(6, 64, |i| {
            if i == 3 {
                panic!("expected failure");
            }
            Spy(Rc::clone(&drops))
        });
    }));
    assert!(result.is_err());
    assert_eq!(drops.get(), 3);
}

#[test]
fn failed_initializer_drops_only_successful_prefix() {
    let drops = Rc::new(Cell::new(0));
    let result = CpuStorage::try_from_fn_fallible(6, 64, |i| {
        if i == 4 {
            Err("stop")
        } else {
            Ok(Spy(Rc::clone(&drops)))
        }
    });
    assert!(matches!(
        result,
        Err(StorageBuildError::Initialize {
            index: 4,
            error: "stop"
        })
    ));
    assert_eq!(drops.get(), 4);
}

#[test]
fn clone_is_deep_and_arc_is_shared_until_make_mut() {
    let original = CpuStorage::try_from_fn(3, 64, |i| i as u64).unwrap();
    let mut independent = original.clone();
    independent[0] = 91;
    assert_eq!(original[0], 0);
    assert_eq!(independent[0], 91);
    assert_eq!(independent.alignment(), 64);

    let shared = original.into_shared();
    let mut other = Arc::clone(&shared);
    assert_eq!(Arc::strong_count(&shared), 2);
    Arc::make_mut(&mut other)[0] = 77;
    assert_eq!(shared[0], 0);
    assert_eq!(other[0], 77);
    assert_eq!(Arc::strong_count(&shared), 1);
}

#[test]
fn layouts_are_bound_by_storage_extent_and_byte_extent() {
    let shape = Kahen::new([2, 3]).unwrap();
    let layout = Layout::contiguous(shape, Order::C).unwrap();
    let mut s = CpuStorage::try_from_vec(vec![0u32, 1, 2, 3, 4, 5]).unwrap();
    {
        let v = s.view(&layout).unwrap();
        assert_eq!(*v.get(&[1, 2]).unwrap(), 5);
        assert!(v.get(&[2, 0]).is_err());
        assert!(v.get(&[0]).is_err());
    }
    {
        let mut v = s.view_mut(&layout).unwrap();
        *v.get_mut(&[1, 1]).unwrap() = 77;
        assert_eq!(*v.get(&[1, 1]).unwrap(), 77);
        assert_eq!(*v.into_read_only().get(&[1, 1]).unwrap(), 77);
    }
    assert_eq!(s[4], 77);
    let too_short = CpuStorage::try_from_vec(vec![0u32; 5]).unwrap();
    assert!(too_short.view(&layout).is_err());
}

#[test]
fn negative_stride_views_do_not_copy() {
    let layout = Layout::from_strides(Kotei::<1>::new([4]).unwrap(), [-1], 3).unwrap();
    let s = CpuStorage::try_from_vec(vec![10u64, 20, 30, 40]).unwrap();
    let v = s.view(&layout).unwrap();
    assert_eq!(*v.get(&[0]).unwrap(), 40);
    assert_eq!(*v.get(&[1]).unwrap(), 30);
    assert_eq!(*v.get(&[3]).unwrap(), 10);
}

#[test]
fn overlapping_views_do_not_yield_multiple_live_mut_references() {
    let layout = Layout::from_strides(Kahen::new([2, 3]).unwrap(), [0, 1], 0).unwrap();
    let mut s = CpuStorage::try_from_vec(vec![1i32, 2, 3]).unwrap();
    let mut v = s.view_mut(&layout).unwrap();
    *v.get_mut(&[0, 1]).unwrap() = 42;
    assert_eq!(*v.get(&[1, 1]).unwrap(), 42);
}

#[test]
fn empty_layout_and_empty_storage_bind_successfully() {
    let layout = Layout::contiguous(Kahen::new([0, 3]).unwrap(), Order::C).unwrap();
    let s = CpuStorage::<f32>::try_from_vec(vec![]).unwrap();
    let v = s.view(&layout).unwrap();
    assert!(v.get(&[0, 0]).is_err());
}

#[test]
fn storage_with_no_host_access_is_a_valid_future_backend_boundary() {
    use kernix_core::storage::{Storage, StorageBuffer};
    struct DeviceMetadata {
        elements: usize,
    }
    impl StorageBuffer<f32> for DeviceMetadata {
        fn len(&self) -> usize {
            self.elements
        }
    }
    let device: Storage<f32, DeviceMetadata> =
        Storage::from_buffer(DeviceMetadata { elements: 32 });
    assert_eq!(device.len(), 32);
    assert!(!device.is_empty());
}

#[test]
fn aligned_empty_buffer_does_not_allocate_or_expose_invalid_pointer() {
    let s = CpuStorage::<u8>::try_from_fn(0, 4096, |_| unreachable!()).unwrap();
    assert_eq!(s.alignment(), 4096);
    assert_eq!(s.as_slice(), &[]);
}

#[test]
fn into_vec_moves_each_element_and_drops_exactly_once() {
    let drops = Rc::new(Cell::new(0));
    let initial = (0..5).map(|_| Spy(Rc::clone(&drops))).collect();
    let aligned = CpuStorage::try_from_vec_aligned(initial, 64).unwrap();
    let values = aligned.into_vec();
    assert_eq!(drops.get(), 0);
    assert_eq!(values.len(), 5);
    drop(values);
    assert_eq!(drops.get(), 5);
}

#[test]
fn storage_mutable_view_preserves_exclusive_borrowing() {
    let l = Layout::from_strides(Kahen::new([2, 2]).unwrap(), [2, 1], 0).unwrap();
    let mut s = CpuStorage::try_from_vec(vec![10, 20, 30, 40]).unwrap();
    {
        let mut view = s.view_mut(&l).unwrap();
        *view.get_mut(&[0, 0]).unwrap() = 100;
        *view.get_mut(&[1, 1]).unwrap() = 400;
    }
    assert_eq!(s.as_slice(), &[100, 20, 30, 400]);
}
