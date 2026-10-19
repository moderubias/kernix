//! Versioned, checked CPU training checkpoints with exact f32/f64 values.
//!
//! Checkpoints store named parameters and optional Adam/AdamW moments, counters,
//! bias powers and hyperparameters. Encode/decode works with no_std+alloc; filesystem
//! helpers require std. All fields are validated before restoring live parameters.
//!
//! ```
//! use kernix::{training::Parameter, checkpoint::Checkpoint, Kahen,Order,Tensor};
//! let weight=Parameter::new(Tensor::from_vec(vec![1.0f32,2.0],Kahen::new([2])?,Order::C)?)?;
//! let bytes=Checkpoint::capture(&[("weight",&weight)],None)?.to_bytes()?;
//! let saved=Checkpoint::<f32>::from_bytes(&bytes)?;
//! let mut restored=weight.clone();
//! saved.restore_weights(&mut [("weight",&mut restored)])?;
//! assert_eq!(restored.value().storage_slice(),&[1.0,2.0]);
//! # Ok::<(),Box<dyn std::error::Error>>(())
//! ```
use crate::{
    FloatMath, Kahen, Order, Tensor,
    autograd::TrainFloat,
    training::{Adam, AdamOptions, AdamState, Parameter, TrainingError},
};
use alloc::{
    collections::BTreeSet,
    rc::Rc,
    string::{String, ToString},
    vec::Vec,
};
use thiserror::Error;

