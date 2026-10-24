# Kernix 0.11.0 · CPU benchmarks

AMD EPYC 7763 64-Core Processor · warmed public API calls · conditions per workload

**Microseconds per call, median. Lower is faster. Bold marks a lowest observed median only among at least two measured libraries.**

Baseline: baseline-windows-latest-40059566395. Sequential run changes are descriptive, not randomized/interleaved process comparisons.

| Workload | Kernix | ndarray | nalgebra | faer | Candle | NumPy | Sequential run change |
|---|---:|---:|---:|---:|---:|---:|---:|
| Independent matrix operands · 128 × 128 · 128 × 128 · f64 | 142.400 | — | — | — | — | **101.500** | +0.7% |
| Independent matrix operands · 128 × 128 · 128 × 128 · f32 | 75.200 | — | — | — | — | **56.400** | +0.0% |
| Matrix multiplication · 32 × 32 · 32 × 32 · f64 | 2.800 | 2.400 | **2.300** | 2.500 | 2.600 | 4.100 | +0.0% |
| Matrix multiplication · 64 × 64 · 64 × 64 · f64 | **13.600** | 60.900 | 58.600 | 14.600 | 14.100 | 15.900 | +0.0% |
| Matrix multiplication · 128 × 128 · 128 × 128 · f64 | 141.000 | 128.400 | 116.000 | 120.300 | 141.300 | **101.300** | +0.7% |
| Matrix multiplication · 256 × 256 · 256 × 256 · f64 | 900.100 | 1176.400 | 918.300 | 1069.100 | 860.000 | **769.700** | +3.6% |
| Matrix multiplication · 64 × 64 · 64 × 64 · f32 | **7.100** | 30.200 | 29.700 | **7.100** | 7.400 | 10.400 | +2.9% |
| Matrix multiplication · 128 × 128 · 128 × 128 · f32 | **47.400** | 123.800 | 53.600 | 72.000 | 75.800 | 56.200 | -37.6% |
| Matrix multiplication · 256 × 256 · 256 × 256 · f32 | 430.500 | 394.800 | 396.300 | 415.800 | 431.500 | **389.500** | +21.0% |
| Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64 | 5.200 | — | — | — | **4.000** | 5.000 | +52.9% |
| Matmul · 2 workers · reused output · 64 × 64 · 64 × 64 · f64 | 39.500 | — | — | — | — | — | -1.5% |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f64 | 84.100 | — | — | — | — | — | -4.9% |
| Matmul · 2 workers · reused output · 256 × 256 · 256 × 256 · f64 | 441.000 | — | — | — | — | — | +0.5% |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f32 | 57.500 | — | — | — | — | — | -2.5% |
| Cholesky factorization · 512 × 512 · f64 | 4728.400 | — | 8226.900 | **3037.000** | — | 6494.200 | -44.9% |
| Column-pivoted QR · 96 × 32 · f64 | 125.100 | — | 107.700 | **78.000** | — | — | +0.1% |
| Column-pivoted QR · 256 × 64 · f64 | 907.700 | — | 915.600 | **411.800** | — | — | -3.1% |
| Minimum-norm least squares · 64 × 32, rank 16, 4 RHS · f64 | **101.800** | — | 105.100 | — | — | 113.300 | +1.2% |
| Minimum-norm least squares · 32 × 64, rank 32, 4 RHS · f64 | **115.700** | — | 175.200 | — | — | 204.200 | +0.7% |
| Thin QR · 96 × 32 · f64 | 113.300 | — | **81.900** | 86.300 | — | 152.100 | +8.4% |
| Thin QR · 256 × 64 · f64 | 802.400 | — | 565.900 | **392.700** | — | 545.200 | +2.4% |
| Cholesky factorization · 64 × 64 · f64 | 35.900 | — | 19.800 | **14.600** | — | 21.500 | +2.3% |
| Cholesky factorization · 128 × 128 · f64 | 222.500 | — | 112.800 | **59.400** | — | 113.300 | -3.0% |
| Cholesky factorization · 256 × 256 · f64 | 737.700 | — | 842.000 | **315.300** | — | 985.100 | -25.6% |
| Shared-variable backward · sum(X * X), 64 × 32; prepared graph · f64 | **13.700** | — | — | — | 59.400 | — | +42.7% |
| Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64 | **4.800** | — | — | — | 9.700 | — | +9.1% |
| Model inference · batched normalized/sliced MLP · f64 | **3.500** | — | — | — | 8.200 | — | +0.0% |
| Model recording + forward · batched normalized/sliced MLP · f64 | **4.800** | — | — | — | 10.100 | — | +0.0% |
| Model backward · prepared graph → fresh gradients · f64 | **5.700** | — | — | — | 31.500 | — | -1.7% |
| Parameter update · transactional SGD, fixed gradients · f64 | 1.000 | — | — | — | — | — | +0.0% |
| Coalesced tensor contraction · 16 × 32 × 8 · 32 × 8 × 16 · f64 | 63.900 | — | — | — | — | **11.900** | +0.3% |
| Row softmax · 256 × 64 · f64 | **114.100** | 153.900 | — | — | 154.700 | 130.800 | -0.3% |
| Create zeros · 64 × 64 · f32 | 0.400 | **0.300** | — | 0.500 | 0.800 | 0.800 | -33.3% |
| Create ones · 64 × 64 · f32 | 0.600 | **0.400** | — | 0.500 | 0.700 | 2.100 | +20.0% |

