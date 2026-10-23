# Kernix 0.9.0 · CPU benchmarks

12th Gen Intel(R) Core(TM) i3-1215U · one BLAS/Rayon thread · allocation-inclusive · warmed individual calls

**Microseconds per call, median. Lower is faster; bold marks the lowest observed median in each row.**

Baseline: 0.8.0-registry-090. Δ is latency change; negative is faster. Runs are sequential, not randomized/interleaved.

| Workload | Kernix | ndarray | nalgebra | faer | Candle | NumPy | Kernix Δ vs baseline |
|---|---:|---:|---:|---:|---:|---:|---:|
| Matrix multiplication · 32 × 32 · 32 × 32 · f64 | 1.637 | **1.394** | 1.650 | 1.442 | 1.420 | 2.388 | +7.1% |
| Matrix multiplication · 64 × 64 · 64 × 64 · f64 | **9.483** | 11.334 | 10.468 | 11.285 | 9.957 | 11.158 | -3.8% |
| Matrix multiplication · 128 × 128 · 128 × 128 · f64 | **73.171** | 119.441 | 117.273 | 78.058 | 74.702 | 92.307 | +0.5% |
| Matrix multiplication · 256 × 256 · 256 × 256 · f64 | **556.114** | 664.653 | 636.472 | 686.047 | 557.263 | 597.657 | +1.0% |
| Matrix multiplication · 64 × 64 · 64 × 64 · f32 | 5.213 | 7.242 | 5.642 | 5.708 | **4.885** | 6.276 | +10.6% |
| Matrix multiplication · 128 × 128 · 128 × 128 · f32 | 36.757 | 53.503 | 49.789 | 41.289 | **34.787** | 36.345 | +5.1% |
| Matrix multiplication · 256 × 256 · 256 × 256 · f32 | **270.542** | 394.088 | 382.811 | 306.861 | 276.250 | 285.924 | -1.0% |
| Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64 | 2.160 | — | — | — | **2.103** | 2.865 | +7.0% |
| Column-pivoted QR · 96 × 32 · f64 | **56.637** | — | 78.306 | 71.206 | — | — | — |
| Column-pivoted QR · 256 × 64 · f64 | 511.262 | — | 781.969 | **466.149** | — | — | — |
| Minimum-norm least squares · 64 × 32, rank 16, 4 RHS · f64 | **49.903** | — | 71.614 | — | — | 63.224 | — |
| Minimum-norm least squares · 32 × 64, rank 32, 4 RHS · f64 | **76.670** | — | 129.405 | — | — | 115.946 | — |
| Thin QR · 96 × 32 · f64 | 49.547 | — | **36.100** | 69.820 | — | 38.052 | +7.8% |
| Thin QR · 256 × 64 · f64 | 513.125 | — | 365.251 | 398.048 | — | **340.891** | +6.7% |
| Cholesky factorization · 64 × 64 · f64 | 20.684 | — | 13.137 | **8.396** | — | 10.231 | -2.4% |
| Cholesky factorization · 128 × 128 · f64 | 122.243 | — | 96.829 | **37.323** | — | 68.058 | +10.1% |
| Cholesky factorization · 256 × 256 · f64 | 619.867 | — | 639.843 | **191.370** | — | 510.375 | +0.9% |
| Row softmax · 256 × 64 · f64 | 83.691 | **71.240** | — | — | 80.473 | 99.419 | +0.1% |
| Create zeros · 64 × 64 · f32 | 0.133 | **0.098** | — | 0.141 | 0.155 | 0.353 | -2.2% |
| Create ones · 64 × 64 · f32 | 0.203 | **0.154** | — | 0.184 | 0.237 | 1.348 | +0.0% |
| Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64 | **2.366** | — | — | — | 4.268 | — | -8.7% |
| Model inference · batched normalized/sliced MLP · f64 | **2.092** | — | — | — | 3.533 | — | -2.1% |
| Model recording + forward · batched normalized/sliced MLP · f64 | **2.656** | — | — | — | 5.141 | — | +2.9% |
| Model backward · prepared graph → fresh gradients · f64 | **3.213** | — | — | — | 13.387 | — | +4.3% |
| Parameter update · transactional SGD, fixed gradients · f64 | **0.552** | — | — | — | — | — | +10.8% |

![Relative latency by workload](latency.png)

Each panel has its own scale. Ratios are library / Kernix median; below 1 means the comparator was faster.

## Reading these results

Minimum-norm rows include factorization and four RHS. Kernix uses pivoted/right QR; NumPy uses native lstsq; nalgebra uses native SVD/solve. Fixtures have a wide rank gap; null-space orthogonality and normal residual are validated independently. Algorithm costs and rank policies differ; see DETAILS.md for thresholds. These are not general SVD comparisons.

- One shared laptop, one pinned CPU and one BLAS/Rayon thread. Frequency and background load are uncontrolled; these observations establish no universal ranking.
- Timing includes output allocation and destruction. Inputs and imports are prepared outside timing. GEMM inputs are row-major; output layout and validation contracts can differ by library.
- Per-call p95 describes 256 individual warmed calls. The 15 batch means form a separate distribution; neither is a confidence interval.
- Rust allocation probes use a separate instrumented binary. Native engines may retain packing buffers; warmed probes exclude their first-use allocation and retention. NumPy native allocations are unavailable, never zero.

[Interactive report](report.html) · [Detailed measurements and method](DETAILS.md) · [Raw samples and provenance](raw.json) · [Printable chart](latency.pdf)

Measured source `4e40ea5a98e7f98505da7cc3dd8a2d80c1a4bd92`. Run `0.9.0-final`. Rendering does not remeasure or modify raw evidence.
