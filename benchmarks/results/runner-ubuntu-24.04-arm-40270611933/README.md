# Kernix 0.12.0 · performance brief

Neoverse-N2 / Linux

**Per-call median, microseconds per operation. Lower is faster. All recorded digits retained.**

46 workloads · 152 observations

Baseline: baseline-ubuntu-24.04-arm-40270611933. Sequential run changes are descriptive; paired process evidence is shown separately.

[Interactive HTML (open locally)](report.html) · [Vector PDF](latency.pdf) · [Raw measurements](raw.json)

[![Performance brief preview](latency.png)](latency.pdf)

## Charts by workload

**Shorter bars mean less time.** Blue is Kernix; grey is a comparison implementation; teal is an explicitly selected Kernix backend. Entries are sorted by the recorded median, with a separate zero-based scale per chart. These point observations do not establish a statistical ranking.

Expand a group to see every chart here on GitHub. Click a chart to enlarge it. The exact table below keeps every source digit.

<details>
<summary>Matrix multiplication · 13 workloads</summary>

**Matrix multiplication · transposed view · 128 × 128 · 128 × 128 · f64**

![Matrix multiplication · transposed view · 128 × 128 · 128 × 128 · f64. Shorter bars mean less time.](charts/matmul_transposed_128_f64.png)

Kernix: **212.186 µs**. Kernix took ≈ 7.7% more time than NumPy in this run.

**Matrix multiplication · transposed view · 128 × 128 · 128 × 128 · f32**

![Matrix multiplication · transposed view · 128 × 128 · 128 × 128 · f32. Shorter bars mean less time.](charts/matmul_transposed_128_f32.png)

Kernix: **92.713 µs**. Kernix took ≈ 7.6% less time than NumPy in this run.

**Independent matrix operands · 128 × 128 · 128 × 128 · f64**

![Independent matrix operands · 128 × 128 · 128 × 128 · f64. Shorter bars mean less time.](charts/matmul_ab_128_f64.png)

Kernix: **168.484 µs**. Kernix took ≈ 13.8% less time than NumPy in this run.

**Independent matrix operands · 128 × 128 · 128 × 128 · f32**

![Independent matrix operands · 128 × 128 · 128 × 128 · f32. Shorter bars mean less time.](charts/matmul_ab_128_f32.png)

Kernix: **86.258 µs**. Kernix took ≈ 13.6% less time than NumPy in this run.

**Matrix multiplication · 32 × 32 · 32 × 32 · f64**

![Matrix multiplication · 32 × 32 · 32 × 32 · f64. Shorter bars mean less time.](charts/matmul_32_f64.png)

Kernix: **3.248 µs**. Kernix took ≈ 6.6% more time than ndarray in this run.

**Matrix multiplication · 64 × 64 · 64 × 64 · f64**

![Matrix multiplication · 64 × 64 · 64 × 64 · f64. Shorter bars mean less time.](charts/matmul_64_f64.png)

Kernix: **22.167 µs**. Kernix took ≈ 1.6% more time than ndarray in this run.

**Matrix multiplication · 128 × 128 · 128 × 128 · f64**

![Matrix multiplication · 128 × 128 · 128 × 128 · f64. Shorter bars mean less time.](charts/matmul_128_f64.png)

Kernix: **168.285 µs**. Kernix took ≈ 0.3% less time than Candle in this run.

**Matrix multiplication · 256 × 256 · 256 × 256 · f64**

![Matrix multiplication · 256 × 256 · 256 × 256 · f64. Shorter bars mean less time.](charts/matmul_256_f64.png)

Kernix: **1320.48 µs**. Kernix took &lt; 0.1% less time than Candle in this run.

**Matrix multiplication · 64 × 64 · 64 × 64 · f32**

![Matrix multiplication · 64 × 64 · 64 × 64 · f32. Shorter bars mean less time.](charts/matmul_64_f32.png)

Kernix: **11.744 µs**. Kernix took ≈ 4.3% more time than nalgebra in this run.

**Matrix multiplication · 128 × 128 · 128 × 128 · f32**

