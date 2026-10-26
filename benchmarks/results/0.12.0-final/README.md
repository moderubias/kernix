# Kernix 0.12.0 · performance brief

12th Gen Intel(R) Core(TM) i3-1215U / Linux

**Per-call median, microseconds per operation. Lower is faster. All recorded digits retained.**

46 workloads · 152 observations

Baseline: 0.11.1-registry-0120. Sequential run changes are descriptive; paired process evidence is shown separately.

[Interactive HTML (open locally)](report.html) · [Vector PDF](latency.pdf) · [Raw measurements](raw.json)

[![Performance brief preview](latency.png)](latency.pdf)

## Charts by workload

**Shorter bars mean less time.** Blue is Kernix; grey is a comparison implementation; teal is an explicitly selected Kernix backend. Entries are sorted by the recorded median, with a separate zero-based scale per chart. These point observations do not establish a statistical ranking.

Expand a group to see every chart here on GitHub. Click a chart to enlarge it. The exact table below keeps every source digit.

<details>
<summary>Matrix multiplication · 13 workloads</summary>

**Matrix multiplication · transposed view · 128 × 128 · 128 × 128 · f64**

![Matrix multiplication · transposed view · 128 × 128 · 128 × 128 · f64. Shorter bars mean less time.](charts/matmul_transposed_128_f64.png)

Kernix: **122.731 µs**. Kernix took ≈ 56.2% more time than NumPy in this run.

**Matrix multiplication · transposed view · 128 × 128 · 128 × 128 · f32**

![Matrix multiplication · transposed view · 128 × 128 · 128 × 128 · f32. Shorter bars mean less time.](charts/matmul_transposed_128_f32.png)

Kernix: **49.265 µs**. Kernix took ≈ 35.6% more time than NumPy in this run.

**Independent matrix operands · 128 × 128 · 128 × 128 · f64**

![Independent matrix operands · 128 × 128 · 128 × 128 · f64. Shorter bars mean less time.](charts/matmul_ab_128_f64.png)

Kernix: **69.063 µs**. Kernix took ≈ 16.1% less time than NumPy in this run.

**Independent matrix operands · 128 × 128 · 128 × 128 · f32**

![Independent matrix operands · 128 × 128 · 128 × 128 · f32. Shorter bars mean less time.](charts/matmul_ab_128_f32.png)

Kernix: **36.784 µs**. Kernix took ≈ 5.8% less time than NumPy in this run.

**Matrix multiplication · 32 × 32 · 32 × 32 · f64**

![Matrix multiplication · 32 × 32 · 32 × 32 · f64. Shorter bars mean less time.](charts/matmul_32_f64.png)

Kernix: **1.566 µs**. Kernix took ≈ 12.5% more time than nalgebra in this run.

**Matrix multiplication · 64 × 64 · 64 × 64 · f64**

![Matrix multiplication · 64 × 64 · 64 × 64 · f64. Shorter bars mean less time.](charts/matmul_64_f64.png)

Kernix: **10.14 µs**. Kernix took ≈ 7.0% more time than Candle in this run.

**Matrix multiplication · 128 × 128 · 128 × 128 · f64**

![Matrix multiplication · 128 × 128 · 128 × 128 · f64. Shorter bars mean less time.](charts/matmul_128_f64.png)

Kernix: **74.135 µs**. Kernix took ≈ 7.6% more time than Candle in this run.

**Matrix multiplication · 256 × 256 · 256 × 256 · f64**

![Matrix multiplication · 256 × 256 · 256 × 256 · f64. Shorter bars mean less time.](charts/matmul_256_f64.png)

Kernix: **585.325 µs**. Kernix took ≈ 4.9% more time than NumPy in this run.

**Matrix multiplication · 64 × 64 · 64 × 64 · f32**

![Matrix multiplication · 64 × 64 · 64 × 64 · f32. Shorter bars mean less time.](charts/matmul_64_f32.png)

Kernix: **4.502 µs**. Kernix took ≈ 7.9% less time than Candle in this run.

