# Kernix 0.11.0 · CPU benchmarks

AMD EPYC 7763 64-Core Processor · warmed public API calls · conditions per workload

**Microseconds per call, median. Lower is faster. Bold marks a lowest observed median only among at least two measured libraries.**

Baseline: baseline-ubuntu-latest-40059566395. Sequential run changes are descriptive, not randomized/interleaved process comparisons.

| Workload | Kernix | ndarray | nalgebra | faer | Candle | NumPy | Sequential run change |
|---|---:|---:|---:|---:|---:|---:|---:|
| Independent matrix operands · 128 × 128 · 128 × 128 · f64 | **94.016** | — | — | — | — | 102.652 | +0.2% |
| Independent matrix operands · 128 × 128 · 128 × 128 · f32 | **45.996** | — | — | — | — | 49.803 | -0.2% |
| Matrix multiplication · 32 × 32 · 32 × 32 · f64 | 2.524 | 2.224 | 2.124 | **2.044** | 2.214 | 3.677 | +0.4% |
| Matrix multiplication · 64 × 64 · 64 × 64 · f64 | **12.724** | 15.509 | 14.958 | 13.455 | 12.844 | 15.148 | -0.2% |
| Matrix multiplication · 128 × 128 · 128 × 128 · f64 | 93.986 | 204.002 | 204.954 | 100.428 | **92.934** | 102.031 | +0.1% |
| Matrix multiplication · 256 × 256 · 256 × 256 · f64 | **730.869** | 910.935 | 904.363 | 969.514 | 738.492 | 766.815 | -0.4% |
| Matrix multiplication · 64 × 64 · 64 × 64 · f32 | 6.612 | 7.454 | 7.354 | **6.512** | 6.583 | 8.105 | +1.4% |
| Matrix multiplication · 128 × 128 · 128 × 128 · f32 | **45.836** | 74.700 | 73.688 | 51.556 | 46.186 | 49.011 | -1.2% |
| Matrix multiplication · 256 × 256 · 256 × 256 · f32 | **352.229** | 600.354 | 600.013 | 418.683 | 353.150 | 361.626 | -1.2% |
| Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64 | **3.126** | — | — | — | 3.247 | 4.188 | +1.6% |
| Matmul · 2 workers · reused output · 64 × 64 · 64 × 64 · f64 | 18.966 | — | — | — | — | — | +0.6% |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f64 | 56.566 | — | — | — | — | — | -1.0% |
| Matmul · 2 workers · reused output · 256 × 256 · 256 × 256 · f64 | 393.688 | — | — | — | — | — | +0.3% |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f32 | 34.084 | — | — | — | — | — | +6.5% |
| Cholesky factorization · 512 × 512 · f64 | 3886.384 | — | 7356.338 | **2584.958** | — | 6194.474 | -49.1% |
| Column-pivoted QR · 96 × 32 · f64 | 78.687 | — | 78.216 | **42.480** | — | — | +0.3% |
| Column-pivoted QR · 256 × 64 · f64 | 851.052 | — | 767.306 | **343.322** | — | — | -0.0% |
| Minimum-norm least squares · 64 × 32, rank 16, 4 RHS · f64 | **62.256** | — | 102.482 | — | — | 103.403 | -0.3% |
| Minimum-norm least squares · 32 × 64, rank 32, 4 RHS · f64 | **84.257** | — | 178.364 | — | — | 188.583 | -0.5% |
| Thin QR · 96 × 32 · f64 | 67.256 | — | 58.970 | **54.262** | — | 70.692 | +0.7% |
| Thin QR · 256 × 64 · f64 | 780.099 | — | 570.307 | **418.203** | — | 540.582 | +0.1% |
| Cholesky factorization · 64 × 64 · f64 | 24.777 | — | 20.910 | **14.097** | — | 18.174 | -0.8% |
| Cholesky factorization · 128 × 128 · f64 | 169.948 | — | 119.193 | **57.718** | — | 108.113 | +0.6% |
| Cholesky factorization · 256 × 256 · f64 | 686.484 | — | 788.336 | **313.717** | — | 1070.464 | -32.3% |
| Shared-variable backward · sum(X * X), 64 × 32; prepared graph · f64 | 10.410 | — | — | — | **4.939** | — | -1.2% |
| Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64 | **4.067** | — | — | — | 7.725 | — | +4.9% |
| Model inference · batched normalized/sliced MLP · f64 | **3.226** | — | — | — | 5.030 | — | -4.4% |
| Model recording + forward · batched normalized/sliced MLP · f64 | **4.098** | — | — | — | 6.201 | — | -1.7% |
| Model backward · prepared graph → fresh gradients · f64 | **5.099** | — | — | — | 20.689 | — | +2.6% |
| Parameter update · transactional SGD, fixed gradients · f64 | 0.802 | — | — | — | — | — | +0.0% |
| Coalesced tensor contraction · 16 × 32 × 8 · 32 × 8 × 16 · f64 | 63.067 | — | — | — | — | **11.331** | -1.0% |
| Row softmax · 256 × 64 · f64 | 119.533 | **110.877** | — | — | 115.145 | 132.998 | -0.0% |
| Create zeros · 64 × 64 · f32 | 0.320 | **0.250** | — | 0.311 | 0.301 | 0.742 | -0.3% |
| Create ones · 64 × 64 · f32 | 0.431 | **0.351** | — | 0.391 | 0.471 | 2.094 | +0.0% |

