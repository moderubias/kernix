# Clean CPU evidence: 0.5.0-registry-041f2a1

Measured source `041f2a158c3e51340b5b9e6df6886fa52584f9b5`, tree `4de2d0afb43c2127b26a5ed51d77d089016271d2`; dirty before/after: false.
Library source: registry 0.5.0. One pinned CPU; f64 operations and explicitly named f32 constructors; row-major inputs.

| Workload / library | Per-call p50 / p95 (us) | Batch-mean p50 / p95 (us) | Rust allocations | Peak additional requested bytes | Process peak RSS (MiB) |
|---|---:|---:|---:|---:|---:|
| cholesky_factor_64_f64/faer | 7.958 / 8.110 | 8.236 / 8.317 | 2 | 33280 | 4.04 |
| cholesky_factor_64_f64/kernix | 22.710 / 27.231 | 24.484 / 28.194 | 2 | 65536 | 4.07 |
| cholesky_factor_64_f64/nalgebra | 20.337 / 25.988 | 14.154 / 14.824 | 1 | 32768 | 3.84 |
| cholesky_factor_64_f64/numpy | 10.144 / 11.198 | 10.972 / 11.189 | unavailable | unavailable | 33.05 |
| create_ones_64x64_f32/candle | 0.216 / 0.231 | 0.230 / 0.235 | 7 | 16696 | 4.55 |
| create_ones_64x64_f32/faer | 0.284 / 0.292 | 0.172 / 0.185 | 1 | 16384 | 4.06 |
| create_ones_64x64_f32/kernix | 0.183 / 0.185 | 0.192 / 0.200 | 1 | 16384 | 3.89 |
| create_ones_64x64_f32/ndarray | 0.137 / 0.155 | 0.146 / 0.149 | 1 | 16384 | 3.78 |
| create_ones_64x64_f32/numpy | 1.339 / 1.592 | 1.325 / 1.339 | unavailable | unavailable | 32.69 |
| create_zeros_64x64_f32/candle | 0.152 / 0.168 | 0.164 / 0.171 | 5 | 16648 | 4.20 |
| create_zeros_64x64_f32/faer | 0.172 / 0.235 | 0.166 / 0.175 | 1 | 16384 | 4.07 |
| create_zeros_64x64_f32/kernix | 0.184 / 0.188 | 0.192 / 0.242 | 1 | 16384 | 3.82 |
| create_zeros_64x64_f32/ndarray | 0.099 / 0.100 | 0.094 / 0.102 | 1 | 16384 | 3.70 |
| create_zeros_64x64_f32/numpy | 0.383 / 0.420 | 0.400 / 0.497 | unavailable | unavailable | 32.76 |
| matmul_128_f64/candle | 70.406 / 77.392 | 73.624 / 77.219 | 9 | 131352 | 5.33 |
| matmul_128_f64/faer | 82.962 / 88.668 | 81.220 / 82.829 | 1 | 131072 | 4.59 |
| matmul_128_f64/kernix | 119.173 / 142.870 | 118.195 / 122.577 | 3 | 327680 | 4.46 |
| matmul_128_f64/nalgebra | 113.461 / 123.052 | 114.742 / 122.916 | 2 | 327680 | 4.20 |
| matmul_128_f64/ndarray | 112.665 / 126.473 | 118.598 / 122.508 | 2 | 327680 | 4.21 |
| matmul_128_f64/numpy | 74.499 / 79.894 | 78.699 / 83.266 | unavailable | unavailable | 34.09 |
| matmul_32_f64/candle | 1.496 / 1.672 | 1.497 / 1.512 | 9 | 8592 | 4.80 |
| matmul_32_f64/faer | 1.512 / 2.160 | 1.558 / 1.629 | 1 | 8192 | 4.18 |
| matmul_32_f64/kernix | 1.531 / 1.792 | 1.583 / 1.643 | 3 | 24576 | 3.98 |
| matmul_32_f64/nalgebra | 1.407 / 1.426 | 1.465 / 1.834 | 2 | 24576 | 3.79 |
| matmul_32_f64/ndarray | 1.493 / 1.669 | 1.444 / 1.525 | 2 | 24576 | 3.84 |
| matmul_32_f64/numpy | 2.620 / 3.326 | 2.610 / 3.154 | unavailable | unavailable | 32.95 |
| matmul_sum_backward_32x16x8_f64/candle | 3.982 / 4.410 | 4.247 / 4.456 | 113 | 27988 | 5.46 |
| matmul_sum_backward_32x16x8_f64/kernix | 2.954 / 3.558 | 2.972 / 3.106 | 19 | 18376 | 4.04 |
| qr_256x64_f64/faer | 426.510 / 448.319 | 424.901 / 455.453 | 7 | 331776 | 4.80 |
| qr_256x64_f64/kernix | 502.964 / 596.322 | 526.567 / 553.733 | 6 | 557568 | 4.75 |
| qr_256x64_f64/nalgebra | 380.362 / 513.470 | 378.185 / 423.091 | 4 | 295424 | 4.21 |
| qr_256x64_f64/numpy | 346.403 / 366.798 | 350.551 / 408.234 | unavailable | unavailable | 33.98 |
| qr_96x32_f64/faer | 65.855 / 76.152 | 70.660 / 75.438 | 7 | 67584 | 4.49 |
| qr_96x32_f64/kernix | 42.173 / 47.291 | 45.039 / 46.740 | 6 | 106752 | 4.16 |
| qr_96x32_f64/nalgebra | 37.531 / 42.209 | 37.521 / 38.272 | 4 | 57600 | 3.92 |
| qr_96x32_f64/numpy | 37.192 / 46.981 | 42.152 / 47.552 | unavailable | unavailable | 33.09 |
| softmax_256x64_f64/candle | 81.511 / 89.934 | 81.963 / 83.643 | 37 | 265200 | 5.64 |
| softmax_256x64_f64/kernix | 81.623 / 88.637 | 85.377 / 95.289 | 1 | 131072 | 4.20 |
| softmax_256x64_f64/ndarray | 71.440 / 76.295 | 73.641 / 74.654 | 1 | 131072 | 3.95 |
| softmax_256x64_f64/numpy | 102.059 / 107.784 | 100.013 / 103.424 | unavailable | unavailable | 33.03 |

