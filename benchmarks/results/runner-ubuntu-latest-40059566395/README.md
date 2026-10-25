# Kernix 0.11.0 · performance brief

AMD EPYC 7763 64-Core Processor / Linux

**Per-call median, microseconds per operation. Lower is faster. All recorded digits retained.**

34 workloads · 116 observations

Baseline: baseline-ubuntu-latest-40059566395. Sequential run changes are descriptive; paired process evidence is shown separately.

[Interactive HTML (open locally)](report.html) · [Vector PDF](latency.pdf) · [Raw measurements](raw.json)

[![Performance brief preview](latency.png)](latency.pdf)

## Charts by workload

**Shorter bars mean less time.** Blue is Kernix; grey is another library; teal is an explicitly selected Kernix backend. Entries are sorted by the recorded median, with a separate zero-based scale per chart. These point observations do not establish a statistical ranking.

Expand a group to see every chart here on GitHub. Click a chart to enlarge it. The exact table below keeps every source digit.

<details>
<summary>Matrix multiplication · 10 workloads</summary>

**Independent matrix operands · 128 × 128 · 128 × 128 · f64**

![Independent matrix operands · 128 × 128 · 128 × 128 · f64. Shorter bars mean less time.](charts/matmul_ab_128_f64.png)

Kernix: **94.016 µs**. Kernix took ≈ 8.4% less time than NumPy in this run.

**Independent matrix operands · 128 × 128 · 128 × 128 · f32**

![Independent matrix operands · 128 × 128 · 128 × 128 · f32. Shorter bars mean less time.](charts/matmul_ab_128_f32.png)

Kernix: **45.996 µs**. Kernix took ≈ 7.6% less time than NumPy in this run.

**Matrix multiplication · 32 × 32 · 32 × 32 · f64**

![Matrix multiplication · 32 × 32 · 32 × 32 · f64. Shorter bars mean less time.](charts/matmul_32_f64.png)

Kernix: **2.524 µs**. Kernix took ≈ 23.5% more time than faer in this run.

**Matrix multiplication · 64 × 64 · 64 × 64 · f64**

![Matrix multiplication · 64 × 64 · 64 × 64 · f64. Shorter bars mean less time.](charts/matmul_64_f64.png)

Kernix: **12.724 µs**. Kernix took ≈ 0.9% less time than Candle in this run.

**Matrix multiplication · 128 × 128 · 128 × 128 · f64**

![Matrix multiplication · 128 × 128 · 128 × 128 · f64. Shorter bars mean less time.](charts/matmul_128_f64.png)

Kernix: **93.986 µs**. Kernix took ≈ 1.1% more time than Candle in this run.

**Matrix multiplication · 256 × 256 · 256 × 256 · f64**

![Matrix multiplication · 256 × 256 · 256 × 256 · f64. Shorter bars mean less time.](charts/matmul_256_f64.png)

Kernix: **730.869 µs**. Kernix took ≈ 1.0% less time than Candle in this run.

**Matrix multiplication · 64 × 64 · 64 × 64 · f32**

![Matrix multiplication · 64 × 64 · 64 × 64 · f32. Shorter bars mean less time.](charts/matmul_64_f32.png)

Kernix: **6.612 µs**. Kernix took ≈ 1.5% more time than faer in this run.

**Matrix multiplication · 128 × 128 · 128 × 128 · f32**

![Matrix multiplication · 128 × 128 · 128 × 128 · f32. Shorter bars mean less time.](charts/matmul_128_f32.png)

Kernix: **45.836 µs**. Kernix took ≈ 0.8% less time than Candle in this run.

**Matrix multiplication · 256 × 256 · 256 × 256 · f32**

![Matrix multiplication · 256 × 256 · 256 × 256 · f32. Shorter bars mean less time.](charts/matmul_256_f32.png)

Kernix: **352.229 µs**. Kernix took ≈ 0.3% less time than Candle in this run.

**Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64**

![Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64. Shorter bars mean less time.](charts/matmul_batched_4x16x32x16_f64.png)

Kernix: **3.126 µs**. Kernix took ≈ 3.7% less time than Candle in this run.

</details>

<details>
<summary>Explicit parallel execution · 4 workloads</summary>

