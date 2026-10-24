# Kernix 0.10.0 · CPU benchmarks

AMD EPYC 7763 64-Core Processor · warmed public API calls · conditions per workload

**Microseconds per call, median. Lower is faster. Bold marks a lowest observed median only among at least two measured libraries.**

No previous-version baseline is paired with this view.

| Workload | Kernix | ndarray | nalgebra | faer | Candle | NumPy |
|---|---:|---:|---:|---:|---:|---:|
| Independent matrix operands · 128 × 128 · 128 × 128 · f64 | 141.400 | — | — | — | — | **102.300** |
| Independent matrix operands · 128 × 128 · 128 × 128 · f32 | 75.200 | — | — | — | — | **56.700** |
| Matrix multiplication · 32 × 32 · 32 × 32 · f64 | 2.800 | 2.400 | **2.300** | 2.500 | 4.300 | 4.100 |
| Matrix multiplication · 64 × 64 · 64 × 64 · f64 | **13.600** | 62.100 | 58.500 | 14.700 | 14.100 | 16.000 |
| Matrix multiplication · 128 × 128 · 128 × 128 · f64 | 140.000 | 114.800 | 115.900 | 102.200 | 141.300 | **100.200** |
| Matrix multiplication · 256 × 256 · 256 × 256 · f64 | 868.900 | 1179.100 | 908.600 | 1081.300 | 875.300 | **762.800** |
| Matrix multiplication · 64 × 64 · 64 × 64 · f32 | **6.900** | 30.400 | 8.200 | 7.000 | 7.400 | 10.400 |
| Matrix multiplication · 128 × 128 · 128 × 128 · f32 | 76.000 | 122.300 | 53.600 | **50.400** | 75.500 | 56.100 |
| Matrix multiplication · 256 × 256 · 256 × 256 · f32 | **355.800** | 393.600 | 397.900 | 405.200 | 431.600 | 388.300 |
| Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64 | **3.400** | — | — | — | 8.200 | 5.000 |
| Matmul · 2 workers · reused output · 64 × 64 · 64 × 64 · f64 | 40.100 | — | — | — | — | — |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f64 | 88.400 | — | — | — | — | — |
| Matmul · 2 workers · reused output · 256 × 256 · 256 × 256 · f64 | 438.700 | — | — | — | — | — |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f32 | 59.000 | — | — | — | — | — |
| Cholesky factorization · 512 × 512 · f64 | 8586.800 | — | 8275.000 | **3164.700** | — | 6804.200 |
| Column-pivoted QR · 96 × 32 · f64 | 125.000 | — | 108.400 | **78.000** | — | — |
| Column-pivoted QR · 256 × 64 · f64 | 936.600 | — | 909.000 | **413.000** | — | — |
| Minimum-norm least squares · 64 × 32, rank 16, 4 RHS · f64 | **100.600** | — | 104.100 | — | — | 114.500 |
| Minimum-norm least squares · 32 × 64, rank 32, 4 RHS · f64 | **114.900** | — | 174.800 | — | — | 215.000 |
| Thin QR · 96 × 32 · f64 | 104.500 | — | 78.500 | 87.600 | — | **77.200** |
| Thin QR · 256 × 64 · f64 | 783.800 | — | 639.600 | **385.900** | — | 546.100 |
| Cholesky factorization · 64 × 64 · f64 | 35.100 | — | 21.500 | **14.500** | — | 21.000 |
| Cholesky factorization · 128 × 128 · f64 | 229.300 | — | 120.700 | **59.000** | — | 128.600 |
| Cholesky factorization · 256 × 256 · f64 | 991.900 | — | 871.800 | **316.200** | — | 953.100 |
| Shared-variable backward · sum(X * X), 64 × 32; prepared graph · f64 | **9.600** | — | — | — | 58.600 | — |
| Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64 | **4.400** | — | — | — | 10.100 | — |
| Model inference · batched normalized/sliced MLP · f64 | **3.500** | — | — | — | 10.100 | — |
| Model recording + forward · batched normalized/sliced MLP · f64 | **4.800** | — | — | — | 10.200 | — |
| Model backward · prepared graph → fresh gradients · f64 | **5.800** | — | — | — | 31.900 | — |
| Parameter update · transactional SGD, fixed gradients · f64 | 1.000 | — | — | — | — | — |
| Coalesced tensor contraction · 16 × 32 × 8 · 32 × 8 × 16 · f64 | 63.700 | — | — | — | — | **11.800** |
| Row softmax · 256 × 64 · f64 | **114.500** | 153.700 | — | — | 155.800 | 132.600 |
| Create zeros · 64 × 64 · f32 | 0.600 | **0.300** | — | 0.500 | 0.500 | 0.800 |
| Create ones · 64 × 64 · f32 | 0.500 | **0.400** | — | 0.500 | 0.700 | 2.100 |

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

CPU affinity was not pinned. Frequency and background load are uncontrolled; observations establish no universal ranking.

Minimum-norm rows use pivoted QR plus scaled implicit RZ in Kernix; NumPy native lstsq; nalgebra native SVD/solve. Algorithm costs and rank policies differ. Normal residuals and null-space orthogonality are independently checked. See DETAILS.md for thresholds.

Ordinary allocating rows include output allocation and destruction. Explicit two-worker rows reuse output and exclude pool preparation. NumPy native allocations are unavailable, never zero. Warmed probes exclude first-use native workspace and retention.

[Interactive report](report.html) · [Full method](DETAILS.md) · [Raw samples](raw.json) · [Printable family charts](latency.pdf)

Measured source `18750693908110110d85d02d0f11af8d58bfae8b`. Run `baseline-windows-latest-40059566395`. Rendering does not change measurements.
