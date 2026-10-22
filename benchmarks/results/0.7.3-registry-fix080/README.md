# Kernix 0.7.3 · CPU benchmarks

12th Gen Intel(R) Core(TM) i3-1215U · one CPU · allocation-inclusive · warmed individual calls

**Microseconds per call, median. Lower is faster; bold marks the lowest observed median in each row.**

No previous-version baseline is paired with this view.

| Workload | Kernix | ndarray | nalgebra | faer | Candle | NumPy |
|---|---:|---:|---:|---:|---:|---:|
| Matrix multiplication · 32 × 32 · 32 × 32 · f64 | 1.737 | **1.467** | 1.495 | 1.708 | 1.497 | 2.599 |
| Matrix multiplication · 64 × 64 · 64 × 64 · f64 | **9.665** | 10.600 | 11.250 | 10.842 | 10.108 | 11.917 |
| Matrix multiplication · 128 × 128 · 128 × 128 · f64 | **73.407** | 119.253 | 117.090 | 85.352 | 76.270 | 78.251 |
| Matrix multiplication · 256 × 256 · 256 × 256 · f64 | 570.453 | 664.428 | 680.841 | 750.506 | **569.457** | 610.098 |
| Matrix multiplication · 64 × 64 · 64 × 64 · f32 | **5.094** | 6.968 | 6.069 | 5.295 | 5.163 | 6.179 |
| Matrix multiplication · 128 × 128 · 128 × 128 · f32 | 50.087 | 49.696 | 49.492 | 41.142 | **36.484** | 39.181 |
| Matrix multiplication · 256 × 256 · 256 × 256 · f32 | **279.287** | 387.241 | 424.814 | 335.476 | 293.756 | 288.021 |
| Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64 | **1.964** | — | — | — | 2.194 | 3.070 |
| Thin QR · 96 × 32 · f64 | 47.046 | — | **37.902** | 74.457 | — | 40.026 |
| Thin QR · 256 × 64 · f64 | 514.650 | — | 376.353 | 435.016 | — | **348.518** |
| Cholesky factorization · 64 × 64 · f64 | 25.208 | — | 15.047 | **8.426** | — | 10.863 |
| Cholesky factorization · 128 × 128 · f64 | 168.412 | — | 104.188 | **39.410** | — | 73.713 |
| Cholesky factorization · 256 × 256 · f64 | 1219.916 | — | 709.381 | **199.262** | — | 510.372 |
| Row softmax · 256 × 64 · f64 | 85.337 | **72.206** | — | — | 82.419 | 108.198 |
| Create zeros · 64 × 64 · f32 | 0.146 | **0.099** | — | 0.167 | 0.155 | 0.389 |
| Create ones · 64 × 64 · f32 | 0.204 | **0.141** | — | 0.189 | 0.230 | 1.333 |
| Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64 | **2.656** | — | — | — | 8.284 | — |
| Model inference · batched normalized/sliced MLP · f64 | **2.238** | — | — | — | 3.524 | — |
| Model recording + forward · batched normalized/sliced MLP · f64 | **2.802** | — | — | — | 5.153 | — |
| Model backward · prepared graph → fresh gradients · f64 | **3.102** | — | — | — | 14.658 | — |
| Parameter update · transactional SGD, fixed gradients · f64 | **0.532** | — | — | — | — | — |

![Relative latency by workload](latency.png)

Each panel has its own scale. Ratios are library / Kernix median; below 1 means the comparator was faster.

## Reading these results

- One shared laptop, one pinned CPU and one BLAS/Rayon thread. Frequency and background load are uncontrolled; these observations establish no universal ranking.
- Timing includes output allocation and destruction. Inputs and imports are prepared outside timing. GEMM inputs are row-major; output layout and validation contracts can differ by library.
- Per-call p95 describes 256 individual warmed calls. The 15 batch means form a separate distribution; neither is a confidence interval.
- Rust allocation probes use a separate instrumented binary. Native engines may retain packing buffers; warmed probes exclude their first-use allocation and retention. NumPy native allocations are unavailable, never zero.

[Interactive report](report.html) · [Detailed measurements and method](DETAILS.md) · [Raw samples and provenance](raw.json) · [Printable chart](latency.pdf)

Measured source `2d62febf07666079a81043b287fe67cdadd2429e`. Run `0.7.3-registry-fix080`. Rendering does not remeasure or modify raw evidence.