**Matmul · 2 workers · reused output · 64 × 64 · 64 × 64 · f64**

![Matmul · 2 workers · reused output · 64 × 64 · 64 × 64 · f64. Shorter bars mean less time.](charts/parallel_matmul_into_64_f64.png)

Kernix: **18.966 µs**. No external comparison with Kernix was measured for this workload.

**Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f64**

![Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f64. Shorter bars mean less time.](charts/parallel_matmul_into_128_f64.png)

Kernix: **56.566 µs**. No external comparison with Kernix was measured for this workload.

**Matmul · 2 workers · reused output · 256 × 256 · 256 × 256 · f64**

![Matmul · 2 workers · reused output · 256 × 256 · 256 × 256 · f64. Shorter bars mean less time.](charts/parallel_matmul_into_256_f64.png)

Kernix: **393.688 µs**. No external comparison with Kernix was measured for this workload.

**Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f32**

![Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f32. Shorter bars mean less time.](charts/parallel_matmul_into_128_f32.png)

Kernix: **34.084 µs**. No external comparison with Kernix was measured for this workload.

</details>

<details>
<summary>Decompositions & solvers · 10 workloads</summary>

**Cholesky factorization · 512 × 512 · f64**

![Cholesky factorization · 512 × 512 · f64. Shorter bars mean less time.](charts/cholesky_factor_512_f64.png)

Kernix: **3886.384 µs**. Kernix took ≈ 50.3% more time than faer in this run.

**Column-pivoted QR · 96 × 32 · f64**

![Column-pivoted QR · 96 × 32 · f64. Shorter bars mean less time.](charts/pivoted_qr_96x32_f64.png)

Kernix: **78.687 µs**. Kernix took ≈ 85.2% more time than faer in this run.

**Column-pivoted QR · 256 × 64 · f64**

![Column-pivoted QR · 256 × 64 · f64. Shorter bars mean less time.](charts/pivoted_qr_256x64_f64.png)

Kernix: **851.052 µs**. Kernix took ≈ 147.9% more time than faer in this run.

**Minimum-norm least squares · 64 × 32, rank 16, 4 RHS · f64**

![Minimum-norm least squares · 64 × 32, rank 16, 4 RHS · f64. Shorter bars mean less time.](charts/min_norm_64x32_rank16_f64.png)

Kernix: **62.25599999999999 µs**. Kernix took ≈ 39.3% less time than nalgebra in this run.

**Minimum-norm least squares · 32 × 64, rank 32, 4 RHS · f64**

![Minimum-norm least squares · 32 × 64, rank 32, 4 RHS · f64. Shorter bars mean less time.](charts/min_norm_32x64_rank32_f64.png)

Kernix: **84.257 µs**. Kernix took ≈ 52.8% less time than nalgebra in this run.

**Thin QR · 96 × 32 · f64**

![Thin QR · 96 × 32 · f64. Shorter bars mean less time.](charts/qr_96x32_f64.png)

Kernix: **67.256 µs**. Kernix took ≈ 23.9% more time than faer in this run.

**Thin QR · 256 × 64 · f64**

![Thin QR · 256 × 64 · f64. Shorter bars mean less time.](charts/qr_256x64_f64.png)

Kernix: **780.099 µs**. Kernix took ≈ 86.5% more time than faer in this run.

**Cholesky factorization · 64 × 64 · f64**

![Cholesky factorization · 64 × 64 · f64. Shorter bars mean less time.](charts/cholesky_factor_64_f64.png)

Kernix: **24.777 µs**. Kernix took ≈ 75.8% more time than faer in this run.

**Cholesky factorization · 128 × 128 · f64**

![Cholesky factorization · 128 × 128 · f64. Shorter bars mean less time.](charts/cholesky_factor_128_f64.png)

Kernix: **169.948 µs**. Kernix took ≈ 194.4% more time than faer in this run.

**Cholesky factorization · 256 × 256 · f64**

![Cholesky factorization · 256 × 256 · f64. Shorter bars mean less time.](charts/cholesky_factor_256_f64.png)

Kernix: **686.484 µs**. Kernix took ≈ 118.8% more time than faer in this run.

</details>

<details>
<summary>Training & autodiff · 6 workloads</summary>

**Shared-variable backward · sum(X * X), 64 × 32; prepared graph · f64**

