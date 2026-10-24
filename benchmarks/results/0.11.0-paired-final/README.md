# Kernix 0.11.0 vs 0.10.0 · paired processes

Current: `0.11.0-apple-policy`. Baseline: `0.10.0-registry-011`.

9 fresh-process pairs per workload. Lower ratio is faster.
95% bootstrap intervals resample process pairs, not individual calls. Exploratory intervals do not correct for multiple comparisons.
Shared-machine drift and small sample counts remain limits; these are not per-call p95 intervals.

| Workload | Current / baseline latency | 95% interval |
|---|---:|---:|
| Matrix multiplication 128 × 128 · 128 × 128 f64 | 1.003× | [0.988, 1.023] |
| Matrix multiplication 128 × 128 · 128 × 128 f32 | 1.029× | [0.953, 1.083] |
| Matrix multiplication 256 × 256 · 256 × 256 f64 | 1.011× | [0.991, 1.029] |
| Matrix multiplication 256 × 256 · 256 × 256 f32 | 1.003× | [0.968, 1.014] |
| Independent matrix operands 128 × 128 · 128 × 128 f64 | 1.005× | [0.977, 1.031] |
| Independent matrix operands 128 × 128 · 128 × 128 f32 | 0.981× | [0.963, 1.007] |
| Cholesky factorization 128 × 128 f64 | 1.033× | [1.008, 1.043] |
| Cholesky factorization 256 × 256 f64 | 0.776× | [0.759, 0.811] |
| Cholesky factorization 512 × 512 f64 | 0.598× | [0.595, 0.612] |
| Matmul · 2 workers · reused output 128 × 128 · 128 × 128 f64 | 0.872× | [0.554, 1.380] |
| Matmul · 2 workers · reused output 256 × 256 · 256 × 256 f64 | 1.038× | [0.957, 1.368] |
| Matmul · 2 workers · reused output 128 × 128 · 128 × 128 f32 | 0.975× | [0.588, 1.166] |

Full worker samples, accuracy checks, randomized order, executable hashes and source metadata: [paired.json](paired.json).
