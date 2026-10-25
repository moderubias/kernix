# Kernix 0.11.0 · performance brief

Neoverse-N2 / Linux

**Per-call median, microseconds per operation. Lower is faster. All recorded digits retained.**

34 workloads · 116 observations

Baseline: baseline-ubuntu-24.04-arm-40059566395. Sequential run changes are descriptive; paired process evidence is shown separately.

[Open the visual report](report.html) · [Vector PDF](latency.pdf) · [Raw measurements](raw.json)

[![Performance brief preview](latency.png)](latency.pdf)

## Charts by workload

**Shorter bars mean less time.** Blue is Kernix; grey is another library; teal is an explicitly selected Kernix backend. Entries are sorted by the recorded median, with a separate zero-based scale per chart. These point observations do not establish a statistical ranking.

Expand a group to see every chart here on GitHub. Click a chart to enlarge it. The exact table below keeps every source digit.

<details>
<summary>Matrix multiplication · 10 workloads</summary>

**Independent matrix operands · 128 × 128 · 128 × 128 · f64**

![Independent matrix operands · 128 × 128 · 128 × 128 · f64. Shorter bars mean less time.](charts/matmul_ab_128_f64.png)

**Independent matrix operands · 128 × 128 · 128 × 128 · f32**

![Independent matrix operands · 128 × 128 · 128 × 128 · f32. Shorter bars mean less time.](charts/matmul_ab_128_f32.png)

**Matrix multiplication · 32 × 32 · 32 × 32 · f64**

![Matrix multiplication · 32 × 32 · 32 × 32 · f64. Shorter bars mean less time.](charts/matmul_32_f64.png)

**Matrix multiplication · 64 × 64 · 64 × 64 · f64**

![Matrix multiplication · 64 × 64 · 64 × 64 · f64. Shorter bars mean less time.](charts/matmul_64_f64.png)

**Matrix multiplication · 128 × 128 · 128 × 128 · f64**

![Matrix multiplication · 128 × 128 · 128 × 128 · f64. Shorter bars mean less time.](charts/matmul_128_f64.png)

**Matrix multiplication · 256 × 256 · 256 × 256 · f64**

![Matrix multiplication · 256 × 256 · 256 × 256 · f64. Shorter bars mean less time.](charts/matmul_256_f64.png)

**Matrix multiplication · 64 × 64 · 64 × 64 · f32**

![Matrix multiplication · 64 × 64 · 64 × 64 · f32. Shorter bars mean less time.](charts/matmul_64_f32.png)

**Matrix multiplication · 128 × 128 · 128 × 128 · f32**

![Matrix multiplication · 128 × 128 · 128 × 128 · f32. Shorter bars mean less time.](charts/matmul_128_f32.png)

**Matrix multiplication · 256 × 256 · 256 × 256 · f32**

![Matrix multiplication · 256 × 256 · 256 × 256 · f32. Shorter bars mean less time.](charts/matmul_256_f32.png)

**Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64**

![Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64. Shorter bars mean less time.](charts/matmul_batched_4x16x32x16_f64.png)

</details>

<details>
<summary>Explicit parallel execution · 4 workloads</summary>

**Matmul · 2 workers · reused output · 64 × 64 · 64 × 64 · f64**

![Matmul · 2 workers · reused output · 64 × 64 · 64 × 64 · f64. Shorter bars mean less time.](charts/parallel_matmul_into_64_f64.png)

**Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f64**

![Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f64. Shorter bars mean less time.](charts/parallel_matmul_into_128_f64.png)

**Matmul · 2 workers · reused output · 256 × 256 · 256 × 256 · f64**

![Matmul · 2 workers · reused output · 256 × 256 · 256 × 256 · f64. Shorter bars mean less time.](charts/parallel_matmul_into_256_f64.png)

**Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f32**

![Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f32. Shorter bars mean less time.](charts/parallel_matmul_into_128_f32.png)

</details>

<details>
<summary>Decompositions & solvers · 10 workloads</summary>

**Cholesky factorization · 512 × 512 · f64**

![Cholesky factorization · 512 × 512 · f64. Shorter bars mean less time.](charts/cholesky_factor_512_f64.png)

**Column-pivoted QR · 96 × 32 · f64**

![Column-pivoted QR · 96 × 32 · f64. Shorter bars mean less time.](charts/pivoted_qr_96x32_f64.png)

