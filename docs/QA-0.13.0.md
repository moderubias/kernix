# Kernix 0.13.0 verification

Additive rank-two sparse matrices preserve existing Tensor/shape/layout/storage
interfaces. `Matrix` selects Dense or canonical CSR by initialized payload bytes,
including usize indices; it does not promise the fastest representation at every
density. [API and cost contracts](SPARSE.md) explain finite arithmetic, signed zero,
explicit conversions, fill-in and unsupported sparse differentiation/factorization.

Release verification and immutable measured revisions are recorded below after
checks complete. Benchmark data and source archives retain their original digits;
sparse construction is measured separately from operations on prepared operands.

## Local checks

| Check | Result |
|---|---|
| Workspace default / all features / no default features, including rustdoc tests | 583 / 589 / 575 passed |
| Release sparse independent regressions | 11 passed |
| Benchmark harness ordinary / allocation probes | 11 / 6 passed |
| Public Markdown examples | 18 passed |
| Report/tool Python tests | 39 / 8 passed |
| Formatting, both Clippy targets with warnings denied, rustdoc warnings denied | Passed |
| Rust 1.86 all-feature compile; real no_std consumer | Passed; 5 downstream tests |
| Stable Wasm no_std consumer | Passed |
| Bounded Miri wrappers, including CSR ownership/heap/CSC borrow | 4 passed |
| Explicit-minor public API compatibility against signed v0.12.0 | All three crates passed, all features and no defaults |
| Executable sparse forward/graph example | Passed |

Tests cover all four storage products, independent scalar/dense oracles, random
rectangular patterns at multiple densities, f32/f64, negative-stride views,
fixed-rank/custom inputs, non-Clone buffer adoption, ordered COO duplicates, invalid
indices, finite/overflow errors, deterministic allocation-size failure, structural
zero cancellation, zero extents and a one-entry product with usize::MAX columns.
Changing custom compression capabilities return errors rather than growing an
unchecked buffer or indexing a shortened slice. The automatic memory policy is
tested on tiny f32 cases where indices make CSR larger than dense.
