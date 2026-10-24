# Kernix 0.7.0 · performance brief

12th Gen Intel(R) Core(TM) i3-1215U / Linux

**Per-call median, microseconds per operation. Lower is faster. All recorded digits retained.**

14 workloads · 50 observations

No previous-version baseline attached. Sequential run changes are descriptive; paired process evidence is shown separately.

[Open the visual report](report.html) · [Vector PDF](latency.pdf) · [Raw measurements](raw.json)

[![Performance brief preview](latency.png)](latency.pdf)

| Workload | Kernix | ndarray | nalgebra | faer | Candle | NumPy |
|---|---:|---:|---:|---:|---:|---:|
| Matrix multiplication · 32 × 32 · 32 × 32 · f64 | 1.67 | 1.473 | 1.588 | 1.578 | **1.422** | 2.894 |
| Matrix multiplication · 128 × 128 · 128 × 128 · f64 | 117.968 | 118.998 | 118.759 | 84.278 | **72.785** | 78.228 |
| Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64 | 2.124 | — | — | — | **1.9700000000000002** | 3.3 |
| Thin QR · 96 × 32 · f64 | 43.656 | — | **37.46** | 66.159 | — | 40.097 |
| Thin QR · 256 × 64 · f64 | 522.77300000000006 | — | 382.268 | 413.236 | — | **359.863** |
| Cholesky factorization · 64 × 64 · f64 | 22.814 | — | 14.206 | **8.562** | — | 10.214 |
| Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64 | **2.367** | — | — | — | 4.0570000000000005 | — |
| Model inference · batched normalized/sliced MLP · f64 | 4.364 | — | — | — | **3.801** | — |
| Model recording + forward · batched normalized/sliced MLP · f64 | **4.864** | — | — | — | 5.214 | — |
| Model backward · prepared graph → fresh gradients · f64 | **9.065** | — | — | — | 14.552 | — |
| Parameter update · transactional SGD, fixed gradients · f64 | 0.48100000000000006 | — | — | — | — | — |
| Row softmax · 256 × 64 · f64 | 86.27 | **75.605** | — | — | 79.866 | 107.298 |
| Create zeros · 64 × 64 · f32 | 0.146 | **0.099** | — | 0.172 | 0.167 | 0.421 |
| Create ones · 64 × 64 · f32 | 0.186 | **0.16** | — | 0.179 | 0.232 | 1.332 |

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

Measured source `4f60380b4a5ac07931347b4f9eba0ac696983f9a`. Run `0.7.0-registry-4f60380`. Rendering does not change measurements.
