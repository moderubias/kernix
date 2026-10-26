# Kernix 0.12.0 · performance brief

Apple M1 (Virtual) / macOS

**Per-call median, microseconds per operation. Lower is faster. All recorded digits retained.**

46 workloads · 156 observations

Baseline: baseline-macos-latest-40270611933. Sequential run changes are descriptive; paired process evidence is shown separately.

[Interactive HTML (open locally)](report.html) · [Vector PDF](latency.pdf) · [Raw measurements](raw.json)

[![Performance brief preview](latency.png)](latency.pdf)

## Charts by workload

**Shorter bars mean less time.** Blue is Kernix; grey is a comparison implementation; teal is an explicitly selected Kernix backend. Entries are sorted by the recorded median, with a separate zero-based scale per chart. These point observations do not establish a statistical ranking.

Expand a group to see every chart here on GitHub. Click a chart to enlarge it. The exact table below keeps every source digit.

<details>
<summary>Matrix multiplication · 13 workloads</summary>

**Matrix multiplication · transposed view · 128 × 128 · 128 × 128 · f64**

![Matrix multiplication · transposed view · 128 × 128 · 128 × 128 · f64. Shorter bars mean less time.](charts/matmul_transposed_128_f64.png)

Kernix: **105.833 µs**. Kernix took ≈ 573.8% more time than NumPy in this run.

**Matrix multiplication · transposed view · 128 × 128 · 128 × 128 · f32**

![Matrix multiplication · transposed view · 128 × 128 · 128 × 128 · f32. Shorter bars mean less time.](charts/matmul_transposed_128_f32.png)

Kernix: **58.291 µs**. Kernix took ≈ 1056.1% more time than NumPy in this run.

**Independent matrix operands · 128 × 128 · 128 × 128 · f64**

![Independent matrix operands · 128 × 128 · 128 × 128 · f64. Shorter bars mean less time.](charts/matmul_ab_128_f64.png)

Kernix: **95.792 µs**. Kernix took ≈ 442.2% more time than NumPy in this run.

**Independent matrix operands · 128 × 128 · 128 × 128 · f32**

![Independent matrix operands · 128 × 128 · 128 × 128 · f32. Shorter bars mean less time.](charts/matmul_ab_128_f32.png)

Kernix: **53 µs**. Kernix took ≈ 802.1% more time than NumPy in this run.

**Matrix multiplication · 32 × 32 · 32 × 32 · f64**

![Matrix multiplication · 32 × 32 · 32 × 32 · f64. Shorter bars mean less time.](charts/matmul_32_f64.png)

Kernix: **2.166 µs**. Kernix took ≈ 67.6% more time than NumPy in this run.

**Matrix multiplication · 64 × 64 · 64 × 64 · f64**

![Matrix multiplication · 64 × 64 · 64 × 64 · f64. Shorter bars mean less time.](charts/matmul_64_f64.png)

Kernix: **13.959 µs**. Kernix took ≈ 298.8% more time than NumPy in this run.

**Matrix multiplication · 128 × 128 · 128 × 128 · f64**

![Matrix multiplication · 128 × 128 · 128 × 128 · f64. Shorter bars mean less time.](charts/matmul_128_f64.png)

Kernix: **96.667 µs**. Kernix took ≈ 440.8% more time than NumPy in this run.

**Matrix multiplication · 256 × 256 · 256 × 256 · f64**

![Matrix multiplication · 256 × 256 · 256 × 256 · f64. Shorter bars mean less time.](charts/matmul_256_f64.png)

Kernix: **741.292 µs**. Kernix took ≈ 533.6% more time than NumPy in this run.

**Matrix multiplication · 64 × 64 · 64 × 64 · f32**

![Matrix multiplication · 64 × 64 · 64 × 64 · f32. Shorter bars mean less time.](charts/matmul_64_f32.png)

Kernix: **7.708 µs**. Kernix took ≈ 302.1% more time than NumPy in this run.

**Matrix multiplication · 128 × 128 · 128 × 128 · f32**

![Matrix multiplication · 128 × 128 · 128 × 128 · f32. Shorter bars mean less time.](charts/matmul_128_f32.png)

Kernix: **48.916 µs**. Kernix took ≈ 726.7% more time than NumPy in this run.

**Matrix multiplication · 256 × 256 · 256 × 256 · f32**

