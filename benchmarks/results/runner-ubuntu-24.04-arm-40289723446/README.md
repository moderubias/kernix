# Kernix 0.13.0 · performance brief

Neoverse-N2 / Linux

**Per-call median, microseconds per operation. Lower is faster. All recorded digits retained.**

51 workloads · 175 observations

Baseline: baseline-ubuntu-24.04-arm-40289723446. Sequential run changes are descriptive; paired process evidence is shown separately.

[Interactive HTML (open locally)](report.html) · [Vector PDF](latency.pdf) · [Raw measurements](raw.json)

[![Performance brief preview](latency.png)](latency.pdf)

## Charts by workload

**Shorter bars mean less time.** Blue is Kernix; grey is a comparison implementation; teal is an explicitly selected Kernix backend. Entries are sorted by the recorded median, with a separate zero-based scale per chart. These point observations do not establish a statistical ranking.

Expand a group to see every chart here on GitHub. Click a chart to enlarge it. The exact table below keeps every source digit.

<details>
<summary>Sparse matrices & memory · 5 workloads</summary>

**Sparse matrix × vector · 2048 × 2048 · five diagonals · f64**

![Sparse matrix × vector · 2048 × 2048 · five diagonals · f64. Shorter bars mean less time.](charts/sparse_matvec_2048_f64.png)

Kernix: **18.142 µs**. Kernix took ≈ 126.3% more time than faer in this run.

**Sparse × dense matrix · 512 × 512 · 32 RHS · five diagonals · f64**

![Sparse × dense matrix · 512 × 512 · 32 RHS · five diagonals · f64. Shorter bars mean less time.](charts/sparse_matmul_512x32_f64.png)

Kernix: **30.454 µs**. Kernix took ≈ 32.1% more time than SciPy · CSR in this run.

**Sparse × dense matrix · 512 × 512 · 32 RHS · five diagonals · f32**

![Sparse × dense matrix · 512 × 512 · 32 RHS · five diagonals · f32. Shorter bars mean less time.](charts/sparse_matmul_512x32_f32.png)

Kernix: **21.079 µs**. Kernix took ≈ 37.4% more time than SciPy · CSR in this run.

**Sparse × sparse matrix · 256 × 256 · five diagonals each · f64**

![Sparse × sparse matrix · 256 × 256 · five diagonals each · f64. Shorter bars mean less time.](charts/sparse_product_256_f64.png)

Kernix: **64.33899999999999 µs**. Kernix took ≈ 36.8% more time than SciPy · CSR in this run.

**Automatic CSR construction · 512 × 512 · clone + scan + compress · f64**

![Automatic CSR construction · 512 × 512 · clone + scan + compress · f64. Shorter bars mean less time.](charts/sparse_auto_512_f64.png)

Kernix: **489.06000000000006 µs**. Construction trade-off: CSR compression versus dense copying; outputs use different storage. Compare payload as well as time.

</details>

<details>
<summary>Matrix multiplication · 13 workloads</summary>

**Matrix multiplication · transposed view · 128 × 128 · 128 × 128 · f64**

![Matrix multiplication · transposed view · 128 × 128 · 128 × 128 · f64. Shorter bars mean less time.](charts/matmul_transposed_128_f64.png)

Kernix: **214.161 µs**. Kernix took ≈ 9.6% more time than NumPy in this run.

**Matrix multiplication · transposed view · 128 × 128 · 128 × 128 · f32**

![Matrix multiplication · transposed view · 128 × 128 · 128 × 128 · f32. Shorter bars mean less time.](charts/matmul_transposed_128_f32.png)

Kernix: **93.163 µs**. Kernix took ≈ 7.0% less time than NumPy in this run.

**Independent matrix operands · 128 × 128 · 128 × 128 · f64**

![Independent matrix operands · 128 × 128 · 128 × 128 · f64. Shorter bars mean less time.](charts/matmul_ab_128_f64.png)

Kernix: **168.693 µs**. Kernix took ≈ 13.8% less time than NumPy in this run.

