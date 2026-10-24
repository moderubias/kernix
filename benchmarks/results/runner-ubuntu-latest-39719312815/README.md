# Kernix 0.8.0 · performance brief

AMD EPYC 7763 64-Core Processor / Linux

**Per-call median, microseconds per operation. Lower is faster. All recorded digits retained.**

21 workloads · 88 observations

No previous-version baseline attached. Sequential run changes are descriptive; paired process evidence is shown separately.

[Open the visual report](report.html) · [Vector PDF](latency.pdf) · [Raw measurements](raw.json)

[![Performance brief preview](latency.png)](latency.pdf)

| Workload | Kernix | ndarray | nalgebra | faer | Candle | NumPy |
|---|---:|---:|---:|---:|---:|---:|
| Matrix multiplication · 32 × 32 · 32 × 32 · f64 | 2.525 | 2.204 | 2.104 | **2.034** | 2.174 | 4.088 |
| Matrix multiplication · 64 × 64 · 64 × 64 · f64 | **12.744** | 15.449 | 14.968 | 13.555 | 12.854 | 14.928 |
| Matrix multiplication · 128 × 128 · 128 × 128 · f64 | 93.687 | 213.732 | 210.025 | 100.529 | **92.925** | 100.99 |
| Matrix multiplication · 256 × 256 · 256 × 256 · f64 | **731.015** | 910.683 | 901.967 | 963.663 | 735.213 | 767.644 |
| Matrix multiplication · 64 × 64 · 64 × 64 · f32 | 6.542 | 7.444 | 7.403 | **6.523** | 6.592 | 8.176 |
| Matrix multiplication · 128 × 128 · 128 × 128 · f32 | 46.287 | 75.692 | 74.28 | 52.289 | **46.177** | 48.983 |
| Matrix multiplication · 256 × 256 · 256 × 256 · f32 | **352.744** | 615.598 | 617.082 | 417.535 | 356.691 | 363.384 |
| Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64 | **3.096** | — | — | — | 3.286 | 4.198 |
| Thin QR · 96 × 32 · f64 | 66.746 | — | 57.739 | **54.122** | — | 70.382 |
| Thin QR · 256 × 64 · f64 | 805.205 | — | 567.719 | **422.545** | — | 537.822 |
| Cholesky factorization · 64 × 64 · f64 | 25.808 | — | 20.228 | **14.206** | — | 18.174 |
| Cholesky factorization · 128 × 128 · f64 | 169.068 | — | 120.737 | **58.871** | — | 107.282 |
| Cholesky factorization · 256 × 256 · f64 | 1030.9900000000001 | — | 820.354 | **316.065** | — | 1094.099 |
| Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64 | **3.977** | — | — | — | 7.564 | — |
| Model inference · batched normalized/sliced MLP · f64 | **3.146** | — | — | — | 4.94 | — |
| Model recording + forward · batched normalized/sliced MLP · f64 | **4.0879999999999995** | — | — | — | 11.381 | — |
| Model backward · prepared graph → fresh gradients · f64 | **4.98** | — | — | — | 20.569 | — |
| Parameter update · transactional SGD, fixed gradients · f64 | 0.831 | — | — | — | — | — |
| Row softmax · 256 × 64 · f64 | 120.307 | **110.859** | — | — | 115.458 | 131.027 |
| Create zeros · 64 × 64 · f32 | 0.311 | **0.23999999999999997** | — | 0.3 | 0.301 | 0.751 |
| Create ones · 64 × 64 · f32 | 0.431 | **0.351** | — | 0.391 | 0.461 | 2.054 |

## Version comparison

0 of 21 workloads have paired evidence. Exploratory 95% intervals resample process pairs, not calls. No multiple-comparison adjustment.

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



Numbers retain source precision; recorded digits do not imply equivalent physical measurement accuracy. Lowest observed medians are bold only when peers were measured. No aggregate winner or universal ranking is inferred.

[Original report and commentary](original-report.txt) · [Raw measurements](raw.json)

Measured source `227f339974f9c31fcf6bd37ccccbda3eedc99591`. Run `runner-ubuntu-latest-39719312815`. Rendering does not change measurements.
