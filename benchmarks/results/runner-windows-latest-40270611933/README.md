# Kernix 0.12.0 · performance brief

AMD EPYC 7763 64-Core Processor / Windows

**Per-call median, microseconds per operation. Lower is faster. All recorded digits retained.**

46 workloads · 152 observations

Baseline: baseline-windows-latest-40270611933. Sequential run changes are descriptive; paired process evidence is shown separately.

[Interactive HTML (open locally)](report.html) · [Vector PDF](latency.pdf) · [Raw measurements](raw.json)

[![Performance brief preview](latency.png)](latency.pdf)

## Charts by workload

**Shorter bars mean less time.** Blue is Kernix; grey is a comparison implementation; teal is an explicitly selected Kernix backend. Entries are sorted by the recorded median, with a separate zero-based scale per chart. These point observations do not establish a statistical ranking.

Expand a group to see every chart here on GitHub. Click a chart to enlarge it. The exact table below keeps every source digit.

<details>
<summary>Matrix multiplication · 13 workloads</summary>

**Matrix multiplication · transposed view · 128 × 128 · 128 × 128 · f64**

![Matrix multiplication · transposed view · 128 × 128 · 128 × 128 · f64. Shorter bars mean less time.](charts/matmul_transposed_128_f64.png)

Kernix: **191 µs**. Kernix took ≈ 78.2% more time than NumPy in this run.

**Matrix multiplication · transposed view · 128 × 128 · 128 × 128 · f32**

![Matrix multiplication · transposed view · 128 × 128 · 128 × 128 · f32. Shorter bars mean less time.](charts/matmul_transposed_128_f32.png)

Kernix: **92.5 µs**. Kernix took ≈ 53.4% more time than NumPy in this run.

**Independent matrix operands · 128 × 128 · 128 × 128 · f64**

![Independent matrix operands · 128 × 128 · 128 × 128 · f64. Shorter bars mean less time.](charts/matmul_ab_128_f64.png)

Kernix: **136.7 µs**. Kernix took ≈ 35.2% more time than NumPy in this run.

**Independent matrix operands · 128 × 128 · 128 × 128 · f32**

![Independent matrix operands · 128 × 128 · 128 × 128 · f32. Shorter bars mean less time.](charts/matmul_ab_128_f32.png)

Kernix: **71.5 µs**. Kernix took ≈ 26.1% more time than NumPy in this run.

**Matrix multiplication · 32 × 32 · 32 × 32 · f64**

![Matrix multiplication · 32 × 32 · 32 × 32 · f64. Shorter bars mean less time.](charts/matmul_32_f64.png)

Kernix: **2.8 µs**. Kernix took ≈ 16.7% more time than ndarray in this run.

**Matrix multiplication · 64 × 64 · 64 × 64 · f64**

![Matrix multiplication · 64 × 64 · 64 × 64 · f64. Shorter bars mean less time.](charts/matmul_64_f64.png)

Kernix: **13.6 µs**. Kernix took ≈ 3.5% less time than Candle in this run.

**Matrix multiplication · 128 × 128 · 128 × 128 · f64**

![Matrix multiplication · 128 × 128 · 128 × 128 · f64. Shorter bars mean less time.](charts/matmul_128_f64.png)

Kernix: **136.1 µs**. Kernix took ≈ 34.9% more time than NumPy in this run.

**Matrix multiplication · 256 × 256 · 256 × 256 · f64**

![Matrix multiplication · 256 × 256 · 256 × 256 · f64. Shorter bars mean less time.](charts/matmul_256_f64.png)

Kernix: **855.4 µs**. Kernix took ≈ 13.0% more time than NumPy in this run.

**Matrix multiplication · 64 × 64 · 64 × 64 · f32**

![Matrix multiplication · 64 × 64 · 64 × 64 · f32. Shorter bars mean less time.](charts/matmul_64_f32.png)

Kernix: **6.9 µs**. Kernix took ≈ 4.2% less time than faer in this run.