## Version comparison

12 of 34 workloads have paired evidence. Intervals resample process pairs, not calls. Exploratory; no multiple-comparison adjustment.

| Workload | Paired latency ratio [95% interval] | Interpretation |
|---|---:|---|
| Independent matrix operands · 128 × 128 · 128 × 128 · f64 | 0.999× [0.995, 1.488] | Timing direction unclear |
| Independent matrix operands · 128 × 128 · 128 × 128 · f32 | 1.000× [0.997, 1.004] | Timing direction unclear |
| Matrix multiplication · 32 × 32 · 32 × 32 · f64 | — | No paired comparison |
| Matrix multiplication · 64 × 64 · 64 × 64 · f64 | — | No paired comparison |
| Matrix multiplication · 128 × 128 · 128 × 128 · f64 | 0.995× [0.985, 1.004] | Timing direction unclear |
| Matrix multiplication · 256 × 256 · 256 × 256 · f64 | 1.001× [0.846, 1.009] | Timing direction unclear |
| Matrix multiplication · 64 × 64 · 64 × 64 · f32 | — | No paired comparison |
| Matrix multiplication · 128 × 128 · 128 × 128 · f32 | 0.998× [0.615, 1.000] | Timing direction unclear |
| Matrix multiplication · 256 × 256 · 256 × 256 · f32 | 1.000× [0.994, 1.001] | Timing direction unclear |
| Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64 | — | No paired comparison |
| Matmul · 2 workers · reused output · 64 × 64 · 64 × 64 · f64 | — | No paired comparison |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f64 | 1.000× [0.964, 1.013] | Timing direction unclear |
| Matmul · 2 workers · reused output · 256 × 256 · 256 × 256 · f64 | 0.999× [0.972, 1.006] | Timing direction unclear |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f32 | 1.009× [0.995, 1.037] | Timing direction unclear |
| Cholesky factorization · 512 × 512 · f64 | 0.546× [0.545, 0.554] | Observed time reduction |
| Column-pivoted QR · 96 × 32 · f64 | — | No paired comparison |
| Column-pivoted QR · 256 × 64 · f64 | — | No paired comparison |
| Minimum-norm least squares · 64 × 32, rank 16, 4 RHS · f64 | — | No paired comparison |
| Minimum-norm least squares · 32 × 64, rank 32, 4 RHS · f64 | — | No paired comparison |
| Thin QR · 96 × 32 · f64 | — | No paired comparison |
| Thin QR · 256 × 64 · f64 | — | No paired comparison |
| Cholesky factorization · 64 × 64 · f64 | — | No paired comparison |
| Cholesky factorization · 128 × 128 · f64 | 1.008× [0.972, 1.011] | Timing direction unclear |
| Cholesky factorization · 256 × 256 · f64 | 0.696× [0.686, 0.713] | Observed time reduction |
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

CPU affinity was not pinned. Frequency and background load are uncontrolled; observations establish no universal ranking.

Minimum-norm rows use pivoted QR plus scaled implicit RZ in Kernix; NumPy native lstsq; nalgebra native SVD/solve. Algorithm costs and rank policies differ. Normal residuals and null-space orthogonality are independently checked. See DETAILS.md for thresholds.

Ordinary allocating rows include output allocation and destruction. Explicit two-worker rows reuse output and exclude pool preparation. NumPy native allocations are unavailable, never zero. Warmed probes exclude first-use native workspace and retention.

[Interactive report](report.html) · [Full method](DETAILS.md) · [Raw samples](raw.json) · [Printable family charts](latency.pdf)

Measured source `18750693908110110d85d02d0f11af8d58bfae8b`. Run `runner-windows-latest-40059566395`. Rendering does not change measurements.
