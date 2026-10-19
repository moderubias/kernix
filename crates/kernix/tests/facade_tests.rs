use kernix::{Kahen, Layout, Order, Tensor, layout, shape, tensor};

#[test]
fn main_facade_exposes_core_tensor_stack() {
    let shape = Kahen::new([2, 2]).unwrap();
    let layout = Layout::contiguous(shape.clone(), Order::C).unwrap();
    assert_eq!(layout.strides(), &[2, 1]);
    let tensor = Tensor::from_vec(vec![1_i32, 2, 3, 4], shape, Order::C).unwrap();
    assert_eq!(*tensor.get(&[1, 0]).unwrap(), 3);
}

#[test]
fn existing_module_paths_remain_available() {
    let _: shape::Kahen = Kahen::scalar();
    let _: layout::Order = Order::F;
    let _ = core::mem::size_of::<tensor::Tensor<i32>>();
}