**Matrix multiplication · 128 × 128 · 128 × 128 · f32**

![Matrix multiplication · 128 × 128 · 128 × 128 · f32. Shorter bars mean less time.](charts/matmul_128_f32.png)

Kernix: **70.9 µs**. Kernix took ≈ 34.0% more time than faer in this run.

**Matrix multiplication · 256 × 256 · 256 × 256 · f32**

![Matrix multiplication · 256 × 256 · 256 × 256 · f32. Shorter bars mean less time.](charts/matmul_256_f32.png)

Kernix: **425.1 µs**. Kernix took ≈ 10.0% more time than NumPy in this run.

**Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64**

![Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64. Shorter bars mean less time.](charts/matmul_batched_4x16x32x16_f64.png)

Kernix: **4.5 µs**. Kernix took ≈ 10.0% less time than NumPy in this run.

**Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f32**

![Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f32. Shorter bars mean less time.](charts/matmul_batched_4x16x32x16_f32.png)

Kernix: **1.4 µs**. Kernix took ≈ 53.3% less time than Candle in this run.

</details>

<details>
<summary>Explicit parallel execution · 4 workloads</summary>

**Matmul · 2 workers · reused output · 64 × 64 · 64 × 64 · f64**

![Matmul · 2 workers · reused output · 64 × 64 · 64 × 64 · f64. Shorter bars mean less time.](charts/parallel_matmul_into_64_f64.png)

Kernix: **39.7 µs**. No external comparison with Kernix was measured for this workload.

**Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f64**

![Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f64. Shorter bars mean less time.](charts/parallel_matmul_into_128_f64.png)

Kernix: **96.7 µs**. No external comparison with Kernix was measured for this workload.

**Matmul · 2 workers · reused output · 256 × 256 · 256 × 256 · f64**

![Matmul · 2 workers · reused output · 256 × 256 · 256 × 256 · f64. Shorter bars mean less time.](charts/parallel_matmul_into_256_f64.png)

Kernix: **446.6 µs**. No external comparison with Kernix was measured for this workload.

**Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f32**

![Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f32. Shorter bars mean less time.](charts/parallel_matmul_into_128_f32.png)

Kernix: **60.9 µs**. No external comparison with Kernix was measured for this workload.

</details>

<details>
<summary>Decompositions & solvers · 19 workloads</summary>

**Cholesky solve · reused factor · 128 × 128, 1 RHS · f64**

![Cholesky solve · reused factor · 128 × 128, 1 RHS · f64. Shorter bars mean less time.](charts/solve_cholesky_128x1_f64.png)

Kernix: **18.1 µs**. Kernix took ≈ 182.8% more time than nalgebra in this run.

**Cholesky solve · reused factor · 256 × 256, 64 RHS · f64**

![Cholesky solve · reused factor · 256 × 256, 64 RHS · f64. Shorter bars mean less time.](charts/solve_cholesky_256x64_f64.png)

Kernix: **1282.6 µs**. Kernix took ≈ 295.3% more time than faer in this run.

**Cholesky solve · reused factor · 512 × 512, 128 RHS · f64**

![Cholesky solve · reused factor · 512 × 512, 128 RHS · f64. Shorter bars mean less time.](charts/solve_cholesky_512x128_f64.png)

Kernix: **9497.7 µs**. Kernix took ≈ 318.7% more time than faer in this run.

**LU solve · reused factor · 256 × 256, 64 RHS · f64**

![LU solve · reused factor · 256 × 256, 64 RHS · f64. Shorter bars mean less time.](charts/solve_lu_256x64_f64.png)

Kernix: **1289.5 µs**. Kernix took ≈ 255.3% more time than faer in this run.

**Cholesky rank-one update · 256 × 256 · f64**

![Cholesky rank-one update · 256 × 256 · f64. Shorter bars mean less time.](charts/cholesky_rank_update_256_f64.png)

Kernix: **168.5 µs**. Kernix took ≈ 58.9% less time than nalgebra in this run.

