# Kernix 0.13.0 · performance brief

Apple M1 (Virtual) / macOS

**Per-call median, microseconds per operation. Lower is faster. All recorded digits retained.**

51 workloads · 179 observations

Baseline: baseline-macos-latest-40289723446. Sequential run changes are descriptive; paired process evidence is shown separately.

[Interactive HTML (open locally)](report.html) · [Vector PDF](latency.pdf) · [Raw measurements](raw.json)

[![Performance brief preview](latency.png)](latency.pdf)

## Charts by workload

**Shorter bars mean less time.** Blue is Kernix; grey is a comparison implementation; teal is an explicitly selected Kernix backend. Entries are sorted by the recorded median, with a separate zero-based scale per chart. These point observations do not establish a statistical ranking.

Expand a group to see every chart here on GitHub. Click a chart to enlarge it. The exact table below keeps every source digit.

<details>
<summary>Sparse matrices & memory · 5 workloads</summary>

**Sparse matrix × vector · 2048 × 2048 · five diagonals · f64**

![Sparse matrix × vector · 2048 × 2048 · five diagonals · f64. Shorter bars mean less time.](charts/sparse_matvec_2048_f64.png)

Kernix: **15.583 µs**. Kernix took ≈ 129.4% more time than faer in this run.

**Sparse × dense matrix · 512 × 512 · 32 RHS · five diagonals · f64**

![Sparse × dense matrix · 512 × 512 · 32 RHS · five diagonals · f64. Shorter bars mean less time.](charts/sparse_matmul_512x32_f64.png)

Kernix: **29.167 µs**. Kernix took ≈ 38.3% more time than SciPy · CSR in this run.

**Sparse × dense matrix · 512 × 512 · 32 RHS · five diagonals · f32**

![Sparse × dense matrix · 512 × 512 · 32 RHS · five diagonals · f32. Shorter bars mean less time.](charts/sparse_matmul_512x32_f32.png)

Kernix: **42.583 µs**. Kernix took ≈ 208.8% more time than SciPy · CSR in this run.

**Sparse × sparse matrix · 256 × 256 · five diagonals each · f64**

![Sparse × sparse matrix · 256 × 256 · five diagonals each · f64. Shorter bars mean less time.](charts/sparse_product_256_f64.png)

Kernix: **61.29199999999999 µs**. Kernix took ≈ 36.3% more time than SciPy · CSR in this run.

**Automatic CSR construction · 512 × 512 · clone + scan + compress · f64**

![Automatic CSR construction · 512 × 512 · clone + scan + compress · f64. Shorter bars mean less time.](charts/sparse_auto_512_f64.png)

Kernix: **744.5 µs**. Construction trade-off: CSR compression versus dense copying; outputs use different storage. Compare payload as well as time.

</details>

<details>
<summary>Matrix multiplication · 13 workloads</summary>

**Matrix multiplication · transposed view · 128 × 128 · 128 × 128 · f64**

![Matrix multiplication · transposed view · 128 × 128 · 128 × 128 · f64. Shorter bars mean less time.](charts/matmul_transposed_128_f64.png)

Kernix: **112.625 µs**. Kernix took ≈ 657.1% more time than NumPy in this run.

**Matrix multiplication · transposed view · 128 × 128 · 128 × 128 · f32**

![Matrix multiplication · transposed view · 128 × 128 · 128 × 128 · f32. Shorter bars mean less time.](charts/matmul_transposed_128_f32.png)

Kernix: **51.416 µs**. Kernix took ≈ 879.4% more time than NumPy in this run.

**Independent matrix operands · 128 × 128 · 128 × 128 · f64**

![Independent matrix operands · 128 × 128 · 128 × 128 · f64. Shorter bars mean less time.](charts/matmul_ab_128_f64.png)

Kernix: **103.75 µs**. Kernix took ≈ 494.2% more time than NumPy in this run.

**Independent matrix operands · 128 × 128 · 128 × 128 · f32**

![Independent matrix operands · 128 × 128 · 128 × 128 · f32. Shorter bars mean less time.](charts/matmul_ab_128_f32.png)

