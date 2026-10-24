# Kernix 0.10.0 · performance brief

AMD EPYC 7763 64-Core Processor / Linux

**Per-call median, microseconds per operation. Lower is faster. All recorded digits retained.**

31 workloads · 108 observations

No previous-version baseline attached. Sequential run changes are descriptive; paired process evidence is shown separately.

[Open the visual report](report.html) · [Vector PDF](latency.pdf) · [Raw measurements](raw.json)

[![Performance brief preview](latency.png)](latency.pdf)

| Workload | Kernix | ndarray | nalgebra | faer | Candle | NumPy |
|---|---:|---:|---:|---:|---:|---:|
| Matrix multiplication · 32 × 32 · 32 × 32 · f64 | 2.505 | 2.184 | 2.113 | **2.054** | 2.174 | 3.657 |
| Matrix multiplication · 64 × 64 · 64 × 64 · f64 | **12.834** | 15.539000000000002 | 14.948 | 13.625 | 12.974 | 14.998 |
| Matrix multiplication · 128 × 128 · 128 × 128 · f64 | **93.655** | 205.072 | 202.298 | 100.477 | 93.715 | 101.269 |
| Matrix multiplication · 256 × 256 · 256 × 256 · f64 | **732.798** | 908.375 | 902.021 | 970.091 | 740.312 | 766.902 |
| Matrix multiplication · 64 × 64 · 64 × 64 · f32 | 6.602 | 7.534 | 7.354 | **6.582** | 6.592 | 8.135 |
| Matrix multiplication · 128 × 128 · 128 × 128 · f32 | 46.327 | 74.98 | 73.387 | 51.026 | **46.025** | 49.122 |
| Matrix multiplication · 256 × 256 · 256 × 256 · f32 | 355.824 | 603.857 | 597.285 | 413.121 | **355.113** | 364.32 |
| Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64 | **3.086** | — | — | — | 3.276 | 4.197 |
| Matmul · 2 workers · reused output · 64 × 64 · 64 × 64 · f64 | 16.601 | — | — | — | — | — |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f64 | 57.177 | — | — | — | — | — |
| Matmul · 2 workers · reused output · 256 × 256 · 256 × 256 · f64 | 395.459 | — | — | — | — | — |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f32 | 31.619000000000004 | — | — | — | — | — |
| Column-pivoted QR · 96 × 32 · f64 | 78.687 | — | 76.964 | **43.501** | — | — |
| Column-pivoted QR · 256 × 64 · f64 | 851.55 | — | 756.301 | **346.206** | — | — |
| Minimum-norm least squares · 64 × 32, rank 16, 4 RHS · f64 | **62.777** | — | 103.583 | — | — | 102.742 |
| Minimum-norm least squares · 32 × 64, rank 32, 4 RHS · f64 | **86.08** | — | 178.463 | — | — | 183.693 |
| Thin QR · 96 × 32 · f64 | 65.522 | — | 57.677 | **54.903** | — | 69.85 |
| Thin QR · 256 × 64 · f64 | 770.688 | — | 557.621 | **417.629** | — | 537.844 |
| Cholesky factorization · 64 × 64 · f64 | 25.017 | — | 20.218 | **14.177** | — | 18.144 |
| Cholesky factorization · 128 × 128 · f64 | 170.349 | — | 120.695 | **58.149** | — | 106.949 |
| Cholesky factorization · 256 × 256 · f64 | 1029.482 | — | 821.313 | **312.673** | — | 1050.872 |
| Shared-variable backward · sum(X * X), 64 × 32; prepared graph · f64 | 10.219 | — | — | — | **4.869** | — |
| Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64 | **4.107** | — | — | — | 7.584 | — |
| Model inference · batched normalized/sliced MLP · f64 | **3.236** | — | — | — | 4.979 | — |
| Model recording + forward · batched normalized/sliced MLP · f64 | **4.147** | — | — | — | 6.202 | — |
| Model backward · prepared graph → fresh gradients · f64 | **5.139** | — | — | — | 20.879 | — |
| Parameter update · transactional SGD, fixed gradients · f64 | 0.801 | — | — | — | — | — |
| Coalesced tensor contraction · 16 × 32 × 8 · 32 × 8 × 16 · f64 | 63.47900000000001 | — | — | — | — | **11.261** |
| Row softmax · 256 × 64 · f64 | 120.205 | **111.358** | — | — | 115.445 | 132.688 |
| Create zeros · 64 × 64 · f32 | 0.32 | **0.23999999999999997** | — | 0.341 | 0.301 | 0.731 |
| Create ones · 64 × 64 · f32 | 0.431 | **0.361** | — | 0.391 | 0.461 | 2.084 |

