# Kernix 0.7.2 verification

This iteration addresses dense f64 GEMM and evidence readability. Large contiguous
products on x86-64/std use serial gemm-f64 0.19 when the engine's complete SIMD
capability check succeeds. Other shapes, types, layouts and targets retain the
existing engine/reference paths. The threshold is m/k/n >=64, with no special case
for 128. Output remains C-order: the native call computes C^T = B^T A^T into the
same allocation. No public API, implicit dtype conversion or external BLAS is added.
The new runtime dependencies are isolated from no_std and other targets.

The engine can retain an L2-sized thread-local packing slab until thread exit.
Warmed allocation probes exclude that first-use allocation and retention. Ordered
matmul remains the explicit alternative without packed element-buffer allocation.
This is a measured internal heuristic, not a universal optimum or zero-cost claim.

## Correctness and compatibility

- **518 default /520 all-feature /511 no-default** workspace tests pass, including
  boundary dimensions, rectangular tails, broadcast batches, ordered results and
  repeated output-buffer reuse initialized with NaNs. Existing dtype/custom-scalar,
  signed-stride, empty and invalid-storage checks remain release gates.
- Formatting, strict Clippy/rustdoc, 13 executable Markdown examples and 11 Python
  checks pass. The operation inventory remains 174 entries and 28 Tape methods.
- The actual no_std consumer executes two workflows on Rust 1.86; workspace MSRV
  and no-default wasm/i686/aarch64 compilation pass. Cross-compilation does not
  establish executed ARM hardware performance.
- Five normal and five allocation-enabled independent harness tests pass, including
  scalar training-stage oracles and Candle VJP comparisons; strict harness Clippy
  passes. A separate coordinate-level native audit passes both AddressSanitizer
  and actual Rust 1.86 execution. This does not certify every assembly instruction
  or hardware configuration. Miri deliberately uses the existing path.

## Clean paired evidence

Measured source `50673332cd664b98fad90a24febce1fd8d6395b0` is clean before/after
both 62-case runs. Reports retain raw distributions, accuracy checks, hashes,
source archives, actual driver manifests/locks, resources and first-use observations.
The baseline pins all three packages to exact registry 0.7.1, VCS `533b693`.
Comparator versions, environment, flags and optimized NumPy/OpenBLAS configuration
match: faer 0.24.4, Candle 0.11, ndarray 0.17.2 and nalgebra 0.35.0. Source archives
exclude local research/planning files.

- [Current readable comparison](../benchmarks/results/0.7.2-clean-5067333/README.md)
  and [standalone HTML](../benchmarks/results/0.7.2-clean-5067333/report.html).
- [Registry 0.7.1 baseline](../benchmarks/results/0.7.1-registry-5067333/README.md).

Allocation-inclusive per-call medians in microseconds:

| Dense f64 square | Registry 0.7.1 | Workspace 0.7.2 | Current NumPy/OpenBLAS |
|---|---:|---:|---:|
| 32 | 1.648 | 1.535 | 2.741 |
| 64 | 11.918 | 9.120 | 12.512 |
| 128 | 120.337 | 68.456 | 75.573 |
| 256 | 674.471 | 548.673 | 592.335 |

For 128, the current per-call p95 is 74.700 us, versus 139.128 us in the baseline.
The separately measured median batch means are 73.188 and 122.893 us; these are
different distributions. The median per-call reduction is about 43.1% against
0.7.1; current Kernix latency is about 9.4% below current NumPy for this case.
All wins/losses remain in the reports. Runs are sequential on a shared laptop,
with uncontrolled frequency/load/cache effects, not randomized trials or a
universal ranking. Factorization and other gaps remain visible.

The renderer adds readable side-by-side library columns, per-workload colored
PNG/PDF panels, optional previous-version deltas and a searchable standalone HTML
view with expandable distributions/resources. It preserves raw evidence; chart
ratios mean library latency / Kernix latency, below one favoring the comparator.
Actual Firefox rendering was visually inspected. Historical reports are unchanged.

## Publication

Registry payloads, release-source CI and downstream validation are recorded after
the publication gates finish.
