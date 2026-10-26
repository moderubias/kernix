# Kernix 0.13.0 verification

Additive rank-two sparse matrices preserve existing Tensor/shape/layout/storage
interfaces. `Matrix` selects Dense or canonical CSR by initialized payload bytes,
including usize indices; it does not promise the fastest representation at every
density. [API and cost contracts](SPARSE.md) explain finite arithmetic, signed zero,
explicit conversions, fill-in and unsupported sparse differentiation/factorization.

Measured/published source is `7c11b2165b0cb7b523db4d938a36f1396396a138`,
crates tree `adf1ed7cd7aefbb4d86894c3810523f6a5aff857`, signed tag `v0.13.0`.
Subsequent presentation/QA changes preserve that entire crates tree.
Benchmark data and source archives retain their original digits;
sparse construction is measured separately from operations on prepared operands.

## Local checks

| Check | Result |
|---|---|
| Workspace default / all features / no default features, including rustdoc tests | 583 / 589 / 575 passed |
| Release sparse independent regressions | 11 passed |
| Benchmark harness ordinary / allocation probes | 11 / 6 passed |
| Public Markdown examples | 18 passed |
| Report/tool Python tests | 40 / 8 passed |
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

## Comparative evidence

The [local report](../benchmarks/results/0.13.0-final/README.md),
[paired dense guards](../benchmarks/results/0.13.0-paired-final/README.md) and
[complete archive](../benchmarks/results/README.md) retain every raw sample, first
call, allocation probe, driver/lock, exact source archive and paired observation.
Four additional hosts run the same frozen source. Previous-version drivers pin all
three dependencies to exact signed Git 0.12 source 90cbbff, whose crate payloads were
independently verified. The driver/source archive revision remains 7c11b21; library
package metadata separately records the actual Git baseline SHA. Missing historical
Matrix observations are omitted, without substitute algorithms.

Local medians below are microseconds, for **cyclic five-diagonal** matrices and
prepared operands; construction is separate. These are point observations from one
host, not process-level confidence intervals for peer rankings.

| Operation | Kernix Matrix | Kernix dense | SciPy CSR | faer CSR | NumPy dense |
|---|---:|---:|---:|---:|---:|
| 2048×2048 matrix-vector, f64 | 11.708 | 3233.102 | 7.04 | 4.938 | 421.739 |
| 512×512 × 512×32, f64 | 24.034 | 421.951 | 22.263 | 42.111 | 428.464 |
| 512×512 × 512×32, f32 | 16.813 | 212.719 | 15.043 | 38.575 | 169.153 |
| 256×256 sparse product, f64 | 86.747 | 573.091 | 42.096 | Not measured | 635.812 |
| 512×512 construction, f64 | 534.67 | 116.698 | 941.507 | Not measured | 81.466 |

The local kernel improvements versus Kernix dense range approximately 6.6–276×.
SciPy/faer still beat Matrix on several sparse kernels. Compression and dense copy
produce different physical outputs; construction is a memory/time trade-off, not
an equivalent-algorithm ranking. Matrix's finite checks also differ from peer IEEE
policies. The benchmark records those costs, actual index widths, thread policy,
nonzero pattern and allocation-inclusive returned outputs. These sparse wins do
not establish a dense128 improvement or a universal density threshold.

For the 2048×2048 f64 fixture, initialized input payload is 33554432 dense bytes
versus 180232 CSR bytes, including usize indices/offsets. The 512×512 f64 fixture is
2097152 versus 45064 bytes; f32 is 1048576 versus 34824. This excludes capacity,
metadata, allocator overhead and construction peak/RSS. SciPy uses narrower indices
in these fixtures and can consume fewer sparse payload bytes than Kernix.
The new reports show a separate memory table; both peer wins and losses remain
visible. Local paired QR 512×128 median is about 1.015× baseline with exploratory
95% interval [1.004,1.034]; it is retained despite unchanged dense numerical source.

## Publication and downstream verification

All three packages were built and verified from clean source 7c11b21 by ordinary
`cargo publish --workspace --dry-run --locked`, then published successfully using
`cargo publish --workspace --locked`. Actual registry downloads contain 48 core,
80 ops and 28 facade files. Every packaged source/legal file, clean VCS identity,
normalized registry dependency and cross-crate lock checksum matches the archive.
No private research, audit PDF, benchmark artifacts or scratch files are packaged.

| Package | Actual SHA-256 |
|---|---|
| kernix-core | `0801a859e7a621a77c4aac5d551fc5b5ad00787ee9a278ebd21c39cf3d3a1de4` |
| kernix-ops | `cf04524122ea2b682fb571e39006d0c2f5c61de41ca5cc8ae806a704ae3343c2` |
| kernix | `71783d2ed95fd29490c17dd2b17f55306cb0cfd8270b0905d2c431738d5092db` |

An independent ordinary registry consumer resolves all three libraries to exactly
0.13.0, with no Git/path patch. Rust 1.86 executes five renamed-dependency no_std
workflow tests plus an executable; another registry consumer runs 31 release
native/QR/solve/rank-modification/sparse tests. Stable Wasm no_std checks pass.
A separate signed-Git consumer repeats those checks and confirms all three nodes
resolve exactly 7c11b21. Both dependency graphs retain their intended feature sets.
The previously delayed 0.12 registry consumers now also pass four MSRV no_std tests,
the executable and a stable Wasm check; its rollout is fully closed.

The [source CI](https://github.com/moderubias/kernix/actions/runs/40289679770),
[tag CI](https://github.com/moderubias/kernix/actions/runs/40291628714),
[API/security assurance](https://github.com/moderubias/kernix/actions/runs/40289679776),
[full core plus bounded-wrapper Miri](https://github.com/moderubias/kernix/actions/runs/40289726931)
and [all four hardware studies](https://github.com/moderubias/kernix/actions/runs/40289723446)
pass. Sparse has no unsafe implementation. Real hardware executes the native dense
kernels; Miri checks bounds/ownership with SIMD dispatch disabled.

## Report verification

The archive contains 98 reports (75 raw runs, 23 paired runs) and 367 protected
measurement/source/driver/method files. All 312 prior protected files, 83 original
commentaries and the original catalogue commentary remain byte-identical.
Every exact median/paired interval is present in the PDFs, and offline HTML and
GitHub PNG links resolve. New rows include readable payload/index-width tables
and a construction-trade-off caption. Firefox 320/768/1440 checks pass on current,
legacy, Mac and paired views, including the memory table/resources tab, filters,
reset/focus, the 98-report catalogue and A4 print.
