# Kernix 0.7.3 · CPU benchmarks

12th Gen Intel(R) Core(TM) i3-1215U · one CPU · allocation-inclusive · warmed individual calls

**Microseconds per call, median. Lower is faster; bold marks the lowest observed median in each row.**

Baseline: 0.7.2-registry-bc459f4. Δ is latency change; negative is faster. Runs are sequential, not randomized/interleaved.

| Workload | Kernix | ndarray | nalgebra | faer | Candle | NumPy | Kernix Δ vs baseline |
|---|---:|---:|---:|---:|---:|---:|---:|
| Matrix multiplication · 32 × 32 · 32 × 32 · f64 | 1.535 | **1.492** | 1.612 | 1.600 | 1.525 | 2.808 | -3.4% |
| Matrix multiplication · 64 × 64 · 64 × 64 · f64 | **9.162** | 11.312 | 11.075 | 10.701 | 9.671 | 12.827 | +0.3% |
| Matrix multiplication · 128 × 128 · 128 × 128 · f64 | 73.624 | 117.855 | 115.569 | 82.494 | **73.614** | 78.457 | +0.8% |
| Matrix multiplication · 256 × 256 · 256 × 256 · f64 | **574.036** | 665.964 | 662.924 | 722.614 | 575.843 | 612.150 | +0.2% |
| Matrix multiplication · 64 × 64 · 64 × 64 · f32 | 6.441 | 6.001 | 5.791 | 5.676 | **4.998** | 6.464 | +9.5% |
| Matrix multiplication · 128 × 128 · 128 × 128 · f32 | **34.749** | 52.912 | 49.771 | 40.606 | 36.865 | 39.432 | -29.3% |
| Matrix multiplication · 256 × 256 · 256 × 256 · f32 | **270.684** | 384.756 | 386.854 | 310.024 | 271.808 | 286.792 | -31.2% |
| Batched multiplication · 4 × 16 × 32 · 1 × 32 × 16 · f64 | **1.881** | — | — | — | 1.995 | 3.391 | -2.8% |
| Thin QR · 96 × 32 · f64 | 42.494 | — | **36.530** | 70.197 | — | 38.405 | -0.6% |
| Thin QR · 256 × 64 · f64 | 506.246 | — | 368.816 | 423.772 | — | **340.898** | +1.9% |
| Cholesky factorization · 64 × 64 · f64 | 25.132 | — | 14.198 | **8.459** | — | 10.865 | +10.2% |
| Row softmax · 256 × 64 · f64 | 84.139 | **68.781** | — | — | 77.918 | 102.145 | -1.4% |
| Create zeros · 64 × 64 · f32 | 0.143 | **0.096** | — | 0.155 | 0.170 | 0.413 | +5.9% |
| Create ones · 64 × 64 · f32 | 0.185 | **0.144** | — | 0.181 | 0.231 | 1.390 | -8.0% |
| Record + backward · sum(A · W), 32 × 16 · 16 × 8 · f64 | **2.378** | — | — | — | 4.273 | — | -1.9% |
| Model inference · batched normalized/sliced MLP · f64 | **2.029** | — | — | — | 3.582 | — | +1.4% |
| Model recording + forward · batched normalized/sliced MLP · f64 | **2.699** | — | — | — | 5.252 | — | -2.8% |
| Model backward · prepared graph → fresh gradients · f64 | **3.160** | — | — | — | 14.881 | — | -1.1% |
| Parameter update · transactional SGD, fixed gradients · f64 | **0.528** | — | — | — | — | — | +2.7% |

![Relative latency by workload](latency.png)

Each panel has its own scale. Ratios are library / Kernix median; below 1 means the comparator was faster.

## Reading these results

- One shared laptop, one pinned CPU and one BLAS/Rayon thread. Frequency and background load are uncontrolled; these observations establish no universal ranking.
- Timing includes output allocation and destruction. Inputs and imports are prepared outside timing. GEMM inputs are row-major; output layout and validation contracts can differ by library.
- Per-call p95 describes 256 individual warmed calls. The 15 batch means form a separate distribution; neither is a confidence interval.
- Rust allocation probes use a separate instrumented binary. Native engines may retain packing buffers; warmed probes exclude their first-use allocation and retention. NumPy native allocations are unavailable, never zero.

[Interactive report](report.html) · [Detailed measurements and method](DETAILS.md) · [Raw samples and provenance](raw.json) · [Printable chart](latency.pdf)

Measured source `bc459f44b276e86275cc4e431e4946443aefb15f`. Run `0.7.3-clean-bc459f4`. Rendering does not remeasure or modify raw evidence.