![Matrix multiplication · 128 × 128 · 128 × 128 · f32. Shorter bars mean less time.](charts/matmul_128_f32.png)

Kernix: **85.586 µs**. Kernix took ≈ 0.4% less time than Candle in this run.

**Matrix multiplication · 256 × 256 · 256 × 256 · f32**

![Matrix multiplication · 256 × 256 · 256 × 256 · f32. Shorter bars mean less time.](charts/matmul_256_f32.png)

Kernix: **663.1 µs**. Kernix took ≈ 1.5% less time than faer in this run.

**Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64**

![Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64. Shorter bars mean less time.](charts/matmul_batched_4x16x32x16_f64.png)

Kernix: **3.8550000000000005 µs**. Kernix took ≈ 13.0% less time than Candle in this run.

**Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f32**

![Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f32. Shorter bars mean less time.](charts/matmul_batched_4x16x32x16_f32.png)

Kernix: **2.384 µs**. Kernix took ≈ 17.2% less time than Candle in this run.

</details>

<details>
<summary>Explicit parallel execution · 4 workloads</summary>

**Matmul · 2 workers · reused output · 64 × 64 · 64 × 64 · f64**

![Matmul · 2 workers · reused output · 64 × 64 · 64 × 64 · f64. Shorter bars mean less time.](charts/parallel_matmul_into_64_f64.png)

Kernix: **16.798 µs**. No external comparison with Kernix was measured for this workload.

**Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f64**

![Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f64. Shorter bars mean less time.](charts/parallel_matmul_into_128_f64.png)

Kernix: **92.914 µs**. No external comparison with Kernix was measured for this workload.

**Matmul · 2 workers · reused output · 256 × 256 · 256 × 256 · f64**

![Matmul · 2 workers · reused output · 256 × 256 · 256 × 256 · f64. Shorter bars mean less time.](charts/parallel_matmul_into_256_f64.png)

Kernix: **698.185 µs**. No external comparison with Kernix was measured for this workload.

**Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f32**

![Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f32. Shorter bars mean less time.](charts/parallel_matmul_into_128_f32.png)

Kernix: **49.044 µs**. No external comparison with Kernix was measured for this workload.

</details>

<details>
<summary>Decompositions & solvers · 19 workloads</summary>

**Cholesky solve · reused factor · 128 × 128, 1 RHS · f64**

![Cholesky solve · reused factor · 128 × 128, 1 RHS · f64. Shorter bars mean less time.](charts/solve_cholesky_128x1_f64.png)

Kernix: **12.839 µs**. Kernix took ≈ 145.8% more time than nalgebra in this run.

**Cholesky solve · reused factor · 256 × 256, 64 RHS · f64**

![Cholesky solve · reused factor · 256 × 256, 64 RHS · f64. Shorter bars mean less time.](charts/solve_cholesky_256x64_f64.png)

Kernix: **1718.605 µs**. Kernix took ≈ 282.9% more time than faer in this run.

**Cholesky solve · reused factor · 512 × 512, 128 RHS · f64**

![Cholesky solve · reused factor · 512 × 512, 128 RHS · f64. Shorter bars mean less time.](charts/solve_cholesky_512x128_f64.png)

Kernix: **13442.704 µs**. Kernix took ≈ 322.6% more time than faer in this run.

**LU solve · reused factor · 256 × 256, 64 RHS · f64**

![LU solve · reused factor · 256 × 256, 64 RHS · f64. Shorter bars mean less time.](charts/solve_lu_256x64_f64.png)

Kernix: **1652.017 µs**. Kernix took ≈ 263.3% more time than faer in this run.

**Cholesky rank-one update · 256 × 256 · f64**

![Cholesky rank-one update · 256 × 256 · f64. Shorter bars mean less time.](charts/cholesky_rank_update_256_f64.png)

Kernix: **151.678 µs**. Kernix took ≈ 348.2% more time than nalgebra in this run.

**Cholesky rank-one downdate · 256 × 256 · f64**

![Cholesky rank-one downdate · 256 × 256 · f64. Shorter bars mean less time.](charts/cholesky_rank_downdate_256_f64.png)

