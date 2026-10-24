# Kernix 0.10.0 · performance brief

AMD EPYC 7763 64-Core Processor / Windows

**Per-call median, microseconds per operation. Lower is faster. All recorded digits retained.**

34 workloads · 116 observations

No previous-version baseline attached. Sequential run changes are descriptive; paired process evidence is shown separately.

[Open the visual report](report.html) · [Vector PDF](latency.pdf) · [Raw measurements](raw.json)

[![Performance brief preview](latency.png)](latency.pdf)

| Workload | Kernix | ndarray | nalgebra | faer | Candle | NumPy |
|---|---:|---:|---:|---:|---:|---:|
| Independent matrix operands · 128 × 128 · 128 × 128 · f64 | 141.4 | — | — | — | — | **102.3** |
| Independent matrix operands · 128 × 128 · 128 × 128 · f32 | 75.2 | — | — | — | — | **56.7** |
| Matrix multiplication · 32 × 32 · 32 × 32 · f64 | 2.8 | 2.4 | **2.3** | 2.5 | 4.3 | 4.1 |
| Matrix multiplication · 64 × 64 · 64 × 64 · f64 | **13.6** | 62.10000000000001 | 58.5 | 14.7 | 14.1 | 16 |
| Matrix multiplication · 128 × 128 · 128 × 128 · f64 | 140 | 114.8 | 115.9 | 102.2 | 141.3 | **100.2** |
| Matrix multiplication · 256 × 256 · 256 × 256 · f64 | 868.9 | 1179.1 | 908.6 | 1081.3 | 875.3 | **762.8** |
| Matrix multiplication · 64 × 64 · 64 × 64 · f32 | **6.9** | 30.4 | 8.2 | 7 | 7.4 | 10.4 |
| Matrix multiplication · 128 × 128 · 128 × 128 · f32 | 76 | 122.29999999999999 | 53.6 | **50.4** | 75.5 | 56.1 |
| Matrix multiplication · 256 × 256 · 256 × 256 · f32 | **355.8** | 393.6 | 397.9 | 405.2 | 431.6 | 388.3 |
| Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64 | **3.4** | — | — | — | 8.2 | 5 |
| Matmul · 2 workers · reused output · 64 × 64 · 64 × 64 · f64 | 40.1 | — | — | — | — | — |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f64 | 88.4 | — | — | — | — | — |
| Matmul · 2 workers · reused output · 256 × 256 · 256 × 256 · f64 | 438.7 | — | — | — | — | — |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f32 | 59 | — | — | — | — | — |
| Cholesky factorization · 512 × 512 · f64 | 8586.8 | — | 8274.999999999999 | **3164.7** | — | 6804.2 |
| Column-pivoted QR · 96 × 32 · f64 | 125 | — | 108.4 | **78** | — | — |
| Column-pivoted QR · 256 × 64 · f64 | 936.6 | — | 909 | **413** | — | — |
| Minimum-norm least squares · 64 × 32, rank 16, 4 RHS · f64 | **100.6** | — | 104.1 | — | — | 114.5 |
| Minimum-norm least squares · 32 × 64, rank 32, 4 RHS · f64 | **114.9** | — | 174.8 | — | — | 215 |
| Thin QR · 96 × 32 · f64 | 104.5 | — | 78.5 | 87.6 | — | **77.2** |
| Thin QR · 256 × 64 · f64 | 783.8 | — | 639.6 | **385.9** | — | 546.1 |
| Cholesky factorization · 64 × 64 · f64 | 35.1 | — | 21.5 | **14.5** | — | 21 |
| Cholesky factorization · 128 × 128 · f64 | 229.3 | — | 120.7 | **59** | — | 128.6 |
| Cholesky factorization · 256 × 256 · f64 | 991.9 | — | 871.8 | **316.2** | — | 953.1 |
| Shared-variable backward · sum(X * X), 64 × 32; prepared graph · f64 | **9.6** | — | — | — | 58.6 | — |
| Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64 | **4.4** | — | — | — | 10.1 | — |
| Model inference · batched normalized/sliced MLP · f64 | **3.5** | — | — | — | 10.1 | — |
| Model recording + forward · batched normalized/sliced MLP · f64 | **4.8** | — | — | — | 10.2 | — |
| Model backward · prepared graph → fresh gradients · f64 | **5.8** | — | — | — | 31.900000000000004 | — |
| Parameter update · transactional SGD, fixed gradients · f64 | 1 | — | — | — | — | — |
| Coalesced tensor contraction · 16 × 32 × 8 · 32 × 8 × 16 · f64 | 63.7 | — | — | — | — | **11.8** |
| Row softmax · 256 × 64 · f64 | **114.5** | 153.7 | — | — | 155.8 | 132.6 |
| Create zeros · 64 × 64 · f32 | 0.6 | **0.3** | — | 0.5 | 0.5 | 0.8 |
| Create ones · 64 × 64 · f32 | 0.5 | **0.4** | — | 0.5 | 0.7 | 2.1 |

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

CPU affinity was not pinned. Frequency and background load are uncontrolled.

Minimum-norm rows use pivoted QR plus scaled implicit RZ in Kernix; NumPy native lstsq; nalgebra native SVD/solve. Algorithm costs and rank policies differ. See original method for thresholds.

Numbers retain source precision; recorded digits do not imply equivalent physical measurement accuracy. Lowest observed medians are bold only when peers were measured. No aggregate winner or universal ranking is inferred.

[Original report and commentary](original-report.txt) · [Raw measurements](raw.json)

Measured source `18750693908110110d85d02d0f11af8d58bfae8b`. Run `baseline-windows-latest-40059566395`. Rendering does not change measurements.
