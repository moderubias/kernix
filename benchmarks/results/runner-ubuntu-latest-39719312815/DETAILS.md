# Clean CPU evidence: runner-ubuntu-latest-39719312815

Measured source `227f339974f9c31fcf6bd37ccccbda3eedc99591`, tree `556afc7a18c74320ec378c75bbefd4422f3d3ef3`; dirty before/after: false.
Library source: workspace 227f339974f9c31fcf6bd37ccccbda3eedc99591. One pinned CPU; dtype explicitly named per workload; row-major inputs.

| Workload / library | Per-call p50 / p95 (us) | Batch-mean p50 / p95 (us) | Rust allocations | Peak additional requested bytes | Process peak RSS (MiB) |
|---|---:|---:|---:|---:|---:|
| cholesky_factor_128_f64/faer | 58.871 / 68.779 | 59.157 / 59.764 | 2 | 132096 | 4.54 |
| cholesky_factor_128_f64/kernix | 169.068 / 181.011 | 172.758 / 204.010 | 1 | 131072 | 4.08 |
| cholesky_factor_128_f64/nalgebra | 120.737 / 132.489 | 121.808 / 128.626 | 1 | 131072 | 3.83 |
| cholesky_factor_128_f64/numpy | 107.282 / 121.809 | 109.294 / 112.286 | unavailable | unavailable | 44.17 |
| cholesky_factor_256_f64/faer | 316.065 / 336.143 | 317.461 / 322.137 | 2 | 526336 | 5.50 |
| cholesky_factor_256_f64/kernix | 1030.990 / 1059.975 | 1038.619 / 1047.416 | 1 | 524288 | 5.09 |
| cholesky_factor_256_f64/nalgebra | 820.354 / 832.767 | 820.375 / 833.000 | 1 | 524288 | 4.46 |
| cholesky_factor_256_f64/numpy | 1094.099 / 1176.464 | 1094.418 / 1158.616 | unavailable | unavailable | 48.18 |
| cholesky_factor_64_f64/faer | 14.206 / 14.277 | 14.487 / 14.579 | 2 | 33280 | 3.74 |
| cholesky_factor_64_f64/kernix | 25.808 / 26.430 | 26.066 / 26.333 | 2 | 33280 | 3.69 |
| cholesky_factor_64_f64/nalgebra | 20.228 / 20.709 | 20.377 / 20.657 | 1 | 32768 | 3.55 |
| cholesky_factor_64_f64/numpy | 18.174 / 19.156 | 18.442 / 24.819 | unavailable | unavailable | 43.47 |
| create_ones_64x64_f32/candle | 0.461 / 0.471 | 0.477 / 0.479 | 7 | 16696 | 4.12 |
| create_ones_64x64_f32/faer | 0.391 / 0.401 | 0.383 / 0.384 | 1 | 16384 | 3.55 |
| create_ones_64x64_f32/kernix | 0.431 / 0.611 | 0.438 / 0.442 | 1 | 16384 | 3.47 |
| create_ones_64x64_f32/ndarray | 0.351 / 0.361 | 0.360 / 0.394 | 1 | 16384 | 3.54 |
| create_ones_64x64_f32/numpy | 2.054 / 2.153 | 2.176 / 2.581 | unavailable | unavailable | 40.01 |
| create_zeros_64x64_f32/candle | 0.301 / 0.310 | 0.312 / 0.402 | 5 | 16648 | 4.03 |
| create_zeros_64x64_f32/faer | 0.300 / 0.301 | 0.272 / 0.274 | 1 | 16384 | 3.74 |
| create_zeros_64x64_f32/kernix | 0.311 / 0.321 | 0.324 / 0.326 | 1 | 16384 | 3.62 |
| create_zeros_64x64_f32/ndarray | 0.240 / 0.241 | 0.246 / 0.247 | 1 | 16384 | 3.52 |
| create_zeros_64x64_f32/numpy | 0.751 / 0.802 | 0.838 / 0.875 | unavailable | unavailable | 39.93 |
| matmul_128_f32/candle | 46.177 / 48.481 | 46.669 / 47.174 | 9 | 65816 | 4.80 |
| matmul_128_f32/faer | 52.289 / 61.024 | 52.490 / 56.289 | 1 | 65536 | 4.15 |
| matmul_128_f32/kernix | 46.287 / 55.514 | 46.793 / 47.077 | 3 | 65544 | 4.46 |
| matmul_128_f32/nalgebra | 74.280 / 90.190 | 77.293 / 78.311 | 2 | 163840 | 3.93 |
| matmul_128_f32/ndarray | 75.692 / 91.843 | 78.159 / 78.516 | 2 | 163840 | 3.95 |
| matmul_128_f32/numpy | 48.983 / 59.352 | 49.745 / 50.858 | unavailable | unavailable | 42.32 |
| matmul_128_f64/candle | 92.925 / 104.517 | 94.308 / 100.484 | 9 | 131352 | 4.95 |
| matmul_128_f64/faer | 100.529 / 112.101 | 101.069 / 102.760 | 1 | 131072 | 4.40 |
| matmul_128_f64/kernix | 93.687 / 103.735 | 94.809 / 95.945 | 3 | 131080 | 4.58 |
| matmul_128_f64/nalgebra | 210.025 / 228.209 | 211.140 / 212.965 | 2 | 327680 | 3.62 |
| matmul_128_f64/ndarray | 213.732 / 230.183 | 213.571 / 216.117 | 2 | 327680 | 3.96 |
| matmul_128_f64/numpy | 100.990 / 113.073 | 103.088 / 103.569 | unavailable | unavailable | 42.49 |
| matmul_256_f32/candle | 356.691 / 374.024 | 359.168 / 363.567 | 9 | 262424 | 5.34 |
| matmul_256_f32/faer | 417.535 / 436.712 | 421.996 / 428.947 | 1 | 262144 | 4.66 |
| matmul_256_f32/kernix | 352.744 / 369.926 | 357.156 / 359.999 | 3 | 262152 | 4.90 |
| matmul_256_f32/nalgebra | 617.082 / 628.603 | 616.426 / 618.981 | 2 | 589824 | 4.33 |
| matmul_256_f32/ndarray | 615.598 / 633.752 | 614.478 / 623.215 | 2 | 589824 | 4.70 |
| matmul_256_f32/numpy | 363.384 / 386.628 | 366.252 / 375.867 | unavailable | unavailable | 46.79 |
| matmul_256_f64/candle | 735.213 / 754.490 | 735.372 / 741.768 | 9 | 524568 | 6.38 |
| matmul_256_f64/faer | 963.663 / 988.850 | 967.136 / 976.969 | 1 | 524288 | 5.55 |
| matmul_256_f64/kernix | 731.015 / 743.910 | 731.411 / 734.312 | 3 | 524296 | 5.69 |
| matmul_256_f64/nalgebra | 901.967 / 915.934 | 903.297 / 906.168 | 2 | 1179648 | 5.32 |
| matmul_256_f64/ndarray | 910.683 / 933.116 | 910.974 / 947.225 | 2 | 1179648 | 5.89 |
| matmul_256_f64/numpy | 767.644 / 795.306 | 771.573 / 954.997 | unavailable | unavailable | 47.76 |
| matmul_32_f64/candle | 2.174 / 2.204 | 2.208 / 2.907 | 9 | 8472 | 4.52 |
| matmul_32_f64/faer | 2.034 / 2.055 | 2.038 / 2.058 | 1 | 8192 | 4.05 |
| matmul_32_f64/kernix | 2.525 / 2.565 | 2.548 / 2.596 | 3 | 24576 | 3.96 |
| matmul_32_f64/nalgebra | 2.104 / 2.154 | 2.142 / 2.482 | 2 | 24576 | 3.71 |
| matmul_32_f64/ndarray | 2.204 / 2.274 | 2.209 / 2.220 | 2 | 24576 | 3.73 |
| matmul_32_f64/numpy | 4.088 / 4.178 | 4.113 / 5.647 | unavailable | unavailable | 41.33 |
| matmul_64_f32/candle | 6.592 / 6.672 | 6.939 / 7.037 | 9 | 16664 | 4.75 |
| matmul_64_f32/faer | 6.523 / 6.602 | 6.598 / 6.604 | 1 | 16384 | 3.94 |
| matmul_64_f32/kernix | 6.542 / 6.623 | 6.608 / 6.682 | 3 | 16448 | 4.27 |
| matmul_64_f32/nalgebra | 7.403 / 7.475 | 7.495 / 7.826 | 2 | 49152 | 3.71 |
| matmul_64_f32/ndarray | 7.444 / 7.534 | 7.507 / 7.815 | 2 | 49152 | 3.86 |
| matmul_64_f32/numpy | 8.176 / 10.179 | 8.865 / 9.011 | unavailable | unavailable | 41.39 |
| matmul_64_f64/candle | 12.854 / 12.955 | 13.006 / 15.576 | 9 | 33048 | 4.62 |
| matmul_64_f64/faer | 13.555 / 13.906 | 13.953 / 15.022 | 1 | 32768 | 4.11 |
| matmul_64_f64/kernix | 12.744 / 12.854 | 12.904 / 12.981 | 3 | 32832 | 4.33 |
| matmul_64_f64/nalgebra | 14.968 / 15.079 | 15.097 / 15.146 | 2 | 98304 | 3.69 |
| matmul_64_f64/ndarray | 15.449 / 15.559 | 15.637 / 15.776 | 2 | 98304 | 3.78 |
| matmul_64_f64/numpy | 14.928 / 15.078 | 15.188 / 15.391 | unavailable | unavailable | 41.54 |
| matmul_batched_4x16x32x16_f64/candle | 3.286 / 3.356 | 3.356 / 3.383 | 25 | 25384 | 4.97 |
| matmul_batched_4x16x32x16_f64/kernix | 3.096 / 3.166 | 3.136 / 3.894 | 7 | 16392 | 4.00 |
| matmul_batched_4x16x32x16_f64/numpy | 4.198 / 4.268 | 4.364 / 4.438 | unavailable | unavailable | 41.29 |
| matmul_sum_backward_32x16x8_f64/candle | 7.564 / 8.165 | 7.519 / 7.684 | 113 | 27988 | 5.08 |
| matmul_sum_backward_32x16x8_f64/kernix | 3.977 / 4.068 | 3.998 / 4.065 | 16 | 18376 | 3.87 |
| qr_256x64_f64/faer | 422.545 / 444.717 | 425.010 / 428.478 | 7 | 331776 | 4.86 |
| qr_256x64_f64/kernix | 805.205 / 817.999 | 804.552 / 815.402 | 6 | 426496 | 4.67 |
| qr_256x64_f64/nalgebra | 567.719 / 584.290 | 569.720 / 651.750 | 4 | 295424 | 3.99 |
| qr_256x64_f64/numpy | 537.822 / 637.209 | 534.900 / 610.185 | unavailable | unavailable | 46.52 |
| qr_96x32_f64/faer | 54.122 / 63.729 | 54.698 / 55.696 | 7 | 67584 | 4.45 |
| qr_96x32_f64/kernix | 66.746 / 75.442 | 67.472 / 67.954 | 6 | 82176 | 4.07 |
| qr_96x32_f64/nalgebra | 57.739 / 65.984 | 58.306 / 58.646 | 4 | 57600 | 3.64 |
| qr_96x32_f64/numpy | 70.382 / 134.674 | 71.451 / 83.097 | unavailable | unavailable | 45.66 |
| softmax_256x64_f64/candle | 115.458 / 125.686 | 116.930 / 118.857 | 37 | 265200 | 5.27 |
| softmax_256x64_f64/kernix | 120.307 / 130.055 | 121.978 / 123.706 | 1 | 131072 | 4.13 |
| softmax_256x64_f64/ndarray | 110.859 / 120.537 | 114.801 / 115.365 | 1 | 131072 | 3.99 |
| softmax_256x64_f64/numpy | 131.027 / 145.053 | 135.632 / 137.705 | unavailable | unavailable | 40.53 |
| training_backward_2x4x8x4_f64/candle | 20.569 / 21.350 | 20.774 / 21.005 | 472 | 8332 | 5.45 |
| training_backward_2x4x8x4_f64/kernix | 4.980 / 5.160 | 5.186 / 6.819 | 23 | 4224 | 4.02 |
| training_forward_2x4x8x4_f64/candle | 4.940 / 5.190 | 4.994 / 5.044 | 126 | 3912 | 5.31 |
| training_forward_2x4x8x4_f64/kernix | 3.146 / 3.226 | 3.321 / 3.366 | 17 | 1032 | 3.67 |
| training_record_2x4x8x4_f64/candle | 11.381 / 12.324 | 6.243 / 7.126 | 149 | 10712 | 5.32 |
| training_record_2x4x8x4_f64/kernix | 4.088 / 4.168 | 4.169 / 4.197 | 22 | 6832 | 3.95 |
| training_sgd_2x4x8x4_f64/kernix | 0.831 / 0.851 | 0.852 / 0.864 | 5 | 688 | 4.02 |

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
