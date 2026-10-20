# Kernix 0.6.0 verification

This release combines the M0 baseline/evidence work with compatible developer
experience improvements. It adds checked zeros/ones, shares scalar identities
through core with existing paths preserved, and adds executable workflow coverage.
Existing literal element/rank semantics, numerical algorithms and runtime dependency
requirements remain intact. No new VJP or accelerator is claimed.

## Clean 0.5.0 baseline

A detached, clean checkout of v0.5.0 (`def2034`) passed 479 default, 481 all-feature
and 472 no-default workspace tests, formatting, strict Clippy/rustdoc, MSRV 1.86
and no-default wasm compilation. Its existing source CI passed Linux/Windows/macOS,
MSRV and all portable compilation checks. The new registry benchmark mode pins
all three Kernix packages exactly to 0.5.0 rather than selecting newer compatible
transitive packages. Earlier 0.5.1 development candidates were never published.

## API and compatibility checks

The ergonomics suite executes the ten workflows listed in API-ERGONOMICS.md.
Additional regressions cover non-Clone/non-Default identities, old Zero/One imports,
array-valued literals, macro evaluation counts/order, inferred dtype, scalar/empty/
high-rank shapes, unsupported zero-sized types and overflow before generation.
External no_std consumers execute on host Rust 1.86 and compile for portable targets.
The core gains no ops dependency or new third-party dependency.

Source-checked coverage classifies 172 numerical entries and 24 Tape recording
methods. Partial rank-two matmul/Linear recording and missing normalization VJPs
remain explicit. Eight Python tests check inventory/version drift, real test evidence,
quantiles and source cleanliness. The profiling wrapper passed an isolated Miri
regression importing its exact source. This does not certify every native kernel.

## Measurement scope

The standalone harness compares ordinary CPU work with ndarray, nalgebra, faer,
Candle 0.11 and explicitly verified NumPy/OpenBLAS. Per-call and batch-mean
quantiles, first-use samples, allocator requests and process RSS are separate metrics.
Timing binaries do not use allocator instrumentation. Construction cases compare
owned 64x64 f32 values; 0.5.0 baselines use full because zeros/ones did not exist,
which is explicit in metadata. Automatic differentiation is compared with a real
Candle graph rather than manual derivatives.

## Local release checks

The 0.6.0 candidate passed 495 default, 497 all-feature and 488 no-default
workspace tests (including rustdoc), strict formatting/Clippy/rustdoc and MSRV
1.86. The documentation consumer executes 12 examples. The separate no_std
workflow executes on Rust 1.86 and compiles for wasm32, i686 and aarch64;
the workspace also passes no-default compilation for those three targets.
The harness passes its two feature-enabled unit tests and strict Clippy.

## Retained clean evidence

Both reports measure harness source `041f2a158c3e51340b5b9e6df6886fa52584f9b5`
with a clean tree before/after and 40 validated library/workload cases each:

- [Workspace 0.6.0](../benchmarks/results/0.6.0-clean-041f2a1/README.md),
  measured crates tree `ac83ebc428cbf804f058c55b9948b117e5ea0cb5`.
- [Published 0.5.0 baseline](../benchmarks/results/0.5.0-registry-041f2a1/README.md),
  all three packages pinned exactly, registry VCS source `def2034`.

The saved driver lockfiles resolve the same ndarray 0.17.2, nalgebra 0.35.0,
faer 0.24.4, Candle 0.11.0 and matrixmultiply 0.3.11. Both use verified NumPy
OpenBLAS, one CPU affinity and one-thread settings. Raw samples, accuracy budgets,
allocator/RSS results, compiler/environment provenance and source archives are retained.
The archives contain no private web/planning files.

For the new 64x64 f32 constructors, per-call medians were 0.134 us zeros and
0.183 us ones, versus 0.184/0.183 us for the previous full implementation.
Each new constructor requested one 16,384-byte buffer and no reallocations.
These sub-microsecond shared-machine observations do not establish a general
speedup. Kernix still loses representative GEMM128, QR and Cholesky cases to
specialized comparators; every result is retained. No existing numerical kernel
was changed in this release.

## Publication and downstream verification

All three 0.6.0 packages were uploaded and confirmed available by Cargo. Their
registry VCS records point to `56e71f003a1e8234ac0bed9bc1a97d8ffac35666`, tagged
`v0.6.0`. This report commit follows the measured source but preserves its exact
crates tree; no published Rust/README payload changed after the benchmark.

Downloaded registry archives were inspected, not only local package staging:

| Package | Files | Archive SHA-256 |
|---|---:|---|
| kernix-core | 48 | `2b89fa09f216cbe06e1dea3e0caca858db8574a2c898e9a66b9ac40e4afe36d4` |
| kernix-ops | 51 | `3996dd2837761806b09c09722689f7da05036fb6586f1b0c08c028ed286fae39` |
| kernix | 26 | `2f04bd00a0330713b0af484f864621cdb3b28e420abcf1b22d04e4753b9eb396` |

Source/test/example/README bytes match the release checkout, VCS records are clean,
and no private planning, web or benchmark files enter these packages. All three
extracted license/passport checks passed.

A separate registry-only consumer with a renamed Kernix dependency and default
features disabled ran on Rust 1.86. It checks identities/custom scalar legacy trait
imports, existing array-element literals, zero-copy Vec adoption, solve and QR.
Its actual no_std library also compiled for wasm32. Cargo metadata confirms all
three dependencies come from registry 0.6.0, with no workspace path substitution.

[Release-source CI](https://github.com/moderubias/kernix/actions/runs/39459504041)
completed successfully on Linux, Windows and macOS, MSRV 1.86, all three portable
targets and the separate benchmark-tooling job. These checks refer to the published
source commit; the subsequent verification-documentation commit does not alter
published crate bytes.
