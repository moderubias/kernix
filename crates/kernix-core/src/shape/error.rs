use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Error)]
#[non_exhaustive]
pub enum ShapeError {
    #[error(
        "shape element count overflow at axis {axis}: \
        {accumulated} x {dim} exceeds usize::MAX"
    )]
    ElementCountOverflow {
        axis: usize,
        accumulated: usize,
        dim: usize,
    },

    #[error(
        "shape rank mismatch: expected rank {expected}, \
        instead got rank {actual}"
    )]
    RankMismatch { expected: usize, actual: usize },

    #[error("axis {axis} out of bounds for shape of rank {rank}")]
    AxisOutOfBounds { axis: isize, rank: usize },

    #[error("axis position {axis} out of bounds for shape of rank {rank}")]
    AxisPositionOutOfBounds { axis: usize, rank: usize },

    #[error("expected dimension 1 at axis {axis}, got {dim}")]
    AxisNotSingleton { axis: usize, dim: usize },

    #[error("duplicate axis {axis} in permutation")]
    DuplicateAxis { axis: usize },

    #[error(
        "cannot broadcast dimensions {left} and {right} \
        at aligned axis {axis}"
    )]
    BroadcastIncompatible {
        axis: usize,
        left: usize,
        right: usize,
    },
}
