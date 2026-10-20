# CPU benchmark reports

Start with the [0.7.3 comparison](0.7.3-clean-bc459f4/README.md): one workload per
row, library columns, colored per-workload charts and a standalone
[HTML report](0.7.3-clean-bc459f4/report.html) with filtering and expandable details.
Download the run directory to open HTML locally with its adjacent chart. Raw
samples, complete methods, source hashes and actual locks remain adjacent. Missing
measurements mean unmeasured, not unsupported.

The paired [registry 0.7.2 baseline](0.7.2-registry-bc459f4/README.md) uses the same
80 cases and comparator versions. f32 GEMM128 measured 34.749 us versus 49.181 us
in 0.7.2; current NumPy/OpenBLAS is 39.432 us. Current f64 GEMM128 is 73.624 us
versus NumPy 78.457 us. These allocation-inclusive one-CPU observations on a shared
laptop are not a universal ranking. The mixed f32 64 result and every other
win/loss remain visible. See [release QA](../../docs/QA-0.7.3.md).

## Previous clean evidence

- [0.7.2](0.7.2-clean-5067333/README.md) and [registry 0.7.1](0.7.1-registry-5067333/README.md): dense f64 engine, 62 cases each; GEMM128 120.337→68.456 us in that paired run. See [0.7.2 QA](../../docs/QA-0.7.2.md) for conditions.

- [0.7.1](0.7.1-clean-4f60380/README.md) and [registry 0.7.0](0.7.0-registry-4f60380/README.md): fused layer normalization, 50 cases each.
- [0.7.0](0.7.0-clean-aa7347a/README.md) and [registry 0.6.0](0.6.0-registry-aa7347a/README.md): batched/vector autodiff and normalized training stages.

## Earlier batch-only history

All values below are median batch time in microseconds. See each report for the
competitors, exact workload, allocation boundaries and numerical API differences.
Earlier runs were unpinned and lacked source snapshots; 0.4 pins one CPU and includes
an exact source archive. Frequency and other laptop activity are uncontrolled.
These trends cannot establish a controlled speedup or any complete-model FPS.

| Version | GEMM 128 f64 | Sum 65536 f64 | Softmax 256x64 f64 | Conv f32 | RGB 1080p to 224 f32 |
|---|---:|---:|---:|---:|---:|
| [0.1.1](0.1.1/README.md) | 3311.847 | 37.352 | 765.891 | — | — |
| [0.2.0](0.2.0/README.md) | 382.258 | 33.711 | 221.617 | — | — |
| [0.3.0](0.3.0/README.md) | 384.712 | 31.040 | 201.896 | 1202.783 | 1105.560 |
| [0.4.0](0.4.0/README.md) | 86.102 | 5.538 | 77.015 | 11.361 | 438.953 |
| [baseline-0.1.0](baseline-0.1.0/README.md) | 3029.011 | 31.027 | 397.788 | — | — |

Kernix and ndarray now share matrixmultiply for native GEMM. Native FMA/order can
change last bits; ordered matmul, reference convolution and compensated sums are
explicit choices. QR/Cholesky and reverse-mode graph overhead remain optimization
work; the current report preserves these unfavorable comparisons too.

The [additional OpenBLAS run](0.4.0-openblas/README.md) uses the same NumPy version
with an optimized bundled BLAS, in an isolated environment. Prefer it when assessing
the NumPy comparison: GEMM128 is Kernix 84.936 us vs NumPy 78.261 us. The original
0.4 report uses system CBLAS 3.12 and remains available unchanged. Both show current
QR/Cholesky gaps, rather than implying general superiority from the reference build.
