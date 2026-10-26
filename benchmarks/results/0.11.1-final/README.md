# Kernix 0.11.1 · performance brief

12th Gen Intel(R) Core(TM) i3-1215U / Linux

**Per-call median, microseconds per operation. Lower is faster. All recorded digits retained.**

35 workloads · 119 observations

Baseline: 0.11.0-registry-0111. Sequential run changes are descriptive; paired process evidence is shown separately.

[Interactive HTML (open locally)](report.html) · [Vector PDF](latency.pdf) · [Raw measurements](raw.json)

[![Performance brief preview](latency.png)](latency.pdf)

## Charts by workload

**Shorter bars mean less time.** Blue is Kernix; grey is a comparison implementation; teal is an explicitly selected Kernix backend. Entries are sorted by the recorded median, with a separate zero-based scale per chart. These point observations do not establish a statistical ranking.

Expand a group to see every chart here on GitHub. Click a chart to enlarge it. The exact table below keeps every source digit.

<details>
<summary>Matrix multiplication · 11 workloads</summary>

**Independent matrix operands · 128 × 128 · 128 × 128 · f64**

![Independent matrix operands · 128 × 128 · 128 × 128 · f64. Shorter bars mean less time.](charts/matmul_ab_128_f64.png)

Kernix: **69.083 µs**. Kernix took ≈ 5.5% less time than NumPy in this run.

**Independent matrix operands · 128 × 128 · 128 × 128 · f32**

![Independent matrix operands · 128 × 128 · 128 × 128 · f32. Shorter bars mean less time.](charts/matmul_ab_128_f32.png)

Kernix: **34.252 µs**. Kernix took ≈ 12.8% less time than NumPy in this run.

**Matrix multiplication · 32 × 32 · 32 × 32 · f64**

![Matrix multiplication · 32 × 32 · 32 × 32 · f64. Shorter bars mean less time.](charts/matmul_32_f64.png)

Kernix: **1.641 µs**. Kernix took ≈ 12.5% more time than Candle in this run.

**Matrix multiplication · 64 × 64 · 64 × 64 · f64**

![Matrix multiplication · 64 × 64 · 64 × 64 · f64. Shorter bars mean less time.](charts/matmul_64_f64.png)

Kernix: **9.146 µs**. Kernix took ≈ 1.2% less time than Candle in this run.

**Matrix multiplication · 128 × 128 · 128 × 128 · f64**

![Matrix multiplication · 128 × 128 · 128 × 128 · f64. Shorter bars mean less time.](charts/matmul_128_f64.png)

Kernix: **70.637 µs**. Kernix took ≈ 2.8% more time than Candle in this run.

**Matrix multiplication · 256 × 256 · 256 × 256 · f64**

![Matrix multiplication · 256 × 256 · 256 × 256 · f64. Shorter bars mean less time.](charts/matmul_256_f64.png)

Kernix: **573.171 µs**. Kernix took ≈ 4.6% more time than Candle in this run.

**Matrix multiplication · 64 × 64 · 64 × 64 · f32**

![Matrix multiplication · 64 × 64 · 64 × 64 · f32. Shorter bars mean less time.](charts/matmul_64_f32.png)

Kernix: **4.525 µs**. Kernix took ≈ 0.7% less time than Candle in this run.

**Matrix multiplication · 128 × 128 · 128 × 128 · f32**

![Matrix multiplication · 128 × 128 · 128 × 128 · f32. Shorter bars mean less time.](charts/matmul_128_f32.png)

Kernix: **34.201 µs**. Kernix took ≈ 0.5% less time than Candle in this run.

**Matrix multiplication · 256 × 256 · 256 × 256 · f32**

![Matrix multiplication · 256 × 256 · 256 × 256 · f32. Shorter bars mean less time.](charts/matmul_256_f32.png)

Kernix: **278.325 µs**. Kernix took ≈ 3.3% more time than Candle in this run.

**Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64**

![Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64. Shorter bars mean less time.](charts/matmul_batched_4x16x32x16_f64.png)

Kernix: **1.406 µs**. Kernix took ≈ 27.5% less time than Candle in this run.

**Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f32**

![Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f32. Shorter bars mean less time.](charts/matmul_batched_4x16x32x16_f32.png)

Kernix: **0.926 µs**. Kernix took ≈ 32.8% less time than Candle in this run.

</details>

