# Kernix 0.11.1 · performance brief

Neoverse-N2 / Linux

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

Kernix: **215.162 µs**. Kernix took ≈ 8.9% more time than NumPy in this run.

**Matrix multiplication · transposed view · 128 × 128 · 128 × 128 · f32**

![Matrix multiplication · transposed view · 128 × 128 · 128 × 128 · f32. Shorter bars mean less time.](charts/matmul_transposed_128_f32.png)

Kernix: **92.37 µs**. Kernix took ≈ 7.9% less time than NumPy in this run.

**Independent matrix operands · 128 × 128 · 128 × 128 · f64**

![Independent matrix operands · 128 × 128 · 128 × 128 · f64. Shorter bars mean less time.](charts/matmul_ab_128_f64.png)

Kernix: **168.973 µs**. Kernix took ≈ 13.6% less time than NumPy in this run.

**Independent matrix operands · 128 × 128 · 128 × 128 · f32**

![Independent matrix operands · 128 × 128 · 128 × 128 · f32. Shorter bars mean less time.](charts/matmul_ab_128_f32.png)

Kernix: **85.747 µs**. Kernix took ≈ 14.2% less time than NumPy in this run.

**Matrix multiplication · 32 × 32 · 32 × 32 · f64**

![Matrix multiplication · 32 × 32 · 32 × 32 · f64. Shorter bars mean less time.](charts/matmul_32_f64.png)

Kernix: **3.232 µs**. Kernix took ≈ 6.0% more time than ndarray in this run.

**Matrix multiplication · 64 × 64 · 64 × 64 · f64**

![Matrix multiplication · 64 × 64 · 64 × 64 · f64. Shorter bars mean less time.](charts/matmul_64_f64.png)

Kernix: **22.239 µs**. Kernix took ≈ 2.1% more time than ndarray in this run.

**Matrix multiplication · 128 × 128 · 128 × 128 · f64**

![Matrix multiplication · 128 × 128 · 128 × 128 · f64. Shorter bars mean less time.](charts/matmul_128_f64.png)

Kernix: **169.069 µs**. Kernix took ≈ 0.1% more time than Candle in this run.

**Matrix multiplication · 256 × 256 · 256 × 256 · f64**

![Matrix multiplication · 256 × 256 · 256 × 256 · f64. Shorter bars mean less time.](charts/matmul_256_f64.png)

Kernix: **1321.711 µs**. Kernix took &lt; 0.1% less time than ndarray in this run.

**Matrix multiplication · 64 × 64 · 64 × 64 · f32**

![Matrix multiplication · 64 × 64 · 64 × 64 · f32. Shorter bars mean less time.](charts/matmul_64_f32.png)

Kernix: **11.767 µs**. Kernix took ≈ 4.5% more time than ndarray in this run.

**Matrix multiplication · 128 × 128 · 128 × 128 · f32**

![Matrix multiplication · 128 × 128 · 128 × 128 · f32. Shorter bars mean less time.](charts/matmul_128_f32.png)

Kernix: **85.658 µs**. Kernix took ≈ 0.3% less time than Candle in this run.

**Matrix multiplication · 256 × 256 · 256 × 256 · f32**

![Matrix multiplication · 256 × 256 · 256 × 256 · f32. Shorter bars mean less time.](charts/matmul_256_f32.png)

Kernix: **661.548 µs**. Kernix took ≈ 2.0% less time than faer in this run.

**Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64**

![Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64. Shorter bars mean less time.](charts/matmul_batched_4x16x32x16_f64.png)

Kernix: **3.832 µs**. Kernix took ≈ 14.2% less time than Candle in this run.

**Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f32**

![Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f32. Shorter bars mean less time.](charts/matmul_batched_4x16x32x16_f32.png)

Kernix: **2.4 µs**. Kernix took ≈ 16.4% less time than Candle in this run.

</details>

<details>
<summary>Explicit parallel execution · 4 workloads</summary>

