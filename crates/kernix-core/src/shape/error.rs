use thiserror::Error;

/// A shape invariant, axis, permutation or broadcasting validation failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Error)]
#[non_exhaustive]
pub enum ShapeError {
    #[error(
        "shape element count overflow at axis {axis}: \
        {accumulated} x {dim} exceeds usize::MAX"
    )]
    /// The nonzero dimension product cannot be represented as usize.
    ElementCountOverflow {
        #[doc = "Zero-based axis or insertion position associated with this error."]
        axis: usize,
        #[doc = "Product of preceding dimensions before the failing multiplication."]
        accumulated: usize,
        #[doc = "Dimension size at the reported axis."]
        dim: usize,
    },

    #[error(
        "shape rank mismatch: expected rank {expected}, \
        instead got rank {actual}"
    )]
    /// The provided rank differs from the required rank.
    RankMismatch {
        #[doc = "Required rank, length or extent."]
        expected: usize,
        #[doc = "Observed rank, length or extent."]
        actual: usize,
    },

    /// The axis is outside the valid range for this rank.
    #[error("axis {axis} out of bounds for shape of rank {rank}")]
    AxisOutOfBounds {
        #[doc = "Zero-based axis or insertion position associated with this error."]
        axis: isize,
        #[doc = "Number of axes in the relevant shape."]
        rank: usize,
    },

    /// The unsigned axis position is invalid for this operation.
    #[error("axis position {axis} out of bounds for shape of rank {rank}")]
    AxisPositionOutOfBounds {
        #[doc = "Zero-based axis or insertion position associated with this error."]
        axis: usize,
        #[doc = "Number of axes in the relevant shape."]
        rank: usize,
    },

    /// The requested squeeze axis has dimension other than one.
    #[error("expected dimension 1 at axis {axis}, got {dim}")]
    AxisNotSingleton {
        #[doc = "Zero-based axis or insertion position associated with this error."]
        axis: usize,
        #[doc = "Dimension size at the reported axis."]
        dim: usize,
    },

    /// An axis occurs more than once in a permutation or reduction.
    #[error("duplicate axis {axis} in permutation")]
    DuplicateAxis {
        #[doc = "Zero-based axis or insertion position associated with this error."]
        axis: usize,
    },

    #[error(
        "cannot broadcast dimensions {left} and {right} \
        at aligned axis {axis}"
    )]
    /// The aligned dimensions cannot broadcast under trailing-axis singleton rules.
    BroadcastIncompatible {
        #[doc = "Zero-based axis or insertion position associated with this error."]
        axis: usize,
        #[doc = "Left operand dimension or axis count."]
        left: usize,
        #[doc = "Right operand dimension or axis count."]
        right: usize,
    },
}