Kernix: **151.558 µs**. Kernix took ≈ 343.7% more time than nalgebra in this run.

**Cholesky rank-one update · 512 × 512 · f64**

![Cholesky rank-one update · 512 × 512 · f64. Shorter bars mean less time.](charts/cholesky_rank_update_512_f64.png)

Kernix: **654.66 µs**. Kernix took ≈ 349.8% more time than nalgebra in this run.

**Thin QR · 512 × 128 · f64**

![Thin QR · 512 × 128 · f64. Shorter bars mean less time.](charts/qr_512x128_f64.png)

Kernix: **4122.5230000000005 µs**. Kernix took ≈ 33.2% more time than faer in this run.

**Thin QR · 1024 × 256 · f64**

![Thin QR · 1024 × 256 · f64. Shorter bars mean less time.](charts/qr_1024x256_f64.png)

Kernix: **23959.756 µs**. Kernix took ≈ 22.4% more time than NumPy in this run.

**Cholesky factorization · 512 × 512 · f64**

![Cholesky factorization · 512 × 512 · f64. Shorter bars mean less time.](charts/cholesky_factor_512_f64.png)

Kernix: **4320.437 µs**. Kernix took ≈ 63.7% more time than faer in this run.

**Column-pivoted QR · 96 × 32 · f64**

![Column-pivoted QR · 96 × 32 · f64. Shorter bars mean less time.](charts/pivoted_qr_96x32_f64.png)

Kernix: **88.378 µs**. Kernix took ≈ 42.0% more time than faer in this run.

**Column-pivoted QR · 256 × 64 · f64**

![Column-pivoted QR · 256 × 64 · f64. Shorter bars mean less time.](charts/pivoted_qr_256x64_f64.png)

Kernix: **783.1 µs**. Kernix took ≈ 65.1% more time than faer in this run.

**Minimum-norm least squares · 64 × 32, rank 16, 4 RHS · f64**

![Minimum-norm least squares · 64 × 32, rank 16, 4 RHS · f64. Shorter bars mean less time.](charts/min_norm_64x32_rank16_f64.png)

Kernix: **72.267 µs**. Kernix took ≈ 22.8% less time than NumPy in this run.

**Minimum-norm least squares · 32 × 64, rank 32, 4 RHS · f64**

![Minimum-norm least squares · 32 × 64, rank 32, 4 RHS · f64. Shorter bars mean less time.](charts/min_norm_32x64_rank32_f64.png)

Kernix: **96.25 µs**. Kernix took ≈ 45.6% less time than NumPy in this run.

**Thin QR · 96 × 32 · f64**

![Thin QR · 96 × 32 · f64. Shorter bars mean less time.](charts/qr_96x32_f64.png)

Kernix: **73.411 µs**. Kernix took ≈ 19.7% more time than nalgebra in this run.

**Thin QR · 256 × 64 · f64**

![Thin QR · 256 × 64 · f64. Shorter bars mean less time.](charts/qr_256x64_f64.png)

Kernix: **693.442 µs**. Kernix took ≈ 53.7% more time than faer in this run.

**Cholesky factorization · 64 × 64 · f64**

![Cholesky factorization · 64 × 64 · f64. Shorter bars mean less time.](charts/cholesky_factor_64_f64.png)

Kernix: **27.374 µs**. Kernix took ≈ 106.6% more time than faer in this run.

**Cholesky factorization · 128 × 128 · f64**

![Cholesky factorization · 128 × 128 · f64. Shorter bars mean less time.](charts/cholesky_factor_128_f64.png)

Kernix: **279.326 µs**. Kernix took ≈ 328.2% more time than faer in this run.

**Cholesky factorization · 256 × 256 · f64**

![Cholesky factorization · 256 × 256 · f64. Shorter bars mean less time.](charts/cholesky_factor_256_f64.png)

Kernix: **763.797 µs**. Kernix took ≈ 89.4% more time than faer in this run.

</details>

<details>
<summary>Training & autodiff · 6 workloads</summary>

**Shared-variable backward · sum(X * X), 64 × 32; prepared graph · f64**

