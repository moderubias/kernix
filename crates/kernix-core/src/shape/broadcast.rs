use smallvec::SmallVec;

use super::{INLINE_RANK, ShapeError, utils::validate_dims};

pub(crate) fn can_broadcast_to(source: &[usize], target: &[usize]) -> bool {
    if source.len() > target.len() {
        return false;
    }

    source
        .iter()
        .rev()
        .zip(target.iter().rev())
        .all(|(&source, &target)| source == target || source == 1)
}

pub(crate) fn broadcast_dims(
    left: &[usize],
    right: &[usize],
) -> Result<SmallVec<[usize; INLINE_RANK]>, ShapeError> {
    let rank = left.len().max(right.len());

    let mut left = left.iter().rev();
    let mut right = right.iter().rev();

    let mut dims = SmallVec::<[usize; INLINE_RANK]>::with_capacity(rank);

    for offset in 0..rank {
        let left = left.next().copied().unwrap_or(1);
        let right = right.next().copied().unwrap_or(1);

        let dim = if left == right {
            left
        } else if left == 1 {
            right
        } else if right == 1 {
            left
        } else {
            return Err(ShapeError::BroadcastIncompatible {
                axis: rank - 1 - offset,
                left,
                right,
            });
        };

        dims.push(dim);
    }

    dims.reverse();

    validate_dims(&dims)?;

    Ok(dims)
}
