# Kernix 0.8.0 · performance brief

Apple M1 (Virtual) / macOS

**Per-call median, microseconds per operation. Lower is faster. All recorded digits retained.**

21 workloads · 88 observations

No previous-version baseline attached. Sequential run changes are descriptive; paired process evidence is shown separately.

[Open the visual report](report.html) · [Vector PDF](latency.pdf) · [Raw measurements](raw.json)

[![Performance brief preview](latency.png)](latency.pdf)

| Workload | Kernix | ndarray | nalgebra | faer | Candle | NumPy |
|---|---:|---:|---:|---:|---:|---:|
| Matrix multiplication · 32 × 32 · 32 × 32 · f64 | 2.25 | 2.041 | 2.083 | 2.417 | 2.542 | **1.458** |
| Matrix multiplication · 64 × 64 · 64 × 64 · f64 | 14.083 | 13.459 | 13.583 | 14.75 | 14.5 | **3.5** |
| Matrix multiplication · 128 × 128 · 128 × 128 · f64 | 103.166 | 101.333 | 101.292 | 104.208 | 97.792 | **16.125** |
| Matrix multiplication · 256 × 256 · 256 × 256 · f64 | 818.166 | 786.333 | 784.708 | 915.084 | 847.667 | **119.833** |
| Matrix multiplication · 64 × 64 · 64 × 64 · f32 | 7.208 | 6.917 | 7.084 | 8.208 | 7.832999999999999 | **1.75** |
| Matrix multiplication · 128 × 128 · 128 × 128 · f32 | 53.917 | 54.792 | 51.583 | 53.333 | 53.416 | **6.083** |
| Matrix multiplication · 256 × 256 · 256 × 256 · f32 | 417.625 | 548.209 | 388.166 | 470.792 | 450.333 | **34.416** |
| Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64 | 2.75 | — | — | — | 3.75 | **2.209** |
| Thin QR · 96 × 32 · f64 | 64.70800000000001 | — | **42.125** | 46 | — | 53.666 |
| Thin QR · 256 × 64 · f64 | 560.541 | — | 354.5 | **302** | — | 411.792 |
| Cholesky factorization · 64 × 64 · f64 | 23.375 | — | 21.042 | **10.792** | — | 15.083 |
| Cholesky factorization · 128 × 128 · f64 | 172.166 | — | 108.25 | **47.167** | — | 56.958 |
| Cholesky factorization · 256 × 256 · f64 | 1195.75 | — | 772.584 | **275.708** | — | 285.125 |
| Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64 | **3.25** | — | — | — | 6.75 | — |
| Model inference · batched normalized/sliced MLP · f64 | **2.25** | — | — | — | 5.709 | — |
| Model recording + forward · batched normalized/sliced MLP · f64 | **3.167** | — | — | — | 6.208 | — |
| Model backward · prepared graph → fresh gradients · f64 | **3.5** | — | — | — | 20.459 | — |
| Parameter update · transactional SGD, fixed gradients · f64 | 0.709 | — | — | — | — | — |
| Row softmax · 256 × 64 · f64 | 78.583 | **61.66700000000001** | — | — | 70.041 | 82.458 |
| Create zeros · 64 × 64 · f32 | 0.333 | **0.25** | — | 0.291 | 0.375 | 0.542 |
| Create ones · 64 × 64 · f32 | **0.292** | 0.333 | — | 0.333 | 0.417 | 1.125 |

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

CPU affinity was not pinned. Frequency and background load are uncontrolled.



Numbers retain source precision; recorded digits do not imply equivalent physical measurement accuracy. Lowest observed medians are bold only when peers were measured. No aggregate winner or universal ranking is inferred.

[Original report and commentary](original-report.txt) · [Raw measurements](raw.json)

Measured source `8b1a73ba926709c8c187a18aafe02840f0a4df50`. Run `runner-macos-latest-39720416210`. Rendering does not change measurements.
