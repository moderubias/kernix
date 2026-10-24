# Kernix 0.10.0 · performance brief

AMD EPYC 7763 64-Core Processor / Linux

**Per-call median, microseconds per operation. Lower is faster. All recorded digits retained.**

34 workloads · 116 observations

No previous-version baseline attached. Sequential run changes are descriptive; paired process evidence is shown separately.

[Open the visual report](report.html) · [Vector PDF](latency.pdf) · [Raw measurements](raw.json)

[![Performance brief preview](latency.png)](latency.pdf)

| Workload | Kernix | ndarray | nalgebra | faer | Candle | NumPy |
|---|---:|---:|---:|---:|---:|---:|
| Independent matrix operands · 128 × 128 · 128 × 128 · f64 | **93.836** | — | — | — | — | 102.021 |
| Independent matrix operands · 128 × 128 · 128 × 128 · f32 | **46.087** | — | — | — | — | 49.573 |
| Matrix multiplication · 32 × 32 · 32 × 32 · f64 | 2.515 | 2.184 | 2.124 | **2.034** | 2.164 | 3.778 |
| Matrix multiplication · 64 × 64 · 64 × 64 · f64 | **12.754** | 15.52 | 14.988 | 13.385 | 12.814 | 15.058 |
| Matrix multiplication · 128 × 128 · 128 × 128 · f64 | 93.866 | 204.092 | 200.324 | 100.428 | **93.174** | 101.881 |
| Matrix multiplication · 256 × 256 · 256 × 256 · f64 | **733.843** | 908.02 | 901.147 | 990.484 | 737.18 | 768.178 |
| Matrix multiplication · 64 × 64 · 64 × 64 · f32 | 6.522 | 7.494 | 7.464 | **6.502** | 6.723 | 8.135 |
| Matrix multiplication · 128 × 128 · 128 × 128 · f32 | 46.407 | 74.67 | 73.337 | 49.212 | **46.036** | 49.392 |
| Matrix multiplication · 256 × 256 · 256 × 256 · f32 | 356.397 | 597.34 | 600.475 | 408.114 | **352.61** | 362.749 |
| Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64 | **3.076** | — | — | — | 3.236 | 4.218 |
| Matmul · 2 workers · reused output · 64 × 64 · 64 × 64 · f64 | 18.845 | — | — | — | — | — |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f64 | 57.127 | — | — | — | — | — |
| Matmul · 2 workers · reused output · 256 × 256 · 256 × 256 · f64 | 392.625 | — | — | — | — | — |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f32 | 32 | — | — | — | — | — |
| Cholesky factorization · 512 × 512 · f64 | 7630.374 | — | 7353.676 | **2616.349** | — | 6224.59 |
| Column-pivoted QR · 96 × 32 · f64 | 78.427 | — | 78.367 | **43.021** | — | — |
| Column-pivoted QR · 256 × 64 · f64 | 851.164 | — | 762.537 | **345.727** | — | — |
| Minimum-norm least squares · 64 × 32, rank 16, 4 RHS · f64 | **62.467** | — | 103.223 | — | — | 102.713 |
| Minimum-norm least squares · 32 × 64, rank 32, 4 RHS · f64 | **84.669** | — | 178.555 | — | — | 185.417 |
| Thin QR · 96 × 32 · f64 | 66.805 | — | 58.88 | **54.001** | — | 69.781 |
| Thin QR · 256 × 64 · f64 | 779.228 | — | 569.847 | **418.002** | — | 537.937 |
| Cholesky factorization · 64 × 64 · f64 | 24.967 | — | 20.969 | **14.087** | — | 18.123 |
| Cholesky factorization · 128 × 128 · f64 | 169.017 | — | 119.263 | **57.919** | — | 108.032 |
| Cholesky factorization · 256 × 256 · f64 | 1013.9490000000001 | — | 790.921 | **312.905** | — | 1080.593 |
| Shared-variable backward · sum(X * X), 64 × 32; prepared graph · f64 | 10.54 | — | — | — | **4.949** | — |
| Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64 | **3.8770000000000005** | — | — | — | 7.715 | — |
| Model inference · batched normalized/sliced MLP · f64 | **3.376** | — | — | — | 5.01 | — |
| Model recording + forward · batched normalized/sliced MLP · f64 | **4.168** | — | — | — | 6.773 | — |
| Model backward · prepared graph → fresh gradients · f64 | **4.969** | — | — | — | 20.859 | — |
| Parameter update · transactional SGD, fixed gradients · f64 | 0.802 | — | — | — | — | — |
| Coalesced tensor contraction · 16 × 32 × 8 · 32 × 8 × 16 · f64 | 63.68900000000001 | — | — | — | — | **11.091** |
| Row softmax · 256 × 64 · f64 | 119.564 | **113.523** | — | — | 115.285 | 133.509 |
| Create zeros · 64 × 64 · f32 | 0.321 | **0.241** | — | 0.33 | 0.301 | 0.742 |
| Create ones · 64 × 64 · f32 | 0.431 | **0.351** | — | 0.391 | 0.47 | 2.064 |

## Version comparison

0 of 34 workloads have paired evidence. Exploratory 95% intervals resample process pairs, not calls. No multiple-comparison adjustment.

| Workload | Current / baseline [95% interval] | Interpretation |
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

## Method and provenance

CPU affinity pinned to logical CPU 0. Frequency and background load are uncontrolled.

Minimum-norm rows use pivoted QR plus scaled implicit RZ in Kernix; NumPy native lstsq; nalgebra native SVD/solve. Algorithm costs and rank policies differ. See original method for thresholds.

Numbers retain source precision; recorded digits do not imply equivalent physical measurement accuracy. Lowest observed medians are bold only when peers were measured. No aggregate winner or universal ranking is inferred.

[Original report and commentary](original-report.txt) · [Raw measurements](raw.json)

Measured source `18750693908110110d85d02d0f11af8d58bfae8b`. Run `baseline-ubuntu-latest-40059566395`. Rendering does not change measurements.
