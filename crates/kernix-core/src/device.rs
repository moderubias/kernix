//! Explicit buffer ownership and transfers for independently implemented devices.
//!
//! A device buffer need not expose a CPU pointer. Execution capabilities live in
//! higher layers; Shape and Layout never depend on a device implementation.
//!
//! ```
//! use kernix_core::device::{Cpu, Device};
//! let cpu = Cpu;
//! let buffer = cpu.upload(vec![1.0f32, 2.0])?;
//! assert_eq!(cpu.download(&buffer)?, vec![1.0, 2.0]);
//! <Cpu as Device<f32>>::synchronize(&cpu)?;
//! # Ok::<(), kernix_core::storage::StorageError>(())
//! ```
use crate::storage::{CpuBuffer, StorageBuffer, StorageError};
use alloc::vec::Vec;

/// Device-owned initialized storage and explicit physical-order host transfers.
///
/// Successful transfers complete before returning. Asynchronous backends can
/// expose additional event APIs without fabricating synchronously usable slices.
/// Implementations must reject buffers belonging to an incompatible device/context.
pub trait Device<T> {
    /// Initialized storage owned by this device; host access is an optional capability.
    type Buffer: StorageBuffer<T>;
    /// Transfer, allocation, ownership or synchronization failure.
    type Error: core::error::Error;
    /// Human-readable backend name; not a unique allocation or device identifier.
    fn name(&self) -> &str;
    /// Move physical C/F-ordered values into device storage, preserving their order.
    fn upload(&self, values: Vec<T>) -> Result<Self::Buffer, Self::Error>;
    /// Copy physical buffer values to CPU and wait for completion.
    fn download(&self, buffer: &Self::Buffer) -> Result<Vec<T>, Self::Error>;
    /// Wait for outstanding work submitted to this device context.
    fn synchronize(&self) -> Result<(), Self::Error>;
}

/// Portable synchronous CPU device, with no runtime or platform-specific dependencies.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Cpu;

impl<T: Clone> Device<T> for Cpu {
    type Buffer = CpuBuffer<T>;
    type Error = StorageError;
    fn name(&self) -> &str {
        "cpu"
    }
    fn upload(&self, values: Vec<T>) -> Result<Self::Buffer, Self::Error> {
        CpuBuffer::try_from_vec(values)
    }
    fn download(&self, buffer: &Self::Buffer) -> Result<Vec<T>, Self::Error> {
        Ok(buffer.as_slice().to_vec())
    }
    fn synchronize(&self) -> Result<(), Self::Error> {
        Ok(())
    }
}
