# Kernix 0.12.0 · performance brief

AMD EPYC 7763 64-Core Processor / Windows

**Per-call median, microseconds per operation. Lower is faster. All recorded digits retained.**

51 workloads · 170 observations

No previous-version baseline attached. Sequential run changes are descriptive; paired process evidence is shown separately.

[Interactive HTML (open locally)](report.html) · [Vector PDF](latency.pdf) · [Raw measurements](raw.json)

[![Performance brief preview](latency.png)](latency.pdf)

## Charts by workload

**Shorter bars mean less time.** Blue is Kernix; grey is a comparison implementation; teal is an explicitly selected Kernix backend. Entries are sorted by the recorded median, with a separate zero-based scale per chart. These point observations do not establish a statistical ranking.

Expand a group to see every chart here on GitHub. Click a chart to enlarge it. The exact table below keeps every source digit.

<details>
<summary>Sparse matrices & memory · 5 workloads</summary>

**Sparse matrix × vector · 2048 × 2048 · five diagonals · f64**

![Sparse matrix × vector · 2048 × 2048 · five diagonals · f64. Shorter bars mean less time.](charts/sparse_matvec_2048_f64.png)

No external comparison with Kernix was measured for this workload.

**Sparse × dense matrix · 512 × 512 · 32 RHS · five diagonals · f64**

![Sparse × dense matrix · 512 × 512 · 32 RHS · five diagonals · f64. Shorter bars mean less time.](charts/sparse_matmul_512x32_f64.png)

No external comparison with Kernix was measured for this workload.

**Sparse × dense matrix · 512 × 512 · 32 RHS · five diagonals · f32**

![Sparse × dense matrix · 512 × 512 · 32 RHS · five diagonals · f32. Shorter bars mean less time.](charts/sparse_matmul_512x32_f32.png)

No external comparison with Kernix was measured for this workload.

**Sparse × sparse matrix · 256 × 256 · five diagonals each · f64**

![Sparse × sparse matrix · 256 × 256 · five diagonals each · f64. Shorter bars mean less time.](charts/sparse_product_256_f64.png)

No external comparison with Kernix was measured for this workload.

**Automatic CSR construction · 512 × 512 · clone + scan + compress · f64**

![Automatic CSR construction · 512 × 512 · clone + scan + compress · f64. Shorter bars mean less time.](charts/sparse_auto_512_f64.png)

Construction trade-off: CSR compression versus dense copying; outputs use different storage. Compare payload as well as time.

</details>

<details>
<summary>Matrix multiplication · 13 workloads</summary>

**Matrix multiplication · transposed view · 128 × 128 · 128 × 128 · f64**

![Matrix multiplication · transposed view · 128 × 128 · 128 × 128 · f64. Shorter bars mean less time.](charts/matmul_transposed_128_f64.png)

Kernix: **198.8 µs**. Kernix took ≈ 86.1% more time than NumPy in this run.

**Matrix multiplication · transposed view · 128 × 128 · 128 × 128 · f32**

![Matrix multiplication · transposed view · 128 × 128 · 128 × 128 · f32. Shorter bars mean less time.](charts/matmul_transposed_128_f32.png)

Kernix: **98.7 µs**. Kernix took ≈ 62.1% more time than NumPy in this run.

**Independent matrix operands · 128 × 128 · 128 × 128 · f64**

![Independent matrix operands · 128 × 128 · 128 × 128 · f64. Shorter bars mean less time.](charts/matmul_ab_128_f64.png)

Kernix: **141.8 µs**. Kernix took ≈ 40.1% more time than NumPy in this run.

**Independent matrix operands · 128 × 128 · 128 × 128 · f32**

![Independent matrix operands · 128 × 128 · 128 × 128 · f32. Shorter bars mean less time.](charts/matmul_ab_128_f32.png)

Kernix: **75.5 µs**. Kernix took ≈ 33.9% more time than NumPy in this run.

**Matrix multiplication · 32 × 32 · 32 × 32 · f64**

![Matrix multiplication · 32 × 32 · 32 × 32 · f64. Shorter bars mean less time.](charts/matmul_32_f64.png)

Kernix: **2.8 µs**. Kernix took ≈ 21.7% more time than nalgebra in this run.

