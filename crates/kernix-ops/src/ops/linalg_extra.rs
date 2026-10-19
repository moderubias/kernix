// Kernix numerics-softmax overlay 04 (applied by tools/kernix-update-04/apply.py)
//! General contraction and small linear-algebra reference kernels.
use alloc::{vec, vec::Vec};
use core::{
    mem::align_of,
    ops::{AddAssign, Mul},
};
use kernix_core::{layout::Order, shape::Kahen, tensor::Tensor};

use super::{
    OpsError, TensorInput, Zero, broadcast::next_c, error::flatten_build_error,
    indexed::normalized_axis,
};

/// Vector dot product, returning a scalar. Empty vectors yield additive zero.
///
/// # Examples
///
/// ```
/// use kernix_core::{Order, shape::Kahen, tensor::Tensor};
/// use kernix_ops::dot;
/// let mut a = Tensor::from_vec(vec![1.0,2.0,3.0,4.0], Kahen::new([2,2])?, Order::C)?;
/// let result = dot(&a.reshape_view([4],Order::C)?,&a.reshape_view([4],Order::C)?)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn dot<T, A, B>(left: &A, right: &B) -> Result<T, OpsError>
where
    T: Zero + Clone + AddAssign<T> + Mul<Output = T>,
    A: TensorInput<T>,
    B: TensorInput<T>,
{
    if left.dims().len() != 1 {
        return Err(OpsError::ExactRank {
            operation: "dot",
            expected: 1,
            actual: left.dims().len(),
        });
    }
    if right.dims().len() != 1 {
        return Err(OpsError::ExactRank {
            operation: "dot",
            expected: 1,
            actual: right.dims().len(),
        });
    }
    if left.dims()[0] != right.dims()[0] {
        return Err(OpsError::ContractDimensionMismatch {
            left: left.dims()[0],
            right: right.dims()[0],
        });
    }
    // Zero-allocation direct-address path for contiguous vector inputs.
    if let (Some(a), Some(b)) = (left.as_c_contiguous_slice(), right.as_c_contiguous_slice()) {
        if a.len() == left.dims()[0] && b.len() == right.dims()[0] {
            let mut total = T::zero();
            for (lhs, rhs) in a.iter().zip(b) {
                total += lhs.clone() * rhs.clone();
            }
            return Ok(total);
        }
    }
    let mut total = T::zero();
    for i in 0..left.dims()[0] {
        total += left.get(&[i])?.clone() * right.get(&[i])?.clone();
    }
    Ok(total)
}

/// Outer product of two vectors. Result shape `[m, n]`.
///
/// # Examples
///
/// ```
/// use kernix_core::{Order, shape::Kahen, tensor::Tensor};
/// use kernix_ops::outer;
/// let mut a = Tensor::from_vec(vec![1.0,2.0,3.0,4.0], Kahen::new([2,2])?, Order::C)?;
/// let result = outer(&a.reshape_view([4],Order::C)?,&a.reshape_view([4],Order::C)?)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn outer<T, A, B>(left: &A, right: &B) -> Result<Tensor<T>, OpsError>
where
    T: Clone + Mul<Output = T>,
    A: TensorInput<T>,
    B: TensorInput<T>,
{
    if left.dims().len() != 1 {
        return Err(OpsError::ExactRank {
            operation: "outer",
            expected: 1,
            actual: left.dims().len(),
        });
    }
    if right.dims().len() != 1 {
        return Err(OpsError::ExactRank {
            operation: "outer",
            expected: 1,
            actual: right.dims().len(),
        });
    }
    Tensor::<T>::try_from_fn(
        Kahen::new([left.dims()[0], right.dims()[0]])?,
        Order::C,
        align_of::<T>(),
        |coords| {
            Ok::<T, OpsError>(left.get(&[coords[0]])?.clone() * right.get(&[coords[1]])?.clone())
        },
    )
    .map_err(flatten_build_error)
}

/// Trace of a rank-2 tensor. `offset > 0` chooses superdiagonal; `offset < 0`
/// chooses subdiagonal. Diagonals outside the matrix yield additive zero.
///
/// # Examples
///
/// ```
/// use kernix_core::{Order, shape::Kahen, tensor::Tensor};
/// use kernix_ops::trace;
/// let mut a = Tensor::from_vec(vec![1.0,2.0,3.0,4.0], Kahen::new([2,2])?, Order::C)?;
/// let result = trace(&a,0)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn trace<T, A>(source: &A, offset: isize) -> Result<T, OpsError>
where
    T: Zero + Clone + AddAssign<T>,
    A: TensorInput<T>,
{
    let dims = source.dims();
    if dims.len() != 2 {
        return Err(OpsError::ExactRank {
            operation: "trace",
            expected: 2,
            actual: dims.len(),
        });
    }
    let by = offset.unsigned_abs();
    let (row, col) = if offset < 0 { (by, 0) } else { (0, by) };
    let count = if row >= dims[0] || col >= dims[1] {
        0
    } else {
        (dims[0] - row).min(dims[1] - col)
    };
    let mut sum = T::zero();
    for i in 0..count {
        sum += source.get(&[row + i, col + i])?.clone();
    }
    Ok(sum)
}