**Column-pivoted QR · 256 × 64 · f64**

![Column-pivoted QR · 256 × 64 · f64. Shorter bars mean less time.](charts/pivoted_qr_256x64_f64.png)

**Minimum-norm least squares · 64 × 32, rank 16, 4 RHS · f64**

![Minimum-norm least squares · 64 × 32, rank 16, 4 RHS · f64. Shorter bars mean less time.](charts/min_norm_64x32_rank16_f64.png)

**Minimum-norm least squares · 32 × 64, rank 32, 4 RHS · f64**

![Minimum-norm least squares · 32 × 64, rank 32, 4 RHS · f64. Shorter bars mean less time.](charts/min_norm_32x64_rank32_f64.png)

**Thin QR · 96 × 32 · f64**

![Thin QR · 96 × 32 · f64. Shorter bars mean less time.](charts/qr_96x32_f64.png)

**Thin QR · 256 × 64 · f64**

![Thin QR · 256 × 64 · f64. Shorter bars mean less time.](charts/qr_256x64_f64.png)

**Cholesky factorization · 64 × 64 · f64**

![Cholesky factorization · 64 × 64 · f64. Shorter bars mean less time.](charts/cholesky_factor_64_f64.png)

**Cholesky factorization · 128 × 128 · f64**

![Cholesky factorization · 128 × 128 · f64. Shorter bars mean less time.](charts/cholesky_factor_128_f64.png)

**Cholesky factorization · 256 × 256 · f64**

![Cholesky factorization · 256 × 256 · f64. Shorter bars mean less time.](charts/cholesky_factor_256_f64.png)

</details>

<details>
<summary>Training & autodiff · 6 workloads</summary>

**Shared-variable backward · sum(X * X), 64 × 32; prepared graph · f64**

![Shared-variable backward · sum(X * X), 64 × 32; prepared graph · f64. Shorter bars mean less time.](charts/training_shared_backward_64x32_f64.png)

**Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64**

![Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64. Shorter bars mean less time.](charts/matmul_sum_backward_32x16x8_f64.png)

**Model inference · batched normalized/sliced MLP · f64**

![Model inference · batched normalized/sliced MLP · f64. Shorter bars mean less time.](charts/training_forward_2x4x8x4_f64.png)

**Model recording + forward · batched normalized/sliced MLP · f64**

![Model recording + forward · batched normalized/sliced MLP · f64. Shorter bars mean less time.](charts/training_record_2x4x8x4_f64.png)

**Model backward · prepared graph → fresh gradients · f64**

![Model backward · prepared graph → fresh gradients · f64. Shorter bars mean less time.](charts/training_backward_2x4x8x4_f64.png)

**Parameter update · transactional SGD, fixed gradients · f64**

![Parameter update · transactional SGD, fixed gradients · f64. Shorter bars mean less time.](charts/training_sgd_2x4x8x4_f64.png)

</details>

<details>
<summary>Tensor contractions · 1 workloads</summary>

**Coalesced tensor contraction · 16 × 32 × 8 · 32 × 8 × 16 · f64**

![Coalesced tensor contraction · 16 × 32 × 8 · 32 × 8 × 16 · f64. Shorter bars mean less time.](charts/tensordot_16x32x8_32x8x16_f64.png)

</details>

<details>
<summary>Statistics & activations · 1 workloads</summary>

**Row softmax · 256 × 64 · f64**

![Row softmax · 256 × 64 · f64. Shorter bars mean less time.](charts/softmax_256x64_f64.png)

</details>

<details>
<summary>Tensor construction · 2 workloads</summary>

**Create zeros · 64 × 64 · f32**

![Create zeros · 64 × 64 · f32. Shorter bars mean less time.](charts/create_zeros_64x64_f32.png)

**Create ones · 64 × 64 · f32**

![Create ones · 64 × 64 · f32. Shorter bars mean less time.](charts/create_ones_64x64_f32.png)

</details>

## Exact latency table

All values below are microseconds per operation. **Lower is faster.** Missing entries were not measured.

