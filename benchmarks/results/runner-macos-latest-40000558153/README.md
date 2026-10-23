# Kernix 0.10.0 · CPU benchmarks

Apple M1 (Virtual) · one BLAS/Rayon thread · allocating APIs unless marked reused · warmed individual calls · explicit 2-worker rows labelled

**Microseconds per call, median. Lower is faster; bold marks the lowest observed median in each row.**

No previous-version baseline is paired with this view.

| Workload | Kernix | ndarray | nalgebra | faer | Candle | NumPy |
|---|---:|---:|---:|---:|---:|---:|
| Matmul · 2 workers · reused output · 64 × 64 · 64 × 64 · f64 | **24.250** | — | — | — | — | — |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f64 | **89.500** | — | — | — | — | — |
| Matmul · 2 workers · reused output · 256 × 256 · 256 × 256 · f64 | **518.917** | — | — | — | — | — |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f32 | **53.500** | — | — | — | — | — |
| Coalesced tensor contraction · 16 × 32 × 8 · 32 × 8 × 16 · f64 | 61.875 | — | — | — | — | **5.708** |
| Shared-variable backward · sum(X * X), 64 × 32; prepared graph · f64 | 6.917 | — | — | — | **3.959** | — |
| Matrix multiplication · 32 × 32 · 32 × 32 · f64 | 2.250 | 2.417 | 2.000 | 2.417 | 2.584 | **1.333** |
| Matrix multiplication · 64 × 64 · 64 × 64 · f64 | 13.167 | 13.250 | 13.542 | 14.709 | 14.500 | **3.250** |
| Matrix multiplication · 128 × 128 · 128 × 128 · f64 | 102.541 | 101.958 | 102.458 | 106.083 | 95.750 | **19.250** |
| Matrix multiplication · 256 × 256 · 256 × 256 · f64 | 751.583 | 802.791 | 786.584 | 838.875 | 820.417 | **110.583** |
| Matrix multiplication · 64 × 64 · 64 × 64 · f32 | 7.250 | 6.917 | 6.958 | 8.208 | 7.833 | **1.833** |
| Matrix multiplication · 128 × 128 · 128 × 128 · f32 | 48.875 | 48.667 | 47.791 | 61.292 | 52.959 | **5.542** |
| Matrix multiplication · 256 × 256 · 256 × 256 · f32 | 392.750 | 391.375 | 406.916 | 414.000 | 468.417 | **29.500** |
| Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64 | 2.834 | — | — | — | 3.792 | **2.000** |
| Column-pivoted QR · 96 × 32 · f64 | 67.959 | — | 92.042 | **62.000** | — | — |
| Column-pivoted QR · 256 × 64 · f64 | 530.291 | — | 983.750 | **321.250** | — | — |
| Minimum-norm least squares · 64 × 32, rank 16, 4 RHS · f64 | **49.792** | — | 72.292 | — | — | 77.917 |
| Minimum-norm least squares · 32 × 64, rank 32, 4 RHS · f64 | **68.542** | — | 145.042 | — | — | 138.459 |
| Thin QR · 96 × 32 · f64 | 54.875 | — | **39.500** | 42.917 | — | 50.167 |
| Thin QR · 256 × 64 · f64 | 526.250 | — | 361.417 | **285.125** | — | 345.667 |
| Cholesky factorization · 64 × 64 · f64 | 21.250 | — | 21.000 | **10.459** | — | 15.000 |
| Cholesky factorization · 128 × 128 · f64 | 172.250 | — | 100.834 | **48.208** | — | 48.250 |
| Cholesky factorization · 256 × 256 · f64 | 1249.167 | — | 702.708 | **252.416** | — | 316.791 |
| Row softmax · 256 × 64 · f64 | 78.041 | **64.167** | — | — | 80.167 | 68.833 |
| Create zeros · 64 × 64 · f32 | **0.250** | 0.333 | — | **0.250** | 0.416 | 0.500 |
| Create ones · 64 × 64 · f32 | **0.333** | **0.333** | — | **0.333** | 0.375 | 1.125 |
| Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64 | **2.959** | — | — | — | 6.458 | — |
| Model inference · batched normalized/sliced MLP · f64 | **2.333** | — | — | — | 6.208 | — |
| Model recording + forward · batched normalized/sliced MLP · f64 | **3.000** | — | — | — | 6.125 | — |
| Model backward · prepared graph → fresh gradients · f64 | **3.542** | — | — | — | 20.292 | — |
| Parameter update · transactional SGD, fixed gradients · f64 | **0.708** | — | — | — | — | — |

![Relative latency by workload](latency.png)

Each panel has its own scale. Ratios are library / Kernix median; below 1 means the comparator was faster.

## Reading these results

Minimum-norm rows include factorization and four RHS. Kernix uses pivoted QR plus scaled implicit RZ; NumPy uses native lstsq; nalgebra uses native SVD/solve. Fixtures have a wide rank gap; null-space orthogonality and normal residual are validated independently. Algorithm costs and rank policies differ; see DETAILS.md for thresholds. These are not general SVD comparisons.

- One measured host and one BLAS/Rayon thread. CPU affinity was not pinned. Frequency and background load are uncontrolled; these observations establish no universal ranking. Rows marked 2 workers use a private two-thread executor and reusable output; their recorded CPU affinity can differ. They compare Kernix versions, not single-thread competitors.
- Timing includes output allocation and destruction except rows explicitly marked reused. Inputs and imports are prepared outside timing. GEMM inputs are row-major; output layout and validation contracts can differ by library.
- Per-call p95 describes 256 individual warmed calls. The 15 batch means form a separate distribution; neither is a confidence interval.
- Rust allocation probes use a separate instrumented binary. Native engines may retain packing buffers; warmed probes exclude their first-use allocation and retention. NumPy native allocations are unavailable, never zero.

[Interactive report](report.html) · [Detailed measurements and method](DETAILS.md) · [Raw samples and provenance](raw.json) · [Printable chart](latency.pdf)

Measured source `1c53d4663a48de5a588809c2b69c3a6a5bdad7b2`. Run `runner-macos-latest-40000558153`. Rendering does not remeasure or modify raw evidence.
