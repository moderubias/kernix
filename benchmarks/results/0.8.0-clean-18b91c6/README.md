# Kernix 0.8.0 · CPU benchmarks

12th Gen Intel(R) Core(TM) i3-1215U · one CPU · allocation-inclusive · warmed individual calls

**Microseconds per call, median. Lower is faster; bold marks the lowest observed median in each row.**

Baseline: 0.7.3-registry-fix080. Δ is latency change; negative is faster. Runs are sequential, not randomized/interleaved.

| Workload | Kernix | ndarray | nalgebra | faer | Candle | NumPy | Kernix Δ vs baseline |
|---|---:|---:|---:|---:|---:|---:|---:|
| Matrix multiplication · 32 × 32 · 32 × 32 · f64 | 1.665 | 1.468 | **1.397** | 1.488 | 1.471 | 2.609 | -4.1% |
| Matrix multiplication · 64 × 64 · 64 × 64 · f64 | **9.375** | 10.540 | 11.111 | 11.000 | 9.730 | 11.883 | -3.0% |
| Matrix multiplication · 128 × 128 · 128 × 128 · f64 | 75.270 | 111.053 | 116.638 | 87.089 | **73.790** | 74.650 | +2.5% |
| Matrix multiplication · 256 × 256 · 256 × 256 · f64 | **564.049** | 647.431 | 684.167 | 762.295 | 574.306 | 617.042 | -1.1% |
| Matrix multiplication · 64 × 64 · 64 × 64 · f32 | **4.958** | 6.093 | 6.041 | 5.229 | 5.286 | 6.192 | -2.7% |
| Matrix multiplication · 128 × 128 · 128 × 128 · f32 | **36.784** | 51.210 | 51.276 | 41.806 | 36.900 | 38.176 | -26.6% |
| Matrix multiplication · 256 × 256 · 256 × 256 · f32 | **279.714** | 384.878 | 405.165 | 339.198 | 288.207 | 283.696 | +0.2% |
| Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64 | **1.945** | — | — | — | 2.229 | 3.030 | -1.0% |
| Thin QR · 96 × 32 · f64 | 47.423 | — | **38.369** | 70.982 | — | 39.069 | +0.8% |
| Thin QR · 256 × 64 · f64 | 484.243 | — | 372.930 | 416.558 | — | **348.582** | -5.9% |
| Cholesky factorization · 64 × 64 · f64 | 21.310 | — | 14.273 | **8.474** | — | 10.673 | -15.5% |
| Cholesky factorization · 128 × 128 · f64 | 121.712 | — | 98.537 | **38.466** | — | 72.842 | -27.7% |
| Cholesky factorization · 256 × 256 · f64 | 633.171 | — | 668.195 | **197.958** | — | 512.867 | -48.1% |
| Row softmax · 256 × 64 · f64 | 87.073 | **75.498** | — | — | 79.161 | 110.763 | +2.0% |
| Create zeros · 64 × 64 · f32 | 0.222 | **0.104** | — | 0.159 | 0.168 | 0.366 | +52.1% |
| Create ones · 64 × 64 · f32 | 0.204 | **0.141** | — | 0.203 | 0.215 | 1.349 | +0.0% |
| Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64 | **2.572** | — | — | — | 4.144 | — | -3.2% |
| Model inference · batched normalized/sliced MLP · f64 | **2.224** | — | — | — | 3.466 | — | -0.6% |
| Model recording + forward · batched normalized/sliced MLP · f64 | **2.720** | — | — | — | 5.085 | — | -2.9% |
| Model backward · prepared graph → fresh gradients · f64 | **3.056** | — | — | — | 14.834 | — | -1.5% |
| Parameter update · transactional SGD, fixed gradients · f64 | **0.529** | — | — | — | — | — | -0.6% |

![Relative latency by workload](latency.png)

Each panel has its own scale. Ratios are library / Kernix median; below 1 means the comparator was faster.

## Reading these results

- One shared laptop, one pinned CPU and one BLAS/Rayon thread. Frequency and background load are uncontrolled; these observations establish no universal ranking.
- Timing includes output allocation and destruction. Inputs and imports are prepared outside timing. GEMM inputs are row-major; output layout and validation contracts can differ by library.
- Per-call p95 describes 256 individual warmed calls. The 15 batch means form a separate distribution; neither is a confidence interval.
- Rust allocation probes use a separate instrumented binary. Native engines may retain packing buffers; warmed probes exclude their first-use allocation and retention. NumPy native allocations are unavailable, never zero.

[Interactive report](report.html) · [Detailed measurements and method](DETAILS.md) · [Raw samples and provenance](raw.json) · [Printable chart](latency.pdf)

Measured source `18b91c6524b593c802cf03d93a1611c322f7d878`. Run `0.8.0-clean-18b91c6`. Rendering does not remeasure or modify raw evidence.