![Matrix multiplication · 256 × 256 · 256 × 256 · f32. Shorter bars mean less time.](charts/matmul_256_f32.png)

Kernix: **366.667 µs**. Kernix took ≈ 939.0% more time than NumPy in this run.

**Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64**

![Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64. Shorter bars mean less time.](charts/matmul_batched_4x16x32x16_f64.png)

Kernix: **2.958 µs**. Kernix took ≈ 41.9% more time than NumPy in this run.

**Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f32**

![Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f32. Shorter bars mean less time.](charts/matmul_batched_4x16x32x16_f32.png)

Kernix: **5.167 µs**. Kernix took ≈ 133.9% more time than NumPy in this run.

</details>

<details>
<summary>Explicit parallel execution · 4 workloads</summary>

**Matmul · 2 workers · reused output · 64 × 64 · 64 × 64 · f64**

![Matmul · 2 workers · reused output · 64 × 64 · 64 × 64 · f64. Shorter bars mean less time.](charts/parallel_matmul_into_64_f64.png)

Kernix: **50.583 µs**. No external comparison with Kernix was measured for this workload.

**Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f64**

![Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f64. Shorter bars mean less time.](charts/parallel_matmul_into_128_f64.png)

Kernix: **152.791 µs**. No external comparison with Kernix was measured for this workload.

**Matmul · 2 workers · reused output · 256 × 256 · 256 × 256 · f64**

![Matmul · 2 workers · reused output · 256 × 256 · 256 × 256 · f64. Shorter bars mean less time.](charts/parallel_matmul_into_256_f64.png)

Kernix: **1184.417 µs**. No external comparison with Kernix was measured for this workload.

**Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f32**

![Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f32. Shorter bars mean less time.](charts/parallel_matmul_into_128_f32.png)

Kernix: **47.5 µs**. No external comparison with Kernix was measured for this workload.

</details>

<details>
<summary>Decompositions & solvers · 19 workloads</summary>

**Cholesky solve · reused factor · 128 × 128, 1 RHS · f64**

![Cholesky solve · reused factor · 128 × 128, 1 RHS · f64. Shorter bars mean less time.](charts/solve_cholesky_128x1_f64.png)

Kernix: **18.958 µs**. Kernix took ≈ 329.2% more time than nalgebra in this run.

**Cholesky solve · reused factor · 256 × 256, 64 RHS · f64**

![Cholesky solve · reused factor · 256 × 256, 64 RHS · f64. Shorter bars mean less time.](charts/solve_cholesky_256x64_f64.png)

Kernix: **1252.875 µs**. Kernix took ≈ 321.4% more time than faer in this run.

**Cholesky solve · reused factor · 512 × 512, 128 RHS · f64**

![Cholesky solve · reused factor · 512 × 512, 128 RHS · f64. Shorter bars mean less time.](charts/solve_cholesky_512x128_f64.png)

Kernix: **13071.542 µs**. Kernix took ≈ 485.0% more time than faer in this run.

**LU solve · reused factor · 256 × 256, 64 RHS · f64**

![LU solve · reused factor · 256 × 256, 64 RHS · f64. Shorter bars mean less time.](charts/solve_lu_256x64_f64.png)

Kernix: **1370.666 µs**. Kernix took ≈ 324.9% more time than faer in this run.

**Cholesky rank-one update · 256 × 256 · f64**

![Cholesky rank-one update · 256 × 256 · f64. Shorter bars mean less time.](charts/cholesky_rank_update_256_f64.png)

Kernix: **120.834 µs**. Kernix took ≈ 85.3% more time than nalgebra in this run.

**Cholesky rank-one downdate · 256 × 256 · f64**

![Cholesky rank-one downdate · 256 × 256 · f64. Shorter bars mean less time.](charts/cholesky_rank_downdate_256_f64.png)

Kernix: **110.625 µs**. Kernix took ≈ 447.4% more time than nalgebra in this run.

**Cholesky rank-one update · 512 × 512 · f64**

![Cholesky rank-one update · 512 × 512 · f64. Shorter bars mean less time.](charts/cholesky_rank_update_512_f64.png)

Kernix: **558.5 µs**. Kernix took ≈ 453.4% more time than nalgebra in this run.

**Thin QR · 512 × 128 · f64**

![Thin QR · 512 × 128 · f64. Shorter bars mean less time.](charts/qr_512x128_f64.png)