## Method and limits

- Per-call quantiles use nearest ranks over 256 individual warmed calls; batch quantiles use 15 means over separate 10-ms batches. They are separate distributions. Raw samples remain in raw.json.
- Eight warmup calls follow validation. Timing includes allocation, dispatch, computation and output destruction. The first call field excludes output destruction. Inputs and library/runtime imports are prepared outside timing.
- Three extra processes record their first operation calls. These are first-use observations, not cache-flushed or device-startup latency. Timer-pair overhead is reported without subtraction; tiny calls are clock-sensitive.
- ndarray and Candle softmax are explicit stable compositions, not claims about a native softmax API.
- Constructor cases allocate owned 64x64 f32 tensors. Pre-0.6 registry baselines use full(value), explicitly marked in metadata; current workspace runs use zeros/ones. No nested literal reinterpretation is involved.
- All GEMM inputs are C-order and match elementwise. Native output layout differs by library. QR materializes thin Q and R in every implementation. Cholesky returns reusable factors (or NumPy L); no extra factor materialization is timed.
- Kernix checks finite values/symmetry. Some competitors validate a narrower contract. No finite-data precondition check was removed to improve timing. Accuracy results and budgets are retained per worker.
- The backward case uses an actual reverse-mode graph in Kernix and Candle for sum(A @ W), with A constant and W trainable. Both copy input/weight data and returned gradients; no manual derivative is used as a framework ranking.
- Allocations are measured in a separately compiled instrumented binary after warmup, outside all timing loops. Counts cover Rust global-allocator requests, not native allocator internals, stack memory or all NumPy allocations. NumPy allocation counts are unavailable, not zero.
- Requested-byte peak is additional live Rust allocation during one call, including output; it is not scratch-only memory or peak RSS. RSS is the process high-water mark including fixtures, validation and runtime, read from Linux VmHWM. Other platforms may report unavailable.
- The published cases are allocation-inclusive CPU APIs and graph workloads. Pure internal kernel-only and end-to-end application/model latency are not measured. No workload result implies a complete application frame rate.
- faer is explicitly serial; Rayon and BLAS thread counts are one. No GPU comparison. Python/NumPy timings include Python dispatch. Backend and dependency versions/configuration, governor and RAM are recorded.
- A shared laptop has uncontrolled load/frequency/cache effects. Report comparator wins and losses; no universal ranking follows. The source archive comes from committed bytes, with result directories excluded. Library dependencies are pinned by the saved driver lockfile.
