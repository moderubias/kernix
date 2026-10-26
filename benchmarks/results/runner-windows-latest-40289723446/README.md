# Kernix 0.13.0 · performance brief

AMD EPYC 7763 64-Core Processor / Windows

**Per-call median, microseconds per operation. Lower is faster. All recorded digits retained.**

51 workloads · 175 observations

Baseline: baseline-windows-latest-40289723446. Sequential run changes are descriptive; paired process evidence is shown separately.

[Interactive HTML (open locally)](report.html) · [Vector PDF](latency.pdf) · [Raw measurements](raw.json)

[![Performance brief preview](latency.png)](latency.pdf)

## Charts by workload

**Shorter bars mean less time.** Blue is Kernix; grey is a comparison implementation; teal is an explicitly selected Kernix backend. Entries are sorted by the recorded median, with a separate zero-based scale per chart. These point observations do not establish a statistical ranking.

Expand a group to see every chart here on GitHub. Click a chart to enlarge it. The exact table below keeps every source digit.

<details>
<summary>Sparse matrices & memory · 5 workloads</summary>

**Sparse matrix × vector · 2048 × 2048 · five diagonals · f64**

![Sparse matrix × vector · 2048 × 2048 · five diagonals · f64. Shorter bars mean less time.](charts/sparse_matvec_2048_f64.png)

Kernix: **19.3 µs**. Kernix took ≈ 144.3% more time than faer in this run.

**Sparse × dense matrix · 512 × 512 · 32 RHS · five diagonals · f64**

![Sparse × dense matrix · 512 × 512 · 32 RHS · five diagonals · f64. Shorter bars mean less time.](charts/sparse_matmul_512x32_f64.png)

Kernix: **69.2 µs**. Kernix took ≈ 62.8% more time than SciPy · CSR in this run.

**Sparse × dense matrix · 512 × 512 · 32 RHS · five diagonals · f32**

![Sparse × dense matrix · 512 × 512 · 32 RHS · five diagonals · f32. Shorter bars mean less time.](charts/sparse_matmul_512x32_f32.png)

Kernix: **20.8 µs**. Kernix took ≈ 21.5% less time than SciPy · CSR in this run.

**Sparse × sparse matrix · 256 × 256 · five diagonals each · f64**

![Sparse × sparse matrix · 256 × 256 · five diagonals each · f64. Shorter bars mean less time.](charts/sparse_product_256_f64.png)

Kernix: **150.3 µs**. Kernix took ≈ 120.1% more time than SciPy · CSR in this run.

**Automatic CSR construction · 512 × 512 · clone + scan + compress · f64**

![Automatic CSR construction · 512 × 512 · clone + scan + compress · f64. Shorter bars mean less time.](charts/sparse_auto_512_f64.png)

Kernix: **1132.2 µs**. Construction trade-off: CSR compression versus dense copying; outputs use different storage. Compare payload as well as time.

</details>

<details>
<summary>Matrix multiplication · 13 workloads</summary>

**Matrix multiplication · transposed view · 128 × 128 · 128 × 128 · f64**

![Matrix multiplication · transposed view · 128 × 128 · 128 × 128 · f64. Shorter bars mean less time.](charts/matmul_transposed_128_f64.png)

Kernix: **199.7 µs**. Kernix took ≈ 86.1% more time than NumPy in this run.

**Matrix multiplication · transposed view · 128 × 128 · 128 × 128 · f32**

![Matrix multiplication · transposed view · 128 × 128 · 128 × 128 · f32. Shorter bars mean less time.](charts/matmul_transposed_128_f32.png)

Kernix: **98.8 µs**. Kernix took ≈ 63.3% more time than NumPy in this run.

**Independent matrix operands · 128 × 128 · 128 × 128 · f64**

![Independent matrix operands · 128 × 128 · 128 × 128 · f64. Shorter bars mean less time.](charts/matmul_ab_128_f64.png)

Kernix: **142.1 µs**. Kernix took ≈ 39.6% more time than NumPy in this run.

**Independent matrix operands · 128 × 128 · 128 × 128 · f32**

![Independent matrix operands · 128 × 128 · 128 × 128 · f32. Shorter bars mean less time.](charts/matmul_ab_128_f32.png)

Kernix: **74.8 µs**. Kernix took ≈ 33.1% more time than NumPy in this run.

**Matrix multiplication · 32 × 32 · 32 × 32 · f64**

![Matrix multiplication · 32 × 32 · 32 × 32 · f64. Shorter bars mean less time.](charts/matmul_32_f64.png)