<details>
<summary>Explicit parallel execution · 4 workloads</summary>

**Matmul · 2 workers · reused output · 64 × 64 · 64 × 64 · f64**

![Matmul · 2 workers · reused output · 64 × 64 · 64 × 64 · f64. Shorter bars mean less time.](charts/parallel_matmul_into_64_f64.png)

Kernix: **9.501 µs**. No external comparison with Kernix was measured for this workload.

**Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f64**

![Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f64. Shorter bars mean less time.](charts/parallel_matmul_into_128_f64.png)

Kernix: **41.479 µs**. No external comparison with Kernix was measured for this workload.

**Matmul · 2 workers · reused output · 256 × 256 · 256 × 256 · f64**

![Matmul · 2 workers · reused output · 256 × 256 · 256 × 256 · f64. Shorter bars mean less time.](charts/parallel_matmul_into_256_f64.png)

Kernix: **297.428 µs**. No external comparison with Kernix was measured for this workload.

**Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f32**

![Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f32. Shorter bars mean less time.](charts/parallel_matmul_into_128_f32.png)

Kernix: **22.18 µs**. No external comparison with Kernix was measured for this workload.

</details>

<details>
<summary>Decompositions & solvers · 10 workloads</summary>

**Cholesky factorization · 512 × 512 · f64**

![Cholesky factorization · 512 × 512 · f64. Shorter bars mean less time.](charts/cholesky_factor_512_f64.png)

Kernix: **2875.84 µs**. Kernix took ≈ 112.1% more time than faer in this run.

**Column-pivoted QR · 96 × 32 · f64**

![Column-pivoted QR · 96 × 32 · f64. Shorter bars mean less time.](charts/pivoted_qr_96x32_f64.png)

Kernix: **56.43 µs**. Kernix took ≈ 21.3% less time than faer in this run.

**Column-pivoted QR · 256 × 64 · f64**

![Column-pivoted QR · 256 × 64 · f64. Shorter bars mean less time.](charts/pivoted_qr_256x64_f64.png)

Kernix: **512.353 µs**. Kernix took ≈ 13.2% more time than faer in this run.

**Minimum-norm least squares · 64 × 32, rank 16, 4 RHS · f64**

![Minimum-norm least squares · 64 × 32, rank 16, 4 RHS · f64. Shorter bars mean less time.](charts/min_norm_64x32_rank16_f64.png)

Kernix: **44.321 µs**. Kernix took ≈ 33.7% less time than NumPy in this run.

**Minimum-norm least squares · 32 × 64, rank 32, 4 RHS · f64**

![Minimum-norm least squares · 32 × 64, rank 32, 4 RHS · f64. Shorter bars mean less time.](charts/min_norm_32x64_rank32_f64.png)

Kernix: **56.68 µs**. Kernix took ≈ 50.7% less time than NumPy in this run.

**Thin QR · 96 × 32 · f64**

![Thin QR · 96 × 32 · f64. Shorter bars mean less time.](charts/qr_96x32_f64.png)

Kernix: **43.392 µs**. Kernix took ≈ 26.2% more time than nalgebra in this run.

**Thin QR · 256 × 64 · f64**

![Thin QR · 256 × 64 · f64. Shorter bars mean less time.](charts/qr_256x64_f64.png)

Kernix: **469.928 µs**. Kernix took ≈ 38.7% more time than NumPy in this run.

**Cholesky factorization · 64 × 64 · f64**

![Cholesky factorization · 64 × 64 · f64. Shorter bars mean less time.](charts/cholesky_factor_64_f64.png)

Kernix: **18.925 µs**. Kernix took ≈ 124.3% more time than faer in this run.

**Cholesky factorization · 128 × 128 · f64**

![Cholesky factorization · 128 × 128 · f64. Shorter bars mean less time.](charts/cholesky_factor_128_f64.png)

Kernix: **110.399 µs**. Kernix took ≈ 194.0% more time than faer in this run.

**Cholesky factorization · 256 × 256 · f64**

![Cholesky factorization · 256 × 256 · f64. Shorter bars mean less time.](charts/cholesky_factor_256_f64.png)

Kernix: **495.69800000000006 µs**. Kernix took ≈ 157.2% more time than faer in this run.

</details>

<details>
<summary>Training & autodiff · 6 workloads</summary>

**Shared-variable backward · sum(X * X), 64 × 32; prepared graph · f64**

