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

Remote CI and actual publication/registry-consumer checks are recorded after
their respective release gates finish.
