# Kernix 0.7.1 · CPU benchmarks

12th Gen Intel(R) Core(TM) i3-1215U · one CPU · allocation-inclusive · warmed individual calls

**Microseconds per call, median. Lower is faster; bold marks the lowest observed median in each row.**

No previous-version baseline is paired with this view.

| Workload | Kernix | ndarray | nalgebra | faer | Candle | NumPy |
|---|---:|---:|---:|---:|---:|---:|
| Matrix multiplication · 32 × 32 · 32 × 32 · f64 | 1.648 | **1.396** | 1.529 | 1.616 | 1.533 | 2.813 |
| Matrix multiplication · 64 × 64 · 64 × 64 · f64 | 11.918 | 11.235 | 11.308 | 10.951 | **10.010** | 12.842 |
| Matrix multiplication · 128 × 128 · 128 × 128 · f64 | 120.337 | 119.341 | 117.145 | 85.355 | **74.649** | 77.968 |
| Matrix multiplication · 256 × 256 · 256 × 256 · f64 | 674.471 | 665.291 | 662.539 | 744.669 | **578.302** | 607.544 |
| Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64 | **1.993** | — | — | — | 2.084 | 3.263 |
| Thin QR · 96 × 32 · f64 | 41.627 | — | **36.761** | 64.548 | — | 39.043 |
| Thin QR · 256 × 64 · f64 | 495.470 | — | 369.394 | 419.047 | — | **346.813** |
| Cholesky factorization · 64 × 64 · f64 | 22.754 | — | 13.762 | **8.147** | — | 10.397 |
| Row softmax · 256 × 64 · f64 | 83.192 | **72.227** | — | — | 77.958 | 100.630 |
| Create zeros · 64 × 64 · f32 | 0.148 | **0.101** | — | 0.189 | 0.169 | 0.434 |
| Create ones · 64 × 64 · f32 | 0.193 | **0.147** | — | 0.179 | 0.225 | 1.352 |
| Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64 | **2.583** | — | — | — | 4.326 | — |
| Model inference · batched normalized/sliced MLP · f64 | **2.063** | — | — | — | 3.523 | — |
| Model recording + forward · batched normalized/sliced MLP · f64 | **2.563** | — | — | — | 5.375 | — |
| Model backward · prepared graph → fresh gradients · f64 | **3.232** | — | — | — | 14.247 | — |
| Parameter update · transactional SGD, fixed gradients · f64 | **0.596** | — | — | — | — | — |

![Relative latency by workload](latency.png)

Each panel has its own scale. Ratios are library / Kernix median; below 1 means the comparator was faster.

## Reading these results

- One shared laptop, one pinned CPU and one BLAS/Rayon thread. Frequency and background load are uncontrolled; these observations establish no universal ranking.
- Timing includes output allocation and destruction. Inputs and imports are prepared outside timing. GEMM inputs are row-major; output layout and validation contracts can differ by library.
- Per-call p95 describes 256 individual warmed calls. The 15 batch means form a separate distribution; neither is a confidence interval.
- Rust allocation probes use a separate instrumented binary. Native engines may retain packing buffers; warmed probes exclude their first-use allocation and retention. NumPy native allocations are unavailable, never zero.

[Interactive report](report.html) · [Detailed measurements and method](DETAILS.md) · [Raw samples and provenance](raw.json) · [Printable chart](latency.pdf)

Measured source `50673332cd664b98fad90a24febce1fd8d6395b0`. Run `0.7.1-registry-5067333`. Rendering does not remeasure or modify raw evidence.
