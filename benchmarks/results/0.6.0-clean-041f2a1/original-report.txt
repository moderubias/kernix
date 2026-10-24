# Clean CPU evidence: 0.6.0-clean-041f2a1

Measured source `041f2a158c3e51340b5b9e6df6886fa52584f9b5`, tree `4de2d0afb43c2127b26a5ed51d77d089016271d2`; dirty before/after: false.
Library source: workspace 041f2a158c3e51340b5b9e6df6886fa52584f9b5. One pinned CPU; f64 operations and explicitly named f32 constructors; row-major inputs.

| Workload / library | Per-call p50 / p95 (us) | Batch-mean p50 / p95 (us) | Rust allocations | Peak additional requested bytes | Process peak RSS (MiB) |
|---|---:|---:|---:|---:|---:|
| cholesky_factor_64_f64/faer | 7.904 / 8.040 | 8.188 / 8.705 | 2 | 33280 | 4.16 |
| cholesky_factor_64_f64/kernix | 22.931 / 25.260 | 24.261 / 30.336 | 2 | 65536 | 4.12 |
| cholesky_factor_64_f64/nalgebra | 13.031 / 14.325 | 13.621 / 14.861 | 1 | 32768 | 3.91 |
| cholesky_factor_64_f64/numpy | 10.924 / 17.209 | 10.749 / 11.189 | unavailable | unavailable | 32.58 |
| create_ones_64x64_f32/candle | 0.238 / 0.250 | 0.226 / 0.232 | 7 | 16696 | 4.59 |
| create_ones_64x64_f32/faer | 0.171 / 0.181 | 0.169 / 0.180 | 1 | 16384 | 3.86 |
| create_ones_64x64_f32/kernix | 0.183 / 0.277 | 0.188 / 0.194 | 1 | 16384 | 3.87 |
| create_ones_64x64_f32/ndarray | 0.155 / 0.156 | 0.145 / 0.150 | 1 | 16384 | 3.81 |
| create_ones_64x64_f32/numpy | 1.221 / 1.427 | 1.300 / 1.448 | unavailable | unavailable | 32.02 |
| create_zeros_64x64_f32/candle | 0.152 / 0.153 | 0.157 / 0.160 | 5 | 16648 | 4.34 |
| create_zeros_64x64_f32/faer | 0.190 / 0.194 | 0.166 / 0.199 | 1 | 16384 | 4.03 |
| create_zeros_64x64_f32/kernix | 0.134 / 0.137 | 0.140 / 0.144 | 1 | 16384 | 3.74 |
| create_zeros_64x64_f32/ndarray | 0.099 / 0.100 | 0.094 / 0.100 | 1 | 16384 | 3.70 |
| create_zeros_64x64_f32/numpy | 0.351 / 0.379 | 0.387 / 0.395 | unavailable | unavailable | 32.03 |
| matmul_128_f64/candle | 69.019 / 82.916 | 71.439 / 75.780 | 9 | 131352 | 5.36 |
| matmul_128_f64/faer | 76.137 / 78.465 | 77.306 / 78.972 | 1 | 131072 | 4.64 |
| matmul_128_f64/kernix | 113.767 / 127.927 | 116.137 / 122.255 | 3 | 327680 | 4.45 |
| matmul_128_f64/nalgebra | 110.415 / 122.328 | 112.113 / 118.237 | 2 | 327680 | 4.07 |
| matmul_128_f64/ndarray | 117.314 / 125.183 | 115.904 / 131.648 | 2 | 327680 | 4.37 |
| matmul_128_f64/numpy | 72.997 / 79.055 | 75.236 / 79.746 | unavailable | unavailable | 33.68 |
| matmul_32_f64/candle | 1.505 / 1.696 | 1.500 / 1.534 | 9 | 8592 | 4.92 |
| matmul_32_f64/faer | 1.493 / 1.546 | 1.480 / 1.721 | 1 | 8192 | 4.16 |
| matmul_32_f64/kernix | 1.541 / 1.944 | 1.563 / 1.644 | 3 | 24576 | 3.98 |
| matmul_32_f64/nalgebra | 1.494 / 1.598 | 1.428 / 1.569 | 2 | 24576 | 3.73 |
| matmul_32_f64/ndarray | 1.407 / 1.555 | 1.451 / 1.666 | 2 | 24576 | 3.64 |
| matmul_32_f64/numpy | 2.426 / 3.284 | 2.483 / 2.569 | unavailable | unavailable | 32.36 |
| matmul_sum_backward_32x16x8_f64/candle | 3.914 / 4.704 | 4.026 / 4.271 | 113 | 27988 | 5.59 |
| matmul_sum_backward_32x16x8_f64/kernix | 3.165 / 3.745 | 3.020 / 3.206 | 19 | 18376 | 4.00 |
| qr_256x64_f64/faer | 393.041 / 426.261 | 404.782 / 418.253 | 7 | 331776 | 5.08 |
| qr_256x64_f64/kernix | 468.986 / 524.363 | 480.923 / 519.950 | 6 | 557568 | 4.88 |
| qr_256x64_f64/nalgebra | 356.914 / 382.462 | 358.303 / 409.421 | 4 | 295424 | 4.27 |
| qr_256x64_f64/numpy | 326.424 / 351.262 | 329.967 / 343.904 | unavailable | unavailable | 33.51 |
| qr_96x32_f64/faer | 69.906 / 95.294 | 66.129 / 69.041 | 7 | 67584 | 4.54 |
| qr_96x32_f64/kernix | 45.810 / 49.252 | 46.038 / 46.735 | 6 | 106752 | 4.06 |
| qr_96x32_f64/nalgebra | 37.619 / 41.175 | 38.282 / 55.679 | 4 | 57600 | 3.66 |
| qr_96x32_f64/numpy | 36.491 / 39.224 | 38.338 / 39.856 | unavailable | unavailable | 32.70 |
| softmax_256x64_f64/candle | 77.876 / 83.362 | 75.333 / 92.035 | 37 | 265200 | 5.69 |
| softmax_256x64_f64/kernix | 84.758 / 97.453 | 85.652 / 90.543 | 1 | 131072 | 4.30 |
| softmax_256x64_f64/ndarray | 67.896 / 72.941 | 69.009 / 71.483 | 1 | 131072 | 4.25 |
| softmax_256x64_f64/numpy | 97.789 / 118.113 | 97.894 / 108.880 | unavailable | unavailable | 32.61 |

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
