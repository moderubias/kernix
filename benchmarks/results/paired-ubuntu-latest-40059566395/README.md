# Kernix 0.11.0 vs 0.10.0 · paired processes

Current: `runner-ubuntu-latest-40059566395`. Baseline: `baseline-ubuntu-latest-40059566395`.

9 fresh-process pairs per workload. Lower ratio is faster.
95% bootstrap intervals resample process pairs, not individual calls. Exploratory intervals do not correct for multiple comparisons.
Shared-machine drift and small sample counts remain limits; these are not per-call p95 intervals.

| Workload | Current / baseline latency | 95% interval |
|---|---:|---:|
| Matrix multiplication 128 × 128 · 128 × 128 f64 | 1.001× | [0.995, 1.007] |
| Matrix multiplication 128 × 128 · 128 × 128 f32 | 1.000× | [0.998, 1.002] |
| Matrix multiplication 256 × 256 · 256 × 256 f64 | 1.001× | [0.999, 1.004] |
| Matrix multiplication 256 × 256 · 256 × 256 f32 | 0.999× | [0.997, 1.001] |
| Independent matrix operands 128 × 128 · 128 × 128 f64 | 1.000× | [0.996, 1.003] |
| Independent matrix operands 128 × 128 · 128 × 128 f32 | 1.001× | [0.993, 1.006] |
| Cholesky factorization 128 × 128 f64 | 1.011× | [1.004, 1.014] |
| Cholesky factorization 256 × 256 f64 | 0.679× | [0.673, 0.685] |
| Cholesky factorization 512 × 512 f64 | 0.509× | [0.506, 0.515] |
| Matmul · 2 workers · reused output 128 × 128 · 128 × 128 f64 | 1.006× | [1.001, 1.020] |
| Matmul · 2 workers · reused output 256 × 256 · 256 × 256 f64 | 1.001× | [0.999, 1.005] |
| Matmul · 2 workers · reused output 128 × 128 · 128 × 128 f32 | 1.002× | [0.991, 1.061] |

Full worker samples, accuracy checks, randomized order, executable hashes and source metadata: [paired.json](paired.json).