Kernix: **2.8 µs**. Kernix took ≈ 16.7% more time than ndarray in this run.

**Matrix multiplication · 64 × 64 · 64 × 64 · f64**

![Matrix multiplication · 64 × 64 · 64 × 64 · f64. Shorter bars mean less time.](charts/matmul_64_f64.png)

Kernix: **13.6 µs**. Kernix took ≈ 4.2% less time than Candle in this run.

**Matrix multiplication · 128 × 128 · 128 × 128 · f64**

![Matrix multiplication · 128 × 128 · 128 × 128 · f64. Shorter bars mean less time.](charts/matmul_128_f64.png)

Kernix: **94.9 µs**. Kernix took ≈ 6.7% less time than NumPy in this run.

**Matrix multiplication · 256 × 256 · 256 × 256 · f64**

![Matrix multiplication · 256 × 256 · 256 × 256 · f64. Shorter bars mean less time.](charts/matmul_256_f64.png)

Kernix: **865 µs**. Kernix took ≈ 10.0% more time than NumPy in this run.

**Matrix multiplication · 64 × 64 · 64 × 64 · f32**

![Matrix multiplication · 64 × 64 · 64 × 64 · f32. Shorter bars mean less time.](charts/matmul_64_f32.png)

Kernix: **6.9 µs**. Kernix took ≈ 1.4% less time than faer in this run.

**Matrix multiplication · 128 × 128 · 128 × 128 · f32**

![Matrix multiplication · 128 × 128 · 128 × 128 · f32. Shorter bars mean less time.](charts/matmul_128_f32.png)

Kernix: **74.4 µs**. Kernix took ≈ 48.2% more time than faer in this run.

**Matrix multiplication · 256 × 256 · 256 × 256 · f32**

![Matrix multiplication · 256 × 256 · 256 × 256 · f32. Shorter bars mean less time.](charts/matmul_256_f32.png)

Kernix: **433.3 µs**. Kernix took ≈ 11.5% more time than NumPy in this run.

**Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64**

![Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64. Shorter bars mean less time.](charts/matmul_batched_4x16x32x16_f64.png)

Kernix: **2.4 µs**. Kernix took ≈ 38.5% less time than Candle in this run.

**Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f32**

![Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f32. Shorter bars mean less time.](charts/matmul_batched_4x16x32x16_f32.png)

Kernix: **1.4 µs**. Kernix took ≈ 51.7% less time than Candle in this run.

</details>

<details>
<summary>Explicit parallel execution · 4 workloads</summary>

**Matmul · 2 workers · reused output · 64 × 64 · 64 × 64 · f64**

![Matmul · 2 workers · reused output · 64 × 64 · 64 × 64 · f64. Shorter bars mean less time.](charts/parallel_matmul_into_64_f64.png)

Kernix: **41.1 µs**. No external comparison with Kernix was measured for this workload.

**Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f64**

![Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f64. Shorter bars mean less time.](charts/parallel_matmul_into_128_f64.png)

Kernix: **83.9 µs**. No external comparison with Kernix was measured for this workload.

**Matmul · 2 workers · reused output · 256 × 256 · 256 × 256 · f64**

![Matmul · 2 workers · reused output · 256 × 256 · 256 × 256 · f64. Shorter bars mean less time.](charts/parallel_matmul_into_256_f64.png)

Kernix: **607.9 µs**. No external comparison with Kernix was measured for this workload.

**Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f32**

![Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f32. Shorter bars mean less time.](charts/parallel_matmul_into_128_f32.png)

Kernix: **60.3 µs**. No external comparison with Kernix was measured for this workload.

</details>

<details>
<summary>Decompositions & solvers · 19 workloads</summary>

**Cholesky solve · reused factor · 128 × 128, 1 RHS · f64**

![Cholesky solve · reused factor · 128 × 128, 1 RHS · f64. Shorter bars mean less time.](charts/solve_cholesky_128x1_f64.png)

Kernix: **18.2 µs**. Kernix took ≈ 188.9% more time than nalgebra in this run.

**Cholesky solve · reused factor · 256 × 256, 64 RHS · f64**

![Cholesky solve · reused factor · 256 × 256, 64 RHS · f64. Shorter bars mean less time.](charts/solve_cholesky_256x64_f64.png)

Kernix: **1395.7 µs**. Kernix took ≈ 324.4% more time than faer in this run.

**Cholesky solve · reused factor · 512 × 512, 128 RHS · f64**

