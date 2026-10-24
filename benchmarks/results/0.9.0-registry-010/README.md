# Kernix 0.9.0 · performance brief

12th Gen Intel(R) Core(TM) i3-1215U / Linux

**Per-call median, microseconds per operation. Lower is faster. All recorded digits retained.**

31 workloads · 108 observations

No previous-version baseline attached. Sequential run changes are descriptive; paired process evidence is shown separately.

[Open the visual report](report.html) · [Vector PDF](latency.pdf) · [Raw measurements](raw.json)

[![Performance brief preview](latency.png)](latency.pdf)

| Workload | Kernix | ndarray | nalgebra | faer | Candle | NumPy |
|---|---:|---:|---:|---:|---:|---:|
| Matrix multiplication · 32 × 32 · 32 × 32 · f64 | 1.687 | 1.526 | **1.481** | 1.561 | 1.574 | 2.788 |
| Matrix multiplication · 64 × 64 · 64 × 64 · f64 | 17.926 | 11.608 | 11.195 | **11.187** | 11.379 | 12.204 |
| Matrix multiplication · 128 × 128 · 128 × 128 · f64 | 83.207 | 121.834 | 120.325 | 86.726 | **78.204** | 82.343 |
| Matrix multiplication · 256 × 256 · 256 × 256 · f64 | 647.477 | 685.012 | 680.5 | 758.898 | **603.518** | 668.632 |
| Matrix multiplication · 64 × 64 · 64 × 64 · f32 | **5.081** | 6.127 | 6.239 | 5.624 | 5.767 | 7.539 |
| Matrix multiplication · 128 × 128 · 128 × 128 · f32 | **38.634** | 52.083 | 50.48 | 50.973 | 39.756 | 40.227 |
| Matrix multiplication · 256 × 256 · 256 × 256 · f32 | **294.932** | 410.973 | 412.324 | 343.122 | 304.379 | 301.429 |
| Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64 | **2.073** | — | — | — | 4.315 | 3.491 |
| Matmul · 2 workers · reused output · 64 × 64 · 64 × 64 · f64 | 10.422 | — | — | — | — | — |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f64 | 105.08 | — | — | — | — | — |
| Matmul · 2 workers · reused output · 256 × 256 · 256 × 256 · f64 | 423.627 | — | — | — | — | — |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f32 | 29.303 | — | — | — | — | — |
| Column-pivoted QR · 96 × 32 · f64 | **58.492** | — | 79.44 | 78.191 | — | — |
| Column-pivoted QR · 256 × 64 · f64 | 599.809 | — | 843.15 | **485.354** | — | — |
| Minimum-norm least squares · 64 × 32, rank 16, 4 RHS · f64 | **51.783** | — | 78.173 | — | — | 71.772 |
| Minimum-norm least squares · 32 × 64, rank 32, 4 RHS · f64 | **81.587** | — | 142.115 | — | — | 129.445 |
| Thin QR · 96 × 32 · f64 | 47.578 | — | **40.857** | 69.889 | — | 43.608 |
| Thin QR · 256 × 64 · f64 | 531.3 | — | 395.774 | 437.89 | — | **374.776** |
| Cholesky factorization · 64 × 64 · f64 | 21.573 | — | 15.131 | **8.936** | — | 11.778 |
| Cholesky factorization · 128 × 128 · f64 | 120.034 | — | 111.245 | **39.611** | — | 74.878 |
| Cholesky factorization · 256 × 256 · f64 | 685.247 | — | 752.003 | **206.834** | — | 545.514 |
| Shared-variable backward · sum(X * X), 64 × 32; prepared graph · f64 | 6.488 | — | — | — | **3.961** | — |
| Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64 | **2.518** | — | — | — | 4.364 | — |
| Model inference · batched normalized/sliced MLP · f64 | **2.572** | — | — | — | 3.88 | — |
| Model recording + forward · batched normalized/sliced MLP · f64 | **2.926** | — | — | — | 5.435 | — |
| Model backward · prepared graph → fresh gradients · f64 | **3.311** | — | — | — | 14.863 | — |
| Parameter update · transactional SGD, fixed gradients · f64 | 0.585 | — | — | — | — | — |
| Coalesced tensor contraction · 16 × 32 × 8 · 32 × 8 × 16 · f64 | 691.795 | — | — | — | — | **12.34** |
| Row softmax · 256 × 64 · f64 | 93.662 | **85.808** | — | — | 93.798 | 110.451 |
| Create zeros · 64 × 64 · f32 | 0.156 | **0.09** | — | 0.16 | 0.176 | 0.396 |
| Create ones · 64 × 64 · f32 | 0.205 | **0.154** | — | 0.196 | 0.225 | 1.295 |

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

Minimum-norm rows use pivoted QR plus scaled right QR in Kernix; NumPy native lstsq; nalgebra native SVD/solve. Algorithm costs and rank policies differ. See original method for thresholds.

Numbers retain source precision; recorded digits do not imply equivalent physical measurement accuracy. Lowest observed medians are bold only when peers were measured. No aggregate winner or universal ranking is inferred.

[Original report and commentary](original-report.txt) · [Raw measurements](raw.json)

Measured source `fcd9fdb97ff307e9c50d0526e8b4e1a580cb44a9`. Run `0.9.0-registry-010`. Rendering does not change measurements.
