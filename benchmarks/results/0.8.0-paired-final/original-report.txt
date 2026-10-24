# Kernix 0.8.0 vs 0.7.3 · paired processes

Current: `0.8.0-clean-18b91c6`. Baseline: `0.7.3-registry-fix080`.

9 fresh-process pairs per workload. Lower ratio is faster.
95% bootstrap intervals resample process pairs, not individual calls. Exploratory intervals do not correct for multiple comparisons.
Shared-machine drift and small sample counts remain limits; these are not per-call p95 intervals.

| Workload | Current / baseline latency | 95% interval |
|---|---:|---:|
| Cholesky factorization 64 × 64 f64 | 0.868× | [0.827, 0.941] |
| Cholesky factorization 128 × 128 f64 | 0.674× | [0.659, 0.683] |
| Cholesky factorization 256 × 256 f64 | 0.493× | [0.481, 0.503] |
| Thin QR 96 × 32 f64 | 1.043× | [1.036, 1.084] |
| Thin QR 256 × 64 f64 | 0.958× | [0.937, 0.988] |
| Matrix multiplication 128 × 128 · 128 × 128 f64 | 1.015× | [1.006, 1.032] |
| Matrix multiplication 128 × 128 · 128 × 128 f32 | 0.994× | [0.925, 1.003] |

Full worker samples, accuracy checks, randomized order, executable hashes and source metadata: [paired.json](paired.json).
