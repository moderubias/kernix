# Kernix 0.7.2 · performance brief

12th Gen Intel(R) Core(TM) i3-1215U / Linux

**Per-call median, microseconds per operation. Lower is faster. All recorded digits retained.**

19 workloads · 80 observations

No previous-version baseline attached. Sequential run changes are descriptive; paired process evidence is shown separately.

[Open the visual report](report.html) · [Vector PDF](latency.pdf) · [Raw measurements](raw.json)

[![Performance brief preview](latency.png)](latency.pdf)

| Workload | Kernix | ndarray | nalgebra | faer | Candle | NumPy |
|---|---:|---:|---:|---:|---:|---:|
| Matrix multiplication · 32 × 32 · 32 × 32 · f64 | 1.589 | **1.412** | 1.498 | 1.875 | 1.5 | 2.822 |
| Matrix multiplication · 64 × 64 · 64 × 64 · f64 | **9.138** | 11.253 | 11.299 | 10.916 | 9.862 | 13.142 |
| Matrix multiplication · 128 × 128 · 128 × 128 · f64 | **73.035** | 119.839 | 118.439 | 86.168 | 73.442 | 79.676 |
| Matrix multiplication · 256 × 256 · 256 × 256 · f64 | **572.709** | 702.672 | 682.415 | 742.204 | 579.849 | 608.764 |
| Matrix multiplication · 64 × 64 · 64 × 64 · f32 | 5.884 | 5.847 | 6.161 | 5.35 | **4.779** | 6.338 |
| Matrix multiplication · 128 × 128 · 128 × 128 · f32 | 49.181 | 51.443 | 53.199 | 41.712 | **34.791** | 39.22 |
| Matrix multiplication · 256 × 256 · 256 × 256 · f32 | 393.444 | 391.705 | 398.043 | 312.433 | **278.711** | 286.706 |
| Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64 | **1.9360000000000002** | — | — | — | 2.078 | 3.332 |
| Thin QR · 96 × 32 · f64 | 42.746 | — | **37.668** | 64.46899999999999 | — | 38.141 |
| Thin QR · 256 × 64 · f64 | 496.68 | — | 371.777 | 407.851 | — | **348.219** |
| Cholesky factorization · 64 × 64 · f64 | 22.798 | — | 13.105 | **8.175999999999999** | — | 10.755 |
| Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64 | **2.423** | — | — | — | 4.231 | — |
| Model inference · batched normalized/sliced MLP · f64 | **2.001** | — | — | — | 3.486 | — |
| Model recording + forward · batched normalized/sliced MLP · f64 | **2.776** | — | — | — | 5.198 | — |
| Model backward · prepared graph → fresh gradients · f64 | **3.194** | — | — | — | 14.451 | — |
| Parameter update · transactional SGD, fixed gradients · f64 | 0.514 | — | — | — | — | — |
| Row softmax · 256 × 64 · f64 | 85.344 | **71.614** | — | — | 80.122 | 99.838 |
| Create zeros · 64 × 64 · f32 | 0.135 | **0.09** | — | 0.143 | 0.169 | 0.396 |
| Create ones · 64 × 64 · f32 | 0.201 | **0.154** | — | 0.202 | 0.237 | 1.345 |

## Version comparison

0 of 19 workloads have paired evidence. Exploratory 95% intervals resample process pairs, not calls. No multiple-comparison adjustment.

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

Measured source `bc459f44b276e86275cc4e431e4946443aefb15f`. Run `0.7.2-registry-bc459f4`. Rendering does not change measurements.
