# Kernix 0.11.1 · performance brief

AMD EPYC 9V45 96-Core Processor / Linux

**Per-call median, microseconds per operation. Lower is faster. All recorded digits retained.**

46 workloads · 149 observations

No previous-version baseline attached. Sequential run changes are descriptive; paired process evidence is shown separately.

[Interactive HTML (open locally)](report.html) · [Vector PDF](latency.pdf) · [Raw measurements](raw.json)

[![Performance brief preview](latency.png)](latency.pdf)

## Charts by workload

**Shorter bars mean less time.** Blue is Kernix; grey is a comparison implementation; teal is an explicitly selected Kernix backend. Entries are sorted by the recorded median, with a separate zero-based scale per chart. These point observations do not establish a statistical ranking.

Expand a group to see every chart here on GitHub. Click a chart to enlarge it. The exact table below keeps every source digit.

<details>
<summary>Matrix multiplication · 13 workloads</summary>

**Matrix multiplication · transposed view · 128 × 128 · 128 × 128 · f64**

![Matrix multiplication · transposed view · 128 × 128 · 128 × 128 · f64. Shorter bars mean less time.](charts/matmul_transposed_128_f64.png)

Kernix: **119.73 µs**. Kernix took ≈ 144.3% more time than NumPy in this run.

**Matrix multiplication · transposed view · 128 × 128 · 128 × 128 · f32**

![Matrix multiplication · transposed view · 128 × 128 · 128 × 128 · f32. Shorter bars mean less time.](charts/matmul_transposed_128_f32.png)

Kernix: **38.367 µs**. Kernix took ≈ 82.5% more time than NumPy in this run.

**Independent matrix operands · 128 × 128 · 128 × 128 · f64**

![Independent matrix operands · 128 × 128 · 128 × 128 · f64. Shorter bars mean less time.](charts/matmul_ab_128_f64.png)

Kernix: **69.274 µs**. Kernix took ≈ 44.0% more time than NumPy in this run.

**Independent matrix operands · 128 × 128 · 128 × 128 · f32**

![Independent matrix operands · 128 × 128 · 128 × 128 · f32. Shorter bars mean less time.](charts/matmul_ab_128_f32.png)

Kernix: **33.961 µs**. Kernix took ≈ 63.4% more time than NumPy in this run.

**Matrix multiplication · 32 × 32 · 32 × 32 · f64**

![Matrix multiplication · 32 × 32 · 32 × 32 · f64. Shorter bars mean less time.](charts/matmul_32_f64.png)

Kernix: **1.112 µs**. Kernix took ≈ 44.0% more time than faer in this run.

**Matrix multiplication · 64 × 64 · 64 × 64 · f64**

![Matrix multiplication · 64 × 64 · 64 × 64 · f64. Shorter bars mean less time.](charts/matmul_64_f64.png)

Kernix: **9.004 µs**. Kernix took ≈ 85.4% more time than faer in this run.

**Matrix multiplication · 128 × 128 · 128 × 128 · f64**

![Matrix multiplication · 128 × 128 · 128 × 128 · f64. Shorter bars mean less time.](charts/matmul_128_f64.png)

Kernix: **68.873 µs**. Kernix took ≈ 79.0% more time than faer in this run.

**Matrix multiplication · 256 × 256 · 256 × 256 · f64**

![Matrix multiplication · 256 × 256 · 256 × 256 · f64. Shorter bars mean less time.](charts/matmul_256_f64.png)

Kernix: **540.022 µs**. Kernix took ≈ 63.3% more time than nalgebra in this run.

**Matrix multiplication · 64 × 64 · 64 × 64 · f32**

![Matrix multiplication · 64 × 64 · 64 × 64 · f32. Shorter bars mean less time.](charts/matmul_64_f32.png)

Kernix: **4.447 µs**. Kernix took ≈ 90.5% more time than faer in this run.

**Matrix multiplication · 128 × 128 · 128 × 128 · f32**

![Matrix multiplication · 128 × 128 · 128 × 128 · f32. Shorter bars mean less time.](charts/matmul_128_f32.png)

Kernix: **34.331 µs**. Kernix took ≈ 71.4% more time than faer in this run.

**Matrix multiplication · 256 × 256 · 256 × 256 · f32**

![Matrix multiplication · 256 × 256 · 256 × 256 · f32. Shorter bars mean less time.](charts/matmul_256_f32.png)

Kernix: **266.7 µs**. Kernix took ≈ 83.8% more time than NumPy in this run.

**Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64**

![Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64. Shorter bars mean less time.](charts/matmul_batched_4x16x32x16_f64.png)

Kernix: **1.352 µs**. Kernix took ≈ 17.2% less time than NumPy in this run.

**Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f32**

![Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f32. Shorter bars mean less time.](charts/matmul_batched_4x16x32x16_f32.png)

Kernix: **0.751 µs**. Kernix took ≈ 43.2% less time than Candle in this run.

</details>

<details>
<summary>Explicit parallel execution · 4 workloads</summary>

**Matmul · 2 workers · reused output · 64 × 64 · 64 × 64 · f64**

![Matmul · 2 workers · reused output · 64 × 64 · 64 × 64 · f64. Shorter bars mean less time.](charts/parallel_matmul_into_64_f64.png)

Kernix: **10.145 µs**. No external comparison with Kernix was measured for this workload.

**Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f64**

![Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f64. Shorter bars mean less time.](charts/parallel_matmul_into_128_f64.png)

Kernix: **42.504 µs**. No external comparison with Kernix was measured for this workload.

**Matmul · 2 workers · reused output · 256 × 256 · 256 × 256 · f64**

![Matmul · 2 workers · reused output · 256 × 256 · 256 × 256 · f64. Shorter bars mean less time.](charts/parallel_matmul_into_256_f64.png)

Kernix: **287.842 µs**. No external comparison with Kernix was measured for this workload.

**Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f32**

![Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f32. Shorter bars mean less time.](charts/parallel_matmul_into_128_f32.png)

Kernix: **24.687 µs**. No external comparison with Kernix was measured for this workload.

</details>

<details>
<summary>Decompositions & solvers · 19 workloads</summary>

**Cholesky solve · reused factor · 128 × 128, 1 RHS · f64**

![Cholesky solve · reused factor · 128 × 128, 1 RHS · f64. Shorter bars mean less time.](charts/solve_cholesky_128x1_f64.png)

Kernix: **9.314 µs**. Kernix took ≈ 158.3% more time than nalgebra in this run.

**Cholesky solve · reused factor · 256 × 256, 64 RHS · f64**

![Cholesky solve · reused factor · 256 × 256, 64 RHS · f64. Shorter bars mean less time.](charts/solve_cholesky_256x64_f64.png)

Kernix: **3845.079 µs**. Kernix took ≈ 3020.1% more time than faer in this run.

**Cholesky solve · reused factor · 512 × 512, 128 RHS · f64**

![Cholesky solve · reused factor · 512 × 512, 128 RHS · f64. Shorter bars mean less time.](charts/solve_cholesky_512x128_f64.png)

Kernix: **42785.584 µs**. Kernix took ≈ 3741.5% more time than faer in this run.

**LU solve · reused factor · 256 × 256, 64 RHS · f64**

![LU solve · reused factor · 256 × 256, 64 RHS · f64. Shorter bars mean less time.](charts/solve_lu_256x64_f64.png)

Kernix: **2725.745 µs**. Kernix took ≈ 1386.5% more time than faer in this run.

**Cholesky rank-one update · 256 × 256 · f64**

![Cholesky rank-one update · 256 × 256 · f64. Shorter bars mean less time.](charts/cholesky_rank_update_256_f64.png)

No external comparison with Kernix was measured for this workload.

**Cholesky rank-one downdate · 256 × 256 · f64**

![Cholesky rank-one downdate · 256 × 256 · f64. Shorter bars mean less time.](charts/cholesky_rank_downdate_256_f64.png)

No external comparison with Kernix was measured for this workload.

**Cholesky rank-one update · 512 × 512 · f64**

![Cholesky rank-one update · 512 × 512 · f64. Shorter bars mean less time.](charts/cholesky_rank_update_512_f64.png)

No external comparison with Kernix was measured for this workload.

**Thin QR · 512 × 128 · f64**

![Thin QR · 512 × 128 · f64. Shorter bars mean less time.](charts/qr_512x128_f64.png)

Kernix: **3512.107 µs**. Kernix took ≈ 190.2% more time than faer in this run.

**Thin QR · 1024 × 256 · f64**

![Thin QR · 1024 × 256 · f64. Shorter bars mean less time.](charts/qr_1024x256_f64.png)

Kernix: **25522.079 µs**. Kernix took ≈ 281.3% more time than faer in this run.

**Cholesky factorization · 512 × 512 · f64**