Kernix: **2764 µs**. Kernix took ≈ 66.8% more time than faer in this run.

**Thin QR · 1024 × 256 · f64**

![Thin QR · 1024 × 256 · f64. Shorter bars mean less time.](charts/qr_1024x256_f64.png)

Kernix: **15596.583 µs**. Kernix took ≈ 23.4% more time than faer in this run.

**Cholesky factorization · 512 × 512 · f64**

![Cholesky factorization · 512 × 512 · f64. Shorter bars mean less time.](charts/cholesky_factor_512_f64.png)

Kernix: **3040.417 µs**. Kernix took ≈ 70.0% more time than faer in this run.

**Column-pivoted QR · 96 × 32 · f64**

![Column-pivoted QR · 96 × 32 · f64. Shorter bars mean less time.](charts/pivoted_qr_96x32_f64.png)

Kernix: **77.542 µs**. Kernix took ≈ 19.7% less time than nalgebra in this run.

**Column-pivoted QR · 256 × 64 · f64**

![Column-pivoted QR · 256 × 64 · f64. Shorter bars mean less time.](charts/pivoted_qr_256x64_f64.png)

Kernix: **533.333 µs**. Kernix took ≈ 63.1% more time than faer in this run.

**Minimum-norm least squares · 64 × 32, rank 16, 4 RHS · f64**

![Minimum-norm least squares · 64 × 32, rank 16, 4 RHS · f64. Shorter bars mean less time.](charts/min_norm_64x32_rank16_f64.png)

Kernix: **54.375 µs**. Kernix took ≈ 25.0% less time than nalgebra in this run.

**Minimum-norm least squares · 32 × 64, rank 32, 4 RHS · f64**

![Minimum-norm least squares · 32 × 64, rank 32, 4 RHS · f64. Shorter bars mean less time.](charts/min_norm_32x64_rank32_f64.png)

Kernix: **73.583 µs**. Kernix took ≈ 46.1% less time than NumPy in this run.

**Thin QR · 96 × 32 · f64**

![Thin QR · 96 × 32 · f64. Shorter bars mean less time.](charts/qr_96x32_f64.png)

Kernix: **55.333 µs**. Kernix took ≈ 22.3% more time than nalgebra in this run.

**Thin QR · 256 × 64 · f64**

![Thin QR · 256 × 64 · f64. Shorter bars mean less time.](charts/qr_256x64_f64.png)

Kernix: **563.25 µs**. Kernix took ≈ 97.2% more time than faer in this run.

**Cholesky factorization · 64 × 64 · f64**

![Cholesky factorization · 64 × 64 · f64. Shorter bars mean less time.](charts/cholesky_factor_64_f64.png)

Kernix: **20.208 µs**. Kernix took ≈ 100.4% more time than faer in this run.

**Cholesky factorization · 128 × 128 · f64**

![Cholesky factorization · 128 × 128 · f64. Shorter bars mean less time.](charts/cholesky_factor_128_f64.png)

Kernix: **165.833 µs**. Kernix took ≈ 251.9% more time than NumPy in this run.

**Cholesky factorization · 256 × 256 · f64**

![Cholesky factorization · 256 × 256 · f64. Shorter bars mean less time.](charts/cholesky_factor_256_f64.png)

Kernix: **588.292 µs**. Kernix took ≈ 111.6% more time than faer in this run.

</details>

<details>
<summary>Training & autodiff · 6 workloads</summary>

**Shared-variable backward · sum(X * X), 64 × 32; prepared graph · f64**

![Shared-variable backward · sum(X * X), 64 × 32; prepared graph · f64. Shorter bars mean less time.](charts/training_shared_backward_64x32_f64.png)

Kernix: **6.375 µs**. Kernix took ≈ 40.4% more time than Candle in this run.

**Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64**

![Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64. Shorter bars mean less time.](charts/matmul_sum_backward_32x16x8_f64.png)

Kernix: **3 µs**. Kernix took ≈ 54.4% less time than Candle in this run.

**Model inference · batched normalized/sliced MLP · f64**

![Model inference · batched normalized/sliced MLP · f64. Shorter bars mean less time.](charts/training_forward_2x4x8x4_f64.png)

Kernix: **2.084 µs**. Kernix took ≈ 67.7% less time than Candle in this run.

**Model recording + forward · batched normalized/sliced MLP · f64**