**Matmul · 2 workers · reused output · 64 × 64 · 64 × 64 · f64**

![Matmul · 2 workers · reused output · 64 × 64 · 64 × 64 · f64. Shorter bars mean less time.](charts/parallel_matmul_into_64_f64.png)

Kernix: **16.847 µs**. No external comparison with Kernix was measured for this workload.

**Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f64**

![Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f64. Shorter bars mean less time.](charts/parallel_matmul_into_128_f64.png)

Kernix: **93.034 µs**. No external comparison with Kernix was measured for this workload.

**Matmul · 2 workers · reused output · 256 × 256 · 256 × 256 · f64**

![Matmul · 2 workers · reused output · 256 × 256 · 256 × 256 · f64. Shorter bars mean less time.](charts/parallel_matmul_into_256_f64.png)

Kernix: **696.881 µs**. No external comparison with Kernix was measured for this workload.

**Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f32**

![Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f32. Shorter bars mean less time.](charts/parallel_matmul_into_128_f32.png)

Kernix: **49.061 µs**. No external comparison with Kernix was measured for this workload.

</details>

<details>
<summary>Decompositions & solvers · 19 workloads</summary>

**Cholesky solve · reused factor · 128 × 128, 1 RHS · f64**

![Cholesky solve · reused factor · 128 × 128, 1 RHS · f64. Shorter bars mean less time.](charts/solve_cholesky_128x1_f64.png)

Kernix: **12.624 µs**. Kernix took ≈ 140.6% more time than nalgebra in this run.

**Cholesky solve · reused factor · 256 × 256, 64 RHS · f64**

![Cholesky solve · reused factor · 256 × 256, 64 RHS · f64. Shorter bars mean less time.](charts/solve_cholesky_256x64_f64.png)

Kernix: **5108.168 µs**. Kernix took ≈ 1037.7% more time than faer in this run.

**Cholesky solve · reused factor · 512 × 512, 128 RHS · f64**

![Cholesky solve · reused factor · 512 × 512, 128 RHS · f64. Shorter bars mean less time.](charts/solve_cholesky_512x128_f64.png)

Kernix: **46750.296 µs**. Kernix took ≈ 1364.7% more time than faer in this run.

**LU solve · reused factor · 256 × 256, 64 RHS · f64**

![LU solve · reused factor · 256 × 256, 64 RHS · f64. Shorter bars mean less time.](charts/solve_lu_256x64_f64.png)

Kernix: **3429.105 µs**. Kernix took ≈ 651.6% more time than faer in this run.

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

Kernix: **4740.465 µs**. Kernix took ≈ 52.9% more time than faer in this run.

**Thin QR · 1024 × 256 · f64**

![Thin QR · 1024 × 256 · f64. Shorter bars mean less time.](charts/qr_1024x256_f64.png)

Kernix: **35699.145 µs**. Kernix took ≈ 81.5% more time than NumPy in this run.

**Cholesky factorization · 512 × 512 · f64**

![Cholesky factorization · 512 × 512 · f64. Shorter bars mean less time.](charts/cholesky_factor_512_f64.png)

Kernix: **4352.098 µs**. Kernix took ≈ 65.1% more time than faer in this run.

**Column-pivoted QR · 96 × 32 · f64**

![Column-pivoted QR · 96 × 32 · f64. Shorter bars mean less time.](charts/pivoted_qr_96x32_f64.png)

Kernix: **88.682 µs**. Kernix took ≈ 42.7% more time than faer in this run.

**Column-pivoted QR · 256 × 64 · f64**

![Column-pivoted QR · 256 × 64 · f64. Shorter bars mean less time.](charts/pivoted_qr_256x64_f64.png)

Kernix: **785.043 µs**. Kernix took ≈ 65.5% more time than faer in this run.

**Minimum-norm least squares · 64 × 32, rank 16, 4 RHS · f64**

![Minimum-norm least squares · 64 × 32, rank 16, 4 RHS · f64. Shorter bars mean less time.](charts/min_norm_64x32_rank16_f64.png)

