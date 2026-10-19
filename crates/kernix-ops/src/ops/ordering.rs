//! Deterministic tensor sorting and selection. Float sorting uses IEEE-754
//! totalOrder (`total_cmp`): NaNs and signed zeros have defined positions.
use alloc::{vec, vec::Vec};
use core::{cmp::Ordering, mem::align_of};
use kernix_core::{layout::Order, shape::Kahen, tensor::Tensor};

use super::{
    OpsError, TensorInput, broadcast::next_c, error::flatten_build_error, indexed::normalized_axis,
};

/// Total ordering for tensor elements. Floats sort using `total_cmp`, not
/// partial ordering, including the ordering of NaNs and signed zeros.
pub trait TotalOrder {
    fn total_order(&self, other: &Self) -> Ordering;
}
macro_rules! total_ord_impl {
    ($($ty:ty),+ $(,)?) => {$(
        impl TotalOrder for $ty {
            fn total_order(&self, other: &Self) -> Ordering { self.cmp(other) }
        }
    )+};
}
total_ord_impl!(
    bool, char, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize
);
impl TotalOrder for f32 {
    fn total_order(&self, other: &Self) -> Ordering {
        self.total_cmp(other)
    }
}
impl TotalOrder for f64 {
    fn total_order(&self, other: &Self) -> Ordering {
        self.total_cmp(other)
    }
}

/// Ascending or descending order. Ties are stable: earlier logical indices
/// precede later ones. This includes equivalent floating-point bit patterns.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SortDirection {
    Ascending,
    Descending,
}

/// Return a new tensor with values sorted independently along `axis`.
pub fn sort_axis<T: TotalOrder + Clone, A: TensorInput<T>>(
    source: &A,
    axis: isize,
    direction: SortDirection,
) -> Result<Tensor<T>, OpsError> {
    let dims = source.dims();
    let axis = normalized_axis(dims.len(), axis)?;
    let mut output =
        Tensor::<T>::try_from_fn(Kahen::new(dims)?, Order::C, align_of::<T>(), |coords| {
            Ok::<T, OpsError>(source.get(coords)?.clone())
        })
        .map_err(flatten_build_error)?;
    if output.numel() == 0 {
        return Ok(output);
    }
    let mut outer_dims = dims.to_vec();
    outer_dims.remove(axis);
    let outer_shape = Kahen::new(&outer_dims)?;
    let mut outer = vec![0; outer_dims.len()];
    let mut coords = vec![0; dims.len()];
    for _ in 0..outer_shape.numel() {
        coords[..axis].copy_from_slice(&outer[..axis]);
        coords[axis + 1..].copy_from_slice(&outer[axis..]);
        let mut values = Vec::with_capacity(dims[axis]);
        for index in 0..dims[axis] {
            coords[axis] = index;
            values.push(source.get(&coords)?.clone());
        }
        values.sort_by(|a, b| {
            let ord = a.total_order(b);
            if direction == SortDirection::Descending {
                ord.reverse()
            } else {
                ord
            }
        });
        for (index, value) in values.into_iter().enumerate() {
            coords[axis] = index;
            *output.get_mut(&coords)? = value;
        }
        next_c(&mut outer, &outer_dims);
    }
    Ok(output)
}

/// The original logical indices that would sort each fiber. Equal values
/// retain increasing index order. Returns a fresh C-contiguous tensor.
pub fn argsort_axis<T: TotalOrder, A: TensorInput<T>>(
    source: &A,
    axis: isize,
    direction: SortDirection,
) -> Result<Tensor<usize>, OpsError> {
    let dims = source.dims();
    let axis = normalized_axis(dims.len(), axis)?;
    let mut output = Tensor::filled(Kahen::new(dims)?, Order::C, align_of::<usize>(), 0usize)?;
    if output.numel() == 0 {
        return Ok(output);
    }
    let mut outer_dims = dims.to_vec();
    outer_dims.remove(axis);
    let outer_shape = Kahen::new(&outer_dims)?;
    let mut outer = vec![0; outer_dims.len()];
    let mut coords = vec![0; dims.len()];
    for _ in 0..outer_shape.numel() {
        coords[..axis].copy_from_slice(&outer[..axis]);
        coords[axis + 1..].copy_from_slice(&outer[axis..]);
        let mut order: Vec<(usize, &T)> = Vec::with_capacity(dims[axis]);
        for index in 0..dims[axis] {
            coords[axis] = index;
            order.push((index, source.get(&coords)?));
        }
        order.sort_by(|a, b| {
            let ord = a.1.total_order(b.1);
            if direction == SortDirection::Descending {
                ord.reverse()
            } else {
                ord
            }
        });
        for (index, (old_index, _)) in order.into_iter().enumerate() {
            coords[axis] = index;
            *output.get_mut(&coords)? = old_index;
        }
        next_c(&mut outer, &outer_dims);
    }
    Ok(output)
}

