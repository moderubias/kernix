#![no_main]
use kernix::{Kahen, Order, Tensor};
use libfuzzer_sys::fuzz_target;
fuzz_target!(|bytes: &[u8]| {
    let rank = bytes.first().copied().unwrap_or(0) as usize % 5;
    let dims: Vec<usize> = (0..rank)
        .map(|i| match bytes.get(i + 1).copied().unwrap_or(0) {
            255 => usize::MAX,
            x => (x % 9) as usize,
        })
        .collect();
    if let Ok(shape) = Kahen::new(&dims) {
        if dims.contains(&0) {
            assert_eq!(shape.numel(), 0);
        }
        if shape.numel() <= 1024 {
            for order in [Order::C, Order::F] {
                if let Ok(a) =
                    Tensor::<f64>::from_vec(vec![1.; shape.numel()], shape.clone(), order)
                {
                    for axis in 0..rank {
                        let reverse = a.reverse_axis(axis).unwrap();
                        let twice = reverse.reverse_axis(axis).unwrap();
                        assert_eq!(twice.layout(), a.layout());
                        match twice.to_owned_contiguous(Order::C) {
                            Ok(owned) => assert_eq!(owned.storage_slice(), a.storage_slice()),
                            // Empty F layouts need not have representable C strides.
                            Err(_) => assert_eq!(a.numel(), 0),
                        }
                    }
                }
            }
        }
    }
});
