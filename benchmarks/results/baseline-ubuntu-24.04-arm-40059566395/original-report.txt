# Kernix 0.10.0 · CPU benchmarks

Neoverse-N2 · warmed public API calls · conditions per workload

**Microseconds per call, median. Lower is faster. Bold marks a lowest observed median only among at least two measured libraries.**

No previous-version baseline is paired with this view.

| Workload | Kernix | ndarray | nalgebra | faer | Candle | NumPy |
|---|---:|---:|---:|---:|---:|---:|
| Independent matrix operands · 128 × 128 · 128 × 128 · f64 | 211.489 | — | — | — | — | **197.028** |
| Independent matrix operands · 128 × 128 · 128 × 128 · f32 | **94.587** | — | — | — | — | 100.017 |
| Matrix multiplication · 32 × 32 · 32 × 32 · f64 | 3.224 | **3.047** | 3.208 | 3.495 | 3.440 | 8.208 |
| Matrix multiplication · 64 × 64 · 64 × 64 · f64 | 22.175 | **21.775** | 22.447 | 22.638 | 22.694 | 31.126 |
| Matrix multiplication · 128 × 128 · 128 × 128 · f64 | 211.068 | 205.890 | 210.284 | 169.740 | **168.398** | 195.517 |
| Matrix multiplication · 256 × 256 · 256 × 256 · f64 | 1339.411 | **1328.265** | 1338.464 | 1550.759 | 1329.027 | 1476.766 |
| Matrix multiplication · 64 × 64 · 64 × 64 · f32 | 11.504 | **11.271** | 11.279 | 12.295 | 11.952 | 20.127 |
| Matrix multiplication · 128 × 128 · 128 × 128 · f32 | 94.099 | 93.634 | 93.585 | 86.835 | **85.954** | 99.801 |
| Matrix multiplication · 256 × 256 · 256 × 256 · f32 | 768.293 | 767.441 | 763.313 | **676.626** | 757.566 | 722.944 |
| Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64 | **3.824** | — | — | — | 4.439 | 12.983 |
| Matmul · 2 workers · reused output · 64 × 64 · 64 × 64 · f64 | 17.174 | — | — | — | — | — |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f64 | 97.891 | — | — | — | — | — |
| Matmul · 2 workers · reused output · 256 × 256 · 256 × 256 · f64 | 841.096 | — | — | — | — | — |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f32 | 49.933 | — | — | — | — | — |
| Cholesky factorization · 512 × 512 · f64 | 15318.115 | — | 8996.093 | **2665.544** | — | 3936.927 |
| Column-pivoted QR · 96 × 32 · f64 | 88.626 | — | 114.242 | **61.950** | — | — |
| Column-pivoted QR · 256 × 64 · f64 | 790.707 | — | 1151.077 | **473.822** | — | — |
| Minimum-norm least squares · 64 × 32, rank 16, 4 RHS · f64 | **72.597** | — | 117.073 | — | — | 93.658 |
| Minimum-norm least squares · 32 × 64, rank 32, 4 RHS · f64 | **96.235** | — | 219.113 | — | — | 176.895 |
| Thin QR · 96 × 32 · f64 | 71.739 | — | **61.413** | 65.355 | — | 66.630 |
| Thin QR · 256 × 64 · f64 | 694.486 | — | 587.694 | **447.704** | — | 573.582 |
| Cholesky factorization · 64 × 64 · f64 | 27.367 | — | 20.334 | **13.208** | — | 18.111 |
| Cholesky factorization · 128 × 128 · f64 | 282.148 | — | 128.991 | **64.939** | — | 83.980 |
| Cholesky factorization · 256 × 256 · f64 | 1988.423 | — | 1033.389 | **405.612** | — | 771.097 |
| Shared-variable backward · sum(X * X), 64 × 32; prepared graph · f64 | 8.343 | — | — | — | **5.031** | — |
| Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64 | **3.511** | — | — | — | 7.327 | — |
| Model inference · batched normalized/sliced MLP · f64 | **3.087** | — | — | — | 5.240 | — |
| Model recording + forward · batched normalized/sliced MLP · f64 | **3.840** | — | — | — | 7.575 | — |
| Model backward · prepared graph → fresh gradients · f64 | **4.184** | — | — | — | 21.135 | — |
| Parameter update · transactional SGD, fixed gradients · f64 | 0.744 | — | — | — | — | — |
| Coalesced tensor contraction · 16 × 32 × 8 · 32 × 8 × 16 · f64 | 46.693 | — | — | — | — | **16.439** |
| Row softmax · 256 × 64 · f64 | 113.033 | **97.441** | — | — | 110.490 | 135.007 |
| Create zeros · 64 × 64 · f32 | 0.200 | **0.136** | — | 0.368 | 0.232 | 0.504 |
| Create ones · 64 × 64 · f32 | 0.256 | **0.208** | — | 0.240 | 0.344 | 1.536 |

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

CPU affinity pinned to logical CPU 0. Frequency and background load are uncontrolled; observations establish no universal ranking.

Minimum-norm rows use pivoted QR plus scaled implicit RZ in Kernix; NumPy native lstsq; nalgebra native SVD/solve. Algorithm costs and rank policies differ. Normal residuals and null-space orthogonality are independently checked. See DETAILS.md for thresholds.

Ordinary allocating rows include output allocation and destruction. Explicit two-worker rows reuse output and exclude pool preparation. NumPy native allocations are unavailable, never zero. Warmed probes exclude first-use native workspace and retention.

[Interactive report](report.html) · [Full method](DETAILS.md) · [Raw samples](raw.json) · [Printable family charts](latency.pdf)

Measured source `18750693908110110d85d02d0f11af8d58bfae8b`. Run `baseline-ubuntu-24.04-arm-40059566395`. Rendering does not change measurements.