![Cholesky solve · reused factor · 512 × 512, 128 RHS · f64. Shorter bars mean less time.](charts/solve_cholesky_512x128_f64.png)

Kernix: **10320.4 µs**. Kernix took ≈ 356.7% more time than faer in this run.

**LU solve · reused factor · 256 × 256, 64 RHS · f64**

![LU solve · reused factor · 256 × 256, 64 RHS · f64. Shorter bars mean less time.](charts/solve_lu_256x64_f64.png)

Kernix: **1302.1 µs**. Kernix took ≈ 248.2% more time than faer in this run.

**Cholesky rank-one update · 256 × 256 · f64**

![Cholesky rank-one update · 256 × 256 · f64. Shorter bars mean less time.](charts/cholesky_rank_update_256_f64.png)

Kernix: **169.5 µs**. Kernix took ≈ 57.8% less time than nalgebra in this run.

**Cholesky rank-one downdate · 256 × 256 · f64**

![Cholesky rank-one downdate · 256 × 256 · f64. Shorter bars mean less time.](charts/cholesky_rank_downdate_256_f64.png)

Kernix: **167.9 µs**. Kernix took ≈ 59.5% less time than nalgebra in this run.

**Cholesky rank-one update · 512 × 512 · f64**

![Cholesky rank-one update · 512 × 512 · f64. Shorter bars mean less time.](charts/cholesky_rank_update_512_f64.png)

Kernix: **1341 µs**. Kernix took ≈ 140.0% more time than nalgebra in this run.

**Thin QR · 512 × 128 · f64**

![Thin QR · 512 × 128 · f64. Shorter bars mean less time.](charts/qr_512x128_f64.png)

Kernix: **4076.5000000000005 µs**. Kernix took ≈ 92.8% more time than faer in this run.

**Thin QR · 1024 × 256 · f64**

![Thin QR · 1024 × 256 · f64. Shorter bars mean less time.](charts/qr_1024x256_f64.png)

Kernix: **21273.4 µs**. Kernix took ≈ 63.5% more time than faer in this run.

**Cholesky factorization · 512 × 512 · f64**

![Cholesky factorization · 512 × 512 · f64. Shorter bars mean less time.](charts/cholesky_factor_512_f64.png)

Kernix: **4659.2 µs**. Kernix took ≈ 53.7% more time than faer in this run.

**Column-pivoted QR · 96 × 32 · f64**

![Column-pivoted QR · 96 × 32 · f64. Shorter bars mean less time.](charts/pivoted_qr_96x32_f64.png)

Kernix: **184.4 µs**. Kernix took ≈ 163.8% more time than faer in this run.

**Column-pivoted QR · 256 × 64 · f64**

![Column-pivoted QR · 256 × 64 · f64. Shorter bars mean less time.](charts/pivoted_qr_256x64_f64.png)

Kernix: **906.5 µs**. Kernix took ≈ 120.6% more time than faer in this run.

**Minimum-norm least squares · 64 × 32, rank 16, 4 RHS · f64**

![Minimum-norm least squares · 64 × 32, rank 16, 4 RHS · f64. Shorter bars mean less time.](charts/min_norm_64x32_rank16_f64.png)

Kernix: **96.8 µs**. Kernix took ≈ 6.5% less time than nalgebra in this run.

**Minimum-norm least squares · 32 × 64, rank 32, 4 RHS · f64**

![Minimum-norm least squares · 32 × 64, rank 32, 4 RHS · f64. Shorter bars mean less time.](charts/min_norm_32x64_rank32_f64.png)

Kernix: **114.3 µs**. Kernix took ≈ 34.8% less time than nalgebra in this run.

**Thin QR · 96 × 32 · f64**

![Thin QR · 96 × 32 · f64. Shorter bars mean less time.](charts/qr_96x32_f64.png)

Kernix: **100.2 µs**. Kernix took ≈ 30.6% more time than NumPy in this run.

**Thin QR · 256 × 64 · f64**

![Thin QR · 256 × 64 · f64. Shorter bars mean less time.](charts/qr_256x64_f64.png)

Kernix: **771.5 µs**. Kernix took ≈ 98.7% more time than faer in this run.

**Cholesky factorization · 64 × 64 · f64**

![Cholesky factorization · 64 × 64 · f64. Shorter bars mean less time.](charts/cholesky_factor_64_f64.png)

Kernix: **37.5 µs**. Kernix took ≈ 155.1% more time than faer in this run.

**Cholesky factorization · 128 × 128 · f64**

