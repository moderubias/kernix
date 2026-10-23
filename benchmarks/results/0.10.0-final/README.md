# Kernix 0.10.0 · CPU benchmarks

12th Gen Intel(R) Core(TM) i3-1215U · one BLAS/Rayon thread · allocating APIs unless marked reused · warmed individual calls · explicit 2-worker rows labelled

**Microseconds per call, median. Lower is faster; bold marks the lowest observed median in each row.**

Baseline: 0.9.0-registry-010. Δ is latency change; negative is faster. Runs are sequential, not randomized/interleaved.

| Workload | Kernix | ndarray | nalgebra | faer | Candle | NumPy | Kernix Δ vs baseline |
|---|---:|---:|---:|---:|---:|---:|---:|
| Matmul · 2 workers · reused output · 64 × 64 · 64 × 64 · f64 | **9.832** | — | — | — | — | — | -5.7% |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f64 | **81.816** | — | — | — | — | — | -22.1% |
| Matmul · 2 workers · reused output · 256 × 256 · 256 × 256 · f64 | **307.403** | — | — | — | — | — | -27.4% |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f32 | **22.644** | — | — | — | — | — | -22.7% |
| Coalesced tensor contraction · 16 × 32 × 8 · 32 × 8 × 16 · f64 | 39.133 | — | — | — | — | **8.329** | -94.3% |
| Shared-variable backward · sum(X * X), 64 × 32; prepared graph · f64 | 6.501 | — | — | — | **4.158** | — | +0.2% |
| Matrix multiplication · 32 × 32 · 32 × 32 · f64 | 1.837 | **1.463** | 1.498 | 1.528 | 1.582 | 2.746 | +8.9% |
| Matrix multiplication · 64 × 64 · 64 × 64 · f64 | 10.905 | 11.592 | 10.727 | 11.511 | **10.449** | 12.507 | -39.2% |
| Matrix multiplication · 128 × 128 · 128 × 128 · f64 | 84.128 | 120.902 | 122.242 | 88.627 | 83.928 | **79.292** | +1.1% |
| Matrix multiplication · 256 × 256 · 256 × 256 · f64 | 650.087 | 683.995 | 693.950 | 880.524 | **606.776** | 667.864 | +0.4% |
| Matrix multiplication · 64 × 64 · 64 × 64 · f32 | **5.146** | 6.229 | 6.191 | 5.562 | 5.410 | 6.525 | +1.3% |
| Matrix multiplication · 128 × 128 · 128 × 128 · f32 | 40.614 | 51.054 | 52.313 | 50.298 | 39.425 | **39.237** | +5.1% |
| Matrix multiplication · 256 × 256 · 256 × 256 · f32 | 301.732 | 414.735 | 415.266 | 345.495 | **300.116** | 302.060 | +2.3% |
| Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64 | **2.204** | — | — | — | 2.439 | 3.399 | +6.3% |
| Column-pivoted QR · 96 × 32 · f64 | **58.549** | — | 81.493 | 77.385 | — | — | +0.1% |
| Column-pivoted QR · 256 × 64 · f64 | 610.066 | — | 807.347 | **480.146** | — | — | +1.7% |
| Minimum-norm least squares · 64 × 32, rank 16, 4 RHS · f64 | **48.841** | — | 78.119 | — | — | 69.879 | -5.7% |
| Minimum-norm least squares · 32 × 64, rank 32, 4 RHS · f64 | **61.880** | — | 140.636 | — | — | 129.431 | -24.2% |
| Thin QR · 96 × 32 · f64 | 49.589 | — | **39.704** | 69.955 | — | 42.526 | +4.2% |
| Thin QR · 256 × 64 · f64 | 539.290 | — | 388.185 | 431.156 | — | **375.857** | +1.5% |
| Cholesky factorization · 64 × 64 · f64 | 23.868 | — | 14.759 | **9.006** | — | 11.489 | +10.6% |
| Cholesky factorization · 128 × 128 · f64 | 124.276 | — | 110.192 | **40.517** | — | 75.570 | +3.5% |
| Cholesky factorization · 256 × 256 · f64 | 670.643 | — | 748.822 | **228.287** | — | 534.461 | -2.1% |
| Row softmax · 256 × 64 · f64 | 93.964 | **85.585** | — | — | 89.845 | 110.693 | +0.3% |
| Create zeros · 64 × 64 · f32 | 0.141 | **0.092** | — | 0.201 | 0.172 | 0.389 | -9.6% |
| Create ones · 64 × 64 · f32 | 0.196 | **0.154** | — | 0.193 | 0.231 | 1.406 | -4.4% |
| Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64 | **2.582** | — | — | — | 4.581 | — | +2.5% |
| Model inference · batched normalized/sliced MLP · f64 | **2.334** | — | — | — | 3.932 | — | -9.3% |
| Model recording + forward · batched normalized/sliced MLP · f64 | **2.774** | — | — | — | 5.592 | — | -5.2% |
| Model backward · prepared graph → fresh gradients · f64 | **3.194** | — | — | — | 15.595 | — | -3.5% |
| Parameter update · transactional SGD, fixed gradients · f64 | **0.584** | — | — | — | — | — | -0.2% |

![Relative latency by workload](latency.png)

Each panel has its own scale. Ratios are library / Kernix median; below 1 means the comparator was faster.

## Reading these results

Minimum-norm rows include factorization and four RHS. Kernix uses pivoted QR plus scaled implicit RZ; NumPy uses native lstsq; nalgebra uses native SVD/solve. Fixtures have a wide rank gap; null-space orthogonality and normal residual are validated independently. Algorithm costs and rank policies differ; see DETAILS.md for thresholds. These are not general SVD comparisons.

- One measured host and one BLAS/Rayon thread. CPU affinity pinned to logical CPU 0. Frequency and background load are uncontrolled; these observations establish no universal ranking. Rows marked 2 workers use a private two-thread executor and reusable output; their recorded CPU affinity can differ. They compare Kernix versions, not single-thread competitors.
- Timing includes output allocation and destruction except rows explicitly marked reused. Inputs and imports are prepared outside timing. GEMM inputs are row-major; output layout and validation contracts can differ by library.
- Per-call p95 describes 256 individual warmed calls. The 15 batch means form a separate distribution; neither is a confidence interval.
- Rust allocation probes use a separate instrumented binary. Native engines may retain packing buffers; warmed probes exclude their first-use allocation and retention. NumPy native allocations are unavailable, never zero.

[Interactive report](report.html) · [Detailed measurements and method](DETAILS.md) · [Raw samples and provenance](raw.json) · [Printable chart](latency.pdf)

Measured source `fcd9fdb97ff307e9c50d0526e8b4e1a580cb44a9`. Run `0.10.0-clean`. Rendering does not remeasure or modify raw evidence.
