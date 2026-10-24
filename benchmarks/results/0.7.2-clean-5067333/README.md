# Kernix 0.7.2 · performance brief

12th Gen Intel(R) Core(TM) i3-1215U / Linux

**Per-call median, microseconds per operation. Lower is faster. All recorded digits retained.**

16 workloads · 62 observations

No previous-version baseline attached. Sequential run changes are descriptive; paired process evidence is shown separately.

[Open the visual report](report.html) · [Vector PDF](latency.pdf) · [Raw measurements](raw.json)

[![Performance brief preview](latency.png)](latency.pdf)

| Workload | Kernix | ndarray | nalgebra | faer | Candle | NumPy |
|---|---:|---:|---:|---:|---:|---:|
| Matrix multiplication · 32 × 32 · 32 × 32 · f64 | 1.535 | **1.402** | 1.504 | 1.631 | 1.573 | 2.741 |
| Matrix multiplication · 64 × 64 · 64 × 64 · f64 | **9.12** | 10.517 | 11.415 | 10.448 | 9.915 | 12.512 |
| Matrix multiplication · 128 × 128 · 128 × 128 · f64 | **68.456** | 115.016 | 112.896 | 81.04 | 71.202 | 75.573 |
| Matrix multiplication · 256 × 256 · 256 × 256 · f64 | **548.673** | 632.796 | 640.174 | 711.27 | 560.287 | 592.335 |
| Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64 | **1.9** | — | — | — | 1.974 | 3.117 |
| Thin QR · 96 × 32 · f64 | 46.649 | — | **35.13** | 65.928 | — | 38.736 |
| Thin QR · 256 × 64 · f64 | 487.3 | — | 363.922 | 405.668 | — | **341.148** |
| Cholesky factorization · 64 × 64 · f64 | 25.25 | — | 13.459 | **7.263** | — | 10.064 |
| Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64 | **2.382** | — | — | — | 6.678 | — |
| Model inference · batched normalized/sliced MLP · f64 | **1.966** | — | — | — | 3.479 | — |
| Model recording + forward · batched normalized/sliced MLP · f64 | **2.438** | — | — | — | 5.299 | — |
| Model backward · prepared graph → fresh gradients · f64 | **3.154** | — | — | — | 14.641 | — |
| Parameter update · transactional SGD, fixed gradients · f64 | 0.489 | — | — | — | — | — |
| Row softmax · 256 × 64 · f64 | 86.258 | **70.921** | — | — | 78.182 | 102.402 |
| Create zeros · 64 × 64 · f32 | 0.145 | **0.089** | — | 0.172 | 0.167 | 0.423 |
| Create ones · 64 × 64 · f32 | 0.185 | **0.144** | — | 0.201 | 0.237 | 1.234 |

## Version comparison

0 of 16 workloads have paired evidence. Exploratory 95% intervals resample process pairs, not calls. No multiple-comparison adjustment.

| Workload | Current / baseline [95% interval] | Interpretation |
|---|---:|---|
| Matrix multiplication · 32 × 32 · 32 × 32 · f64 | — | No paired comparison |
| Matrix multiplication · 64 × 64 · 64 × 64 · f64 | — | No paired comparison |
| Matrix multiplication · 128 × 128 · 128 × 128 · f64 | — | No paired comparison |
| Matrix multiplication · 256 × 256 · 256 × 256 · f64 | — | No paired comparison |
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

Measured source `50673332cd664b98fad90a24febce1fd8d6395b0`. Run `0.7.2-clean-5067333`. Rendering does not change measurements.