![Model recording + forward · batched normalized/sliced MLP · f64. Shorter bars mean less time.](charts/training_record_2x4x8x4_f64.png)

Kernix: **3.458 µs**. Kernix took ≈ 36.6% less time than Candle in this run.

**Model backward · prepared graph → fresh gradients · f64**

![Model backward · prepared graph → fresh gradients · f64. Shorter bars mean less time.](charts/training_backward_2x4x8x4_f64.png)

Kernix: **3.667 µs**. Kernix took ≈ 80.8% less time than Candle in this run.

**Parameter update · transactional SGD, fixed gradients · f64**

![Parameter update · transactional SGD, fixed gradients · f64. Shorter bars mean less time.](charts/training_sgd_2x4x8x4_f64.png)

Kernix: **0.75 µs**. No external comparison with Kernix was measured for this workload.

</details>

<details>
<summary>Tensor contractions · 1 workloads</summary>

**Coalesced tensor contraction · 16 × 32 × 8 · 32 × 8 × 16 · f64**

![Coalesced tensor contraction · 16 × 32 × 8 · 32 × 8 × 16 · f64. Shorter bars mean less time.](charts/tensordot_16x32x8_32x8x16_f64.png)

Kernix: **64.458 µs**. Kernix took ≈ 1005.1% more time than NumPy in this run.

</details>

<details>
<summary>Statistics & activations · 1 workloads</summary>

**Row softmax · 256 × 64 · f64**

![Row softmax · 256 × 64 · f64. Shorter bars mean less time.](charts/softmax_256x64_f64.png)

Kernix: **84.541 µs**. Kernix took ≈ 28.1% more time than ndarray in this run.

</details>

<details>
<summary>Tensor construction · 2 workloads</summary>

**Create zeros · 64 × 64 · f32**

![Create zeros · 64 × 64 · f32. Shorter bars mean less time.](charts/create_zeros_64x64_f32.png)

Kernix: **0.291 µs**. Kernix took ≈ 16.4% more time than faer in this run.

**Create ones · 64 × 64 · f32**

![Create ones · 64 × 64 · f32. Shorter bars mean less time.](charts/create_ones_64x64_f32.png)

Kernix: **0.333 µs**. Kernix took ≈ 33.2% more time than ndarray in this run.

</details>

## Exact latency table

All values below are microseconds per operation. **Lower is faster.** Missing entries were not measured.

