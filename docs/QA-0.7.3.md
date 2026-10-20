# Kernix 0.7.3 verification

This patch extends the large-dense serial x86-64/std GEMM engine to f32. Both f32
and f64 now use the same checked scalar macro, m/k/n >=64 threshold and complete
runtime SIMD check. There is no new public API, dtype promotion or output-layout
change. Small/skinny/strided/portable and ordered paths remain. gemm-f32 0.19 is an
additional target-specific std dependency; no_std excludes gemm packages. Packed
scratch can allocate and retain thread-local memory until thread exit, now also
explained directly in rustdoc and package READMEs. Warm probes exclude first-use
allocation and retention. Reusing output does not imply scratch-free execution.

## Checks

- **520 default /522 all-feature /513 no-default** workspace tests pass. f32 adds
  threshold/rectangular checks against a wider f64 coordinate oracle, exact integer
  broadcast batches and repeated NaN-initialized output reuse. f64 regressions and
  existing custom-scalar, invalid-span, empty, signed-stride and ordered gates pass.
- Strict formatting/Clippy/rustdoc, 13 executable Markdown examples and 11 Python
  checks pass. Inventory remains 174 numerical entries and 28 recording methods.
- The actual no_std consumer executes two workflows on Rust 1.86. Workspace MSRV
  and no-default wasm/i686/aarch64 compilation pass; cross-compilation does not
  establish hardware performance. Both scalar-coordinate f32/f64 audits execute
  under AddressSanitizer and on Rust 1.86. Miri retains the existing path; this is
  not certification of every native instruction or hardware configuration.
- Six normal and five allocation-enabled harness tests pass, including all five
  Rust f32 GEMM workers checked against a wider oracle, independent Candle VJPs and
  scalar training-stage checks. Strict harness Clippy passes. NumPy first-use
  workers validate f32 dtype and retain the existing f64 batched checks.

## Clean paired evidence

Measured source `bc459f44b276e86275cc4e431e4946443aefb15f`, clean before/after;
both runs retain 80 workload/library cases, distributions, first-use observations,
accuracy/resource fields, actual manifests/locks and source/binary hashes. All
three baseline packages are exact registry 0.7.2, VCS `62f32c1`. Environment, flags,
NumPy/OpenBLAS configuration and actual comparator versions match. Source archives
exclude local planning/research files.

- [Readable current comparison](../benchmarks/results/0.7.3-clean-bc459f4/README.md)
  and [standalone HTML](../benchmarks/results/0.7.3-clean-bc459f4/report.html).
- [Exact registry 0.7.2](../benchmarks/results/0.7.2-registry-bc459f4/README.md).

Allocation-inclusive per-call medians in microseconds:

| Square product | Registry 0.7.2 | Workspace 0.7.3 | Current NumPy/OpenBLAS |
|---|---:|---:|---:|
| f32 64 | 5.884 | 6.441 | 6.464 |
| f32 128 | 49.181 | 34.749 | 39.432 |
| f32 256 | 393.444 | 270.684 | 286.792 |
| f64 64 | 9.138 | 9.162 | 12.827 |
| f64 128 | 73.035 | 73.624 | 78.457 |
| f64 256 | 572.709 | 574.036 | 612.150 |

f32 GEMM128 and GEMM256 per-call medians fall about 29.3% and 31.2% against the
paired baseline. The f32 64 case has a higher per-call median (6.441 versus 5.884 us)
and a lower separately measured median batch mean (4.936 versus 6.013 us). These
are distinct distributions; no blanket speedup is claimed. f64 per-call observations
differ by less than 1%; its engine is retained. Every comparator win/loss remains
visible. f32 validation accumulates actual f32 inputs in f64 outside timing, budget
1e-5 relative-plus-absolute for every comparator; the f64 budget stays 1e-11.

Runs are sequential on a shared laptop with uncontrolled load/frequency/cache
behavior. Observed medians/p95 are not significance estimates, universal rankings
or complete application latency. Factorization and other gaps remain visible.

## Publication

Registry payloads, downstream consumers and release-source CI are recorded after
those gates finish.
