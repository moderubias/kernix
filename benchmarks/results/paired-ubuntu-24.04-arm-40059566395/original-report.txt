# Kernix 0.11.0 vs 0.10.0 · paired processes

Current: `runner-ubuntu-24.04-arm-40059566395`. Baseline: `baseline-ubuntu-24.04-arm-40059566395`.

9 fresh-process pairs per workload. Lower ratio is faster.
95% bootstrap intervals resample process pairs, not individual calls. Exploratory intervals do not correct for multiple comparisons.
Shared-machine drift and small sample counts remain limits; these are not per-call p95 intervals.

| Workload | Current / baseline latency | 95% interval |
|---|---:|---:|
| Matrix multiplication 128 × 128 · 128 × 128 f64 | 0.804× | [0.796, 0.808] |
| Matrix multiplication 128 × 128 · 128 × 128 f32 | 0.910× | [0.908, 0.912] |
| Matrix multiplication 256 × 256 · 256 × 256 f64 | 0.994× | [0.993, 0.997] |
| Matrix multiplication 256 × 256 · 256 × 256 f32 | 0.862× | [0.860, 0.864] |
| Independent matrix operands 128 × 128 · 128 × 128 f64 | 0.797× | [0.785, 0.801] |
| Independent matrix operands 128 × 128 · 128 × 128 f32 | 0.906× | [0.901, 0.910] |
| Cholesky factorization 128 × 128 f64 | 1.004× | [1.001, 1.005] |
| Cholesky factorization 256 × 256 f64 | 0.387× | [0.385, 0.389] |
| Cholesky factorization 512 × 512 f64 | 0.285× | [0.285, 0.286] |
| Matmul · 2 workers · reused output 128 × 128 · 128 × 128 f64 | 0.945× | [0.928, 0.975] |
| Matmul · 2 workers · reused output 256 × 256 · 256 × 256 f64 | 0.835× | [0.827, 0.841] |
| Matmul · 2 workers · reused output 128 × 128 · 128 × 128 f32 | 0.986× | [0.981, 0.994] |

Full worker samples, accuracy checks, randomized order, executable hashes and source metadata: [paired.json](paired.json).
