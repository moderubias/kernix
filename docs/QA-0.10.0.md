# Kernix 0.10.0 verification — 2026-10-23

0.10.0 optimizes existing minimum-norm, contraction, autodiff accumulation and explicit
parallel CPU paths. No public operation or scalar bound changes. Rust 1.86, Edition 2024,
no_std + alloc, fallible construction, existing literals and generic elements remain supported.
This is a measured subset of optimization work; full fused packing, blocked factorizations,
vendor and accelerator backends remain open.

## Source and publication

All three published packages and signed tag v0.10.0 use clean source `a47540c14faaef260207dd92d09c514c64528602`.
Local evidence measures `fcd9fdb97ff307e9c50d0526e8b4e1a580cb44a9`; hardware evidence
measures `1c53d4663a48de5a588809c2b69c3a6a5bdad7b2`. Their numerical crates tree is identical:
`bded13e32892c0dcae27c172b1c2ff4674284b45`. The exact registry 0.9.0 comparison uses
its actual published packages and the same current driver, with no substitute algorithms.

Actual registry archives were downloaded and matched byte-for-byte against source, legal
texts, clean VCS metadata and package include boundaries. Local research, plans, PDFs,
benchmarks and fuzz artifacts are absent from the crates.

| Package | Files | Archive SHA-256 |
|---|---:|---|
| kernix-core | 48 | `d2991e5fea88753e26f06c61eab9864ce2946fe235b6c3166cac770dacb777de` |
| kernix-ops | 60 | `0ecbd687c77857bec98f2e00b80b8af46243f9e2762eb0daa85c20430bc8ee1a` |
| kernix | 27 | `19740376870eaf6d739f8997b66a802c069dfb9561a5d23c9718685211a64acd` |