![Shared-variable backward · sum(X * X), 64 × 32; prepared graph · f64. Shorter bars mean less time.](charts/training_shared_backward_64x32_f64.png)

Kernix: **8.408 µs**. Kernix took ≈ 68.5% more time than Candle in this run.

**Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64**

![Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64. Shorter bars mean less time.](charts/matmul_sum_backward_32x16x8_f64.png)

Kernix: **3.519 µs**. Kernix took ≈ 51.0% less time than Candle in this run.

**Model inference · batched normalized/sliced MLP · f64**

![Model inference · batched normalized/sliced MLP · f64. Shorter bars mean less time.](charts/training_forward_2x4x8x4_f64.png)

Kernix: **3.048 µs**. Kernix took ≈ 42.3% less time than Candle in this run.

**Model recording + forward · batched normalized/sliced MLP · f64**

![Model recording + forward · batched normalized/sliced MLP · f64. Shorter bars mean less time.](charts/training_record_2x4x8x4_f64.png)

Kernix: **3.8480000000000005 µs**. Kernix took ≈ 49.4% less time than Candle in this run.

**Model backward · prepared graph → fresh gradients · f64**

![Model backward · prepared graph → fresh gradients · f64. Shorter bars mean less time.](charts/training_backward_2x4x8x4_f64.png)

Kernix: **4.183 µs**. Kernix took ≈ 80.3% less time than Candle in this run.

**Parameter update · transactional SGD, fixed gradients · f64**

![Parameter update · transactional SGD, fixed gradients · f64. Shorter bars mean less time.](charts/training_sgd_2x4x8x4_f64.png)

Kernix: **0.744 µs**. No external comparison with Kernix was measured for this workload.

</details>

<details>
<summary>Tensor contractions · 1 workloads</summary>

**Coalesced tensor contraction · 16 × 32 × 8 · 32 × 8 × 16 · f64**

![Coalesced tensor contraction · 16 × 32 × 8 · 32 × 8 × 16 · f64. Shorter bars mean less time.](charts/tensordot_16x32x8_32x8x16_f64.png)

Kernix: **46.733 µs**. Kernix took ≈ 184.4% more time than NumPy in this run.

</details>

<details>
<summary>Statistics & activations · 1 workloads</summary>

**Row softmax · 256 × 64 · f64**

![Row softmax · 256 × 64 · f64. Shorter bars mean less time.](charts/softmax_256x64_f64.png)

Kernix: **113.008 µs**. Kernix took ≈ 15.9% more time than ndarray in this run.

</details>

<details>
<summary>Tensor construction · 2 workloads</summary>

**Create zeros · 64 × 64 · f32**

![Create zeros · 64 × 64 · f32. Shorter bars mean less time.](charts/create_zeros_64x64_f32.png)

Kernix: **0.208 µs**. Kernix took ≈ 52.9% more time than ndarray in this run.

**Create ones · 64 × 64 · f32**

![Create ones · 64 × 64 · f32. Shorter bars mean less time.](charts/create_ones_64x64_f32.png)

Kernix: **0.256 µs**. Kernix took ≈ 23.1% more time than ndarray in this run.

</details>

## Exact latency table

All values below are microseconds per operation. **Lower is faster.** Missing entries were not measured.

