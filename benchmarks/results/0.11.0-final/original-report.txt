# Kernix 0.11.0 · CPU benchmarks

12th Gen Intel(R) Core(TM) i3-1215U · warmed public API calls · conditions per workload

**Microseconds per call, median. Lower is faster. Bold marks a lowest observed median only among at least two measured libraries.**

Baseline: 0.10.0-registry-011. Sequential run changes are descriptive, not randomized/interleaved process comparisons.

| Workload | Kernix | ndarray | nalgebra | faer | Candle | NumPy | Sequential run change |
|---|---:|---:|---:|---:|---:|---:|---:|
| Independent matrix operands · 128 × 128 · 128 × 128 · f64 | **79.522** | — | — | — | — | 82.316 | +7.5% |
| Independent matrix operands · 128 × 128 · 128 × 128 · f32 | **40.851** | — | — | — | — | 41.398 | +19.3% |
| Matrix multiplication · 32 × 32 · 32 × 32 · f64 | 1.822 | **1.610** | 1.693 | 1.646 | 1.752 | 4.777 | +19.9% |
| Matrix multiplication · 64 × 64 · 64 × 64 · f64 | **10.685** | 12.993 | 11.254 | 12.064 | 11.060 | 12.896 | +11.5% |
| Matrix multiplication · 128 × 128 · 128 × 128 · f64 | **81.483** | 133.454 | 126.333 | 91.301 | 91.219 | 94.252 | +7.2% |
| Matrix multiplication · 256 × 256 · 256 × 256 · f64 | 651.551 | 745.908 | 730.698 | 834.945 | **644.914** | 686.909 | +15.3% |
| Matrix multiplication · 64 × 64 · 64 × 64 · f32 | **5.239** | 7.002 | 6.643 | 5.852 | 5.459 | 6.689 | +14.5% |
| Matrix multiplication · 128 × 128 · 128 × 128 · f32 | 46.010 | 54.284 | 54.141 | 47.184 | 41.717 | **41.294** | +15.4% |
| Matrix multiplication · 256 × 256 · 256 × 256 · f32 | 346.674 | 453.024 | 446.141 | 358.189 | **311.220** | 333.271 | +12.1% |
| Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64 | **2.279** | — | — | — | 2.293 | 3.410 | +20.8% |
| Matmul · 2 workers · reused output · 64 × 64 · 64 × 64 · f64 | 11.276 | — | — | — | — | — | +16.6% |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f64 | 79.016 | — | — | — | — | — | +91.4% |
| Matmul · 2 workers · reused output · 256 × 256 · 256 × 256 · f64 | 343.076 | — | — | — | — | — | +16.4% |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f32 | 41.048 | — | — | — | — | — | +4.8% |
| Cholesky factorization · 512 × 512 · f64 | 4062.093 | — | 8337.880 | **2059.378** | — | 8994.431 | -21.2% |
| Column-pivoted QR · 96 × 32 · f64 | **61.850** | — | 84.895 | 81.252 | — | — | +15.8% |
| Column-pivoted QR · 256 × 64 · f64 | 602.805 | — | 856.584 | **514.231** | — | — | +15.9% |
| Minimum-norm least squares · 64 × 32, rank 16, 4 RHS · f64 | **54.399** | — | 81.705 | — | — | 74.806 | +26.1% |
| Minimum-norm least squares · 32 × 64, rank 32, 4 RHS · f64 | **66.701** | — | 152.598 | — | — | 146.425 | +17.0% |
| Thin QR · 96 × 32 · f64 | 53.055 | — | **43.968** | 75.509 | — | 45.085 | +19.2% |
| Thin QR · 256 × 64 · f64 | 588.471 | — | 434.514 | 508.628 | — | **430.596** | +21.2% |
| Cholesky factorization · 64 × 64 · f64 | 23.116 | — | 15.500 | **9.262** | — | 11.825 | +18.1% |
| Cholesky factorization · 128 × 128 · f64 | 128.368 | — | 111.377 | **47.544** | — | 77.201 | +8.1% |
| Cholesky factorization · 256 × 256 · f64 | 579.500 | — | 774.853 | **227.006** | — | 605.474 | -23.2% |
| Shared-variable backward · sum(X * X), 64 × 32; prepared graph · f64 | 6.946 | — | — | — | **4.717** | — | +14.8% |
| Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64 | **2.753** | — | — | — | 4.459 | — | +6.3% |
| Model inference · batched normalized/sliced MLP · f64 | **4.036** | — | — | — | 4.733 | — | +87.3% |
| Model recording + forward · batched normalized/sliced MLP · f64 | **2.840** | — | — | — | 8.494 | — | +1.6% |
| Model backward · prepared graph → fresh gradients · f64 | **3.515** | — | — | — | 21.894 | — | +11.6% |
| Parameter update · transactional SGD, fixed gradients · f64 | 0.584 | — | — | — | — | — | +11.0% |
| Coalesced tensor contraction · 16 × 32 × 8 · 32 × 8 × 16 · f64 | 41.049 | — | — | — | — | **8.467** | +16.6% |
| Row softmax · 256 × 64 · f64 | 100.743 | **78.637** | — | — | 97.887 | 117.684 | +18.3% |
| Create zeros · 64 × 64 · f32 | **0.155** | 0.195 | — | 0.175 | 0.168 | 0.445 | -15.8% |
| Create ones · 64 × 64 · f32 | 0.205 | **0.169** | — | 0.207 | 0.250 | 1.399 | -9.3% |