![Cholesky factorization · 128 × 128 · f64. Shorter bars mean less time.](charts/cholesky_factor_128_f64.png)

Kernix: **220.6 µs**. Kernix took ≈ 270.1% more time than faer in this run.

**Cholesky factorization · 256 × 256 · f64**

![Cholesky factorization · 256 × 256 · f64. Shorter bars mean less time.](charts/cholesky_factor_256_f64.png)

Kernix: **705.7 µs**. Kernix took ≈ 124.7% more time than faer in this run.

</details>

<details>
<summary>Training & autodiff · 6 workloads</summary>

**Shared-variable backward · sum(X * X), 64 × 32; prepared graph · f64**

![Shared-variable backward · sum(X * X), 64 × 32; prepared graph · f64. Shorter bars mean less time.](charts/training_shared_backward_64x32_f64.png)

Kernix: **33.1 µs**. Kernix took ≈ 42.9% less time than Candle in this run.

**Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64**

![Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64. Shorter bars mean less time.](charts/matmul_sum_backward_32x16x8_f64.png)

Kernix: **4 µs**. Kernix took ≈ 59.6% less time than Candle in this run.

**Model inference · batched normalized/sliced MLP · f64**

![Model inference · batched normalized/sliced MLP · f64. Shorter bars mean less time.](charts/training_forward_2x4x8x4_f64.png)

Kernix: **3.6 µs**. Kernix took ≈ 56.1% less time than Candle in this run.

**Model recording + forward · batched normalized/sliced MLP · f64**

![Model recording + forward · batched normalized/sliced MLP · f64. Shorter bars mean less time.](charts/training_record_2x4x8x4_f64.png)

Kernix: **5 µs**. Kernix took ≈ 48.5% less time than Candle in this run.

**Model backward · prepared graph → fresh gradients · f64**

![Model backward · prepared graph → fresh gradients · f64. Shorter bars mean less time.](charts/training_backward_2x4x8x4_f64.png)

Kernix: **6 µs**. Kernix took ≈ 81.2% less time than Candle in this run.

**Parameter update · transactional SGD, fixed gradients · f64**

![Parameter update · transactional SGD, fixed gradients · f64. Shorter bars mean less time.](charts/training_sgd_2x4x8x4_f64.png)

Kernix: **1 µs**. No external comparison with Kernix was measured for this workload.

</details>

<details>
<summary>Tensor contractions · 1 workloads</summary>

**Coalesced tensor contraction · 16 × 32 × 8 · 32 × 8 × 16 · f64**

![Coalesced tensor contraction · 16 × 32 × 8 · 32 × 8 × 16 · f64. Shorter bars mean less time.](charts/tensordot_16x32x8_32x8x16_f64.png)

Kernix: **63.89999999999999 µs**. Kernix took ≈ 428.1% more time than NumPy in this run.

</details>

<details>
<summary>Statistics & activations · 1 workloads</summary>

**Row softmax · 256 × 64 · f64**

![Row softmax · 256 × 64 · f64. Shorter bars mean less time.](charts/softmax_256x64_f64.png)

Kernix: **112.5 µs**. Kernix took ≈ 14.4% less time than NumPy in this run.

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