| Workload | Kernix | ndarray | nalgebra | faer | Candle | NumPy | Kernix refactor | Sequential run change |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| Matrix multiplication · transposed view · 128 × 128 · 128 × 128 · f64 | 212.186 | — | — | — | — | **197.098** | — | -1.4% |
| Matrix multiplication · transposed view · 128 × 128 · 128 × 128 · f32 | **92.713** | — | — | — | — | 100.37 | — | +0.4% |
| Independent matrix operands · 128 × 128 · 128 × 128 · f64 | **168.484** | — | — | — | — | 195.419 | — | -0.3% |
| Independent matrix operands · 128 × 128 · 128 × 128 · f32 | **86.258** | — | — | — | — | 99.857 | — | +0.6% |
| Matrix multiplication · 32 × 32 · 32 × 32 · f64 | 3.248 | **3.048** | 3.224 | 3.504 | 3.424 | 8.199 | — | +0.5% |
| Matrix multiplication · 64 × 64 · 64 × 64 · f64 | 22.167 | **21.815** | 22.455 | 22.615 | 22.59 | 31.046 | — | -0.3% |
| Matrix multiplication · 128 × 128 · 128 × 128 · f64 | **168.285** | 210.954 | 207.994 | 168.933 | 168.82 | 195.723 | — | -0.5% |
| Matrix multiplication · 256 × 256 · 256 × 256 · f64 | **1320.48** | 1323.982 | 1332.286 | 1534.937 | 1321.199 | 1469.973 | — | -0.1% |
| Matrix multiplication · 64 × 64 · 64 × 64 · f32 | 11.744 | 11.264 | **11.263** | 12.287 | 11.952 | 20.095 | — | -0.2% |
| Matrix multiplication · 128 × 128 · 128 × 128 · f32 | **85.586** | 94.098 | 93.25 | 87.146 | 85.93 | 99.953 | — | -0.1% |
| Matrix multiplication · 256 × 256 · 256 × 256 · f32 | **663.1** | 757.653 | 756.726 | 673.002 | 753.95 | 724.352 | — | +0.2% |
| Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64 | **3.8550000000000005** | — | — | — | 4.432 | 12.991 | — | +0.6% |
| Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f32 | **2.384** | — | — | — | 2.88 | 5.16 | — | -0.7% |
| Matmul · 2 workers · reused output · 64 × 64 · 64 × 64 · f64 | 16.798 | — | — | — | — | — | — | -0.3% |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f64 | 92.914 | — | — | — | — | — | — | -0.1% |
| Matmul · 2 workers · reused output · 256 × 256 · 256 × 256 · f64 | 698.185 | — | — | — | — | — | — | +0.2% |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f32 | 49.044 | — | — | — | — | — | — | -0.0% |
| Cholesky solve · reused factor · 128 × 128, 1 RHS · f64 | 12.839 | — | **5.224** | 5.896 | — | — | — | +1.7% |
| Cholesky solve · reused factor · 256 × 256, 64 RHS · f64 | 1718.605 | — | 1276.226 | **448.842** | — | — | — | -66.4% |
| Cholesky solve · reused factor · 512 × 512, 128 RHS · f64 | 13442.704 | — | 12652.581 | **3181.25** | — | — | — | -71.2% |
| LU solve · reused factor · 256 × 256, 64 RHS · f64 | 1652.017 | — | 1228.773 | **454.778** | — | — | — | -51.8% |
| Cholesky rank-one update · 256 × 256 · f64 | 151.678 | — | **33.845** | — | — | — | 765.988 | — |
| Cholesky rank-one downdate · 256 × 256 · f64 | 151.558 | — | **34.157** | — | — | — | 764.757 | — |
| Cholesky rank-one update · 512 × 512 · f64 | 654.66 | — | **145.559** | — | — | — | 4323.053 | — |
| Thin QR · 512 × 128 · f64 | 4122.5230000000005 | — | 4608.794 | **3095.064** | — | 4138.346 | — | -13.0% |
| Thin QR · 1024 × 256 · f64 | 23959.756 | — | 35803.543 | 20071.459 | — | **19573.491** | — | -32.9% |
| Cholesky factorization · 512 × 512 · f64 | 4320.437 | — | 8407.403 | **2639.439** | — | 3804.216 | — | -0.7% |
| Column-pivoted QR · 96 × 32 · f64 | 88.378 | — | 114.272 | **62.23600000000001** | — | — | — | -0.3% |
| Column-pivoted QR · 256 × 64 · f64 | 783.1 | — | 1150.907 | **474.448** | — | — | — | -0.2% |
| Minimum-norm least squares · 64 × 32, rank 16, 4 RHS · f64 | **72.267** | — | 117.024 | — | — | 93.65 | — | -0.6% |
| Minimum-norm least squares · 32 × 64, rank 32, 4 RHS · f64 | **96.25** | — | 218.081 | — | — | 176.948 | — | -0.3% |
| Thin QR · 96 × 32 · f64 | 73.411 | — | **61.308** | 65.45199999999999 | — | 66.027 | — | +2.3% |
| Thin QR · 256 × 64 · f64 | 693.442 | — | 586.225 | **451.25** | — | 565.426 | — | -0.3% |
| Cholesky factorization · 64 × 64 · f64 | 27.374 | — | 20.334 | **13.247** | — | 17.983 | — | +0.2% |
| Cholesky factorization · 128 × 128 · f64 | 279.326 | — | 128.89600000000001 | **65.236** | — | 82.282 | — | +0.8% |
| Cholesky factorization · 256 × 256 · f64 | 763.797 | — | 1029.8900000000001 | **403.316** | — | 748.773 | — | -0.5% |
| Shared-variable backward · sum(X * X), 64 × 32; prepared graph · f64 | 8.408 | — | — | — | **4.991** | — | — | +0.5% |
| Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64 | **3.519** | — | — | — | 7.183 | — | — | +0.2% |
| Model inference · batched normalized/sliced MLP · f64 | **3.048** | — | — | — | 5.28 | — | — | +0.5% |
| Model recording + forward · batched normalized/sliced MLP · f64 | **3.8480000000000005** | — | — | — | 7.607 | — | — | +0.0% |
| Model backward · prepared graph → fresh gradients · f64 | **4.183** | — | — | — | 21.199 | — | — | -0.6% |
| Parameter update · transactional SGD, fixed gradients · f64 | 0.744 | — | — | — | — | — | — | +1.1% |
| Coalesced tensor contraction · 16 × 32 × 8 · 32 × 8 × 16 · f64 | 46.733 | — | — | — | — | **16.431** | — | +0.1% |
| Row softmax · 256 × 64 · f64 | 113.008 | **97.465** | — | — | 110.384 | 134.095 | — | -0.1% |
| Create zeros · 64 × 64 · f32 | 0.208 | **0.136** | — | 0.32 | 0.232 | 0.512 | — | +4.0% |
| Create ones · 64 × 64 · f32 | 0.256 | **0.208** | — | 0.23999999999999997 | 0.344 | 1.52 | — | +3.2% |

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
| Matrix multiplication · transposed view · 128 × 128 · 128 × 128 · f64 | 0.9997738334754727× [0.9953238460306846, 1.0194679976178997] | Timing direction unclear |
| Matrix multiplication · transposed view · 128 × 128 · 128 × 128 · f32 | 0.9970730657484128× [0.9917739551423491, 1.0087548004946951] | Timing direction unclear |
| Independent matrix operands · 128 × 128 · 128 × 128 · f64 | 0.998486398278278× [0.9957582184517497, 1.003750007416945] | Timing direction unclear |
| Independent matrix operands · 128 × 128 · 128 × 128 · f32 | 1.0026257439607889× [1.000093394662495, 1.0043065616552176] | Observed time increase |
| Matrix multiplication · 32 × 32 · 32 × 32 · f64 | 1× [0.995085995085995, 1.0003087372645878] | Timing direction unclear |
| Matrix multiplication · 64 × 64 · 64 × 64 · f64 | — | No paired comparison |
| Matrix multiplication · 128 × 128 · 128 × 128 · f64 | 0.9988561419813765× [0.9961922526300783, 1.0001961530231342] | Timing direction unclear |
| Matrix multiplication · 256 × 256 · 256 × 256 · f64 | 1.0001756182785322× [0.9991833617518403, 1.0009622780930691] | Timing direction unclear |
| Matrix multiplication · 64 × 64 · 64 × 64 · f32 | — | No paired comparison |
| Matrix multiplication · 128 × 128 · 128 × 128 · f32 | 1.002169477214626× [1.0002808134228816, 1.0036371291239314] | Observed time increase |
| Matrix multiplication · 256 × 256 · 256 × 256 · f32 | 0.9983208921448529× [0.9957323002012497, 0.9991901500488781] | Observed time reduction |
| Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64 | 1.0041753653444678× [1.0020876826722338, 1.0062761506276152] | Observed time increase |
| Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f32 | 0.9995819397993311× [0.9966555183946488, 1] | Timing direction unclear |
| Matmul · 2 workers · reused output · 64 × 64 · 64 × 64 · f64 | — | No paired comparison |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f64 | 1.0091958049613556× [0.9895708760248662, 1.0212185378419087] | Timing direction unclear |
| Matmul · 2 workers · reused output · 256 × 256 · 256 × 256 · f64 | 1.0036071534217699× [0.997541024812381, 1.006617502894442] | Timing direction unclear |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f32 | 1.0003274059219547× [0.9985314514155177, 1.0022661848471857] | Timing direction unclear |
| Cholesky solve · reused factor · 128 × 128, 1 RHS · f64 | 1.0234986945169713× [1.0037549870922318, 1.028179190751445] | Observed time increase |
| Cholesky solve · reused factor · 256 × 256, 64 RHS · f64 | 0.33792112617530035× [0.3375684915229404, 0.3384791160852235] | Observed time reduction |
| Cholesky solve · reused factor · 512 × 512, 128 RHS · f64 | — | No paired comparison |
| LU solve · reused factor · 256 × 256, 64 RHS · f64 | 0.47913481363078275× [0.47624069058659213, 0.48246535292720216] | Observed time reduction |
| Cholesky rank-one update · 256 × 256 · f64 | — | No paired comparison |
| Cholesky rank-one downdate · 256 × 256 · f64 | — | No paired comparison |
| Cholesky rank-one update · 512 × 512 · f64 | — | No paired comparison |
| Thin QR · 512 × 128 · f64 | 0.8726855191223079× [0.8695658207967547, 0.8755869170594145] | Observed time reduction |
| Thin QR · 1024 × 256 · f64 | 0.6789324478702599× [0.6782241501332267, 0.6818595664936169] | Observed time reduction |
| Cholesky factorization · 512 × 512 · f64 | 0.9962459765630284× [0.994448678774562, 0.999483314139962] | Observed time reduction |
| Column-pivoted QR · 96 × 32 · f64 | — | No paired comparison |
| Column-pivoted QR · 256 × 64 · f64 | — | No paired comparison |
| Minimum-norm least squares · 64 × 32, rank 16, 4 RHS · f64 | — | No paired comparison |
| Minimum-norm least squares · 32 × 64, rank 32, 4 RHS · f64 | — | No paired comparison |
| Thin QR · 96 × 32 · f64 | — | No paired comparison |
| Thin QR · 256 × 64 · f64 | 0.9995848863933859× [0.9948733492097857, 1.0002446010470083] | Timing direction unclear |
| Cholesky factorization · 64 × 64 · f64 | — | No paired comparison |
| Cholesky factorization · 128 × 128 · f64 | 1.0067263843058276× [1.000258549180004, 1.0133176513163846] | Observed time increase |
| Cholesky factorization · 256 × 256 · f64 | 0.9993030841457505× [0.9967016127769105, 1.0029723941355255] | Timing direction unclear |
| Shared-variable backward · sum(X * X), 64 × 32; prepared graph · f64 | — | No paired comparison |
| Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64 | 0.9954545454545455× [0.9889674681753889, 1.0002854695974879] | Timing direction unclear |
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

Minimum-norm rows use pivoted QR plus scaled implicit RZ in Kernix; NumPy native lstsq; nalgebra native SVD/solve. Algorithm costs and rank policies differ. See original method for thresholds. Kernix/nalgebra: prepared base factor, owned modified factor; refactor: same Kernix version, full factorization of preformed modified matrix; base setup and matrix formation excluded

Numbers retain source precision; recorded digits do not imply equivalent physical measurement accuracy. Lowest observed medians are bold only when peers were measured. No aggregate winner or universal ranking is inferred.

[Original report and commentary](original-report.txt) · [Raw measurements](raw.json)

Measured source `90cbbff60e5d7458ef9f89918c99a6cca94b51d1`. Run `runner-ubuntu-24.04-arm-40270611933`. Rendering does not change measurements.
