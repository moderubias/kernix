# Kernix 0.8.0 · CPU benchmarks

AMD EPYC 7763 64-Core Processor · one BLAS/Rayon thread · allocation-inclusive · warmed individual calls

**Microseconds per call, median. Lower is faster; bold marks the lowest observed median in each row.**

No previous-version baseline is paired with this view.

| Workload | Kernix | ndarray | nalgebra | faer | Candle | NumPy |
|---|---:|---:|---:|---:|---:|---:|
| Matrix multiplication · 32 × 32 · 32 × 32 · f64 | 2.525 | 2.204 | 2.104 | **2.034** | 2.174 | 4.088 |
| Matrix multiplication · 64 × 64 · 64 × 64 · f64 | **12.744** | 15.449 | 14.968 | 13.555 | 12.854 | 14.928 |
| Matrix multiplication · 128 × 128 · 128 × 128 · f64 | 93.687 | 213.732 | 210.025 | 100.529 | **92.925** | 100.990 |
| Matrix multiplication · 256 × 256 · 256 × 256 · f64 | **731.015** | 910.683 | 901.967 | 963.663 | 735.213 | 767.644 |
| Matrix multiplication · 64 × 64 · 64 × 64 · f32 | 6.542 | 7.444 | 7.403 | **6.523** | 6.592 | 8.176 |
| Matrix multiplication · 128 × 128 · 128 × 128 · f32 | 46.287 | 75.692 | 74.280 | 52.289 | **46.177** | 48.983 |
| Matrix multiplication · 256 × 256 · 256 × 256 · f32 | **352.744** | 615.598 | 617.082 | 417.535 | 356.691 | 363.384 |
| Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64 | **3.096** | — | — | — | 3.286 | 4.198 |
| Thin QR · 96 × 32 · f64 | 66.746 | — | 57.739 | **54.122** | — | 70.382 |
| Thin QR · 256 × 64 · f64 | 805.205 | — | 567.719 | **422.545** | — | 537.822 |
| Cholesky factorization · 64 × 64 · f64 | 25.808 | — | 20.228 | **14.206** | — | 18.174 |
| Cholesky factorization · 128 × 128 · f64 | 169.068 | — | 120.737 | **58.871** | — | 107.282 |
| Cholesky factorization · 256 × 256 · f64 | 1030.990 | — | 820.354 | **316.065** | — | 1094.099 |
| Row softmax · 256 × 64 · f64 | 120.307 | **110.859** | — | — | 115.458 | 131.027 |
| Create zeros · 64 × 64 · f32 | 0.311 | **0.240** | — | 0.300 | 0.301 | 0.751 |
| Create ones · 64 × 64 · f32 | 0.431 | **0.351** | — | 0.391 | 0.461 | 2.054 |
| Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64 | **3.977** | — | — | — | 7.564 | — |
| Model inference · batched normalized/sliced MLP · f64 | **3.146** | — | — | — | 4.940 | — |
| Model recording + forward · batched normalized/sliced MLP · f64 | **4.088** | — | — | — | 11.381 | — |
| Model backward · prepared graph → fresh gradients · f64 | **4.980** | — | — | — | 20.569 | — |
| Parameter update · transactional SGD, fixed gradients · f64 | **0.831** | — | — | — | — | — |

![Relative latency by workload](latency.png)

Each panel has its own scale. Ratios are library / Kernix median; below 1 means the comparator was faster.

## Reading these results

- One shared laptop, one pinned CPU and one BLAS/Rayon thread. Frequency and background load are uncontrolled; these observations establish no universal ranking.
- Timing includes output allocation and destruction. Inputs and imports are prepared outside timing. GEMM inputs are row-major; output layout and validation contracts can differ by library.
- Per-call p95 describes 256 individual warmed calls. The 15 batch means form a separate distribution; neither is a confidence interval.
- Rust allocation probes use a separate instrumented binary. Native engines may retain packing buffers; warmed probes exclude their first-use allocation and retention. NumPy native allocations are unavailable, never zero.

[Interactive report](report.html) · [Detailed measurements and method](DETAILS.md) · [Raw samples and provenance](raw.json) · [Printable chart](latency.pdf)

Measured source `227f339974f9c31fcf6bd37ccccbda3eedc99591`. Run `runner-ubuntu-latest-39719312815`. Rendering does not remeasure or modify raw evidence.