![Shared-variable backward · sum(X * X), 64 × 32; prepared graph · f64. Shorter bars mean less time.](charts/training_shared_backward_64x32_f64.png)

Kernix: **10.41 µs**. Kernix took ≈ 110.8% more time than Candle in this run.

**Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64**

![Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64. Shorter bars mean less time.](charts/matmul_sum_backward_32x16x8_f64.png)

Kernix: **4.067 µs**. Kernix took ≈ 47.4% less time than Candle in this run.

**Model inference · batched normalized/sliced MLP · f64**

![Model inference · batched normalized/sliced MLP · f64. Shorter bars mean less time.](charts/training_forward_2x4x8x4_f64.png)

Kernix: **3.226 µs**. Kernix took ≈ 35.9% less time than Candle in this run.

**Model recording + forward · batched normalized/sliced MLP · f64**

![Model recording + forward · batched normalized/sliced MLP · f64. Shorter bars mean less time.](charts/training_record_2x4x8x4_f64.png)

Kernix: **4.098 µs**. Kernix took ≈ 33.9% less time than Candle in this run.

**Model backward · prepared graph → fresh gradients · f64**

![Model backward · prepared graph → fresh gradients · f64. Shorter bars mean less time.](charts/training_backward_2x4x8x4_f64.png)

Kernix: **5.099 µs**. Kernix took ≈ 75.4% less time than Candle in this run.

**Parameter update · transactional SGD, fixed gradients · f64**

![Parameter update · transactional SGD, fixed gradients · f64. Shorter bars mean less time.](charts/training_sgd_2x4x8x4_f64.png)

Kernix: **0.802 µs**. No external comparison with Kernix was measured for this workload.

</details>

<details>
<summary>Tensor contractions · 1 workloads</summary>

**Coalesced tensor contraction · 16 × 32 × 8 · 32 × 8 × 16 · f64**

![Coalesced tensor contraction · 16 × 32 × 8 · 32 × 8 × 16 · f64. Shorter bars mean less time.](charts/tensordot_16x32x8_32x8x16_f64.png)

Kernix: **63.067 µs**. Kernix took ≈ 456.6% more time than NumPy in this run.

</details>

<details>
<summary>Statistics & activations · 1 workloads</summary>

**Row softmax · 256 × 64 · f64**

![Row softmax · 256 × 64 · f64. Shorter bars mean less time.](charts/softmax_256x64_f64.png)

Kernix: **119.533 µs**. Kernix took ≈ 7.8% more time than ndarray in this run.

</details>

<details>
<summary>Tensor construction · 2 workloads</summary>

**Create zeros · 64 × 64 · f32**

![Create zeros · 64 × 64 · f32. Shorter bars mean less time.](charts/create_zeros_64x64_f32.png)

Kernix: **0.32 µs**. Kernix took ≈ 28.0% more time than ndarray in this run.

**Create ones · 64 × 64 · f32**

![Create ones · 64 × 64 · f32. Shorter bars mean less time.](charts/create_ones_64x64_f32.png)

Kernix: **0.431 µs**. Kernix took ≈ 22.8% more time than ndarray in this run.

</details>

## Exact latency table

All values below are microseconds per operation. **Lower is faster.** Missing entries were not measured.