**Cholesky rank-one downdate · 256 × 256 · f64**

![Cholesky rank-one downdate · 256 × 256 · f64. Shorter bars mean less time.](charts/cholesky_rank_downdate_256_f64.png)

Kernix: **200.9 µs**. Kernix took ≈ 52.4% less time than nalgebra in this run.

**Cholesky rank-one update · 512 × 512 · f64**

![Cholesky rank-one update · 512 × 512 · f64. Shorter bars mean less time.](charts/cholesky_rank_update_512_f64.png)

Kernix: **1368.1 µs**. Kernix took ≈ 147.9% more time than nalgebra in this run.

**Thin QR · 512 × 128 · f64**

![Thin QR · 512 × 128 · f64. Shorter bars mean less time.](charts/qr_512x128_f64.png)

Kernix: **3975.5 µs**. Kernix took ≈ 87.5% more time than faer in this run.

**Thin QR · 1024 × 256 · f64**

![Thin QR · 1024 × 256 · f64. Shorter bars mean less time.](charts/qr_1024x256_f64.png)

Kernix: **22884.8 µs**. Kernix took ≈ 81.1% more time than faer in this run.

**Cholesky factorization · 512 × 512 · f64**

![Cholesky factorization · 512 × 512 · f64. Shorter bars mean less time.](charts/cholesky_factor_512_f64.png)

Kernix: **4606.7 µs**. Kernix took ≈ 58.4% more time than faer in this run.

**Column-pivoted QR · 96 × 32 · f64**

![Column-pivoted QR · 96 × 32 · f64. Shorter bars mean less time.](charts/pivoted_qr_96x32_f64.png)

Kernix: **137.7 µs**. Kernix took ≈ 99.6% more time than faer in this run.

**Column-pivoted QR · 256 × 64 · f64**

![Column-pivoted QR · 256 × 64 · f64. Shorter bars mean less time.](charts/pivoted_qr_256x64_f64.png)

Kernix: **913.6 µs**. Kernix took ≈ 117.1% more time than faer in this run.

**Minimum-norm least squares · 64 × 32, rank 16, 4 RHS · f64**

![Minimum-norm least squares · 64 × 32, rank 16, 4 RHS · f64. Shorter bars mean less time.](charts/min_norm_64x32_rank16_f64.png)

Kernix: **91.5 µs**. Kernix took ≈ 12.4% less time than nalgebra in this run.

**Minimum-norm least squares · 32 × 64, rank 32, 4 RHS · f64**

![Minimum-norm least squares · 32 × 64, rank 32, 4 RHS · f64. Shorter bars mean less time.](charts/min_norm_32x64_rank32_f64.png)

Kernix: **110.7 µs**. Kernix took ≈ 37.6% less time than nalgebra in this run.

**Thin QR · 96 × 32 · f64**

![Thin QR · 96 × 32 · f64. Shorter bars mean less time.](charts/qr_96x32_f64.png)

Kernix: **111.1 µs**. Kernix took ≈ 40.1% more time than nalgebra in this run.

**Thin QR · 256 × 64 · f64**

![Thin QR · 256 × 64 · f64. Shorter bars mean less time.](charts/qr_256x64_f64.png)

Kernix: **845.1 µs**. Kernix took ≈ 81.7% more time than faer in this run.

**Cholesky factorization · 64 × 64 · f64**

![Cholesky factorization · 64 × 64 · f64. Shorter bars mean less time.](charts/cholesky_factor_64_f64.png)

Kernix: **35.8 µs**. Kernix took ≈ 146.9% more time than faer in this run.

**Cholesky factorization · 128 × 128 · f64**

![Cholesky factorization · 128 × 128 · f64. Shorter bars mean less time.](charts/cholesky_factor_128_f64.png)

Kernix: **221.6 µs**. Kernix took ≈ 276.9% more time than faer in this run.

**Cholesky factorization · 256 × 256 · f64**

