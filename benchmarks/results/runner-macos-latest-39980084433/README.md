# Kernix 0.9.0 · performance brief

Apple M1 (Virtual) / macOS

**Per-call median, microseconds per operation. Lower is faster. All recorded digits retained.**

25 workloads · 100 observations

No previous-version baseline attached. Sequential run changes are descriptive; paired process evidence is shown separately.

[Open the visual report](report.html) · [Vector PDF](latency.pdf) · [Raw measurements](raw.json)

[![Performance brief preview](latency.png)](latency.pdf)

| Workload | Kernix | ndarray | nalgebra | faer | Candle | NumPy |
|---|---:|---:|---:|---:|---:|---:|
| Matrix multiplication · 32 × 32 · 32 × 32 · f64 | 2.25 | 2.083 | 2.083 | 2.417 | 2.583 | **1.541** |
| Matrix multiplication · 64 × 64 · 64 × 64 · f64 | 12.959 | 13.375 | 15.583999999999998 | 14.709 | 14.5 | **3.5** |
| Matrix multiplication · 128 × 128 · 128 × 128 · f64 | 102.541 | 109.792 | 103.667 | 108.5 | 97.75 | **17.667** |
| Matrix multiplication · 256 × 256 · 256 × 256 · f64 | 845.417 | 816.333 | 1022.4170000000001 | 822.75 | 883 | **120.834** |
| Matrix multiplication · 64 × 64 · 64 × 64 · f32 | 10.5 | 6.959 | 6.917 | 8.209 | 7.916 | **1.916** |
| Matrix multiplication · 128 × 128 · 128 × 128 · f32 | 52.459 | 47.834 | 51.208 | 55.5 | 52.916 | **6.125** |
| Matrix multiplication · 256 × 256 · 256 × 256 · f32 | 372.5 | 390.542 | 384.375 | 458.125 | 408.125 | **39.916** |
| Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64 | 2.875 | — | — | — | 3.583 | **1.791** |
| Column-pivoted QR · 96 × 32 · f64 | 73.25 | — | 96.375 | **40.167** | — | — |
| Column-pivoted QR · 256 × 64 · f64 | 605.417 | — | 1009.125 | **309.791** | — | — |
| Minimum-norm least squares · 64 × 32, rank 16, 4 RHS · f64 | **51.25** | — | 69.834 | — | — | 81 |
| Minimum-norm least squares · 32 × 64, rank 32, 4 RHS · f64 | **95.917** | — | 152.958 | — | — | 136.125 |
| Thin QR · 96 × 32 · f64 | 56.792 | — | **43.542** | 50.541 | — | 47.5 |
| Thin QR · 256 × 64 · f64 | 497.91699999999994 | — | 356.042 | **303.167** | — | 364.458 |
| Cholesky factorization · 64 × 64 · f64 | 22.292 | — | 21.042 | **10.334** | — | 15.083 |
| Cholesky factorization · 128 × 128 · f64 | 167.667 | — | 104 | **49.5** | — | 108.167 |
| Cholesky factorization · 256 × 256 · f64 | 1322.125 | — | 730.125 | **250.708** | — | 330.125 |
| Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64 | **8.042** | — | — | — | 19.416 | — |
| Model inference · batched normalized/sliced MLP · f64 | **2.25** | — | — | — | 5.458 | — |
| Model recording + forward · batched normalized/sliced MLP · f64 | **2.875** | — | — | — | 5.625 | — |
| Model backward · prepared graph → fresh gradients · f64 | **3.75** | — | — | — | 20.959 | — |
| Parameter update · transactional SGD, fixed gradients · f64 | 0.75 | — | — | — | — | — |
| Row softmax · 256 × 64 · f64 | 78.917 | 65.25 | — | — | **64.99999999999999** | 76.667 |
| Create zeros · 64 × 64 · f32 | 0.75 | **0.291** | — | **0.291** | 0.458 | 0.542 |
| Create ones · 64 × 64 · f32 | **0.292** | 0.333 | — | 0.333 | 0.417 | 1.125 |

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

CPU affinity was not pinned. Frequency and background load are uncontrolled.

Minimum-norm rows use pivoted QR plus scaled right QR in Kernix; NumPy native lstsq; nalgebra native SVD/solve. Algorithm costs and rank policies differ. See original method for thresholds.

Numbers retain source precision; recorded digits do not imply equivalent physical measurement accuracy. Lowest observed medians are bold only when peers were measured. No aggregate winner or universal ranking is inferred.

[Original report and commentary](original-report.txt) · [Raw measurements](raw.json)

Measured source `9b7ef492c99bf2202e7eb3b07a79ef8f108991c7`. Run `runner-macos-latest-39980084433`. Rendering does not change measurements.
