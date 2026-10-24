# Kernix 0.7.2 · CPU benchmarks

12th Gen Intel(R) Core(TM) i3-1215U · one CPU · allocation-inclusive · warmed individual calls

**Microseconds per call, median. Lower is faster; bold marks the lowest observed median in each row.**

No previous-version baseline is paired with this view.

| Workload | Kernix | ndarray | nalgebra | faer | Candle | NumPy |
|---|---:|---:|---:|---:|---:|---:|
| Matrix multiplication · 32 × 32 · 32 × 32 · f64 | 1.589 | **1.412** | 1.498 | 1.875 | 1.500 | 2.822 |
| Matrix multiplication · 64 × 64 · 64 × 64 · f64 | **9.138** | 11.253 | 11.299 | 10.916 | 9.862 | 13.142 |
| Matrix multiplication · 128 × 128 · 128 × 128 · f64 | **73.035** | 119.839 | 118.439 | 86.168 | 73.442 | 79.676 |
| Matrix multiplication · 256 × 256 · 256 × 256 · f64 | **572.709** | 702.672 | 682.415 | 742.204 | 579.849 | 608.764 |
| Matrix multiplication · 64 × 64 · 64 × 64 · f32 | 5.884 | 5.847 | 6.161 | 5.350 | **4.779** | 6.338 |
| Matrix multiplication · 128 × 128 · 128 × 128 · f32 | 49.181 | 51.443 | 53.199 | 41.712 | **34.791** | 39.220 |
| Matrix multiplication · 256 × 256 · 256 × 256 · f32 | 393.444 | 391.705 | 398.043 | 312.433 | **278.711** | 286.706 |
| Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64 | **1.936** | — | — | — | 2.078 | 3.332 |
| Thin QR · 96 × 32 · f64 | 42.746 | — | **37.668** | 64.469 | — | 38.141 |
| Thin QR · 256 × 64 · f64 | 496.680 | — | 371.777 | 407.851 | — | **348.219** |
| Cholesky factorization · 64 × 64 · f64 | 22.798 | — | 13.105 | **8.176** | — | 10.755 |
| Row softmax · 256 × 64 · f64 | 85.344 | **71.614** | — | — | 80.122 | 99.838 |
| Create zeros · 64 × 64 · f32 | 0.135 | **0.090** | — | 0.143 | 0.169 | 0.396 |
| Create ones · 64 × 64 · f32 | 0.201 | **0.154** | — | 0.202 | 0.237 | 1.345 |
| Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64 | **2.423** | — | — | — | 4.231 | — |
| Model inference · batched normalized/sliced MLP · f64 | **2.001** | — | — | — | 3.486 | — |
| Model recording + forward · batched normalized/sliced MLP · f64 | **2.776** | — | — | — | 5.198 | — |
| Model backward · prepared graph → fresh gradients · f64 | **3.194** | — | — | — | 14.451 | — |
| Parameter update · transactional SGD, fixed gradients · f64 | **0.514** | — | — | — | — | — |

![Relative latency by workload](latency.png)

Each panel has its own scale. Ratios are library / Kernix median; below 1 means the comparator was faster.

## Reading these results

- One shared laptop, one pinned CPU and one BLAS/Rayon thread. Frequency and background load are uncontrolled; these observations establish no universal ranking.
- Timing includes output allocation and destruction. Inputs and imports are prepared outside timing. GEMM inputs are row-major; output layout and validation contracts can differ by library.
- Per-call p95 describes 256 individual warmed calls. The 15 batch means form a separate distribution; neither is a confidence interval.
- Rust allocation probes use a separate instrumented binary. Native engines may retain packing buffers; warmed probes exclude their first-use allocation and retention. NumPy native allocations are unavailable, never zero.

[Interactive report](report.html) · [Detailed measurements and method](DETAILS.md) · [Raw samples and provenance](raw.json) · [Printable chart](latency.pdf)

Measured source `bc459f44b276e86275cc4e431e4946443aefb15f`. Run `0.7.2-registry-bc459f4`. Rendering does not remeasure or modify raw evidence.
