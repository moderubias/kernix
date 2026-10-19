use smallvec::SmallVec;

use super::{LayoutError, Order};

/// Half-open range `[start, end)` of potentially addressed storage elements.
/// An empty tensor has no span (`None`), not a zero-width range.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StorageSpan {
    pub start: usize,
    pub end: usize,
}

impl StorageSpan {
    pub fn len(self) -> usize {
        self.end - self.start
    }

    pub fn is_empty(self) -> bool {
        self.start == self.end
    }
}

/// Conservative aliasing classification. `Unknown` is never permission for
/// creating independent mutable references.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Overlap {
    NonOverlapping,
    Overlapping,
    Unknown,
}

pub(super) fn checked_span(
    dims: &[usize],
    strides: &[isize],
    origin: usize,
) -> Result<Option<StorageSpan>, LayoutError> {
    if dims.len() != strides.len() {
        return Err(LayoutError::StrideRankMismatch {
            expected: dims.len(),
            actual: strides.len(),
        });
    }
    if dims.contains(&0) {
        return Ok(None);
    }

    let mut lo = i128::try_from(origin).map_err(|_| LayoutError::AddressOverflow)?;
    let mut hi = lo;
    for (&dim, &stride) in dims.iter().zip(strides) {
        let count = i128::try_from(dim - 1).map_err(|_| LayoutError::AddressOverflow)?;
        let contribution = count
            .checked_mul(stride as i128)
            .ok_or(LayoutError::AddressOverflow)?;
        if contribution < 0 {
            lo = lo
                .checked_add(contribution)
                .ok_or(LayoutError::AddressOverflow)?;
        } else {
            hi = hi
                .checked_add(contribution)
                .ok_or(LayoutError::AddressOverflow)?;
        }
    }
    if lo < 0 {
        return Err(LayoutError::NegativeStorageOffset);
    }

    let start = usize::try_from(lo).map_err(|_| LayoutError::AddressOverflow)?;
    let end_exclusive = hi.checked_add(1).ok_or(LayoutError::AddressOverflow)?;
    let end = usize::try_from(end_exclusive).map_err(|_| LayoutError::AddressOverflow)?;
    Ok(Some(StorageSpan { start, end }))
}

pub(super) fn contiguous_strides(
    dims: &[usize],
    out: &mut [isize],
    order: Order,
) -> Result<(), LayoutError> {
    if dims.len() != out.len() {
        return Err(LayoutError::StrideRankMismatch {
            expected: dims.len(),
            actual: out.len(),
        });
    }
    let rank = dims.len();
    let mut acc = 1usize;
    for step in 0..rank {
        let axis = match order {
            Order::C => rank - 1 - step,
            Order::F => step,
        };
        out[axis] = isize::try_from(acc).map_err(|_| LayoutError::StrideOverflow { axis })?;
        if step + 1 < rank {
            let next_axis = match order {
                Order::C => axis - 1,
                Order::F => axis + 1,
            };
            acc = acc
                .checked_mul(dims[axis])
                .ok_or(LayoutError::StrideOverflow { axis: next_axis })?;
        }
    }
    Ok(())
}

pub(super) fn is_contiguous(dims: &[usize], strides: &[isize], order: Order) -> bool {
    if dims.len() != strides.len() {
        return false;
    }
    if dims.contains(&0) {
        return true;
    }
    let mut expected = 1u128;
    for step in 0..dims.len() {
        let axis = match order {
            Order::C => dims.len() - 1 - step,
            Order::F => step,
        };
        if dims[axis] <= 1 {
            continue;
        }
        if strides[axis] <= 0 || strides[axis] as u128 != expected {
            return false;
        }
        match expected.checked_mul(dims[axis] as u128) {
            Some(next) => expected = next,
            None => return false,
        }
    }
    true
}

