use core::fmt;

use thiserror::Error;

/// Errors in allocating or adopting host storage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
#[non_exhaustive]
pub enum StorageError {
    /// Zero-sized element types require an unsupported dedicated storage policy.
    #[error("zero-sized element types are unsupported by Kernix storage")]
    ZeroSizedElementType,

    /// The requested alignment is not a sufficient power of two.
    #[error("invalid alignment {requested}: expected a power of two >= {minimum}")]
    InvalidAlignment {
        #[doc = "Requested alignment in bytes."]
        requested: usize,
        #[doc = "Minimum permitted alignment in bytes."]
        minimum: usize,
    },

    /// Element count times element size exceeds the CPU addressable extent.
    #[error("storage of {len} elements of size {element_size} exceeds the CPU addressable extent")]
    CapacityOverflow {
        #[doc = "Requested number of elements."]
        len: usize,
        #[doc = "Size of one element in bytes."]
        element_size: usize,
    },

    /// The allocator could not satisfy the requested extent and alignment.
    #[error("allocation of {bytes} bytes at alignment {alignment} failed")]
    AllocationFailed {
        #[doc = "Requested allocation size in bytes."]
        bytes: usize,
        #[doc = "Requested allocation alignment in bytes."]
        alignment: usize,
    },
}

/// Storage-allocation failure or an error produced by a fallible initializer.
#[derive(Debug)]
pub enum StorageBuildError<E> {
    /// Storage allocation or validation failed; contains the underlying error.
    Storage(StorageError),
    /// A fallible initializer failed; the initialized prefix has been cleaned up.
    Initialize {
        #[doc = "Failing logical or physical element index as specified by the variant."]
        index: usize,
        #[doc = "Original initializer error, preserved without conversion."]
        error: E,
    },
}

impl<E> From<StorageError> for StorageBuildError<E> {
    fn from(error: StorageError) -> Self {
        Self::Storage(error)
    }
}

impl<E: fmt::Display> fmt::Display for StorageBuildError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Storage(error) => fmt::Display::fmt(error, f),
            Self::Initialize { index, error } => {
                write!(f, "storage initializer failed at element {index}: {error}")
            }
        }
    }
}

#[cfg(feature = "std")]
impl<E: core::error::Error + 'static> core::error::Error for StorageBuildError<E> {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            Self::Storage(err) => Some(err),
            Self::Initialize { error, .. } => Some(error),
        }
    }
}
