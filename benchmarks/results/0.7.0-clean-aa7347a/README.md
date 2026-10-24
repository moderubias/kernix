# Kernix 0.7.0 · performance brief

12th Gen Intel(R) Core(TM) i3-1215U / Linux

**Per-call median, microseconds per operation. Lower is faster. All recorded digits retained.**

14 workloads · 50 observations

No previous-version baseline attached. Sequential run changes are descriptive; paired process evidence is shown separately.

[Open the visual report](report.html) · [Vector PDF](latency.pdf) · [Raw measurements](raw.json)

[![Performance brief preview](latency.png)](latency.pdf)

| Workload | Kernix | ndarray | nalgebra | faer | Candle | NumPy |
|---|---:|---:|---:|---:|---:|---:|
| Matrix multiplication · 32 × 32 · 32 × 32 · f64 | 1.661 | **1.407** | 1.423 | 1.746 | 1.505 | 2.527 |
| Matrix multiplication · 128 × 128 · 128 × 128 · f64 | 115.505 | 113.795 | 115.99 | 80.167 | **73.667** | 76.259 |
| Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64 | 2.06 | — | — | — | **1.9089999999999998** | 3.004 |
| Thin QR · 96 × 32 · f64 | 41.632 | — | 37.427 | 70.004 | — | **36.782** |
| Thin QR · 256 × 64 · f64 | 496.64200000000006 | — | 363.898 | 406.038 | — | **338.857** |
| Cholesky factorization · 64 × 64 · f64 | 25.074 | — | 13.479 | **8.421** | — | 10.169 |
| Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64 | **2.403** | — | — | — | 3.927 | — |
| Model inference · batched normalized/sliced MLP · f64 | 3.809 | — | — | — | **3.282** | — |
| Model recording + forward · batched normalized/sliced MLP · f64 | **4.646** | — | — | — | 4.756 | — |
| Model backward · prepared graph → fresh gradients · f64 | **8.668** | — | — | — | 13.309 | — |
| Parameter update · transactional SGD, fixed gradients · f64 | 0.48299999999999994 | — | — | — | — | — |
| Row softmax · 256 × 64 · f64 | 84.4 | **68.581** | — | — | 74.177 | 105.246 |
| Create zeros · 64 × 64 · f32 | 0.146 | **0.09** | — | 0.189 | 0.152 | 0.407 |
| Create ones · 64 × 64 · f32 | 0.194 | **0.153** | — | 0.178 | 0.216 | 1.223 |

## Version comparison

0 of 14 workloads have paired evidence. Exploratory 95% intervals resample process pairs, not calls. No multiple-comparison adjustment.

| Workload | Current / baseline [95% interval] | Interpretation |
|---|---:|---|
| Matrix multiplication · 32 × 32 · 32 × 32 · f64 | — | No paired comparison |
| Matrix multiplication · 128 × 128 · 128 × 128 · f64 | — | No paired comparison |
| Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64 | — | No paired comparison |
| Thin QR · 96 × 32 · f64 | — | No paired comparison |
| Thin QR · 256 × 64 · f64 | — | No paired comparison |
| Cholesky factorization · 64 × 64 · f64 | — | No paired comparison |
| Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64 | — | No paired comparison |
| Model inference · batched normalized/sliced MLP · f64 | — | No paired comparison |
| Model recording + forward · batched normalized/sliced MLP · f64 | — | No paired comparison |
| Model backward · prepared graph → fresh gradients · f64 | — | No paired comparison |
| Parameter update · transactional SGD, fixed gradients · f64 | — | No paired comparison |
| Row softmax · 256 × 64 · f64 | — | No paired comparison |
| Create zeros · 64 × 64 · f32 | — | No paired comparison |
| Create ones · 64 × 64 · f32 | — | No paired comparison |

## Method and provenance

CPU affinity pinned to logical CPU 0. Frequency and background load are uncontrolled.



Numbers retain source precision; recorded digits do not imply equivalent physical measurement accuracy. Lowest observed medians are bold only when peers were measured. No aggregate winner or universal ranking is inferred.

[Original report and commentary](original-report.txt) · [Raw measurements](raw.json)

Measured source `aa7347a201ce9294b02990b7598af146470bca65`. Run `0.7.0-clean-aa7347a`. Rendering does not change measurements.
