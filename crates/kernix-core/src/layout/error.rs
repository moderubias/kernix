use crate::shape::ShapeError;
use thiserror::Error;

/// Errors in validating or transforming an affine layout.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[non_exhaustive]
pub enum LayoutError {
    #[error(transparent)]
    Shape(#[from] ShapeError),

    #[error("expected {expected} strides, got {actual}")]
    StrideRankMismatch { expected: usize, actual: usize },

    #[error("canonical stride at axis {axis} cannot be represented as isize")]
    StrideOverflow { axis: usize },

    #[error("layout address arithmetic overflow or address outside usize storage range")]
    AddressOverflow,

    #[error("layout reaches a negative storage element index")]
    NegativeStorageOffset,

    #[error("layout requires {required} storage elements but storage has {available}")]
    StorageTooSmall { required: usize, available: usize },

    #[error("axis {axis} out of bounds for rank {rank}")]
    AxisOutOfBounds { axis: usize, rank: usize },

    #[error("index rank mismatch: expected {expected}, got {actual}")]
    IndexRankMismatch { expected: usize, actual: usize },

    #[error("index {index} out of bounds for dimension {dim} at axis {axis}")]
    IndexOutOfBounds {
        axis: usize,
        index: usize,
        dim: usize,
    },

    #[error("invalid slice at axis {axis}: start={start}, len={len}, step={step}, dim={dim}")]
    InvalidSlice {
        axis: usize,
        start: usize,
        len: usize,
        step: isize,
        dim: usize,
    },

    #[error("slice step cannot be zero")]
    ZeroSliceStep,

    #[error("broadcast cannot expand dimension {source_dim} to {target} at axis {axis}")]
    BroadcastIncompatible {
        axis: usize,
        source_dim: usize,
        target: usize,
    },

    #[error("reshape element count mismatch: old {old}, new {new}")]
    ReshapeElementCountMismatch { old: usize, new: usize },

    #[error("requested reshape cannot be represented as a view in the selected order")]
    NotViewReshapable,

    #[error("zero-sized element types require a dedicated storage policy")]
    ZeroSizedElementType,

    #[error("storage byte span exceeds isize::MAX")]
    StorageByteExtentOverflow,
}
