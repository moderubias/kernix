use alloc::vec;
use core::ops::{AddAssign, DivAssign, MulAssign, SubAssign};
use kernix_core::{
    layout::{LayoutShape, Overlap},
    storage::HostBuffer,
    tensor::{Tensor, TensorError, TensorViewMut},
};

use super::{OpsError, TensorInput, broadcast::next_c};

/// Exclusive writable host tensor contract. Operations require provably
/// nonoverlapping logical elements (not merely a mutable backing slice).
pub trait TensorOutput<T>: TensorInput<T> {
    fn get_mut(&mut self, indices: &[usize]) -> Result<&mut T, TensorError>;
    fn is_non_overlapping(&self) -> bool;
}

impl<T, S: LayoutShape, B: HostBuffer<T>> TensorOutput<T> for Tensor<T, S, B> {
    fn get_mut(&mut self, indices: &[usize]) -> Result<&mut T, TensorError> {
        Tensor::get_mut(self, indices)
    }
    fn is_non_overlapping(&self) -> bool {
        self.layout().overlap() == Overlap::NonOverlapping
    }
}

impl<T, S: LayoutShape> TensorOutput<T> for TensorViewMut<'_, T, S> {
    fn get_mut(&mut self, indices: &[usize]) -> Result<&mut T, TensorError> {
        TensorViewMut::get_mut(self, indices)
    }
    fn is_non_overlapping(&self) -> bool {
        self.layout().overlap() == Overlap::NonOverlapping
    }
}

/// Applies a per-element mutation. RHS may broadcast to the exact LHS shape.
/// On a panic in `update`, already updated elements remain updated.
pub fn zip_mut<T, U, D, S, F>(dest: &mut D, source: &S, mut update: F) -> Result<(), OpsError>
where
    D: TensorOutput<T>,
    S: TensorInput<U>,
    F: FnMut(&mut T, &U),
{
    let shape = kernix_core::shape::Kahen::new(dest.dims())?;
    let out = shape.dims();
    let rhs = source.dims();
    if rhs.len() > out.len() {
        return Err(OpsError::InPlaceBroadcastIncompatible);
    }
    for (axis, (&src_dim, &dst_dim)) in rhs.iter().zip(&out[out.len() - rhs.len()..]).enumerate() {
        if src_dim != dst_dim && src_dim != 1 {
            return Err(OpsError::BroadcastIncompatible {
                axis: out.len() - rhs.len() + axis,
                left: dst_dim,
                right: src_dim,
            });
        }
    }
    if !dest.is_non_overlapping() {
        return Err(OpsError::OverlappingOutput);
    }
    let mut coords = vec![0; out.len()];
    let mut ri = vec![0; rhs.len()];
    for _ in 0..shape.numel() {
        for (i, &dim) in rhs.iter().enumerate() {
            ri[i] = if dim == 1 {
                0
            } else {
                coords[out.len() - rhs.len() + i]
            };
        }
        let right = source.get(&ri)?;
        update(dest.get_mut(&coords)?, right);
        next_c(&mut coords, shape.dims());
    }
    Ok(())
}

pub fn assign<T, D, S>(dest: &mut D, source: &S) -> Result<(), OpsError>
where
    T: Clone,
    D: TensorOutput<T>,
    S: TensorInput<T>,
{
    zip_mut(dest, source, |a, b| *a = b.clone())
}

macro_rules! assign_op {
    ($name:ident, $trait_name:ident, $method:ident) => {
        pub fn $name<T, D, S>(dest: &mut D, source: &S) -> Result<(), OpsError>
        where
            T: Clone + $trait_name<T>,
            D: TensorOutput<T>,
            S: TensorInput<T>,
        {
            zip_mut(dest, source, |a, b| a.$method(b.clone()))
        }
    };
}
assign_op!(add_assign, AddAssign, add_assign);
assign_op!(sub_assign, SubAssign, sub_assign);
assign_op!(mul_assign, MulAssign, mul_assign);
assign_op!(div_assign, DivAssign, div_assign);
