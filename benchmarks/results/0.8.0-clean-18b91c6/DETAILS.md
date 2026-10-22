# Clean CPU evidence: 0.8.0-clean-18b91c6

Measured source `18b91c6524b593c802cf03d93a1611c322f7d878`, tree `c86e2762823ac567f8a7893bb8d38d39b0c4bddc`; dirty before/after: false.
Library source: workspace 18b91c6524b593c802cf03d93a1611c322f7d878. One pinned CPU; dtype explicitly named per workload; row-major inputs.

| Workload / library | Per-call p50 / p95 (us) | Batch-mean p50 / p95 (us) | Rust allocations | Peak additional requested bytes | Process peak RSS (MiB) |
|---|---:|---:|---:|---:|---:|
| cholesky_factor_128_f64/faer | 38.466 / 39.725 | 39.400 / 40.513 | 2 | 132096 | 5.13 |
| cholesky_factor_128_f64/kernix | 121.712 / 127.809 | 117.083 / 123.187 | 1 | 131072 | 4.33 |
| cholesky_factor_128_f64/nalgebra | 98.537 / 105.605 | 170.730 / 187.864 | 1 | 131072 | 4.17 |
| cholesky_factor_128_f64/numpy | 72.842 / 75.455 | 74.759 / 77.725 | unavailable | unavailable | 41.74 |
| cholesky_factor_256_f64/faer | 197.958 / 210.303 | 203.710 / 219.184 | 2 | 526336 | 5.96 |
| cholesky_factor_256_f64/kernix | 633.171 / 661.512 | 628.592 / 650.344 | 1 | 524288 | 5.51 |
| cholesky_factor_256_f64/nalgebra | 668.195 / 716.806 | 692.612 / 729.411 | 1 | 524288 | 4.89 |
| cholesky_factor_256_f64/numpy | 512.867 / 574.590 | 525.344 / 561.527 | unavailable | unavailable | 45.83 |
| cholesky_factor_64_f64/faer | 8.474 / 8.526 | 8.558 / 8.628 | 2 | 33280 | 4.54 |
| cholesky_factor_64_f64/kernix | 21.310 / 23.048 | 22.170 / 24.131 | 2 | 33280 | 4.04 |
| cholesky_factor_64_f64/nalgebra | 14.273 / 14.321 | 14.563 / 15.328 | 1 | 32768 | 3.90 |
| cholesky_factor_64_f64/numpy | 10.673 / 11.154 | 11.149 / 11.809 | unavailable | unavailable | 41.11 |
| create_ones_64x64_f32/candle | 0.215 / 0.217 | 0.236 / 0.247 | 7 | 16696 | 4.68 |
| create_ones_64x64_f32/faer | 0.203 / 0.244 | 0.180 / 0.184 | 1 | 16384 | 4.28 |
| create_ones_64x64_f32/kernix | 0.204 / 0.207 | 0.203 / 0.208 | 1 | 16384 | 3.89 |
| create_ones_64x64_f32/ndarray | 0.141 / 0.145 | 0.147 / 0.164 | 1 | 16384 | 3.87 |
| create_ones_64x64_f32/numpy | 1.349 / 2.206 | 1.363 / 1.456 | unavailable | unavailable | 40.49 |
| create_zeros_64x64_f32/candle | 0.168 / 0.170 | 0.159 / 0.172 | 5 | 16648 | 4.48 |
| create_zeros_64x64_f32/faer | 0.159 / 0.162 | 0.198 / 0.202 | 1 | 16384 | 4.35 |
| create_zeros_64x64_f32/kernix | 0.222 / 0.314 | 0.149 / 0.193 | 1 | 16384 | 3.87 |
| create_zeros_64x64_f32/ndarray | 0.104 / 0.146 | 0.094 / 0.096 | 1 | 16384 | 3.87 |
| create_zeros_64x64_f32/numpy | 0.366 / 0.402 | 0.406 / 0.782 | unavailable | unavailable | 40.52 |
| matmul_128_f32/candle | 36.900 / 57.442 | 37.634 / 38.285 | 9 | 65816 | 5.20 |
| matmul_128_f32/faer | 41.806 / 42.913 | 41.310 / 42.819 | 1 | 65536 | 4.82 |
| matmul_128_f32/kernix | 36.784 / 38.511 | 37.318 / 38.272 | 3 | 65544 | 4.60 |
| matmul_128_f32/nalgebra | 51.276 / 56.796 | 52.231 / 53.323 | 2 | 163840 | 4.31 |
| matmul_128_f32/ndarray | 51.210 / 64.891 | 52.379 / 54.132 | 2 | 163840 | 4.39 |
| matmul_128_f32/numpy | 38.176 / 40.243 | 40.527 / 42.012 | unavailable | unavailable | 41.84 |
| matmul_128_f64/candle | 73.790 / 80.265 | 75.595 / 98.686 | 9 | 131352 | 5.55 |
| matmul_128_f64/faer | 87.089 / 92.625 | 85.760 / 87.381 | 1 | 131072 | 4.83 |
| matmul_128_f64/kernix | 75.270 / 80.751 | 75.763 / 76.457 | 3 | 131080 | 4.98 |
| matmul_128_f64/nalgebra | 116.638 / 121.901 | 120.217 / 137.664 | 2 | 327680 | 4.46 |
| matmul_128_f64/ndarray | 111.053 / 122.755 | 116.339 / 147.313 | 2 | 327680 | 4.53 |
| matmul_128_f64/numpy | 74.650 / 90.719 | 78.802 / 81.192 | unavailable | unavailable | 41.90 |
| matmul_256_f32/candle | 288.207 / 300.434 | 293.921 / 379.243 | 9 | 262424 | 5.94 |
| matmul_256_f32/faer | 339.198 / 352.097 | 342.249 / 356.441 | 1 | 262144 | 5.12 |
| matmul_256_f32/kernix | 279.714 / 304.026 | 282.363 / 290.486 | 3 | 262152 | 5.32 |
| matmul_256_f32/nalgebra | 405.165 / 429.482 | 412.403 / 424.740 | 2 | 589824 | 4.81 |
| matmul_256_f32/ndarray | 384.878 / 418.325 | 392.721 / 416.347 | 2 | 589824 | 4.46 |
| matmul_256_f32/numpy | 283.696 / 297.111 | 287.521 / 290.482 | unavailable | unavailable | 46.03 |
| matmul_256_f64/candle | 574.306 / 594.213 | 589.419 / 859.635 | 9 | 524568 | 7.02 |
| matmul_256_f64/faer | 762.295 / 791.443 | 765.764 / 818.138 | 1 | 524288 | 6.04 |
| matmul_256_f64/kernix | 564.049 / 585.096 | 575.562 / 604.452 | 3 | 524296 | 6.24 |
| matmul_256_f64/nalgebra | 684.167 / 727.433 | 684.286 / 695.689 | 2 | 1179648 | 5.65 |
| matmul_256_f64/ndarray | 647.431 / 682.906 | 648.903 / 663.322 | 2 | 1179648 | 6.11 |
| matmul_256_f64/numpy | 617.042 / 642.462 | 623.105 / 643.041 | unavailable | unavailable | 45.83 |
| matmul_32_f64/candle | 1.471 / 1.505 | 1.560 / 2.393 | 9 | 8592 | 5.05 |
| matmul_32_f64/faer | 1.488 / 1.511 | 1.558 / 2.137 | 1 | 8192 | 4.00 |
| matmul_32_f64/kernix | 1.665 / 1.697 | 1.727 / 1.784 | 3 | 24576 | 3.92 |
| matmul_32_f64/nalgebra | 1.397 / 1.413 | 1.471 / 1.876 | 2 | 24576 | 3.98 |
| matmul_32_f64/ndarray | 1.468 / 1.522 | 1.472 / 2.447 | 2 | 24576 | 4.04 |
| matmul_32_f64/numpy | 2.609 / 4.042 | 2.625 / 2.738 | unavailable | unavailable | 40.79 |
| matmul_64_f32/candle | 5.286 / 5.402 | 5.245 / 5.416 | 9 | 16664 | 5.23 |
| matmul_64_f32/faer | 5.229 / 5.402 | 5.263 / 5.506 | 1 | 16384 | 4.25 |
| matmul_64_f32/kernix | 4.958 / 5.083 | 5.147 / 5.475 | 3 | 16544 | 4.68 |
| matmul_64_f32/nalgebra | 6.041 / 6.090 | 6.045 / 6.102 | 2 | 49152 | 3.77 |
| matmul_64_f32/ndarray | 6.093 / 7.072 | 5.903 / 6.180 | 2 | 49152 | 4.27 |
| matmul_64_f32/numpy | 6.192 / 6.714 | 6.286 / 6.304 | unavailable | unavailable | 40.96 |
| matmul_64_f64/candle | 9.730 / 10.039 | 10.096 / 10.731 | 9 | 33048 | 5.01 |
| matmul_64_f64/faer | 11.000 / 11.180 | 11.079 / 11.231 | 1 | 32768 | 4.55 |
| matmul_64_f64/kernix | 9.375 / 10.352 | 10.039 / 10.094 | 3 | 32928 | 4.69 |
| matmul_64_f64/nalgebra | 11.111 / 11.315 | 11.435 / 11.685 | 2 | 98304 | 4.18 |
| matmul_64_f64/ndarray | 10.540 / 10.643 | 11.086 / 11.298 | 2 | 98304 | 4.12 |
| matmul_64_f64/numpy | 11.883 / 14.276 | 12.495 / 12.808 | unavailable | unavailable | 41.00 |
| matmul_batched_4x16x32x16_f64/candle | 2.229 / 2.280 | 2.283 / 2.393 | 25 | 25504 | 5.48 |
| matmul_batched_4x16x32x16_f64/kernix | 1.945 / 1.997 | 2.053 / 2.067 | 7 | 16392 | 4.38 |
| matmul_batched_4x16x32x16_f64/numpy | 3.030 / 3.090 | 3.109 / 3.207 | unavailable | unavailable | 40.76 |
| matmul_sum_backward_32x16x8_f64/candle | 4.144 / 4.344 | 4.255 / 4.344 | 113 | 27988 | 5.62 |
| matmul_sum_backward_32x16x8_f64/kernix | 2.572 / 2.617 | 2.649 / 2.808 | 16 | 18376 | 4.29 |
| qr_256x64_f64/faer | 416.558 / 467.942 | 417.482 / 447.828 | 7 | 331776 | 5.33 |
| qr_256x64_f64/kernix | 484.243 / 511.016 | 497.120 / 510.847 | 6 | 426496 | 4.83 |
| qr_256x64_f64/nalgebra | 372.930 / 384.504 | 372.643 / 377.775 | 4 | 295424 | 4.32 |
| qr_256x64_f64/numpy | 348.582 / 363.440 | 354.572 / 372.674 | unavailable | unavailable | 42.44 |
| qr_96x32_f64/faer | 70.982 / 84.389 | 71.183 / 74.795 | 7 | 67584 | 4.98 |
| qr_96x32_f64/kernix | 47.423 / 49.301 | 48.920 / 50.219 | 6 | 82176 | 4.33 |
| qr_96x32_f64/nalgebra | 38.369 / 39.543 | 38.223 / 44.158 | 4 | 57600 | 4.10 |
| qr_96x32_f64/numpy | 39.069 / 51.114 | 40.029 / 41.293 | unavailable | unavailable | 41.69 |
| softmax_256x64_f64/candle | 79.161 / 83.885 | 79.584 / 80.966 | 37 | 265200 | 5.94 |
| softmax_256x64_f64/kernix | 87.073 / 92.849 | 87.665 / 89.672 | 1 | 131072 | 4.48 |
| softmax_256x64_f64/ndarray | 75.498 / 105.525 | 75.580 / 79.507 | 1 | 131072 | 4.41 |
| softmax_256x64_f64/numpy | 110.763 / 115.911 | 112.926 / 118.901 | unavailable | unavailable | 40.84 |
| training_backward_2x4x8x4_f64/candle | 14.834 / 15.272 | 14.675 / 14.920 | 472 | 8332 | 6.16 |
| training_backward_2x4x8x4_f64/kernix | 3.056 / 3.216 | 3.224 / 3.305 | 23 | 4224 | 4.63 |
| training_forward_2x4x8x4_f64/candle | 3.466 / 3.687 | 3.572 / 3.676 | 126 | 3912 | 5.94 |
| training_forward_2x4x8x4_f64/kernix | 2.224 / 2.269 | 2.217 / 2.354 | 17 | 1032 | 4.56 |
| training_record_2x4x8x4_f64/candle | 5.085 / 5.396 | 5.243 / 5.386 | 149 | 10712 | 6.31 |
| training_record_2x4x8x4_f64/kernix | 2.720 / 2.835 | 2.780 / 2.833 | 22 | 6832 | 4.53 |
| training_sgd_2x4x8x4_f64/kernix | 0.529 / 0.554 | 0.543 / 0.565 | 5 | 688 | 4.67 |

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
- Native engines may retain thread-local packing buffers. Warmed allocation probes exclude first-use allocations and retained scratch; first-use timing and process RSS are separate observations. See the measured library numerical contract for that version.
- Requested-byte peak is additional live Rust allocation during one call, including output; it is not scratch-only memory or peak RSS. RSS is the process high-water mark including fixtures, validation and runtime, read from Linux VmHWM. Other platforms may report unavailable.
- The published cases are allocation-inclusive CPU APIs and graph workloads. Pure internal kernel-only and end-to-end application/model latency are not measured. No workload result implies a complete application frame rate.
- faer is explicitly serial; Rayon and BLAS thread counts are one. No GPU comparison. Python/NumPy timings include Python dispatch. Backend and dependency versions/configuration, governor and RAM are recorded.
- Training stages use batched matmul + bias + layer_norm + tanh + step-two feature slice + MSE (Candle index_select). Forward excludes graph recording; record includes parameter/input/target snapshots and eager forward/loss; backward reuses a prepared graph and returns fresh derivatives. Allocation probes measure the corresponding stage, including graph values/scratch, not graph-container-only bytes. SGD uses fixed precomputed real gradients and includes transactional validation/parameter updates; it is not an end-to-end converging training loop. Candle normalization is an explicit population-statistics composition; its checks/rounding differ. Historical registry baselines without these Tape operations omit the stages, recorded in metadata, rather than substitute manual derivatives.
- A shared laptop has uncontrolled load/frequency/cache effects. Report comparator wins and losses; no universal ranking follows. The source archive comes from committed bytes, with result directories excluded. Library dependencies are pinned by the saved driver lockfile.