| Workload | Kernix | ndarray | nalgebra | faer | Candle | NumPy | Kernix refactor | Kernix · explicit dense | SciPy · CSR | Sequential run change |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| Sparse matrix × vector · 2048 × 2048 · five diagonals · f64 | 19.3 | — | — | **7.900000000000001** | — | 815.4 | — | 2588.1 | 11.2 | — |
| Sparse × dense matrix · 512 × 512 · 32 RHS · five diagonals · f64 | 69.2 | — | — | 67.5 | — | 469.9 | — | 627.9 | **42.5** | — |
| Sparse × dense matrix · 512 × 512 · 32 RHS · five diagonals · f32 | **20.8** | — | — | 95.3 | — | 220.3 | — | 333.2 | 26.5 | — |
| Sparse × sparse matrix · 256 × 256 · five diagonals each · f64 | 150.3 | — | — | — | — | 759.3 | — | 756.7 | **68.3** | — |
| Automatic CSR construction · 512 × 512 · clone + scan + compress · f64 | 1132.2 | — | — | — | — | 414.2 | — | **386.4** | 1468.9 | — |
| Matrix multiplication · transposed view · 128 × 128 · 128 × 128 · f64 | 199.7 | — | — | — | — | **107.3** | — | — | — | +0.5% |
| Matrix multiplication · transposed view · 128 × 128 · 128 × 128 · f32 | 98.8 | — | — | — | — | **60.5** | — | — | — | +0.1% |
| Independent matrix operands · 128 × 128 · 128 × 128 · f64 | 142.1 | — | — | — | — | **101.8** | — | — | — | +0.2% |
| Independent matrix operands · 128 × 128 · 128 × 128 · f32 | 74.8 | — | — | — | — | **56.2** | — | — | — | -0.9% |
| Matrix multiplication · 32 × 32 · 32 × 32 · f64 | 2.8 | **2.4** | 3.5 | 2.5 | 4.5 | 6.7 | — | — | — | +0.0% |
| Matrix multiplication · 64 × 64 · 64 × 64 · f64 | **13.6** | 60 | 57.9 | 14.6 | 14.2 | 16 | — | — | — | +3.0% |
| Matrix multiplication · 128 × 128 · 128 × 128 · f64 | **94.9** | 114.4 | 115.9 | 107.3 | 150.4 | 101.7 | — | — | — | -1.1% |
| Matrix multiplication · 256 × 256 · 256 × 256 · f64 | 865 | 1181.8 | 905.1 | 1213.6 | 856 | **786.4** | — | — | — | +18.8% |
| Matrix multiplication · 64 × 64 · 64 × 64 · f32 | **6.9** | 30 | 30.1 | 7 | 7.4 | 10.4 | — | — | — | -1.4% |
| Matrix multiplication · 128 × 128 · 128 × 128 · f32 | 74.4 | 128.2 | 53.8 | **50.2** | 75.3 | 56.2 | — | — | — | -1.5% |
| Matrix multiplication · 256 × 256 · 256 × 256 · f32 | 433.3 | 390.8 | 398 | 430.4 | 451.8 | **388.7** | — | — | — | +1.4% |
| Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64 | **2.4** | — | — | — | 3.9 | 5 | — | — | — | +0.0% |
| Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f32 | **1.4** | — | — | — | 2.9 | 4.8 | — | — | — | +0.0% |
| Matmul · 2 workers · reused output · 64 × 64 · 64 × 64 · f64 | 41.1 | — | — | — | — | — | — | — | — | -26.7% |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f64 | 83.9 | — | — | — | — | — | — | — | — | +1.6% |
| Matmul · 2 workers · reused output · 256 × 256 · 256 × 256 · f64 | 607.9 | — | — | — | — | — | — | — | — | +37.8% |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f32 | 60.3 | — | — | — | — | — | — | — | — | +6.0% |
| Cholesky solve · reused factor · 128 × 128, 1 RHS · f64 | 18.2 | — | **6.3** | 6.5 | — | — | — | — | — | -1.6% |
| Cholesky solve · reused factor · 256 × 256, 64 RHS · f64 | 1395.7 | — | 1326.8 | **328.9** | — | — | — | — | — | +7.7% |
| Cholesky solve · reused factor · 512 × 512, 128 RHS · f64 | 10320.4 | — | 10418.7 | **2259.9** | — | — | — | — | — | +7.9% |
| LU solve · reused factor · 256 × 256, 64 RHS · f64 | 1302.1 | — | 1177.9 | **373.9** | — | — | — | — | — | -0.3% |
| Cholesky rank-one update · 256 × 256 · f64 | **169.5** | — | 402.1 | — | — | — | 1106.1 | — | — | -1.1% |
| Cholesky rank-one downdate · 256 × 256 · f64 | **167.9** | — | 415 | — | — | — | 1105.6 | — | — | -0.1% |
| Cholesky rank-one update · 512 × 512 · f64 | 1341 | — | **558.8** | — | — | — | 4640.4 | — | — | -0.8% |
| Thin QR · 512 × 128 · f64 | 4076.5000000000005 | — | 4001.9 | **2114.1** | — | 4854.2 | — | — | — | +2.2% |
| Thin QR · 1024 × 256 · f64 | 21273.4 | — | 32876.9 | **13013.9** | — | 21444 | — | — | — | +1.5% |
| Cholesky factorization · 512 × 512 · f64 | 4659.2 | — | 8121.8 | **3030.9** | — | 6826.6 | — | — | — | +0.5% |
| Column-pivoted QR · 96 × 32 · f64 | 184.4 | — | 106.3 | **69.9** | — | — | — | — | — | +47.5% |
| Column-pivoted QR · 256 × 64 · f64 | 906.5 | — | 917.3 | **410.9** | — | — | — | — | — | -2.1% |
| Minimum-norm least squares · 64 × 32, rank 16, 4 RHS · f64 | **96.8** | — | 103.5 | — | — | 112.8 | — | — | — | +0.2% |
| Minimum-norm least squares · 32 × 64, rank 32, 4 RHS · f64 | **114.3** | — | 175.2 | — | — | 203.7 | — | — | — | -0.5% |
| Thin QR · 96 × 32 · f64 | 100.2 | — | 79.6 | 86 | — | **76.7** | — | — | — | -15.4% |
| Thin QR · 256 × 64 · f64 | 771.5 | — | 580 | **388.2** | — | 548 | — | — | — | +2.7% |
| Cholesky factorization · 64 × 64 · f64 | 37.5 | — | 21 | **14.7** | — | 21.2 | — | — | — | +4.7% |
| Cholesky factorization · 128 × 128 · f64 | 220.6 | — | 117.6 | **59.6** | — | 114.2 | — | — | — | -0.6% |
| Cholesky factorization · 256 × 256 · f64 | 705.7 | — | 853.9 | **314.1** | — | 947.1 | — | — | — | +1.2% |
| Shared-variable backward · sum(X * X), 64 × 32; prepared graph · f64 | **33.1** | — | — | — | 58 | — | — | — | — | +241.2% |
| Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64 | **4** | — | — | — | 9.9 | — | — | — | — | -13.0% |
| Model inference · batched normalized/sliced MLP · f64 | **3.6** | — | — | — | 8.2 | — | — | — | — | +2.9% |
| Model recording + forward · batched normalized/sliced MLP · f64 | **5** | — | — | — | 9.7 | — | — | — | — | +2.0% |
| Model backward · prepared graph → fresh gradients · f64 | **6** | — | — | — | 32 | — | — | — | — | -1.6% |
| Parameter update · transactional SGD, fixed gradients · f64 | 1 | — | — | — | — | — | — | — | — | +0.0% |
| Coalesced tensor contraction · 16 × 32 × 8 · 32 × 8 × 16 · f64 | 63.89999999999999 | — | — | — | — | **12.1** | — | — | — | -0.2% |
| Row softmax · 256 × 64 · f64 | **112.5** | 153.7 | — | — | 184.7 | 131.5 | — | — | — | -0.1% |
| Create zeros · 64 × 64 · f32 | 0.4 | **0.3** | — | 0.5 | 0.5 | 0.8 | — | — | — | +0.0% |
| Create ones · 64 × 64 · f32 | 0.5 | **0.4** | — | 0.5 | 0.7 | 2.1 | — | — | — | +0.0% |