/// Top-k values and their original axis indices in descending order.
/// For equal values, the earliest logical index wins. k=0 is valid.
/// This reference implementation sorts the entire axis; accelerated kernels
/// can replace it with partial selection without changing output semantics.
pub fn topk_axis<T: TotalOrder + Clone, A: TensorInput<T>>(
    source: &A,
    axis: isize,
    k: usize,
) -> Result<(Tensor<T>, Tensor<usize>), OpsError> {
    let dims = source.dims();
    let axis = normalized_axis(dims.len(), axis)?;
    if k > dims[axis] {
        return Err(OpsError::IndexOutOfBounds {
            axis,
            index: k,
            dim: dims[axis],
        });
    }
    let mut out_dims = dims.to_vec();
    out_dims[axis] = k;
    let shape = Kahen::new(out_dims)?;
    if shape.numel() == 0 {
        return Ok((
            Tensor::from_vec(Vec::new(), shape.clone(), Order::C)?,
            Tensor::from_vec(Vec::new(), shape, Order::C)?,
        ));
    }
    let mut outer_dims = dims.to_vec();
    outer_dims.remove(axis);
    let outer_shape = Kahen::new(&outer_dims)?;
    // Materialize each selected fiber once, not once per output element.
    let mut values = Vec::with_capacity(shape.numel());
    let mut indices = Vec::with_capacity(shape.numel());
    // Build in C-order, using a temporary per-output-coordinate selection cache.
    // Precomputing by fibers is easy but would not preserve C-order when axis
    // is not the final axis; instead precompute a flat (source-axis) map.
    let mut selections: Vec<Vec<(usize, T)>> = Vec::with_capacity(outer_shape.numel());
    let mut outer = vec![0; outer_dims.len()];
    let mut coords = vec![0; dims.len()];
    for _ in 0..outer_shape.numel() {
        coords[..axis].copy_from_slice(&outer[..axis]);
        coords[axis + 1..].copy_from_slice(&outer[axis..]);
        let mut fiber = Vec::with_capacity(dims[axis]);
        for i in 0..dims[axis] {
            coords[axis] = i;
            fiber.push((i, source.get(&coords)?.clone()));
        }
        fiber.sort_by(|a, b| b.1.total_order(&a.1));
        fiber.truncate(k);
        selections.push(fiber);
        next_c(&mut outer, &outer_dims);
    }
    let mut out_coords = vec![0; shape.rank()];
    let mut other_coords = vec![0; outer_dims.len()];
    for _ in 0..shape.numel() {
        other_coords[..axis].copy_from_slice(&out_coords[..axis]);
        other_coords[axis..].copy_from_slice(&out_coords[axis + 1..]);
        // Safe flat index: outer_shape has validated numel and coordinates.
        let mut flat = 0usize;
        for (&coord, &dim) in other_coords.iter().zip(&outer_dims) {
            flat = flat * dim + coord;
        }
        let (index, value) = &selections[flat][out_coords[axis]];
        indices.push(*index);
        values.push(value.clone());
        next_c(&mut out_coords, shape.dims());
    }
    Ok((
        Tensor::from_vec(values, shape.clone(), Order::C)?,
        Tensor::from_vec(indices, shape, Order::C)?,
    ))
}