**Matrix multiplication · 64 × 64 · 64 × 64 · f64**

![Matrix multiplication · 64 × 64 · 64 × 64 · f64. Shorter bars mean less time.](charts/matmul_64_f64.png)

Kernix: **13.2 µs**. Kernix took ≈ 7.0% less time than Candle in this run.

**Matrix multiplication · 128 × 128 · 128 × 128 · f64**

![Matrix multiplication · 128 × 128 · 128 × 128 · f64. Shorter bars mean less time.](charts/matmul_128_f64.png)

Kernix: **96 µs**. Kernix took ≈ 4.0% less time than NumPy in this run.

**Matrix multiplication · 256 × 256 · 256 × 256 · f64**

![Matrix multiplication · 256 × 256 · 256 × 256 · f64. Shorter bars mean less time.](charts/matmul_256_f64.png)

Kernix: **728.4 µs**. Kernix took ≈ 3.9% less time than NumPy in this run.

**Matrix multiplication · 64 × 64 · 64 × 64 · f32**

![Matrix multiplication · 64 × 64 · 64 × 64 · f32. Shorter bars mean less time.](charts/matmul_64_f32.png)

Kernix: **7 µs**. Kernix took ≈ 1.4% less time than faer in this run.

**Matrix multiplication · 128 × 128 · 128 × 128 · f32**

![Matrix multiplication · 128 × 128 · 128 × 128 · f32. Shorter bars mean less time.](charts/matmul_128_f32.png)

Kernix: **75.5 µs**. Kernix took ≈ 50.7% more time than faer in this run.

**Matrix multiplication · 256 × 256 · 256 × 256 · f32**

![Matrix multiplication · 256 × 256 · 256 × 256 · f32. Shorter bars mean less time.](charts/matmul_256_f32.png)

Kernix: **427.3 µs**. Kernix took ≈ 10.4% more time than NumPy in this run.

**Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64**

![Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64. Shorter bars mean less time.](charts/matmul_batched_4x16x32x16_f64.png)

Kernix: **2.4 µs**. Kernix took ≈ 42.9% less time than Candle in this run.

**Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f32**

![Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f32. Shorter bars mean less time.](charts/matmul_batched_4x16x32x16_f32.png)

Kernix: **1.4 µs**. Kernix took ≈ 51.7% less time than Candle in this run.

</details>

<details>
<summary>Explicit parallel execution · 4 workloads</summary>

**Matmul · 2 workers · reused output · 64 × 64 · 64 × 64 · f64**

![Matmul · 2 workers · reused output · 64 × 64 · 64 × 64 · f64. Shorter bars mean less time.](charts/parallel_matmul_into_64_f64.png)

Kernix: **56.1 µs**. No external comparison with Kernix was measured for this workload.

**Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f64**

![Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f64. Shorter bars mean less time.](charts/parallel_matmul_into_128_f64.png)

Kernix: **82.6 µs**. No external comparison with Kernix was measured for this workload.

**Matmul · 2 workers · reused output · 256 × 256 · 256 × 256 · f64**

![Matmul · 2 workers · reused output · 256 × 256 · 256 × 256 · f64. Shorter bars mean less time.](charts/parallel_matmul_into_256_f64.png)

Kernix: **441.1 µs**. No external comparison with Kernix was measured for this workload.

**Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f32**

![Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f32. Shorter bars mean less time.](charts/parallel_matmul_into_128_f32.png)

Kernix: **56.9 µs**. No external comparison with Kernix was measured for this workload.

</details>

<details>
<summary>Decompositions & solvers · 19 workloads</summary>

**Cholesky solve · reused factor · 128 × 128, 1 RHS · f64**

![Cholesky solve · reused factor · 128 × 128, 1 RHS · f64. Shorter bars mean less time.](charts/solve_cholesky_128x1_f64.png)

Kernix: **18.5 µs**. Kernix took ≈ 189.1% more time than nalgebra in this run.

**Cholesky solve · reused factor · 256 × 256, 64 RHS · f64**

![Cholesky solve · reused factor · 256 × 256, 64 RHS · f64. Shorter bars mean less time.](charts/solve_cholesky_256x64_f64.png)

