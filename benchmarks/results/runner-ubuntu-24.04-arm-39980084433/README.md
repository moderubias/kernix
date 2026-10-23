# Kernix 0.9.0 · CPU benchmarks

Neoverse-N2 · one BLAS/Rayon thread · allocation-inclusive · warmed individual calls

**Microseconds per call, median. Lower is faster; bold marks the lowest observed median in each row.**

No previous-version baseline is paired with this view.

| Workload | Kernix | ndarray | nalgebra | faer | Candle | NumPy |
|---|---:|---:|---:|---:|---:|---:|
| Matrix multiplication · 32 × 32 · 32 × 32 · f64 | 3.239 | **3.056** | 3.216 | 3.504 | 3.424 | 8.200 |
| Matrix multiplication · 64 × 64 · 64 × 64 · f64 | 22.174 | **21.799** | 22.463 | 22.631 | 22.574 | 31.118 |
| Matrix multiplication · 128 × 128 · 128 × 128 · f64 | 210.410 | 207.458 | 208.642 | 169.821 | **169.268** | 196.011 |
| Matrix multiplication · 256 × 256 · 256 × 256 · f64 | 1335.542 | 1326.502 | 1334.373 | 1548.982 | **1324.023** | 1466.035 |
| Matrix multiplication · 64 × 64 · 64 × 64 · f32 | 11.504 | **11.247** | 11.271 | 12.295 | 11.951 | 20.119 |
| Matrix multiplication · 128 × 128 · 128 × 128 · f32 | 94.506 | 93.594 | 94.962 | 87.051 | **85.819** | 100.065 |
| Matrix multiplication · 256 × 256 · 256 × 256 · f32 | 770.612 | 765.316 | 761.412 | **674.018** | 757.933 | 729.143 |
| Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64 | **3.856** | — | — | — | 4.448 | 12.975 |
| Column-pivoted QR · 96 × 32 · f64 | 88.146 | — | 114.304 | **61.988** | — | — |
| Column-pivoted QR · 256 × 64 · f64 | 786.763 | — | 1151.545 | **474.936** | — | — |
| Minimum-norm least squares · 64 × 32, rank 16, 4 RHS · f64 | **76.059** | — | 116.800 | — | — | 93.706 |
| Minimum-norm least squares · 32 × 64, rank 32, 4 RHS · f64 | **114.616** | — | 217.986 | — | — | 176.460 |
| Thin QR · 96 × 32 · f64 | 70.155 | — | **61.388** | 65.356 | — | 65.963 |
| Thin QR · 256 × 64 · f64 | 690.305 | — | 588.263 | **443.554** | — | 576.105 |
| Cholesky factorization · 64 × 64 · f64 | 27.334 | — | 20.350 | **13.256** | — | 18.031 |
| Cholesky factorization · 128 × 128 · f64 | 280.789 | — | 128.775 | **64.996** | — | 82.746 |
| Cholesky factorization · 256 × 256 · f64 | 1994.912 | — | 1031.706 | **407.037** | — | 751.805 |
| Row softmax · 256 × 64 · f64 | 113.000 | **97.266** | — | — | 111.553 | 134.894 |
| Create zeros · 64 × 64 · f32 | 0.184 | **0.136** | — | 0.328 | 0.256 | 0.512 |
| Create ones · 64 × 64 · f32 | 0.248 | **0.208** | — | 0.240 | 0.344 | 1.520 |
| Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64 | **3.512** | — | — | — | 7.128 | — |
| Model inference · batched normalized/sliced MLP · f64 | **3.087** | — | — | — | 5.279 | — |
| Model recording + forward · batched normalized/sliced MLP · f64 | **3.823** | — | — | — | 7.615 | — |
| Model backward · prepared graph → fresh gradients · f64 | **4.192** | — | — | — | 21.263 | — |
| Parameter update · transactional SGD, fixed gradients · f64 | **0.736** | — | — | — | — | — |

![Relative latency by workload](latency.png)

Each panel has its own scale. Ratios are library / Kernix median; below 1 means the comparator was faster.

## Reading these results

Minimum-norm rows include factorization and four RHS. Kernix uses pivoted/right QR; NumPy uses native lstsq; nalgebra uses native SVD/solve. Fixtures have a wide rank gap; null-space orthogonality and normal residual are validated independently. Algorithm costs and rank policies differ; see DETAILS.md for thresholds. These are not general SVD comparisons.

- One measured host and one BLAS/Rayon thread. CPU affinity pinned to logical CPU 0. Frequency and background load are uncontrolled; these observations establish no universal ranking.
- Timing includes output allocation and destruction. Inputs and imports are prepared outside timing. GEMM inputs are row-major; output layout and validation contracts can differ by library.
- Per-call p95 describes 256 individual warmed calls. The 15 batch means form a separate distribution; neither is a confidence interval.
- Rust allocation probes use a separate instrumented binary. Native engines may retain packing buffers; warmed probes exclude their first-use allocation and retention. NumPy native allocations are unavailable, never zero.

[Interactive report](report.html) · [Detailed measurements and method](DETAILS.md) · [Raw samples and provenance](raw.json) · [Printable chart](latency.pdf)

Measured source `9b7ef492c99bf2202e7eb3b07a79ef8f108991c7`. Run `runner-ubuntu-24.04-arm-39980084433`. Rendering does not remeasure or modify raw evidence.
