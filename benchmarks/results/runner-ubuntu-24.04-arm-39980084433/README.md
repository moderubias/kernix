# Kernix 0.9.0 · performance brief

Neoverse-N2 / Linux

**Per-call median, microseconds per operation. Lower is faster. All recorded digits retained.**

25 workloads · 100 observations

No previous-version baseline attached. Sequential run changes are descriptive; paired process evidence is shown separately.

[Open the visual report](report.html) · [Vector PDF](latency.pdf) · [Raw measurements](raw.json)

[![Performance brief preview](latency.png)](latency.pdf)

| Workload | Kernix | ndarray | nalgebra | faer | Candle | NumPy |
|---|---:|---:|---:|---:|---:|---:|
| Matrix multiplication · 32 × 32 · 32 × 32 · f64 | 3.239 | **3.056** | 3.216 | 3.504 | 3.424 | 8.2 |
| Matrix multiplication · 64 × 64 · 64 × 64 · f64 | 22.174 | **21.799** | 22.463 | 22.631 | 22.574 | 31.118 |
| Matrix multiplication · 128 × 128 · 128 × 128 · f64 | 210.41 | 207.458 | 208.642 | 169.821 | **169.268** | 196.011 |
| Matrix multiplication · 256 × 256 · 256 × 256 · f64 | 1335.542 | 1326.502 | 1334.373 | 1548.982 | **1324.023** | 1466.035 |
| Matrix multiplication · 64 × 64 · 64 × 64 · f32 | 11.504 | **11.247** | 11.271 | 12.295 | 11.951 | 20.119 |
| Matrix multiplication · 128 × 128 · 128 × 128 · f32 | 94.506 | 93.594 | 94.962 | 87.051 | **85.819** | 100.065 |
| Matrix multiplication · 256 × 256 · 256 × 256 · f32 | 770.612 | 765.316 | 761.412 | **674.018** | 757.933 | 729.143 |
| Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64 | **3.856** | — | — | — | 4.448 | 12.975 |
| Column-pivoted QR · 96 × 32 · f64 | 88.146 | — | 114.304 | **61.988** | — | — |
| Column-pivoted QR · 256 × 64 · f64 | 786.763 | — | 1151.545 | **474.936** | — | — |
| Minimum-norm least squares · 64 × 32, rank 16, 4 RHS · f64 | **76.059** | — | 116.8 | — | — | 93.706 |
| Minimum-norm least squares · 32 × 64, rank 32, 4 RHS · f64 | **114.616** | — | 217.986 | — | — | 176.46 |
| Thin QR · 96 × 32 · f64 | 70.155 | — | **61.388** | 65.356 | — | 65.963 |
| Thin QR · 256 × 64 · f64 | 690.305 | — | 588.263 | **443.554** | — | 576.105 |
| Cholesky factorization · 64 × 64 · f64 | 27.334 | — | 20.35 | **13.256** | — | 18.031 |
| Cholesky factorization · 128 × 128 · f64 | 280.789 | — | 128.775 | **64.996** | — | 82.746 |
| Cholesky factorization · 256 × 256 · f64 | 1994.9120000000002 | — | 1031.7060000000001 | **407.037** | — | 751.805 |
| Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64 | **3.512** | — | — | — | 7.128 | — |
| Model inference · batched normalized/sliced MLP · f64 | **3.087** | — | — | — | 5.279 | — |
| Model recording + forward · batched normalized/sliced MLP · f64 | **3.8229999999999995** | — | — | — | 7.615 | — |
| Model backward · prepared graph → fresh gradients · f64 | **4.192** | — | — | — | 21.263 | — |
| Parameter update · transactional SGD, fixed gradients · f64 | 0.736 | — | — | — | — | — |
| Row softmax · 256 × 64 · f64 | 113 | **97.266** | — | — | 111.553 | 134.894 |
| Create zeros · 64 × 64 · f32 | 0.184 | **0.136** | — | 0.328 | 0.256 | 0.512 |
| Create ones · 64 × 64 · f32 | 0.248 | **0.208** | — | 0.23999999999999997 | 0.344 | 1.52 |

## Version comparison

0 of 25 workloads have paired evidence. Exploratory 95% intervals resample process pairs, not calls. No multiple-comparison adjustment.

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
| Column-pivoted QR · 96 × 32 · f64 | — | No paired comparison |
| Column-pivoted QR · 256 × 64 · f64 | — | No paired comparison |
| Minimum-norm least squares · 64 × 32, rank 16, 4 RHS · f64 | — | No paired comparison |
| Minimum-norm least squares · 32 × 64, rank 32, 4 RHS · f64 | — | No paired comparison |
| Thin QR · 96 × 32 · f64 | — | No paired comparison |
| Thin QR · 256 × 64 · f64 | — | No paired comparison |
| Cholesky factorization · 64 × 64 · f64 | — | No paired comparison |
| Cholesky factorization · 128 × 128 · f64 | — | No paired comparison |
| Cholesky factorization · 256 × 256 · f64 | — | No paired comparison |
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

Minimum-norm rows use pivoted QR plus scaled right QR in Kernix; NumPy native lstsq; nalgebra native SVD/solve. Algorithm costs and rank policies differ. See original method for thresholds.

Numbers retain source precision; recorded digits do not imply equivalent physical measurement accuracy. Lowest observed medians are bold only when peers were measured. No aggregate winner or universal ranking is inferred.

[Original report and commentary](original-report.txt) · [Raw measurements](raw.json)

Measured source `9b7ef492c99bf2202e7eb3b07a79ef8f108991c7`. Run `runner-ubuntu-24.04-arm-39980084433`. Rendering does not change measurements.
