# Kernix 0.8.0 · CPU benchmarks

AMD EPYC 7763 64-Core Processor · one BLAS/Rayon thread · allocation-inclusive · warmed individual calls

**Microseconds per call, median. Lower is faster; bold marks the lowest observed median in each row.**

No previous-version baseline is paired with this view.

| Workload | Kernix | ndarray | nalgebra | faer | Candle | NumPy |
|---|---:|---:|---:|---:|---:|---:|
| Matrix multiplication · 32 × 32 · 32 × 32 · f64 | 2.800 | 2.400 | **2.300** | 2.500 | 2.800 | 4.200 |
| Matrix multiplication · 64 × 64 · 64 × 64 · f64 | **13.500** | 53.700 | 50.800 | 14.500 | 27.900 | 15.800 |
| Matrix multiplication · 128 × 128 · 128 × 128 · f64 | 137.700 | 115.300 | 115.900 | 116.700 | 136.900 | **100.800** |
| Matrix multiplication · 256 × 256 · 256 × 256 · f64 | 843.800 | 1164.600 | 908.200 | 1088.100 | 867.900 | **757.500** |
| Matrix multiplication · 64 × 64 · 64 × 64 · f32 | **7.000** | 7.900 | 27.300 | 7.100 | 7.500 | 10.400 |
| Matrix multiplication · 128 × 128 · 128 × 128 · f32 | 76.000 | 115.400 | 53.700 | **50.200** | 72.400 | 56.200 |
| Matrix multiplication · 256 × 256 · 256 × 256 · f32 | 440.400 | 397.500 | 399.700 | 424.900 | 432.300 | **391.500** |
| Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64 | **3.400** | — | — | — | 4.000 | 5.100 |
| Thin QR · 96 × 32 · f64 | 105.200 | — | 79.600 | 83.700 | — | **77.000** |
| Thin QR · 256 × 64 · f64 | 810.100 | — | 564.800 | **439.400** | — | 552.600 |
| Cholesky factorization · 64 × 64 · f64 | 35.800 | — | 20.100 | **14.500** | — | 21.200 |
| Cholesky factorization · 128 × 128 · f64 | 221.700 | — | 112.500 | **59.200** | — | 113.700 |
| Cholesky factorization · 256 × 256 · f64 | 1012.900 | — | 859.800 | **313.500** | — | 948.800 |
| Row softmax · 256 × 64 · f64 | **112.500** | 162.900 | — | — | 155.000 | 134.500 |
| Create zeros · 64 × 64 · f32 | 0.400 | **0.300** | — | 0.500 | 0.500 | 0.800 |
| Create ones · 64 × 64 · f32 | 0.500 | **0.400** | — | 0.500 | 1.300 | 2.100 |
| Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64 | **5.100** | — | — | — | 10.200 | — |
| Model inference · batched normalized/sliced MLP · f64 | **3.800** | — | — | — | 8.200 | — |
| Model recording + forward · batched normalized/sliced MLP · f64 | **5.000** | — | — | — | 10.600 | — |
| Model backward · prepared graph → fresh gradients · f64 | **6.000** | — | — | — | 31.500 | — |
| Parameter update · transactional SGD, fixed gradients · f64 | **1.000** | — | — | — | — | — |

![Relative latency by workload](latency.png)

Each panel has its own scale. Ratios are library / Kernix median; below 1 means the comparator was faster.

## Reading these results

- One shared laptop, one pinned CPU and one BLAS/Rayon thread. Frequency and background load are uncontrolled; these observations establish no universal ranking.
- Timing includes output allocation and destruction. Inputs and imports are prepared outside timing. GEMM inputs are row-major; output layout and validation contracts can differ by library.
- Per-call p95 describes 256 individual warmed calls. The 15 batch means form a separate distribution; neither is a confidence interval.
- Rust allocation probes use a separate instrumented binary. Native engines may retain packing buffers; warmed probes exclude their first-use allocation and retention. NumPy native allocations are unavailable, never zero.

[Interactive report](report.html) · [Detailed measurements and method](DETAILS.md) · [Raw samples and provenance](raw.json) · [Printable chart](latency.pdf)

Measured source `227f339974f9c31fcf6bd37ccccbda3eedc99591`. Run `runner-windows-latest-39719312815`. Rendering does not remeasure or modify raw evidence.