## Matrix memory

Initialized input payload bytes, including sparse indices/offsets. Excludes capacity, object metadata, allocator overhead and construction peak. Same five-diagonal values; SciPy index width is measured rather than assumed.

| Workload | Implementation | Stored entries | Payload bytes | Dense bytes | Index bytes |
|---|---|---:|---:|---:|---:|
| Sparse matrix × vector · 2048 × 2048 · five diagonals · f64 | faer | 10240 | 180232 | 33554432 | 8 |
| Sparse matrix × vector · 2048 × 2048 · five diagonals · f64 | SciPy · CSR | 10240 | 131076 | 33554432 | 4 |
| Sparse matrix × vector · 2048 × 2048 · five diagonals · f64 | NumPy | 10240 | 33554432 | 33554432 | 0 |
| Sparse matrix × vector · 2048 × 2048 · five diagonals · f64 | Kernix · explicit dense | 10240 | 33554432 | 33554432 | 0 |
| Sparse matrix × vector · 2048 × 2048 · five diagonals · f64 | Kernix | 10240 | 180232 | 33554432 | 8 |
| Sparse × dense matrix · 512 × 512 · 32 RHS · five diagonals · f64 | SciPy · CSR | 2560 | 32772 | 2097152 | 4 |
| Sparse × dense matrix · 512 × 512 · 32 RHS · five diagonals · f64 | Kernix · explicit dense | 2560 | 2097152 | 2097152 | 0 |
| Sparse × dense matrix · 512 × 512 · 32 RHS · five diagonals · f64 | NumPy | 2560 | 2097152 | 2097152 | 0 |
| Sparse × dense matrix · 512 × 512 · 32 RHS · five diagonals · f64 | faer | 2560 | 45064 | 2097152 | 8 |
| Sparse × dense matrix · 512 × 512 · 32 RHS · five diagonals · f64 | Kernix | 2560 | 45064 | 2097152 | 8 |
| Sparse × dense matrix · 512 × 512 · 32 RHS · five diagonals · f32 | SciPy · CSR | 2560 | 22532 | 1048576 | 4 |
| Sparse × dense matrix · 512 × 512 · 32 RHS · five diagonals · f32 | Kernix | 2560 | 34824 | 1048576 | 8 |
| Sparse × dense matrix · 512 × 512 · 32 RHS · five diagonals · f32 | Kernix · explicit dense | 2560 | 1048576 | 1048576 | 0 |
| Sparse × dense matrix · 512 × 512 · 32 RHS · five diagonals · f32 | NumPy | 2560 | 1048576 | 1048576 | 0 |
| Sparse × dense matrix · 512 × 512 · 32 RHS · five diagonals · f32 | faer | 2560 | 34824 | 1048576 | 8 |
| Sparse × sparse matrix · 256 × 256 · five diagonals each · f64 | NumPy | 1280 | 524288 | 524288 | 0 |
| Sparse × sparse matrix · 256 × 256 · five diagonals each · f64 | SciPy · CSR | 1280 | 16388 | 524288 | 4 |
| Sparse × sparse matrix · 256 × 256 · five diagonals each · f64 | Kernix · explicit dense | 1280 | 524288 | 524288 | 0 |
| Sparse × sparse matrix · 256 × 256 · five diagonals each · f64 | Kernix | 1280 | 22536 | 524288 | 8 |
| Automatic CSR construction · 512 × 512 · clone + scan + compress · f64 | Kernix | 2560 | 45064 | 2097152 | 8 |
| Automatic CSR construction · 512 × 512 · clone + scan + compress · f64 | SciPy · CSR | 2560 | 32772 | 2097152 | 4 |
| Automatic CSR construction · 512 × 512 · clone + scan + compress · f64 | Kernix · explicit dense | 2560 | 2097152 | 2097152 | 0 |
| Automatic CSR construction · 512 × 512 · clone + scan + compress · f64 | NumPy | 2560 | 2097152 | 2097152 | 0 |