**Matrix multiplication · 128 × 128 · 128 × 128 · f32**

![Matrix multiplication · 128 × 128 · 128 × 128 · f32. Shorter bars mean less time.](charts/matmul_128_f32.png)

Kernix: **34.163 µs**. Kernix took ≈ 2.3% less time than Candle in this run.

**Matrix multiplication · 256 × 256 · 256 × 256 · f32**

![Matrix multiplication · 256 × 256 · 256 × 256 · f32. Shorter bars mean less time.](charts/matmul_256_f32.png)

Kernix: **271.677 µs**. Kernix took &lt; 0.1% more time than Candle in this run.

**Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64**

![Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64. Shorter bars mean less time.](charts/matmul_batched_4x16x32x16_f64.png)

Kernix: **2.0049999999999998 µs**. Kernix took ≈ 9.8% less time than Candle in this run.

**Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f32**

![Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f32. Shorter bars mean less time.](charts/matmul_batched_4x16x32x16_f32.png)

Kernix: **0.961 µs**. Kernix took ≈ 29.9% less time than Candle in this run.

</details>

<details>
<summary>Explicit parallel execution · 4 workloads</summary>

**Matmul · 2 workers · reused output · 64 × 64 · 64 × 64 · f64**

![Matmul · 2 workers · reused output · 64 × 64 · 64 × 64 · f64. Shorter bars mean less time.](charts/parallel_matmul_into_64_f64.png)

Kernix: **9.656 µs**. No external comparison with Kernix was measured for this workload.

**Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f64**

![Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f64. Shorter bars mean less time.](charts/parallel_matmul_into_128_f64.png)

Kernix: **41.991 µs**. No external comparison with Kernix was measured for this workload.

**Matmul · 2 workers · reused output · 256 × 256 · 256 × 256 · f64**

![Matmul · 2 workers · reused output · 256 × 256 · 256 × 256 · f64. Shorter bars mean less time.](charts/parallel_matmul_into_256_f64.png)

Kernix: **306.27 µs**. No external comparison with Kernix was measured for this workload.

**Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f32**

![Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f32. Shorter bars mean less time.](charts/parallel_matmul_into_128_f32.png)

Kernix: **22.457 µs**. No external comparison with Kernix was measured for this workload.

</details>

<details>
<summary>Decompositions & solvers · 19 workloads</summary>

**Cholesky solve · reused factor · 128 × 128, 1 RHS · f64**

![Cholesky solve · reused factor · 128 × 128, 1 RHS · f64. Shorter bars mean less time.](charts/solve_cholesky_128x1_f64.png)

Kernix: **9.304 µs**. Kernix took ≈ 136.1% more time than faer in this run.

**Cholesky solve · reused factor · 256 × 256, 64 RHS · f64**

![Cholesky solve · reused factor · 256 × 256, 64 RHS · f64. Shorter bars mean less time.](charts/solve_cholesky_256x64_f64.png)

Kernix: **1007.406 µs**. Kernix took ≈ 405.7% more time than faer in this run.

**Cholesky solve · reused factor · 512 × 512, 128 RHS · f64**

![Cholesky solve · reused factor · 512 × 512, 128 RHS · f64. Shorter bars mean less time.](charts/solve_cholesky_512x128_f64.png)

Kernix: **9067.001 µs**. Kernix took ≈ 446.9% more time than faer in this run.

**LU solve · reused factor · 256 × 256, 64 RHS · f64**

![LU solve · reused factor · 256 × 256, 64 RHS · f64. Shorter bars mean less time.](charts/solve_lu_256x64_f64.png)

Kernix: **1103.033 µs**. Kernix took ≈ 359.9% more time than faer in this run.

**Cholesky rank-one update · 256 × 256 · f64**

![Cholesky rank-one update · 256 × 256 · f64. Shorter bars mean less time.](charts/cholesky_rank_update_256_f64.png)

Kernix: **112.75 µs**. Kernix took ≈ 419.3% more time than nalgebra in this run.