Kernix: **54.917 µs**. Kernix took ≈ 821.6% more time than NumPy in this run.

**Matrix multiplication · 32 × 32 · 32 × 32 · f64**

![Matrix multiplication · 32 × 32 · 32 × 32 · f64. Shorter bars mean less time.](charts/matmul_32_f64.png)

Kernix: **2.25 µs**. Kernix took ≈ 10.2% more time than ndarray in this run.

**Matrix multiplication · 64 × 64 · 64 × 64 · f64**

![Matrix multiplication · 64 × 64 · 64 × 64 · f64. Shorter bars mean less time.](charts/matmul_64_f64.png)

Kernix: **14.084 µs**. Kernix took ≈ 328.0% more time than NumPy in this run.

**Matrix multiplication · 128 × 128 · 128 × 128 · f64**

![Matrix multiplication · 128 × 128 · 128 × 128 · f64. Shorter bars mean less time.](charts/matmul_128_f64.png)

Kernix: **102.25 µs**. Kernix took ≈ 491.3% more time than NumPy in this run.

**Matrix multiplication · 256 × 256 · 256 × 256 · f64**

![Matrix multiplication · 256 × 256 · 256 × 256 · f64. Shorter bars mean less time.](charts/matmul_256_f64.png)

Kernix: **811.75 µs**. Kernix took ≈ 551.4% more time than NumPy in this run.

**Matrix multiplication · 64 × 64 · 64 × 64 · f32**

![Matrix multiplication · 64 × 64 · 64 × 64 · f32. Shorter bars mean less time.](charts/matmul_64_f32.png)

Kernix: **7.25 µs**. Kernix took ≈ 31.8% more time than NumPy in this run.

**Matrix multiplication · 128 × 128 · 128 × 128 · f32**

![Matrix multiplication · 128 × 128 · 128 × 128 · f32. Shorter bars mean less time.](charts/matmul_128_f32.png)

Kernix: **57.542 µs**. Kernix took ≈ 859.0% more time than NumPy in this run.

**Matrix multiplication · 256 × 256 · 256 × 256 · f32**

![Matrix multiplication · 256 × 256 · 256 × 256 · f32. Shorter bars mean less time.](charts/matmul_256_f32.png)

Kernix: **416 µs**. Kernix took ≈ 1184.9% more time than NumPy in this run.

**Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64**

![Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64. Shorter bars mean less time.](charts/matmul_batched_4x16x32x16_f64.png)

Kernix: **2.833 µs**. Kernix took ≈ 38.7% more time than NumPy in this run.

**Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f32**

![Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f32. Shorter bars mean less time.](charts/matmul_batched_4x16x32x16_f32.png)

Kernix: **1.75 µs**. Kernix took ≈ 20.7% less time than NumPy in this run.

</details>

<details>
<summary>Explicit parallel execution · 4 workloads</summary>

**Matmul · 2 workers · reused output · 64 × 64 · 64 × 64 · f64**

![Matmul · 2 workers · reused output · 64 × 64 · 64 × 64 · f64. Shorter bars mean less time.](charts/parallel_matmul_into_64_f64.png)

Kernix: **22.708 µs**. No external comparison with Kernix was measured for this workload.

**Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f64**

![Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f64. Shorter bars mean less time.](charts/parallel_matmul_into_128_f64.png)

Kernix: **146.875 µs**. No external comparison with Kernix was measured for this workload.

**Matmul · 2 workers · reused output · 256 × 256 · 256 × 256 · f64**

![Matmul · 2 workers · reused output · 256 × 256 · 256 × 256 · f64. Shorter bars mean less time.](charts/parallel_matmul_into_256_f64.png)

Kernix: **747.042 µs**. No external comparison with Kernix was measured for this workload.

**Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f32**

![Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f32. Shorter bars mean less time.](charts/parallel_matmul_into_128_f32.png)

Kernix: **55.917 µs**. No external comparison with Kernix was measured for this workload.

</details>