**Independent matrix operands · 128 × 128 · 128 × 128 · f32**

![Independent matrix operands · 128 × 128 · 128 × 128 · f32. Shorter bars mean less time.](charts/matmul_ab_128_f32.png)

Kernix: **85.468 µs**. Kernix took ≈ 14.6% less time than NumPy in this run.

**Matrix multiplication · 32 × 32 · 32 × 32 · f64**

![Matrix multiplication · 32 × 32 · 32 × 32 · f64. Shorter bars mean less time.](charts/matmul_32_f64.png)

Kernix: **3.248 µs**. Kernix took ≈ 6.8% more time than ndarray in this run.

**Matrix multiplication · 64 × 64 · 64 × 64 · f64**

![Matrix multiplication · 64 × 64 · 64 × 64 · f64. Shorter bars mean less time.](charts/matmul_64_f64.png)

Kernix: **22.182 µs**. Kernix took ≈ 1.8% more time than ndarray in this run.

**Matrix multiplication · 128 × 128 · 128 × 128 · f64**

![Matrix multiplication · 128 × 128 · 128 × 128 · f64. Shorter bars mean less time.](charts/matmul_128_f64.png)

Kernix: **168.406 µs**. Kernix took ≈ 0.1% more time than Candle in this run.

**Matrix multiplication · 256 × 256 · 256 × 256 · f64**

![Matrix multiplication · 256 × 256 · 256 × 256 · f64. Shorter bars mean less time.](charts/matmul_256_f64.png)

Kernix: **1323.383 µs**. Kernix took &lt; 0.1% less time than Candle in this run.

**Matrix multiplication · 64 × 64 · 64 × 64 · f32**

![Matrix multiplication · 64 × 64 · 64 × 64 · f32. Shorter bars mean less time.](charts/matmul_64_f32.png)

Kernix: **11.767 µs**. Kernix took ≈ 4.3% more time than ndarray in this run.

**Matrix multiplication · 128 × 128 · 128 × 128 · f32**

![Matrix multiplication · 128 × 128 · 128 × 128 · f32. Shorter bars mean less time.](charts/matmul_128_f32.png)

Kernix: **85.419 µs**. Kernix took ≈ 0.5% less time than Candle in this run.

**Matrix multiplication · 256 × 256 · 256 × 256 · f32**

![Matrix multiplication · 256 × 256 · 256 × 256 · f32. Shorter bars mean less time.](charts/matmul_256_f32.png)

Kernix: **663.041 µs**. Kernix took ≈ 1.7% less time than faer in this run.

**Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64**

![Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64. Shorter bars mean less time.](charts/matmul_batched_4x16x32x16_f64.png)

Kernix: **3.8310000000000005 µs**. Kernix took ≈ 13.1% less time than Candle in this run.

**Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f32**

![Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f32. Shorter bars mean less time.](charts/matmul_batched_4x16x32x16_f32.png)

Kernix: **2.4 µs**. Kernix took ≈ 16.4% less time than Candle in this run.

</details>

<details>
<summary>Explicit parallel execution · 4 workloads</summary>

**Matmul · 2 workers · reused output · 64 × 64 · 64 × 64 · f64**

![Matmul · 2 workers · reused output · 64 × 64 · 64 × 64 · f64. Shorter bars mean less time.](charts/parallel_matmul_into_64_f64.png)

Kernix: **16.448 µs**. No external comparison with Kernix was measured for this workload.

**Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f64**

![Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f64. Shorter bars mean less time.](charts/parallel_matmul_into_128_f64.png)

Kernix: **93.93 µs**. No external comparison with Kernix was measured for this workload.

**Matmul · 2 workers · reused output · 256 × 256 · 256 × 256 · f64**

![Matmul · 2 workers · reused output · 256 × 256 · 256 × 256 · f64. Shorter bars mean less time.](charts/parallel_matmul_into_256_f64.png)

Kernix: **692.783 µs**. No external comparison with Kernix was measured for this workload.

**Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f32**

![Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f32. Shorter bars mean less time.](charts/parallel_matmul_into_128_f32.png)

Kernix: **50.333 µs**. No external comparison with Kernix was measured for this workload.

</details>

<details>
<summary>Decompositions & solvers · 19 workloads</summary>

**Cholesky solve · reused factor · 128 × 128, 1 RHS · f64**

![Cholesky solve · reused factor · 128 × 128, 1 RHS · f64. Shorter bars mean less time.](charts/solve_cholesky_128x1_f64.png)

Kernix: **12.719 µs**. Kernix took ≈ 142.0% more time than nalgebra in this run.

**Cholesky solve · reused factor · 256 × 256, 64 RHS · f64**

![Cholesky solve · reused factor · 256 × 256, 64 RHS · f64. Shorter bars mean less time.](charts/solve_cholesky_256x64_f64.png)

Kernix: **1719.759 µs**. Kernix took ≈ 279.9% more time than faer in this run.

**Cholesky solve · reused factor · 512 × 512, 128 RHS · f64**

![Cholesky solve · reused factor · 512 × 512, 128 RHS · f64. Shorter bars mean less time.](charts/solve_cholesky_512x128_f64.png)

Kernix: **13394.223 µs**. Kernix took ≈ 319.4% more time than faer in this run.

**LU solve · reused factor · 256 × 256, 64 RHS · f64**

![LU solve · reused factor · 256 × 256, 64 RHS · f64. Shorter bars mean less time.](charts/solve_lu_256x64_f64.png)

Kernix: **1654.199 µs**. Kernix took ≈ 261.4% more time than faer in this run.

**Cholesky rank-one update · 256 × 256 · f64**

![Cholesky rank-one update · 256 × 256 · f64. Shorter bars mean less time.](charts/cholesky_rank_update_256_f64.png)

Kernix: **151.119 µs**. Kernix took ≈ 346.5% more time than nalgebra in this run.

**Cholesky rank-one downdate · 256 × 256 · f64**

![Cholesky rank-one downdate · 256 × 256 · f64. Shorter bars mean less time.](charts/cholesky_rank_downdate_256_f64.png)

Kernix: **150.517 µs**. Kernix took ≈ 348.4% more time than nalgebra in this run.

**Cholesky rank-one update · 512 × 512 · f64**

![Cholesky rank-one update · 512 × 512 · f64. Shorter bars mean less time.](charts/cholesky_rank_update_512_f64.png)

Kernix: **657.513 µs**. Kernix took ≈ 343.3% more time than nalgebra in this run.

**Thin QR · 512 × 128 · f64**

![Thin QR · 512 × 128 · f64. Shorter bars mean less time.](charts/qr_512x128_f64.png)

Kernix: **4162.684 µs**. Kernix took ≈ 33.7% more time than faer in this run.

**Thin QR · 1024 × 256 · f64**

![Thin QR · 1024 × 256 · f64. Shorter bars mean less time.](charts/qr_1024x256_f64.png)

Kernix: **24134.286 µs**. Kernix took ≈ 21.8% more time than NumPy in this run.

**Cholesky factorization · 512 × 512 · f64**

![Cholesky factorization · 512 × 512 · f64. Shorter bars mean less time.](charts/cholesky_factor_512_f64.png)

Kernix: **4336.307 µs**. Kernix took ≈ 64.7% more time than faer in this run.

**Column-pivoted QR · 96 × 32 · f64**

![Column-pivoted QR · 96 × 32 · f64. Shorter bars mean less time.](charts/pivoted_qr_96x32_f64.png)

Kernix: **88.163 µs**. Kernix took ≈ 41.6% more time than faer in this run.

**Column-pivoted QR · 256 × 64 · f64**

![Column-pivoted QR · 256 × 64 · f64. Shorter bars mean less time.](charts/pivoted_qr_256x64_f64.png)

Kernix: **782.192 µs**. Kernix took ≈ 65.0% more time than faer in this run.