![Shared-variable backward · sum(X * X), 64 × 32; prepared graph · f64. Shorter bars mean less time.](charts/training_shared_backward_64x32_f64.png)

Kernix: **6.702 µs**. Kernix took ≈ 76.1% more time than Candle in this run.

**Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64**

![Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64. Shorter bars mean less time.](charts/matmul_sum_backward_32x16x8_f64.png)

Kernix: **2.228 µs**. Kernix took ≈ 44.1% less time than Candle in this run.

**Model inference · batched normalized/sliced MLP · f64**

![Model inference · batched normalized/sliced MLP · f64. Shorter bars mean less time.](charts/training_forward_2x4x8x4_f64.png)

Kernix: **2.151 µs**. Kernix took ≈ 31.8% less time than Candle in this run.

**Model recording + forward · batched normalized/sliced MLP · f64**

![Model recording + forward · batched normalized/sliced MLP · f64. Shorter bars mean less time.](charts/training_record_2x4x8x4_f64.png)

Kernix: **2.945 µs**. Kernix took ≈ 43.1% less time than Candle in this run.

**Model backward · prepared graph → fresh gradients · f64**

![Model backward · prepared graph → fresh gradients · f64. Shorter bars mean less time.](charts/training_backward_2x4x8x4_f64.png)

Kernix: **2.973 µs**. Kernix took ≈ 79.6% less time than Candle in this run.

**Parameter update · transactional SGD, fixed gradients · f64**

![Parameter update · transactional SGD, fixed gradients · f64. Shorter bars mean less time.](charts/training_sgd_2x4x8x4_f64.png)

Kernix: **0.515 µs**. No external comparison with Kernix was measured for this workload.

</details>

<details>
<summary>Tensor contractions · 1 workloads</summary>

**Coalesced tensor contraction · 16 × 32 × 8 · 32 × 8 × 16 · f64**

![Coalesced tensor contraction · 16 × 32 × 8 · 32 × 8 × 16 · f64. Shorter bars mean less time.](charts/tensordot_16x32x8_32x8x16_f64.png)

Kernix: **36.005 µs**. Kernix took ≈ 365.0% more time than NumPy in this run.

</details>

<details>
<summary>Statistics & activations · 1 workloads</summary>

**Row softmax · 256 × 64 · f64**

![Row softmax · 256 × 64 · f64. Shorter bars mean less time.](charts/softmax_256x64_f64.png)

Kernix: **81.343 µs**. Kernix took ≈ 18.8% more time than ndarray in this run.

</details>

<details>
<summary>Tensor construction · 2 workloads</summary>

**Create zeros · 64 × 64 · f32**

![Create zeros · 64 × 64 · f32. Shorter bars mean less time.](charts/create_zeros_64x64_f32.png)

Kernix: **0.141 µs**. Kernix took ≈ 56.7% more time than ndarray in this run.

**Create ones · 64 × 64 · f32**

![Create ones · 64 × 64 · f32. Shorter bars mean less time.](charts/create_ones_64x64_f32.png)

Kernix: **0.212 µs**. Kernix took ≈ 51.4% more time than ndarray in this run.

</details>

## Exact latency table

All values below are microseconds per operation. **Lower is faster.** Missing entries were not measured.