Kernix: **1296.3 µs**. Kernix took ≈ 297.6% more time than faer in this run.

**Cholesky solve · reused factor · 512 × 512, 128 RHS · f64**

![Cholesky solve · reused factor · 512 × 512, 128 RHS · f64. Shorter bars mean less time.](charts/solve_cholesky_512x128_f64.png)

Kernix: **9565.9 µs**. Kernix took ≈ 322.5% more time than faer in this run.

**LU solve · reused factor · 256 × 256, 64 RHS · f64**

![LU solve · reused factor · 256 × 256, 64 RHS · f64. Shorter bars mean less time.](charts/solve_lu_256x64_f64.png)

Kernix: **1306.1 µs**. Kernix took ≈ 251.0% more time than faer in this run.

**Cholesky rank-one update · 256 × 256 · f64**

![Cholesky rank-one update · 256 × 256 · f64. Shorter bars mean less time.](charts/cholesky_rank_update_256_f64.png)

Kernix: **171.4 µs**. Kernix took ≈ 59.1% less time than nalgebra in this run.

**Cholesky rank-one downdate · 256 × 256 · f64**

![Cholesky rank-one downdate · 256 × 256 · f64. Shorter bars mean less time.](charts/cholesky_rank_downdate_256_f64.png)

Kernix: **168.1 µs**. Kernix took ≈ 57.9% less time than nalgebra in this run.

**Cholesky rank-one update · 512 × 512 · f64**

![Cholesky rank-one update · 512 × 512 · f64. Shorter bars mean less time.](charts/cholesky_rank_update_512_f64.png)

Kernix: **1351.3 µs**. Kernix took ≈ 141.4% more time than nalgebra in this run.

**Thin QR · 512 × 128 · f64**

![Thin QR · 512 × 128 · f64. Shorter bars mean less time.](charts/qr_512x128_f64.png)

Kernix: **3989.6000000000005 µs**. Kernix took ≈ 90.4% more time than faer in this run.

**Thin QR · 1024 × 256 · f64**

![Thin QR · 1024 × 256 · f64. Shorter bars mean less time.](charts/qr_1024x256_f64.png)

Kernix: **20948.7 µs**. Kernix took ≈ 63.4% more time than faer in this run.

**Cholesky factorization · 512 × 512 · f64**

![Cholesky factorization · 512 × 512 · f64. Shorter bars mean less time.](charts/cholesky_factor_512_f64.png)

Kernix: **4636.6 µs**. Kernix took ≈ 60.4% more time than faer in this run.

**Column-pivoted QR · 96 × 32 · f64**

![Column-pivoted QR · 96 × 32 · f64. Shorter bars mean less time.](charts/pivoted_qr_96x32_f64.png)

Kernix: **125 µs**. Kernix took ≈ 73.9% more time than faer in this run.

**Column-pivoted QR · 256 × 64 · f64**

![Column-pivoted QR · 256 × 64 · f64. Shorter bars mean less time.](charts/pivoted_qr_256x64_f64.png)

Kernix: **925.8 µs**. Kernix took ≈ 123.9% more time than faer in this run.

**Minimum-norm least squares · 64 × 32, rank 16, 4 RHS · f64**

![Minimum-norm least squares · 64 × 32, rank 16, 4 RHS · f64. Shorter bars mean less time.](charts/min_norm_64x32_rank16_f64.png)

Kernix: **96.6 µs**. Kernix took ≈ 6.7% less time than nalgebra in this run.

**Minimum-norm least squares · 32 × 64, rank 32, 4 RHS · f64**

![Minimum-norm least squares · 32 × 64, rank 32, 4 RHS · f64. Shorter bars mean less time.](charts/min_norm_32x64_rank32_f64.png)

Kernix: **114.9 µs**. Kernix took ≈ 34.3% less time than nalgebra in this run.

**Thin QR · 96 × 32 · f64**

![Thin QR · 96 × 32 · f64. Shorter bars mean less time.](charts/qr_96x32_f64.png)

Kernix: **118.5 µs**. Kernix took ≈ 54.9% more time than NumPy in this run.

**Thin QR · 256 × 64 · f64**

![Thin QR · 256 × 64 · f64. Shorter bars mean less time.](charts/qr_256x64_f64.png)

