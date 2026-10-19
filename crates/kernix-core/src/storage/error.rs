use core::fmt;

use thiserror::Error;

/// Errors in allocating or adopting host storage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
#[non_exhaustive]
pub enum StorageError {
    #[error("zero-sized element types are unsupported by Kernix storage")]
    ZeroSizedElementType,

    #[error("invalid alignment {requested}: expected a power of two >= {minimum}")]
    InvalidAlignment { requested: usize, minimum: usize },

    #[error("storage of {len} elements of size {element_size} exceeds the CPU addressable extent")]
    CapacityOverflow { len: usize, element_size: usize },

    #[error("allocation of {bytes} bytes at alignment {alignment} failed")]
    AllocationFailed { bytes: usize, alignment: usize },
}

/// Storage-allocation failure or an error produced by a fallible initializer.
#[derive(Debug)]
pub enum StorageBuildError<E> {
    Storage(StorageError),
    Initialize { index: usize, error: E },
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
