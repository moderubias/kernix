# Kernix 0.10.0 · performance brief

INTEL(R) XEON(R) PLATINUM 8573C / Windows

**Per-call median, microseconds per operation. Lower is faster. All recorded digits retained.**

31 workloads · 108 observations

No previous-version baseline attached. Sequential run changes are descriptive; paired process evidence is shown separately.

[Open the visual report](report.html) · [Vector PDF](latency.pdf) · [Raw measurements](raw.json)

[![Performance brief preview](latency.png)](latency.pdf)

| Workload | Kernix | ndarray | nalgebra | faer | Candle | NumPy |
|---|---:|---:|---:|---:|---:|---:|
| Matrix multiplication · 32 × 32 · 32 × 32 · f64 | 1.8 | **1.4** | **1.4** | 1.9 | 2.7 | 2.4 |
| Matrix multiplication · 64 × 64 · 64 × 64 · f64 | 14.9 | 74.7 | 72.1 | 9.6 | 15.6 | **8.2** |
| Matrix multiplication · 128 × 128 · 128 × 128 · f64 | 106.9 | 66.9 | 66.1 | **60.6** | 178 | 72.2 |
| Matrix multiplication · 256 × 256 · 256 × 256 · f64 | 827.7 | 1056.9 | 524.19999999999994 | 511.3 | 833.2 | **485.7** |
| Matrix multiplication · 64 × 64 · 64 × 64 · f32 | 7.5 | 35.9 | 4.8 | **4.7** | 8.1 | **4.7** |
| Matrix multiplication · 128 × 128 · 128 × 128 · f32 | 54.2 | 132 | **31.2** | 31.8 | 98.8 | 38.2 |
| Matrix multiplication · 256 × 256 · 256 × 256 · f32 | 403.9 | **224.5** | 229.2 | 232.2 | 403.6 | 240.9 |
| Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64 | **2.4** | — | — | — | 4.5 | 2.9 |
| Matmul · 2 workers · reused output · 64 × 64 · 64 × 64 · f64 | 106.9 | — | — | — | — | — |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f64 | 114.9 | — | — | — | — | — |
| Matmul · 2 workers · reused output · 256 × 256 · 256 × 256 · f64 | 504.09999999999994 | — | — | — | — | — |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f32 | 77.8 | — | — | — | — | — |
| Column-pivoted QR · 96 × 32 · f64 | **134.5** | — | 142.4 | 147.8 | — | — |
| Column-pivoted QR · 256 × 64 · f64 | 1042.1 | — | 1347.4 | **758.7** | — | — |
| Minimum-norm least squares · 64 × 32, rank 16, 4 RHS · f64 | 102 | — | 164.1 | — | — | **99.4** |
| Minimum-norm least squares · 32 × 64, rank 32, 4 RHS · f64 | **114.6** | — | 164.4 | — | — | 177.6 |
| Thin QR · 96 × 32 · f64 | 131.2 | — | 81.9 | 130.80000000000001 | — | **65** |
| Thin QR · 256 × 64 · f64 | 841.7 | — | 734.2 | 644 | — | **535.1** |
| Cholesky factorization · 64 × 64 · f64 | 29.3 | — | 19.4 | **17.2** | — | 19.3 |
| Cholesky factorization · 128 × 128 · f64 | 166.6 | — | 122.6 | **54** | — | 121.9 |
| Cholesky factorization · 256 × 256 · f64 | 928.1 | — | 919.2 | **235.6** | — | 1008.2 |
| Shared-variable backward · sum(X * X), 64 × 32; prepared graph · f64 | **10.6** | — | — | — | 73.4 | — |
| Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64 | **4.2** | — | — | — | 10.3 | — |
| Model inference · batched normalized/sliced MLP · f64 | **3.4** | — | — | — | 8.8 | — |
| Model recording + forward · batched normalized/sliced MLP · f64 | **4.7** | — | — | — | 11.5 | — |
| Model backward · prepared graph → fresh gradients · f64 | **5.6** | — | — | — | 35.3 | — |
| Parameter update · transactional SGD, fixed gradients · f64 | 0.9 | — | — | — | — | — |
| Coalesced tensor contraction · 16 × 32 × 8 · 32 × 8 × 16 · f64 | 48.5 | — | — | — | — | **8.1** |
| Row softmax · 256 × 64 · f64 | 118 | **101.4** | — | — | 205.5 | 129.2 |
| Create zeros · 64 × 64 · f32 | 0.3 | **0.2** | — | 0.5 | 0.5 | 0.7 |
| Create ones · 64 × 64 · f32 | 0.5 | **0.4** | — | **0.4** | 0.7 | 1.7 |

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

CPU affinity was not pinned. Frequency and background load are uncontrolled.

Minimum-norm rows use pivoted QR plus scaled implicit RZ in Kernix; NumPy native lstsq; nalgebra native SVD/solve. Algorithm costs and rank policies differ. See original method for thresholds.

Numbers retain source precision; recorded digits do not imply equivalent physical measurement accuracy. Lowest observed medians are bold only when peers were measured. No aggregate winner or universal ranking is inferred.

[Original report and commentary](original-report.txt) · [Raw measurements](raw.json)

Measured source `1c53d4663a48de5a588809c2b69c3a6a5bdad7b2`. Run `runner-windows-latest-40000558153`. Rendering does not change measurements.
