# Kernix 0.12.0 verification — 2026-10-26

0.12 preserves existing public signatures and generic scalar bounds, speeds up
reusable multi-RHS solves and large thin QR, and adds owned Cholesky rank-one
updates/downdates. The [local report](../benchmarks/results/0.12.0-final/README.md)
and [complete archive](../benchmarks/results/README.md) include exact source values,
readable workload charts and separate version/library comparisons.

## Implementation and costs

- [Row-wise substitution](../crates/kernix-ops/src/linalg/triangular.rs) reuses
  contiguous RHS rows above four columns. Increasing-k scalar subtraction is
  retained for each element, without extra solve scratch. At most four columns
  retain the earlier scalar path. Existing LU, Cholesky and Qr solve APIs benefit.
- [Compact-WY QR](../crates/kernix-ops/src/linalg/blocked_qr.rs) groups 32 stable
  Householder reflectors on native-capable tall/square inputs with m>=192/n>=128.
  Cached upper-triangular T is reused during Q construction. Tiled copies avoid
  strided cache conflicts. The eventual Q output doubles as product scratch;
  extra panel workspace is O((m+n)*32), plus existing native packing/TLS costs.
  Unsupported/nonfinite products fall back before committing a block update.
  Thin shapes, scaled norms, subnormal handling and rank policies are preserved;
  blocked/FMA arithmetic can change final bits. Pivoted QR is unchanged.
- [Rank-one modifications](../crates/kernix-ops/src/linalg/cholesky_update.rs)
  construct a new factor of A+x*x^T or A-x*x^T in O(n²). They allocate n² output
  and n scratch, support signed-stride vectors and preserve both inputs on failure.
  Updates use scaled orthogonal rotations; downdates retain the representable
  diagonal gap and reject nonpositive pivots. Near-boundary conditioning and
  finite-dtype range remain limits. No decomposition VJP is added.
- Eligible positive-stride std/x86-64 panels use checked serial native GEMM;
  outside Windows, this extension is limited to smallest dimension 32 and a
  64³ work floor. Windows also admits larger general positive-stride views.
  Borrowed transposes/F-order/gaps need no materialized tensor copy, but native
  kernels can pack/cache scratch. Ordered/negative-stride/no_std paths remain.

Broader positive-stride dispatch was rejected after an initial Linux transpose128
f64 regression of about 5%. Final paired Linux transpose128f64 is ≈0.9962
[0.9935,1.0092], with no clear direction. Windows transpose128 f64/f32 takes about
17% less time in the final run. Dense128 default engine is unchanged; it remains
behind NumPy on some hosts. No universal lead or blanket zero-overhead claim follows.

## Measured effect

The table reports current / exact registry 0.11.1 latency with exploratory 95%
bootstrap intervals over nine fresh-process pairs. Below 1 means less time.
Displayed summaries use six significant digits; source JSON/exact tables retain
all digits. These intervals have no multiple-comparison adjustment.

| Machine | Cholesky 256, 64 RHS | LU 256, 64 RHS | Thin QR 1024 × 256 |
|---|---:|---:|---:|
| Local Intel i3-1215U | ≈ ≈ 0.360477 [≈ 0.35471, ≈ 0.380534] | ≈ ≈ 0.442876 [≈ 0.43983, ≈ 0.458018] | ≈ ≈ 0.499179 [≈ 0.48036, ≈ 0.507861] |
| Linux x86-64 | ≈ ≈ 0.2235 [≈ 0.219004, ≈ 0.228882] | ≈ ≈ 0.311462 [≈ 0.305192, ≈ 0.315533] | ≈ ≈ 0.552914 [≈ 0.544556, ≈ 0.556903] |
| Linux ARM64 | ≈ ≈ 0.337921 [≈ 0.337568, ≈ 0.338479] | ≈ ≈ 0.479135 [≈ 0.476241, ≈ 0.482465] | ≈ ≈ 0.678932 [≈ 0.678224, ≈ 0.68186] |
| Windows x86-64 | ≈ ≈ 0.187177 [≈ 0.186561, ≈ 0.188582] | ≈ ≈ 0.222329 [≈ 0.222121, ≈ 0.222893] | ≈ ≈ 0.578889 [≈ 0.577532, ≈ 0.582638] |
| macOS | ≈ ≈ 0.339146 [≈ 0.334744, ≈ 0.34213] | ≈ ≈ 0.339566 [≈ 0.299537, ≈ 0.352048] | ≈ ≈ 0.525156 [≈ 0.499582, ≈ 0.533394] |

Paired multi-RHS Cholesky gains are about 2.8–5.3× across these five hosts;
LU gains about 2.1–4.5×; large QR about 1.5–2.0×. Prepared-factor solves allocate
owned results with decomposition outside timing for every comparator.