![Cholesky factorization · 512 × 512 · f64. Shorter bars mean less time.](charts/cholesky_factor_512_f64.png)

Kernix: **2333.015 µs**. Kernix took ≈ 158.8% more time than faer in this run.

**Column-pivoted QR · 96 × 32 · f64**

![Column-pivoted QR · 96 × 32 · f64. Shorter bars mean less time.](charts/pivoted_qr_96x32_f64.png)

Kernix: **46.029 µs**. Kernix took ≈ 88.1% more time than faer in this run.

**Column-pivoted QR · 256 × 64 · f64**

![Column-pivoted QR · 256 × 64 · f64. Shorter bars mean less time.](charts/pivoted_qr_256x64_f64.png)

Kernix: **531.489 µs**. Kernix took ≈ 146.0% more time than faer in this run.

**Minimum-norm least squares · 64 × 32, rank 16, 4 RHS · f64**

![Minimum-norm least squares · 64 × 32, rank 16, 4 RHS · f64. Shorter bars mean less time.](charts/min_norm_64x32_rank16_f64.png)

Kernix: **36.285 µs**. Kernix took ≈ 42.9% less time than NumPy in this run.

**Minimum-norm least squares · 32 × 64, rank 32, 4 RHS · f64**

![Minimum-norm least squares · 32 × 64, rank 32, 4 RHS · f64. Shorter bars mean less time.](charts/min_norm_32x64_rank32_f64.png)

Kernix: **47.852 µs**. Kernix took ≈ 59.6% less time than NumPy in this run.

**Thin QR · 96 × 32 · f64**

![Thin QR · 96 × 32 · f64. Shorter bars mean less time.](charts/qr_96x32_f64.png)

Kernix: **39.81 µs**. Kernix took ≈ 37.4% more time than faer in this run.

**Thin QR · 256 × 64 · f64**

![Thin QR · 256 × 64 · f64. Shorter bars mean less time.](charts/qr_256x64_f64.png)

Kernix: **507.753 µs**. Kernix took ≈ 122.3% more time than faer in this run.

**Cholesky factorization · 64 × 64 · f64**

![Cholesky factorization · 64 × 64 · f64. Shorter bars mean less time.](charts/cholesky_factor_64_f64.png)

Kernix: **16.104 µs**. Kernix took ≈ 126.5% more time than faer in this run.

**Cholesky factorization · 128 × 128 · f64**

![Cholesky factorization · 128 × 128 · f64. Shorter bars mean less time.](charts/cholesky_factor_128_f64.png)

Kernix: **97.277 µs**. Kernix took ≈ 250.0% more time than faer in this run.

**Cholesky factorization · 256 × 256 · f64**

![Cholesky factorization · 256 × 256 · f64. Shorter bars mean less time.](charts/cholesky_factor_256_f64.png)

Kernix: **433.551 µs**. Kernix took ≈ 203.8% more time than faer in this run.

</details>

<details>
<summary>Training & autodiff · 6 workloads</summary>

**Shared-variable backward · sum(X * X), 64 × 32; prepared graph · f64**

![Shared-variable backward · sum(X * X), 64 × 32; prepared graph · f64. Shorter bars mean less time.](charts/training_shared_backward_64x32_f64.png)

Kernix: **4.977 µs**. Kernix took ≈ 60.3% more time than Candle in this run.

**Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64**

![Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64. Shorter bars mean less time.](charts/matmul_sum_backward_32x16x8_f64.png)

Kernix: **2.053 µs**. Kernix took ≈ 46.3% less time than Candle in this run.

**Model inference · batched normalized/sliced MLP · f64**

![Model inference · batched normalized/sliced MLP · f64. Shorter bars mean less time.](charts/training_forward_2x4x8x4_f64.png)

Kernix: **1.893 µs**. Kernix took ≈ 35.9% less time than Candle in this run.

**Model recording + forward · batched normalized/sliced MLP · f64**

![Model recording + forward · batched normalized/sliced MLP · f64. Shorter bars mean less time.](charts/training_record_2x4x8x4_f64.png)

Kernix: **2.423 µs**. Kernix took ≈ 42.7% less time than Candle in this run.

**Model backward · prepared graph → fresh gradients · f64**

![Model backward · prepared graph → fresh gradients · f64. Shorter bars mean less time.](charts/training_backward_2x4x8x4_f64.png)

Kernix: **3.034 µs**. Kernix took ≈ 76.2% less time than Candle in this run.

**Parameter update · transactional SGD, fixed gradients · f64**