**Minimum-norm least squares · 64 × 32, rank 16, 4 RHS · f64**

![Minimum-norm least squares · 64 × 32, rank 16, 4 RHS · f64. Shorter bars mean less time.](charts/min_norm_64x32_rank16_f64.png)

Kernix: **72.22 µs**. Kernix took ≈ 22.9% less time than NumPy in this run.

**Minimum-norm least squares · 32 × 64, rank 32, 4 RHS · f64**

![Minimum-norm least squares · 32 × 64, rank 32, 4 RHS · f64. Shorter bars mean less time.](charts/min_norm_32x64_rank32_f64.png)

Kernix: **96.54 µs**. Kernix took ≈ 45.2% less time than NumPy in this run.

**Thin QR · 96 × 32 · f64**

![Thin QR · 96 × 32 · f64. Shorter bars mean less time.](charts/qr_96x32_f64.png)

Kernix: **73.244 µs**. Kernix took ≈ 19.5% more time than nalgebra in this run.

**Thin QR · 256 × 64 · f64**

![Thin QR · 256 × 64 · f64. Shorter bars mean less time.](charts/qr_256x64_f64.png)

Kernix: **693.83 µs**. Kernix took ≈ 55.1% more time than faer in this run.

**Cholesky factorization · 64 × 64 · f64**

![Cholesky factorization · 64 × 64 · f64. Shorter bars mean less time.](charts/cholesky_factor_64_f64.png)

Kernix: **27.374 µs**. Kernix took ≈ 106.5% more time than faer in this run.

**Cholesky factorization · 128 × 128 · f64**

![Cholesky factorization · 128 × 128 · f64. Shorter bars mean less time.](charts/cholesky_factor_128_f64.png)

Kernix: **277.143 µs**. Kernix took ≈ 324.6% more time than faer in this run.

**Cholesky factorization · 256 × 256 · f64**

![Cholesky factorization · 256 × 256 · f64. Shorter bars mean less time.](charts/cholesky_factor_256_f64.png)

Kernix: **769.282 µs**. Kernix took ≈ 90.1% more time than faer in this run.

</details>

<details>
<summary>Training & autodiff · 6 workloads</summary>

**Shared-variable backward · sum(X * X), 64 × 32; prepared graph · f64**

![Shared-variable backward · sum(X * X), 64 × 32; prepared graph · f64. Shorter bars mean less time.](charts/training_shared_backward_64x32_f64.png)

Kernix: **8.368 µs**. Kernix took ≈ 67.9% more time than Candle in this run.

**Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64**

![Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64. Shorter bars mean less time.](charts/matmul_sum_backward_32x16x8_f64.png)

Kernix: **3.536 µs**. Kernix took ≈ 51.3% less time than Candle in this run.

**Model inference · batched normalized/sliced MLP · f64**

![Model inference · batched normalized/sliced MLP · f64. Shorter bars mean less time.](charts/training_forward_2x4x8x4_f64.png)

Kernix: **3.031 µs**. Kernix took ≈ 43.2% less time than Candle in this run.

**Model recording + forward · batched normalized/sliced MLP · f64**

![Model recording + forward · batched normalized/sliced MLP · f64. Shorter bars mean less time.](charts/training_record_2x4x8x4_f64.png)

Kernix: **3.856 µs**. Kernix took ≈ 49.3% less time than Candle in this run.

**Model backward · prepared graph → fresh gradients · f64**

![Model backward · prepared graph → fresh gradients · f64. Shorter bars mean less time.](charts/training_backward_2x4x8x4_f64.png)

Kernix: **4.2 µs**. Kernix took ≈ 80.2% less time than Candle in this run.

**Parameter update · transactional SGD, fixed gradients · f64**

![Parameter update · transactional SGD, fixed gradients · f64. Shorter bars mean less time.](charts/training_sgd_2x4x8x4_f64.png)

Kernix: **0.728 µs**. No external comparison with Kernix was measured for this workload.

</details>

<details>
<summary>Tensor contractions · 1 workloads</summary>

