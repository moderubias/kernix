# Kernix 0.8.0 · performance brief

Neoverse-N2 / Linux

**Per-call median, microseconds per operation. Lower is faster. All recorded digits retained.**

21 workloads · 88 observations

No previous-version baseline attached. Sequential run changes are descriptive; paired process evidence is shown separately.

[Open the visual report](report.html) · [Vector PDF](latency.pdf) · [Raw measurements](raw.json)

[![Performance brief preview](latency.png)](latency.pdf)

| Workload | Kernix | ndarray | nalgebra | faer | Candle | NumPy |
|---|---:|---:|---:|---:|---:|---:|
| Matrix multiplication · 32 × 32 · 32 × 32 · f64 | 3.24 | **3.048** | 3.216 | 3.511 | 3.44 | 8.239 |
| Matrix multiplication · 64 × 64 · 64 × 64 · f64 | 22.174 | **21.799** | 22.486 | 22.63 | 22.559 | 31.022 |
| Matrix multiplication · 128 × 128 · 128 × 128 · f64 | 225.569 | 224.513 | 225.025 | 169.485 | **168.765** | 194.635 |
| Matrix multiplication · 256 × 256 · 256 × 256 · f64 | 1332.135 | 1324.464 | 1329.142 | 1621.235 | **1321.904** | 1463.59 |
| Matrix multiplication · 64 × 64 · 64 × 64 · f32 | 11.528 | **11.263** | 11.28 | 12.287 | 12 | 20.118 |
| Matrix multiplication · 128 × 128 · 128 × 128 · f32 | 97.833 | 97.337 | 97.426 | 87.203 | **85.922** | 99.833 |
| Matrix multiplication · 256 × 256 · 256 × 256 · f32 | 802.778 | 799.795 | 795.611 | **677.707** | 724.783 | 727.023 |
| Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64 | **3.8310000000000005** | — | — | — | 4.464 | 12.999 |
| Thin QR · 96 × 32 · f64 | 89.546 | — | **60.876** | 65.39500000000001 | — | 65.963 |
| Thin QR · 256 × 64 · f64 | 917.547 | — | 593.232 | **448.938** | — | 572.866 |
| Cholesky factorization · 64 × 64 · f64 | 27.326 | — | 20.359 | **13.231** | — | 18.143 |
| Cholesky factorization · 128 × 128 · f64 | 279.557 | — | 128.69499999999999 | **65.01199999999999** | — | 85.178 |
| Cholesky factorization · 256 × 256 · f64 | 2015.201 | — | 1029.683 | **404.461** | — | 821.793 |
| Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64 | **3.527** | — | — | — | 7.223 | — |
| Model inference · batched normalized/sliced MLP · f64 | **3.023** | — | — | — | 5.32 | — |
| Model recording + forward · batched normalized/sliced MLP · f64 | **3.856** | — | — | — | 7.584 | — |
| Model backward · prepared graph → fresh gradients · f64 | **4.256** | — | — | — | 21.134 | — |
| Parameter update · transactional SGD, fixed gradients · f64 | 0.728 | — | — | — | — | — |
| Row softmax · 256 × 64 · f64 | 113.168 | **97.41** | — | — | 110.969 | 135.151 |
| Create zeros · 64 × 64 · f32 | 0.184 | **0.136** | — | 0.32 | 0.23999999999999997 | 0.504 |
| Create ones · 64 × 64 · f32 | 0.256 | **0.208** | — | 0.23999999999999997 | 0.344 | 1.528 |

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

Measured source `227f339974f9c31fcf6bd37ccccbda3eedc99591`. Run `runner-ubuntu-24.04-arm-39719312815`. Rendering does not change measurements.