| Workload | Kernix | Kernix · explicit Accelerate | ndarray | nalgebra | faer | Candle | NumPy | Kernix refactor | Sequential run change |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| Matrix multiplication · transposed view · 128 × 128 · 128 × 128 · f64 | 105.833 | — | — | — | — | — | **15.708** | — | -13.4% |
| Matrix multiplication · transposed view · 128 × 128 · 128 × 128 · f32 | 58.291 | — | — | — | — | — | **5.042** | — | +22.0% |
| Independent matrix operands · 128 × 128 · 128 × 128 · f64 | 95.792 | — | — | — | — | — | **17.667** | — | +0.0% |
| Independent matrix operands · 128 × 128 · 128 × 128 · f32 | 53 | — | — | — | — | — | **5.875** | — | +0.2% |
| Matrix multiplication · 32 × 32 · 32 × 32 · f64 | 2.166 | — | 2.125 | 2.542 | 2.417 | 4.458 | **1.292** | — | +0.0% |
| Matrix multiplication · 64 × 64 · 64 × 64 · f64 | 13.959 | — | 14.333 | 12.625 | 13.75 | 14.833 | **3.5** | — | +7.7% |
| Matrix multiplication · 128 × 128 · 128 × 128 · f64 | 96.667 | **17.708** | 94.083 | 102 | 107.083 | 107.042 | 17.875 | — | +1.1% |
| Matrix multiplication · 256 × 256 · 256 × 256 · f64 | 741.292 | 123.58399999999999 | 815.042 | 797.041 | 893.792 | 876.167 | **117** | — | +2.1% |
| Matrix multiplication · 64 × 64 · 64 × 64 · f32 | 7.708 | — | 7.083 | 6.917 | 7.75 | 7.334 | **1.917** | — | +14.2% |
| Matrix multiplication · 128 × 128 · 128 × 128 · f32 | 48.916 | **5.834** | 49.542 | 50.875 | 51.541 | 51.125 | 5.917 | — | -0.8% |
| Matrix multiplication · 256 × 256 · 256 × 256 · f32 | 366.667 | 35.625 | 365.334 | 438.25 | 436.083 | 428.916 | **35.291** | — | +1.7% |
| Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64 | 2.958 | — | — | — | — | 3.75 | **2.084** | — | +7.6% |
| Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f32 | 5.167 | — | — | — | — | 2.708 | **2.209** | — | +163.9% |
| Matmul · 2 workers · reused output · 64 × 64 · 64 × 64 · f64 | 50.583 | — | — | — | — | — | — | — | +159.4% |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f64 | 152.791 | — | — | — | — | — | — | — | +133.7% |
| Matmul · 2 workers · reused output · 256 × 256 · 256 × 256 · f64 | 1184.417 | — | — | — | — | — | — | — | +193.3% |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f32 | 47.5 | — | — | — | — | — | — | — | +1.5% |
| Cholesky solve · reused factor · 128 × 128, 1 RHS · f64 | 18.958 | — | — | **4.417** | 5.666 | — | — | — | +1.8% |
| Cholesky solve · reused factor · 256 × 256, 64 RHS · f64 | 1252.875 | — | — | 930.5 | **297.292** | — | — | — | -63.8% |
| Cholesky solve · reused factor · 512 × 512, 128 RHS · f64 | 13071.542 | — | — | 7049.125 | **2234.375** | — | — | — | -64.2% |
| LU solve · reused factor · 256 × 256, 64 RHS · f64 | 1370.666 | — | — | 951 | **322.584** | — | — | — | -63.7% |
| Cholesky rank-one update · 256 × 256 · f64 | 120.834 | — | — | **65.20900000000001** | — | — | — | 643.25 | — |
| Cholesky rank-one downdate · 256 × 256 · f64 | 110.625 | — | — | **20.209** | — | — | — | 585.125 | — |
| Cholesky rank-one update · 512 × 512 · f64 | 558.5 | — | — | **100.916** | — | — | — | 3070.125 | — |
| Thin QR · 512 × 128 · f64 | 2764 | — | — | 2897.042 | **1656.917** | — | 2239.542 | — | -31.1% |
| Thin QR · 1024 × 256 · f64 | 15596.583 | — | — | 24327.084 | **12638.334** | — | 13289.792 | — | -52.9% |
| Cholesky factorization · 512 × 512 · f64 | 3040.417 | — | — | 5920.084 | **1788** | — | 2407.75 | — | +1.0% |
| Column-pivoted QR · 96 × 32 · f64 | **77.542** | — | — | 96.542 | 124.334 | — | — | — | +14.7% |
| Column-pivoted QR · 256 × 64 · f64 | 533.333 | — | — | 1064.5 | **326.958** | — | — | — | -11.9% |
| Minimum-norm least squares · 64 × 32, rank 16, 4 RHS · f64 | **54.375** | — | — | 72.458 | — | — | 79.542 | — | +19.1% |
| Minimum-norm least squares · 32 × 64, rank 32, 4 RHS · f64 | **73.583** | — | — | 155 | — | — | 136.417 | — | +21.4% |
| Thin QR · 96 × 32 · f64 | 55.333 | — | — | **45.25** | 50.208 | — | 46.958 | — | +4.3% |
| Thin QR · 256 × 64 · f64 | 563.25 | — | — | 331.75 | **285.583** | — | 373.667 | — | +9.1% |
| Cholesky factorization · 64 × 64 · f64 | 20.208 | — | — | 19.5 | **10.083** | — | 13.75 | — | -7.8% |
| Cholesky factorization · 128 × 128 · f64 | 165.833 | — | — | 117 | 51.542 | — | **47.125** | — | -18.1% |
| Cholesky factorization · 256 × 256 · f64 | 588.292 | — | — | 669.292 | **278** | — | 332.25 | — | -5.3% |
| Shared-variable backward · sum(X * X), 64 × 32; prepared graph · f64 | 6.375 | — | — | — | — | **4.541** | — | — | +6.2% |
| Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64 | **3** | — | — | — | — | 6.583 | — | — | -2.7% |
| Model inference · batched normalized/sliced MLP · f64 | **2.084** | — | — | — | — | 6.458 | — | — | -3.8% |
| Model recording + forward · batched normalized/sliced MLP · f64 | **3.458** | — | — | — | — | 5.458 | — | — | +6.4% |
| Model backward · prepared graph → fresh gradients · f64 | **3.667** | — | — | — | — | 19.125 | — | — | +12.8% |
| Parameter update · transactional SGD, fixed gradients · f64 | 0.75 | — | — | — | — | — | — | — | -5.2% |
| Coalesced tensor contraction · 16 × 32 × 8 · 32 × 8 × 16 · f64 | 64.458 | — | — | — | — | — | **5.833** | — | -0.0% |
| Row softmax · 256 × 64 · f64 | 84.541 | — | **66** | — | — | 75.625 | 68.958 | — | +0.5% |
| Create zeros · 64 × 64 · f32 | 0.291 | — | 0.333 | — | **0.25** | 0.333 | 0.584 | — | -0.3% |
| Create ones · 64 × 64 · f32 | 0.333 | — | **0.25** | — | 0.375 | 0.542 | 1.292 | — | -55.6% |