| Workload | Kernix | ndarray | nalgebra | faer | Candle | NumPy | Sequential run change |
|---|---:|---:|---:|---:|---:|---:|---:|
| Independent matrix operands · 128 × 128 · 128 × 128 · f64 | **69.083** | — | — | — | — | 73.137 | -2.2% |
| Independent matrix operands · 128 × 128 · 128 × 128 · f32 | **34.252** | — | — | — | — | 39.283 | -6.9% |
| Matrix multiplication · 32 × 32 · 32 × 32 · f64 | 1.641 | 1.486 | 1.625 | 1.569 | **1.459** | 2.548 | -6.3% |
| Matrix multiplication · 64 × 64 · 64 × 64 · f64 | **9.146** | 10.907 | 11.253 | 10.277 | 9.258 | 11.889 | -8.8% |
| Matrix multiplication · 128 × 128 · 128 × 128 · f64 | 70.637 | 110.826 | 110.851 | 76.996 | **68.719** | 72.935 | -4.0% |
| Matrix multiplication · 256 × 256 · 256 × 256 · f64 | 573.171 | 618.725 | 617.727 | 717.663 | **547.945** | 601.651 | +0.7% |
| Matrix multiplication · 64 × 64 · 64 × 64 · f32 | **4.525** | 5.662 | 6.065 | 5.358 | 4.557 | 5.772 | -6.0% |
| Matrix multiplication · 128 × 128 · 128 × 128 · f32 | **34.201** | 53.355 | 49.338 | 39.162 | 34.36 | 36.504 | -12.5% |
| Matrix multiplication · 256 × 256 · 256 × 256 · f32 | 278.325 | 407.805 | 387.295 | 302.741 | **269.307** | 280.075 | -1.7% |
| Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64 | **1.406** | — | — | — | 1.94 | 2.779 | -27.8% |
| Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f32 | **0.926** | — | — | — | 1.377 | 2.031 | -33.3% |
| Matmul · 2 workers · reused output · 64 × 64 · 64 × 64 · f64 | 9.501 | — | — | — | — | — | -3.0% |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f64 | 41.479 | — | — | — | — | — | -1.8% |
| Matmul · 2 workers · reused output · 256 × 256 · 256 × 256 · f64 | 297.428 | — | — | — | — | — | +0.7% |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f32 | 22.18 | — | — | — | — | — | -51.3% |
| Cholesky factorization · 512 × 512 · f64 | 2875.84 | — | 5261.263 | **1355.642** | — | 3360.38 | +0.3% |
| Column-pivoted QR · 96 × 32 · f64 | **56.43** | — | 76.393 | 71.704 | — | — | -3.6% |
| Column-pivoted QR · 256 × 64 · f64 | 512.353 | — | 775.494 | **452.78** | — | — | -5.5% |
| Minimum-norm least squares · 64 × 32, rank 16, 4 RHS · f64 | **44.321** | — | 77.227 | — | — | 66.814 | +1.0% |
| Minimum-norm least squares · 32 × 64, rank 32, 4 RHS · f64 | **56.68** | — | 126.02300000000001 | — | — | 114.961 | -5.2% |
| Thin QR · 96 × 32 · f64 | 43.392 | — | **34.387** | 62.48499999999999 | — | 36.755 | -0.2% |
| Thin QR · 256 × 64 · f64 | 469.928 | — | 374.716 | 394.945 | — | **338.736** | -0.5% |
| Cholesky factorization · 64 × 64 · f64 | 18.925 | — | 13.118 | **8.439** | — | 10.374 | +0.5% |
| Cholesky factorization · 128 × 128 · f64 | 110.399 | — | 89.792 | **37.553** | — | 73.702 | -2.4% |
| Cholesky factorization · 256 × 256 · f64 | 495.69800000000006 | — | 639.934 | **192.757** | — | 516.323 | -1.6% |
| Shared-variable backward · sum(X * X), 64 × 32; prepared graph · f64 | 6.702 | — | — | — | **3.806** | — | +9.5% |
| Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64 | **2.228** | — | — | — | 3.983 | — | -6.0% |
| Model inference · batched normalized/sliced MLP · f64 | **2.151** | — | — | — | 3.152 | — | +4.1% |
| Model recording + forward · batched normalized/sliced MLP · f64 | **2.945** | — | — | — | 5.18 | — | +9.6% |
| Model backward · prepared graph → fresh gradients · f64 | **2.973** | — | — | — | 14.581 | — | -1.9% |
| Parameter update · transactional SGD, fixed gradients · f64 | 0.515 | — | — | — | — | — | -8.0% |
| Coalesced tensor contraction · 16 × 32 × 8 · 32 × 8 × 16 · f64 | 36.005 | — | — | — | — | **7.743** | +7.5% |
| Row softmax · 256 × 64 · f64 | 81.343 | **68.484** | — | — | 81.85 | 97.519 | -5.1% |
| Create zeros · 64 × 64 · f32 | 0.141 | **0.09** | — | 0.144 | 0.167 | 0.346 | -9.0% |
| Create ones · 64 × 64 · f32 | 0.212 | **0.14** | — | 0.182 | 0.236 | 1.213 | +15.2% |

## Version comparison

16 of 35 workloads have paired evidence. Exploratory 95% intervals resample process pairs, not calls. No multiple-comparison adjustment.


### Version changes at a glance

The vertical line marks unchanged time. Left is faster; right is slower. Ranges are exploratory 95% intervals over fresh-process pairs. An interval crossing the line leaves direction unclear.