**Coalesced tensor contraction · 16 × 32 × 8 · 32 × 8 × 16 · f64**

![Coalesced tensor contraction · 16 × 32 × 8 · 32 × 8 × 16 · f64. Shorter bars mean less time.](charts/tensordot_16x32x8_32x8x16_f64.png)

Kernix: **46.718 µs**. Kernix took ≈ 184.6% more time than NumPy in this run.

</details>

<details>
<summary>Statistics & activations · 1 workloads</summary>

**Row softmax · 256 × 64 · f64**

![Row softmax · 256 × 64 · f64. Shorter bars mean less time.](charts/softmax_256x64_f64.png)

Kernix: **113.009 µs**. Kernix took ≈ 16.2% more time than ndarray in this run.

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

| Workload | Kernix | ndarray | nalgebra | faer | Candle | NumPy | Kernix refactor | Kernix · explicit dense | SciPy · CSR | Sequential run change |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| Sparse matrix × vector · 2048 × 2048 · five diagonals · f64 | 18.142 | — | — | **8.016000000000001** | — | 805.888 | — | 2914.433 | 9.76 | — |
| Sparse × dense matrix · 512 × 512 · 32 RHS · five diagonals · f64 | 30.454 | — | — | 68.052 | — | 1064.764 | — | 783.657 | **23.047** | — |
| Sparse × dense matrix · 512 × 512 · 32 RHS · five diagonals · f32 | 21.079 | — | — | 64.019 | — | 512.47 | — | 380.793 | **15.343** | — |
| Sparse × sparse matrix · 256 × 256 · five diagonals each · f64 | 64.33899999999999 | — | — | — | — | 1475.794 | — | 1324.545 | **47.037** | — |
| Automatic CSR construction · 512 × 512 · clone + scan + compress · f64 | 489.06000000000006 | — | — | — | — | 55.813 | — | **53.397** | 752.028 | — |
| Matrix multiplication · transposed view · 128 × 128 · 128 × 128 · f64 | 214.161 | — | — | — | — | **195.461** | — | — | — | +1.7% |
| Matrix multiplication · transposed view · 128 × 128 · 128 × 128 · f32 | **93.163** | — | — | — | — | 100.17 | — | — | — | +0.5% |
| Independent matrix operands · 128 × 128 · 128 × 128 · f64 | **168.693** | — | — | — | — | 195.731 | — | — | — | +0.2% |
| Independent matrix operands · 128 × 128 · 128 × 128 · f32 | **85.468** | — | — | — | — | 100.042 | — | — | — | -0.2% |
| Matrix multiplication · 32 × 32 · 32 × 32 · f64 | 3.248 | **3.04** | 3.208 | 3.496 | 3.44 | 8.216 | — | — | — | +0.2% |
| Matrix multiplication · 64 × 64 · 64 × 64 · f64 | 22.182 | **21.799** | 22.463 | 22.639 | 22.558 | 31.086 | — | — | — | +0.1% |
| Matrix multiplication · 128 × 128 · 128 × 128 · f64 | 168.406 | 209.087 | 209.276 | 169.638 | **168.223** | 196.046 | — | — | — | +0.1% |
| Matrix multiplication · 256 × 256 · 256 × 256 · f64 | **1323.383** | 1324.634 | 1331.586 | 1543.694 | 1324.601 | 1470.338 | — | — | — | +0.0% |
| Matrix multiplication · 64 × 64 · 64 × 64 · f32 | 11.767 | **11.279** | **11.279** | 12.295 | 11.959 | 20.119 | — | — | — | +0.0% |
| Matrix multiplication · 128 × 128 · 128 × 128 · f32 | **85.419** | 93.987 | 93.604 | 86.907 | 85.867 | 99.849 | — | — | — | -0.3% |
| Matrix multiplication · 256 × 256 · 256 × 256 · f32 | **663.041** | 763.872 | 762.699 | 674.769 | 758.614 | 722.714 | — | — | — | +0.1% |
| Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64 | **3.8310000000000005** | — | — | — | 4.408 | 12.999 | — | — | — | +0.2% |
| Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f32 | **2.4** | — | — | — | 2.872 | 5.151 | — | — | — | +0.3% |
| Matmul · 2 workers · reused output · 64 × 64 · 64 × 64 · f64 | 16.448 | — | — | — | — | — | — | — | — | +0.1% |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f64 | 93.93 | — | — | — | — | — | — | — | — | +0.8% |
| Matmul · 2 workers · reused output · 256 × 256 · 256 × 256 · f64 | 692.783 | — | — | — | — | — | — | — | — | -0.4% |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f32 | 50.333 | — | — | — | — | — | — | — | — | +3.7% |
| Cholesky solve · reused factor · 128 × 128, 1 RHS · f64 | 12.719 | — | **5.256** | 5.887 | — | — | — | — | — | +0.1% |
| Cholesky solve · reused factor · 256 × 256, 64 RHS · f64 | 1719.759 | — | 1286.693 | **452.655** | — | — | — | — | — | +0.1% |
| Cholesky solve · reused factor · 512 × 512, 128 RHS · f64 | 13394.223 | — | 12850.165 | **3193.317** | — | — | — | — | — | -1.1% |
| LU solve · reused factor · 256 × 256, 64 RHS · f64 | 1654.199 | — | 1234.129 | **457.732** | — | — | — | — | — | +0.0% |
| Cholesky rank-one update · 256 × 256 · f64 | 151.119 | — | **33.846** | — | — | — | 764.425 | — | — | -0.5% |
| Cholesky rank-one downdate · 256 × 256 · f64 | 150.517 | — | **33.566** | — | — | — | 764.447 | — | — | +0.3% |
| Cholesky rank-one update · 512 × 512 · f64 | 657.513 | — | **148.319** | — | — | — | 4356.637 | — | — | +0.9% |
| Thin QR · 512 × 128 · f64 | 4162.684 | — | 4646.439 | **3113.54** | — | 4122.093 | — | — | — | +0.5% |
| Thin QR · 1024 × 256 · f64 | 24134.286 | — | 36091.08 | 20229.733 | — | **19809.741** | — | — | — | -0.8% |
| Cholesky factorization · 512 × 512 · f64 | 4336.307 | — | 8489.056 | **2633.094** | — | 3859.118 | — | — | — | +0.0% |
| Column-pivoted QR · 96 × 32 · f64 | 88.163 | — | 114.211 | **62.251** | — | — | — | — | — | -0.1% |
| Column-pivoted QR · 256 × 64 · f64 | 782.192 | — | 1151.266 | **473.945** | — | — | — | — | — | -0.5% |
| Minimum-norm least squares · 64 × 32, rank 16, 4 RHS · f64 | **72.22** | — | 117.265 | — | — | 93.675 | — | — | — | -0.2% |
| Minimum-norm least squares · 32 × 64, rank 32, 4 RHS · f64 | **96.54** | — | 218.379 | — | — | 176.124 | — | — | — | +0.1% |
| Thin QR · 96 × 32 · f64 | 73.244 | — | **61.29199999999999** | 65.53000000000001 | — | 66.685 | — | — | — | -0.1% |
| Thin QR · 256 × 64 · f64 | 693.83 | — | 585.304 | **447.474** | — | 565.441 | — | — | — | +0.2% |
| Cholesky factorization · 64 × 64 · f64 | 27.374 | — | 20.391 | **13.256** | — | 17.671 | — | — | — | +0.2% |
| Cholesky factorization · 128 × 128 · f64 | 277.143 | — | 128.952 | **65.268** | — | 85.267 | — | — | — | +0.4% |
| Cholesky factorization · 256 × 256 · f64 | 769.282 | — | 1031.412 | **404.576** | — | 745.574 | — | — | — | +0.4% |
| Shared-variable backward · sum(X * X), 64 × 32; prepared graph · f64 | 8.368 | — | — | — | **4.983** | — | — | — | — | -0.2% |
| Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64 | **3.536** | — | — | — | 7.255 | — | — | — | — | +0.9% |
| Model inference · batched normalized/sliced MLP · f64 | **3.031** | — | — | — | 5.335 | — | — | — | — | -0.3% |
| Model recording + forward · batched normalized/sliced MLP · f64 | **3.856** | — | — | — | 7.6 | — | — | — | — | +1.3% |
| Model backward · prepared graph → fresh gradients · f64 | **4.2** | — | — | — | 21.198 | — | — | — | — | +0.8% |
| Parameter update · transactional SGD, fixed gradients · f64 | 0.728 | — | — | — | — | — | — | — | — | +0.0% |
| Coalesced tensor contraction · 16 × 32 × 8 · 32 × 8 × 16 · f64 | 46.718 | — | — | — | — | **16.415** | — | — | — | -0.1% |
| Row softmax · 256 × 64 · f64 | 113.009 | **97.274** | — | — | 110.506 | 134.696 | — | — | — | -0.2% |
| Create zeros · 64 × 64 · f32 | 0.208 | **0.136** | — | 0.32 | 0.23999999999999997 | 0.528 | — | — | — | +4.0% |
| Create ones · 64 × 64 · f32 | 0.256 | **0.208** | — | 0.23999999999999997 | 0.336 | 1.528 | — | — | — | +0.0% |

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
| Matrix multiplication · transposed view · 128 × 128 · 128 × 128 · f64 | 0.9932030905284503× [0.9862801868789893, 1.007483188743173] | Timing direction unclear |
| Matrix multiplication · transposed view · 128 × 128 · 128 × 128 · f32 | 0.9959944736588449× [0.9920063778899814, 0.9988831974914898] | Observed time reduction |
| Independent matrix operands · 128 × 128 · 128 × 128 · f64 | 1.0021819561711414× [0.992643247573273, 1.0048934706273134] | Timing direction unclear |
| Independent matrix operands · 128 × 128 · 128 × 128 · f32 | 0.9977473271394556× [0.9967462797966133, 1.0013204173921173] | Timing direction unclear |
| Matrix multiplication · 32 × 32 · 32 × 32 · f64 | 1.0003095017022594× [0.9996913580246913, 1.0024752475247525] | Timing direction unclear |
| Matrix multiplication · 64 × 64 · 64 × 64 · f64 | — | No paired comparison |
| Matrix multiplication · 128 × 128 · 128 × 128 · f64 | 1.002187844645146× [0.9990965717274499, 1.0042834705630383] | Timing direction unclear |
| Matrix multiplication · 256 × 256 · 256 × 256 · f64 | 1.0000733170524125× [0.9988657096941796, 1.0012095566320618] | Timing direction unclear |
| Matrix multiplication · 64 × 64 · 64 × 64 · f32 | — | No paired comparison |
| Matrix multiplication · 128 × 128 · 128 × 128 · f32 | 0.9988776130571014× [0.9974802855676357, 1.0012975171833356] | Timing direction unclear |
| Matrix multiplication · 256 × 256 · 256 × 256 · f32 | 1.0003351722513611× [0.9982475085060689, 1.0032564327396971] | Timing direction unclear |
| Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64 | 1.0020833333333337× [0.9979209979209976, 1.004428236519927] | Timing direction unclear |
| Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f32 | 1.0033444816053512× [1.000418235048097, 1.0033444816053512] | Observed time increase |
| Matmul · 2 workers · reused output · 64 × 64 · 64 × 64 · f64 | — | No paired comparison |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f64 | 0.9968366943455912× [0.9819406051349345, 1.0067972337839728] | Timing direction unclear |
| Matmul · 2 workers · reused output · 256 × 256 · 256 × 256 · f64 | 0.9989854787615325× [0.9964796177870741, 1.008395017412424] | Timing direction unclear |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f32 | 0.9975581228325775× [0.9960705660840735, 1.000474285478616] | Timing direction unclear |
| Cholesky solve · reused factor · 128 × 128, 1 RHS · f64 | 1.0209528362014022× [1.0044167521097878, 1.028552992418132] | Observed time increase |
| Cholesky solve · reused factor · 256 × 256, 64 RHS · f64 | 0.9999389678238367× [0.9988822449399083, 1.0019213770644335] | Timing direction unclear |
| Cholesky solve · reused factor · 512 × 512, 128 RHS · f64 | — | No paired comparison |
| LU solve · reused factor · 256 × 256, 64 RHS · f64 | 0.999545442181394× [0.9989342808541711, 1.0014338340055817] | Timing direction unclear |
| Cholesky rank-one update · 256 × 256 · f64 | — | No paired comparison |
| Cholesky rank-one downdate · 256 × 256 · f64 | — | No paired comparison |
| Cholesky rank-one update · 512 × 512 · f64 | — | No paired comparison |
| Thin QR · 512 × 128 · f64 | 1.0013490003755645× [0.9992072846344657, 1.0035722503949913] | Timing direction unclear |
| Thin QR · 1024 × 256 · f64 | 0.9999680183567204× [0.998100644669221, 1.006039028938504] | Timing direction unclear |
| Cholesky factorization · 512 × 512 · f64 | 0.9991390368849621× [0.994341539844011, 1.0006276015852136] | Timing direction unclear |
| Column-pivoted QR · 96 × 32 · f64 | — | No paired comparison |
| Column-pivoted QR · 256 × 64 · f64 | — | No paired comparison |
| Minimum-norm least squares · 64 × 32, rank 16, 4 RHS · f64 | — | No paired comparison |
| Minimum-norm least squares · 32 × 64, rank 32, 4 RHS · f64 | — | No paired comparison |
| Thin QR · 96 × 32 · f64 | — | No paired comparison |
| Thin QR · 256 × 64 · f64 | 0.9947204933445353× [0.9924569352137417, 1.0028312037757108] | Timing direction unclear |
| Cholesky factorization · 64 × 64 · f64 | — | No paired comparison |
| Cholesky factorization · 128 × 128 · f64 | 1.0025886337406835× [0.9989399875376913, 1.0068572085058107] | Timing direction unclear |
| Cholesky factorization · 256 × 256 · f64 | 0.9943369422082682× [0.9896212753455892, 1.001343488214958] | Timing direction unclear |
| Shared-variable backward · sum(X * X), 64 × 32; prepared graph · f64 | — | No paired comparison |
| Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64 | 1.0022779043280183× [1.0002841716396704, 1.0113895216400912] | Observed time increase |
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

