# Kernix 0.11.0 verification — 2026-10-24

0.11.0 adds blocked native Cholesky, explicit NEON selection on ARM and an optional,
explicit Apple Accelerate executor. Reports now distinguish version changes from
library comparisons. Existing constructors, generic elements, shape/layout validation,
Rust 1.86, Edition 2024 and no_std + alloc remain supported.

## Source and publication

Numerical source has crates tree `a5645528bd0f6226fa78b13b8ceac8296a9cced7`.
Local and hardware evidence measure clean `18750693908110110d85d02d0f11af8d58bfae8b`.
All baselines use exact published 0.10.0 packages and the same current driver.

All three packages and signed tag v0.11.0 use clean source
`18750693908110110d85d02d0f11af8d58bfae8b`. Actual registry archives match
declared source files, legal texts and clean VCS metadata byte-for-byte. Package
include boundaries contain only the declared source, examples, tests and legal/docs files.

| Package | Files | Archive SHA-256 |
|---|---:|---|
| kernix-core | 48 | `4a57e812eb731a6113ed952dc77d0dd7741baf8bafdb1a45eb2d9a034cc7e2a5` |
| kernix-ops | 67 | `d31c7ec21072c542fab73237b631fbe472ac0e0145c3c59abef347114012d52c` |
| kernix | 27 | `761ae900cee325659ebaad86d60b9a367d9c5f8700626ef4a8e83aa5f03abb07` |