| Workload | Kernix | ndarray | nalgebra | faer | Candle | NumPy | Sequential run change |
|---|---:|---:|---:|---:|---:|---:|---:|
| Independent matrix operands · 128 × 128 · 128 × 128 · f64 | **94.016** | — | — | — | — | 102.652 | +0.2% |
| Independent matrix operands · 128 × 128 · 128 × 128 · f32 | **45.996** | — | — | — | — | 49.803 | -0.2% |
| Matrix multiplication · 32 × 32 · 32 × 32 · f64 | 2.524 | 2.224 | 2.124 | **2.0439999999999998** | 2.214 | 3.677 | +0.4% |
| Matrix multiplication · 64 × 64 · 64 × 64 · f64 | **12.724** | 15.509 | 14.958 | 13.455 | 12.844 | 15.148 | -0.2% |
| Matrix multiplication · 128 × 128 · 128 × 128 · f64 | 93.986 | 204.002 | 204.954 | 100.428 | **92.934** | 102.031 | +0.1% |
| Matrix multiplication · 256 × 256 · 256 × 256 · f64 | **730.869** | 910.935 | 904.363 | 969.514 | 738.492 | 766.815 | -0.4% |
| Matrix multiplication · 64 × 64 · 64 × 64 · f32 | 6.612 | 7.454 | 7.354 | **6.512** | 6.583 | 8.105 | +1.4% |
| Matrix multiplication · 128 × 128 · 128 × 128 · f32 | **45.836** | 74.7 | 73.688 | 51.556 | 46.186 | 49.011 | -1.2% |
| Matrix multiplication · 256 × 256 · 256 × 256 · f32 | **352.229** | 600.354 | 600.013 | 418.683 | 353.15 | 361.626 | -1.2% |
| Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64 | **3.126** | — | — | — | 3.247 | 4.188 | +1.6% |
| Matmul · 2 workers · reused output · 64 × 64 · 64 × 64 · f64 | 18.966 | — | — | — | — | — | +0.6% |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f64 | 56.566 | — | — | — | — | — | -1.0% |
| Matmul · 2 workers · reused output · 256 × 256 · 256 × 256 · f64 | 393.688 | — | — | — | — | — | +0.3% |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f32 | 34.084 | — | — | — | — | — | +6.5% |
| Cholesky factorization · 512 × 512 · f64 | 3886.384 | — | 7356.338 | **2584.958** | — | 6194.474 | -49.1% |
| Column-pivoted QR · 96 × 32 · f64 | 78.687 | — | 78.216 | **42.48** | — | — | +0.3% |
| Column-pivoted QR · 256 × 64 · f64 | 851.052 | — | 767.306 | **343.322** | — | — | -0.0% |
| Minimum-norm least squares · 64 × 32, rank 16, 4 RHS · f64 | **62.25599999999999** | — | 102.482 | — | — | 103.403 | -0.3% |
| Minimum-norm least squares · 32 × 64, rank 32, 4 RHS · f64 | **84.257** | — | 178.364 | — | — | 188.583 | -0.5% |
| Thin QR · 96 × 32 · f64 | 67.256 | — | 58.97 | **54.262** | — | 70.692 | +0.7% |
| Thin QR · 256 × 64 · f64 | 780.099 | — | 570.307 | **418.203** | — | 540.582 | +0.1% |
| Cholesky factorization · 64 × 64 · f64 | 24.777 | — | 20.91 | **14.097** | — | 18.174 | -0.8% |
| Cholesky factorization · 128 × 128 · f64 | 169.948 | — | 119.193 | **57.718** | — | 108.113 | +0.6% |
| Cholesky factorization · 256 × 256 · f64 | 686.484 | — | 788.336 | **313.717** | — | 1070.464 | -32.3% |
| Shared-variable backward · sum(X * X), 64 × 32; prepared graph · f64 | 10.41 | — | — | — | **4.939** | — | -1.2% |
| Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64 | **4.067** | — | — | — | 7.725 | — | +4.9% |
| Model inference · batched normalized/sliced MLP · f64 | **3.226** | — | — | — | 5.03 | — | -4.4% |
| Model recording + forward · batched normalized/sliced MLP · f64 | **4.098** | — | — | — | 6.201 | — | -1.7% |
| Model backward · prepared graph → fresh gradients · f64 | **5.099** | — | — | — | 20.689 | — | +2.6% |
| Parameter update · transactional SGD, fixed gradients · f64 | 0.802 | — | — | — | — | — | +0.0% |
| Coalesced tensor contraction · 16 × 32 × 8 · 32 × 8 × 16 · f64 | 63.067 | — | — | — | — | **11.331** | -1.0% |
| Row softmax · 256 × 64 · f64 | 119.533 | **110.877** | — | — | 115.145 | 132.998 | -0.0% |
| Create zeros · 64 × 64 · f32 | 0.32 | **0.25** | — | 0.311 | 0.301 | 0.742 | -0.3% |
| Create ones · 64 × 64 · f32 | 0.431 | **0.351** | — | 0.391 | 0.471 | 2.094 | +0.0% |

## Version comparison

12 of 34 workloads have paired evidence. Exploratory 95% intervals resample process pairs, not calls. No multiple-comparison adjustment.