Kernix: **751 µs**. Kernix took ≈ 94.2% more time than faer in this run.

**Cholesky factorization · 64 × 64 · f64**

![Cholesky factorization · 64 × 64 · f64. Shorter bars mean less time.](charts/cholesky_factor_64_f64.png)

Kernix: **35.8 µs**. Kernix took ≈ 145.2% more time than faer in this run.

**Cholesky factorization · 128 × 128 · f64**

![Cholesky factorization · 128 × 128 · f64. Shorter bars mean less time.](charts/cholesky_factor_128_f64.png)

Kernix: **221.9 µs**. Kernix took ≈ 273.6% more time than faer in this run.

**Cholesky factorization · 256 × 256 · f64**

![Cholesky factorization · 256 × 256 · f64. Shorter bars mean less time.](charts/cholesky_factor_256_f64.png)

Kernix: **697 µs**. Kernix took ≈ 123.8% more time than faer in this run.

</details>

<details>
<summary>Training & autodiff · 6 workloads</summary>

**Shared-variable backward · sum(X * X), 64 × 32; prepared graph · f64**

![Shared-variable backward · sum(X * X), 64 × 32; prepared graph · f64. Shorter bars mean less time.](charts/training_shared_backward_64x32_f64.png)

Kernix: **9.7 µs**. Kernix took ≈ 83.4% less time than Candle in this run.

**Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64**

![Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64. Shorter bars mean less time.](charts/matmul_sum_backward_32x16x8_f64.png)

Kernix: **4.6 µs**. Kernix took ≈ 53.5% less time than Candle in this run.

**Model inference · batched normalized/sliced MLP · f64**

![Model inference · batched normalized/sliced MLP · f64. Shorter bars mean less time.](charts/training_forward_2x4x8x4_f64.png)

Kernix: **3.5 µs**. Kernix took ≈ 56.8% less time than Candle in this run.

**Model recording + forward · batched normalized/sliced MLP · f64**

![Model recording + forward · batched normalized/sliced MLP · f64. Shorter bars mean less time.](charts/training_record_2x4x8x4_f64.png)

Kernix: **4.9 µs**. Kernix took ≈ 51.5% less time than Candle in this run.

**Model backward · prepared graph → fresh gradients · f64**

![Model backward · prepared graph → fresh gradients · f64. Shorter bars mean less time.](charts/training_backward_2x4x8x4_f64.png)

Kernix: **6.1 µs**. Kernix took ≈ 80.9% less time than Candle in this run.

**Parameter update · transactional SGD, fixed gradients · f64**

![Parameter update · transactional SGD, fixed gradients · f64. Shorter bars mean less time.](charts/training_sgd_2x4x8x4_f64.png)

Kernix: **1 µs**. No external comparison with Kernix was measured for this workload.

</details>

<details>
<summary>Tensor contractions · 1 workloads</summary>

**Coalesced tensor contraction · 16 × 32 × 8 · 32 × 8 × 16 · f64**

![Coalesced tensor contraction · 16 × 32 × 8 · 32 × 8 × 16 · f64. Shorter bars mean less time.](charts/tensordot_16x32x8_32x8x16_f64.png)

Kernix: **64 µs**. Kernix took ≈ 428.9% more time than NumPy in this run.

</details>

<details>
<summary>Statistics & activations · 1 workloads</summary>

**Row softmax · 256 × 64 · f64**

![Row softmax · 256 × 64 · f64. Shorter bars mean less time.](charts/softmax_256x64_f64.png)

Kernix: **112.6 µs**. Kernix took ≈ 15.0% less time than NumPy in this run.

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

