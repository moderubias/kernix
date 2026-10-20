# Kernix 0.5.1 evidence and verification

This release follows the baseline-integrity milestone. It changes documentation,
source-audited coverage data, development/CI tooling and package versions. Numerical
behavior and library runtime dependencies are unchanged from 0.5.0.

## Independent clean baseline

A detached, clean checkout of release tag v0.5.0 (`def2034`) was tested locally:
479 default, 481 all-feature and 472 no-default workspace tests passed, including
doctests. Formatting, strict all-target/all-feature Clippy, strict rustdoc, MSRV
1.86 and no-default wasm compilation passed. The source CI for that tag previously
passed Linux/Windows/macOS, MSRV and all three portable targets. This verifies the
baseline rather than relying solely on a prior audit's source inspection.

## Candidate checks

The 0.5.1 candidate passed 479 default, 481 all-feature and 472 no-default tests,
strict formatting/Clippy/rustdoc, MSRV 1.86, wasm/i686/aarch64 compilation and nine
downstream README/guide examples. Seven Python tool tests passed, as did the two
Rust tests in the instrumented harness (one quantile test and one allocator probe). The public examples and new source
coverage checker also run in CI. The inventory has 172 numerical entries and 24
Tape recording methods; it explicitly records partial rank-two matmul/linear and
missing layer-normalization VJPs. Its tests reject missing/duplicate/stale entries,
new recording methods and missing numerical regression-test evidence.

The standalone benchmark tool has independent tests for nearest-rank quantiles,
source cleanliness and allocator accounting. Its allocation probe passed one
isolated Miri test importing the exact profiling module. This checks the new dev
allocator wrapper, not all dependencies or all architecture-specific kernels.
Libraries contain no new unsafe code. The ordinary timing binary has no counting
allocator; instrumented timing is rejected.

## Measurement scope

Clean workspace and exact registry 0.5.0 runs are produced by the same standalone
harness. Source/tree IDs, actual manifests and lockfiles, binary/archive hashes,
individual samples, batch means, first-use observations, observed numerical errors,
Rust allocator requests and process RSS are retained separately. Registry mode pins
all three packages exactly. The numerical crate-tree hash permits checking that
later report-only commits do not change measured library bytes.

Comparators include ndarray, nalgebra, faer, Candle CPU and an optimized NumPy
OpenBLAS build. The autodiff comparison uses real graphs on both sides, not manual
derivatives. No API optimization, new derivative, pure-kernel measurement, GPU
support or full application latency improvement is claimed by this release.

Publication and final clean-source measurements are recorded after completion.