<details>
<summary>Decompositions & solvers · 19 workloads</summary>

**Cholesky solve · reused factor · 128 × 128, 1 RHS · f64**

![Cholesky solve · reused factor · 128 × 128, 1 RHS · f64. Shorter bars mean less time.](charts/solve_cholesky_128x1_f64.png)

Kernix: **15.709000000000002 µs**. Kernix took ≈ 233.7% more time than nalgebra in this run.

**Cholesky solve · reused factor · 256 × 256, 64 RHS · f64**

![Cholesky solve · reused factor · 256 × 256, 64 RHS · f64. Shorter bars mean less time.](charts/solve_cholesky_256x64_f64.png)

Kernix: **1265.292 µs**. Kernix took ≈ 275.5% more time than faer in this run.

**Cholesky solve · reused factor · 512 × 512, 128 RHS · f64**

![Cholesky solve · reused factor · 512 × 512, 128 RHS · f64. Shorter bars mean less time.](charts/solve_cholesky_512x128_f64.png)

Kernix: **11043.792 µs**. Kernix took ≈ 342.5% more time than faer in this run.

**LU solve · reused factor · 256 × 256, 64 RHS · f64**

![LU solve · reused factor · 256 × 256, 64 RHS · f64. Shorter bars mean less time.](charts/solve_lu_256x64_f64.png)

Kernix: **1430.416 µs**. Kernix took ≈ 342.6% more time than faer in this run.

**Cholesky rank-one update · 256 × 256 · f64**

![Cholesky rank-one update · 256 × 256 · f64. Shorter bars mean less time.](charts/cholesky_rank_update_256_f64.png)

Kernix: **118.75 µs**. Kernix took ≈ 393.1% more time than nalgebra in this run.

**Cholesky rank-one downdate · 256 × 256 · f64**

![Cholesky rank-one downdate · 256 × 256 · f64. Shorter bars mean less time.](charts/cholesky_rank_downdate_256_f64.png)

Kernix: **117.584 µs**. Kernix took ≈ 366.5% more time than nalgebra in this run.

**Cholesky rank-one update · 512 × 512 · f64**

![Cholesky rank-one update · 512 × 512 · f64. Shorter bars mean less time.](charts/cholesky_rank_update_512_f64.png)

Kernix: **510.12500000000006 µs**. Kernix took ≈ 396.7% more time than nalgebra in this run.

**Thin QR · 512 × 128 · f64**

![Thin QR · 512 × 128 · f64. Shorter bars mean less time.](charts/qr_512x128_f64.png)

Kernix: **2751.375 µs**. Kernix took ≈ 38.8% more time than faer in this run.

**Thin QR · 1024 × 256 · f64**

![Thin QR · 1024 × 256 · f64. Shorter bars mean less time.](charts/qr_1024x256_f64.png)

Kernix: **15756.625 µs**. Kernix took ≈ 25.0% more time than NumPy in this run.

**Cholesky factorization · 512 × 512 · f64**

![Cholesky factorization · 512 × 512 · f64. Shorter bars mean less time.](charts/cholesky_factor_512_f64.png)

Kernix: **3560.667 µs**. Kernix took ≈ 69.1% more time than faer in this run.

**Column-pivoted QR · 96 × 32 · f64**

![Column-pivoted QR · 96 × 32 · f64. Shorter bars mean less time.](charts/pivoted_qr_96x32_f64.png)

Kernix: **75.875 µs**. Kernix took ≈ 74.4% more time than faer in this run.

**Column-pivoted QR · 256 × 64 · f64**

![Column-pivoted QR · 256 × 64 · f64. Shorter bars mean less time.](charts/pivoted_qr_256x64_f64.png)

Kernix: **610.167 µs**. Kernix took ≈ 89.1% more time than faer in this run.

**Minimum-norm least squares · 64 × 32, rank 16, 4 RHS · f64**

![Minimum-norm least squares · 64 × 32, rank 16, 4 RHS · f64. Shorter bars mean less time.](charts/min_norm_64x32_rank16_f64.png)

