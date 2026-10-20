# Kernix 0.5.0 verification

Scope: additive tensor literals/construction/prelude and faster materialized
Householder QR. Existing shape/layout/storage/device capabilities remain available.
No unsafe code or runtime dependency was added.

## Local checks

Linux x86_64, rustc 1.99.0:

- Default workspace tests: 479 passed, including doctests and ragged-row compile failure.
- All-feature workspace tests: 481 passed, including portable math and parallel execution.
- No-default-feature workspace tests: 472 passed, including the new macro/QR regressions.
- Public README/user-guide downstream examples: 9 passed.
- Formatting, strict all-target/all-feature Clippy and strict rustdoc passed.
- Rust 1.86 workspace compilation passed. A separate renamed-dependency no_std
  consumer uses every macro without importing alloc and compiles on Rust 1.86.
- Stable no-default-feature compilation passed for wasm32-unknown-unknown,
  i686-unknown-linux-gnu and aarch64-unknown-linux-gnu. The separate macro consumer
  also compiles for wasm32. These are compilation checks, not runtime validation.
- Publication file lists contain 46 core, 51 ops and 24 facade files; private
  development files are absent. All packages retain explicit include allowlists.

QR checks cover zero, tall, square and wide matrices, four-lane tails, C/F orders
and transpose views, f32/f64 reconstruction/orthogonality, magnitudes 1e-200 to
1e200, finite results near f64::MAX and nonfinite rejection. Property tests compare
full-column-rank least-squares solutions with independent nalgebra Householder QR.
A targeted regression exercises ordered retry when regrouped dot products overflow.
Existing decomposition tolerances were not relaxed. Different reduction grouping
can change floating last bits; no bitwise compatibility is promised for QR.

Construction checks cover ownership without Clone/Copy, one-time evaluation,
Vec allocation adoption, scalars, empty/high-rank shapes, invalid extents, count
mismatches, zero-sized elements and qualified macro use from downstream code.

## Measurements

The retained [0.5.0 benchmark report](../benchmarks/results/0.5.0/README.md)
includes raw batches, dependencies, source archive/hash and printable PDF/charts.
It measures four QR sizes against nalgebra and NumPy, and dynamic owned tensor
construction against ndarray and NumPy, alongside the existing workloads.
Construction includes allocation and metadata validation. QR includes materialized
thin Q and R. Single pinned CPU, one BLAS thread, allocation-inclusive except
explicit reuse cases. Batch p95 is not a per-request latency percentile.

## Release

Publication and remote verification are recorded after the release gate completes.
