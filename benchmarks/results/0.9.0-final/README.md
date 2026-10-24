# Kernix 0.9.0 · performance brief

12th Gen Intel(R) Core(TM) i3-1215U / Linux

**Per-call median, microseconds per operation. Lower is faster. All recorded digits retained.**

25 workloads · 100 observations

Baseline: 0.8.0-registry-090. Sequential run changes are descriptive; paired process evidence is shown separately.

[Open the visual report](report.html) · [Vector PDF](latency.pdf) · [Raw measurements](raw.json)

[![Performance brief preview](latency.png)](latency.pdf)

| Workload | Kernix | ndarray | nalgebra | faer | Candle | NumPy | Sequential run change |
|---|---:|---:|---:|---:|---:|---:|---:|
| Matrix multiplication · 32 × 32 · 32 × 32 · f64 | 1.637 | **1.394** | 1.65 | 1.442 | 1.42 | 2.388 | +7.1% |
| Matrix multiplication · 64 × 64 · 64 × 64 · f64 | **9.483** | 11.334 | 10.468 | 11.285 | 9.957 | 11.158 | -3.8% |
| Matrix multiplication · 128 × 128 · 128 × 128 · f64 | **73.171** | 119.441 | 117.273 | 78.058 | 74.702 | 92.307 | +0.5% |
| Matrix multiplication · 256 × 256 · 256 × 256 · f64 | **556.114** | 664.653 | 636.472 | 686.047 | 557.263 | 597.657 | +1.0% |
| Matrix multiplication · 64 × 64 · 64 × 64 · f32 | 5.213 | 7.242 | 5.642 | 5.708 | **4.885** | 6.276 | +10.6% |
| Matrix multiplication · 128 × 128 · 128 × 128 · f32 | 36.757 | 53.503 | 49.789 | 41.289 | **34.787** | 36.345 | +5.1% |
| Matrix multiplication · 256 × 256 · 256 × 256 · f32 | **270.542** | 394.088 | 382.811 | 306.861 | 276.25 | 285.924 | -1.0% |
| Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64 | 2.16 | — | — | — | **2.103** | 2.865 | +7.0% |
| Column-pivoted QR · 96 × 32 · f64 | **56.637** | — | 78.306 | 71.206 | — | — | — |
| Column-pivoted QR · 256 × 64 · f64 | 511.26199999999994 | — | 781.969 | **466.149** | — | — | — |
| Minimum-norm least squares · 64 × 32, rank 16, 4 RHS · f64 | **49.903** | — | 71.614 | — | — | 63.224 | — |
| Minimum-norm least squares · 32 × 64, rank 32, 4 RHS · f64 | **76.67** | — | 129.405 | — | — | 115.946 | — |
| Thin QR · 96 × 32 · f64 | 49.547 | — | **36.1** | 69.82 | — | 38.052 | +7.8% |
| Thin QR · 256 × 64 · f64 | 513.125 | — | 365.251 | 398.048 | — | **340.891** | +6.7% |
| Cholesky factorization · 64 × 64 · f64 | 20.684 | — | 13.137 | **8.396** | — | 10.231 | -2.4% |
| Cholesky factorization · 128 × 128 · f64 | 122.24299999999999 | — | 96.829 | **37.323** | — | 68.058 | +10.1% |
| Cholesky factorization · 256 × 256 · f64 | 619.867 | — | 639.843 | **191.37** | — | 510.375 | +0.9% |
| Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64 | **2.366** | — | — | — | 4.268 | — | -8.7% |
| Model inference · batched normalized/sliced MLP · f64 | **2.092** | — | — | — | 3.533 | — | -2.1% |
| Model recording + forward · batched normalized/sliced MLP · f64 | **2.656** | — | — | — | 5.141 | — | +2.9% |
| Model backward · prepared graph → fresh gradients · f64 | **3.213** | — | — | — | 13.387 | — | +4.3% |
| Parameter update · transactional SGD, fixed gradients · f64 | 0.552 | — | — | — | — | — | +10.8% |
| Row softmax · 256 × 64 · f64 | 83.691 | **71.24** | — | — | 80.473 | 99.419 | +0.1% |
| Create zeros · 64 × 64 · f32 | 0.133 | **0.098** | — | 0.141 | 0.155 | 0.353 | -2.2% |
| Create ones · 64 × 64 · f32 | 0.203 | **0.154** | — | 0.184 | 0.237 | 1.348 | +0.0% |

## Version comparison

7 of 25 workloads have paired evidence. Exploratory 95% intervals resample process pairs, not calls. No multiple-comparison adjustment.

| Workload | Current / baseline [95% interval] | Interpretation |
|---|---:|---|
| Matrix multiplication · 32 × 32 · 32 × 32 · f64 | — | No paired comparison |
| Matrix multiplication · 64 × 64 · 64 × 64 · f64 | — | No paired comparison |
| Matrix multiplication · 128 × 128 · 128 × 128 · f64 | 1.0018462734428142× [0.9800260353103897, 1.0123633757391148] | Timing direction unclear |
| Matrix multiplication · 256 × 256 · 256 × 256 · f64 | — | No paired comparison |
| Matrix multiplication · 64 × 64 · 64 × 64 · f32 | — | No paired comparison |
| Matrix multiplication · 128 × 128 · 128 × 128 · f32 | 0.9867207984437114× [0.9328305380091483, 1.0138829376981746] | Timing direction unclear |
| Matrix multiplication · 256 × 256 · 256 × 256 · f32 | — | No paired comparison |
| Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64 | — | No paired comparison |
| Column-pivoted QR · 96 × 32 · f64 | — | No paired comparison |
| Column-pivoted QR · 256 × 64 · f64 | — | No paired comparison |
| Minimum-norm least squares · 64 × 32, rank 16, 4 RHS · f64 | — | No paired comparison |
| Minimum-norm least squares · 32 × 64, rank 32, 4 RHS · f64 | — | No paired comparison |
| Thin QR · 96 × 32 · f64 | 1.0491846686820097× [0.9656443002198263, 1.0593508293468055] | Timing direction unclear |
| Thin QR · 256 × 64 · f64 | 1.020154379165327× [0.9954990521073648, 1.0457764236184073] | Timing direction unclear |
| Cholesky factorization · 64 × 64 · f64 | 0.9773944919635305× [0.9235327313769752, 1.0403771163433535] | Timing direction unclear |
| Cholesky factorization · 128 × 128 · f64 | 0.9789425417092872× [0.9268104333538715, 1.018511826247222] | Timing direction unclear |
| Cholesky factorization · 256 × 256 · f64 | 1.0069935500152878× [0.9932964002130146, 1.025894234196159] | Timing direction unclear |
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

Measured source `4e40ea5a98e7f98505da7cc3dd8a2d80c1a4bd92`. Run `0.9.0-final`. Rendering does not change measurements.