## Version comparison

24 of 46 workloads have paired evidence. Exploratory 95% intervals resample process pairs, not calls. No multiple-comparison adjustment.


### Version changes at a glance

The vertical line marks unchanged time. Left is faster; right is slower. Ranges are exploratory 95% intervals over fresh-process pairs. An interval crossing the line leaves direction unclear.

![Matrix multiplication. Current divided by baseline; below 1 is faster.](charts/paired-matmul-1.png)

![Matrix multiplication. Current divided by baseline; below 1 is faster.](charts/paired-matmul-2.png)

![Explicit parallel execution. Current divided by baseline; below 1 is faster.](charts/paired-parallel-1.png)

![Decompositions & solvers. Current divided by baseline; below 1 is faster.](charts/paired-algebra-1.png)

![Decompositions & solvers. Current divided by baseline; below 1 is faster.](charts/paired-algebra-2.png)

![Training & autodiff. Current divided by baseline; below 1 is faster.](charts/paired-training-1.png)

| Workload | Current / baseline [95% interval] | Interpretation |
|---|---:|---|
| Matrix multiplication · transposed view · 128 × 128 · 128 × 128 · f64 | 1.0065869156092198× [0.9334875283446712, 1.0366644664466447] | Timing direction unclear |
| Matrix multiplication · transposed view · 128 × 128 · 128 × 128 · f32 | 1× [0.9692970033422081, 1.0242323200992556] | Timing direction unclear |
| Independent matrix operands · 128 × 128 · 128 × 128 · f64 | 0.9959645031113004× [0.9219876288659794, 1.0164903225806452] | Timing direction unclear |
| Independent matrix operands · 128 × 128 · 128 × 128 · f32 | 1.0047169811320755× [0.9952867538931413, 1.039894165994258] | Timing direction unclear |
| Matrix multiplication · 32 × 32 · 32 × 32 · f64 | 1× [0.9642077695329551, 1.0186666666666666] | Timing direction unclear |
| Matrix multiplication · 64 × 64 · 64 × 64 · f64 | — | No paired comparison |
| Matrix multiplication · 128 × 128 · 128 × 128 · f64 | 1.0341187136270142× [0.9979659367396594, 1.0490804116484436] | Timing direction unclear |
| Matrix multiplication · 256 × 256 · 256 × 256 · f64 | 0.9968545275313336× [0.9725039674746369, 1.0434932624259767] | Timing direction unclear |
| Matrix multiplication · 64 × 64 · 64 × 64 · f32 | — | No paired comparison |
| Matrix multiplication · 128 × 128 · 128 × 128 · f32 | 0.9852494088503547× [0.9299589318823753, 1.0053293424102498] | Timing direction unclear |
| Matrix multiplication · 256 × 256 · 256 × 256 · f32 | 0.9854261083743843× [0.9696438041145655, 0.995589617708063] | Observed time reduction |
| Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64 | 1× [0.8290898448931812, 1.0305454545454547] | Timing direction unclear |
| Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f32 | 1.0229132569558101× [0.9333333333333333, 1.0463169642857142] | Timing direction unclear |
| Matmul · 2 workers · reused output · 64 × 64 · 64 × 64 · f64 | — | No paired comparison |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f64 | 1.0216484838542095× [0.5988323603002502, 1.2320360685302074] | Timing direction unclear |
| Matmul · 2 workers · reused output · 256 × 256 · 256 × 256 · f64 | 1.0137858548492327× [0.7405825022956959, 1.0541735305919873] | Timing direction unclear |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f32 | 1.0029569620253165× [0.9843949367088608, 2.0068221741084193] | Timing direction unclear |
| Cholesky solve · reused factor · 128 × 128, 1 RHS · f64 | 0.9999363422242026× [0.8434728164457894, 1.0342578520770012] | Timing direction unclear |
| Cholesky solve · reused factor · 256 × 256, 64 RHS · f64 | 0.3391463976714771× [0.3347444354363207, 0.34212995784873657] | Observed time reduction |
| Cholesky solve · reused factor · 512 × 512, 128 RHS · f64 | — | No paired comparison |
| LU solve · reused factor · 256 × 256, 64 RHS · f64 | 0.3395659435871721× [0.29953666777081356, 0.3520476746781338] | Observed time reduction |
| Cholesky rank-one update · 256 × 256 · f64 | — | No paired comparison |
| Cholesky rank-one downdate · 256 × 256 · f64 | — | No paired comparison |
| Cholesky rank-one update · 512 × 512 · f64 | — | No paired comparison |
| Thin QR · 512 × 128 · f64 | 0.6364933793072876× [0.6209107889725535, 0.6756612892776158] | Observed time reduction |
| Thin QR · 1024 × 256 · f64 | 0.5251560195707211× [0.4995823827409169, 0.533394334068534] | Observed time reduction |
| Cholesky factorization · 512 × 512 · f64 | 0.9951801061606067× [0.9423570914820015, 1.0506741840165743] | Timing direction unclear |
| Column-pivoted QR · 96 × 32 · f64 | — | No paired comparison |
| Column-pivoted QR · 256 × 64 · f64 | — | No paired comparison |
| Minimum-norm least squares · 64 × 32, rank 16, 4 RHS · f64 | — | No paired comparison |
| Minimum-norm least squares · 32 × 64, rank 32, 4 RHS · f64 | — | No paired comparison |
| Thin QR · 96 × 32 · f64 | — | No paired comparison |
| Thin QR · 256 × 64 · f64 | 1.0203252032520325× [0.9828453796889296, 1.0506988647316935] | Timing direction unclear |
| Cholesky factorization · 64 × 64 · f64 | — | No paired comparison |
| Cholesky factorization · 128 × 128 · f64 | 0.995259977566306× [0.9373491217847332, 1.0350073578595318] | Timing direction unclear |
| Cholesky factorization · 256 × 256 · f64 | 0.9960448117608732× [0.9707239802826404, 1.0095877952453927] | Timing direction unclear |
| Shared-variable backward · sum(X * X), 64 × 32; prepared graph · f64 | — | No paired comparison |
| Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64 | 0.98656× [0.9482543640897756, 1.1184085885696242] | Timing direction unclear |
| Model inference · batched normalized/sliced MLP · f64 | — | No paired comparison |
| Model recording + forward · batched normalized/sliced MLP · f64 | — | No paired comparison |
| Model backward · prepared graph → fresh gradients · f64 | — | No paired comparison |
| Parameter update · transactional SGD, fixed gradients · f64 | — | No paired comparison |
| Coalesced tensor contraction · 16 × 32 × 8 · 32 × 8 × 16 · f64 | — | No paired comparison |
| Row softmax · 256 × 64 · f64 | — | No paired comparison |
| Create zeros · 64 × 64 · f32 | — | No paired comparison |
| Create ones · 64 × 64 · f32 | — | No paired comparison |

## Method and provenance

CPU affinity was not pinned. Frequency and background load are uncontrolled.

Minimum-norm rows use pivoted QR plus scaled implicit RZ in Kernix; NumPy native lstsq; nalgebra native SVD/solve. Algorithm costs and rank policies differ. See original method for thresholds. Kernix/nalgebra: prepared base factor, owned modified factor; refactor: same Kernix version, full factorization of preformed modified matrix; base setup and matrix formation excluded

Numbers retain source precision; recorded digits do not imply equivalent physical measurement accuracy. Lowest observed medians are bold only when peers were measured. No aggregate winner or universal ranking is inferred.

[Original report and commentary](original-report.txt) · [Raw measurements](raw.json)

Measured source `90cbbff60e5d7458ef9f89918c99a6cca94b51d1`. Run `runner-macos-latest-40270611933`. Rendering does not change measurements.