[GitHub release and downloadable reports](https://github.com/moderubias/kernix/releases/tag/v0.10.0).

## Numerical and ownership contracts

- [Private structured RZ](../crates/kernix-ops/src/linalg/rz.rs) replaces the additional dense
  right QR in deficient/wide minimum-norm solving. It exploits the leading upper-triangular
  block and stores implicit right reflectors, applying them directly into the final permuted
  output. It preserves the existing diagonal rank cutoff, rank-truncated matrix, minimum-norm
  semantics, finite-intermediate errors and late RHS scaling. No normal equations, inverse,
  explicit right orthogonal matrix, new rank policy or SVD claim. Extra factorization costs
  O((n-r)r²) time/O(nr) memory per call; full-column-rank and rank-zero paths retain their contracts.
- Adjacent C-contiguous suffix/prefix contractions borrow flat input slices and remove inner
  coordinate validation. Output allocation remains. Scalar bounds, per-product clone counts,
  increasing contraction order and free-axis output order are preserved. Permuted axes,
  arbitrary strides and general capabilities retain the indexed fallback. This is a coalesced
  subset, not a full TBLIS/GETT packed implementation or a new generic GEMM trait requirement.
- Gradient accumulation mutates only owned buffers private to one backward call. Retained
  Gradients, primals and future calls remain isolated, including an overflow failure after
  a finite branch. Custom Add semantics are preserved separately from AddAssign.
- Explicit parallel execution uses native-eligible multiples of 64 rows on x86 V3 for
  sufficiently large unordered native matmul. Small/skinny/ordered/portable paths keep their
  previous policy. The library imposes no affinity, implicit global threading change or device
  transfer. Worker-local native packing can retain scratch until pool/thread destruction.

## Verification

- Workspace: 548 default, 551 all-feature and 541 no-default tests, including doctests;
  fourteen executable Markdown examples. Strict formatting, Clippy and rustdoc pass.
  Eighteen release-mode pivoted/factorization/contraction tests pass.
- Eleven pivoted-QR groups include independent SVD/analytic oracles, normal residuals and
  null-space orthogonality, repeated solves, 1/4/64 RHS, narrow/wide tails, f32/f64 scaling,
  subnormal norms, rank/empty/resource/layout checks and late RHS scaling.
- New contraction tests include non-Copy custom elements with counted Clone calls, exact
  indexed equivalence, reordered axes, C/F/reversed/scalar/outer/empty paths and invalid capabilities.
  Parallel tests compare f32/f64 coordinate oracles at tails and native boundaries with 2/4 workers.
- Fifty-two actual AddressSanitizer checks pass across native kernels, decompositions,
  contractions, parallel execution and retained-gradient/overflow behavior. Local bounded
  factorization fuzzing completes 147,081 inputs in 46 seconds. CI completes 809,252 factorization
  and 4,091,866 layout executions in 181 seconds each; no proof of correctness is implied.
- Rust 1.86 workspace check and three no_std workflow tests pass; wasm consumer compiles.
  Registry-only renamed no_std executable, five std/native/parallel tests on 1.86 and registry
  wasm check pass. Metadata resolves exactly all three 0.10.0 registry packages
  with no local patches.
- API compatibility against v0.9.0 passes all three packages with all/no-default features.
  Inventory remains 191 numerical entries and 28 Tape recording methods. No factorization VJP added.
- Nine normal/five allocator harness tests, seven assurance-tool and fifteen benchmark/report
  Python tests pass. An actual Firefox session validates six queries (13 matmul, 2 minimum,
  6 f32, 3 backward, 0 missing, 31 unfiltered rows) and all three footer links.
- Fresh library line coverage is 7,679/8,952 = 85.78%, matching CI and preserving the 85% floor.
  This is not branch, assembly, generic-instantiation or mathematical-domain coverage.
- Runtime audit: zero vulnerabilities and visible paste maintenance warning. Benchmark-only
  zip 8.6 retains three exact extraction-advisory exceptions until 2026-11-22; unknown, expired,
  other-version and runtime findings fail. See [assurance policy](ASSURANCE.md).

## Local paired performance

[108-pair current report](../benchmarks/results/0.10.0-final/README.md),
[108-pair exact registry 0.9.0 baseline](../benchmarks/results/0.9.0-registry-010/README.md),
[nine paired process blocks](../benchmarks/results/0.10.0-paired-final/README.md).
Reports retain individual/batch/first-call samples, allocation probes, accuracy checks,
source/manifest/lock snapshots and executable hashes. Standalone HTML provides searchable
human-readable labels, library tables, expandable distributions and explicit thread/output policies.

Below are median paired fresh-process latency ratios, current/baseline; smaller is faster.
Intervals bootstrap process pairs, not calls; exploratory, without multiple-comparison adjustment.
Frequency and shared-host background load are uncontrolled. Timing includes output allocation
unless the row explicitly says reused output. Parallel rows configure two workers, Linux CPU mask
[0,2], and exclude pool/output construction; they compare versions, not single-thread competitors.

| Workload | Latency ratio | 95% interval |
|---|---:|---:|
| Minimum norm 64×32, rank 16 | 0.960 | [0.939, 1.005] |
| Minimum norm 32×64, rank 32 | 0.759 | [0.718, 0.769] |
| Adjacent-axis contraction | 0.059 | [0.058, 0.062] |
| Shared-variable backward | 0.969 | [0.908, 1.005] |
| Two workers, reused output, 64 f64 | 0.940 | [0.468, 1.421] |
| Two workers, reused output, 128 f64 | 0.774 | [0.769, 0.845] |
| Two workers, reused output, 256 f64 | 0.736 | [0.725, 0.761] |
| Two workers, reused output, 128 f32 | 0.769 | [0.438, 0.803] |
| Ordinary allocating GEMM 128 f64 | 1.017 | [0.982, 1.044] |
| Ordinary allocating GEMM 128 f32 | 0.994 | [0.958, 1.032] |
| Thin QR 96×32 | 1.013 | [0.981, 1.061] |
| Thin QR 256×64 | 1.004 | [0.970, 1.046] |
| Cholesky 128 | 1.000 | [0.983, 1.023] |

No stable timing win is claimed for small minimum norm, shared backward, 64 parallel, serial
GEMM128, ordinary QR or Cholesky. Their intervals include no change. The 128 f32 parallel
interval is especially broad; scheduling sensitivity remains visible. RZ wide peak requested
bytes fall from 102,144 to 59,136, allocations 28→20. Shared backward falls 10→8 allocations
and 66,152→49,768 peak requested bytes; these warmed Rust allocator probes exclude fixture
memory, first-use native retention and uninstrumented system allocators.

On the local host the contraction is about 17× faster than 0.9.0, but NumPy still wins
8.329 versus 39.133 µs. Shared backward also trails Candle (6.501 versus 4.158 µs). Ordinary
GEMM128 trails NumPy locally: f64 84.128 versus 79.292 µs, f32 40.614 versus 39.237 µs.
The serial GEMM engine was not optimized in this release. Full GEMMFIP-style packing and
blocked QR/CPQR/LU/Cholesky remain future work, not completed features.

## Actual hardware

All four [hardware jobs](https://github.com/moderubias/kernix/actions/runs/40000558153)
complete 108 workload/library pairs with unchanged accuracy budgets. These are separate
actual hosts, not compile-only claims. Their reports preserve actual CPU, affinity, native
backend and first-use/resource observations. Windows/macOS affinity is unpinned; Linux
uses recorded per-workload CPU sets. Ordinary BLAS/Rayon policy is one thread; explicit
Kernix pool rows use two workers. On macOS a benchmark-only SDK controller sets/gets
Accelerate single-thread mode in the same NumPy worker and records its hash.

Median µs per ordinary allocating call, Kernix / NumPy:

| Host | GEMM128 f64 | GEMM128 f32 | Wide minimum norm 32×64 |
|---|---:|---:|---:|
| [AMD EPYC 7763 / Linux](../benchmarks/results/runner-ubuntu-latest-40000558153/README.md) | 93.655 / 101.269 | 46.327 / 49.122 | 86.080 / 183.693 |
| [Neoverse N2 / Linux](../benchmarks/results/runner-ubuntu-24.04-arm-40000558153/README.md) | 208.887 / 195.222 | 94.356 / 99.836 | 96.523 / 176.404 |
| [Intel Xeon Platinum 8573C / Windows](../benchmarks/results/runner-windows-latest-40000558153/README.md) | 106.900 / 72.200 | 54.200 / 38.200 | 114.600 / 177.600 |
| [Apple M1 / macOS](../benchmarks/results/runner-macos-latest-40000558153/README.md) | 102.541 / 19.250 | 48.875 / 5.542 | 68.542 / 138.459 |

Kernix does not lead every row. Windows small rank-16 minimum norm also loses (102.0 versus
99.4 µs), and contraction loses to NumPy on all four hosts. Explicit parallel Windows128
f64 is slower than ordinary serial in this observation (114.9 versus 106.9 µs), with different
output-allocation boundaries. This is not a thread-scaling guarantee. Hosted CPU models differ
from earlier releases (Windows changed AMD→Intel); do not attribute cross-run changes to version.
Minimum norm compares QR/RZ against native SVD/lstsq with disclosed, different rank policies;
these fixtures have a wide rank gap and establish no general SVD performance claim.

## CI receipts

[Numerical source CI, nine jobs](https://github.com/moderubias/kernix/actions/runs/40000509551),
[assurance, four jobs](https://github.com/moderubias/kernix/actions/runs/40000509537),
[fuzz, two jobs](https://github.com/moderubias/kernix/actions/runs/40000562613),
[hardware, four jobs](https://github.com/moderubias/kernix/actions/runs/40000558153),
[measured source CI](https://github.com/moderubias/kernix/actions/runs/40000886179) and
[measured source assurance](https://github.com/moderubias/kernix/actions/runs/40000886135)
all pass. The release-source [CI](https://github.com/moderubias/kernix/actions/runs/40002303308)
and [assurance](https://github.com/moderubias/kernix/actions/runs/40002303273) also pass.
