# Kernix 0.9.0 · CPU benchmarks

12th Gen Intel(R) Core(TM) i3-1215U · one BLAS/Rayon thread · allocating APIs unless marked reused · warmed individual calls · explicit 2-worker rows labelled

**Microseconds per call, median. Lower is faster; bold marks the lowest observed median in each row.**

No previous-version baseline is paired with this view.

| Workload | Kernix | ndarray | nalgebra | faer | Candle | NumPy |
|---|---:|---:|---:|---:|---:|---:|
| Matmul · 2 workers · reused output · 64 × 64 · 64 × 64 · f64 | **10.422** | — | — | — | — | — |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f64 | **105.080** | — | — | — | — | — |
| Matmul · 2 workers · reused output · 256 × 256 · 256 × 256 · f64 | **423.627** | — | — | — | — | — |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f32 | **29.303** | — | — | — | — | — |
| Coalesced tensor contraction · 16 × 32 × 8 · 32 × 8 × 16 · f64 | 691.795 | — | — | — | — | **12.340** |
| Shared-variable backward · sum(X * X), 64 × 32; prepared graph · f64 | 6.488 | — | — | — | **3.961** | — |
| Matrix multiplication · 32 × 32 · 32 × 32 · f64 | 1.687 | 1.526 | **1.481** | 1.561 | 1.574 | 2.788 |
| Matrix multiplication · 64 × 64 · 64 × 64 · f64 | 17.926 | 11.608 | 11.195 | **11.187** | 11.379 | 12.204 |
| Matrix multiplication · 128 × 128 · 128 × 128 · f64 | 83.207 | 121.834 | 120.325 | 86.726 | **78.204** | 82.343 |
| Matrix multiplication · 256 × 256 · 256 × 256 · f64 | 647.477 | 685.012 | 680.500 | 758.898 | **603.518** | 668.632 |
| Matrix multiplication · 64 × 64 · 64 × 64 · f32 | **5.081** | 6.127 | 6.239 | 5.624 | 5.767 | 7.539 |
| Matrix multiplication · 128 × 128 · 128 × 128 · f32 | **38.634** | 52.083 | 50.480 | 50.973 | 39.756 | 40.227 |
| Matrix multiplication · 256 × 256 · 256 × 256 · f32 | **294.932** | 410.973 | 412.324 | 343.122 | 304.379 | 301.429 |
| Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64 | **2.073** | — | — | — | 4.315 | 3.491 |
| Column-pivoted QR · 96 × 32 · f64 | **58.492** | — | 79.440 | 78.191 | — | — |
| Column-pivoted QR · 256 × 64 · f64 | 599.809 | — | 843.150 | **485.354** | — | — |
| Minimum-norm least squares · 64 × 32, rank 16, 4 RHS · f64 | **51.783** | — | 78.173 | — | — | 71.772 |
| Minimum-norm least squares · 32 × 64, rank 32, 4 RHS · f64 | **81.587** | — | 142.115 | — | — | 129.445 |
| Thin QR · 96 × 32 · f64 | 47.578 | — | **40.857** | 69.889 | — | 43.608 |
| Thin QR · 256 × 64 · f64 | 531.300 | — | 395.774 | 437.890 | — | **374.776** |
| Cholesky factorization · 64 × 64 · f64 | 21.573 | — | 15.131 | **8.936** | — | 11.778 |
| Cholesky factorization · 128 × 128 · f64 | 120.034 | — | 111.245 | **39.611** | — | 74.878 |
| Cholesky factorization · 256 × 256 · f64 | 685.247 | — | 752.003 | **206.834** | — | 545.514 |
| Row softmax · 256 × 64 · f64 | 93.662 | **85.808** | — | — | 93.798 | 110.451 |
| Create zeros · 64 × 64 · f32 | 0.156 | **0.090** | — | 0.160 | 0.176 | 0.396 |
| Create ones · 64 × 64 · f32 | 0.205 | **0.154** | — | 0.196 | 0.225 | 1.295 |
| Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64 | **2.518** | — | — | — | 4.364 | — |
| Model inference · batched normalized/sliced MLP · f64 | **2.572** | — | — | — | 3.880 | — |
| Model recording + forward · batched normalized/sliced MLP · f64 | **2.926** | — | — | — | 5.435 | — |
| Model backward · prepared graph → fresh gradients · f64 | **3.311** | — | — | — | 14.863 | — |
| Parameter update · transactional SGD, fixed gradients · f64 | **0.585** | — | — | — | — | — |

![Relative latency by workload](latency.png)

Each panel has its own scale. Ratios are library / Kernix median; below 1 means the comparator was faster.

## Reading these results

Minimum-norm rows include factorization and four RHS. Kernix uses pivoted QR plus scaled right QR; NumPy uses native lstsq; nalgebra uses native SVD/solve. Fixtures have a wide rank gap; null-space orthogonality and normal residual are validated independently. Algorithm costs and rank policies differ; see DETAILS.md for thresholds. These are not general SVD comparisons.

- One measured host and one BLAS/Rayon thread. CPU affinity pinned to logical CPU 0. Frequency and background load are uncontrolled; these observations establish no universal ranking. Rows marked 2 workers use a private two-thread executor and reusable output; their recorded CPU affinity can differ. They compare Kernix versions, not single-thread competitors.
- Timing includes output allocation and destruction except rows explicitly marked reused. Inputs and imports are prepared outside timing. GEMM inputs are row-major; output layout and validation contracts can differ by library.
- Per-call p95 describes 256 individual warmed calls. The 15 batch means form a separate distribution; neither is a confidence interval.
- Rust allocation probes use a separate instrumented binary. Native engines may retain packing buffers; warmed probes exclude their first-use allocation and retention. NumPy native allocations are unavailable, never zero.

[Interactive report](report.html) · [Detailed measurements and method](DETAILS.md) · [Raw samples and provenance](raw.json) · [Printable chart](latency.pdf)

Measured source `fcd9fdb97ff307e9c50d0526e8b4e1a580cb44a9`. Run `0.9.0-registry-010`. Rendering does not remeasure or modify raw evidence.
