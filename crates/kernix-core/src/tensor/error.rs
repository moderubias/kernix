use core::fmt;
use thiserror::Error;

use crate::{
    layout::{LayoutError, Order},
    storage::{StorageBuildError, StorageError},
};

/// Error in tensor construction, indexing, or a no-copy view operation.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[non_exhaustive]
pub enum TensorError {
    /// Affine layout construction or transformation failed.
    #[error(transparent)]
    Layout(#[from] LayoutError),

    /// Storage allocation or validation failed; contains the underlying error.
    #[error(transparent)]
    Storage(#[from] StorageError),

    /// The number of supplied values differs from the shape element count.
    #[error("shape requires {expected} elements, provided {actual}")]
    ElementCountMismatch {
        #[doc = "Required rank, length or extent."]
        expected: usize,
        #[doc = "Observed rank, length or extent."]
        actual: usize,
    },

    /// The logical tensor cannot be borrowed as a slice in the requested order.
    #[error("tensor cannot be exposed as a {order:?}-contiguous slice")]
    NotContiguous {
        #[doc = "Requested logical contiguous traversal order."]
        order: Order,
    },
}

/// Allocation/layout error, or an error from a user-supplied initializer.
#[derive(Debug)]
pub enum TensorBuildError<E> {
    /// Tensor validation or allocation failed; contains the underlying tensor error.
    Tensor(TensorError),
    /// A fallible initializer failed; the initialized prefix has been cleaned up.
    Initialize {
        #[doc = "Failing logical or physical element index as specified by the variant."]
        index: usize,
        #[doc = "Original initializer error, preserved without conversion."]
        error: E,
    },
}

impl<E> From<TensorError> for TensorBuildError<E> {
    fn from(value: TensorError) -> Self {
        Self::Tensor(value)
    }
}

impl<E> From<StorageBuildError<E>> for TensorBuildError<E> {
    fn from(value: StorageBuildError<E>) -> Self {
        match value {
            StorageBuildError::Storage(err) => Self::Tensor(TensorError::Storage(err)),
            StorageBuildError::Initialize { index, error } => Self::Initialize { index, error },
        }
    }
}

impl<E: fmt::Display> fmt::Display for TensorBuildError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Tensor(error) => fmt::Display::fmt(error, f),
            Self::Initialize { index, error } => write!(
                f,
                "tensor initializer failed at physical element {index}: {error}"
            ),
        }
    }
}

#[cfg(feature = "std")]
impl<E: core::error::Error + 'static> core::error::Error for TensorBuildError<E> {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            Self::Tensor(error) => Some(error),
            Self::Initialize { error, .. } => Some(error),
        }
    }
}