Kernix: **72.667 µs**. Kernix took ≈ 22.5% less time than NumPy in this run.

**Minimum-norm least squares · 32 × 64, rank 32, 4 RHS · f64**

![Minimum-norm least squares · 32 × 64, rank 32, 4 RHS · f64. Shorter bars mean less time.](charts/min_norm_32x64_rank32_f64.png)

Kernix: **96.513 µs**. Kernix took ≈ 45.3% less time than NumPy in this run.

**Thin QR · 96 × 32 · f64**

![Thin QR · 96 × 32 · f64. Shorter bars mean less time.](charts/qr_96x32_f64.png)

Kernix: **71.763 µs**. Kernix took ≈ 17.0% more time than nalgebra in this run.

**Thin QR · 256 × 64 · f64**

![Thin QR · 256 × 64 · f64. Shorter bars mean less time.](charts/qr_256x64_f64.png)

Kernix: **695.61 µs**. Kernix took ≈ 56.0% more time than faer in this run.

**Cholesky factorization · 64 × 64 · f64**

![Cholesky factorization · 64 × 64 · f64. Shorter bars mean less time.](charts/cholesky_factor_64_f64.png)

Kernix: **27.326 µs**. Kernix took ≈ 106.3% more time than faer in this run.

**Cholesky factorization · 128 × 128 · f64**

![Cholesky factorization · 128 × 128 · f64. Shorter bars mean less time.](charts/cholesky_factor_128_f64.png)

Kernix: **277.221 µs**. Kernix took ≈ 325.6% more time than faer in this run.

**Cholesky factorization · 256 × 256 · f64**

![Cholesky factorization · 256 × 256 · f64. Shorter bars mean less time.](charts/cholesky_factor_256_f64.png)

Kernix: **767.732 µs**. Kernix took ≈ 90.6% more time than faer in this run.

</details>

<details>
<summary>Training & autodiff · 6 workloads</summary>

**Shared-variable backward · sum(X * X), 64 × 32; prepared graph · f64**

![Shared-variable backward · sum(X * X), 64 × 32; prepared graph · f64. Shorter bars mean less time.](charts/training_shared_backward_64x32_f64.png)

Kernix: **8.367 µs**. Kernix took ≈ 68.4% more time than Candle in this run.

**Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64**

![Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64. Shorter bars mean less time.](charts/matmul_sum_backward_32x16x8_f64.png)

Kernix: **3.512 µs**. Kernix took ≈ 52.2% less time than Candle in this run.

**Model inference · batched normalized/sliced MLP · f64**

![Model inference · batched normalized/sliced MLP · f64. Shorter bars mean less time.](charts/training_forward_2x4x8x4_f64.png)

Kernix: **3.032 µs**. Kernix took ≈ 42.7% less time than Candle in this run.

**Model recording + forward · batched normalized/sliced MLP · f64**

![Model recording + forward · batched normalized/sliced MLP · f64. Shorter bars mean less time.](charts/training_record_2x4x8x4_f64.png)

Kernix: **3.8469999999999995 µs**. Kernix took ≈ 49.4% less time than Candle in this run.

**Model backward · prepared graph → fresh gradients · f64**

![Model backward · prepared graph → fresh gradients · f64. Shorter bars mean less time.](charts/training_backward_2x4x8x4_f64.png)

Kernix: **4.207 µs**. Kernix took ≈ 80.0% less time than Candle in this run.

**Parameter update · transactional SGD, fixed gradients · f64**

![Parameter update · transactional SGD, fixed gradients · f64. Shorter bars mean less time.](charts/training_sgd_2x4x8x4_f64.png)

Kernix: **0.736 µs**. No external comparison with Kernix was measured for this workload.

</details>

<details>
<summary>Tensor contractions · 1 workloads</summary>

**Coalesced tensor contraction · 16 × 32 × 8 · 32 × 8 × 16 · f64**