| Workload | Kernix | ndarray | nalgebra | faer | Candle | NumPy | Kernix refactor | Kernix · explicit dense | SciPy · CSR |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| Sparse matrix × vector · 2048 × 2048 · five diagonals · f64 | — | — | — | **7.900000000000001** | — | 655.9 | — | 2669.8 | 11.3 |
| Sparse × dense matrix · 512 × 512 · 32 RHS · five diagonals · f64 | — | — | — | 67.7 | — | 466.3 | — | 626.3 | **41.6** |
| Sparse × dense matrix · 512 × 512 · 32 RHS · five diagonals · f32 | — | — | — | 64.1 | — | 215.8 | — | 332.1 | **26.5** |
| Sparse × sparse matrix · 256 × 256 · five diagonals each · f64 | — | — | — | — | — | 760 | — | 745.7 | **66.8** |
| Automatic CSR construction · 512 × 512 · clone + scan + compress · f64 | — | — | — | — | — | 392.4 | — | **382.1** | 1468.3 |
| Matrix multiplication · transposed view · 128 × 128 · 128 × 128 · f64 | 198.8 | — | — | — | — | **106.8** | — | — | — |
| Matrix multiplication · transposed view · 128 × 128 · 128 × 128 · f32 | 98.7 | — | — | — | — | **60.9** | — | — | — |
| Independent matrix operands · 128 × 128 · 128 × 128 · f64 | 141.8 | — | — | — | — | **101.2** | — | — | — |
| Independent matrix operands · 128 × 128 · 128 × 128 · f32 | 75.5 | — | — | — | — | **56.4** | — | — | — |
| Matrix multiplication · 32 × 32 · 32 × 32 · f64 | 2.8 | 2.4 | **2.3** | 2.5 | 2.6 | 4.2 | — | — | — |
| Matrix multiplication · 64 × 64 · 64 × 64 · f64 | **13.2** | 58.2 | 57.8 | 14.5 | 14.2 | 16 | — | — | — |
| Matrix multiplication · 128 × 128 · 128 × 128 · f64 | **96** | 114.6 | 115.7 | 103.1 | 141.2 | 100 | — | — | — |
| Matrix multiplication · 256 × 256 · 256 × 256 · f64 | **728.4** | 1171.6 | 909.6 | 1086.1 | 870.6 | 758 | — | — | — |
| Matrix multiplication · 64 × 64 · 64 × 64 · f32 | **7** | 29.8 | 7.700000000000001 | 7.1 | 7.4 | 10.5 | — | — | — |
| Matrix multiplication · 128 × 128 · 128 × 128 · f32 | 75.5 | 123.6 | 53.5 | **50.1** | 75.1 | 56.1 | — | — | — |
| Matrix multiplication · 256 × 256 · 256 × 256 · f32 | 427.3 | 389.7 | 395 | 409.8 | 427.6 | **387.1** | — | — | — |
| Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64 | **2.4** | — | — | — | 4.2 | 5 | — | — | — |
| Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f32 | **1.4** | — | — | — | 2.9 | 4.8 | — | — | — |
| Matmul · 2 workers · reused output · 64 × 64 · 64 × 64 · f64 | 56.1 | — | — | — | — | — | — | — | — |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f64 | 82.6 | — | — | — | — | — | — | — | — |
| Matmul · 2 workers · reused output · 256 × 256 · 256 × 256 · f64 | 441.1 | — | — | — | — | — | — | — | — |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f32 | 56.9 | — | — | — | — | — | — | — | — |
| Cholesky solve · reused factor · 128 × 128, 1 RHS · f64 | 18.5 | — | **6.4** | 6.5 | — | — | — | — | — |
| Cholesky solve · reused factor · 256 × 256, 64 RHS · f64 | 1296.3 | — | 1320.1 | **326** | — | — | — | — | — |
| Cholesky solve · reused factor · 512 × 512, 128 RHS · f64 | 9565.9 | — | 10219.6 | **2264.3** | — | — | — | — | — |
| LU solve · reused factor · 256 × 256, 64 RHS · f64 | 1306.1 | — | 1154.8 | **372.1** | — | — | — | — | — |
| Cholesky rank-one update · 256 × 256 · f64 | **171.4** | — | 419.4 | — | — | — | 1118 | — | — |
| Cholesky rank-one downdate · 256 × 256 · f64 | **168.1** | — | 399.6 | — | — | — | 1112.3 | — | — |
| Cholesky rank-one update · 512 × 512 · f64 | 1351.3 | — | **559.8** | — | — | — | 4671.4 | — | — |
| Thin QR · 512 × 128 · f64 | 3989.6000000000005 | — | 4291.3 | **2094.8999999999998** | — | 4035.3 | — | — | — |
| Thin QR · 1024 × 256 · f64 | 20948.7 | — | 32593.900000000004 | **12819.1** | — | 21414.1 | — | — | — |
| Cholesky factorization · 512 × 512 · f64 | 4636.6 | — | 8270.2 | **2890.9** | — | 6183.1 | — | — | — |
| Column-pivoted QR · 96 × 32 · f64 | 125 | — | 108.3 | **71.9** | — | — | — | — | — |
| Column-pivoted QR · 256 × 64 · f64 | 925.8 | — | 906 | **413.4** | — | — | — | — | — |
| Minimum-norm least squares · 64 × 32, rank 16, 4 RHS · f64 | **96.6** | — | 103.5 | — | — | 112.7 | — | — | — |
| Minimum-norm least squares · 32 × 64, rank 32, 4 RHS · f64 | **114.9** | — | 174.8 | — | — | 204.3 | — | — | — |
| Thin QR · 96 × 32 · f64 | 118.5 | — | 82.4 | 85.9 | — | **76.5** | — | — | — |
| Thin QR · 256 × 64 · f64 | 751 | — | 650.3 | **386.7** | — | 546.6 | — | — | — |
| Cholesky factorization · 64 × 64 · f64 | 35.8 | — | 20.6 | **14.6** | — | 20.8 | — | — | — |
| Cholesky factorization · 128 × 128 · f64 | 221.9 | — | 121.8 | **59.4** | — | 112.9 | — | — | — |
| Cholesky factorization · 256 × 256 · f64 | 697 | — | 873.5 | **311.4** | — | 988.4 | — | — | — |
| Shared-variable backward · sum(X * X), 64 × 32; prepared graph · f64 | **9.7** | — | — | — | 58.3 | — | — | — | — |
| Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64 | **4.6** | — | — | — | 9.9 | — | — | — | — |
| Model inference · batched normalized/sliced MLP · f64 | **3.5** | — | — | — | 8.1 | — | — | — | — |
| Model recording + forward · batched normalized/sliced MLP · f64 | **4.9** | — | — | — | 10.1 | — | — | — | — |
| Model backward · prepared graph → fresh gradients · f64 | **6.1** | — | — | — | 31.900000000000004 | — | — | — | — |
| Parameter update · transactional SGD, fixed gradients · f64 | 1 | — | — | — | — | — | — | — | — |
| Coalesced tensor contraction · 16 × 32 × 8 · 32 × 8 × 16 · f64 | 64 | — | — | — | — | **12.1** | — | — | — |
| Row softmax · 256 × 64 · f64 | **112.6** | 162.8 | — | — | 167.9 | 132.5 | — | — | — |
| Create zeros · 64 × 64 · f32 | 0.4 | **0.3** | — | 0.5 | 0.5 | 0.8 | — | — | — |
| Create ones · 64 × 64 · f32 | 0.5 | **0.4** | — | 0.5 | 0.7 | 2.1 | — | — | — |

