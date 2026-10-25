# Kernix 0.11.1 verification — 2026-10-25

0.11.1 improves small dense x86-64 products and makes every retained workload
readable directly on GitHub. The [local report](../benchmarks/results/0.11.1-final/README.md)
and [complete archive](../benchmarks/results/README.md) retain exact source values.
[Release downloads](https://github.com/moderubias/kernix/releases/tag/v0.11.1)
provide the interactive HTML, vector PDF and original measurement evidence.

## Implementation and numerical contracts

The private [small GEMM kernels](../crates/kernix-ops/src/ops/small_gemm.rs) reuse
right-hand rows directly with four accumulator rows and two SIMD vectors per row.
Runtime AVX2 and FMA checks enable this path only for std/x86-64, rows 4–32,
contraction 4–64, and at most 16 columns in complete tiles of 8 (f64) or 16 (f32).
Remaining rows have explicit 1/2/3-row specializations; other columns use the
existing engine. Checked slice extents precede all pointer access; unaligned
inputs are supported. Contiguous broadcast batches use the same per-matrix path.

The native hook requires no heap scratch, including first use. Ordinary matmul
still allocates output and metadata. Fused rounding may change last bits, within
checked error budgets. Public signatures, scalar bounds, constructors, zero-size
semantics, generic scalars, ordered execution, signed-stride views, backend
boundaries, Rust 1.86, Edition 2024 and no_std + alloc remain supported.
The default 128/256 engines and Cholesky algorithm remain the existing ones.

## Measured effect

Each number below is current / exact registry 0.11.0 latency, with an exploratory
95% bootstrap interval over nine fresh-process pairs. Below 1 means less time.
The workload is [4,16,32] times broadcast [1,32,16]. Source tables preserve every
digit; this summary uses six significant digits and ≈ for rounding. No
multiple-comparison adjustment or universal hardware ranking is implied.

| Machine | f64 ratio [95% interval] | f32 ratio [95% interval] |
|---|---:|---:|
| Local Intel i3-1215U | ≈ 0.757491 [≈ 0.726359, ≈ 0.819638] | ≈ 0.709977 [≈ 0.695122, ≈ 0.772523] |
| Linux x86-64 | ≈ 0.65975 [≈ 0.658956, ≈ 0.666667] | ≈ 0.581121 [≈ 0.573386, ≈ 0.588552] |
| Windows x86-64 | ≈ 0.714286 [≈ 0.694444, ≈ 0.722222] | ≈ 0.666667 [≈ 0.666667, 0.75] |
| Linux ARM64 | 1 [≈ 0.997921, ≈ 1.00209] | ≈ 1.00669 [1, ≈ 1.00671] |
| macOS | 1 [≈ 0.958333, 1.076] | 1 [0.8832, 1.024] |

On the three tested x86-64 hosts, paired median time reductions are approximately
24–34% for f64 and 29–42% for f32. The new path does not apply to ARM/macOS.
In the Linux x86-64 warmed allocation probe, each batched operation falls from
7 to 3 Rust allocations. Peak live requested bytes fall from 16,392 to 8,200
for f64 and 8,200 to 4,104 for f32; these include output and are not process RSS.

The wider 32-column prototype was rejected after a 32×32 regression. The shipped
path excludes those columns. Final paired 32×32 intervals include 1 on all hosts.
Large serial128 remains an open performance target. Linux x86-64 f64 128×128
shows about 1% more time (ratio ≈1.00998, interval ≈[1.00530, 1.01141]) despite the
same large engine; this observation is retained. Its cause is not isolated by this experiment. Full reports show all sixteen paired workloads,
unclear directions, peer losses and explicit optional backends.

Each current/baseline run retains 119 library/workload observations across 35
workloads; macOS retains 123 including four explicit Accelerate observations.
[Actual four-platform workflow](https://github.com/moderubias/kernix/actions/runs/40161751367)
passed all four jobs. Ordinary operations include allocation; explicit parallel
rows reuse output and two workers with pool preparation excluded. Host load,
frequency, timer resolution and scheduling limit inference. Peer medians are
point observations, separate from paired version intervals.

## Verification

- Local workspace: 557 default, 563 all-feature and 549 no-default tests including
  doctests; 17 release-mode native/blocked tests. Six new regression groups cover
  3,360 dtype/shape combinations, all row tails, unaligned/padded slices, repeated
  output writes, invalid/overflow extents before writes, broadcast1/4/16/64,
  independent dyadic coordinate oracles, scaled non-dyadic error bounds and
  first-use scratch allocation counts.
- Strict formatting, Clippy, rustdoc, Rust 1.86, executable Markdown and no_std
  workflow checks pass. Actual Linux x86-64/ARM64, Windows and macOS tests, plus
  wasm/i686/ARM compilation, pass in all nine
  [source CI jobs](https://github.com/moderubias/kernix/actions/runs/40162484782).
- API compatibility against v0.11.0 passes all three packages with all/no-default
  features. AddressSanitizer, dependency audit and coverage pass in all four
  [assurance jobs](https://github.com/moderubias/kernix/actions/runs/40162484774).
  Measured library line coverage is 7,882/9,175
  (85.907357%), above 85%. Coverage is not a mathematical-domain proof.
- Nine normal and five allocation harness tests, 35 benchmark/report tests,
  seven tool tests and fourteen executable Markdown examples pass. Windows
  exposed a path-separator defect in report hashing; manifest keys now use POSIX
  separators on every platform, verified by actual Windows CI.
- Audit gates pass with the documented paste maintenance warning and three exact
  benchmark-only zip 8.6 extraction-advisory exceptions expiring 2026-11-22.

## Source and publication

All three published packages and signed tag v0.11.1 use clean `834d8fad54af5d322077941db2dabdb417328c26`,
crates tree `3d9e081a50d29e788b817971fe1c047ac6470e37`.
Local evidence measures clean `b2c458d113aa23aa24e93fbfdabd55b55e3e6831`, with
byte-identical crates. Hardware evidence measures clean
`450df128c1ffb55a9a861230edb6b9a5ac3ef5b4`: all production source, manifests and
build scripts match publication byte-for-byte. The sole crates-tree difference
adds m=6 to the integration-test fixture, covering the two-row remainder.
Baselines resolve exactly all three published 0.11.0 packages with the current
driver; source snapshots identify the driver revision and lockfile.

Actual registry archives match their clean VCS metadata, declared source files
and legal texts. No private research, planning, PDF or benchmark image enters the
packages. Both independent metadata graphs resolve exactly all three registry
0.11.1 packages, without local patches. The renamed no_std dependency executable
runs on Rust 1.86; six release-mode native/parallel/factorization tests on Rust
1.86 include small-kernel boundaries and reused output; a stable wasm consumer
library compiles. These registry executions run locally on x86-64; other hardware
is covered by matching production source CI, not a post-publication registry run.

| Package | Files | Actual archive SHA-256 |
|---|---:|---|
| kernix-core | 48 | `9bed0064a44aea225128982cd773d62e77d7fbe006f0324e70916383318f54a1` |
| kernix-ops | 69 | `f1d1e70ef19f0497bd8656180ad63e0e58a9a9a5ba745412516deb7222672395` |
| kernix | 27 | `1fff0d52a9a44670f6bd2a752e0835d6424c447db4c3d3e4560a1c5cc1852246` |

## Presentation and preserved evidence

All 68 retained reports have ranked, zero-based latency graphs with large direct
labels, plain-language captions and explicit units. Every workload is available
in expandable GitHub sections, with text summaries for narrow screens. Version
sheets identify both releases and show intervals around an unchanged-time line.
HTML provides search, filters, resources and exact tables; PDF preserves source
precision in its appendix. Color is reinforced with library names and text.

All 202 previous immutable measurement/source/lock/method files remain byte-identical.
New measurements are added separately. The integrity manifest hashes evidence and
original commentary; generated charts are explicitly presentation assets. Exact
values and every local report link are checked across the archive. Eight legacy
release ZIPs (0.8–0.11) also retain original data/source/lock/method bytes and paths,
with their presentation rebuilt and every offline link verified. Browser checks
cover actual 320/768/1440 viewports, filters, keyboard controls and printing.

Serial128 kernel families, Cholesky reusable native packing/register panels,
compact-WY QR and accelerator extensions remain future work. This patch does not
complete the broader numerical/backend roadmap or establish a lead in every task.
