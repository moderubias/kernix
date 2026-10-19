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
    /// Exclusively borrow one element at checked logical coordinates. The borrow
    /// is tied to self, preserving safety even for overlapping layouts. O(rank).
    fn get_mut(&mut self, indices: &[usize]) -> Result<&mut T, TensorError>;
    /// Return true only when distinct logical output indices are proven disjoint.
    /// Unknown overlap must return false; CPU in-place kernels reject such outputs.
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
///
/// # Examples
///
/// ```
/// use kernix_core::{Order, shape::Kahen, tensor::Tensor};
/// use kernix_ops::zip_mut;
/// let mut a = Tensor::from_vec(vec![1.0,2.0,3.0,4.0], Kahen::new([2,2])?, Order::C)?;
/// let b = a.clone();
/// let result = zip_mut(&mut a,&b,|x,y| *x+=*y)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
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

/// Copy source values into a proven nonoverlapping destination.
/// The source must broadcast to the existing destination shape. Reject shape/overlap
/// errors before mutation; a later user-defined access error may leave partial updates.
///
/// # Examples
///
/// ```
/// use kernix_core::{Order, shape::Kahen, tensor::Tensor};
/// use kernix_ops::assign;
/// let mut a = Tensor::from_vec(vec![1.0,2.0,3.0,4.0], Kahen::new([2,2])?, Order::C)?;
/// let b = a.clone();
/// let result = assign(&mut a,&b)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
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
        /// Update a proven nonoverlapping destination using broadcast source values.
        /// Reject incompatible shapes or unproven disjointness before writes; native
        /// scalar arithmetic semantics apply, and later read errors may leave partial updates.
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
