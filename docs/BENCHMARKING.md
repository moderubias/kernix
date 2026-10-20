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

The selected Python needs NumPy and Matplotlib. The optional PDF renderer needs
Pandoc and XeLaTeX. `--require-openblas` rejects a different NumPy BLAS build rather
than silently substituting it. Use an environment with an optimized NumPy wheel
or a documented vendor build. MKL/Accelerate and GPU baselines are not provided by
this harness. Candle CPU supplies the equivalent automatic-differentiation case;
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

Operation inputs are f64 and C-order in every comparator; constructor cases
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
checks its result against a scalar/analytic/reconstruction oracle, warms eight times,
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