## Matrix memory

Initialized input payload bytes, including sparse indices/offsets. Excludes capacity, object metadata, allocator overhead and construction peak. Same five-diagonal values; SciPy index width is measured rather than assumed.

| Workload | Implementation | Stored entries | Payload bytes | Dense bytes | Index bytes |
|---|---|---:|---:|---:|---:|
| Sparse matrix × vector · 2048 × 2048 · five diagonals · f64 | SciPy · CSR | 10240 | 131076 | 33554432 | 4 |
| Sparse matrix × vector · 2048 × 2048 · five diagonals · f64 | NumPy | 10240 | 33554432 | 33554432 | 0 |
| Sparse matrix × vector · 2048 × 2048 · five diagonals · f64 | faer | 10240 | 180232 | 33554432 | 8 |
| Sparse matrix × vector · 2048 × 2048 · five diagonals · f64 | Kernix · explicit dense | 10240 | 33554432 | 33554432 | 0 |
| Sparse × dense matrix · 512 × 512 · 32 RHS · five diagonals · f64 | SciPy · CSR | 2560 | 32772 | 2097152 | 4 |
| Sparse × dense matrix · 512 × 512 · 32 RHS · five diagonals · f64 | Kernix · explicit dense | 2560 | 2097152 | 2097152 | 0 |
| Sparse × dense matrix · 512 × 512 · 32 RHS · five diagonals · f64 | NumPy | 2560 | 2097152 | 2097152 | 0 |
| Sparse × dense matrix · 512 × 512 · 32 RHS · five diagonals · f64 | faer | 2560 | 45064 | 2097152 | 8 |
| Sparse × dense matrix · 512 × 512 · 32 RHS · five diagonals · f32 | SciPy · CSR | 2560 | 22532 | 1048576 | 4 |
| Sparse × dense matrix · 512 × 512 · 32 RHS · five diagonals · f32 | Kernix · explicit dense | 2560 | 1048576 | 1048576 | 0 |
| Sparse × dense matrix · 512 × 512 · 32 RHS · five diagonals · f32 | faer | 2560 | 34824 | 1048576 | 8 |
| Sparse × dense matrix · 512 × 512 · 32 RHS · five diagonals · f32 | NumPy | 2560 | 1048576 | 1048576 | 0 |
| Sparse × sparse matrix · 256 × 256 · five diagonals each · f64 | SciPy · CSR | 1280 | 16388 | 524288 | 4 |
| Sparse × sparse matrix · 256 × 256 · five diagonals each · f64 | NumPy | 1280 | 524288 | 524288 | 0 |
| Sparse × sparse matrix · 256 × 256 · five diagonals each · f64 | Kernix · explicit dense | 1280 | 524288 | 524288 | 0 |
| Automatic CSR construction · 512 × 512 · clone + scan + compress · f64 | NumPy | 2560 | 2097152 | 2097152 | 0 |
| Automatic CSR construction · 512 × 512 · clone + scan + compress · f64 | SciPy · CSR | 2560 | 32772 | 2097152 | 4 |
| Automatic CSR construction · 512 × 512 · clone + scan + compress · f64 | Kernix · explicit dense | 2560 | 2097152 | 2097152 | 0 |

