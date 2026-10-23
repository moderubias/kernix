# Reproducing CPU evidence

Use the standalone development harness; it does not add dependencies to Kernix or
its published crates. Its lockfile pins ndarray, nalgebra, faer and Candle. The
harness uses stable Rust separately from the library's MSRV/no_std checks. The
Candle 0.11 framework baseline may compile native development dependencies and
need a host C/C++ toolchain. None becomes a Kernix runtime requirement.

```sh
python benchmarks/evidence.py unique-run --require-openblas
python benchmarks/evidence.py unique-baseline --registry-baseline 0.5.0 --require-openblas
```

The selected Python needs NumPy and Matplotlib. PNG/PDF charts and the standalone HTML report are rendered locally. `--require-openblas` rejects a different NumPy BLAS build rather
than silently substituting it. Use an environment with an optimized NumPy wheel
or a documented vendor build. The NumPy build supplies its recorded BLAS backend; macOS Accelerate is supported
with verified same-worker-thread control. GPU baselines are not provided. Candle CPU supplies the equivalent automatic-differentiation case;
Burn and other frameworks remain future comparator work.

Commit source and lockfile changes first. The runner refuses tracked or untracked
working-tree changes, checks the revision again afterward, and archives committed
bytes. It records source/tree IDs, archive and executable SHA-256, dependency tree,
actual driver manifest/lockfile, compiler versions/flags, CPU/RAM/governor, thread
limits and NumPy backend configuration. Results are created at a unique local
ignored path; existing runs are never overwritten. Copy a completed official run
into `benchmarks/results/<run>/` only after measurement. That later report commit
is distinct from the measured source commit, which is named in its metadata.

The registry-baseline mode pins all three Kernix packages to the requested exact
version. It retains the generated driver and resolved lockfile, preventing a newer
compatible transitive Kernix package from changing the baseline. Workspace mode
measures the numerical code in the named clean source commit.

## Timing boundaries

Operation inputs are C-order with explicitly named f32/f64 dtype in every comparator; constructor cases
explicitly use f32 and allocate 64x64 owned values. Pre-0.6 registry baselines use
full(value), recorded as the legacy constructor implementation, since zeros/ones
did not exist. Outputs retain their library's
native layout. GEMM times allocating APIs. QR materializes owned thin Q and R in
all implementations; Cholesky times reusable factors (NumPy returns L). Kernix's
finite/symmetry checks remain enabled; the other libraries' contracts may differ.
The ndarray/Candle softmax comparisons are explicit stable compositions.

The autodiff case records `sum(A @ W)`, treats A as constant, and returns W's gradient.
Kernix and Candle both copy input/weight data and copy the returned gradient. Both
construct and destroy their own real reverse-mode graph. This compares equivalent
mathematical work, while graph representation and validation remain library choices.
It is separate from the historical manual-derivative SGD lower bound.

Each worker records its first operation call with fixture/framework loading excluded,
checks its result against a scalar/analytic/reconstruction oracle, warms at least eight times
and for at least 10 ms (0.8 and later; earlier runs used eight calls),
and saves 256 individually timed calls plus 15 separately timed 10-ms batch means.
Each distribution has its own nearest-rank p50/p95/max and raw observations in order.
Three extra processes provide first-use observations. They do not flush CPU caches
or measure application startup. Per-call warmed timings include output destruction;
the first-call return measurement excludes destruction. Timer-pair overhead is
retained without subtraction. This matters for tiny operations.

Allocation requests are measured after warmup in a separately compiled binary
with `allocation-counting`. Timed workers use the ordinary allocator and reject
instrumented timing. Rust allocator requests exclude allocator internals, stack
memory and native allocations outside that allocator. NumPy allocator counts are
unavailable, never reported as zero. Peak requested bytes include output and
transient allocations; they are not scratch-only bytes. Process peak RSS includes
fixtures, validation, runtime and outputs; Linux uses VmHWM, other systems may
report unavailable. RSS and requested bytes answer different questions.

Pinned affinity and one-thread settings do not control shared-machine load,
frequency, thermals or cache state. Kernel-only latency and complete application
latency are not measured. A call-level p95 does not establish a model/frame p95.
Accuracy budgets and observed errors are saved independently of speed. No tolerance
is relaxed to produce a faster score. Historical reports retain their original
batch statistics and must not be reinterpreted as individual-call distributions.

## Checks

```sh
python -m unittest discover -s benchmarks -p 'test_*.py'
python -m unittest discover -s tools -p 'test_*.py'
python tools/check_operation_support.py
cargo test --manifest-path benchmarks/harness/Cargo.toml --locked
cargo test --manifest-path benchmarks/harness/Cargo.toml --locked --features allocation-counting -- --test-threads=1
```