**Cholesky rank-one downdate · 256 × 256 · f64**

![Cholesky rank-one downdate · 256 × 256 · f64. Shorter bars mean less time.](charts/cholesky_rank_downdate_256_f64.png)

Kernix: **119.132 µs**. Kernix took ≈ 399.1% more time than nalgebra in this run.

**Cholesky rank-one update · 512 × 512 · f64**

![Cholesky rank-one update · 512 × 512 · f64. Shorter bars mean less time.](charts/cholesky_rank_update_512_f64.png)

Kernix: **722.691 µs**. Kernix took ≈ 389.9% more time than nalgebra in this run.

**Thin QR · 512 × 128 · f64**

![Thin QR · 512 × 128 · f64. Shorter bars mean less time.](charts/qr_512x128_f64.png)

Kernix: **2667.134 µs**. Kernix took ≈ 21.9% more time than faer in this run.

**Thin QR · 1024 × 256 · f64**

![Thin QR · 1024 × 256 · f64. Shorter bars mean less time.](charts/qr_1024x256_f64.png)

Kernix: **17883.006 µs**. Kernix took ≈ 36.5% more time than faer in this run.

**Cholesky factorization · 512 × 512 · f64**

![Cholesky factorization · 512 × 512 · f64. Shorter bars mean less time.](charts/cholesky_factor_512_f64.png)

Kernix: **3174.989 µs**. Kernix took ≈ 129.8% more time than faer in this run.

**Column-pivoted QR · 96 × 32 · f64**

![Column-pivoted QR · 96 × 32 · f64. Shorter bars mean less time.](charts/pivoted_qr_96x32_f64.png)

Kernix: **52.032 µs**. Kernix took ≈ 26.2% less time than faer in this run.

**Column-pivoted QR · 256 × 64 · f64**

![Column-pivoted QR · 256 × 64 · f64. Shorter bars mean less time.](charts/pivoted_qr_256x64_f64.png)

Kernix: **542.78 µs**. Kernix took ≈ 14.9% more time than faer in this run.

**Minimum-norm least squares · 64 × 32, rank 16, 4 RHS · f64**

![Minimum-norm least squares · 64 × 32, rank 16, 4 RHS · f64. Shorter bars mean less time.](charts/min_norm_64x32_rank16_f64.png)

Kernix: **41.648 µs**. Kernix took ≈ 37.8% less time than NumPy in this run.

**Minimum-norm least squares · 32 × 64, rank 32, 4 RHS · f64**

![Minimum-norm least squares · 32 × 64, rank 32, 4 RHS · f64. Shorter bars mean less time.](charts/min_norm_32x64_rank32_f64.png)

Kernix: **65.96 µs**. Kernix took ≈ 42.3% less time than NumPy in this run.

**Thin QR · 96 × 32 · f64**

![Thin QR · 96 × 32 · f64. Shorter bars mean less time.](charts/qr_96x32_f64.png)

Kernix: **47.241 µs**. Kernix took ≈ 37.3% more time than nalgebra in this run.

**Thin QR · 256 × 64 · f64**

![Thin QR · 256 × 64 · f64. Shorter bars mean less time.](charts/qr_256x64_f64.png)

Kernix: **515.95600000000006 µs**. Kernix took ≈ 63.2% more time than NumPy in this run.

**Cholesky factorization · 64 × 64 · f64**

![Cholesky factorization · 64 × 64 · f64. Shorter bars mean less time.](charts/cholesky_factor_64_f64.png)

Kernix: **21.617 µs**. Kernix took ≈ 158.3% more time than faer in this run.

**Cholesky factorization · 128 × 128 · f64**

![Cholesky factorization · 128 × 128 · f64. Shorter bars mean less time.](charts/cholesky_factor_128_f64.png)

Kernix: **112.836 µs**. Kernix took ≈ 203.1% more time than faer in this run.

**Cholesky factorization · 256 × 256 · f64**