![Parameter update · transactional SGD, fixed gradients · f64. Shorter bars mean less time.](charts/training_sgd_2x4x8x4_f64.png)

Kernix: **0.531 µs**. No external comparison with Kernix was measured for this workload.

</details>

<details>
<summary>Tensor contractions · 1 workloads</summary>

**Coalesced tensor contraction · 16 × 32 × 8 · 32 × 8 × 16 · f64**

![Coalesced tensor contraction · 16 × 32 × 8 · 32 × 8 × 16 · f64. Shorter bars mean less time.](charts/tensordot_16x32x8_32x8x16_f64.png)

Kernix: **31.628000000000004 µs**. Kernix took ≈ 537.9% more time than NumPy in this run.

</details>

<details>
<summary>Statistics & activations · 1 workloads</summary>

**Row softmax · 256 × 64 · f64**

![Row softmax · 256 × 64 · f64. Shorter bars mean less time.](charts/softmax_256x64_f64.png)

Kernix: **69.214 µs**. Kernix took ≈ 61.2% more time than ndarray in this run.

</details>

<details>
<summary>Tensor construction · 2 workloads</summary>

**Create zeros · 64 × 64 · f32**

![Create zeros · 64 × 64 · f32. Shorter bars mean less time.](charts/create_zeros_64x64_f32.png)

Kernix: **0.151 µs**. Kernix took ≈ 67.8% more time than ndarray in this run.

**Create ones · 64 × 64 · f32**

![Create ones · 64 × 64 · f32. Shorter bars mean less time.](charts/create_ones_64x64_f32.png)

Kernix: **0.23 µs**. Kernix took ≈ 53.3% more time than ndarray in this run.

</details>

## Exact latency table

All values below are microseconds per operation. **Lower is faster.** Missing entries were not measured.

