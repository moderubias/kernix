# Kernix 0.8.0 verification — 2026-10-22

0.8.0 adds norm-1 conditioning and working-precision LU refinement, reduces
factorization workspace/copies and fixes huge empty QR execution. It preserves
array-valued literals, fallibility, generic scalar contracts, Rust 1.86 and
no_std + alloc. SVD, pivoted QR, sparse/complex solvers and GPU execution remain
unimplemented. Faster factorization remains an open objective.

## Source and publication

All three crates and signed tag v0.8.0 use clean source
`227f339974f9c31fcf6bd37ccccbda3eedc99591`. The measured numerical source is
`18b91c6`; the exact-registry driver was resolved at `2d62feb`. Their numerical
crates tree is identical: `fafab942938f451dcfd2ed27307be24af16ff21c`.
Later commits improve development assurance/presentation without changing crates.
The registry baseline pins all three packages to 0.7.3 (published VCS ff2874f),
retaining the same peer lockfile and measurement driver source.

Downloaded registry archives were checked against source bytes, legal texts,
clean VCS metadata and package include boundaries. No research, PDF, planning,
benchmark or fuzz files entered the packages. Root conventional license texts
copy the existing MIT/Apache terms; licensing is unchanged.

| Package | Files | Archive SHA-256 |
|---|---:|---|
| kernix-core | 48 | `6e3cede5df20f86372b6c369fdf79840ecb37598f55ce5ee58df89c7cf2b9567` |
| kernix-ops | 56 | `b5c4e80e84758342f08ae6ce5e1f4ee16075eec6e11227ac2276d58c6b5f5b7e` |
| kernix | 27 | `932bb552d74f23a4427d022ecd6e2bdf720508b1080c4144e24831dc384519e3` |

