# Kernix 0.7.2 · CPU benchmarks

12th Gen Intel(R) Core(TM) i3-1215U · one CPU · allocation-inclusive · warmed individual calls

**Microseconds per call, median. Lower is faster; bold marks the lowest observed median in each row.**

Baseline: 0.7.1-registry-5067333. Δ is latency change; negative is faster. Runs are sequential, not randomized/interleaved.

| Workload | Kernix | ndarray | nalgebra | faer | Candle | NumPy | Kernix Δ vs baseline |
|---|---:|---:|---:|---:|---:|---:|---:|
| Matrix multiplication · 32 × 32 · 32 × 32 · f64 | 1.535 | **1.402** | 1.504 | 1.631 | 1.573 | 2.741 | -6.9% |
| Matrix multiplication · 64 × 64 · 64 × 64 · f64 | **9.120** | 10.517 | 11.415 | 10.448 | 9.915 | 12.512 | -23.5% |
| Matrix multiplication · 128 × 128 · 128 × 128 · f64 | **68.456** | 115.016 | 112.896 | 81.040 | 71.202 | 75.573 | -43.1% |
| Matrix multiplication · 256 × 256 · 256 × 256 · f64 | **548.673** | 632.796 | 640.174 | 711.270 | 560.287 | 592.335 | -18.7% |
| Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64 | **1.900** | — | — | — | 1.974 | 3.117 | -4.7% |
| Thin QR · 96 × 32 · f64 | 46.649 | — | **35.130** | 65.928 | — | 38.736 | +12.1% |
| Thin QR · 256 × 64 · f64 | 487.300 | — | 363.922 | 405.668 | — | **341.148** | -1.6% |
| Cholesky factorization · 64 × 64 · f64 | 25.250 | — | 13.459 | **7.263** | — | 10.064 | +11.0% |
| Row softmax · 256 × 64 · f64 | 86.258 | **70.921** | — | — | 78.182 | 102.402 | +3.7% |
| Create zeros · 64 × 64 · f32 | 0.145 | **0.089** | — | 0.172 | 0.167 | 0.423 | -2.0% |
| Create ones · 64 × 64 · f32 | 0.185 | **0.144** | — | 0.201 | 0.237 | 1.234 | -4.1% |
| Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64 | **2.382** | — | — | — | 6.678 | — | -7.8% |
| Model inference · batched normalized/sliced MLP · f64 | **1.966** | — | — | — | 3.479 | — | -4.7% |
| Model recording + forward · batched normalized/sliced MLP · f64 | **2.438** | — | — | — | 5.299 | — | -4.9% |
| Model backward · prepared graph → fresh gradients · f64 | **3.154** | — | — | — | 14.641 | — | -2.4% |
| Parameter update · transactional SGD, fixed gradients · f64 | **0.489** | — | — | — | — | — | -18.0% |

![Relative latency by workload](latency.png)

Each panel has its own scale. Ratios are library / Kernix median; below 1 means the comparator was faster.

## Reading these results

- One shared laptop, one pinned CPU and one BLAS/Rayon thread. Frequency and background load are uncontrolled; these observations establish no universal ranking.
- Timing includes output allocation and destruction. Inputs and imports are prepared outside timing. GEMM inputs are row-major; output layout and validation contracts can differ by library.
- Per-call p95 describes 256 individual warmed calls. The 15 batch means form a separate distribution; neither is a confidence interval.
- Rust allocation probes use a separate instrumented binary. Native engines may retain packing buffers; warmed probes exclude their first-use allocation and retention. NumPy native allocations are unavailable, never zero.

[Interactive report](report.html) · [Detailed measurements and method](DETAILS.md) · [Raw samples and provenance](raw.json) · [Printable chart](latency.pdf)

Measured source `50673332cd664b98fad90a24febce1fd8d6395b0`. Run `0.7.2-clean-5067333`. Rendering does not remeasure or modify raw evidence.