| Workload | Kernix | ndarray | nalgebra | faer | Candle | NumPy | Sequential run change |
|---|---:|---:|---:|---:|---:|---:|---:|
| Independent matrix operands · 128 × 128 · 128 × 128 · f64 | **169.458** | — | — | — | — | 196.488 | -19.9% |
| Independent matrix operands · 128 × 128 · 128 × 128 · f32 | **86.965** | — | — | — | — | 99.957 | -8.1% |
| Matrix multiplication · 32 × 32 · 32 × 32 · f64 | 3.248 | **3.048** | 3.224 | 3.504 | 3.44 | 8.207 | +0.7% |
| Matrix multiplication · 64 × 64 · 64 × 64 · f64 | 22.247 | **21.808** | 22.471 | 22.647 | 22.543 | 31.111 | +0.3% |
| Matrix multiplication · 128 × 128 · 128 × 128 · f64 | **167.786** | 208.28 | 208.808 | 169.098 | 168.273 | 194.28 | -20.5% |
| Matrix multiplication · 256 × 256 · 256 × 256 · f64 | 1332.074 | 1332.212 | 1343.02 | 1551.065 | **1330.557** | 1475.672 | -0.5% |
| Matrix multiplication · 64 × 64 · 64 × 64 · f32 | 11.751 | **11.272** | 11.279 | 12.303 | 11.968 | 20.112 | +2.1% |
| Matrix multiplication · 128 × 128 · 128 × 128 · f32 | **85.557** | 93.285 | 93.509 | 87.141 | 85.788 | 99.772 | -9.1% |
| Matrix multiplication · 256 × 256 · 256 × 256 · f32 | **665.766** | 767.138 | 766.243 | 678.132 | 757.955 | 728.777 | -13.3% |
| Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64 | **3.88** | — | — | — | 4.416 | 12.999 | +1.5% |
| Matmul · 2 workers · reused output · 64 × 64 · 64 × 64 · f64 | 18.887 | — | — | — | — | — | +10.0% |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f64 | 92.685 | — | — | — | — | — | -5.3% |
| Matmul · 2 workers · reused output · 256 × 256 · 256 × 256 · f64 | 697.427 | — | — | — | — | — | -17.1% |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f32 | 50.829 | — | — | — | — | — | +1.8% |
| Cholesky factorization · 512 × 512 · f64 | 4366.255 | — | 9204.768 | **2673.472** | — | 4027.441 | -71.5% |
| Column-pivoted QR · 96 × 32 · f64 | 88.533 | — | 114.244 | **62.11800000000001** | — | — | -0.1% |
| Column-pivoted QR · 256 × 64 · f64 | 793.153 | — | 1150.714 | **473.974** | — | — | +0.3% |
| Minimum-norm least squares · 64 × 32, rank 16, 4 RHS · f64 | **72.397** | — | 116.964 | — | — | 93.901 | -0.3% |
| Minimum-norm least squares · 32 × 64, rank 32, 4 RHS · f64 | **96.173** | — | 218.537 | — | — | 176.609 | -0.1% |
| Thin QR · 96 × 32 · f64 | 72.093 | — | **61.30900000000001** | 65.373 | — | 66.293 | +0.5% |
| Thin QR · 256 × 64 · f64 | 695.948 | — | 586.681 | **447.455** | — | 573.546 | +0.2% |
| Cholesky factorization · 64 × 64 · f64 | 27.703 | — | 20.311 | **13.255** | — | 18.039 | +1.2% |
| Cholesky factorization · 128 × 128 · f64 | 280.005 | — | 128.682 | **64.95000000000001** | — | 86.549 | -0.8% |
| Cholesky factorization · 256 × 256 · f64 | 771.122 | — | 1035.745 | **407.64** | — | 767.587 | -61.2% |
| Shared-variable backward · sum(X * X), 64 × 32; prepared graph · f64 | 8.368 | — | — | — | **5.007** | — | +0.3% |
| Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64 | **3.496** | — | — | — | 7.288 | — | -0.4% |
| Model inference · batched normalized/sliced MLP · f64 | **3.064** | — | — | — | 5.287 | — | -0.7% |
| Model recording + forward · batched normalized/sliced MLP · f64 | **3.8480000000000005** | — | — | — | 7.64 | — | +0.2% |
| Model backward · prepared graph → fresh gradients · f64 | **4.175** | — | — | — | 21.136 | — | -0.2% |
| Parameter update · transactional SGD, fixed gradients · f64 | 0.72 | — | — | — | — | — | -3.2% |
| Coalesced tensor contraction · 16 × 32 × 8 · 32 × 8 × 16 · f64 | 46.742 | — | — | — | — | **16.472** | +0.1% |
| Row softmax · 256 × 64 · f64 | 113.236 | **97.389** | — | — | 110.555 | 133.851 | +0.2% |
| Create zeros · 64 × 64 · f32 | 0.2 | **0.136** | — | 0.36 | 0.232 | 0.512 | +0.0% |
| Create ones · 64 × 64 · f32 | 0.248 | **0.208** | — | 0.23999999999999997 | 0.344 | 1.52 | -3.1% |