Kernix: **56.417 µs**. Kernix took ≈ 25.5% less time than nalgebra in this run.

**Minimum-norm least squares · 32 × 64, rank 32, 4 RHS · f64**

![Minimum-norm least squares · 32 × 64, rank 32, 4 RHS · f64. Shorter bars mean less time.](charts/min_norm_32x64_rank32_f64.png)

Kernix: **64.959 µs**. Kernix took ≈ 57.5% less time than nalgebra in this run.

**Thin QR · 96 × 32 · f64**

![Thin QR · 96 × 32 · f64. Shorter bars mean less time.](charts/qr_96x32_f64.png)

Kernix: **63.20799999999999 µs**. Kernix took ≈ 52.5% more time than nalgebra in this run.

**Thin QR · 256 × 64 · f64**

![Thin QR · 256 × 64 · f64. Shorter bars mean less time.](charts/qr_256x64_f64.png)

Kernix: **575.75 µs**. Kernix took ≈ 84.0% more time than faer in this run.

**Cholesky factorization · 64 × 64 · f64**

![Cholesky factorization · 64 × 64 · f64. Shorter bars mean less time.](charts/cholesky_factor_64_f64.png)

Kernix: **21.834 µs**. Kernix took ≈ 60.7% more time than faer in this run.

**Cholesky factorization · 128 × 128 · f64**

![Cholesky factorization · 128 × 128 · f64. Shorter bars mean less time.](charts/cholesky_factor_128_f64.png)

Kernix: **192.625 µs**. Kernix took ≈ 290.5% more time than faer in this run.

**Cholesky factorization · 256 × 256 · f64**

![Cholesky factorization · 256 × 256 · f64. Shorter bars mean less time.](charts/cholesky_factor_256_f64.png)

Kernix: **707.458 µs**. Kernix took ≈ 158.4% more time than faer in this run.

</details>

<details>
<summary>Training & autodiff · 6 workloads</summary>

**Shared-variable backward · sum(X * X), 64 × 32; prepared graph · f64**

![Shared-variable backward · sum(X * X), 64 × 32; prepared graph · f64. Shorter bars mean less time.](charts/training_shared_backward_64x32_f64.png)

Kernix: **6 µs**. Kernix took ≈ 33.3% more time than Candle in this run.

**Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64**

![Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64. Shorter bars mean less time.](charts/matmul_sum_backward_32x16x8_f64.png)

Kernix: **3.792 µs**. Kernix took ≈ 45.2% less time than Candle in this run.

**Model inference · batched normalized/sliced MLP · f64**

![Model inference · batched normalized/sliced MLP · f64. Shorter bars mean less time.](charts/training_forward_2x4x8x4_f64.png)

Kernix: **2.25 µs**. Kernix took ≈ 61.2% less time than Candle in this run.

**Model recording + forward · batched normalized/sliced MLP · f64**

![Model recording + forward · batched normalized/sliced MLP · f64. Shorter bars mean less time.](charts/training_record_2x4x8x4_f64.png)

Kernix: **3.166 µs**. Kernix took ≈ 50.0% less time than Candle in this run.

**Model backward · prepared graph → fresh gradients · f64**

![Model backward · prepared graph → fresh gradients · f64. Shorter bars mean less time.](charts/training_backward_2x4x8x4_f64.png)

Kernix: **3.375 µs**. Kernix took ≈ 85.5% less time than Candle in this run.

**Parameter update · transactional SGD, fixed gradients · f64**

![Parameter update · transactional SGD, fixed gradients · f64. Shorter bars mean less time.](charts/training_sgd_2x4x8x4_f64.png)

Kernix: **0.75 µs**. No external comparison with Kernix was measured for this workload.

</details>

<details>
<summary>Tensor contractions · 1 workloads</summary>

**Coalesced tensor contraction · 16 × 32 × 8 · 32 × 8 × 16 · f64**

![Coalesced tensor contraction · 16 × 32 × 8 · 32 × 8 × 16 · f64. Shorter bars mean less time.](charts/tensordot_16x32x8_32x8x16_f64.png)

