# Kernix 0.8.0 · CPU benchmarks

12th Gen Intel(R) Core(TM) i3-1215U · one BLAS/Rayon thread · allocation-inclusive · warmed individual calls

**Microseconds per call, median. Lower is faster; bold marks the lowest observed median in each row.**

No previous-version baseline is paired with this view.

| Workload | Kernix | ndarray | nalgebra | faer | Candle | NumPy |
|---|---:|---:|---:|---:|---:|---:|
| Matrix multiplication · 32 × 32 · 32 × 32 · f64 | 1.529 | 1.517 | **1.384** | 1.780 | 1.434 | 2.519 |
| Matrix multiplication · 64 × 64 · 64 × 64 · f64 | 9.853 | 10.659 | 10.516 | 10.484 | **9.673** | 11.954 |
| Matrix multiplication · 128 × 128 · 128 × 128 · f64 | 72.804 | 112.933 | 113.800 | 83.957 | **71.942** | 74.198 |
| Matrix multiplication · 256 × 256 · 256 × 256 · f64 | **550.760** | 661.684 | 662.855 | 747.131 | 561.715 | 611.331 |
| Matrix multiplication · 64 × 64 · 64 × 64 · f32 | **4.712** | 5.697 | 5.660 | 5.643 | 4.928 | 5.767 |
| Matrix multiplication · 128 × 128 · 128 × 128 · f32 | **34.962** | 49.395 | 47.523 | 40.147 | 35.161 | 39.079 |
| Matrix multiplication · 256 × 256 · 256 × 256 · f32 | **273.282** | 386.693 | 394.214 | 327.257 | 280.492 | 278.882 |
| Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64 | **2.018** | — | — | — | 2.153 | 2.945 |
| Thin QR · 96 × 32 · f64 | 45.978 | — | **36.882** | 68.343 | — | 38.230 |
| Thin QR · 256 × 64 · f64 | 480.702 | — | 368.639 | 399.014 | — | **336.724** |
| Cholesky factorization · 64 × 64 · f64 | 21.199 | — | 14.344 | **8.489** | — | 10.184 |
| Cholesky factorization · 128 × 128 · f64 | 110.993 | — | 94.382 | **39.205** | — | 71.079 |
| Cholesky factorization · 256 × 256 · f64 | 614.523 | — | 642.426 | **194.519** | — | 512.960 |
| Row softmax · 256 × 64 · f64 | 83.640 | **68.899** | — | — | 77.915 | 102.018 |
| Create zeros · 64 × 64 · f32 | 0.136 | **0.120** | — | 0.156 | 0.167 | 0.389 |
| Create ones · 64 × 64 · f32 | 0.203 | **0.140** | — | 0.268 | 0.226 | 1.336 |
| Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64 | **2.592** | — | — | — | 3.958 | — |
| Model inference · batched normalized/sliced MLP · f64 | **2.136** | — | — | — | 3.391 | — |
| Model recording + forward · batched normalized/sliced MLP · f64 | **2.582** | — | — | — | 5.017 | — |
| Model backward · prepared graph → fresh gradients · f64 | **3.082** | — | — | — | 13.914 | — |
| Parameter update · transactional SGD, fixed gradients · f64 | **0.498** | — | — | — | — | — |

![Relative latency by workload](latency.png)

Each panel has its own scale. Ratios are library / Kernix median; below 1 means the comparator was faster.

## Reading these results

- One measured host and one BLAS/Rayon thread. CPU affinity pinned to logical CPU 0. Frequency and background load are uncontrolled; these observations establish no universal ranking.
- Timing includes output allocation and destruction. Inputs and imports are prepared outside timing. GEMM inputs are row-major; output layout and validation contracts can differ by library.
- Per-call p95 describes 256 individual warmed calls. The 15 batch means form a separate distribution; neither is a confidence interval.
- Rust allocation probes use a separate instrumented binary. Native engines may retain packing buffers; warmed probes exclude their first-use allocation and retention. NumPy native allocations are unavailable, never zero.

[Interactive report](report.html) · [Detailed measurements and method](DETAILS.md) · [Raw samples and provenance](raw.json) · [Printable chart](latency.pdf)

Measured source `281af4b51aa0fe6f354770c2d4f3d18544586377`. Run `0.8.0-registry-090`. Rendering does not remeasure or modify raw evidence.