The new rank-one methods are compared with nalgebra's prepared-factor update
(including its clone for owned output) and the separately labelled Kernix refactor
of an already formed modified matrix. Matrix formation is excluded, so this is a
conservative workflow reference. It is not another library's sparse/rank-update
implementation, nor an acceleration of Cholesky::factor itself. The local 256
update is 112.75 µs versus 529.381 µs for refactor and 21.713 µs for nalgebra: a workflow
improvement with a substantial remaining peer gap. Registry 0.11.1 has no such
Kernix API; its three missing observations remain unavailable, never fabricated.

All adverse observations remain visible. Examples with exploratory intervals
above 1 include local f32 batched multiplication (approximately +5.2%) and Cholesky 256
(+about 1.5%), Linux x86 backward (approximately +5.3%) and ARM scalar Cholesky solve
(approximately +2.3%). Several subpercent slowdowns also remain in the full tables. These
are retained observations, not dismissed because a source kernel is unchanged.
Windows dense128 intervals are particularly wide; no improvement is inferred.
Frequency, scheduling, unpinned Windows/macOS processes, code generation and shared
host load limit inference. Peer medians are point observations, not paired rankings.

[All four hardware jobs](https://github.com/moderubias/kernix/actions/runs/40270611933)
passed. Each current run has 152 observations across 46 workloads; registry baseline
has 149. Mac adds four explicit Accelerate observations to each. Twenty-four
version workloads are paired per host. Allocating ordinary operations and explicit
two-worker reusable-output operations remain separately labelled.

## Verification

- Local workspace: 571 default / 577 all-feature / 563 no-default tests including
  doctests; 29 release-mode native, factor and substitution regression groups.
- Independent reconstruction, orthogonality and scalar-coordinate oracles;
  known solutions, non-dyadic bitwise substitution, f32/f64/scaled/empty cases,
  reversed/gapped views, three panel orientations, output canaries and Tape
  derivatives. Custom scalar kernels that decline or return NaNs exercise
  compact-WY fallback without changing input data.
- Strict formatting, Clippy, rustdoc, Rust 1.86, 15 executable Markdown examples,
  four no_std workflows including real MSRV execution and wasm compilation.
  Ten normal/five allocator harness tests, 37 report tests and seven tool tests.
- [Source CI](https://github.com/moderubias/kernix/actions/runs/40270611006): all nine
  jobs pass, including actual Linux x86/ARM, Windows/macOS runtime tests and
  wasm/i686/ARM compile checks.
- [Assurance](https://github.com/moderubias/kernix/actions/runs/40270611037): API
  compatibility against v0.11.1 for all three crates with full/minimal features,
  AddressSanitizer, dependency audit and coverage all pass. Library line coverage
  is 8,224 / 9,555 = 86.070120%. Existing documented audit exceptions remain scoped to
  the benchmark-only zip dependency; no new runtime advisory exception is added.
- [Bounded fuzzing](https://github.com/moderubias/kernix/actions/runs/40227589286)
  passes on the pre-restriction source with the same numerical algorithms.
- [Full core/native Miri at the initial source](https://github.com/moderubias/kernix/actions/runs/40227258643) passed; its native step took over
  three hours with SIMD disabled. A separate bounded downstream f32/f64 wrapper
  fixture now checks addressing, invalid/empty extents, signed/zero strides,
  aliased read inputs and output canaries. Native execution and two consecutive
  cached Miri runs pass locally. The [bounded Miri CI](https://github.com/moderubias/kernix/actions/runs/40273798912)
  also passes, retaining the full core suite. The redundant final three-hour
  rerun was cancelled after this passed. Full large shapes remain unchanged in hardware
  and ASan CI; this interpreter check does not certify SIMD instructions.
- 83 retained reports, 312 protected data/source/driver/method files; all 257 prior
  protected files, 68 original commentaries and original archive commentary remain
  byte-identical. Exact medians/paired interval digits occur in every PDF; every
  offline HTML/GitHub chart link resolves. Firefox 320/768/1440, search, views,
  reset/focus, exact values and A4 browser print pass on current/legacy/Mac/paired
  reports and the 83-report catalogue. All three release ZIPs pass hash/link checks.

## Source and registry status

Clean measured and publication source is `90cbbff60e5d7458ef9f89918c99a6cca94b51d1`,
crates tree `841d489e565232cd518890780d866b92e80178dd`. Local and all four hardware studies measure exactly
those production, manifest, build-script and integration-test bytes. Subsequent
report/QA commits keep the entire crates tree unchanged. Registry baselines pin
all three packages to exact 0.11.1 and retain the generated driver/lockfile.

Publication in progress: kernix-core 0.12.0 was accepted and its downloaded archive
checksum/VCS match the clean source. The canonical registry index has not yet
propagated that dependency, so kernix-ops/kernix publication and actual-registry
consumer verification are pending. No completed publication is claimed here.

Sparse COO/CSR/CSC, sparse kernels, SVD/eigen/complex solvers and GPU execution
remain unimplemented. Sparse is a separate additive direction; it must preserve
dense APIs and avoid implicit densification. Existing neural/training components
remain compositions of linear algebra, not a claim of a complete model stack.