pub(super) fn overlap(
    dims: &[usize],
    strides: &[isize],
    span: Option<StorageSpan>,
    numel: usize,
) -> Overlap {
    if numel <= 1 {
        return Overlap::NonOverlapping;
    }
    let Some(span) = span else {
        return Overlap::NonOverlapping;
    };
    if span.len() < numel {
        return Overlap::Overlapping;
    }
    let mut active: SmallVec<[(u128, usize); 6]> = dims
        .iter()
        .copied()
        .zip(strides.iter().copied())
        .filter_map(|(d, s)| (d > 1).then_some((s.unsigned_abs() as u128, d)))
        .collect();
    if active.iter().any(|(s, _)| *s == 0) {
        return Overlap::Overlapping;
    }
    active.sort_unstable_by_key(|(s, _)| *s);
    let mut covered = 1u128;
    let mut previous: Option<u128> = None;
    for (stride, dim) in active {
        if previous == Some(stride) {
            return Overlap::Overlapping;
        }
        if stride < covered {
            return Overlap::Unknown;
        }
        covered += (dim as u128 - 1) * stride;
        previous = Some(stride);
    }
    Overlap::NonOverlapping
}

/// Attempt a no-copy reshape in the requested logical traversal order.
/// Handles padded axes and reversed chunks when factorization permits; never
/// fabricates a contiguous view over discontinuous storage.
pub(super) fn reshape_strides(
    old_dims: &[usize],
    old_strides: &[isize],
    new_dims: &[usize],
    order: Order,
) -> Result<Option<SmallVec<[isize; 6]>>, LayoutError> {
    let mut result: SmallVec<[isize; 6]> = core::iter::repeat_n(0, new_dims.len()).collect();

    // The caller handles zero/one-element tensors: all strides are arbitrary
    // because no pair of distinct logical indices exists.
    let old_axes: SmallVec<[usize; 6]> = match order {
        Order::C => (0..old_dims.len())
            .rev()
            .filter(|&i| old_dims[i] > 1)
            .collect(),
        Order::F => (0..old_dims.len()).filter(|&i| old_dims[i] > 1).collect(),
    };
    let new_axes: SmallVec<[usize; 6]> = match order {
        Order::C => (0..new_dims.len())
            .rev()
            .filter(|&i| new_dims[i] > 1)
            .collect(),
        Order::F => (0..new_dims.len()).filter(|&i| new_dims[i] > 1).collect(),
    };

    // Chunks group consecutive axes whose strides are consecutive in the
    // selected traversal order. (chunk logical extent, fastest stride)
    let mut chunks: SmallVec<[(usize, isize); 6]> = SmallVec::new();
    let mut previous: Option<(usize, isize)> = None;
    for axis in old_axes {
        let dim = old_dims[axis];
        let stride = old_strides[axis];
        let joins = previous.is_some_and(|(fast_dim, fast_stride)| {
            (stride as i128) == (fast_stride as i128) * (fast_dim as i128)
        });
        if joins {
            let chunk = chunks.last_mut().ok_or(LayoutError::AddressOverflow)?;
            chunk.0 = chunk
                .0
                .checked_mul(dim)
                .ok_or(LayoutError::AddressOverflow)?;
        } else {
            chunks.push((dim, stride));
        }
        previous = Some((dim, stride));
    }

    let mut next_new = 0usize;
    for (chunk_extent, chunk_stride) in chunks {
        let mut consumed = 1usize;
        while consumed < chunk_extent {
            let Some(&axis) = new_axes.get(next_new) else {
                return Ok(None);
            };
            let dim = new_dims[axis];
            let Some(next_consumed) = consumed.checked_mul(dim) else {
                return Ok(None);
            };
            if next_consumed > chunk_extent || chunk_extent % next_consumed != 0 {
                return Ok(None);
            }
            let stride = (chunk_stride as i128)
                .checked_mul(consumed as i128)
                .ok_or(LayoutError::AddressOverflow)?;
            let Ok(stride) = isize::try_from(stride) else {
                return Ok(None);
            };
            result[axis] = stride;
            consumed = next_consumed;
            next_new += 1;
        }
    }
    if next_new != new_axes.len() {
        return Ok(None);
    }
    Ok(Some(result))
}
