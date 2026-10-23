# Kernix 0.9.0 verification — 2026-10-23

0.9.0 adds column-pivoted QR with explicit rank policy and minimum-norm solving for
rank-deficient/underdetermined systems. It also fixes Q orthogonality at subnormal
norms in ordinary and pivoted QR. Existing unpivoted solve contracts, array-valued
literals, Rust 1.86, no_std + alloc and pure Rust execution remain supported.
SVD, pseudoinverse, eigenproblems and optional vendor/GPU backends remain open.

## Source and publication

All three published crates and signed tag v0.9.0 use clean source
`9b7ef492c99bf2202e7eb3b07a79ef8f108991c7`. Measured source is
`4e40ea5a98e7f98505da7cc3dd8a2d80c1a4bd92`; their numerical crates tree is identical:
`211f096d49f0b068ea945a579456f018e8fedf37`. Subsequent report/QA changes do not
change the published numerical crates. Exact-registry baseline pins all three
packages to 0.8.0, with the same measurement-driver source and comparator lock.
The retained baseline predates the subnormal correction; final candidate and paired
repetitions were rerun after that correction. Superseded candidate runs were not promoted.

Registry archives were downloaded and checked byte-for-byte against source, legal
texts, clean VCS metadata and include boundaries. Research, PDF, local plans,
benchmarks and fuzz files are absent from the packages.

| Package | Files | Archive SHA-256 |
|---|---:|---|
| kernix-core | 48 | `2958b58162d1071b6aafc001a9bb7b71d4dad98d2e4036d785463258d10609af` |
| kernix-ops | 58 | `c034e0edf23de18a4a7bc001d12acfdbcd33dfdc053c25cd4867ebf5f6b93220` |
| kernix | 27 | `9450f2b39913e937dac2647110db4206d0eaab952d6d683b865e34da0091123b` |

