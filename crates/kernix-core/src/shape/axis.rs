use super::ShapeError;

pub(crate) fn normalize_axis(rank: usize, axis: isize) -> Result<usize, ShapeError> {
    let normalized = if axis >= 0 {
        axis as usize
    } else {
        rank.checked_add_signed(axis)
            .ok_or(ShapeError::AxisOutOfBounds { axis, rank })?
    };

    if normalized >= rank {
        return Err(ShapeError::AxisOutOfBounds { axis, rank });
    }

    Ok(normalized)
}
