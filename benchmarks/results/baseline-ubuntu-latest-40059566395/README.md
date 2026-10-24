# Kernix 0.10.0 · CPU benchmarks

AMD EPYC 7763 64-Core Processor · warmed public API calls · conditions per workload

**Microseconds per call, median. Lower is faster. Bold marks a lowest observed median only among at least two measured libraries.**

No previous-version baseline is paired with this view.

| Workload | Kernix | ndarray | nalgebra | faer | Candle | NumPy |
|---|---:|---:|---:|---:|---:|---:|
| Independent matrix operands · 128 × 128 · 128 × 128 · f64 | **93.836** | — | — | — | — | 102.021 |
| Independent matrix operands · 128 × 128 · 128 × 128 · f32 | **46.087** | — | — | — | — | 49.573 |
| Matrix multiplication · 32 × 32 · 32 × 32 · f64 | 2.515 | 2.184 | 2.124 | **2.034** | 2.164 | 3.778 |
| Matrix multiplication · 64 × 64 · 64 × 64 · f64 | **12.754** | 15.520 | 14.988 | 13.385 | 12.814 | 15.058 |
| Matrix multiplication · 128 × 128 · 128 × 128 · f64 | 93.866 | 204.092 | 200.324 | 100.428 | **93.174** | 101.881 |
| Matrix multiplication · 256 × 256 · 256 × 256 · f64 | **733.843** | 908.020 | 901.147 | 990.484 | 737.180 | 768.178 |
| Matrix multiplication · 64 × 64 · 64 × 64 · f32 | 6.522 | 7.494 | 7.464 | **6.502** | 6.723 | 8.135 |
| Matrix multiplication · 128 × 128 · 128 × 128 · f32 | 46.407 | 74.670 | 73.337 | 49.212 | **46.036** | 49.392 |
| Matrix multiplication · 256 × 256 · 256 × 256 · f32 | 356.397 | 597.340 | 600.475 | 408.114 | **352.610** | 362.749 |
| Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64 | **3.076** | — | — | — | 3.236 | 4.218 |
| Matmul · 2 workers · reused output · 64 × 64 · 64 × 64 · f64 | 18.845 | — | — | — | — | — |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f64 | 57.127 | — | — | — | — | — |
| Matmul · 2 workers · reused output · 256 × 256 · 256 × 256 · f64 | 392.625 | — | — | — | — | — |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f32 | 32.000 | — | — | — | — | — |
| Cholesky factorization · 512 × 512 · f64 | 7630.374 | — | 7353.676 | **2616.349** | — | 6224.590 |
| Column-pivoted QR · 96 × 32 · f64 | 78.427 | — | 78.367 | **43.021** | — | — |
| Column-pivoted QR · 256 × 64 · f64 | 851.164 | — | 762.537 | **345.727** | — | — |
| Minimum-norm least squares · 64 × 32, rank 16, 4 RHS · f64 | **62.467** | — | 103.223 | — | — | 102.713 |
| Minimum-norm least squares · 32 × 64, rank 32, 4 RHS · f64 | **84.669** | — | 178.555 | — | — | 185.417 |
| Thin QR · 96 × 32 · f64 | 66.805 | — | 58.880 | **54.001** | — | 69.781 |
| Thin QR · 256 × 64 · f64 | 779.228 | — | 569.847 | **418.002** | — | 537.937 |
| Cholesky factorization · 64 × 64 · f64 | 24.967 | — | 20.969 | **14.087** | — | 18.123 |
| Cholesky factorization · 128 × 128 · f64 | 169.017 | — | 119.263 | **57.919** | — | 108.032 |
| Cholesky factorization · 256 × 256 · f64 | 1013.949 | — | 790.921 | **312.905** | — | 1080.593 |
| Shared-variable backward · sum(X * X), 64 × 32; prepared graph · f64 | 10.540 | — | — | — | **4.949** | — |
| Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64 | **3.877** | — | — | — | 7.715 | — |
| Model inference · batched normalized/sliced MLP · f64 | **3.376** | — | — | — | 5.010 | — |
| Model recording + forward · batched normalized/sliced MLP · f64 | **4.168** | — | — | — | 6.773 | — |
| Model backward · prepared graph → fresh gradients · f64 | **4.969** | — | — | — | 20.859 | — |
| Parameter update · transactional SGD, fixed gradients · f64 | 0.802 | — | — | — | — | — |
| Coalesced tensor contraction · 16 × 32 × 8 · 32 × 8 × 16 · f64 | 63.689 | — | — | — | — | **11.091** |
| Row softmax · 256 × 64 · f64 | 119.564 | **113.523** | — | — | 115.285 | 133.509 |
| Create zeros · 64 × 64 · f32 | 0.321 | **0.241** | — | 0.330 | 0.301 | 0.742 |
| Create ones · 64 × 64 · f32 | 0.431 | **0.351** | — | 0.391 | 0.470 | 2.064 |

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

Measured source `18750693908110110d85d02d0f11af8d58bfae8b`. Run `baseline-ubuntu-latest-40059566395`. Rendering does not change measurements.
