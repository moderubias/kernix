use core::ops::Range;

use super::TensorError;
use crate::layout::{Layout, LayoutShape, Order};

/// Convert an order-specific flat storage index into logical coordinates.
/// Called only for in-range indices of a nonempty canonical tensor.
pub(super) fn unravel(mut linear: usize, dims: &[usize], order: Order, out: &mut [usize]) {
    debug_assert_eq!(dims.len(), out.len());
    let rank = dims.len();
    for step in 0..rank {
        let axis = match order {
            Order::C => rank - 1 - step,
            Order::F => step,
        };
        let dim = dims[axis];
        debug_assert_ne!(dim, 0);
        out[axis] = linear % dim;
        linear /= dim;
    }
}

/// A range is exposed only when logical iteration really corresponds to a
/// single, positive, unit-step physical interval in the requested order.
pub(super) fn contiguous_range<S: LayoutShape>(
    layout: &Layout<S>,
    order: Order,
) -> Result<Range<usize>, TensorError> {
    let is_contiguous = match order {
        Order::C => layout.is_c_contiguous(),
        Order::F => layout.is_f_contiguous(),
    };
    if !is_contiguous {
        return Err(TensorError::NotContiguous { order });
    }
    if layout.is_empty() {
        return Ok(0..0);
    }
    let span = layout.span().ok_or(TensorError::NotContiguous { order })?;
    if span.len() != layout.numel() {
        return Err(TensorError::NotContiguous { order });
    }
    Ok(span.start..span.end)
}