![Cholesky factorization · 256 × 256 · f64. Shorter bars mean less time.](charts/cholesky_factor_256_f64.png)

Kernix: **690.1 µs**. Kernix took ≈ 117.1% more time than faer in this run.

</details>

<details>
<summary>Training & autodiff · 6 workloads</summary>

**Shared-variable backward · sum(X * X), 64 × 32; prepared graph · f64**

![Shared-variable backward · sum(X * X), 64 × 32; prepared graph · f64. Shorter bars mean less time.](charts/training_shared_backward_64x32_f64.png)

Kernix: **10.5 µs**. Kernix took ≈ 81.9% less time than Candle in this run.

**Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64**

![Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64. Shorter bars mean less time.](charts/matmul_sum_backward_32x16x8_f64.png)

Kernix: **4.2 µs**. Kernix took ≈ 57.1% less time than Candle in this run.

**Model inference · batched normalized/sliced MLP · f64**

![Model inference · batched normalized/sliced MLP · f64. Shorter bars mean less time.](charts/training_forward_2x4x8x4_f64.png)

Kernix: **3.5 µs**. Kernix took ≈ 77.3% less time than Candle in this run.

**Model recording + forward · batched normalized/sliced MLP · f64**

![Model recording + forward · batched normalized/sliced MLP · f64. Shorter bars mean less time.](charts/training_record_2x4x8x4_f64.png)

Kernix: **4.9 µs**. Kernix took ≈ 52.0% less time than Candle in this run.

**Model backward · prepared graph → fresh gradients · f64**

![Model backward · prepared graph → fresh gradients · f64. Shorter bars mean less time.](charts/training_backward_2x4x8x4_f64.png)

Kernix: **5.9 µs**. Kernix took ≈ 81.6% less time than Candle in this run.

**Parameter update · transactional SGD, fixed gradients · f64**

![Parameter update · transactional SGD, fixed gradients · f64. Shorter bars mean less time.](charts/training_sgd_2x4x8x4_f64.png)

Kernix: **1 µs**. No external comparison with Kernix was measured for this workload.

</details>

<details>
<summary>Tensor contractions · 1 workloads</summary>

**Coalesced tensor contraction · 16 × 32 × 8 · 32 × 8 × 16 · f64**

![Coalesced tensor contraction · 16 × 32 × 8 · 32 × 8 × 16 · f64. Shorter bars mean less time.](charts/tensordot_16x32x8_32x8x16_f64.png)

Kernix: **64.1 µs**. Kernix took ≈ 434.2% more time than NumPy in this run.

</details>

<details>
<summary>Statistics & activations · 1 workloads</summary>

**Row softmax · 256 × 64 · f64**

![Row softmax · 256 × 64 · f64. Shorter bars mean less time.](charts/softmax_256x64_f64.png)

Kernix: **114 µs**. Kernix took ≈ 14.4% less time than NumPy in this run.

</details>

<details>
<summary>Tensor construction · 2 workloads</summary>

**Create zeros · 64 × 64 · f32**

![Create zeros · 64 × 64 · f32. Shorter bars mean less time.](charts/create_zeros_64x64_f32.png)

Kernix: **0.4 µs**. Kernix took ≈ 33.3% more time than ndarray in this run.

**Create ones · 64 × 64 · f32**

![Create ones · 64 × 64 · f32. Shorter bars mean less time.](charts/create_ones_64x64_f32.png)

Kernix: **0.5 µs**. Kernix took ≈ 25.0% more time than ndarray in this run.

</details>

## Exact latency table

All values below are microseconds per operation. **Lower is faster.** Missing entries were not measured.