## Version comparison

0 of 31 workloads have paired evidence. Exploratory 95% intervals resample process pairs, not calls. No multiple-comparison adjustment.

| Workload | Current / baseline [95% interval] | Interpretation |
|---|---:|---|
| Matrix multiplication · 32 × 32 · 32 × 32 · f64 | — | No paired comparison |
| Matrix multiplication · 64 × 64 · 64 × 64 · f64 | — | No paired comparison |
| Matrix multiplication · 128 × 128 · 128 × 128 · f64 | — | No paired comparison |
| Matrix multiplication · 256 × 256 · 256 × 256 · f64 | — | No paired comparison |
| Matrix multiplication · 64 × 64 · 64 × 64 · f32 | — | No paired comparison |
| Matrix multiplication · 128 × 128 · 128 × 128 · f32 | — | No paired comparison |
| Matrix multiplication · 256 × 256 · 256 × 256 · f32 | — | No paired comparison |
| Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64 | — | No paired comparison |
| Matmul · 2 workers · reused output · 64 × 64 · 64 × 64 · f64 | — | No paired comparison |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f64 | — | No paired comparison |
| Matmul · 2 workers · reused output · 256 × 256 · 256 × 256 · f64 | — | No paired comparison |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f32 | — | No paired comparison |
| Column-pivoted QR · 96 × 32 · f64 | — | No paired comparison |
| Column-pivoted QR · 256 × 64 · f64 | — | No paired comparison |
| Minimum-norm least squares · 64 × 32, rank 16, 4 RHS · f64 | — | No paired comparison |
| Minimum-norm least squares · 32 × 64, rank 32, 4 RHS · f64 | — | No paired comparison |
| Thin QR · 96 × 32 · f64 | — | No paired comparison |
| Thin QR · 256 × 64 · f64 | — | No paired comparison |
| Cholesky factorization · 64 × 64 · f64 | — | No paired comparison |
| Cholesky factorization · 128 × 128 · f64 | — | No paired comparison |
| Cholesky factorization · 256 × 256 · f64 | — | No paired comparison |
| Shared-variable backward · sum(X * X), 64 × 32; prepared graph · f64 | — | No paired comparison |
| Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64 | — | No paired comparison |
| Model inference · batched normalized/sliced MLP · f64 | — | No paired comparison |
| Model recording + forward · batched normalized/sliced MLP · f64 | — | No paired comparison |
| Model backward · prepared graph → fresh gradients · f64 | — | No paired comparison |
| Parameter update · transactional SGD, fixed gradients · f64 | — | No paired comparison |
| Coalesced tensor contraction · 16 × 32 × 8 · 32 × 8 × 16 · f64 | — | No paired comparison |
| Row softmax · 256 × 64 · f64 | — | No paired comparison |
| Create zeros · 64 × 64 · f32 | — | No paired comparison |
| Create ones · 64 × 64 · f32 | — | No paired comparison |

## Method and provenance

CPU affinity pinned to logical CPU 0. Frequency and background load are uncontrolled.

Minimum-norm rows use pivoted QR plus scaled implicit RZ in Kernix; NumPy native lstsq; nalgebra native SVD/solve. Algorithm costs and rank policies differ. See original method for thresholds.

Numbers retain source precision; recorded digits do not imply equivalent physical measurement accuracy. Lowest observed medians are bold only when peers were measured. No aggregate winner or universal ranking is inferred.

[Original report and commentary](original-report.txt) · [Raw measurements](raw.json)

Measured source `1c53d4663a48de5a588809c2b69c3a6a5bdad7b2`. Run `runner-ubuntu-latest-40000558153`. Rendering does not change measurements.