![Cholesky factorization · 256 × 256 · f64. Shorter bars mean less time.](charts/cholesky_factor_256_f64.png)

Kernix: **485.969 µs**. Kernix took ≈ 150.4% more time than faer in this run.

</details>

<details>
<summary>Training & autodiff · 6 workloads</summary>

**Shared-variable backward · sum(X * X), 64 × 32; prepared graph · f64**

![Shared-variable backward · sum(X * X), 64 × 32; prepared graph · f64. Shorter bars mean less time.](charts/training_shared_backward_64x32_f64.png)

Kernix: **6.062 µs**. Kernix took ≈ 56.6% more time than Candle in this run.

**Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64**

![Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64. Shorter bars mean less time.](charts/matmul_sum_backward_32x16x8_f64.png)

Kernix: **2.254 µs**. Kernix took ≈ 45.7% less time than Candle in this run.

**Model inference · batched normalized/sliced MLP · f64**

![Model inference · batched normalized/sliced MLP · f64. Shorter bars mean less time.](charts/training_forward_2x4x8x4_f64.png)

Kernix: **2.188 µs**. Kernix took ≈ 36.6% less time than Candle in this run.

**Model recording + forward · batched normalized/sliced MLP · f64**

![Model recording + forward · batched normalized/sliced MLP · f64. Shorter bars mean less time.](charts/training_record_2x4x8x4_f64.png)

Kernix: **2.7 µs**. Kernix took ≈ 46.6% less time than Candle in this run.

**Model backward · prepared graph → fresh gradients · f64**

![Model backward · prepared graph → fresh gradients · f64. Shorter bars mean less time.](charts/training_backward_2x4x8x4_f64.png)

Kernix: **2.943 µs**. Kernix took ≈ 79.7% less time than Candle in this run.

**Parameter update · transactional SGD, fixed gradients · f64**

![Parameter update · transactional SGD, fixed gradients · f64. Shorter bars mean less time.](charts/training_sgd_2x4x8x4_f64.png)

Kernix: **0.51 µs**. No external comparison with Kernix was measured for this workload.

</details>

<details>
<summary>Tensor contractions · 1 workloads</summary>

**Coalesced tensor contraction · 16 × 32 × 8 · 32 × 8 × 16 · f64**

![Coalesced tensor contraction · 16 × 32 × 8 · 32 × 8 × 16 · f64. Shorter bars mean less time.](charts/tensordot_16x32x8_32x8x16_f64.png)

Kernix: **35.885 µs**. Kernix took ≈ 398.3% more time than NumPy in this run.

</details>

<details>
<summary>Statistics & activations · 1 workloads</summary>

**Row softmax · 256 × 64 · f64**

![Row softmax · 256 × 64 · f64. Shorter bars mean less time.](charts/softmax_256x64_f64.png)

Kernix: **75.698 µs**. Kernix took ≈ 9.5% more time than ndarray in this run.

</details>

<details>
<summary>Tensor construction · 2 workloads</summary>

**Create zeros · 64 × 64 · f32**

![Create zeros · 64 × 64 · f32. Shorter bars mean less time.](charts/create_zeros_64x64_f32.png)

Kernix: **0.141 µs**. Kernix took ≈ 43.9% more time than ndarray in this run.

**Create ones · 64 × 64 · f32**

![Create ones · 64 × 64 · f32. Shorter bars mean less time.](charts/create_ones_64x64_f32.png)

Kernix: **0.201 µs**. Kernix took ≈ 43.6% more time than ndarray in this run.

</details>

## Exact latency table

All values below are microseconds per operation. **Lower is faster.** Missing entries were not measured.