## Version comparison

24 of 51 workloads have paired evidence. Exploratory 95% intervals resample process pairs, not calls. No multiple-comparison adjustment.


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
| Sparse matrix × vector · 2048 × 2048 · five diagonals · f64 | — | No paired comparison |
| Sparse × dense matrix · 512 × 512 · 32 RHS · five diagonals · f64 | — | No paired comparison |
| Sparse × dense matrix · 512 × 512 · 32 RHS · five diagonals · f32 | — | No paired comparison |
| Sparse × sparse matrix · 256 × 256 · five diagonals each · f64 | — | No paired comparison |
| Automatic CSR construction · 512 × 512 · clone + scan + compress · f64 | — | No paired comparison |
| Matrix multiplication · transposed view · 128 × 128 · 128 × 128 · f64 | 1.0135542168674698× [0.5107446276861569, 1.0339157245632067] | Timing direction unclear |
| Matrix multiplication · transposed view · 128 × 128 · 128 × 128 · f32 | 1.020387359836901× [0.990990990990991, 1.0287769784172662] | Timing direction unclear |
| Independent matrix operands · 128 × 128 · 128 × 128 · f64 | 0.9978947368421053× [0.9895324494068388, 0.9992997198879552] | Observed time reduction |
| Independent matrix operands · 128 × 128 · 128 × 128 · f32 | 0.9922480620155039× [0.9896507115135834, 1.0025974025974025] | Timing direction unclear |
| Matrix multiplication · 32 × 32 · 32 × 32 · f64 | 1× [1, 1] | Timing direction unclear |
| Matrix multiplication · 64 × 64 · 64 × 64 · f64 | — | No paired comparison |
| Matrix multiplication · 128 × 128 · 128 × 128 · f64 | 1.0010593220338984× [0.9968354430379747, 1.0031712473572938] | Timing direction unclear |
| Matrix multiplication · 256 × 256 · 256 × 256 · f64 | 1.0054614964500272× [0.9927105831533477, 1.0194106641721234] | Timing direction unclear |
| Matrix multiplication · 64 × 64 · 64 × 64 · f32 | — | No paired comparison |
| Matrix multiplication · 128 × 128 · 128 × 128 · f32 | 1× [0.9957716701902748, 1.0021231422505308] | Timing direction unclear |
| Matrix multiplication · 256 × 256 · 256 × 256 · f32 | 0.9974789915966387× [0.8314212128307188, 1.0033670033670035] | Timing direction unclear |
| Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64 | 1× [0.9615384615384616, 1.04] | Timing direction unclear |
| Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f32 | 1× [1, 1] | Timing direction unclear |
| Matmul · 2 workers · reused output · 64 × 64 · 64 × 64 · f64 | — | No paired comparison |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f64 | 1.0118906064209274× [0.7700320512820513, 1.256198347107438] | Timing direction unclear |
| Matmul · 2 workers · reused output · 256 × 256 · 256 × 256 · f64 | 0.9625962596259626× [0.6264762427904422, 1.0512041150339022] | Timing direction unclear |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f32 | 1× [0.9807383627608346, 1.25705329153605] | Timing direction unclear |
| Cholesky solve · reused factor · 128 × 128, 1 RHS · f64 | 1× [0.978494623655914, 1] | Timing direction unclear |
| Cholesky solve · reused factor · 256 × 256, 64 RHS · f64 | 1.0877385048482953× [1.0835212691500116, 1.0905533380292713] | Observed time increase |
| Cholesky solve · reused factor · 512 × 512, 128 RHS · f64 | — | No paired comparison |
| LU solve · reused factor · 256 × 256, 64 RHS · f64 | 0.980936403843221× [0.9784331656759641, 0.9852986217457886] | Observed time reduction |
| Cholesky rank-one update · 256 × 256 · f64 | — | No paired comparison |
| Cholesky rank-one downdate · 256 × 256 · f64 | — | No paired comparison |
| Cholesky rank-one update · 512 × 512 · f64 | — | No paired comparison |
| Thin QR · 512 × 128 · f64 | 0.9980635263938836× [0.9794909809735605, 1.004127710364034] | Timing direction unclear |
| Thin QR · 1024 × 256 · f64 | 0.9930393565603598× [0.9869423490620097, 1.0008656185663485] | Timing direction unclear |
| Cholesky factorization · 512 × 512 · f64 | 0.999373094965304× [0.9815937955664504, 1.0053508504807065] | Timing direction unclear |
| Column-pivoted QR · 96 × 32 · f64 | — | No paired comparison |
| Column-pivoted QR · 256 × 64 · f64 | — | No paired comparison |
| Minimum-norm least squares · 64 × 32, rank 16, 4 RHS · f64 | — | No paired comparison |
| Minimum-norm least squares · 32 × 64, rank 32, 4 RHS · f64 | — | No paired comparison |
| Thin QR · 96 × 32 · f64 | — | No paired comparison |
| Thin QR · 256 × 64 · f64 | 1.0235950780159837× [0.9791957376633261, 1.0424158014687264] | Timing direction unclear |
| Cholesky factorization · 64 × 64 · f64 | — | No paired comparison |
| Cholesky factorization · 128 × 128 · f64 | 0.9961055819991346× [0.9587852494577006, 1.0378208014407924] | Timing direction unclear |
| Cholesky factorization · 256 × 256 · f64 | 0.9976703552708212× [0.9940622737146995, 1.0071553738317758] | Timing direction unclear |
| Shared-variable backward · sum(X * X), 64 × 32; prepared graph · f64 | — | No paired comparison |
| Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64 | 0.96× [0.94, 0.9795918367346939] | Observed time reduction |
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