Kernix: **74.708 µs**. Kernix took ≈ 1218.3% more time than NumPy in this run.

</details>

<details>
<summary>Statistics & activations · 1 workloads</summary>

**Row softmax · 256 × 64 · f64**

![Row softmax · 256 × 64 · f64. Shorter bars mean less time.](charts/softmax_256x64_f64.png)

Kernix: **94.459 µs**. Kernix took ≈ 48.5% more time than ndarray in this run.

</details>

<details>
<summary>Tensor construction · 2 workloads</summary>

**Create zeros · 64 × 64 · f32**

![Create zeros · 64 × 64 · f32. Shorter bars mean less time.](charts/create_zeros_64x64_f32.png)

Kernix: **0.292 µs**. Kernix took ≈ 0.3% more time than faer in this run.

**Create ones · 64 × 64 · f32**

![Create ones · 64 × 64 · f32. Shorter bars mean less time.](charts/create_ones_64x64_f32.png)

Kernix: **0.292 µs**. Kernix took ≈ 22.1% less time than ndarray in this run.

</details>

## Exact latency table

All values below are microseconds per operation. **Lower is faster.** Missing entries were not measured.

| Workload | Kernix | Kernix · explicit Accelerate | ndarray | nalgebra | faer | Candle | NumPy | Kernix refactor | Kernix · explicit dense | SciPy · CSR | Sequential run change |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| Sparse matrix × vector · 2048 × 2048 · five diagonals · f64 | 15.583 | — | — | — | **6.792** | — | 1356.042 | — | 3004.417 | 8.25 | — |
| Sparse × dense matrix · 512 × 512 · 32 RHS · five diagonals · f64 | 29.167 | — | — | — | 54.167 | — | 87.084 | — | 440.417 | **21.083** | — |
| Sparse × dense matrix · 512 × 512 · 32 RHS · five diagonals · f32 | 42.583 | — | — | — | 49.584 | — | 31.458 | — | 262.625 | **13.792** | — |
| Sparse × sparse matrix · 256 × 256 · five diagonals each · f64 | 61.29199999999999 | — | — | — | — | — | 129.208 | — | 827.416 | **44.958** | — |
| Automatic CSR construction · 512 × 512 · clone + scan + compress · f64 | 744.5 | — | — | — | — | — | 180.291 | — | **53.167** | 964.292 | — |
| Matrix multiplication · transposed view · 128 × 128 · 128 × 128 · f64 | 112.625 | — | — | — | — | — | **14.875** | — | — | — | +9.7% |
| Matrix multiplication · transposed view · 128 × 128 · 128 × 128 · f32 | 51.416 | — | — | — | — | — | **5.25** | — | — | — | +0.7% |
| Independent matrix operands · 128 × 128 · 128 × 128 · f64 | 103.75 | — | — | — | — | — | **17.459** | — | — | — | +1.8% |
| Independent matrix operands · 128 × 128 · 128 × 128 · f32 | 54.917 | — | — | — | — | — | **5.959** | — | — | — | +7.7% |
| Matrix multiplication · 32 × 32 · 32 × 32 · f64 | 2.25 | — | **2.041** | 2.125 | 2.5 | 2.708 | 4.166 | — | — | — | +0.0% |
| Matrix multiplication · 64 × 64 · 64 × 64 · f64 | 14.084 | — | 13.416 | 13.542 | 14.791 | 14.584 | **3.291** | — | — | — | -1.7% |
| Matrix multiplication · 128 × 128 · 128 × 128 · f64 | 102.25 | 17.916 | 106.583 | 111.708 | 108.25 | 101.417 | **17.292** | — | — | — | -5.6% |
| Matrix multiplication · 256 × 256 · 256 × 256 · f64 | 811.75 | **120.834** | 784.667 | 926.958 | 811.208 | 889.875 | 124.625 | — | — | — | +4.5% |
| Matrix multiplication · 64 × 64 · 64 × 64 · f32 | 7.25 | — | 7.042 | 6.917 | 8.167 | 7.916 | **5.5** | — | — | — | +0.6% |
| Matrix multiplication · 128 × 128 · 128 × 128 · f32 | 57.542 | 6.458 | 51.583 | 56.916 | 55.291 | 52.667 | **6** | — | — | — | +9.7% |
| Matrix multiplication · 256 × 256 · 256 × 256 · f32 | 416 | 35.583 | 432.208 | 388.042 | 427.667 | 390.541 | **32.375** | — | — | — | +6.8% |
| Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64 | 2.833 | — | — | — | — | 4.666 | **2.042** | — | — | — | +0.0% |
| Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f32 | **1.75** | — | — | — | — | 2.792 | 2.208 | — | — | — | -4.6% |
| Matmul · 2 workers · reused output · 64 × 64 · 64 × 64 · f64 | 22.708 | — | — | — | — | — | — | — | — | — | +8.6% |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f64 | 146.875 | — | — | — | — | — | — | — | — | — | +16.8% |
| Matmul · 2 workers · reused output · 256 × 256 · 256 × 256 · f64 | 747.042 | — | — | — | — | — | — | — | — | — | +52.5% |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f32 | 55.917 | — | — | — | — | — | — | — | — | — | -2.3% |
| Cholesky solve · reused factor · 128 × 128, 1 RHS · f64 | 15.709000000000002 | — | — | **4.708** | 4.75 | — | — | — | — | — | +0.0% |
| Cholesky solve · reused factor · 256 × 256, 64 RHS · f64 | 1265.292 | — | — | 974.459 | **337** | — | — | — | — | — | -5.8% |
| Cholesky solve · reused factor · 512 × 512, 128 RHS · f64 | 11043.792 | — | — | 7446.625 | **2496** | — | — | — | — | — | +13.8% |
| LU solve · reused factor · 256 × 256, 64 RHS · f64 | 1430.416 | — | — | 950.125 | **323.166** | — | — | — | — | — | +24.5% |
| Cholesky rank-one update · 256 × 256 · f64 | 118.75 | — | — | **24.083** | — | — | — | 639.333 | — | — | -0.9% |
| Cholesky rank-one downdate · 256 × 256 · f64 | 117.584 | — | — | **25.208** | — | — | — | 649.833 | — | — | +3.0% |
| Cholesky rank-one update · 512 × 512 · f64 | 510.12500000000006 | — | — | **102.709** | — | — | — | 3539.25 | — | — | +9.6% |
| Thin QR · 512 × 128 · f64 | 2751.375 | — | — | 3003.75 | **1981.9159999999998** | — | 2461.375 | — | — | — | +5.0% |
| Thin QR · 1024 × 256 · f64 | 15756.625 | — | — | 25029.375 | 12839.417 | — | **12603.334** | — | — | — | +8.4% |
| Cholesky factorization · 512 × 512 · f64 | 3560.667 | — | — | 5559.25 | **2105.209** | — | 2385.625 | — | — | — | -0.3% |
| Column-pivoted QR · 96 × 32 · f64 | 75.875 | — | — | 108.5 | **43.5** | — | — | — | — | — | +13.2% |
| Column-pivoted QR · 256 × 64 · f64 | 610.167 | — | — | 1071.417 | **322.584** | — | — | — | — | — | +12.4% |
| Minimum-norm least squares · 64 × 32, rank 16, 4 RHS · f64 | **56.417** | — | — | 75.75 | — | — | 99.5 | — | — | — | +19.3% |
| Minimum-norm least squares · 32 × 64, rank 32, 4 RHS · f64 | **64.959** | — | — | 152.75 | — | — | 159.125 | — | — | — | -0.4% |
| Thin QR · 96 × 32 · f64 | 63.20799999999999 | — | — | **41.459** | 47.167 | — | 50.542 | — | — | — | +8.4% |
| Thin QR · 256 × 64 · f64 | 575.75 | — | — | 357.542 | **312.917** | — | 394.708 | — | — | — | +4.9% |
| Cholesky factorization · 64 × 64 · f64 | 21.834 | — | — | 22.042 | **13.583** | — | 16.042 | — | — | — | -0.4% |
| Cholesky factorization · 128 × 128 · f64 | 192.625 | — | — | 127.334 | **49.333** | — | 74.625 | — | — | — | +12.2% |
| Cholesky factorization · 256 × 256 · f64 | 707.458 | — | — | 783.084 | **273.833** | — | 319.458 | — | — | — | +14.7% |
| Shared-variable backward · sum(X * X), 64 × 32; prepared graph · f64 | 6 | — | — | — | — | **4.5** | — | — | — | — | +0.0% |
| Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64 | **3.792** | — | — | — | — | 6.917 | — | — | — | — | +16.7% |
| Model inference · batched normalized/sliced MLP · f64 | **2.25** | — | — | — | — | 5.792 | — | — | — | — | -1.8% |
| Model recording + forward · batched normalized/sliced MLP · f64 | **3.166** | — | — | — | — | 6.334 | — | — | — | — | -3.8% |
| Model backward · prepared graph → fresh gradients · f64 | **3.375** | — | — | — | — | 23.209 | — | — | — | — | -1.2% |
| Parameter update · transactional SGD, fixed gradients · f64 | 0.75 | — | — | — | — | — | — | — | — | — | +0.0% |
| Coalesced tensor contraction · 16 × 32 × 8 · 32 × 8 × 16 · f64 | 74.708 | — | — | — | — | — | **5.667** | — | — | — | +20.7% |
| Row softmax · 256 × 64 · f64 | 94.459 | — | **63.625** | — | — | 72.416 | 75.625 | — | — | — | +12.8% |
| Create zeros · 64 × 64 · f32 | 0.292 | — | 0.334 | — | **0.291** | 0.542 | 0.5 | — | — | — | -22.1% |
| Create ones · 64 × 64 · f32 | **0.292** | — | 0.375 | — | 0.416 | 1 | 1.25 | — | — | — | -12.3% |

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
| Sparse × dense matrix · 512 × 512 · 32 RHS · five diagonals · f64 | Kernix | 2560 | 45064 | 2097152 | 8 |
| Sparse × dense matrix · 512 × 512 · 32 RHS · five diagonals · f64 | NumPy | 2560 | 2097152 | 2097152 | 0 |
| Sparse × dense matrix · 512 × 512 · 32 RHS · five diagonals · f64 | faer | 2560 | 45064 | 2097152 | 8 |
| Sparse × dense matrix · 512 × 512 · 32 RHS · five diagonals · f32 | Kernix | 2560 | 34824 | 1048576 | 8 |
| Sparse × dense matrix · 512 × 512 · 32 RHS · five diagonals · f32 | SciPy · CSR | 2560 | 22532 | 1048576 | 4 |
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
| Matrix multiplication · transposed view · 128 × 128 · 128 × 128 · f64 | 1.0408798988621997× [0.9586351199098664, 1.0546903947744524] | Timing direction unclear |
| Matrix multiplication · transposed view · 128 × 128 · 128 × 128 · f32 | 0.9951534420254735× [0.961185853240413, 1.0564999190545572] | Timing direction unclear |
| Independent matrix operands · 128 × 128 · 128 × 128 · f64 | 1× [0.8936943272386311, 1.0455315505683473] | Timing direction unclear |
| Independent matrix operands · 128 × 128 · 128 × 128 · f32 | 0.9991960784313726× [0.9549636566040207, 1.0070588235294118] | Timing direction unclear |
| Matrix multiplication · 32 × 32 · 32 × 32 · f64 | 0.9821038847664775× [0.945438673068529, 1] | Timing direction unclear |
| Matrix multiplication · 64 × 64 · 64 × 64 · f64 | — | No paired comparison |
| Matrix multiplication · 128 × 128 · 128 × 128 · f64 | 1.00001011807797× [0.9388606238475425, 1.0940139622787055] | Timing direction unclear |
| Matrix multiplication · 256 × 256 · 256 × 256 · f64 | 0.9962362276819777× [0.9468969486462286, 1.0104691648198838] | Timing direction unclear |
| Matrix multiplication · 64 × 64 · 64 × 64 · f32 | — | No paired comparison |
| Matrix multiplication · 128 × 128 · 128 × 128 · f32 | 0.9991894510013245× [0.9028559547324492, 1.0428752324799178] | Timing direction unclear |
| Matrix multiplication · 256 × 256 · 256 × 256 · f32 | 1.0213066491832494× [0.8922478299910207, 1.1183833501174891] | Timing direction unclear |
| Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64 | 0.9853099247581512× [0.9565217391304348, 1] | Timing direction unclear |
| Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f32 | 1.0474285714285714× [0.9541984732824428, 1.158669574700109] | Timing direction unclear |
| Matmul · 2 workers · reused output · 64 × 64 · 64 × 64 · f64 | — | No paired comparison |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f64 | 0.9864052621133509× [0.4546756756756757, 1.1396162361623616] | Timing direction unclear |
| Matmul · 2 workers · reused output · 256 × 256 · 256 × 256 · f64 | 0.9985013499839345× [0.7590126712952051, 1.061062513641804] | Timing direction unclear |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f32 | 0.9885894736842106× [0.7937915742793792, 1.2318772378516625] | Timing direction unclear |
| Cholesky solve · reused factor · 128 × 128, 1 RHS · f64 | 0.9921669382128087× [0.9164682291013913, 1.0027768595041322] | Timing direction unclear |
| Cholesky solve · reused factor · 256 × 256, 64 RHS · f64 | 0.9885282882882883× [0.8586642621355661, 1.079443083541639] | Timing direction unclear |
| Cholesky solve · reused factor · 512 × 512, 128 RHS · f64 | — | No paired comparison |
| LU solve · reused factor · 256 × 256, 64 RHS · f64 | 1.209485633752933× [1.1546833718414344, 1.3779067180145486] | Observed time increase |
| Cholesky rank-one update · 256 × 256 · f64 | — | No paired comparison |
| Cholesky rank-one downdate · 256 × 256 · f64 | — | No paired comparison |
| Cholesky rank-one update · 512 × 512 · f64 | — | No paired comparison |
| Thin QR · 512 × 128 · f64 | 0.9936245424891716× [0.8976748411295378, 1.0586904929547531] | Timing direction unclear |
| Thin QR · 1024 × 256 · f64 | 1.0182715183366788× [0.9539091491294941, 1.0669591519324717] | Timing direction unclear |
| Cholesky factorization · 512 × 512 · f64 | 1.0001606038705533× [0.9693697400669682, 1.1191409331409332] | Timing direction unclear |
| Column-pivoted QR · 96 × 32 · f64 | — | No paired comparison |
| Column-pivoted QR · 256 × 64 · f64 | — | No paired comparison |
| Minimum-norm least squares · 64 × 32, rank 16, 4 RHS · f64 | — | No paired comparison |
| Minimum-norm least squares · 32 × 64, rank 32, 4 RHS · f64 | — | No paired comparison |
| Thin QR · 96 × 32 · f64 | — | No paired comparison |
| Thin QR · 256 × 64 · f64 | 0.9986932253431998× [0.9374925164989188, 1.0351285018554286] | Timing direction unclear |
| Cholesky factorization · 64 × 64 · f64 | — | No paired comparison |
| Cholesky factorization · 128 × 128 · f64 | 0.9675904789281748× [0.916102372139973, 1.0378457059679767] | Timing direction unclear |
| Cholesky factorization · 256 × 256 · f64 | 1.000398406374502× [0.9715976292854505, 1.0930526315789473] | Timing direction unclear |
| Shared-variable backward · sum(X * X), 64 × 32; prepared graph · f64 | — | No paired comparison |
| Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64 | 0.9727626459143969× [0.9375937593759376, 0.9869077306733167] | Observed time reduction |
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

Measured source `7c11b2165b0cb7b523db4d938a36f1396396a138`. Run `runner-macos-latest-40289723446`. Rendering does not change measurements.