## Version comparison

12 of 34 workloads have paired evidence. Intervals resample process pairs, not calls. Exploratory; no multiple-comparison adjustment.

| Workload | Paired latency ratio [95% interval] | Interpretation |
|---|---:|---|
| Independent matrix operands · 128 × 128 · 128 × 128 · f64 | 1.000× [0.996, 1.003] | Timing direction unclear |
| Independent matrix operands · 128 × 128 · 128 × 128 · f32 | 1.001× [0.993, 1.006] | Timing direction unclear |
| Matrix multiplication · 32 × 32 · 32 × 32 · f64 | — | No paired comparison |
| Matrix multiplication · 64 × 64 · 64 × 64 · f64 | — | No paired comparison |
| Matrix multiplication · 128 × 128 · 128 × 128 · f64 | 1.001× [0.995, 1.007] | Timing direction unclear |
| Matrix multiplication · 256 × 256 · 256 × 256 · f64 | 1.001× [0.999, 1.004] | Timing direction unclear |
| Matrix multiplication · 64 × 64 · 64 × 64 · f32 | — | No paired comparison |
| Matrix multiplication · 128 × 128 · 128 × 128 · f32 | 1.000× [0.998, 1.002] | Timing direction unclear |
| Matrix multiplication · 256 × 256 · 256 × 256 · f32 | 0.999× [0.997, 1.001] | Timing direction unclear |
| Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64 | — | No paired comparison |
| Matmul · 2 workers · reused output · 64 × 64 · 64 × 64 · f64 | — | No paired comparison |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f64 | 1.006× [1.001, 1.020] | Observed time increase |
| Matmul · 2 workers · reused output · 256 × 256 · 256 × 256 · f64 | 1.001× [0.999, 1.005] | Timing direction unclear |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f32 | 1.002× [0.991, 1.061] | Timing direction unclear |
| Cholesky factorization · 512 × 512 · f64 | 0.509× [0.506, 0.515] | Observed time reduction |
| Column-pivoted QR · 96 × 32 · f64 | — | No paired comparison |
| Column-pivoted QR · 256 × 64 · f64 | — | No paired comparison |
| Minimum-norm least squares · 64 × 32, rank 16, 4 RHS · f64 | — | No paired comparison |
| Minimum-norm least squares · 32 × 64, rank 32, 4 RHS · f64 | — | No paired comparison |
| Thin QR · 96 × 32 · f64 | — | No paired comparison |
| Thin QR · 256 × 64 · f64 | — | No paired comparison |
| Cholesky factorization · 64 × 64 · f64 | — | No paired comparison |
| Cholesky factorization · 128 × 128 · f64 | 1.011× [1.004, 1.014] | Observed time increase |
| Cholesky factorization · 256 × 256 · f64 | 0.679× [0.673, 0.685] | Observed time reduction |
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

Measured source `18750693908110110d85d02d0f11af8d58bfae8b`. Run `runner-ubuntu-latest-40059566395`. Rendering does not change measurements.