Minimum-norm rows use pivoted QR plus scaled implicit RZ in Kernix; NumPy native lstsq; nalgebra native SVD/solve. Algorithm costs and rank policies differ. See original method for thresholds. Kernix/nalgebra: prepared base factor, owned modified factor; refactor: same Kernix version, full factorization of preformed modified matrix; base setup and matrix formation excluded Rank two; cyclic five diagonals; exact dyadic values; 1 worker; owned output; prepared operands except auto construction. Kernix Matrix finite checks vs peer IEEE policies; payload excludes capacity, metadata and allocator overhead. CSR indices usize in Kernix/faer and actual recorded SciPy width. SpGEMM heap merge vs SciPy column accumulator; fill-in depends on pattern. Construction: Kernix clones a Vec then validates/scans/compresses; SciPy borrows dense input and allocates CSR output; NumPy/dense Kernix copy only. Construction compares API trade-offs, not identical algorithms. No universal sparse speed or density threshold claim.

Numbers retain source precision; recorded digits do not imply equivalent physical measurement accuracy. Lowest observed medians are bold only when peers were measured. No aggregate winner or universal ranking is inferred.

[Original report and commentary](original-report.txt) · [Raw measurements](raw.json)

Measured source `7c11b2165b0cb7b523db4d938a36f1396396a138`. Run `runner-windows-latest-40289723446`. Rendering does not change measurements.