![Coalesced tensor contraction · 16 × 32 × 8 · 32 × 8 × 16 · f64. Shorter bars mean less time.](charts/tensordot_16x32x8_32x8x16_f64.png)

Kernix: **46.677 µs**. Kernix took ≈ 180.8% more time than NumPy in this run.

</details>

<details>
<summary>Statistics & activations · 1 workloads</summary>

**Row softmax · 256 × 64 · f64**

![Row softmax · 256 × 64 · f64. Shorter bars mean less time.](charts/softmax_256x64_f64.png)

Kernix: **113.144 µs**. Kernix took ≈ 16.2% more time than ndarray in this run.

</details>

<details>
<summary>Tensor construction · 2 workloads</summary>

**Create zeros · 64 × 64 · f32**

![Create zeros · 64 × 64 · f32. Shorter bars mean less time.](charts/create_zeros_64x64_f32.png)

Kernix: **0.2 µs**. Kernix took ≈ 47.1% more time than ndarray in this run.

**Create ones · 64 × 64 · f32**

![Create ones · 64 × 64 · f32. Shorter bars mean less time.](charts/create_ones_64x64_f32.png)

Kernix: **0.248 µs**. Kernix took ≈ 19.2% more time than ndarray in this run.

</details>

## Exact latency table

All values below are microseconds per operation. **Lower is faster.** Missing entries were not measured.