![Matrix multiplication. Current divided by baseline; below 1 is faster.](charts/paired-matmul-1.png)

![Matrix multiplication. Current divided by baseline; below 1 is faster.](charts/paired-matmul-2.png)

![Explicit parallel execution. Current divided by baseline; below 1 is faster.](charts/paired-parallel-1.png)

![Decompositions & solvers. Current divided by baseline; below 1 is faster.](charts/paired-algebra-1.png)

![Training & autodiff. Current divided by baseline; below 1 is faster.](charts/paired-training-1.png)

| Workload | Current / baseline [95% interval] | Interpretation |
|---|---:|---|
| Independent matrix operands · 128 × 128 · 128 × 128 · f64 | 0.9973845772650938× [0.9513612643104681, 1.0198856125019133] | Timing direction unclear |
| Independent matrix operands · 128 × 128 · 128 × 128 · f32 | 0.9919104759336659× [0.971837475943946, 1.0259382052210737] | Timing direction unclear |
| Matrix multiplication · 32 × 32 · 32 × 32 · f64 | 0.9904882688649335× [0.9548349739432542, 1.0424050632911392] | Timing direction unclear |
| Matrix multiplication · 64 × 64 · 64 × 64 · f64 | — | No paired comparison |
| Matrix multiplication · 128 × 128 · 128 × 128 · f64 | 1.012244500410469× [0.996820129959906, 1.0277707380187007] | Timing direction unclear |
| Matrix multiplication · 256 × 256 · 256 × 256 · f64 | 0.9959523669853934× [0.9901368615535494, 1.0083672209329557] | Timing direction unclear |
| Matrix multiplication · 64 × 64 · 64 × 64 · f32 | — | No paired comparison |
| Matrix multiplication · 128 × 128 · 128 × 128 · f32 | 1.0019341832842976× [0.9540607054963085, 1.0580849852211405] | Timing direction unclear |
| Matrix multiplication · 256 × 256 · 256 × 256 · f32 | 0.9991795672844394× [0.9917177400090559, 1.0197192236299086] | Timing direction unclear |
| Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64 | 0.7574906367041199× [0.7263590680676107, 0.8196382428940568] | Observed time reduction |
| Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f32 | 0.7099767981438515× [0.6951219512195121, 0.7725225225225225] | Observed time reduction |
| Matmul · 2 workers · reused output · 64 × 64 · 64 × 64 · f64 | — | No paired comparison |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f64 | 0.9707103267549454× [0.5377926771777277, 0.9987317073170732] | Observed time reduction |
| Matmul · 2 workers · reused output · 256 × 256 · 256 × 256 · f64 | 0.9909823528068854× [0.9644791506637074, 1.0365048026342756] | Timing direction unclear |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f32 | 0.9912084047650446× [0.5711984041736996, 1.7238628349788434] | Timing direction unclear |
| Cholesky factorization · 512 × 512 · f64 | 1.0037438332284923× [0.9868867083576638, 1.0199812625802072] | Timing direction unclear |
| Column-pivoted QR · 96 × 32 · f64 | — | No paired comparison |
| Column-pivoted QR · 256 × 64 · f64 | — | No paired comparison |
| Minimum-norm least squares · 64 × 32, rank 16, 4 RHS · f64 | — | No paired comparison |
| Minimum-norm least squares · 32 × 64, rank 32, 4 RHS · f64 | — | No paired comparison |
| Thin QR · 96 × 32 · f64 | — | No paired comparison |
| Thin QR · 256 × 64 · f64 | — | No paired comparison |
| Cholesky factorization · 64 × 64 · f64 | — | No paired comparison |
| Cholesky factorization · 128 × 128 · f64 | 0.9840000693012699× [0.9571668921497207, 1.0007533641606483] | Timing direction unclear |
| Cholesky factorization · 256 × 256 · f64 | 0.9955355668450756× [0.9890255559008289, 1.0107981826317975] | Timing direction unclear |
| Shared-variable backward · sum(X * X), 64 × 32; prepared graph · f64 | — | No paired comparison |
| Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64 | 0.9321063394683027× [0.9020817270624518, 0.9764801297648013] | Observed time reduction |
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

Measured source `b2c458d113aa23aa24e93fbfdabd55b55e3e6831`. Run `0.11.1-final`. Rendering does not change measurements.
