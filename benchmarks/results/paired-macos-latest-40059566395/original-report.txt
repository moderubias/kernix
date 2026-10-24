# Kernix 0.11.0 vs 0.10.0 · paired processes

Current: `runner-macos-latest-40059566395`. Baseline: `baseline-macos-latest-40059566395`.

9 fresh-process pairs per workload. Lower ratio is faster.
95% bootstrap intervals resample process pairs, not individual calls. Exploratory intervals do not correct for multiple comparisons.
Shared-machine drift and small sample counts remain limits; these are not per-call p95 intervals.

| Workload | Current / baseline latency | 95% interval |
|---|---:|---:|
| Matrix multiplication 128 × 128 · 128 × 128 f64 | 0.981× | [0.964, 1.032] |
| Matrix multiplication 128 × 128 · 128 × 128 f32 | 1.018× | [0.980, 1.057] |
| Matrix multiplication 256 × 256 · 256 × 256 f64 | 1.006× | [0.917, 1.012] |
| Matrix multiplication 256 × 256 · 256 × 256 f32 | 0.981× | [0.913, 1.001] |
| Independent matrix operands 128 × 128 · 128 × 128 f64 | 0.981× | [0.960, 1.000] |
| Independent matrix operands 128 × 128 · 128 × 128 f32 | 1.000× | [0.983, 1.036] |
| Cholesky factorization 128 × 128 f64 | 1.001× | [1.000, 1.062] |
| Cholesky factorization 256 × 256 f64 | 0.520× | [0.510, 0.522] |
| Cholesky factorization 512 × 512 f64 | 0.372× | [0.368, 0.398] |
| Matmul · 2 workers · reused output 128 × 128 · 128 × 128 f64 | 0.910× | [0.894, 0.968] |
| Matmul · 2 workers · reused output 256 × 256 · 256 × 256 f64 | 0.889× | [0.876, 0.927] |
| Matmul · 2 workers · reused output 128 × 128 · 128 × 128 f32 | 0.796× | [0.768, 0.829] |

Full worker samples, accuracy checks, randomized order, executable hashes and source metadata: [paired.json](paired.json).
