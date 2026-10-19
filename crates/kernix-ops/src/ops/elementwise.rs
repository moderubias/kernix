use alloc::vec;
use core::{
    mem::align_of,
    ops::{Add, Div, Mul, Neg, Rem, Sub},
};
use kernix_core::{layout::Order, shape::Kahen, tensor::Tensor};

use super::{
    OpsError, TensorInput,
    broadcast::{broadcast_shape, project_broadcast},
    error::flatten_build_error,
};

/// Unary map over logical elements, returning an owned C-contiguous CPU tensor.
pub fn map<T, U, A, F>(source: &A, mut f: F) -> Result<Tensor<U>, OpsError>
where
    A: TensorInput<T>,
    F: FnMut(&T) -> U,
{
    let shape = Kahen::new(source.dims())?;
    Tensor::<U>::try_from_fn(shape, Order::C, align_of::<U>(), |coords| {
        let value = source.get(coords)?;
        Ok::<U, OpsError>(f(value))
    })
    .map_err(flatten_build_error)
}

/// Binary elementwise map with bidirectional NumPy-style broadcasting.
/// Both source tensors are only borrowed; all output elements are fresh.
pub fn zip_map<L, R, U, A, B, F>(left: &A, right: &B, mut f: F) -> Result<Tensor<U>, OpsError>
where
    A: TensorInput<L>,
    B: TensorInput<R>,
    F: FnMut(&L, &R) -> U,
{
    let dims = broadcast_shape(left.dims(), right.dims())?;
    let shape = Kahen::new(&dims)?;
    let mut li = vec![0; left.dims().len()];
    let mut ri = vec![0; right.dims().len()];
    Tensor::<U>::try_from_fn(shape, Order::C, align_of::<U>(), |coords| {
        project_broadcast(coords, left.dims(), &mut li);
        project_broadcast(coords, right.dims(), &mut ri);
        let a = left.get(&li)?;
        let b = right.get(&ri)?;
        Ok::<U, OpsError>(f(a, b))
    })
    .map_err(flatten_build_error)
}

macro_rules! binary_op {
    ($name:ident, $op:ident, $method:ident) => {
        pub fn $name<T, A, B>(left: &A, right: &B) -> Result<Tensor<T>, OpsError>
        where
            T: Clone + $op<Output = T>,
            A: TensorInput<T>,
            B: TensorInput<T>,
        {
            zip_map(left, right, |a, b| a.clone().$method(b.clone()))
        }
    };
}
binary_op!(add, Add, add);
binary_op!(sub, Sub, sub);
binary_op!(mul, Mul, mul);
binary_op!(div, Div, div);
binary_op!(rem, Rem, rem);

pub fn neg<T, A>(source: &A) -> Result<Tensor<T>, OpsError>
where
    T: Clone + Neg<Output = T>,
    A: TensorInput<T>,
{
    map(source, |value| -value.clone())
}
