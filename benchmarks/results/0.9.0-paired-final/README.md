# Kernix 0.9.0 vs 0.8.0 · paired processes

Current: `0.9.0-final`. Baseline: `0.8.0-registry-090`.

9 fresh-process pairs per workload. Lower ratio is faster.
95% bootstrap intervals resample process pairs, not individual calls. Exploratory intervals do not correct for multiple comparisons.
Shared-machine drift and small sample counts remain limits; these are not per-call p95 intervals.

| Workload | Current / baseline latency | 95% interval |
|---|---:|---:|
| Cholesky factorization 64 × 64 f64 | 0.977× | [0.924, 1.040] |
| Cholesky factorization 128 × 128 f64 | 0.979× | [0.927, 1.019] |
| Cholesky factorization 256 × 256 f64 | 1.007× | [0.993, 1.026] |
| Thin QR 96 × 32 f64 | 1.049× | [0.966, 1.059] |
| Thin QR 256 × 64 f64 | 1.020× | [0.995, 1.046] |
| Matrix multiplication 128 × 128 · 128 × 128 f64 | 1.002× | [0.980, 1.012] |
| Matrix multiplication 128 × 128 · 128 × 128 f32 | 0.987× | [0.933, 1.014] |

Full worker samples, accuracy checks, randomized order, executable hashes and source metadata: [paired.json](paired.json).