/// Arbitrary-axis tensor contraction, the semantic basis for a future einsum
/// planner. Result axes are the uncontracted left axes followed by the
/// uncontracted right axes; the contracted dimensions must match pairwise.
/// Duplicate contraction axes are errors, and an empty contract means outer.
///
/// # Examples
///
/// ```
/// use kernix_core::{Order, shape::Kahen, tensor::Tensor};
/// use kernix_ops::tensordot;
/// let mut a = Tensor::from_vec(vec![1.0,2.0,3.0,4.0], Kahen::new([2,2])?, Order::C)?;
/// let result = tensordot(&a,&a,&[1],&[0])?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn tensordot<T, A, B>(
    left: &A,
    right: &B,
    left_axes: &[isize],
    right_axes: &[isize],
) -> Result<Tensor<T>, OpsError>
where
    T: Zero + Clone + AddAssign<T> + Mul<Output = T>,
    A: TensorInput<T>,
    B: TensorInput<T>,
{
    let la = left.dims();
    let rb = right.dims();
    if left_axes.len() != right_axes.len() {
        return Err(OpsError::ContractAxesMismatch {
            left: left_axes.len(),
            right: right_axes.len(),
        });
    }
    let mut contracted_left = vec![false; la.len()];
    let mut contracted_right = vec![false; rb.len()];
    let mut axes_l = Vec::with_capacity(left_axes.len());
    let mut axes_r = Vec::with_capacity(right_axes.len());
    let mut contract_dims = Vec::with_capacity(left_axes.len());
    for (&l, &r) in left_axes.iter().zip(right_axes) {
        let a = normalized_axis(la.len(), l)?;
        let b = normalized_axis(rb.len(), r)?;
        if contracted_left[a] || contracted_right[b] {
            return Err(OpsError::DuplicateAxis {
                axis: if contracted_left[a] { a } else { b },
            });
        }
        if la[a] != rb[b] {
            return Err(OpsError::ContractDimensionMismatch {
                left: la[a],
                right: rb[b],
            });
        }
        contracted_left[a] = true;
        contracted_right[b] = true;
        axes_l.push(a);
        axes_r.push(b);
        contract_dims.push(la[a]);
    }
    let left_free: Vec<usize> = (0..la.len()).filter(|&i| !contracted_left[i]).collect();
    let right_free: Vec<usize> = (0..rb.len()).filter(|&i| !contracted_right[i]).collect();
    let mut out_dims = Vec::with_capacity(left_free.len() + right_free.len());
    out_dims.extend(left_free.iter().map(|&i| la[i]));
    out_dims.extend(right_free.iter().map(|&i| rb[i]));
    let result_shape = Kahen::new(out_dims)?;
    if result_shape.numel() == 0 {
        return Ok(Tensor::from_vec(Vec::new(), result_shape, Order::C)?);
    }
    let contraction = Kahen::new(&contract_dims)?;
    let mut ai = vec![0; la.len()];
    let mut bi = vec![0; rb.len()];
    let mut inner = vec![0; contract_dims.len()];
    Tensor::<T>::try_from_fn(result_shape, Order::C, align_of::<T>(), |coords| {
        for (j, &axis) in left_free.iter().enumerate() {
            ai[axis] = coords[j];
        }
        for (j, &axis) in right_free.iter().enumerate() {
            bi[axis] = coords[left_free.len() + j];
        }
        inner.fill(0);
        let mut sum = T::zero();
        for _ in 0..contraction.numel() {
            for (j, &axis) in axes_l.iter().enumerate() {
                ai[axis] = inner[j];
            }
            for (j, &axis) in axes_r.iter().enumerate() {
                bi[axis] = inner[j];
            }
            sum += left.get(&ai)?.clone() * right.get(&bi)?.clone();
            next_c(&mut inner, &contract_dims);
        }
        Ok::<T, OpsError>(sum)
    })
    .map_err(flatten_build_error)
}