[GitHub release and downloadable reports](https://github.com/moderubias/kernix/releases/tag/v0.8.0).

## Verification

- Workspace tests: 532 default, 534 all features, 525 without default features,
  including Rust doctests. Thirteen public Markdown examples also pass downstream.
- Strict formatting, Clippy, rustdoc; MSRV 1.86 default/no-default compilation;
  stable wasm consumer; renamed registry-only no_std consumer executes on 1.86.
  Both registry consumers resolve all three exact 0.8.0 packages without path patches.
- Three independent std registry tests on Rust 1.86: f32/f64 native coordinate
  oracles and Cholesky SPD families at n=64/96/97/128 and condition 1e4–1e12.
  The latter checks reconstruction and an analytic Sherman–Morrison solution.
- Seventeen AddressSanitizer tests cover checked native kernels, decomposition
  regressions and diagnostics. New tests cover scales, layouts, exact ordered
  small-Cholesky arithmetic, empty huge axes, malformed contiguous capabilities,
  condition families, refinement budgets/stagnation and improving-step acceptance.
- Inventory: 182 numerical entries, 28 Tape recording methods. New decomposition
  diagnostics/refinement have no VJP; inspection/move methods are classified explicitly.
- Development harness: 6 normal / 5 allocator tests. Python: 7 assurance-tool tests,
  12 benchmark/report tests after backend-policy and readable-hardware checks.
- API compatibility against v0.7.3 passes for all three crates with all features
  and without defaults. JSON retains commands/status/full diagnostics rather than
  pretending the tool provides a stable structured finding schema.
- Initial library line coverage: 7,263/8,501 = 85.44%; CI floor 85%, pinned
  Rust 1.99.0 / cargo-llvm-cov 0.9.1. This does not raise MSRV or establish branch,
  assembly, generic-instantiation or mathematical-domain coverage.

[Release-source CI](https://github.com/moderubias/kernix/actions/runs/39719270795)
passes all nine jobs, including Linux ARM64 execution, Windows/macOS, MSRV and
three compile targets. [Tag CI](https://github.com/moderubias/kernix/actions/runs/39719682984)
also passes. [API/advisory/ASan/coverage assurance](https://github.com/moderubias/kernix/actions/runs/39720001660)
passes all four jobs. [Scheduled-style fuzz execution](https://github.com/moderubias/kernix/actions/runs/39719596511)
passes: 2,594,783 factorization and 2,251,016 layout runs over 181 seconds each.
The initial prebuilt cargo-fuzz selected musl; CI now explicitly targets GNU.
Fuzzing remains bounded counterexample search, not a numerical proof.

Runtime dependency audit finds zero vulnerabilities. Both trees retain the visible
paste maintenance warning. Benchmark-only Candle/zip has three narrowly reviewed
extraction advisories with an exact-version exception expiring 2026-11-22; unknown,
runtime, other-version and expired findings fail. See [assurance policy](ASSURANCE.md).

## Local before/after evidence

[88-case candidate](../benchmarks/results/0.8.0-clean-18b91c6/README.md),
[exact 0.7.3 baseline](../benchmarks/results/0.7.3-registry-fix080/README.md) and
[nine paired process repetitions](../benchmarks/results/0.8.0-paired-final/README.md)
retain raw observations, source snapshots, actual manifests/locks and binary hashes.
Each ordinary run shuffles workers, warms at least eight calls and 10 ms, and
separates per-call p50/p95/p99 from batch means. The paired bootstrap resamples
process pairs, with exploratory intervals and no multiple-comparison correction.

| Workload | Paired current/baseline latency | 95% interval |
|---|---:|---:|
| Cholesky 64 | 0.868× | 0.827–0.941 |
| Cholesky 128 | 0.674× | 0.659–0.683 |
| Cholesky 256 | 0.493× | 0.481–0.503 |
| QR 96×32 | 1.043× | 1.036–1.084 |
| QR 256×64 | 0.958× | 0.937–0.988 |
| GEMM 128 f64 | 1.015× | 1.006–1.032 |
| GEMM 128 f32 | 0.994× | 0.925–1.003 |

Lower is faster. Small QR trades about 4% latency for less copying/storage; its
peak requested bytes fall from 106,752 to 82,176. Large QR falls from 557,568 to
426,496. Cholesky peak bytes at 64 fall from 65,536 to 33,280; at 256 from
1,048,576 to 524,288. Allocation probes exclude retained first-use native scratch.
GEMM's engine is unchanged; its observed f64 slowdown is retained, not attributed
to a kernel optimization. The ordinary local GEMM 128 f64 run is 75.270 µs versus
NumPy/OpenBLAS 74.650 µs. Factorizations still trail the fastest comparators.

## Additional hardware

The [four-runner workflow](https://github.com/moderubias/kernix/actions/runs/39719312815)
executed real Linux x86-64/ARM64, Windows and macOS workers. The initial macOS run
used Accelerate without explicit thread verification and is not a controlled
single-thread comparison. Its original observations remain unchanged; the
[corrected macOS run](https://github.com/moderubias/kernix/actions/runs/39720416210)
requires BLASSetThreading/BLASGetThreading on each NumPy worker thread. The controller
hash and verified policy are retained. Affinity is unavailable on macOS/Windows.

Median microseconds per allocating GEMM call, Kernix / NumPy; these are separate
within-host comparisons, not a ranking of hardware or statistically paired runs:

| Host | NumPy backend | 128 f64 | 128 f32 | Report |
|---|---|---:|---:|---|
| AMD EPYC 7763, Linux VM | OpenBLAS | 93.687 / 100.990 | 46.287 / 48.983 | [88 cases](../benchmarks/results/runner-ubuntu-latest-39719312815/README.md) |
| Neoverse N2, Linux VM | OpenBLAS | 225.569 / 194.635 | 97.833 / 99.833 | [88 cases](../benchmarks/results/runner-ubuntu-24.04-arm-39719312815/README.md) |
| AMD EPYC 7763, Windows VM | OpenBLAS | 137.700 / 100.800 | 76.000 / 56.200 | [88 cases](../benchmarks/results/runner-windows-latest-39719312815/README.md) |
| Apple M1 virtual, macOS | Accelerate, verified single-thread policy | 103.166 / 16.125 | 53.917 / 6.083 | [88 cases](../benchmarks/results/runner-macos-latest-39720416210/README.md) |

These results expose substantial Apple and Windows gaps; no claim of universal
leadership is supported. Further work needs actual backend/dispatch/kernel
profiling, explicit optional vendor adapters and stronger decomposition kernels.
Hosted virtualization, migration, frequency and background load remain limitations.