| Workload | Kernix | ndarray | nalgebra | faer | Candle | NumPy | Kernix refactor | Sequential run change |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| Matrix multiplication · transposed view · 128 × 128 · 128 × 128 · f64 | 122.731 | — | — | — | — | **78.575** | — | -2.2% |
| Matrix multiplication · transposed view · 128 × 128 · 128 × 128 · f32 | 49.265 | — | — | — | — | **36.336** | — | +1.8% |
| Independent matrix operands · 128 × 128 · 128 × 128 · f64 | **69.063** | — | — | — | — | 82.268 | — | -6.9% |
| Independent matrix operands · 128 × 128 · 128 × 128 · f32 | **36.784** | — | — | — | — | 39.052 | — | -2.7% |
| Matrix multiplication · 32 × 32 · 32 × 32 · f64 | 1.566 | 1.501 | **1.392** | 1.541 | 1.4 | 2.355 | — | -4.1% |
| Matrix multiplication · 64 × 64 · 64 × 64 · f64 | 10.14 | 10.543 | 11.275 | 10.286 | **9.478** | 10.994 | — | +2.6% |
| Matrix multiplication · 128 × 128 · 128 × 128 · f64 | 74.135 | 119.173 | 119.011 | 71.69 | **68.889** | 77.964 | — | -5.7% |
| Matrix multiplication · 256 × 256 · 256 × 256 · f64 | 585.325 | 666.647 | 631.138 | 679.925 | 563.193 | **557.881** | — | +4.9% |
| Matrix multiplication · 64 × 64 · 64 × 64 · f32 | **4.502** | 6.152 | 6.083 | 4.91 | 4.89 | 5.775 | — | -12.7% |
| Matrix multiplication · 128 × 128 · 128 × 128 · f32 | **34.163** | 48.903 | 50.392 | 37.779 | 34.972 | 36.352 | — | -2.4% |
| Matrix multiplication · 256 × 256 · 256 × 256 · f32 | 271.677 | 413.609 | 387.147 | 305.567 | **271.568** | 280.681 | — | -6.0% |
| Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64 | **2.0049999999999998** | — | — | — | 2.222 | 2.756 | — | +34.3% |
| Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f32 | **0.961** | — | — | — | 1.371 | 2.032 | — | -8.1% |
| Matmul · 2 workers · reused output · 64 × 64 · 64 × 64 · f64 | 9.656 | — | — | — | — | — | — | -12.1% |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f64 | 41.991 | — | — | — | — | — | — | +1.3% |
| Matmul · 2 workers · reused output · 256 × 256 · 256 × 256 · f64 | 306.27 | — | — | — | — | — | — | +1.0% |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f32 | 22.457 | — | — | — | — | — | — | -4.0% |
| Cholesky solve · reused factor · 128 × 128, 1 RHS · f64 | 9.304 | — | 4.0439999999999995 | **3.941** | — | — | — | -20.7% |
| Cholesky solve · reused factor · 256 × 256, 64 RHS · f64 | 1007.406 | — | 769.663 | **199.222** | — | — | — | -66.1% |
| Cholesky solve · reused factor · 512 × 512, 128 RHS · f64 | 9067.001 | — | 6934.999 | **1658.008** | — | — | — | -79.8% |
| LU solve · reused factor · 256 × 256, 64 RHS · f64 | 1103.033 | — | 754.701 | **239.854** | — | — | — | -54.4% |
| Cholesky rank-one update · 256 × 256 · f64 | 112.75 | — | **21.713** | — | — | — | 529.381 | — |
| Cholesky rank-one downdate · 256 × 256 · f64 | 119.132 | — | **23.87** | — | — | — | 540.5 | — |
| Cholesky rank-one update · 512 × 512 · f64 | 722.691 | — | **147.525** | — | — | — | 2818.152 | — |
| Thin QR · 512 × 128 · f64 | 2667.134 | — | 3130.708 | **2187.147** | — | 2526.361 | — | -25.8% |
| Thin QR · 1024 × 256 · f64 | 17883.006 | — | 26987.916 | **13100.052** | — | 13946.581 | — | -39.6% |
| Cholesky factorization · 512 × 512 · f64 | 3174.989 | — | 5227.821 | **1381.789** | — | 3828.883 | — | +13.3% |
| Column-pivoted QR · 96 × 32 · f64 | **52.032** | — | 83.223 | 70.533 | — | — | — | -3.2% |
| Column-pivoted QR · 256 × 64 · f64 | 542.78 | — | 785.617 | **472.299** | — | — | — | +3.4% |
| Minimum-norm least squares · 64 × 32, rank 16, 4 RHS · f64 | **41.648** | — | 73.977 | — | — | 66.909 | — | -2.9% |
| Minimum-norm least squares · 32 × 64, rank 32, 4 RHS · f64 | **65.96** | — | 124.52700000000001 | — | — | 114.402 | — | +20.0% |
| Thin QR · 96 × 32 · f64 | 47.241 | — | **34.4** | 65.21500000000001 | — | 39.234 | — | +3.1% |
| Thin QR · 256 × 64 · f64 | 515.95600000000006 | — | 372.457 | 426.41 | — | **316.185** | — | +8.0% |
| Cholesky factorization · 64 × 64 · f64 | 21.617 | — | 14.128 | **8.37** | — | 17.409 | — | +3.4% |
| Cholesky factorization · 128 × 128 · f64 | 112.836 | — | 96.496 | **37.224** | — | 68.714 | — | +2.0% |
| Cholesky factorization · 256 × 256 · f64 | 485.969 | — | 913.281 | **194.053** | — | 514.59 | — | -5.2% |
| Shared-variable backward · sum(X * X), 64 × 32; prepared graph · f64 | 6.062 | — | — | — | **3.87** | — | — | -9.7% |
| Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64 | **2.254** | — | — | — | 4.153 | — | — | -5.8% |
| Model inference · batched normalized/sliced MLP · f64 | **2.188** | — | — | — | 3.451 | — | — | +5.3% |
| Model recording + forward · batched normalized/sliced MLP · f64 | **2.7** | — | — | — | 5.06 | — | — | +3.6% |
| Model backward · prepared graph → fresh gradients · f64 | **2.943** | — | — | — | 14.463 | — | — | -6.9% |
| Parameter update · transactional SGD, fixed gradients · f64 | 0.51 | — | — | — | — | — | — | -3.0% |
| Coalesced tensor contraction · 16 × 32 × 8 · 32 × 8 × 16 · f64 | 35.885 | — | — | — | — | **7.201** | — | -0.4% |
| Row softmax · 256 × 64 · f64 | 75.698 | **69.113** | — | — | 80.789 | 100.295 | — | -10.5% |
| Create zeros · 64 × 64 · f32 | 0.141 | **0.098** | — | 0.143 | 0.168 | 0.376 | — | -2.8% |
| Create ones · 64 × 64 · f32 | 0.201 | **0.14** | — | 0.203 | 0.218 | 1.331 | — | +9.8% |

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
| Matrix multiplication · transposed view · 128 × 128 · 128 × 128 · f64 | 0.9741677808632452× [0.9438496742229416, 0.9984214886844286] | Observed time reduction |
| Matrix multiplication · transposed view · 128 × 128 · 128 × 128 · f32 | 0.9990550437042287× [0.9766783174641437, 1.0745344105206183] | Timing direction unclear |
| Independent matrix operands · 128 × 128 · 128 × 128 · f64 | 1.0028067202414839× [0.9733149389042176, 1.0364563187268607] | Timing direction unclear |
| Independent matrix operands · 128 × 128 · 128 × 128 · f32 | 1.0011050291676304× [0.9775446246082573, 1.0589875111507583] | Timing direction unclear |
| Matrix multiplication · 32 × 32 · 32 × 32 · f64 | 1.0133164235890932× [0.9890243902439024, 1.1006211180124224] | Timing direction unclear |
| Matrix multiplication · 64 × 64 · 64 × 64 · f64 | — | No paired comparison |
| Matrix multiplication · 128 × 128 · 128 × 128 · f64 | 0.9941706698332163× [0.9845926337875073, 0.9993919658419922] | Observed time reduction |
| Matrix multiplication · 256 × 256 · 256 × 256 · f64 | 1.0054575241831976× [0.981300896638036, 1.025779538880808] | Timing direction unclear |
| Matrix multiplication · 64 × 64 · 64 × 64 · f32 | — | No paired comparison |
| Matrix multiplication · 128 × 128 · 128 × 128 · f32 | 1.0183357601251146× [0.993922368385511, 1.0356599674090168] | Timing direction unclear |
| Matrix multiplication · 256 × 256 · 256 × 256 · f32 | 0.9973555103077842× [0.9729474862969293, 0.9986107093239619] | Observed time reduction |
| Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64 | 1.1025043680838673× [0.9660265878877399, 1.1978680879413723] | Timing direction unclear |
| Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f32 | 1.0519618239660657× [1.006554307116105, 1.1666666666666667] | Observed time increase |
| Matmul · 2 workers · reused output · 64 × 64 · 64 × 64 · f64 | — | No paired comparison |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f64 | 1.003797530380243× [0.7079180180333158, 1.7986926866237212] | Timing direction unclear |
| Matmul · 2 workers · reused output · 256 × 256 · 256 × 256 · f64 | 0.965063468796831× [0.9576436699021517, 1.0025281357038003] | Timing direction unclear |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f32 | 1.0060232840382972× [0.9759132167367722, 1.7554905490549055] | Timing direction unclear |
| Cholesky solve · reused factor · 128 × 128, 1 RHS · f64 | 0.8549598323436954× [0.8378494623655914, 0.8691263176701804] | Observed time reduction |
| Cholesky solve · reused factor · 256 × 256, 64 RHS · f64 | 0.3604773282010496× [0.35471006182411524, 0.38053385725526634] | Observed time reduction |
| Cholesky solve · reused factor · 512 × 512, 128 RHS · f64 | — | No paired comparison |
| LU solve · reused factor · 256 × 256, 64 RHS · f64 | 0.44287614487737254× [0.4398298429214799, 0.4580180849626527] | Observed time reduction |
| Cholesky rank-one update · 256 × 256 · f64 | — | No paired comparison |
| Cholesky rank-one downdate · 256 × 256 · f64 | — | No paired comparison |
| Cholesky rank-one update · 512 × 512 · f64 | — | No paired comparison |
| Thin QR · 512 × 128 · f64 | 0.7576909363946751× [0.7432072422955074, 0.8038720198704554] | Observed time reduction |
| Thin QR · 1024 × 256 · f64 | 0.4991791918264273× [0.4803599515307621, 0.5078611944426149] | Observed time reduction |
| Cholesky factorization · 512 × 512 · f64 | 1.00042872342826× [0.9864813132222578, 1.0167709594881083] | Timing direction unclear |
| Column-pivoted QR · 96 × 32 · f64 | — | No paired comparison |
| Column-pivoted QR · 256 × 64 · f64 | — | No paired comparison |
| Minimum-norm least squares · 64 × 32, rank 16, 4 RHS · f64 | — | No paired comparison |
| Minimum-norm least squares · 32 × 64, rank 32, 4 RHS · f64 | — | No paired comparison |
| Thin QR · 96 × 32 · f64 | — | No paired comparison |
| Thin QR · 256 × 64 · f64 | 0.9788347720307858× [0.9634141448186757, 0.989606890943739] | Observed time reduction |
| Cholesky factorization · 64 × 64 · f64 | — | No paired comparison |
| Cholesky factorization · 128 × 128 · f64 | 0.995463411620374× [0.9778598743749026, 1.0276817508382503] | Timing direction unclear |
| Cholesky factorization · 256 × 256 · f64 | 1.01549232141547× [1.0091252328428177, 1.031882568924252] | Observed time increase |
| Shared-variable backward · sum(X * X), 64 × 32; prepared graph · f64 | — | No paired comparison |
| Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64 | 0.9895963379109447× [0.9548841059602649, 1.0287800687285222] | Timing direction unclear |
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

Measured source `90cbbff60e5d7458ef9f89918c99a6cca94b51d1`. Run `0.12.0-final`. Rendering does not change measurements.
