use crate::shape::ShapeError;
use thiserror::Error;

/// Errors in validating or transforming an affine layout.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[non_exhaustive]
pub enum LayoutError {
    /// Shape validation failed; contains the underlying shape error.
    #[error(transparent)]
    Shape(#[from] ShapeError),

    /// The stride count does not match the shape rank.
    #[error("expected {expected} strides, got {actual}")]
    StrideRankMismatch {
        #[doc = "Required rank, length or extent."]
        expected: usize,
        #[doc = "Observed rank, length or extent."]
        actual: usize,
    },

    /// A required signed stride cannot be represented as isize.
    #[error("canonical stride at axis {axis} cannot be represented as isize")]
    StrideOverflow {
        #[doc = "Zero-based axis or insertion position associated with this error."]
        axis: usize,
    },

    /// Address arithmetic cannot be represented in the supported address range.
    #[error("layout address arithmetic overflow or address outside usize storage range")]
    AddressOverflow,

    /// The layout reaches a negative physical element offset.
    #[error("layout reaches a negative storage element index")]
    NegativeStorageOffset,

    /// The backing allocation is shorter than the reachable element span.
    #[error("layout requires {required} storage elements but storage has {available}")]
    StorageTooSmall {
        #[doc = "Minimum reachable storage length in elements."]
        required: usize,
        #[doc = "Provided initialized storage length in elements."]
        available: usize,
    },

    /// The axis is outside the valid range for this rank.
    #[error("axis {axis} out of bounds for rank {rank}")]
    AxisOutOfBounds {
        #[doc = "Zero-based axis or insertion position associated with this error."]
        axis: usize,
        #[doc = "Number of axes in the relevant shape."]
        rank: usize,
    },

    /// The coordinate count differs from the layout rank.
    #[error("index rank mismatch: expected {expected}, got {actual}")]
    IndexRankMismatch {
        #[doc = "Required rank, length or extent."]
        expected: usize,
        #[doc = "Observed rank, length or extent."]
        actual: usize,
    },

    /// A logical coordinate is outside its dimension.
    #[error("index {index} out of bounds for dimension {dim} at axis {axis}")]
    IndexOutOfBounds {
        #[doc = "Zero-based axis or insertion position associated with this error."]
        axis: usize,
        #[doc = "Failing logical or physical element index as specified by the variant."]
        index: usize,
        #[doc = "Dimension size at the reported axis."]
        dim: usize,
    },

    /// A start/length/step combination addresses outside the source axis.
    #[error("invalid slice at axis {axis}: start={start}, len={len}, step={step}, dim={dim}")]
    InvalidSlice {
        #[doc = "Zero-based axis or insertion position associated with this error."]
        axis: usize,
        #[doc = "First included physical element index or source slice index."]
        start: usize,
        #[doc = "Requested number of elements."]
        len: usize,
        #[doc = "Signed increment between source indices."]
        step: isize,
        #[doc = "Dimension size at the reported axis."]
        dim: usize,
    },

    /// A slice step of zero is not permitted.
    #[error("slice step cannot be zero")]
    ZeroSliceStep,

    /// The aligned dimensions cannot broadcast under trailing-axis singleton rules.
    #[error("broadcast cannot expand dimension {source_dim} to {target} at axis {axis}")]
    BroadcastIncompatible {
        #[doc = "Zero-based axis or insertion position associated with this error."]
        axis: usize,
        #[doc = "Original dimension size before broadcasting."]
        source_dim: usize,
        #[doc = "Requested broadcast dimension size."]
        target: usize,
    },

    /// The requested view shape has a different element count.
    #[error("reshape element count mismatch: old {old}, new {new}")]
    ReshapeElementCountMismatch {
        #[doc = "Original element count."]
        old: usize,
        #[doc = "Requested element count."]
        new: usize,
    },

    /// The stride chunks cannot express the requested no-copy reshape.
    #[error("requested reshape cannot be represented as a view in the selected order")]
    NotViewReshapable,

    /// Zero-sized element types require an unsupported dedicated storage policy.
    #[error("zero-sized element types require a dedicated storage policy")]
    ZeroSizedElementType,

    /// The storage byte extent exceeds isize::MAX.
    #[error("storage byte span exceeds isize::MAX")]
    StorageByteExtentOverflow,
}
