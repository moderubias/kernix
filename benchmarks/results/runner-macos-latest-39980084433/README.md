# Kernix 0.9.0 · performance brief

Apple M1 (Virtual) / macOS

**Per-call median, microseconds per operation. Lower is faster. All recorded digits retained.**

25 workloads · 100 observations

No previous-version baseline attached. Sequential run changes are descriptive; paired process evidence is shown separately.

[Interactive HTML (open locally)](report.html) · [Vector PDF](latency.pdf) · [Raw measurements](raw.json)

[![Performance brief preview](latency.png)](latency.pdf)

## Charts by workload

**Shorter bars mean less time.** Blue is Kernix; grey is a comparison implementation; teal is an explicitly selected Kernix backend. Entries are sorted by the recorded median, with a separate zero-based scale per chart. These point observations do not establish a statistical ranking.

Expand a group to see every chart here on GitHub. Click a chart to enlarge it. The exact table below keeps every source digit.

<details>
<summary>Matrix multiplication · 8 workloads</summary>

**Matrix multiplication · 32 × 32 · 32 × 32 · f64**

![Matrix multiplication · 32 × 32 · 32 × 32 · f64. Shorter bars mean less time.](charts/matmul_32_f64.png)

Kernix: **2.25 µs**. Kernix took ≈ 46.0% more time than NumPy in this run.

**Matrix multiplication · 64 × 64 · 64 × 64 · f64**

![Matrix multiplication · 64 × 64 · 64 × 64 · f64. Shorter bars mean less time.](charts/matmul_64_f64.png)

Kernix: **12.959 µs**. Kernix took ≈ 270.3% more time than NumPy in this run.

**Matrix multiplication · 128 × 128 · 128 × 128 · f64**

![Matrix multiplication · 128 × 128 · 128 × 128 · f64. Shorter bars mean less time.](charts/matmul_128_f64.png)

Kernix: **102.541 µs**. Kernix took ≈ 480.4% more time than NumPy in this run.

**Matrix multiplication · 256 × 256 · 256 × 256 · f64**

![Matrix multiplication · 256 × 256 · 256 × 256 · f64. Shorter bars mean less time.](charts/matmul_256_f64.png)

Kernix: **845.417 µs**. Kernix took ≈ 599.7% more time than NumPy in this run.

**Matrix multiplication · 64 × 64 · 64 × 64 · f32**

![Matrix multiplication · 64 × 64 · 64 × 64 · f32. Shorter bars mean less time.](charts/matmul_64_f32.png)

Kernix: **10.5 µs**. Kernix took ≈ 448.0% more time than NumPy in this run.

**Matrix multiplication · 128 × 128 · 128 × 128 · f32**

![Matrix multiplication · 128 × 128 · 128 × 128 · f32. Shorter bars mean less time.](charts/matmul_128_f32.png)

Kernix: **52.459 µs**. Kernix took ≈ 756.5% more time than NumPy in this run.

**Matrix multiplication · 256 × 256 · 256 × 256 · f32**

![Matrix multiplication · 256 × 256 · 256 × 256 · f32. Shorter bars mean less time.](charts/matmul_256_f32.png)

Kernix: **372.5 µs**. Kernix took ≈ 833.2% more time than NumPy in this run.

**Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64**

![Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64. Shorter bars mean less time.](charts/matmul_batched_4x16x32x16_f64.png)

Kernix: **2.875 µs**. Kernix took ≈ 60.5% more time than NumPy in this run.

</details>

<details>
<summary>Decompositions & solvers · 9 workloads</summary>

**Column-pivoted QR · 96 × 32 · f64**

![Column-pivoted QR · 96 × 32 · f64. Shorter bars mean less time.](charts/pivoted_qr_96x32_f64.png)

Kernix: **73.25 µs**. Kernix took ≈ 82.4% more time than faer in this run.

**Column-pivoted QR · 256 × 64 · f64**

![Column-pivoted QR · 256 × 64 · f64. Shorter bars mean less time.](charts/pivoted_qr_256x64_f64.png)

Kernix: **605.417 µs**. Kernix took ≈ 95.4% more time than faer in this run.

**Minimum-norm least squares · 64 × 32, rank 16, 4 RHS · f64**

![Minimum-norm least squares · 64 × 32, rank 16, 4 RHS · f64. Shorter bars mean less time.](charts/min_norm_64x32_rank16_f64.png)

