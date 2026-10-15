use super::ShapeError;

pub(crate) fn validate_dims(dims: &[usize]) -> Result<(), ShapeError> {
    if dims.contains(&0) {
        return Ok(());
    }

    let mut accumulated = 1usize;

    for (axis, &dim) in dims.iter().enumerate() {
        accumulated = accumulated
            .checked_mul(dim)
            .ok_or(ShapeError::ElementCountOverflow {
                axis,
                accumulated,
                dim,
            })?;
    }

    Ok(())
}

pub(crate) fn numel(dims: &[usize]) -> usize {
    if dims.contains(&0) {
        0
    } else {
        dims.iter().product()
    }
}
