# Kernix 0.11.0 vs 0.10.0 · paired processes

Current: `runner-windows-latest-40059566395`. Baseline: `baseline-windows-latest-40059566395`.

9 fresh-process pairs per workload. Lower ratio is faster.
95% bootstrap intervals resample process pairs, not individual calls. Exploratory intervals do not correct for multiple comparisons.
Shared-machine drift and small sample counts remain limits; these are not per-call p95 intervals.

| Workload | Current / baseline latency | 95% interval |
|---|---:|---:|
| Matrix multiplication 128 × 128 · 128 × 128 f64 | 0.995× | [0.985, 1.004] |
| Matrix multiplication 128 × 128 · 128 × 128 f32 | 0.998× | [0.615, 1.000] |
| Matrix multiplication 256 × 256 · 256 × 256 f64 | 1.001× | [0.846, 1.009] |
| Matrix multiplication 256 × 256 · 256 × 256 f32 | 1.000× | [0.994, 1.001] |
| Independent matrix operands 128 × 128 · 128 × 128 f64 | 0.999× | [0.995, 1.488] |
| Independent matrix operands 128 × 128 · 128 × 128 f32 | 1.000× | [0.997, 1.004] |
| Cholesky factorization 128 × 128 f64 | 1.008× | [0.972, 1.011] |
| Cholesky factorization 256 × 256 f64 | 0.696× | [0.686, 0.713] |
| Cholesky factorization 512 × 512 f64 | 0.546× | [0.545, 0.554] |
| Matmul · 2 workers · reused output 128 × 128 · 128 × 128 f64 | 1.000× | [0.964, 1.013] |
| Matmul · 2 workers · reused output 256 × 256 · 256 × 256 f64 | 0.999× | [0.972, 1.006] |
| Matmul · 2 workers · reused output 128 × 128 · 128 × 128 f32 | 1.009× | [0.995, 1.037] |

Full worker samples, accuracy checks, randomized order, executable hashes and source metadata: [paired.json](paired.json).