Kernix: **51.25 µs**. Kernix took ≈ 26.6% less time than nalgebra in this run.

**Minimum-norm least squares · 32 × 64, rank 32, 4 RHS · f64**

![Minimum-norm least squares · 32 × 64, rank 32, 4 RHS · f64. Shorter bars mean less time.](charts/min_norm_32x64_rank32_f64.png)

Kernix: **95.917 µs**. Kernix took ≈ 29.5% less time than NumPy in this run.

**Thin QR · 96 × 32 · f64**

![Thin QR · 96 × 32 · f64. Shorter bars mean less time.](charts/qr_96x32_f64.png)

Kernix: **56.792 µs**. Kernix took ≈ 30.4% more time than nalgebra in this run.

**Thin QR · 256 × 64 · f64**

![Thin QR · 256 × 64 · f64. Shorter bars mean less time.](charts/qr_256x64_f64.png)

Kernix: **497.91699999999994 µs**. Kernix took ≈ 64.2% more time than faer in this run.

**Cholesky factorization · 64 × 64 · f64**

![Cholesky factorization · 64 × 64 · f64. Shorter bars mean less time.](charts/cholesky_factor_64_f64.png)

Kernix: **22.292 µs**. Kernix took ≈ 115.7% more time than faer in this run.

**Cholesky factorization · 128 × 128 · f64**

![Cholesky factorization · 128 × 128 · f64. Shorter bars mean less time.](charts/cholesky_factor_128_f64.png)

Kernix: **167.667 µs**. Kernix took ≈ 238.7% more time than faer in this run.

**Cholesky factorization · 256 × 256 · f64**

![Cholesky factorization · 256 × 256 · f64. Shorter bars mean less time.](charts/cholesky_factor_256_f64.png)

Kernix: **1322.125 µs**. Kernix took ≈ 427.4% more time than faer in this run.

</details>

<details>
<summary>Training & autodiff · 5 workloads</summary>

**Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64**

![Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64. Shorter bars mean less time.](charts/matmul_sum_backward_32x16x8_f64.png)

Kernix: **8.042 µs**. Kernix took ≈ 58.6% less time than Candle in this run.

**Model inference · batched normalized/sliced MLP · f64**

![Model inference · batched normalized/sliced MLP · f64. Shorter bars mean less time.](charts/training_forward_2x4x8x4_f64.png)

Kernix: **2.25 µs**. Kernix took ≈ 58.8% less time than Candle in this run.

**Model recording + forward · batched normalized/sliced MLP · f64**

![Model recording + forward · batched normalized/sliced MLP · f64. Shorter bars mean less time.](charts/training_record_2x4x8x4_f64.png)

Kernix: **2.875 µs**. Kernix took ≈ 48.9% less time than Candle in this run.

**Model backward · prepared graph → fresh gradients · f64**

![Model backward · prepared graph → fresh gradients · f64. Shorter bars mean less time.](charts/training_backward_2x4x8x4_f64.png)

Kernix: **3.75 µs**. Kernix took ≈ 82.1% less time than Candle in this run.

**Parameter update · transactional SGD, fixed gradients · f64**

![Parameter update · transactional SGD, fixed gradients · f64. Shorter bars mean less time.](charts/training_sgd_2x4x8x4_f64.png)

Kernix: **0.75 µs**. No external comparison with Kernix was measured for this workload.

</details>

<details>
<summary>Statistics & activations · 1 workloads</summary>

**Row softmax · 256 × 64 · f64**

![Row softmax · 256 × 64 · f64. Shorter bars mean less time.](charts/softmax_256x64_f64.png)

Kernix: **78.917 µs**. Kernix took ≈ 21.4% more time than Candle in this run.

</details>

<details>
<summary>Tensor construction · 2 workloads</summary>

**Create zeros · 64 × 64 · f32**

![Create zeros · 64 × 64 · f32. Shorter bars mean less time.](charts/create_zeros_64x64_f32.png)

Kernix: **0.75 µs**. Kernix took ≈ 157.7% more time than faer in this run.

**Create ones · 64 × 64 · f32**

![Create ones · 64 × 64 · f32. Shorter bars mean less time.](charts/create_ones_64x64_f32.png)

Kernix: **0.292 µs**. Kernix took ≈ 12.3% less time than faer in this run.

</details>

## Exact latency table

All values below are microseconds per operation. **Lower is faster.** Missing entries were not measured.

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
