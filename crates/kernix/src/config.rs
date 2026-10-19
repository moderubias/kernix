//! Optional application configuration with explicit loading and safe defaults.
//!
//! ```
//! use kernix::config::{Config, Context};
//! let config = Config::from_toml("[cpu]\nthreads = 1")?;
//! let context = Context::new(config)?;
//! assert_eq!(context.cpu().threads(),1);
//! // Context::default() never touches the filesystem.
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
use crate::{CpuExecutor, OpsError};
use serde::Deserialize;
use std::{
    fmt,
    path::{Path, PathBuf},
};

/// Application settings. Unknown tables/keys are rejected instead of ignored.
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Config {
    /// Portable CPU execution settings.
    pub cpu: CpuConfig,
}
/// CPU scheduling settings; deterministic scalar summation order is preserved.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct CpuConfig {
    /// Worker count. Default one; more than one requires the `parallel` feature.
    pub threads: usize,
}
impl Default for CpuConfig {
    fn default() -> Self {
        Self { threads: 1 }
    }
}

/// Parsing, filesystem or validated-execution-context construction failure.
#[derive(Debug)]
#[non_exhaustive]
pub enum ConfigError {
    /// The TOML syntax/schema is invalid, or contains unknown keys.
    Parse(toml::de::Error),
    /// Reading a specified file failed (an optional missing file is handled separately).
    Io {
        /// File the application explicitly requested.
        path: PathBuf,
        /// Original filesystem failure.
        source: std::io::Error,
    },
    /// CPU settings cannot be fulfilled by the selected build or operating system.
    Execution(OpsError),
}
impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parse(e) => write!(f, "invalid Kernix configuration: {e}"),
            Self::Io { path, source } => write!(
                f,
                "cannot read Kernix configuration {}: {source}",
                path.display()
            ),
            Self::Execution(e) => write!(f, "cannot create Kernix execution context: {e}"),
        }
    }
}
impl std::error::Error for ConfigError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(match self {
            Self::Parse(e) => e,
            Self::Io { source, .. } => source,
            Self::Execution(e) => e,
        })
    }
}
impl Config {
    /// Parse strict TOML settings. Empty input uses defaults; malformed/unknown
    /// values or zero workers return an error. More-worker support is checked by Context.
    pub fn from_toml(input: &str) -> Result<Self, ConfigError> {
        let config: Self = toml::from_str(input).map_err(ConfigError::Parse)?;
        if config.cpu.threads == 0 {
            return Err(ConfigError::Execution(OpsError::InvalidParameter {
                operation: "Config::from_toml",
                parameter: "cpu.threads must be positive",
            }));
        }
        Ok(config)
    }
    /// Read an explicitly chosen optional file. Only NotFound selects defaults;
    /// permission/read/UTF-8/schema errors are preserved with the requested path.
    pub fn load_optional(path: impl AsRef<Path>) -> Result<Self, ConfigError> {
        let path = path.as_ref();
        match std::fs::metadata(path) {
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(Self::default()),
            Err(source) => {
                return Err(ConfigError::Io {
                    path: path.to_path_buf(),
                    source,
                });
            }
            Ok(metadata) if !metadata.is_file() => {
                return Err(ConfigError::Io {
                    path: path.to_path_buf(),
                    source: std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "configuration path is not a regular file",
                    ),
                });
            }
            Ok(_) => (),
        }
        let input = std::fs::read_to_string(path).map_err(|source| ConfigError::Io {
            path: path.to_path_buf(),
            source,
        })?;
        Self::from_toml(&input)
    }
}
/// An application-owned CPU context. Does not modify global settings or inspect
/// the current directory unless the application explicitly loads a file.
#[derive(Clone, Default)]
pub struct Context {
    config: Config,
    cpu: CpuExecutor,
}
impl Context {
    /// Validate settings and construct the requested CPU scheduling policy.
    pub fn new(config: Config) -> Result<Self, ConfigError> {
        let cpu = CpuExecutor::new(config.cpu.threads).map_err(ConfigError::Execution)?;
        Ok(Self { config, cpu })
    }
    /// Load optional TOML from a caller-chosen path, then validate executor support.
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, ConfigError> {
        Self::new(Config::load_optional(path)?)
    }
    /// Borrow the validated settings used to construct this context.
    pub fn config(&self) -> &Config {
        &self.config
    }
    /// Borrow the context's CPU executor; use this to apply its worker policy.
    pub fn cpu(&self) -> &CpuExecutor {
        &self.cpu
    }
}