The inventory checker validates the project's explicit export conventions and
known declaration macros; it is not a general Rust parser or a gradient proof.
Changes in those conventions require updating the checker and reviewing its table.


## Tensor training stages (0.7)

The normalized batched fixture uses input `[2,4,8]`, weight `[8,4]`, bias `[4]`,
last-axis population layer norm, tanh, a step-two last-axis slice selecting two
features and MSE. Candle uses index_select for the equivalent slice, with indices
prepared outside timing. A separate scalar implementation
validates losses, with f64 central differences for weight/bias derivatives. Candle
uses a real graph and explicit population-statistics normalization composition.
Independent harness tests also compare vector/broadcast matmul and affine norm VJPs.

- `training_forward`: inference kernels/loss, without a graph.
- `training_record`: fresh parameter/input/target snapshots plus eager graph and forward/loss.
- `training_backward`: fresh derivatives of a graph prepared outside timing.
- `training_sgd`: transactional Kernix SGD with fixed real precomputed gradients;
  repeated calls update the parameters but are not a converging full training loop.

Allocation counts/peak requested bytes belong to each stage; recording includes
retained graph values and transient kernel scratch, not isolated container bytes.
No subtraction of noisy timing medians is used to assert a graph-only cost. Candle
and Kernix validation/normalization rounding differ. Registry baselines before0.7
omit these unsupported Tape stages and record why, rather than replace them with
manual derivatives. The new broadcast-batched GEMM case is available in0.6 too,
so it remains a direct baseline. CPU affinity/thread/dtype controls are unchanged.


## Reading and comparing reports

Each run includes a compact Markdown table, a standalone `report.html` with workload
filtering and expandable distributions/resources, and colored per-workload PNG/PDF
panels. Times are per-call medians in microseconds; relative chart values are library
latency divided by Kernix latency. Below one means that comparator was faster. Every
workload has its own scale. Missing measurements are shown as unavailable, never zero.

Pair a retained current run with its separately measured exact registry baseline:

```sh
python benchmarks/report.py web/benchmarks/current --baseline web/benchmarks/baseline
```

This rewrites presentation files only. It does not modify raw observations or claim
that rendering remeasures code. Historical official artifacts are retained unchanged.
The comparison names both runs, displays signed latency changes (negative is faster)
and flags CPU/toolchain/governor/thread/backend mismatches. Sequential, unrandomized
runs on a shared laptop do not establish statistical significance. Full p95/batch/RSS/
first-use/accuracy/source/lock evidence remains in `DETAILS.md` and `raw.json`.

Dense f64 GEMM in 0.7.2 may cache thread-local packing memory. Warmed Rust allocation
probes therefore exclude its initial slab allocation and retention; they do not
establish allocation-free first use or scratch-free execution. Engine selection is
specified in NUMERICS.md. NumPy uses its recorded optimized BLAS, not a reference build.


The 0.7.3 suite adds dense f32 GEMM at 64/128/256. Rust and NumPy fixtures use the
same f32 values, with no timed conversion. Validation accumulates the actual f32
inputs in f64 outside timing and uses relative-plus-absolute budget 1e-5 for all
comparators. The existing f64 budget remains 1e-11. First-use, timing and resource
boundaries are unchanged; both exact previous-version and candidate runs include
all 80 workload/library pairs.

## Process order and uncertainty (0.8)

New runs shuffle all workload/library workers with `--seed` (default 20261022),
retaining the exact order. Every call/batch distribution also stores nearest-rank
p99. Two additional Cholesky sizes, 128 and 256, bring the suite to 88 pairs.
The exact-registry driver preserves the peer lockfile and updates only Kernix;
this also preserves reproducibility when an upstream comparator dependency is yanked.

After both runs complete, interleave their frozen, hash-verified executables:

```sh
python benchmarks/compare.py web/benchmarks/current web/benchmarks/baseline web/benchmarks/paired --rounds 9
```

This runs fresh processes in seeded shuffled blocks on the same recorded CPU,
checking recorded environment compatibility first. The report gives the median of
paired process per-call-p50 latency ratios and a 95% percentile bootstrap interval
from 10,000 resamples of process pairs. It never treats the 256 calls within one
process as independent process repetitions. Full process distributions, accuracy,
order and source/binary metadata remain in paired.json. Intervals are exploratory,
with no multiple-comparison adjustment; nine pairs and shared-machine conditions
limit inference. They are not confidence intervals for per-call p95 or universal
hardware rankings. Original single-run distributions remain unchanged.