| Workload | Kernix | ndarray | nalgebra | faer | Candle | NumPy | Kernix refactor |
|---|---:|---:|---:|---:|---:|---:|---:|
| Matrix multiplication · transposed view · 128 × 128 · 128 × 128 · f64 | 119.73 | — | — | — | — | **49.013** | — |
| Matrix multiplication · transposed view · 128 × 128 · 128 × 128 · f32 | 38.367 | — | — | — | — | **21.021** | — |
| Independent matrix operands · 128 × 128 · 128 × 128 · f64 | 69.274 | — | — | — | — | **48.102** | — |
| Independent matrix operands · 128 × 128 · 128 × 128 · f32 | 33.961 | — | — | — | — | **20.781** | — |
| Matrix multiplication · 32 × 32 · 32 × 32 · f64 | 1.112 | 0.9820000000000001 | 0.9909999999999999 | **0.772** | 1.392 | 1.412 | — |
| Matrix multiplication · 64 × 64 · 64 × 64 · f64 | 9.004 | 6.35 | 6.37 | **4.857** | 9.044 | 4.978 | — |
| Matrix multiplication · 128 × 128 · 128 × 128 · f64 | 68.873 | 121.813 | 120.28 | **38.478** | 68.863 | 48.262 | — |
| Matrix multiplication · 256 × 256 · 256 × 256 · f64 | 540.022 | 346.461 | **330.727** | 338.719 | 544.648 | 358.018 | — |
| Matrix multiplication · 64 × 64 · 64 × 64 · f32 | 4.447 | 3.435 | 3.405 | **2.334** | 4.537 | 2.975 | — |
| Matrix multiplication · 128 × 128 · 128 × 128 · f32 | 34.331 | 39.76 | 40.24 | **20.03** | 34.232 | 21.082 | — |
| Matrix multiplication · 256 × 256 · 256 × 256 · f32 | 266.7 | 325.86 | 329.905 | 150.466 | 263.946 | **145.118** | — |
| Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64 | **1.352** | — | — | — | 1.922 | 1.632 | — |
| Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f32 | **0.751** | — | — | — | 1.322 | 1.462 | — |
| Matmul · 2 workers · reused output · 64 × 64 · 64 × 64 · f64 | 10.145 | — | — | — | — | — | — |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f64 | 42.504 | — | — | — | — | — | — |
| Matmul · 2 workers · reused output · 256 × 256 · 256 × 256 · f64 | 287.842 | — | — | — | — | — | — |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f32 | 24.687 | — | — | — | — | — | — |
| Cholesky solve · reused factor · 128 × 128, 1 RHS · f64 | 9.314 | — | **3.606** | 3.676 | — | — | — |
| Cholesky solve · reused factor · 256 × 256, 64 RHS · f64 | 3845.079 | — | 732.881 | **123.235** | — | — | — |
| Cholesky solve · reused factor · 512 × 512, 128 RHS · f64 | 42785.584 | — | 5924.202 | **1113.783** | — | — | — |
| LU solve · reused factor · 256 × 256, 64 RHS · f64 | 2725.745 | — | 681.494 | **183.366** | — | — | — |
| Cholesky rank-one update · 256 × 256 · f64 | — | — | **17.927** | — | — | — | 429.575 |
| Cholesky rank-one downdate · 256 × 256 · f64 | — | — | **17.726** | — | — | — | 436.506 |
| Cholesky rank-one update · 512 × 512 · f64 | — | — | **95.133** | — | — | — | 2325.754 |
| Thin QR · 512 × 128 · f64 | 3512.107 | — | 2920.398 | **1210.388** | — | 2099.615 | — |
| Thin QR · 1024 × 256 · f64 | 25522.079 | — | 22188.67 | **6692.596** | — | 11212.453 | — |
| Cholesky factorization · 512 × 512 · f64 | 2333.015 | — | 4188.855 | **901.335** | — | 4389.956 | — |
| Column-pivoted QR · 96 × 32 · f64 | 46.029 | — | 59.37 | **24.476** | — | — | — |
| Column-pivoted QR · 256 × 64 · f64 | 531.489 | — | 611.399 | **216.094** | — | — | — |
| Minimum-norm least squares · 64 × 32, rank 16, 4 RHS · f64 | **36.285** | — | 69.334 | — | — | 63.515 | — |
| Minimum-norm least squares · 32 × 64, rank 32, 4 RHS · f64 | **47.852** | — | 123.435 | — | — | 118.498 | — |
| Thin QR · 96 × 32 · f64 | 39.81 | — | 34.672 | **28.984** | — | 35.564 | — |
| Thin QR · 256 × 64 · f64 | 507.753 | — | 356.125 | **228.423** | — | 315.955 | — |
| Cholesky factorization · 64 × 64 · f64 | 16.104 | — | 15.133 | **7.111** | — | 10.757 | — |
| Cholesky factorization · 128 × 128 · f64 | 97.277 | — | 85.699 | **27.792** | — | 88.803 | — |
| Cholesky factorization · 256 × 256 · f64 | 433.551 | — | 557.468 | **142.725** | — | 821.845 | — |
| Shared-variable backward · sum(X * X), 64 × 32; prepared graph · f64 | 4.977 | — | — | — | **3.105** | — | — |
| Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64 | **2.053** | — | — | — | 3.8260000000000005 | — | — |
| Model inference · batched normalized/sliced MLP · f64 | **1.893** | — | — | — | 2.955 | — | — |
| Model recording + forward · batched normalized/sliced MLP · f64 | **2.423** | — | — | — | 4.226 | — | — |
| Model backward · prepared graph → fresh gradients · f64 | **3.034** | — | — | — | 12.739 | — | — |
| Parameter update · transactional SGD, fixed gradients · f64 | 0.531 | — | — | — | — | — | — |
| Coalesced tensor contraction · 16 × 32 × 8 · 32 × 8 × 16 · f64 | 31.628000000000004 | — | — | — | — | **4.958** | — |
| Row softmax · 256 × 64 · f64 | 69.214 | **42.944** | — | — | 44.216 | 44.307 | — |
| Create zeros · 64 × 64 · f32 | 0.151 | **0.09** | — | 0.14 | 0.16 | 0.34 | — |
| Create ones · 64 × 64 · f32 | 0.23 | **0.15** | — | 0.18 | 0.25 | 1.242 | — |

## Version comparison

0 of 46 workloads have paired evidence. Exploratory 95% intervals resample process pairs, not calls. No multiple-comparison adjustment.