/// A malformed/incompatible checkpoint, training validation or filesystem failure.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum CheckpointError {
    /// Format, dtype, name, geometry, checksum or restore-contract validation failed.
    #[error("invalid Kernix checkpoint: {0}")]
    Invalid(&'static str),
    /// Parsed values violate parameter/optimizer invariants.
    #[error(transparent)]
    Training(#[from] TrainingError),
    /// A selected file could not be read/written. Only available with std.
    #[cfg(feature = "std")]
    #[error("checkpoint I/O failed: {0}")]
    Io(#[from] std::io::Error),
}
#[derive(Clone)]
struct SavedState<T> {
    m: Vec<T>,
    v: Vec<T>,
    step: u64,
    beta1_power: T,
    beta2_power: T,
}
struct Entry<T> {
    name: String,
    value: Tensor<T>,
    state: Option<SavedState<T>>,
}
/// An immutable snapshot of named parameters and optional Adam optimizer state.
/// Version 1 uses little-endian dimensions/scalars, exact dtype bits and an FNV-1a
/// checksum for accidental corruption detection (not a cryptographic signature).
/// Captured names form the restore schema; order need not match on restore.
pub struct Checkpoint<T: TrainFloat> {
    entries: Vec<Entry<T>>,
    adam: Option<AdamOptions<T>>,
}
fn invalid(message: &'static str) -> CheckpointError {
    CheckpointError::Invalid(message)
}
impl<T: TrainFloat> Checkpoint<T> {
    /// Copy a named set of parameters and, when provided, their optimizer moments.
    /// Names must be nonempty and unique. Only supplied parameter identities are
    /// captured; include the entire model to resume its complete optimizer state.
    pub fn capture(
        parameters: &[(&str, &Parameter<T>)],
        adam: Option<&Adam<T>>,
    ) -> Result<Self, CheckpointError> {
        let mut names = BTreeSet::new();
        let mut entries = Vec::with_capacity(parameters.len());
        for &(name, parameter) in parameters {
            if name.is_empty() || !names.insert(name) {
                return Err(invalid("parameter names must be nonempty and unique"));
            }
            let state = adam
                .and_then(|a| {
                    a.states
                        .iter()
                        .find(|s| Rc::ptr_eq(&s.owner, &parameter.owner))
                })
                .map(|s| SavedState {
                    m: s.m.clone(),
                    v: s.v.clone(),
                    step: s.step,
                    beta1_power: s.beta1_power,
                    beta2_power: s.beta2_power,
                });
            entries.push(Entry {
                name: name.to_string(),
                value: parameter.value.clone(),
                state,
            });
        }
        Ok(Self {
            entries,
            adam: adam.map(|a| a.options),
        })
    }
    /// Iterate the stored immutable parameter names and canonical C-order values.
    pub fn parameters(&self) -> impl ExactSizeIterator<Item = (&str, &Tensor<T>)> {
        self.entries.iter().map(|e| (e.name.as_str(), &e.value))
    }
    fn candidates(
        &self,
        parameters: &[(&str, &mut Parameter<T>)],
    ) -> Result<Vec<Tensor<T>>, CheckpointError> {
        if parameters.len() != self.entries.len() {
            return Err(invalid("parameter name set differs from checkpoint"));
        }
        let mut names = BTreeSet::new();
        let mut values = Vec::with_capacity(parameters.len());
        for (name, parameter) in parameters {
            if !names.insert(*name) {
                return Err(invalid("duplicate restore parameter name"));
            }
            let e = self
                .entries
                .iter()
                .find(|e| e.name == *name)
                .ok_or_else(|| invalid("missing restore parameter name"))?;
            if parameter.value.shape() != e.value.shape() {
                return Err(invalid("restore shape mismatch"));
            }
            values.push(e.value.clone());
        }
        Ok(values)
    }
    /// Restore only weights (for inference), validating the full name/shape set
    /// before any mutation. Optimizer state is left to the caller's explicit policy.
    pub fn restore_weights(
        &self,
        parameters: &mut [(&str, &mut Parameter<T>)],
    ) -> Result<(), CheckpointError> {
        let values = self.candidates(parameters)?;
        for ((_, p), v) in parameters.iter_mut().zip(values) {
            p.value = v;
        }
        Ok(())
    }
    /// Restore weights and optional Adam state transactionally. Optimizer presence
    /// must match the snapshot. Existing optimizer moments are replaced, and saved
    /// hyperparameters restored. Names can be supplied in a different order.
    pub fn restore(
        &self,
        parameters: &mut [(&str, &mut Parameter<T>)],
        optimizer: Option<&mut Adam<T>>,
    ) -> Result<(), CheckpointError> {
        if self.adam.is_some() != optimizer.is_some() {
            return Err(invalid(
                "optimizer presence mismatch; use restore_weights for inference",
            ));
        }
        let values = self.candidates(parameters)?;
        let mut states = Vec::new();
        if let Some(options) = self.adam {
            Adam::from_options(options)?;
            for (name, p) in parameters.iter() {
                let e = self
                    .entries
                    .iter()
                    .find(|e| e.name == *name)
                    .expect("names validated");
                if let Some(s) = &e.state {
                    states.push(AdamState {
                        owner: p.owner.clone(),
                        m: s.m.clone(),
                        v: s.v.clone(),
                        step: s.step,
                        beta1_power: s.beta1_power,
                        beta2_power: s.beta2_power,
                    });
                }
            }
        }
        for ((_, p), v) in parameters.iter_mut().zip(values) {
            p.value = v;
        }
        if let Some(optimizer) = optimizer {
            optimizer.options = self.adam.expect("presence validated");
            optimizer.states = states;
        }
        Ok(())
    }
    /// Encode version-one little-endian values with a trailing checksum. Only
    /// f32/f64 dtype identifiers are supported. No platform-native pointers are stored.
    pub fn to_bytes(&self) -> Result<Vec<u8>, CheckpointError> {
        if T::DTYPE != 1 && T::DTYPE != 2 {
            return Err(invalid("unsupported checkpoint dtype"));
        }
        let mut out = b"KRNXCK01".to_vec();
        out.extend_from_slice(&1u16.to_le_bytes());
        out.push(T::DTYPE);
        write_size(&mut out, self.entries.len());
        for e in &self.entries {
            write_size(&mut out, e.name.len());
            out.extend_from_slice(e.name.as_bytes());
            write_size(&mut out, e.value.rank());
            for &dim in e.value.shape().dims() {
                write_size(&mut out, dim);
            }
            write_size(&mut out, e.value.numel());
            for &value in e.value.storage_slice() {
                write_float(&mut out, value);
            }
        }
        out.push(u8::from(self.adam.is_some()));
        if let Some(o) = self.adam {
            for value in [o.learning_rate, o.beta1, o.beta2, o.epsilon, o.weight_decay] {
                write_float(&mut out, value);
            }
            for e in &self.entries {
                out.push(u8::from(e.state.is_some()));
                if let Some(s) = &e.state {
                    out.extend_from_slice(&s.step.to_le_bytes());
                    write_float(&mut out, s.beta1_power);
                    write_float(&mut out, s.beta2_power);
                    for &value in s.m.iter().chain(&s.v) {
                        write_float(&mut out, value);
                    }
                }
            }
        }
        let digest = checksum(&out);
        out.extend_from_slice(&digest.to_le_bytes());
        Ok(out)
    }
    /// Validate and decode a bounded byte slice. Reject unknown version/dtype,
    /// checksum failure, duplicate names, overflow, nonfinite values, invalid Adam
    /// state, trailing bytes and truncated payloads before returning a snapshot.
    /// Allocation lengths are bounded by checked remaining payload lengths.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, CheckpointError> {
        if bytes.len() < 28 {
            return Err(invalid("truncated header"));
        }
        let payload = &bytes[..bytes.len() - 8];
        let stored = u64::from_le_bytes(bytes[bytes.len() - 8..].try_into().expect("eight bytes"));
        if checksum(payload) != stored {
            return Err(invalid("checksum mismatch"));
        }
        let mut r = Reader {
            bytes: payload,
            position: 0,
        };
        if r.take(8)? != b"KRNXCK01" || r.array::<2>()? != 1u16.to_le_bytes() {
            return Err(invalid("unknown checkpoint magic/version"));
        }
        let dtype = r.array::<1>()?[0];
        if dtype != T::DTYPE || !(dtype == 1 || dtype == 2) {
            return Err(invalid("dtype mismatch or unsupported dtype"));
        }
        let count = r.size()?;
        if count > r.remaining() / 24 {
            return Err(invalid("parameter count exceeds payload"));
        }
        let mut entries = Vec::with_capacity(count);
        let mut names = BTreeSet::new();
        for _ in 0..count {
            let length = r.size()?;
            let name = core::str::from_utf8(r.take(length)?)
                .map_err(|_| invalid("invalid UTF-8 name"))?
                .to_string();
            if name.is_empty() || !names.insert(name.clone()) {
                return Err(invalid("empty or duplicate parameter name"));
            }
            let rank = r.size()?;
            if rank > r.remaining() / 8 {
                return Err(invalid("rank exceeds payload"));
            }
            let mut dims = Vec::with_capacity(rank);
            for _ in 0..rank {
                dims.push(r.size()?);
            }
            let shape = Kahen::new(dims).map_err(TrainingError::from)?;
            let len = r.size()?;
            if len != shape.numel() {
                return Err(invalid("element count mismatch"));
            }
            let data = r.values::<T>(len)?;
            let value = Tensor::from_vec(data, shape, Order::C).map_err(TrainingError::from)?;
            entries.push(Entry {
                name,
                value,
                state: None,
            });
        }
        let adam = match r.array::<1>()?[0] {
            0 => None,
            1 => {
                let options = AdamOptions {
                    learning_rate: r.float()?,
                    beta1: r.float()?,
                    beta2: r.float()?,
                    epsilon: r.float()?,
                    weight_decay: r.float()?,
                };
                Adam::from_options(options)?;
                for e in &mut entries {
                    match r.array::<1>()?[0] {
                        0 => (),
                        1 => {
                            let step = u64::from_le_bytes(r.array()?);
                            let beta1_power = r.float()?;
                            let beta2_power = r.float()?;
                            if step == 0
                                || beta1_power < T::zero()
                                || beta1_power >= T::one()
                                || beta2_power < T::zero()
                                || beta2_power >= T::one()
                            {
                                return Err(invalid("invalid Adam counter or powers"));
                            }
                            let m = r.values(e.value.numel())?;
                            let v = r.values(e.value.numel())?;
                            if v.iter().any(|x| *x < T::zero()) {
                                return Err(invalid("negative Adam second moment"));
                            }
                            e.state = Some(SavedState {
                                m,
                                v,
                                step,
                                beta1_power,
                                beta2_power,
                            });
                        }
                        _ => return Err(invalid("invalid optimizer-state presence flag")),
                    }
                }
                Some(options)
            }
            _ => return Err(invalid("invalid optimizer presence flag")),
        };
        if r.remaining() != 0 {
            return Err(invalid("trailing payload bytes"));
        }
        Ok(Self { entries, adam })
    }
    /// Write a temporary sibling file, sync it and rename it into place. Requires
    /// std; failed writes clean up the temporary file. Filesystem rename/durability
    /// semantics still depend on the host filesystem. Existing checkpoints are replaced.
    #[cfg(feature = "std")]
    pub fn save(&self, path: impl AsRef<std::path::Path>) -> Result<(), CheckpointError> {
        use std::io::Write;
        let path = path.as_ref();
        let bytes = self.to_bytes()?;
        let temporary = path.with_extension(alloc::format!("kernix-tmp-{}", std::process::id()));
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temporary)?;
        let result = (|| {
            file.write_all(&bytes)?;
            file.sync_all()?;
            drop(file);
            std::fs::rename(&temporary, path)
        })(); // close before rename on Windows
        if result.is_err() {
            let _ = std::fs::remove_file(&temporary);
        }
        result?;
        Ok(())
    }
    /// Read a checkpoint with a default 256 MiB file-size limit. Use load_with_limit
    /// for an explicitly different resource budget; missing files remain errors.
    #[cfg(feature = "std")]
    pub fn load(path: impl AsRef<std::path::Path>) -> Result<Self, CheckpointError> {
        Self::load_with_limit(path, 256 * 1024 * 1024)
    }
    /// Read at most max_bytes+1 bytes, rejecting excess size before parsing.
    /// The read cap also handles a file growing between metadata inspection and read.
    #[cfg(feature = "std")]
    pub fn load_with_limit(
        path: impl AsRef<std::path::Path>,
        max_bytes: usize,
    ) -> Result<Self, CheckpointError> {
        use std::io::Read;
        let file = std::fs::File::open(path)?;
        let cap = u64::try_from(max_bytes)
            .ok()
            .and_then(|n| n.checked_add(1))
            .ok_or_else(|| invalid("file limit out of range"))?;
        if file.metadata()?.len() > max_bytes as u64 {
            return Err(invalid("checkpoint exceeds file size limit"));
        }
        let mut bytes = Vec::new();
        file.take(cap).read_to_end(&mut bytes)?;
        if bytes.len() > max_bytes {
            return Err(invalid("checkpoint exceeds file size limit"));
        }
        Self::from_bytes(&bytes)
    }
}
fn write_size(out: &mut Vec<u8>, value: usize) {
    out.extend_from_slice(&(value as u64).to_le_bytes());
}
fn write_float<T: TrainFloat>(out: &mut Vec<u8>, value: T) {
    if T::DTYPE == 1 {
        out.extend_from_slice(&(value.to_f64() as f32).to_bits().to_le_bytes());
    } else {
        out.extend_from_slice(&value.to_f64().to_bits().to_le_bytes());
    }
}
fn checksum(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0xcbf29ce484222325, |hash, &b| {
        (hash ^ u64::from(b)).wrapping_mul(0x100000001b3)
    })
}
struct Reader<'a> {
    bytes: &'a [u8],
    position: usize,
}
impl<'a> Reader<'a> {
    fn remaining(&self) -> usize {
        self.bytes.len() - self.position
    }
    fn take(&mut self, len: usize) -> Result<&'a [u8], CheckpointError> {
        let end = self
            .position
            .checked_add(len)
            .ok_or_else(|| invalid("length overflow"))?;
        let bytes = self
            .bytes
            .get(self.position..end)
            .ok_or_else(|| invalid("truncated payload"))?;
        self.position = end;
        Ok(bytes)
    }
    fn array<const N: usize>(&mut self) -> Result<[u8; N], CheckpointError> {
        Ok(self.take(N)?.try_into().expect("fixed byte count"))
    }
    fn size(&mut self) -> Result<usize, CheckpointError> {
        usize::try_from(u64::from_le_bytes(self.array()?))
            .map_err(|_| invalid("dimension outside usize range"))
    }
    fn float<T: TrainFloat>(&mut self) -> Result<T, CheckpointError> {
        let value = if T::DTYPE == 1 {
            f32::from_bits(u32::from_le_bytes(self.array()?)) as f64
        } else {
            f64::from_bits(u64::from_le_bytes(self.array()?))
        };
        let value = T::from_f64(value);
        if !FloatMath::is_finite(value) {
            return Err(invalid("nonfinite checkpoint scalar"));
        }
        Ok(value)
    }
    fn values<T: TrainFloat>(&mut self, len: usize) -> Result<Vec<T>, CheckpointError> {
        let size = if T::DTYPE == 1 { 4 } else { 8 };
        if len > self.remaining() / size {
            return Err(invalid("element count exceeds payload"));
        }
        let mut out = Vec::with_capacity(len);
        for _ in 0..len {
            out.push(self.float()?);
        }
        Ok(out)
    }
}