Minimum-norm rows use pivoted QR plus scaled implicit RZ in Kernix; NumPy native lstsq; nalgebra native SVD/solve. Algorithm costs and rank policies differ. See original method for thresholds. Kernix/nalgebra: prepared base factor, owned modified factor; refactor: same Kernix version, full factorization of preformed modified matrix; base setup and matrix formation excluded Rank two; cyclic five diagonals; exact dyadic values; 1 worker; owned output; prepared operands except auto construction. Kernix Matrix finite checks vs peer IEEE policies; payload excludes capacity, metadata and allocator overhead. CSR indices usize in Kernix/faer and actual recorded SciPy width. SpGEMM heap merge vs SciPy column accumulator; fill-in depends on pattern. Construction: Kernix clones a Vec then validates/scans/compresses; SciPy borrows dense input and allocates CSR output; NumPy/dense Kernix copy only. Construction compares API trade-offs, not identical algorithms. No universal sparse speed or density threshold claim.

Numbers retain source precision; recorded digits do not imply equivalent physical measurement accuracy. Lowest observed medians are bold only when peers were measured. No aggregate winner or universal ranking is inferred.

[Original report and commentary](original-report.txt) · [Raw measurements](raw.json)

Measured source `7c11b2165b0cb7b523db4d938a36f1396396a138`. Run `runner-ubuntu-24.04-arm-40289723446`. Rendering does not change measurements.