Expanded HTML tables show call p99 (unavailable for old artifacts) and nominal
square-GEMM throughput, `2*n^3 / per_call_p50_ns` GFLOP/s. This divides a conventional
operation count by allocation-inclusive API latency; it is not a pure-kernel metric
and does not remove validation, copies, packing or allocation costs.

Hardware-workflow runners on Linux/ARM64, Windows and macOS retain separate reports.
Python pins Linux affinity when supported; macOS/Windows affinity may be unavailable,
which is recorded as null rather than an assumed fixed core. Hardware details and
NumPy's actual BLAS configuration are retained; runner labels alone do not establish
CPU vendor, physical host isolation or reproducible multi-machine speed rankings.

On Accelerate-backed macOS, OpenBLAS/OMP environment variables do not establish
Accelerate's thread policy. New runs compile the benchmark-only SDK shim and call
[BLASSetThreading / BLASGetThreading](https://developer.apple.com/documentation/accelerate/blassetthreading(_:))
on the same NumPy worker thread before timing, requiring the single-thread model.
The SDK supplies the enum value; it is not guessed in Python. Missing/unsupported
control fails the run. Metadata retains verification and the controller hash.
First-call operation timing excludes this explicit control initialization.
The initial macOS run39719312815 lacked this check and must not establish a
single-thread comparison; its original samples are not rewritten.

## Pivoted factors and minimum norm (0.9)

Four new workloads bring candidate runs to 100 workload/library pairs. The two
column-pivoted QR cases materialize thin Q/R and an owned permutation in Kernix,
nalgebra and faer. Each result must reconstruct the permuted original and have
orthogonal Q. Pivot ties/order may differ; the validator follows each library's
actual permutation. Kernix uses partial norm estimates with cancellation-triggered recomputation;
comparator algorithms/workspace and validation policies may differ.

The 64×32 rank-16 and 32×64 rank-32 cases solve for four RHS columns, including
factorization on every timed call. A is built from an independently full-rank U
and exactly representable dependent columns `U[:,j] + 0.5*U[:,j+1]`. Kernix uses
pivoted QR plus a scaled right QR in 0.9 (structured implicit RZ from 0.10),
nalgebra uses its native SVD/solve, and NumPy
uses native lstsq. Thus the mathematical minimum-norm problem agrees, but the
factorization algorithms are different. Null-space orthogonality and normal
residuals are checked independently. Kernix/NumPy use relative cutoff 1e-12;
nalgebra solve uses absolute singular cutoff 1e-10. These fixtures have a wide
gap between retained and zero singular values, not an ambiguous threshold boundary.
No comparison asserts identical rank policies for arbitrary matrices.

Exact registry versions before 0.9 omit the new Kernix workloads altogether;
`legacy-pivoted` prevents unavailable APIs from being compiled. Missing historical
results are documented, not filled with a substitute implementation. Existing
88-case observations and measurement boundaries remain independently comparable.


## Explicit execution cases (0.10)

The four parallel_matmul_into cases use a two-worker private Kernix executor,
reusing output storage allocated before the first operation. They compare versions
of Kernix, with no single-thread peer ranking. The safe RefCell wrapper cost is
included; private pool construction is outside timing. Linux selects up to two
allowed logical CPUs, preferring distinct reported physical cores, and records
the exact mask per workload. It does not infer P/E classes. Windows/macOS affinity
is unavailable. Ordinary rows keep their one-thread and single-CPU Linux policy.

Shared-variable backward computes sum(X*X) on a prepared real tensor graph in
Kernix and Candle and returns fresh gradient stores. Coalesced tensordot uses the
actual Kernix/NumPy arbitrary-axis APIs on matching contiguous tensors; reshapes
and preparation are outside timing only when they are fixture construction.


## Readable comparisons and explicit vendor observations (0.11)

Version comparisons use matching frozen process pairs when attached; sequential
single-run deltas remain descriptive in Markdown. No paired interval is inferred
from calls in one process. A lone Kernix result receives no peer-winner emphasis.
The standalone HTML separates versions/libraries, filters all workload views, reports
empty results and preserves semantic row/column headers. It keeps losses and actual
per-row worker/output contracts visible. PDF charts have at most six family panels
per page; HTML charts keep readable text on narrow displays. presentation.json
records renderer/source fingerprints separately from the measured revision.

New independent-operand128 cases use dyadic f32/f64 matrices and coordinate oracles;
the historical serial fixtures still use A*A. Cholesky512 exercises the blocked path.
On supported macOS current builds, four serial GEMM rows additionally measure the
explicit AccelerateExecutor, with output allocation and thread-policy save/restore
included. Native vendor allocations are opaque. This is a distinct optional backend;
registry0.10 has no substitute entry for that API. Hardware runs measure registry0.10
on the same host and interleave frozen processes before interpreting version changes.
