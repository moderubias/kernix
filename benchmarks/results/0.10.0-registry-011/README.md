# Kernix 0.10.0 · CPU benchmarks

12th Gen Intel(R) Core(TM) i3-1215U · warmed public API calls · conditions per workload

**Microseconds per call, median. Lower is faster. Bold marks a lowest observed median only among at least two measured libraries.**

No previous-version baseline is paired with this view.

| Workload | Kernix | ndarray | nalgebra | faer | Candle | NumPy |
|---|---:|---:|---:|---:|---:|---:|
| Independent matrix operands · 128 × 128 · 128 × 128 · f64 | **74.000** | — | — | — | — | 113.635 |
| Independent matrix operands · 128 × 128 · 128 × 128 · f32 | **34.236** | — | — | — | — | 39.301 |
| Matrix multiplication · 32 × 32 · 32 × 32 · f64 | 1.520 | 1.509 | 1.504 | **1.489** | 1.676 | 2.675 |
| Matrix multiplication · 64 × 64 · 64 × 64 · f64 | **9.579** | 11.013 | 10.680 | 10.864 | 10.025 | 11.344 |
| Matrix multiplication · 128 × 128 · 128 × 128 · f64 | **76.016** | 126.261 | 112.473 | 80.246 | 78.010 | 82.695 |
| Matrix multiplication · 256 × 256 · 256 × 256 · f64 | **564.860** | 654.160 | 665.835 | 725.077 | 566.777 | 599.005 |
| Matrix multiplication · 64 × 64 · 64 × 64 · f32 | **4.577** | 6.270 | 6.062 | 5.174 | 4.719 | 5.806 |
| Matrix multiplication · 128 × 128 · 128 × 128 · f32 | 39.879 | 51.568 | 48.494 | 42.098 | 38.177 | **37.724** |
| Matrix multiplication · 256 × 256 · 256 × 256 · f32 | 309.202 | 407.021 | 392.602 | 312.027 | **280.464** | 294.419 |
| Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64 | **1.886** | — | — | — | 1.936 | 3.054 |
| Matmul · 2 workers · reused output · 64 × 64 · 64 × 64 · f64 | 9.672 | — | — | — | — | — |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f64 | 41.284 | — | — | — | — | — |
| Matmul · 2 workers · reused output · 256 × 256 · 256 × 256 · f64 | 294.668 | — | — | — | — | — |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f32 | 39.180 | — | — | — | — | — |
| Cholesky factorization · 512 × 512 · f64 | 5157.487 | — | 6158.120 | **1541.686** | — | 4039.864 |
| Column-pivoted QR · 96 × 32 · f64 | **53.411** | — | 73.743 | 71.009 | — | — |
| Column-pivoted QR · 256 × 64 · f64 | 519.996 | — | 781.249 | **446.662** | — | — |
| Minimum-norm least squares · 64 × 32, rank 16, 4 RHS · f64 | **43.150** | — | 73.676 | — | — | 64.624 |
| Minimum-norm least squares · 32 × 64, rank 32, 4 RHS · f64 | **56.992** | — | 141.059 | — | — | 119.212 |
| Thin QR · 96 × 32 · f64 | 44.501 | — | **37.932** | 65.412 | — | 40.505 |
| Thin QR · 256 × 64 · f64 | 485.343 | — | **374.058** | 406.431 | — | 377.160 |
| Cholesky factorization · 64 × 64 · f64 | 19.580 | — | 13.622 | **8.319** | — | 10.821 |
| Cholesky factorization · 128 × 128 · f64 | 118.804 | — | 98.295 | **38.835** | — | 69.128 |
| Cholesky factorization · 256 × 256 · f64 | 755.046 | — | 691.424 | **194.443** | — | 522.822 |
| Shared-variable backward · sum(X * X), 64 × 32; prepared graph · f64 | 6.053 | — | — | — | **3.848** | — |
| Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64 | **2.589** | — | — | — | 5.091 | — |
| Model inference · batched normalized/sliced MLP · f64 | **2.155** | — | — | — | 3.370 | — |
| Model recording + forward · batched normalized/sliced MLP · f64 | **2.794** | — | — | — | 5.180 | — |
| Model backward · prepared graph → fresh gradients · f64 | **3.151** | — | — | — | 14.237 | — |
| Parameter update · transactional SGD, fixed gradients · f64 | 0.526 | — | — | — | — | — |
| Coalesced tensor contraction · 16 × 32 × 8 · 32 × 8 × 16 · f64 | 35.216 | — | — | — | — | **7.537** |
| Row softmax · 256 × 64 · f64 | 85.131 | **72.742** | — | — | 88.328 | 99.236 |
| Create zeros · 64 × 64 · f32 | 0.184 | **0.099** | — | 0.158 | 0.152 | 0.392 |
| Create ones · 64 × 64 · f32 | 0.226 | **0.154** | — | 0.201 | 0.216 | 1.281 |

## Version comparison

0 of 34 workloads have paired evidence. Intervals resample process pairs, not calls. Exploratory; no multiple-comparison adjustment.

| Workload | Paired latency ratio [95% interval] | Interpretation |
|---|---:|---|
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
| Matmul · 2 workers · reused output · 64 × 64 · 64 × 64 · f64 | — | No paired comparison |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f64 | — | No paired comparison |
| Matmul · 2 workers · reused output · 256 × 256 · 256 × 256 · f64 | — | No paired comparison |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f32 | — | No paired comparison |
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

## Method and limits

CPU affinity pinned to logical CPU 0. Frequency and background load are uncontrolled; observations establish no universal ranking.

Minimum-norm rows use pivoted QR plus scaled implicit RZ in Kernix; NumPy native lstsq; nalgebra native SVD/solve. Algorithm costs and rank policies differ. Normal residuals and null-space orthogonality are independently checked. See DETAILS.md for thresholds.

Ordinary allocating rows include output allocation and destruction. Explicit two-worker rows reuse output and exclude pool preparation. NumPy native allocations are unavailable, never zero. Warmed probes exclude first-use native workspace and retention.

[Interactive report](report.html) · [Full method](DETAILS.md) · [Raw samples](raw.json) · [Printable family charts](latency.pdf)

Measured source `9a9a0e5eec261c1ed4482fe23215c514e5b20272`. Run `0.10.0-registry-011`. Rendering does not change measurements.