| Workload | Kernix | ndarray | nalgebra | faer | Candle | NumPy | Kernix refactor |
|---|---:|---:|---:|---:|---:|---:|---:|
| Matrix multiplication · transposed view · 128 × 128 · 128 × 128 · f64 | 215.162 | — | — | — | — | **197.507** | — |
| Matrix multiplication · transposed view · 128 × 128 · 128 × 128 · f32 | **92.37** | — | — | — | — | 100.249 | — |
| Independent matrix operands · 128 × 128 · 128 × 128 · f64 | **168.973** | — | — | — | — | 195.667 | — |
| Independent matrix operands · 128 × 128 · 128 × 128 · f32 | **85.747** | — | — | — | — | 99.978 | — |
| Matrix multiplication · 32 × 32 · 32 × 32 · f64 | 3.232 | **3.048** | 3.216 | 3.512 | 3.448 | 8.2 | — |
| Matrix multiplication · 64 × 64 · 64 × 64 · f64 | 22.239 | **21.782** | 22.455 | 22.654 | 22.502 | 31.054 | — |
| Matrix multiplication · 128 × 128 · 128 × 128 · f64 | 169.069 | 207.458 | 210.138 | 169.26 | **168.829** | 195.17 | — |
| Matrix multiplication · 256 × 256 · 256 × 256 · f64 | **1321.711** | 1322.775 | 1328.19 | 1532.384 | 1322.999 | 1462.838 | — |
| Matrix multiplication · 64 × 64 · 64 × 64 · f32 | 11.767 | **11.256** | 11.28 | 12.319 | 11.975 | 20.111 | — |
| Matrix multiplication · 128 × 128 · 128 × 128 · f32 | **85.658** | 93.634 | 93.434 | 87.13 | 85.922 | 100.466 | — |
| Matrix multiplication · 256 × 256 · 256 × 256 · f32 | **661.548** | 760.421 | 759.293 | 675.331 | 749.902 | 722.743 | — |
| Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64 | **3.832** | — | — | — | 4.464 | 12.96 | — |
| Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f32 | **2.4** | — | — | — | 2.872 | 5.16 | — |
| Matmul · 2 workers · reused output · 64 × 64 · 64 × 64 · f64 | 16.847 | — | — | — | — | — | — |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f64 | 93.034 | — | — | — | — | — | — |
| Matmul · 2 workers · reused output · 256 × 256 · 256 × 256 · f64 | 696.881 | — | — | — | — | — | — |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f32 | 49.061 | — | — | — | — | — | — |
| Cholesky solve · reused factor · 128 × 128, 1 RHS · f64 | 12.624 | — | **5.247** | 5.919 | — | — | — |
| Cholesky solve · reused factor · 256 × 256, 64 RHS · f64 | 5108.168 | — | 1275.986 | **449.002** | — | — | — |
| Cholesky solve · reused factor · 512 × 512, 128 RHS · f64 | 46750.296 | — | 12663.051 | **3191.721** | — | — | — |
| LU solve · reused factor · 256 × 256, 64 RHS · f64 | 3429.105 | — | 1241.292 | **456.217** | — | — | — |
| Cholesky rank-one update · 256 × 256 · f64 | — | — | **33.485** | — | — | — | 762.884 |
| Cholesky rank-one downdate · 256 × 256 · f64 | — | — | **34.054** | — | — | — | 765.573 |
| Cholesky rank-one update · 512 × 512 · f64 | — | — | **146.262** | — | — | — | 4357.818 |
| Thin QR · 512 × 128 · f64 | 4740.465 | — | 4626.744 | **3100.728** | — | 4128.25 | — |
| Thin QR · 1024 × 256 · f64 | 35699.145 | — | 35936.844 | 20266.282 | — | **19667.011** | — |
| Cholesky factorization · 512 × 512 · f64 | 4352.098 | — | 8613.371 | **2635.678** | — | 3859.372 | — |
| Column-pivoted QR · 96 × 32 · f64 | 88.682 | — | 114.336 | **62.164** | — | — | — |
| Column-pivoted QR · 256 × 64 · f64 | 785.043 | — | 1150.73 | **474.328** | — | — | — |
| Minimum-norm least squares · 64 × 32, rank 16, 4 RHS · f64 | **72.667** | — | 117.168 | — | — | 93.754 | — |
| Minimum-norm least squares · 32 × 64, rank 32, 4 RHS · f64 | **96.513** | — | 218.585 | — | — | 176.3 | — |
| Thin QR · 96 × 32 · f64 | 71.763 | — | **61.332** | 65.275 | — | 66.099 | — |
| Thin QR · 256 × 64 · f64 | 695.61 | — | 585.457 | **445.986** | — | 562.474 | — |
| Cholesky factorization · 64 × 64 · f64 | 27.326 | — | 20.343 | **13.248** | — | 18.006 | — |
| Cholesky factorization · 128 × 128 · f64 | 277.221 | — | 128.87900000000001 | **65.132** | — | 83.691 | — |
| Cholesky factorization · 256 × 256 · f64 | 767.732 | — | 1030.499 | **402.901** | — | 749.182 | — |
| Shared-variable backward · sum(X * X), 64 × 32; prepared graph · f64 | 8.367 | — | — | — | **4.968** | — | — |
| Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64 | **3.512** | — | — | — | 7.343 | — | — |
| Model inference · batched normalized/sliced MLP · f64 | **3.032** | — | — | — | 5.287 | — | — |
| Model recording + forward · batched normalized/sliced MLP · f64 | **3.8469999999999995** | — | — | — | 7.608 | — | — |
| Model backward · prepared graph → fresh gradients · f64 | **4.207** | — | — | — | 21.086 | — | — |
| Parameter update · transactional SGD, fixed gradients · f64 | 0.736 | — | — | — | — | — | — |
| Coalesced tensor contraction · 16 × 32 × 8 · 32 × 8 × 16 · f64 | 46.677 | — | — | — | — | **16.623** | — |
| Row softmax · 256 × 64 · f64 | 113.144 | **97.385** | — | — | 110.345 | 134.623 | — |
| Create zeros · 64 × 64 · f32 | 0.2 | **0.136** | — | 0.32 | 0.23999999999999997 | 0.504 | — |
| Create ones · 64 × 64 · f32 | 0.248 | **0.208** | — | 0.23999999999999997 | 0.344 | 1.52 | — |

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

Measured source `90cbbff60e5d7458ef9f89918c99a6cca94b51d1`. Run `baseline-ubuntu-24.04-arm-40270611933`. Rendering does not change measurements.