### Version changes at a glance

The vertical line marks unchanged time. Left is faster; right is slower. Ranges are exploratory 95% intervals over fresh-process pairs. An interval crossing the line leaves direction unclear.

![Matrix multiplication. Current divided by baseline; below 1 is faster.](charts/paired-matmul-1.png)

![Explicit parallel execution. Current divided by baseline; below 1 is faster.](charts/paired-parallel-1.png)

![Decompositions & solvers. Current divided by baseline; below 1 is faster.](charts/paired-algebra-1.png)

| Workload | Current / baseline [95% interval] | Interpretation |
|---|---:|---|
| Independent matrix operands · 128 × 128 · 128 × 128 · f64 | 0.9995639040163377× [0.9955594464602143, 1.003105490635505] | Timing direction unclear |
| Independent matrix operands · 128 × 128 · 128 × 128 · f32 | 1.001321604991767× [0.9934858354794728, 1.0058788947677837] | Timing direction unclear |
| Matrix multiplication · 32 × 32 · 32 × 32 · f64 | — | No paired comparison |
| Matrix multiplication · 64 × 64 · 64 × 64 · f64 | — | No paired comparison |
| Matrix multiplication · 128 × 128 · 128 × 128 · f64 | 1.0009641651936365× [0.9952760966204274, 1.0073507188808772] | Timing direction unclear |
| Matrix multiplication · 256 × 256 · 256 × 256 · f64 | 1.001448242726027× [0.9985006212294634, 1.0042017208478298] | Timing direction unclear |
| Matrix multiplication · 64 × 64 · 64 × 64 · f32 | — | No paired comparison |
| Matrix multiplication · 128 × 128 · 128 × 128 · f32 | 1.0004347259053168× [0.9980513997445168, 1.0019336476416016] | Timing direction unclear |
| Matrix multiplication · 256 × 256 · 256 × 256 · f32 | 0.999291967226579× [0.9974740323731636, 1.0009405573441617] | Timing direction unclear |
| Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64 | — | No paired comparison |
| Matmul · 2 workers · reused output · 64 × 64 · 64 × 64 · f64 | — | No paired comparison |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f64 | 1.0058258237116304× [1.0012331107861963, 1.019572302490545] | Observed time increase |
| Matmul · 2 workers · reused output · 256 × 256 · 256 × 256 · f64 | 1.0005125276025437× [0.9991821128070911, 1.0050150348855331] | Timing direction unclear |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f32 | 1.0015813276827221× [0.9912324492979719, 1.061308636493387] | Timing direction unclear |
| Cholesky factorization · 512 × 512 · f64 | 0.5090629428301201× [0.5064715577820311, 0.5154443163666095] | Observed time reduction |
| Column-pivoted QR · 96 × 32 · f64 | — | No paired comparison |
| Column-pivoted QR · 256 × 64 · f64 | — | No paired comparison |
| Minimum-norm least squares · 64 × 32, rank 16, 4 RHS · f64 | — | No paired comparison |
| Minimum-norm least squares · 32 × 64, rank 32, 4 RHS · f64 | — | No paired comparison |
| Thin QR · 96 × 32 · f64 | — | No paired comparison |
| Thin QR · 256 × 64 · f64 | — | No paired comparison |
| Cholesky factorization · 64 × 64 · f64 | — | No paired comparison |
| Cholesky factorization · 128 × 128 · f64 | 1.0105549175378568× [1.0039669636470052, 1.0137466886231832] | Observed time increase |
| Cholesky factorization · 256 × 256 · f64 | 0.6790232533939629× [0.6733664516997802, 0.6847512849206271] | Observed time reduction |
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

Minimum-norm rows use pivoted QR plus scaled implicit RZ in Kernix; NumPy native lstsq; nalgebra native SVD/solve. Algorithm costs and rank policies differ. See original method for thresholds.

Numbers retain source precision; recorded digits do not imply equivalent physical measurement accuracy. Lowest observed medians are bold only when peers were measured. No aggregate winner or universal ranking is inferred.

[Original report and commentary](original-report.txt) · [Raw measurements](raw.json)

Measured source `18750693908110110d85d02d0f11af8d58bfae8b`. Run `runner-ubuntu-latest-40059566395`. Rendering does not change measurements.
