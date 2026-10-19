//! Configuration defaults, strict schema and explicit context behavior.
#![cfg(feature = "config")]
use kernix::config::{Config, Context};
#[test]
fn empty_input_and_missing_optional_file_use_defaults() {
    assert_eq!(Config::from_toml("").unwrap(), Config::default());
    let path = std::env::temp_dir()
        .join(format!("kernix-absent-{}", std::process::id()))
        .join("kernix.toml");
    assert_eq!(Config::load_optional(path).unwrap(), Config::default());
    assert_eq!(Context::default().cpu().threads(), 1);
}
#[test]
fn typos_and_invalid_thread_counts_are_errors() {
    for input in [
        "[cpu]\nthread=4",
        "[gpu]\nthreads=2",
        "[cpu]\nthreads=0",
        "[cpu]\nthreads=-1",
        "nonsense",
    ] {
        assert!(Config::from_toml(input).is_err(), "{input}");
    }
}
#[test]
fn filesystem_errors_are_not_silently_defaulted() {
    assert!(Config::load_optional(std::env::temp_dir()).is_err());
}
#[test]
fn valid_explicit_context() {
    let config = Config::from_toml("[cpu]\nthreads=1").unwrap();
    assert_eq!(Context::new(config.clone()).unwrap().config(), &config);
}
#[cfg(not(feature = "parallel"))]
#[test]
fn multiple_threads_require_feature() {
    assert!(Context::new(Config::from_toml("[cpu]\nthreads=2").unwrap()).is_err());
}
#[cfg(feature = "parallel")]
#[test]
fn thread_setting_constructs_private_pool() {
    let context = Context::new(Config::from_toml("[cpu]\nthreads=2").unwrap()).unwrap();
    assert_eq!(context.cpu().threads(), 2);
}