[GitHub release and downloadable reports](https://github.com/moderubias/kernix/releases/tag/v0.11.0).

## Implemented contracts

- [Blocked Cholesky](../crates/kernix-ops/src/linalg/blocked_cholesky.rs) activates
  for native floating types at n ≥ 192. Sixteen-column left-looking panels use
  checked, borrowed strided matrix inputs for Schur updates and one reusable
  64×16 workspace. The checked input copy becomes the output; no second n² input
  or materialized transpose is introduced. Native kernels can allocate packing
  scratch. The generic fallback, validation ordering, finite-intermediate checks,
  symmetry budget and global pivot indices remain. Smaller paths are unchanged.
- [ARM NEON dispatch](../crates/kernix-ops/src/ops/neon.rs) directly selects the
  upstream NEON tables for eligible dense f32/f64 matrices outside macOS. Existing ARM SIMD
  was already present; this changes kernel selection. It avoids optional AMX
  selection through dependency feature unification and does not change global
  kernel thresholds. macOS retains its previous default matrixmultiply engine after
  fixed NEON showed a paired f32 GEMM256 regression; rejected candidate data is not
  promoted as final evidence. Explicit CPU pool execution admits native ARM chunks.
- [AccelerateExecutor](../crates/kernix-ops/src/accelerate.rs) requires the optional
  `accelerate` feature, macOS 15+ and an Apple SDK/C compiler. Its C bridge uses
  SDK declarations and versioned symbols. It accepts C-contiguous rank-two f32/f64
  inputs, allocates output, and rejects unsupported ranks/layouts without copying
  or silently falling back. Each BLAS call saves the caller policy, verifies
  single-thread mode, and restores the policy. Availability is fallible; empty
  outputs need no BLAS call. This is explicit vendor execution, separate from the
  default Rust CPU path; no batched/view/Tape integration or device transfer added.
- New independent A·B 128 fixtures supplement existing A·A cases with exact dyadic
  coordinate oracles. Cholesky 512 supplements 64/128/256. No accuracy budget was
  loosened. Macro semantics and public scalar bounds remain unchanged.

## Verification

- Workspace: 551 default, 557 all-feature, 544 no-default tests including doctests;
  macOS all-feature has one additional SDK thread-policy test. Twenty-seven
  release-mode blocked/diagnostics/native/pivoted tests pass. Strict formatting,
  Clippy and rustdoc pass. Fourteen executable Markdown examples pass.
- Three blocked-factorization regression groups check analytic δI+uuᵀ factors,
  independent reconstruction, n=127/128/191/192/193/255/256/257/383/512, condition
  numbers through 10¹², f32/f64 scales, F/reversed layouts, unchanged inputs,
  validation precedence and failures across panel boundaries. Native coordinate
  oracles, decomposition diagnostics and pivoted-QR regressions remain enabled.
- Actual macOS CI compiles the SDK bridge, checks numerical/layout/empty/error
  behavior and verifies restoration on caller and spawned threads. Non-macOS
  tests require explicit unavailability rather than a substitute implementation.
- Rust 1.86 workspace and AArch64 all-feature checks pass; three no_std workflow
  tests and wasm compilation pass. API compatibility against v0.10.0 passes all
  three packages with all/no-default features. Inventory remains 191 numerical
  entries and 28 Tape recording methods; no factorization VJP added.
- Forty local native/decomposition/parallel AddressSanitizer tests pass; CI also
  runs fifteen gradient checks, 55 total. Expanded factorization fuzzing reaches
  real 191–257 block boundaries, analytic scaled SPD factors and the existing
  small QR/Cholesky domain: local 13,294 executions/46 seconds; CI 42,779 factor
  and 3,725,956 layout executions/181 seconds each. Bounded fuzzing is no proof.
- Nine normal/five allocation harness tests, twenty benchmark/report Python and
  seven assurance-tool tests pass. Fresh library line coverage: 7,824/9,118 =
  85.808%, above the 85% floor; this does not measure mathematical-domain coverage.
- Runtime audit reports zero vulnerabilities and the visible paste maintenance
  warning. Three exact benchmark-only zip 8.6 extraction-advisory exceptions remain
  until 2026-11-22; runtime, unknown, other-version and expired findings fail.

Registry-only verification passes on Rust 1.86: a renamed no_std dependency
executable checks literals, compatibility, Vec ownership, diagnostics, minimum norm,
contractions and training; six release-mode native/parallel/factor/optional-feature
tests check exact coordinate oracles, reused output and analytic SPD through n=512.
A stable wasm32 consumer library also compiles. Both metadata graphs resolve
exactly all three 0.11.0 registry packages, without local patches. Actual macOS
SDK execution is covered by byte-identical source CI/hardware, rather than a
post-publication registry execution on a Mac.

## Human-readable evidence

[Local comparison](../benchmarks/results/0.11.0-final/README.md),
[exact registry baseline](../benchmarks/results/0.10.0-registry-011/README.md),
[nine paired process blocks](../benchmarks/results/0.11.0-paired-final/README.md).
Current and baseline each retain 116 workload/library observations across 34 workloads.
HTML has separate version/library views, paired intervals and interpretation, visible
losses, searchable labels/dtypes, synchronized charts/resources and keyboard controls.
PDF charts use at most six panels per landscape family page, nine pages in this suite.
Raw samples are immutable; presentation.json records renderer/input hashes separately
from measured source, locks and frozen executable hashes.

Actual Firefox BiDi viewports 320/768/1366 pass without page overflow. Six queries
return 15/2/7/3/0/34 rows, with matching charts/resource details; family filtering,
reset focus, mode controls and three footer links pass. Native A4 printing retains
methods and resource tables across 33 pages. Browser accessibility automation was
disabled after a Firefox IPC failure during the first print attempt; the rerun passes.
The seven-library macOS variant also passes the same viewport/filter/link/print
checks and keeps explicit Accelerate in its own column. Long PDF vendor labels
fit within reserved printable margins, separate from the source footer.

Ratios below are medians of nine fresh-process paired p50 latency ratios, current/baseline;
lower is faster. Brackets are exploratory 95% percentile bootstrap intervals across
process pairs, without multiple-comparison adjustment. They are not per-call tail
intervals. Host load/frequency and Windows/macOS scheduling remain uncontrolled.
Ordinary operations include output allocation; explicit pool rows use two workers and
reused output, excluding setup, with recorded Linux affinity.

| Local workload | Current / baseline [95% interval] |
|---|---:|
| Matrix multiplication · 128 × 128 · 128 × 128 · f64 | 1.003 [0.988, 1.023] |
| Matrix multiplication · 128 × 128 · 128 × 128 · f32 | 1.029 [0.953, 1.083] |
| Matrix multiplication · 256 × 256 · 256 × 256 · f64 | 1.011 [0.991, 1.029] |
| Matrix multiplication · 256 × 256 · 256 × 256 · f32 | 1.003 [0.968, 1.014] |
| Independent matrix operands · 128 × 128 · 128 × 128 · f64 | 1.005 [0.977, 1.031] |
| Independent matrix operands · 128 × 128 · 128 × 128 · f32 | 0.981 [0.963, 1.007] |
| Cholesky factorization · 128 × 128 · f64 | 1.033 [1.008, 1.043] |
| Cholesky factorization · 256 × 256 · f64 | 0.776 [0.759, 0.811] |
| Cholesky factorization · 512 × 512 · f64 | 0.598 [0.595, 0.612] |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f64 | 0.872 [0.554, 1.380] |
| Matmul · 2 workers · reused output · 256 × 256 · 256 × 256 · f64 | 1.038 [0.957, 1.368] |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f32 | 0.975 [0.588, 1.166] |

Default serial x86 GEMM128 retains its previous engine; no algorithmic speedup is claimed.
Current f64/f32 medians are 81.483/46.010 µs, versus NumPy 94.252/41.294. These are single-run peer observations, separate from paired version evidence.

| Local Cholesky | Kernix / NumPy / faer, µs | Old → new allocation calls | New peak requested bytes |
|---|---:|---:|---:|
| 256 f64 | 579.500 / 605.474 / 227.006 | 1 → 49 | 645,120 |
| 512 f64 | 4062.093 / 8994.431 / 2059.378 | 1 → 161 | 2,269,184 |

The bounded panel workspace does not bound whole-kernel memory: output, shape and native packing scratch are additional costs. Native packing increases allocation traffic; faer remains faster. The local report retains all uncertain directions and regressions, including the unchanged small-factor paths.

## Actual hardware

[Four hardware jobs](https://github.com/moderubias/kernix/actions/runs/40059566395)
measure current and exact registry baseline on the same host and interleave frozen
workers. Each records 116/116 observations and 12 paired workloads; macOS current has
four additional explicitly named Accelerate cells (120). Windows/macOS are unpinned;
Linux affinity and explicit two-worker CPU sets are recorded. Ordinary BLAS/Rayon policy
is one thread. Mac NumPy workers use SDK set/get verification; source and binary controller
hashes are retained. Different Mach-O UUID/install paths are provenance, not a different
thread policy; matching controller source and verified settings are required.

| Host | GEMM128 f64 ratio [95% interval] | Cholesky256 ratio [95% interval] | Cholesky512 ratio [95% interval] |
|---|---:|---:|---:|
| [AMD EPYC 7763 64-Core Processor / Linux](../benchmarks/results/runner-ubuntu-latest-40059566395/README.md) | 1.001 [0.995, 1.007] | 0.679 [0.673, 0.685] | 0.509 [0.506, 0.515] |
| [Neoverse-N2 / Linux](../benchmarks/results/runner-ubuntu-24.04-arm-40059566395/README.md) | 0.804 [0.796, 0.808] | 0.387 [0.385, 0.389] | 0.285 [0.285, 0.286] |
| [AMD EPYC 7763 64-Core Processor / Windows](../benchmarks/results/runner-windows-latest-40059566395/README.md) | 0.995 [0.985, 1.004] | 0.696 [0.686, 0.713] | 0.546 [0.545, 0.554] |
| [Apple M1 (Virtual) / macOS](../benchmarks/results/runner-macos-latest-40059566395/README.md) | 0.981 [0.964, 1.032] | 0.520 [0.510, 0.522] | 0.372 [0.368, 0.398] |

Observed ordinary allocating GEMM128 medians; µs per call. Peers have no process-paired confidence intervals in this suite.

| Host | Kernix / NumPy f64 | Kernix / NumPy f32 |
|---|---:|---:|
| AMD EPYC 7763 64-Core Processor / Linux | 93.986 / 102.031 | 45.836 / 49.011 |
| Neoverse-N2 / Linux | 167.786 / 194.280 | 85.557 / 99.772 |
| AMD EPYC 7763 64-Core Processor / Windows | 141.000 / 101.300 | 47.400 / 56.200 |
| Apple M1 (Virtual) / macOS | 111.833 / 17.291 | 48.875 / 5.250 |

ARM independent A·B and f32 controls:

- Matrix multiplication, 128 × 128 · 128 × 128, f32: 0.910 [0.908, 0.912].
- Matrix multiplication, 256 × 256 · 256 × 256, f32: 0.862 [0.860, 0.864].
- Independent matrix operands, 128 × 128 · 128 × 128, f64: 0.797 [0.785, 0.801].
- Independent matrix operands, 128 × 128 · 128 × 128, f32: 0.906 [0.901, 0.910].

Cholesky peer medians expose remaining gaps:

| Host | Cholesky512 Kernix / NumPy / faer, µs |
|---|---:|
| AMD EPYC 7763 64-Core Processor / Linux | 3886.384 / 6194.474 / 2584.958 |
| Neoverse-N2 / Linux | 4366.255 / 4027.441 / 2673.472 |
| AMD EPYC 7763 64-Core Processor / Windows | 4728.400 / 6494.200 / 3037.000 |
| Apple M1 (Virtual) / macOS | 3496.208 / 1888.375 / 1658.792 |

All per-workload intervals, slower rows and unknown peer measurements remain in the reports. Do not infer version effects from different hosts or runs.

Apple vendor execution is a separate optional backend; the default CPU path remains matrixmultiply.

| Apple GEMM | Default Kernix / explicit Accelerate / NumPy, µs |
|---|---:|
| 128 × 128 · 128 × 128 f64 | 111.833 / 16.167 / 17.291 |
| 128 × 128 · 128 × 128 f32 | 48.875 / 6.500 / 5.250 |
| 256 × 256 · 256 × 256 f64 | 753.208 / 101.958 / 115.875 |
| 256 × 256 · 256 × 256 f32 | 398.125 / 42.292 / 35.834 |

Both vendor paths use verified single-thread SDK policy. Kernix includes output allocation and per-call policy save/restore; NumPy configures policy before timing. Native vendor allocations are opaque. These observations establish neither default-backend acceleration nor a universal Rust/native ranking.

## CI receipts and remaining work

[Numerical source CI](https://github.com/moderubias/kernix/actions/runs/40059565899),
[assurance](https://github.com/moderubias/kernix/actions/runs/40059565925),
[expanded fuzz](https://github.com/moderubias/kernix/actions/runs/40015761709)
pass. The [signed-tag CI](https://github.com/moderubias/kernix/actions/runs/40060901689)
is tracked separately from the source checks; final documentation CI follows the
report commit. Numerical crate bytes are identical across presentation-only commits.

Full GEMMFIP integration, further x86 GEMM128 improvements, blocked LU/QR/CPQR,
SVD/eigen/complex/sparse/GPU and broader Apple/Tape integration remain open.
The library does not lead every workload. This release implements measured subsets
and preserves visible gaps instead of claiming the research program is complete.