[GitHub release and downloadable reports](https://github.com/moderubias/kernix/releases/tag/v0.9.0).

## Numerical contracts

The permutation maps factor columns to original columns: `A[:, permutation] = Q R`.
Thin Q/R reconstruct the original regardless of the selected numerical rank.
The leading diagonal cutoff is relative to the largest R diagonal, defaulting to
epsilon times max(m,n,1). Any finite nonnegative tolerance is accepted; zero only
drops exact-zero computed diagonals. This is not an SVD rank, strong RRQR, condition
estimate or optimal low-rank truncation. Solves use the explicitly rank-truncated matrix.

`solve_least_squares` obtains minimum Euclidean norm from a second scaled QR of
the leading R transpose, with O(n r²) extra work per deficient/wide solve. A separate
`solve_basic_least_squares` sets free permuted coefficients to zero. Neither forms
normal equations or an inverse. Full-column-rank solving reuses triangular factors.
Finite RHS data is borrowed; final permutation/scaling writes directly to output.

Both QR implementations now construct dimensionless Householder vectors from separate
scale/unit-norm components when epsilon times the physical norm underflows. A failing
smallest-subnormal regression previously produced Q norm squared about 0.8889; new f32/f64
tests check orthogonality directly. R and solves still have finite-dtype range/rounding
limits. Nonfinite required intermediates return errors; this is no arbitrary-range
or forward-error guarantee. Neither decomposition has an autodiff VJP.

## Verification

- Workspace: 542 default, 544 all-feature and 535 no-default tests, including Rust
  doctests; fourteen executable public Markdown examples. Strict formatting, Clippy
  and rustdoc pass. Thirteen release-mode decomposition regression tests pass.
- Nine pivoted-QR groups cover reconstruction, orthogonality, independent SVD oracles,
  analytic null spaces, stationarity, minimum versus basic solution norm, rank policies,
  f32/f64 scales, subnormal Q, layouts/ownership, malformed capabilities, huge empty
  axes and fallible resource rejection. A high-dynamic-range RHS regression prevents
  premature rescaling from discarding a representable solution component.
- Rust 1.86 no_std workflow executes three tests. Actual renamed registry-only consumer
  executes on 1.86; four independent std registry tests cover native f32/f64 coordinate
  oracles, conditioned SPD families and a rank-deficient f32 solution. Registry-only
  no_std consumer also compiles for wasm. Metadata verifies all three exact 0.9.0 packages
  without local patches.
- AddressSanitizer passes 26 decomposition/native-wrapper/diagnostic tests. Local bounded
  factorization fuzzing completes 102,254 executions in 41 seconds. This is counterexample
  search, not a proof.
- Inventory: 191 numerical entries and 28 Tape recording methods; decomposition solves
  explicitly have no VJP. API compatibility against v0.8.0 passes for all three crates
  with all features and without defaults.
- Development harness: eight normal / five allocator tests. Python: seven assurance-tool
  and fourteen benchmark/report tests, including actual CPU affinity, algorithm disclosures and working HTML links.
  Template placeholders use reserved delimiters so DETAILS.md remains a valid link;
  literal marker text in source metadata is preserved. Rendering changes no measurements.
  An actual headless Firefox session checks matmul/minimum/f32/backward filtering,
  a missing query, clearing the filter and all three footer links. The advertised
  matmul query now searches workload identifiers as well as human-readable labels.
- Fresh library line coverage: 7,556/8,811 = 85.76%, retaining the 85% floor. Stale instrumented
  objects initially duplicated old linalg mappings; `cargo llvm-cov clean` removed those
  mappings before this measurement. CI now cleans first. Coverage is not a mathematical
  domain, assembly, generic-instantiation or branch-coverage guarantee.
- Runtime advisory audit: zero vulnerabilities, visible paste maintenance warning. Benchmark-only
  Candle/zip retains the three exact extraction-advisory exceptions expiring 2026-11-22;
  runtime, unknown, other-version or expired findings fail. See [assurance policy](ASSURANCE.md).

## Local performance evidence

[100-case candidate](../benchmarks/results/0.9.0-final/README.md),
[exact 0.8.0 registry baseline, 88 cases](../benchmarks/results/0.8.0-registry-090/README.md),
and [nine paired process blocks](../benchmarks/results/0.9.0-paired-final/README.md)
retain individual/batch/first-use observations, accuracy checks, allocation probes,
source/manifest/lock snapshots, binary hashes and backend metadata. The old version
lacks the four new workloads; no substitute is presented as historical Kernix.

Median microseconds per allocating call on the measured Intel i3-1215U host:

| New workload | Kernix | nalgebra | faer | NumPy |
|---|---:|---:|---:|---:|
| Pivoted QR 96×32 | 56.637 | 78.306 | 71.206 | — |
| Pivoted QR 256×64 | 511.262 | 781.969 | 466.149 | — |
| Minimum norm 64×32, rank 16, four RHS | 49.903 | 71.614 | — | 63.224 |
| Minimum norm 32×64, rank 32, four RHS | 76.670 | 129.405 | — | 115.946 |

Pivoted QR materializes thin Q/R and an owned permutation. Minimum-norm timings include
factorization: Kernix uses pivoted/right QR (relative cutoff 1e-12), nalgebra native
SVD/solve (absolute cutoff 1e-10), NumPy native lstsq (rcond 1e-12). Algorithm costs
and truncation policies differ. Fixtures have a wide rank gap; independent null-space
orthogonality and normal-residual budgets are 2e-10. These are workload comparisons,
not a general SVD ranking or equivalence near a threshold. Large pivoted QR still trails faer.

| Existing workload | Paired current/baseline latency | 95% interval |
|---|---:|---:|
| Cholesky factorization 64 × 64 f64 | 0.977× | 0.924–1.040 |
| Cholesky factorization 128 × 128 f64 | 0.979× | 0.927–1.019 |
| Cholesky factorization 256 × 256 f64 | 1.007× | 0.993–1.026 |
| Thin QR 96 × 32 f64 | 1.049× | 0.966–1.059 |
| Thin QR 256 × 64 f64 | 1.020× | 0.995–1.046 |
| Matrix multiplication 128 × 128 · 128 × 128 f64 | 1.002× | 0.980–1.012 |
| Matrix multiplication 128 × 128 · 128 × 128 f32 | 0.987× | 0.933–1.014 |

Lower is faster. The exploratory paired bootstrap resamples fresh process pairs,
not calls, and does not correct for multiple comparisons. These intervals all include
one; they support no stable speedup of the existing kernels. Ordinary GEMM 128 f64 is
73.171 µs versus NumPy 92.307 µs in this run, while f32 is 36.757 versus 36.345 µs
and Candle 34.787 µs. The GEMM engine is unchanged; separate shared-machine observations
must not be mistaken for an optimization. Factorization and platform gaps remain open.

## Hosted verification and hardware

[Release-source CI](https://github.com/moderubias/kernix/actions/runs/39980049028)
and [tag CI](https://github.com/moderubias/kernix/actions/runs/39980331359) each pass
all nine jobs, including actual ARM64 execution, Windows/macOS, Rust 1.86 and
three portable compile targets. [Assurance](https://github.com/moderubias/kernix/actions/runs/39980049096)
passes API, dependency, ASan and coverage jobs; its coverage matches 7,556/8,811.
[Bounded fuzz execution](https://github.com/moderubias/kernix/actions/runs/39980088474)
passes 1,217,941 factorization and 2,053,213 layout executions over 181 seconds each.

[All four hardware jobs](https://github.com/moderubias/kernix/actions/runs/39980084433)
pass 100 workload/library pairs each. These are within-host observations, not a
ranking of hardware or paired version regressions. Windows/macOS have no pinned
affinity; Linux pins a CPU. macOS verifies Accelerate single-thread policy through
BLASSetThreading/BLASGetThreading on the same NumPy worker thread; the SDK controller
hash is retained. Other runners record OpenBLAS and requested one-thread settings.

Microseconds, Kernix / NumPy for GEMM and minimum-norm solving; Kernix / faer for QR:

| Host | GEMM 128 f64 | GEMM 128 f32 | Minimum norm 64×32 rank16 | Minimum norm 32×64 rank32 | Pivoted QR 256×64 | Report |
|---|---:|---:|---:|---:|---:|---|
| AMD EPYC 7763 64-Core Processor, Linux | 92.974 / 101.749 | 46.076 / 48.992 | 66.464 / 102.662 | 106.679 / 185.075 | 849.293 / 344.391 | [100 cases](../benchmarks/results/runner-ubuntu-latest-39980084433/README.md) |
| Neoverse-N2, Linux | 210.410 / 196.011 | 94.506 / 100.065 | 76.059 / 93.706 | 114.616 / 176.460 | 786.763 / 474.936 | [100 cases](../benchmarks/results/runner-ubuntu-24.04-arm-39980084433/README.md) |
| AMD EPYC 9V74 80-Core Processor, Windows | 80.300 / 102.100 | 40.500 / 46.900 | 90.800 / 85.600 | 153.400 / 159.000 | 752.700 / 320.200 | [100 cases](../benchmarks/results/runner-windows-latest-39980084433/README.md) |
| Apple M1 (Virtual), macOS | 102.541 / 17.667 | 52.459 / 6.125 | 51.250 / 81.000 | 95.917 / 136.125 | 605.417 / 309.791 | [100 cases](../benchmarks/results/runner-macos-latest-39980084433/README.md) |

Minimum-norm fixtures win most of these measured NumPy comparisons, with the
algorithm/cutoff differences described above. The Windows rank-16 case loses
(90.800 versus 85.600 µs); that observation is retained. Pivoted QR still trails faer
on the hosted runners, and Apple GEMM has a large gap even under verified threading.
No universal leadership claim is supported. Further work needs profiling, blocked
factorization kernels and an explicit optional vendor adapter with clear memory,
layout and threading contracts. Default pure Rust/no_std boundaries are preserved.
Hosted virtualization, frequency changes, migration and background load remain limits.

The Windows runner selected EPYC 9V74, whereas the 0.8 report used EPYC 7763.
Its faster GEMM observations do not establish a version speedup; the engine is unchanged.