## Version comparison

0 of 51 workloads have paired evidence. Exploratory 95% intervals resample process pairs, not calls. No multiple-comparison adjustment.

| Workload | Current / baseline [95% interval] | Interpretation |
|---|---:|---|
| Sparse matrix × vector · 2048 × 2048 · five diagonals · f64 | — | No paired comparison |
| Sparse × dense matrix · 512 × 512 · 32 RHS · five diagonals · f64 | — | No paired comparison |
| Sparse × dense matrix · 512 × 512 · 32 RHS · five diagonals · f32 | — | No paired comparison |
| Sparse × sparse matrix · 256 × 256 · five diagonals each · f64 | — | No paired comparison |
| Automatic CSR construction · 512 × 512 · clone + scan + compress · f64 | — | No paired comparison |
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

CPU affinity was not pinned. Frequency and background load are uncontrolled.

Minimum-norm rows use pivoted QR plus scaled implicit RZ in Kernix; NumPy native lstsq; nalgebra native SVD/solve. Algorithm costs and rank policies differ. See original method for thresholds. Kernix/nalgebra: prepared base factor, owned modified factor; refactor: same Kernix version, full factorization of preformed modified matrix; base setup and matrix formation excluded Rank two; cyclic five diagonals; exact dyadic values; 1 worker; owned output; prepared operands except auto construction. Kernix Matrix finite checks vs peer IEEE policies; payload excludes capacity, metadata and allocator overhead. CSR indices usize in Kernix/faer and actual recorded SciPy width. SpGEMM heap merge vs SciPy column accumulator; fill-in depends on pattern. Construction: Kernix clones a Vec then validates/scans/compresses; SciPy borrows dense input and allocates CSR output; NumPy/dense Kernix copy only. Construction compares API trade-offs, not identical algorithms. No universal sparse speed or density threshold claim. This baseline has no Kernix Matrix API; missing observations are omitted, not fabricated.

Numbers retain source precision; recorded digits do not imply equivalent physical measurement accuracy. Lowest observed medians are bold only when peers were measured. No aggregate winner or universal ranking is inferred.

[Original report and commentary](original-report.txt) · [Raw measurements](raw.json)

Measured source `7c11b2165b0cb7b523db4d938a36f1396396a138`. Run `baseline-windows-latest-40289723446`. Rendering does not change measurements.
