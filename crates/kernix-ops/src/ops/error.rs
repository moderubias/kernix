use alloc::vec::Vec;
use core::fmt;
use kernix_core::{shape::ShapeError, tensor::TensorError};

/// Errors in logical tensor operations and checked execution planning.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum OpsError {
    Shape(ShapeError),
    Tensor(TensorError),
    BroadcastIncompatible {
        axis: usize,
        left: usize,
        right: usize,
    },
    ContractDimensionMismatch {
        left: usize,
        right: usize,
    },
    InvalidRank {
        operation: &'static str,
        minimum: usize,
        actual: usize,
    },
    AxisOutOfBounds {
        axis: isize,
        rank: usize,
    },
    DuplicateAxis {
        axis: usize,
    },
    EmptyReduction,
    ReductionCountOverflow,
    OverlappingOutput,
    InPlaceBroadcastIncompatible,
    /// Explicit checked integer arithmetic failed at this *output* coordinate.
    /// Includes division by zero, signed division overflow and out-of-range
    /// results of add/sub/mul/neg. Normal operators retain native semantics.
    Arithmetic {
        operation: &'static str,
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
            Self::InvalidRank {
                operation,
                minimum,
                actual,
            } => write!(f, "{operation} requires rank >= {minimum}, got {actual}"),
            Self::AxisOutOfBounds { axis, rank } => {
                write!(f, "axis {axis} is out of bounds for rank {rank}")
            }
            Self::DuplicateAxis { axis } => write!(f, "duplicate reduction axis {axis}"),
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
