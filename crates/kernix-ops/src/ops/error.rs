use alloc::vec::Vec;
use core::fmt;
use kernix_core::{shape::ShapeError, tensor::TensorError};

/// Errors in logical tensor operations and checked execution planning.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum OpsError {
    /// Shape validation failed; contains the underlying shape error.
    Shape(ShapeError),
    /// Tensor validation or allocation failed; contains the underlying tensor error.
    Tensor(TensorError),
    /// The aligned dimensions cannot broadcast under trailing-axis singleton rules.
    BroadcastIncompatible {
        #[doc = "Zero-based axis or insertion position associated with this error."]
        axis: usize,
        #[doc = "Left operand dimension or axis count."]
        left: usize,
        #[doc = "Right operand dimension or axis count."]
        right: usize,
    },
    /// The two contracted dimensions have different sizes.
    ContractDimensionMismatch {
        #[doc = "Left operand dimension or axis count."]
        left: usize,
        #[doc = "Right operand dimension or axis count."]
        right: usize,
    },
    /// An operation requires an exact rank that the input does not have.
    ExactRank {
        #[doc = "Operation that detected the failure."]
        operation: &'static str,
        #[doc = "Required rank, length or extent."]
        expected: usize,
        #[doc = "Observed rank, length or extent."]
        actual: usize,
    },
    /// The input rank is below the minimum required by the operation.
    InvalidRank {
        #[doc = "Operation that detected the failure."]
        operation: &'static str,
        #[doc = "Minimum permitted alignment in bytes."]
        minimum: usize,
        #[doc = "Observed rank, length or extent."]
        actual: usize,
    },
    /// The axis is outside the valid range for this rank.
    AxisOutOfBounds {
        #[doc = "Zero-based axis or insertion position associated with this error."]
        axis: isize,
        #[doc = "Number of axes in the relevant shape."]
        rank: usize,
    },
    /// An axis occurs more than once in a permutation or reduction.
    DuplicateAxis {
        #[doc = "Zero-based axis or insertion position associated with this error."]
        axis: usize,
    },
    /// An empty reduction has no defined identity for this operation.
    EmptyReduction,
    /// Reduction count minus correction is not positive.
    InvalidDegreesOfFreedom {
        #[doc = "Number of contributing reduction elements."]
        count: usize,
        #[doc = "Degrees of freedom subtracted from the reduction count."]
        correction: usize,
    },
    /// The operation requires at least one input tensor.
    EmptyInputSequence {
        #[doc = "Operation that detected the failure."]
        operation: &'static str,
    },
    /// An input dimension differs from the required dimension.
    DimensionMismatch {
        #[doc = "Zero-based axis or insertion position associated with this error."]
        axis: usize,
        #[doc = "Required rank, length or extent."]
        expected: usize,
        #[doc = "Observed rank, length or extent."]
        actual: usize,
    },
    /// Computing an output dimension overflowed usize.
    DimensionOverflow {
        #[doc = "Zero-based axis or insertion position associated with this error."]
        axis: usize,
    },
    /// A logical coordinate is outside its dimension.
    IndexOutOfBounds {
        #[doc = "Zero-based axis or insertion position associated with this error."]
        axis: usize,
        #[doc = "Failing logical or physical element index as specified by the variant."]
        index: usize,
        #[doc = "Dimension size at the reported axis."]
        dim: usize,
    },
    /// Padding specifications must have one pair per axis.
    PaddingRankMismatch {
        #[doc = "Required rank, length or extent."]
        expected: usize,
        #[doc = "Observed rank, length or extent."]
        actual: usize,
    },
    /// The partition lengths do not sum to the source dimension.
    SplitSizesMismatch {
        #[doc = "Zero-based axis or insertion position associated with this error."]
        axis: usize,
        #[doc = "Required rank, length or extent."]
        expected: usize,
        #[doc = "Observed rank, length or extent."]
        actual: usize,
    },
    /// The left and right contraction axis lists have different lengths.
    ContractAxesMismatch {
        #[doc = "Left operand dimension or axis count."]
        left: usize,
        #[doc = "Right operand dimension or axis count."]
        right: usize,
    },
    /// An operation parameter violates its documented domain.
    InvalidParameter {
        #[doc = "Operation that detected the failure."]
        operation: &'static str,
        #[doc = "Invalid parameter and its required constraint."]
        parameter: &'static str,
    },
    /// A checked scalar conversion failed at these logical coordinates.
    Cast {
        #[doc = "Logical C-order coordinates of the failing element."]
        indices: Vec<usize>,
    },
    /// A lower clamp bound is greater than the upper bound or incomparable.
    InvalidBounds,
    /// An operation requiring finite data encountered NaN or infinity.
    NonFiniteInput {
        #[doc = "Operation that detected the failure."]
        operation: &'static str,
        #[doc = "Logical C-order coordinates of the failing element."]
        indices: Vec<usize>,
    },
    /// The contributing element count cannot be represented as usize.
    ReductionCountOverflow,
    /// Distinct destination indices are overlapping or not proven disjoint.
    OverlappingOutput,
    /// The source or mask cannot broadcast to the existing destination shape.
    InPlaceBroadcastIncompatible,
    /// Explicit checked integer arithmetic failed at this *output* coordinate.
    /// Includes division by zero, signed division overflow and out-of-range
    /// results of add/sub/mul/neg. Normal operators retain native semantics.
    Arithmetic {
        #[doc = "Operation that detected the failure."]
        operation: &'static str,
        #[doc = "Logical C-order coordinates of the failing element."]
        indices: Vec<usize>,
    },
}

