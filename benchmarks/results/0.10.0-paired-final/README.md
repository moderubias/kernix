# Kernix 0.10.0 vs 0.9.0 · paired processes

Current: `0.10.0-clean`. Baseline: `0.9.0-registry-010`.

9 fresh-process pairs per workload. Lower ratio is faster.
95% bootstrap intervals resample process pairs, not individual calls. Exploratory intervals do not correct for multiple comparisons.
Shared-machine drift and small sample counts remain limits; these are not per-call p95 intervals.

| Workload | Current / baseline latency | 95% interval |
|---|---:|---:|
| Minimum-norm least squares 64 × 32, rank 16, 4 RHS f64 | 0.960× | [0.939, 1.005] |
| Minimum-norm least squares 32 × 64, rank 32, 4 RHS f64 | 0.759× | [0.718, 0.769] |
| Coalesced tensor contraction 16 × 32 × 8 · 32 × 8 × 16 f64 | 0.059× | [0.058, 0.062] |
| Shared-variable backward sum(X * X), 64 × 32; prepared graph f64 | 0.969× | [0.908, 1.005] |
| Matmul · 2 workers · reused output 64 × 64 · 64 × 64 f64 | 0.940× | [0.468, 1.421] |
| Matmul · 2 workers · reused output 128 × 128 · 128 × 128 f64 | 0.774× | [0.769, 0.845] |
| Matmul · 2 workers · reused output 256 × 256 · 256 × 256 f64 | 0.736× | [0.725, 0.761] |
| Matmul · 2 workers · reused output 128 × 128 · 128 × 128 f32 | 0.769× | [0.438, 0.803] |
| Matrix multiplication 128 × 128 · 128 × 128 f64 | 1.017× | [0.982, 1.044] |
| Matrix multiplication 128 × 128 · 128 × 128 f32 | 0.994× | [0.958, 1.032] |
| Thin QR 96 × 32 f64 | 1.013× | [0.981, 1.061] |
| Thin QR 256 × 64 f64 | 1.004× | [0.970, 1.046] |
| Cholesky factorization 128 × 128 f64 | 1.000× | [0.983, 1.023] |

Full worker samples, accuracy checks, randomized order, executable hashes and source metadata: [paired.json](paired.json).
