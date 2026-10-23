# Kernix 0.10.0 · CPU benchmarks

AMD EPYC 7763 64-Core Processor · one BLAS/Rayon thread · allocating APIs unless marked reused · warmed individual calls · explicit 2-worker rows labelled

**Microseconds per call, median. Lower is faster; bold marks the lowest observed median in each row.**

No previous-version baseline is paired with this view.

| Workload | Kernix | ndarray | nalgebra | faer | Candle | NumPy |
|---|---:|---:|---:|---:|---:|---:|
| Matmul · 2 workers · reused output · 64 × 64 · 64 × 64 · f64 | **16.601** | — | — | — | — | — |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f64 | **57.177** | — | — | — | — | — |
| Matmul · 2 workers · reused output · 256 × 256 · 256 × 256 · f64 | **395.459** | — | — | — | — | — |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f32 | **31.619** | — | — | — | — | — |
| Coalesced tensor contraction · 16 × 32 × 8 · 32 × 8 × 16 · f64 | 63.479 | — | — | — | — | **11.261** |
| Shared-variable backward · sum(X * X), 64 × 32; prepared graph · f64 | 10.219 | — | — | — | **4.869** | — |
| Matrix multiplication · 32 × 32 · 32 × 32 · f64 | 2.505 | 2.184 | 2.113 | **2.054** | 2.174 | 3.657 |
| Matrix multiplication · 64 × 64 · 64 × 64 · f64 | **12.834** | 15.539 | 14.948 | 13.625 | 12.974 | 14.998 |
| Matrix multiplication · 128 × 128 · 128 × 128 · f64 | **93.655** | 205.072 | 202.298 | 100.477 | 93.715 | 101.269 |
| Matrix multiplication · 256 × 256 · 256 × 256 · f64 | **732.798** | 908.375 | 902.021 | 970.091 | 740.312 | 766.902 |
| Matrix multiplication · 64 × 64 · 64 × 64 · f32 | 6.602 | 7.534 | 7.354 | **6.582** | 6.592 | 8.135 |
| Matrix multiplication · 128 × 128 · 128 × 128 · f32 | 46.327 | 74.980 | 73.387 | 51.026 | **46.025** | 49.122 |
| Matrix multiplication · 256 × 256 · 256 × 256 · f32 | 355.824 | 603.857 | 597.285 | 413.121 | **355.113** | 364.320 |
| Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64 | **3.086** | — | — | — | 3.276 | 4.197 |
| Column-pivoted QR · 96 × 32 · f64 | 78.687 | — | 76.964 | **43.501** | — | — |
| Column-pivoted QR · 256 × 64 · f64 | 851.550 | — | 756.301 | **346.206** | — | — |
| Minimum-norm least squares · 64 × 32, rank 16, 4 RHS · f64 | **62.777** | — | 103.583 | — | — | 102.742 |
| Minimum-norm least squares · 32 × 64, rank 32, 4 RHS · f64 | **86.080** | — | 178.463 | — | — | 183.693 |
| Thin QR · 96 × 32 · f64 | 65.522 | — | 57.677 | **54.903** | — | 69.850 |
| Thin QR · 256 × 64 · f64 | 770.688 | — | 557.621 | **417.629** | — | 537.844 |
| Cholesky factorization · 64 × 64 · f64 | 25.017 | — | 20.218 | **14.177** | — | 18.144 |
| Cholesky factorization · 128 × 128 · f64 | 170.349 | — | 120.695 | **58.149** | — | 106.949 |
| Cholesky factorization · 256 × 256 · f64 | 1029.482 | — | 821.313 | **312.673** | — | 1050.872 |
| Row softmax · 256 × 64 · f64 | 120.205 | **111.358** | — | — | 115.445 | 132.688 |
| Create zeros · 64 × 64 · f32 | 0.320 | **0.240** | — | 0.341 | 0.301 | 0.731 |
| Create ones · 64 × 64 · f32 | 0.431 | **0.361** | — | 0.391 | 0.461 | 2.084 |
| Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64 | **4.107** | — | — | — | 7.584 | — |
| Model inference · batched normalized/sliced MLP · f64 | **3.236** | — | — | — | 4.979 | — |
| Model recording + forward · batched normalized/sliced MLP · f64 | **4.147** | — | — | — | 6.202 | — |
| Model backward · prepared graph → fresh gradients · f64 | **5.139** | — | — | — | 20.879 | — |
| Parameter update · transactional SGD, fixed gradients · f64 | **0.801** | — | — | — | — | — |

![Relative latency by workload](latency.png)

Each panel has its own scale. Ratios are library / Kernix median; below 1 means the comparator was faster.

## Reading these results

Minimum-norm rows include factorization and four RHS. Kernix uses pivoted QR plus scaled implicit RZ; NumPy uses native lstsq; nalgebra uses native SVD/solve. Fixtures have a wide rank gap; null-space orthogonality and normal residual are validated independently. Algorithm costs and rank policies differ; see DETAILS.md for thresholds. These are not general SVD comparisons.

- One measured host and one BLAS/Rayon thread. CPU affinity pinned to logical CPU 0. Frequency and background load are uncontrolled; these observations establish no universal ranking. Rows marked 2 workers use a private two-thread executor and reusable output; their recorded CPU affinity can differ. They compare Kernix versions, not single-thread competitors.
- Timing includes output allocation and destruction except rows explicitly marked reused. Inputs and imports are prepared outside timing. GEMM inputs are row-major; output layout and validation contracts can differ by library.
- Per-call p95 describes 256 individual warmed calls. The 15 batch means form a separate distribution; neither is a confidence interval.
- Rust allocation probes use a separate instrumented binary. Native engines may retain packing buffers; warmed probes exclude their first-use allocation and retention. NumPy native allocations are unavailable, never zero.

[Interactive report](report.html) · [Detailed measurements and method](DETAILS.md) · [Raw samples and provenance](raw.json) · [Printable chart](latency.pdf)

Measured source `1c53d4663a48de5a588809c2b69c3a6a5bdad7b2`. Run `runner-ubuntu-latest-40000558153`. Rendering does not remeasure or modify raw evidence.
