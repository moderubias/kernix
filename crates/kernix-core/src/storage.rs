//! Device-independent storage contract and a concrete, owned CPU backend.
//!
//! `Storage` owns elements. `Layout` describes indexing; neither owns the other.
//! Borrowed views bind them only after validating every reachable element.
//! Strides are expressed in *elements*, not bytes.

mod buffer;
mod cpu;
mod error;
mod view;

pub use buffer::{CpuStorage, HostBuffer, Storage, StorageBuffer};
pub use cpu::CpuBuffer;
pub use error::{StorageBuildError, StorageError};
pub use view::{StorageView, StorageViewMut};

#[cfg(target_has_atomic = "ptr")]
pub use buffer::SharedStorage;

#[cfg(target_has_atomic = "ptr")]
pub type SharedCpuStorage<T> = SharedStorage<T, CpuBuffer<T>>;
