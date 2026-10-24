# Kernix 0.10.0 · CPU benchmarks

Apple M1 (Virtual) · warmed public API calls · conditions per workload

**Microseconds per call, median. Lower is faster. Bold marks a lowest observed median only among at least two measured libraries.**

No previous-version baseline is paired with this view.

| Workload | Kernix | ndarray | nalgebra | faer | Candle | NumPy |
|---|---:|---:|---:|---:|---:|---:|
| Independent matrix operands · 128 × 128 · 128 × 128 · f64 | 100.417 | — | — | — | — | **16.000** |
| Independent matrix operands · 128 × 128 · 128 × 128 · f32 | 63.375 | — | — | — | — | **5.209** |
| Matrix multiplication · 32 × 32 · 32 × 32 · f64 | 2.750 | 6.000 | 2.167 | 2.417 | 2.750 | **1.292** |
| Matrix multiplication · 64 × 64 · 64 × 64 · f64 | 13.417 | 13.750 | 12.667 | 14.708 | 13.584 | **3.500** |
| Matrix multiplication · 128 × 128 · 128 × 128 · f64 | 95.750 | 97.583 | 113.208 | 101.208 | 98.208 | **16.125** |
| Matrix multiplication · 256 × 256 · 256 × 256 · f64 | 854.500 | 853.417 | 772.958 | 887.584 | 790.792 | **123.083** |
| Matrix multiplication · 64 × 64 · 64 × 64 · f32 | 6.750 | 6.667 | 7.250 | 10.625 | 8.000 | **1.750** |
| Matrix multiplication · 128 × 128 · 128 × 128 · f32 | 50.625 | 51.459 | 49.625 | 55.458 | 65.208 | **6.334** |
| Matrix multiplication · 256 × 256 · 256 × 256 · f32 | 405.709 | 422.250 | 410.542 | 422.917 | 373.459 | **34.750** |
| Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64 | 2.542 | — | — | — | 3.791 | **2.167** |
| Matmul · 2 workers · reused output · 64 × 64 · 64 × 64 · f64 | 38.250 | — | — | — | — | — |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f64 | 141.291 | — | — | — | — | — |
| Matmul · 2 workers · reused output · 256 × 256 · 256 × 256 · f64 | 454.917 | — | — | — | — | — |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f32 | 52.375 | — | — | — | — | — |
| Cholesky factorization · 512 × 512 · f64 | 9378.625 | — | 5356.917 | **1815.166** | — | 2404.042 |
| Column-pivoted QR · 96 × 32 · f64 | 68.833 | — | 92.958 | **39.791** | — | — |
| Column-pivoted QR · 256 × 64 · f64 | 597.625 | — | 1054.208 | **327.292** | — | — |
| Minimum-norm least squares · 64 × 32, rank 16, 4 RHS · f64 | **45.750** | — | 75.833 | — | — | 79.875 |
| Minimum-norm least squares · 32 × 64, rank 32, 4 RHS · f64 | **66.375** | — | 152.375 | — | — | 143.292 |
| Thin QR · 96 × 32 · f64 | 57.417 | — | **44.917** | 45.041 | — | 52.958 |
| Thin QR · 256 × 64 · f64 | 557.541 | — | 359.708 | **332.917** | — | 416.083 |
| Cholesky factorization · 64 × 64 · f64 | 20.167 | — | 22.000 | **11.084** | — | 16.084 |
| Cholesky factorization · 128 × 128 · f64 | 197.250 | — | 99.083 | **50.042** | — | 51.125 |
| Cholesky factorization · 256 × 256 · f64 | 1116.750 | — | 696.042 | **266.584** | — | 309.542 |
| Shared-variable backward · sum(X * X), 64 × 32; prepared graph · f64 | 8.000 | — | — | — | **4.000** | — |
| Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64 | **3.292** | — | — | — | 7.208 | — |
| Model inference · batched normalized/sliced MLP · f64 | **2.417** | — | — | — | 5.708 | — |
| Model recording + forward · batched normalized/sliced MLP · f64 | **3.208** | — | — | — | 6.375 | — |
| Model backward · prepared graph → fresh gradients · f64 | **3.209** | — | — | — | 18.958 | — |
| Parameter update · transactional SGD, fixed gradients · f64 | 0.708 | — | — | — | — | — |
| Coalesced tensor contraction · 16 × 32 × 8 · 32 × 8 × 16 · f64 | 62.625 | — | — | — | — | **5.583** |
| Row softmax · 256 × 64 · f64 | 85.958 | **65.208** | — | — | 76.250 | 74.416 |
| Create zeros · 64 × 64 · f32 | 0.292 | **0.250** | — | 0.291 | 0.375 | 0.625 |
| Create ones · 64 × 64 · f32 | **0.333** | **0.333** | — | 0.792 | 0.417 | 1.084 |

## Version comparison

0 of 34 workloads have paired evidence. Intervals resample process pairs, not calls. Exploratory; no multiple-comparison adjustment.

| Workload | Paired latency ratio [95% interval] | Interpretation |
|---|---:|---|
| Independent matrix operands · 128 × 128 · 128 × 128 · f64 | — | No paired comparison |
| Independent matrix operands · 128 × 128 · 128 × 128 · f32 | — | No paired comparison |
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
| Cholesky factorization · 512 × 512 · f64 | — | No paired comparison |
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

## Method and limits

CPU affinity was not pinned. Frequency and background load are uncontrolled; observations establish no universal ranking.

Minimum-norm rows use pivoted QR plus scaled implicit RZ in Kernix; NumPy native lstsq; nalgebra native SVD/solve. Algorithm costs and rank policies differ. Normal residuals and null-space orthogonality are independently checked. See DETAILS.md for thresholds.

Ordinary allocating rows include output allocation and destruction. Explicit two-worker rows reuse output and exclude pool preparation. NumPy native allocations are unavailable, never zero. Warmed probes exclude first-use native workspace and retention.

[Interactive report](report.html) · [Full method](DETAILS.md) · [Raw samples](raw.json) · [Printable family charts](latency.pdf)

Measured source `18750693908110110d85d02d0f11af8d58bfae8b`. Run `baseline-macos-latest-40059566395`. Rendering does not change measurements.