impl From<ShapeError> for OpsError {
    fn from(value: ShapeError) -> Self {
        Self::Shape(value)
    }
}
impl From<TensorError> for OpsError {
    fn from(value: TensorError) -> Self {
        Self::Tensor(value)
    }
}

impl fmt::Display for OpsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Shape(error) => fmt::Display::fmt(error, f),
            Self::Tensor(error) => fmt::Display::fmt(error, f),
            Self::BroadcastIncompatible { axis, left, right } => write!(
                f,
                "broadcast mismatch at output axis {axis}: {left} vs {right}"
            ),
            Self::ContractDimensionMismatch { left, right } => {
                write!(f, "matmul contraction mismatch: {left} vs {right}")
            }
            Self::ExactRank {
                operation,
                expected,
                actual,
            } => write!(f, "{operation} requires rank {expected}, got {actual}"),
            Self::InvalidRank {
                operation,
                minimum,
                actual,
            } => write!(f, "{operation} requires rank >= {minimum}, got {actual}"),
            Self::AxisOutOfBounds { axis, rank } => {
                write!(f, "axis {axis} is out of bounds for rank {rank}")
            }
            Self::DuplicateAxis { axis } => write!(f, "duplicate reduction axis {axis}"),
            Self::EmptyInputSequence { operation } => {
                write!(f, "{operation} requires at least one input tensor")
            }
            Self::DimensionMismatch {
                axis,
                expected,
                actual,
            } => write!(
                f,
                "dimension mismatch at axis {axis}: expected {expected}, got {actual}"
            ),
            Self::DimensionOverflow { axis } => write!(f, "dimension size overflow at axis {axis}"),
            Self::IndexOutOfBounds { axis, index, dim } => {
                write!(f, "index {index} out of bounds at axis {axis} (size {dim})")
            }
            Self::PaddingRankMismatch { expected, actual } => write!(
                f,
                "padding rank mismatch: expected {expected} axes, got {actual}"
            ),
            Self::SplitSizesMismatch {
                axis,
                expected,
                actual,
            } => write!(
                f,
                "split sizes at axis {axis} sum to {actual}, expected {expected}"
            ),
            Self::ContractAxesMismatch { left, right } => write!(
                f,
                "tensordot contracted axis count mismatch: {left} vs {right}"
            ),
            Self::InvalidParameter {
                operation,
                parameter,
            } => write!(f, "{operation}: invalid {parameter}"),
            Self::Cast { indices } => write!(f, "checked dtype conversion failed at {indices:?}"),
            Self::InvalidBounds => {
                f.write_str("invalid clamp bounds (minimum > maximum or unordered)")
            }
            Self::NonFiniteInput { operation, indices } => write!(
                f,
                "{operation} requires finite inputs; invalid value at {indices:?}"
            ),
            Self::InvalidDegreesOfFreedom { count, correction } => write!(
                f,
                "variance/std requires count > correction, got count {count}, correction {correction}"
            ),
            Self::EmptyReduction => {
                f.write_str("reduction has no elements (no identity/undefined mean)")
            }
            Self::ReductionCountOverflow => f.write_str("reduction element count overflows usize"),
            Self::OverlappingOutput => {
                f.write_str("in-place destination may contain aliased logical elements")
            }
            Self::InPlaceBroadcastIncompatible => {
                f.write_str("right-hand shape cannot broadcast into destination rank")
            }
            Self::Arithmetic { operation, indices } => {
                write!(f, "checked {operation} failed at logical index {indices:?}")
            }
        }
    }
}

impl core::error::Error for OpsError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            Self::Shape(error) => Some(error),
            Self::Tensor(error) => Some(error),
            _ => None,
        }
    }
}

pub(crate) fn flatten_build_error(
    error: kernix_core::tensor::TensorBuildError<OpsError>,
) -> OpsError {
    use kernix_core::tensor::TensorBuildError;
    match error {
        TensorBuildError::Tensor(error) => OpsError::Tensor(error),
        TensorBuildError::Initialize { error, .. } => error,
    }
}