| Workload | Current / baseline [95% interval] | Interpretation |
|---|---:|---|
| Matrix multiplication · transposed view · 128 × 128 · 128 × 128 · f64 | — | No paired comparison |
| Matrix multiplication · transposed view · 128 × 128 · 128 × 128 · f32 | — | No paired comparison |
| Independent matrix operands · 128 × 128 · 128 × 128 · f64 | — | No paired comparison |
| Independent matrix operands · 128 × 128 · 128 × 128 · f32 | — | No paired comparison |
| Matrix multiplication · 32 × 32 · 32 × 32 · f64 | — | No paired comparison |
| Matrix multiplication · 64 × 64 · 64 × 64 · f64 | — | No paired comparison |
| Matrix multiplication · 128 × 128 · 128 × 128 · f64 | — | No paired comparison |
| Matrix multiplication · 256 × 256 · 256 × 256 · f64 | — | No paired comparison |
| Matrix multiplication · 64 × 64 · 64 × 64 · f32 | — | No paired comparison |
| Matrix multiplication · 128 × 128 · 128 × 128 · f32 | — | No paired comparison |
| Matrix multiplication · 256 × 256 · 256 × 256 · f32 | — | No paired comparison |
| Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64 | — | No paired comparison |
| Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f32 | — | No paired comparison |
| Matmul · 2 workers · reused output · 64 × 64 · 64 × 64 · f64 | — | No paired comparison |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f64 | — | No paired comparison |
| Matmul · 2 workers · reused output · 256 × 256 · 256 × 256 · f64 | — | No paired comparison |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f32 | — | No paired comparison |
| Cholesky solve · reused factor · 128 × 128, 1 RHS · f64 | — | No paired comparison |
| Cholesky solve · reused factor · 256 × 256, 64 RHS · f64 | — | No paired comparison |
| Cholesky solve · reused factor · 512 × 512, 128 RHS · f64 | — | No paired comparison |
| LU solve · reused factor · 256 × 256, 64 RHS · f64 | — | No paired comparison |
| Cholesky rank-one update · 256 × 256 · f64 | — | No paired comparison |
| Cholesky rank-one downdate · 256 × 256 · f64 | — | No paired comparison |
| Cholesky rank-one update · 512 × 512 · f64 | — | No paired comparison |
| Thin QR · 512 × 128 · f64 | — | No paired comparison |
| Thin QR · 1024 × 256 · f64 | — | No paired comparison |
| Cholesky factorization · 512 × 512 · f64 | — | No paired comparison |
| Column-pivoted QR · 96 × 32 · f64 | — | No paired comparison |
| Column-pivoted QR · 256 × 64 · f64 | — | No paired comparison |
| Minimum-norm least squares · 64 × 32, rank 16, 4 RHS · f64 | — | No paired comparison |
| Minimum-norm least squares · 32 × 64, rank 32, 4 RHS · f64 | — | No paired comparison |
| Thin QR · 96 × 32 · f64 | — | No paired comparison |
| Thin QR · 256 × 64 · f64 | — | No paired comparison |
| Cholesky factorization · 64 × 64 · f64 | — | No paired comparison |
| Cholesky factorization · 128 × 128 · f64 | — | No paired comparison |
| Cholesky factorization · 256 × 256 · f64 | — | No paired comparison |
| Shared-variable backward · sum(X * X), 64 × 32; prepared graph · f64 | — | No paired comparison |
| Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64 | — | No paired comparison |
| Model inference · batched normalized/sliced MLP · f64 | — | No paired comparison |
| Model recording + forward · batched normalized/sliced MLP · f64 | — | No paired comparison |
| Model backward · prepared graph → fresh gradients · f64 | — | No paired comparison |
| Parameter update · transactional SGD, fixed gradients · f64 | — | No paired comparison |
| Coalesced tensor contraction · 16 × 32 × 8 · 32 × 8 × 16 · f64 | — | No paired comparison |
| Row softmax · 256 × 64 · f64 | — | No paired comparison |
| Create zeros · 64 × 64 · f32 | — | No paired comparison |
| Create ones · 64 × 64 · f32 | — | No paired comparison |

## Method and provenance

CPU affinity pinned to logical CPU 0. Frequency and background load are uncontrolled.

Minimum-norm rows use pivoted QR plus scaled implicit RZ in Kernix; NumPy native lstsq; nalgebra native SVD/solve. Algorithm costs and rank policies differ. See original method for thresholds. Kernix/nalgebra: prepared base factor, owned modified factor; refactor: same Kernix version, full factorization of preformed modified matrix; base setup and matrix formation excluded This registry baseline has no rank modification API; Kernix modification entries are omitted, not replaced.

Numbers retain source precision; recorded digits do not imply equivalent physical measurement accuracy. Lowest observed medians are bold only when peers were measured. No aggregate winner or universal ranking is inferred.

[Original report and commentary](original-report.txt) · [Raw measurements](raw.json)

Measured source `90cbbff60e5d7458ef9f89918c99a6cca94b51d1`. Run `baseline-ubuntu-latest-40270611933`. Rendering does not change measurements.