## Version comparison

12 of 34 workloads have paired evidence. Intervals resample process pairs, not calls. Exploratory; no multiple-comparison adjustment.

| Workload | Paired latency ratio [95% interval] | Interpretation |
|---|---:|---|
| Independent matrix operands · 128 × 128 · 128 × 128 · f64 | 1.005× [0.977, 1.031] | Timing direction unclear |
| Independent matrix operands · 128 × 128 · 128 × 128 · f32 | 0.981× [0.963, 1.007] | Timing direction unclear |
| Matrix multiplication · 32 × 32 · 32 × 32 · f64 | — | No paired comparison |
| Matrix multiplication · 64 × 64 · 64 × 64 · f64 | — | No paired comparison |
| Matrix multiplication · 128 × 128 · 128 × 128 · f64 | 1.003× [0.988, 1.023] | Timing direction unclear |
| Matrix multiplication · 256 × 256 · 256 × 256 · f64 | 1.011× [0.991, 1.029] | Timing direction unclear |
| Matrix multiplication · 64 × 64 · 64 × 64 · f32 | — | No paired comparison |
| Matrix multiplication · 128 × 128 · 128 × 128 · f32 | 1.029× [0.953, 1.083] | Timing direction unclear |
| Matrix multiplication · 256 × 256 · 256 × 256 · f32 | 1.003× [0.968, 1.014] | Timing direction unclear |
| Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64 | — | No paired comparison |
| Matmul · 2 workers · reused output · 64 × 64 · 64 × 64 · f64 | — | No paired comparison |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f64 | 0.872× [0.554, 1.380] | Timing direction unclear |
| Matmul · 2 workers · reused output · 256 × 256 · 256 × 256 · f64 | 1.038× [0.957, 1.368] | Timing direction unclear |
| Matmul · 2 workers · reused output · 128 × 128 · 128 × 128 · f32 | 0.975× [0.588, 1.166] | Timing direction unclear |
| Cholesky factorization · 512 × 512 · f64 | 0.598× [0.595, 0.612] | Observed time reduction |
| Column-pivoted QR · 96 × 32 · f64 | — | No paired comparison |
| Column-pivoted QR · 256 × 64 · f64 | — | No paired comparison |
| Minimum-norm least squares · 64 × 32, rank 16, 4 RHS · f64 | — | No paired comparison |
| Minimum-norm least squares · 32 × 64, rank 32, 4 RHS · f64 | — | No paired comparison |
| Thin QR · 96 × 32 · f64 | — | No paired comparison |
| Thin QR · 256 × 64 · f64 | — | No paired comparison |
| Cholesky factorization · 64 × 64 · f64 | — | No paired comparison |
| Cholesky factorization · 128 × 128 · f64 | 1.033× [1.008, 1.043] | Observed time increase |
| Cholesky factorization · 256 × 256 · f64 | 0.776× [0.759, 0.811] | Observed time reduction |
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

Measured source `18750693908110110d85d02d0f11af8d58bfae8b`. Run `0.11.0-apple-policy`. Rendering does not change measurements.
