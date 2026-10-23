# Kernix 0.9.0 · CPU benchmarks

Apple M1 (Virtual) · one BLAS/Rayon thread · allocation-inclusive · warmed individual calls

**Microseconds per call, median. Lower is faster; bold marks the lowest observed median in each row.**

No previous-version baseline is paired with this view.

| Workload | Kernix | ndarray | nalgebra | faer | Candle | NumPy |
|---|---:|---:|---:|---:|---:|---:|
| Matrix multiplication · 32 × 32 · 32 × 32 · f64 | 2.250 | 2.083 | 2.083 | 2.417 | 2.583 | **1.541** |
| Matrix multiplication · 64 × 64 · 64 × 64 · f64 | 12.959 | 13.375 | 15.584 | 14.709 | 14.500 | **3.500** |
| Matrix multiplication · 128 × 128 · 128 × 128 · f64 | 102.541 | 109.792 | 103.667 | 108.500 | 97.750 | **17.667** |
| Matrix multiplication · 256 × 256 · 256 × 256 · f64 | 845.417 | 816.333 | 1022.417 | 822.750 | 883.000 | **120.834** |
| Matrix multiplication · 64 × 64 · 64 × 64 · f32 | 10.500 | 6.959 | 6.917 | 8.209 | 7.916 | **1.916** |
| Matrix multiplication · 128 × 128 · 128 × 128 · f32 | 52.459 | 47.834 | 51.208 | 55.500 | 52.916 | **6.125** |
| Matrix multiplication · 256 × 256 · 256 × 256 · f32 | 372.500 | 390.542 | 384.375 | 458.125 | 408.125 | **39.916** |
| Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64 | 2.875 | — | — | — | 3.583 | **1.791** |
| Column-pivoted QR · 96 × 32 · f64 | 73.250 | — | 96.375 | **40.167** | — | — |
| Column-pivoted QR · 256 × 64 · f64 | 605.417 | — | 1009.125 | **309.791** | — | — |
| Minimum-norm least squares · 64 × 32, rank 16, 4 RHS · f64 | **51.250** | — | 69.834 | — | — | 81.000 |
| Minimum-norm least squares · 32 × 64, rank 32, 4 RHS · f64 | **95.917** | — | 152.958 | — | — | 136.125 |
| Thin QR · 96 × 32 · f64 | 56.792 | — | **43.542** | 50.541 | — | 47.500 |
| Thin QR · 256 × 64 · f64 | 497.917 | — | 356.042 | **303.167** | — | 364.458 |
| Cholesky factorization · 64 × 64 · f64 | 22.292 | — | 21.042 | **10.334** | — | 15.083 |
| Cholesky factorization · 128 × 128 · f64 | 167.667 | — | 104.000 | **49.500** | — | 108.167 |
| Cholesky factorization · 256 × 256 · f64 | 1322.125 | — | 730.125 | **250.708** | — | 330.125 |
| Row softmax · 256 × 64 · f64 | 78.917 | 65.250 | — | — | **65.000** | 76.667 |
| Create zeros · 64 × 64 · f32 | 0.750 | **0.291** | — | **0.291** | 0.458 | 0.542 |
| Create ones · 64 × 64 · f32 | **0.292** | 0.333 | — | 0.333 | 0.417 | 1.125 |
| Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64 | **8.042** | — | — | — | 19.416 | — |
| Model inference · batched normalized/sliced MLP · f64 | **2.250** | — | — | — | 5.458 | — |
| Model recording + forward · batched normalized/sliced MLP · f64 | **2.875** | — | — | — | 5.625 | — |
| Model backward · prepared graph → fresh gradients · f64 | **3.750** | — | — | — | 20.959 | — |
| Parameter update · transactional SGD, fixed gradients · f64 | **0.750** | — | — | — | — | — |

![Relative latency by workload](latency.png)

Each panel has its own scale. Ratios are library / Kernix median; below 1 means the comparator was faster.

## Reading these results

Minimum-norm rows include factorization and four RHS. Kernix uses pivoted/right QR; NumPy uses native lstsq; nalgebra uses native SVD/solve. Fixtures have a wide rank gap; null-space orthogonality and normal residual are validated independently. Algorithm costs and rank policies differ; see DETAILS.md for thresholds. These are not general SVD comparisons.

- One measured host and one BLAS/Rayon thread. CPU affinity was not pinned. Frequency and background load are uncontrolled; these observations establish no universal ranking.
- Timing includes output allocation and destruction. Inputs and imports are prepared outside timing. GEMM inputs are row-major; output layout and validation contracts can differ by library.
- Per-call p95 describes 256 individual warmed calls. The 15 batch means form a separate distribution; neither is a confidence interval.
- Rust allocation probes use a separate instrumented binary. Native engines may retain packing buffers; warmed probes exclude their first-use allocation and retention. NumPy native allocations are unavailable, never zero.

[Interactive report](report.html) · [Detailed measurements and method](DETAILS.md) · [Raw samples and provenance](raw.json) · [Printable chart](latency.pdf)

Measured source `9b7ef492c99bf2202e7eb3b07a79ef8f108991c7`. Run `runner-macos-latest-39980084433`. Rendering does not remeasure or modify raw evidence.