| Workload | Kernix | ndarray | nalgebra | faer | Candle | NumPy | Kernix refactor | Sequential run change |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| Matrix multiplication · transposed view · 128 × 128 · 128 × 128 · f64 | 191 | — | — | — | — | **107.2** | — | -18.8% |
| Matrix multiplication · transposed view · 128 × 128 · 128 × 128 · f32 | 92.5 | — | — | — | — | **60.3** | — | -21.5% |
| Independent matrix operands · 128 × 128 · 128 × 128 · f64 | 136.7 | — | — | — | — | **101.1** | — | -1.1% |
| Independent matrix operands · 128 × 128 · 128 × 128 · f32 | 71.5 | — | — | — | — | **56.7** | — | -5.3% |
| Matrix multiplication · 32 × 32 · 32 × 32 · f64 | 2.8 | **2.4** | **2.4** | 2.5 | 2.5 | 4.2 | — | +0.0% |
| Matrix multiplication · 64 × 64 · 64 × 64 · f64 | **13.6** | 52.1 | 52.7 | 14.5 | 14.1 | 15.9 | — | +0.7% |
| Matrix multiplication · 128 × 128 · 128 × 128 · f64 | 136.1 | 117.4 | 113.8 | 111 | 139.1 | **100.9** | — | +0.5% |
| Matrix multiplication · 256 × 256 · 256 × 256 · f64 | 855.4 | 1144.8 | 956.2 | 1132.3 | 855.4 | **756.8** | — | +18.0% |
| Matrix multiplication · 64 × 64 · 64 × 64 · f32 | **6.9** | 26.1 | 7.8 | 7.2 | 7.4 | 10.4 | — | -10.4% |
| Matrix multiplication · 128 × 128 · 128 × 128 · f32 | 70.9 | 109.9 | 53.6 | **52.9** | 72.2 | 56.2 | — | +0.1% |
| Matrix multiplication · 256 × 256 · 256 × 256 · f32 | 425.1 | 389.1 | 394.1 | 421.9 | 451.1 | **386.5** | — | +19.0% |
| Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64 | **4.5** | — | — | — | 7.5 | 5 | — | +87.5% |
| Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f32 | **1.4** | — | — | — | 3 | 4.8 | — | +0.0% |
| Matmul · 2 workers · reused output · 64 × 64 · 64 × 64 · f64 | 39.7 | — | — | — | — | — | — | -51.3% |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f64 | 96.7 | — | — | — | — | — | — | +15.8% |
| Matmul · 2 workers · reused output · 256 × 256 · 256 × 256 · f64 | 446.6 | — | — | — | — | — | — | +6.4% |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f32 | 60.9 | — | — | — | — | — | — | +5.7% |
| Cholesky solve · reused factor · 128 × 128, 1 RHS · f64 | 18.1 | — | **6.4** | 6.6 | — | — | — | -4.7% |
| Cholesky solve · reused factor · 256 × 256, 64 RHS · f64 | 1282.6 | — | 1390.7 | **324.5** | — | — | — | -81.3% |
| Cholesky solve · reused factor · 512 × 512, 128 RHS · f64 | 9497.7 | — | 10940.4 | **2268.5** | — | — | — | -90.3% |
| LU solve · reused factor · 256 × 256, 64 RHS · f64 | 1289.5 | — | 1196.9 | **362.9** | — | — | — | -77.9% |
| Cholesky rank-one update · 256 × 256 · f64 | **168.5** | — | 409.7 | — | — | — | 1096.8 | — |
| Cholesky rank-one downdate · 256 × 256 · f64 | **200.9** | — | 421.9 | — | — | — | 1098.4 | — |
| Cholesky rank-one update · 512 × 512 · f64 | 1368.1 | — | **551.8** | — | — | — | 4768.9 | — |
| Thin QR · 512 × 128 · f64 | 3975.5 | — | 4390.9 | **2120.2** | — | 4093.5 | — | -18.2% |
| Thin QR · 1024 × 256 · f64 | 22884.8 | — | 33780.4 | **12635.7** | — | 20803 | — | -36.3% |
| Cholesky factorization · 512 × 512 · f64 | 4606.7 | — | 8140.700000000001 | **2907.5** | — | 6342.1 | — | +2.4% |
| Column-pivoted QR · 96 × 32 · f64 | 137.7 | — | 105.5 | **69** | — | — | — | +16.5% |
| Column-pivoted QR · 256 × 64 · f64 | 913.6 | — | 935.1 | **420.9** | — | — | — | +1.2% |
| Minimum-norm least squares · 64 × 32, rank 16, 4 RHS · f64 | **91.5** | — | 104.4 | — | — | 200.9 | — | -5.9% |
| Minimum-norm least squares · 32 × 64, rank 32, 4 RHS · f64 | **110.7** | — | 177.3 | — | — | 206.1 | — | +0.5% |
| Thin QR · 96 × 32 · f64 | 111.1 | — | **79.3** | 83 | — | 127.2 | — | +13.0% |
| Thin QR · 256 × 64 · f64 | 845.1 | — | 642.9 | **465.2** | — | 548.6 | — | +9.2% |
| Cholesky factorization · 64 × 64 · f64 | 35.8 | — | 20.3 | **14.5** | — | 21.1 | — | +0.3% |
| Cholesky factorization · 128 × 128 · f64 | 221.6 | — | 117.9 | **58.8** | — | 113.7 | — | -3.9% |
| Cholesky factorization · 256 × 256 · f64 | 690.1 | — | 869.8 | **317.8** | — | 937.8 | — | -1.6% |
| Shared-variable backward · sum(X * X), 64 × 32; prepared graph · f64 | **10.5** | — | — | — | 58 | — | — | +2.9% |
| Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64 | **4.2** | — | — | — | 9.8 | — | — | -6.7% |
| Model inference · batched normalized/sliced MLP · f64 | **3.5** | — | — | — | 15.400000000000002 | — | — | +0.0% |
| Model recording + forward · batched normalized/sliced MLP · f64 | **4.9** | — | — | — | 10.2 | — | — | -2.0% |
| Model backward · prepared graph → fresh gradients · f64 | **5.9** | — | — | — | 32 | — | — | -3.3% |
| Parameter update · transactional SGD, fixed gradients · f64 | 1 | — | — | — | — | — | — | +0.0% |
| Coalesced tensor contraction · 16 × 32 × 8 · 32 × 8 × 16 · f64 | 64.1 | — | — | — | — | **12** | — | +0.3% |
| Row softmax · 256 × 64 · f64 | **114** | 153.7 | — | — | 181.7 | 133.2 | — | +0.0% |
| Create zeros · 64 × 64 · f32 | 0.4 | **0.3** | — | 0.5 | 0.5 | 0.8 | — | +0.0% |
| Create ones · 64 × 64 · f32 | 0.5 | **0.4** | — | 0.5 | 0.7 | 2.1 | — | +0.0% |

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
| Matrix multiplication · transposed view · 128 × 128 · 128 × 128 · f64 | 0.8276896699528504× [0.8116910229645093, 0.8392311052861512] | Observed time reduction |
| Matrix multiplication · transposed view · 128 × 128 · 128 × 128 · f32 | 0.8266666666666667× [0.8089117395029991, 0.8577699736611062] | Observed time reduction |
| Independent matrix operands · 128 × 128 · 128 × 128 · f64 | 0.9978308026030369× [0.9956772334293948, 1.032490974729242] | Timing direction unclear |
| Independent matrix operands · 128 × 128 · 128 × 128 · f32 | 0.9919571045576407× [0.908344733242134, 1.0027359781121752] | Timing direction unclear |
| Matrix multiplication · 32 × 32 · 32 × 32 · f64 | 1× [1, 1] | Timing direction unclear |
| Matrix multiplication · 64 × 64 · 64 × 64 · f64 | — | No paired comparison |
| Matrix multiplication · 128 × 128 · 128 × 128 · f64 | 0.9989429175475687× [0.6957478005865103, 1.4261603375527425] | Timing direction unclear |
| Matrix multiplication · 256 × 256 · 256 × 256 · f64 | 0.9964324917672887× [0.8442432683379758, 1.0080343537886134] | Timing direction unclear |
| Matrix multiplication · 64 × 64 · 64 × 64 · f32 | — | No paired comparison |
| Matrix multiplication · 128 × 128 · 128 × 128 · f32 | 1× [0.6532593619972261, 1.0021276595744681] | Timing direction unclear |
| Matrix multiplication · 256 × 256 · 256 × 256 · f32 | 0.9974604966139955× [0.843441466854725, 1.0025359256128488] | Timing direction unclear |
| Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64 | 1× [1, 1.28] | Timing direction unclear |
| Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f32 | 1× [1, 1] | Timing direction unclear |
| Matmul · 2 workers · reused output · 64 × 64 · 64 × 64 · f64 | — | No paired comparison |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f64 | 1.0096153846153846× [0.9894490035169988, 1.0169491525423728] | Timing direction unclear |
| Matmul · 2 workers · reused output · 256 × 256 · 256 × 256 · f64 | 1.0204508482454102× [0.7212323064113239, 1.3266775017413512] | Timing direction unclear |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f32 | 1.0017421602787457× [0.9702797202797203, 1.2707993474714518] | Timing direction unclear |
| Cholesky solve · reused factor · 128 × 128, 1 RHS · f64 | 0.9738219895287958× [0.968421052631579, 0.9738219895287958] | Observed time reduction |
| Cholesky solve · reused factor · 256 × 256, 64 RHS · f64 | 0.18717663909268947× [0.1865605658709107, 0.18858212447356107] | Observed time reduction |
| Cholesky solve · reused factor · 512 × 512, 128 RHS · f64 | — | No paired comparison |
| LU solve · reused factor · 256 × 256, 64 RHS · f64 | 0.22232916265640038× [0.22212072115550346, 0.22289301685586516] | Observed time reduction |
| Cholesky rank-one update · 256 × 256 · f64 | — | No paired comparison |
| Cholesky rank-one downdate · 256 × 256 · f64 | — | No paired comparison |
| Cholesky rank-one update · 512 × 512 · f64 | — | No paired comparison |
| Thin QR · 512 × 128 · f64 | 0.7779124044095739× [0.7753794875913581, 0.7799979937807202] | Observed time reduction |
| Thin QR · 1024 × 256 · f64 | 0.578889476761995× [0.5775322067599985, 0.5826379763927365] | Observed time reduction |
| Cholesky factorization · 512 × 512 · f64 | 1.0013484710078733× [0.979931106784484, 1.0090029357399153] | Timing direction unclear |
| Column-pivoted QR · 96 × 32 · f64 | — | No paired comparison |
| Column-pivoted QR · 256 × 64 · f64 | — | No paired comparison |
| Minimum-norm least squares · 64 × 32, rank 16, 4 RHS · f64 | — | No paired comparison |
| Minimum-norm least squares · 32 × 64, rank 32, 4 RHS · f64 | — | No paired comparison |
| Thin QR · 96 × 32 · f64 | — | No paired comparison |
| Thin QR · 256 × 64 · f64 | 0.9572822465492623× [0.9372616558002215, 1.021105141980046] | Timing direction unclear |
| Cholesky factorization · 64 × 64 · f64 | — | No paired comparison |
| Cholesky factorization · 128 × 128 · f64 | 1.0008684324793746× [0.9608865710560626, 1.0411206507004067] | Timing direction unclear |
| Cholesky factorization · 256 × 256 · f64 | 1.0045150014564521× [1.0007302468234263, 1.0139275766016713] | Observed time increase |
| Shared-variable backward · sum(X * X), 64 × 32; prepared graph · f64 | — | No paired comparison |
| Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64 | 1× [0.9791666666666666, 1.0208333333333333] | Timing direction unclear |
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

Measured source `90cbbff60e5d7458ef9f89918c99a6cca94b51d1`. Run `runner-windows-latest-40270611933`. Rendering does not change measurements.