## Version comparison

12 of 34 workloads have paired evidence. Exploratory 95% intervals resample process pairs, not calls. No multiple-comparison adjustment.


### Version changes at a glance

The vertical line marks unchanged time. Left is faster; right is slower. Ranges are exploratory 95% intervals over fresh-process pairs. An interval crossing the line leaves direction unclear.

![Matrix multiplication. Current divided by baseline; below 1 is faster.](charts/paired-matmul-1.png)

![Explicit parallel execution. Current divided by baseline; below 1 is faster.](charts/paired-parallel-1.png)

![Decompositions & solvers. Current divided by baseline; below 1 is faster.](charts/paired-algebra-1.png)

| Workload | Current / baseline [95% interval] | Interpretation |
|---|---:|---|
| Independent matrix operands · 128 × 128 · 128 × 128 · f64 | 0.7968843618933493× [0.7850777114855892, 0.8007173985605017] | Observed time reduction |
| Independent matrix operands · 128 × 128 · 128 × 128 · f32 | 0.9059853428794695× [0.9014599690978463, 0.9103167103740432] | Observed time reduction |
| Matrix multiplication · 32 × 32 · 32 × 32 · f64 | — | No paired comparison |
| Matrix multiplication · 64 × 64 · 64 × 64 · f64 | — | No paired comparison |
| Matrix multiplication · 128 × 128 · 128 × 128 · f64 | 0.8039757672973089× [0.7961028439858441, 0.8075187028214404] | Observed time reduction |
| Matrix multiplication · 256 × 256 · 256 × 256 · f64 | 0.9937348734791259× [0.9925390619170559, 0.9968513537905461] | Observed time reduction |
| Matrix multiplication · 64 × 64 · 64 × 64 · f32 | — | No paired comparison |
| Matrix multiplication · 128 × 128 · 128 × 128 · f32 | 0.9104404550339998× [0.9083013425523341, 0.9116032637490727] | Observed time reduction |
| Matrix multiplication · 256 × 256 · 256 × 256 · f32 | 0.8616485177090156× [0.8596499666554435, 0.8637385121301779] | Observed time reduction |
| Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64 | — | No paired comparison |
| Matmul · 2 workers · reused output · 64 × 64 · 64 × 64 · f64 | — | No paired comparison |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f64 | 0.9447061485610845× [0.9284550919482365, 0.9752992394224139] | Observed time reduction |
| Matmul · 2 workers · reused output · 256 × 256 · 256 × 256 · f64 | 0.8347739732517124× [0.8272963184491992, 0.8406043323345531] | Observed time reduction |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f32 | 0.9857735743644997× [0.981228974660111, 0.9936317176395433] | Observed time reduction |
| Cholesky factorization · 512 × 512 · f64 | 0.2852289637520951× [0.2845246063012664, 0.2860102284176894] | Observed time reduction |
| Column-pivoted QR · 96 × 32 · f64 | — | No paired comparison |
| Column-pivoted QR · 256 × 64 · f64 | — | No paired comparison |
| Minimum-norm least squares · 64 × 32, rank 16, 4 RHS · f64 | — | No paired comparison |
| Minimum-norm least squares · 32 × 64, rank 32, 4 RHS · f64 | — | No paired comparison |
| Thin QR · 96 × 32 · f64 | — | No paired comparison |
| Thin QR · 256 × 64 · f64 | — | No paired comparison |
| Cholesky factorization · 64 × 64 · f64 | — | No paired comparison |
| Cholesky factorization · 128 × 128 · f64 | 1.00357375150991× [1.0008964317729707, 1.0050536592369261] | Observed time increase |
| Cholesky factorization · 256 × 256 · f64 | 0.3869933969656744× [0.3851419972722835, 0.38939434067885376] | Observed time reduction |
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

Measured source `18750693908110110d85d02d0f11af8d58bfae8b`. Run `runner-ubuntu-24.04-arm-40059566395`. Rendering does not change measurements.
