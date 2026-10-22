# Clean CPU evidence: runner-macos-latest-39720416210

Measured source `8b1a73ba926709c8c187a18aafe02840f0a4df50`, tree `3b35675a3f75f39d2bd737919bdda9a0b13a7a1e`; dirty before/after: false.
Library source: workspace 8b1a73ba926709c8c187a18aafe02840f0a4df50. One pinned CPU; dtype explicitly named per workload; row-major inputs.

| Workload / library | Per-call p50 / p95 (us) | Batch-mean p50 / p95 (us) | Rust allocations | Peak additional requested bytes | Process peak RSS (MiB) |
|---|---:|---:|---:|---:|---:|
| cholesky_factor_128_f64/faer | 47.167 / 53.000 | 48.920 / 54.692 | 16 | 149504 | unavailable |
| cholesky_factor_128_f64/kernix | 172.166 / 181.667 | 176.643 / 193.020 | 1 | 131072 | unavailable |
| cholesky_factor_128_f64/nalgebra | 108.250 / 126.166 | 121.739 / 179.070 | 1 | 131072 | unavailable |
| cholesky_factor_128_f64/numpy | 56.958 / 503.292 | 60.510 / 94.489 | unavailable | unavailable | unavailable |
| cholesky_factor_256_f64/faer | 275.708 / 324.541 | 284.279 / 334.300 | 76 | 592384 | unavailable |
| cholesky_factor_256_f64/kernix | 1195.750 / 1232.792 | 1183.398 / 1211.653 | 1 | 524288 | unavailable |
| cholesky_factor_256_f64/nalgebra | 772.584 / 895.458 | 764.997 / 877.705 | 1 | 524288 | unavailable |
| cholesky_factor_256_f64/numpy | 285.125 / 313.125 | 313.559 / 335.144 | unavailable | unavailable | unavailable |
| cholesky_factor_64_f64/faer | 10.792 / 10.958 | 10.877 / 11.479 | 2 | 33280 | unavailable |
| cholesky_factor_64_f64/kernix | 23.375 / 42.958 | 22.265 / 27.344 | 2 | 33280 | unavailable |
| cholesky_factor_64_f64/nalgebra | 21.042 / 21.541 | 21.497 / 23.708 | 1 | 32768 | unavailable |
| cholesky_factor_64_f64/numpy | 15.083 / 16.125 | 15.070 / 16.055 | unavailable | unavailable | unavailable |
| create_ones_64x64_f32/candle | 0.417 / 0.458 | 0.442 / 0.953 | 7 | 16696 | unavailable |
| create_ones_64x64_f32/faer | 0.333 / 0.334 | 0.339 / 0.377 | 1 | 16384 | unavailable |
| create_ones_64x64_f32/kernix | 0.292 / 0.541 | 0.335 / 0.387 | 1 | 16384 | unavailable |
| create_ones_64x64_f32/ndarray | 0.333 / 0.375 | 0.321 / 0.403 | 1 | 16384 | unavailable |
| create_ones_64x64_f32/numpy | 1.125 / 1.250 | 1.207 / 1.388 | unavailable | unavailable | unavailable |
| create_zeros_64x64_f32/candle | 0.375 / 0.459 | 0.365 / 0.383 | 5 | 16648 | unavailable |
| create_zeros_64x64_f32/faer | 0.291 / 0.292 | 0.320 / 0.428 | 1 | 16384 | unavailable |
| create_zeros_64x64_f32/kernix | 0.333 / 0.334 | 0.354 / 0.518 | 1 | 16384 | unavailable |
| create_zeros_64x64_f32/ndarray | 0.250 / 0.292 | 0.276 / 0.312 | 1 | 16384 | unavailable |
| create_zeros_64x64_f32/numpy | 0.542 / 0.959 | 0.835 / 1.694 | unavailable | unavailable | unavailable |
| matmul_128_f32/candle | 53.416 / 63.250 | 55.961 / 61.763 | 10 | 327768 | unavailable |
| matmul_128_f32/faer | 53.333 / 53.459 | 53.549 / 54.798 | 3 | 393224 | unavailable |
| matmul_128_f32/kernix | 53.917 / 135.458 | 62.003 / 91.237 | 3 | 163840 | unavailable |
| matmul_128_f32/nalgebra | 51.583 / 56.334 | 52.875 / 58.912 | 2 | 163840 | unavailable |
| matmul_128_f32/ndarray | 54.792 / 91.625 | 65.175 / 106.711 | 2 | 163840 | unavailable |
| matmul_128_f32/numpy | 6.083 / 6.542 | 6.149 / 7.930 | unavailable | unavailable | unavailable |
| matmul_128_f64/candle | 97.792 / 102.375 | 98.334 / 99.289 | 10 | 655448 | unavailable |
| matmul_128_f64/faer | 104.208 / 117.709 | 112.362 / 175.518 | 3 | 786440 | unavailable |
| matmul_128_f64/kernix | 103.166 / 123.625 | 108.511 / 118.376 | 3 | 327680 | unavailable |
| matmul_128_f64/nalgebra | 101.292 / 121.500 | 119.544 / 176.149 | 2 | 327680 | unavailable |
| matmul_128_f64/ndarray | 101.333 / 117.000 | 104.172 / 108.935 | 2 | 327680 | unavailable |
| matmul_128_f64/numpy | 16.125 / 16.333 | 16.227 / 16.278 | unavailable | unavailable | unavailable |
| matmul_256_f32/candle | 450.333 / 570.125 | 443.810 / 598.912 | 10 | 786520 | unavailable |
| matmul_256_f32/faer | 470.792 / 888.458 | 480.859 / 1320.073 | 3 | 1048584 | unavailable |
| matmul_256_f32/kernix | 417.625 / 654.583 | 409.915 / 578.644 | 3 | 589824 | unavailable |
| matmul_256_f32/nalgebra | 388.166 / 434.500 | 396.745 / 529.601 | 2 | 589824 | unavailable |
| matmul_256_f32/ndarray | 548.209 / 2499.167 | 1052.354 / 4733.281 | 2 | 589824 | unavailable |
| matmul_256_f32/numpy | 34.416 / 35.666 | 34.881 / 35.215 | unavailable | unavailable | unavailable |
| matmul_256_f64/candle | 847.667 / 961.459 | 865.819 / 1054.258 | 10 | 1572952 | unavailable |
| matmul_256_f64/faer | 915.084 / 1349.917 | 899.694 / 1153.213 | 3 | 1572872 | unavailable |
| matmul_256_f64/kernix | 818.166 / 2128.334 | 825.423 / 853.944 | 3 | 1179648 | unavailable |
| matmul_256_f64/nalgebra | 784.708 / 931.334 | 950.519 / 1402.620 | 2 | 1179648 | unavailable |
| matmul_256_f64/ndarray | 786.333 / 849.625 | 797.795 / 829.333 | 2 | 1179648 | unavailable |
| matmul_256_f64/numpy | 119.833 / 165.208 | 161.642 / 287.613 | unavailable | unavailable | unavailable |
| matmul_32_f64/candle | 2.542 / 2.542 | 2.493 / 2.580 | 10 | 18512 | unavailable |
| matmul_32_f64/faer | 2.417 / 3.000 | 2.454 / 2.543 | 3 | 26624 | unavailable |
| matmul_32_f64/kernix | 2.250 / 2.916 | 2.369 / 2.590 | 3 | 24576 | unavailable |
| matmul_32_f64/nalgebra | 2.083 / 2.583 | 2.106 / 2.160 | 2 | 24576 | unavailable |
| matmul_32_f64/ndarray | 2.041 / 2.666 | 2.142 / 2.528 | 2 | 24576 | unavailable |
| matmul_32_f64/numpy | 1.458 / 1.625 | 1.521 / 1.539 | unavailable | unavailable | unavailable |
| matmul_64_f32/candle | 7.833 / 8.041 | 8.440 / 13.496 | 10 | 33872 | unavailable |
| matmul_64_f32/faer | 8.208 / 10.459 | 8.690 / 9.507 | 3 | 50176 | unavailable |
| matmul_64_f32/kernix | 7.208 / 7.209 | 7.345 / 8.776 | 3 | 49152 | unavailable |
| matmul_64_f32/nalgebra | 7.084 / 14.042 | 8.219 / 13.775 | 2 | 49152 | unavailable |
| matmul_64_f32/ndarray | 6.917 / 7.125 | 7.014 / 7.212 | 2 | 49152 | unavailable |
| matmul_64_f32/numpy | 1.750 / 1.917 | 2.028 / 2.542 | unavailable | unavailable | unavailable |
| matmul_64_f64/candle | 14.500 / 14.917 | 14.649 / 14.873 | 10 | 66640 | unavailable |
| matmul_64_f64/faer | 14.750 / 15.125 | 14.947 / 15.057 | 3 | 99328 | unavailable |
| matmul_64_f64/kernix | 14.083 / 17.833 | 14.266 / 14.955 | 3 | 98304 | unavailable |
| matmul_64_f64/nalgebra | 13.583 / 21.708 | 14.059 / 15.084 | 2 | 98304 | unavailable |
| matmul_64_f64/ndarray | 13.459 / 16.958 | 17.560 / 36.056 | 2 | 98304 | unavailable |
| matmul_64_f64/numpy | 3.500 / 3.958 | 3.582 / 5.125 | unavailable | unavailable | unavailable |
| matmul_batched_4x16x32x16_f64/candle | 3.750 / 4.958 | 4.344 / 8.760 | 25 | 27232 | unavailable |
| matmul_batched_4x16x32x16_f64/kernix | 2.750 / 2.792 | 2.715 / 2.805 | 7 | 16392 | unavailable |
| matmul_batched_4x16x32x16_f64/numpy | 2.209 / 2.625 | 2.294 / 2.684 | unavailable | unavailable | unavailable |
| matmul_sum_backward_32x16x8_f64/candle | 6.750 / 7.167 | 6.343 / 6.677 | 113 | 27980 | unavailable |
| matmul_sum_backward_32x16x8_f64/kernix | 3.250 / 4.250 | 3.258 / 3.658 | 16 | 18376 | unavailable |
| qr_256x64_f64/faer | 302.000 / 321.792 | 308.076 / 314.357 | 83 | 335880 | unavailable |
| qr_256x64_f64/kernix | 560.541 / 626.208 | 594.250 / 747.978 | 6 | 426496 | unavailable |
| qr_256x64_f64/nalgebra | 354.500 / 369.583 | 359.260 / 374.358 | 4 | 295424 | unavailable |
| qr_256x64_f64/numpy | 411.792 / 588.250 | 412.907 / 462.328 | unavailable | unavailable | unavailable |
| qr_96x32_f64/faer | 46.000 / 55.667 | 47.513 / 50.853 | 41 | 94216 | unavailable |
| qr_96x32_f64/kernix | 64.708 / 82.584 | 68.680 / 88.301 | 6 | 82176 | unavailable |
| qr_96x32_f64/nalgebra | 42.125 / 54.625 | 45.847 / 61.249 | 4 | 57600 | unavailable |
| qr_96x32_f64/numpy | 53.666 / 70.792 | 65.121 / 567.480 | unavailable | unavailable | unavailable |
| softmax_256x64_f64/candle | 70.041 / 83.750 | 73.935 / 84.700 | 37 | 265200 | unavailable |
| softmax_256x64_f64/kernix | 78.583 / 93.333 | 84.263 / 102.203 | 1 | 131072 | unavailable |
| softmax_256x64_f64/ndarray | 61.667 / 75.667 | 62.795 / 65.988 | 1 | 131072 | unavailable |
| softmax_256x64_f64/numpy | 82.458 / 122.208 | 93.058 / 118.574 | unavailable | unavailable | unavailable |
| training_backward_2x4x8x4_f64/candle | 20.459 / 26.125 | 18.520 / 23.780 | 476 | 21284 | unavailable |
| training_backward_2x4x8x4_f64/kernix | 3.500 / 3.750 | 3.305 / 3.529 | 23 | 4224 | unavailable |
| training_forward_2x4x8x4_f64/candle | 5.709 / 7.208 | 4.716 / 5.851 | 126 | 9568 | unavailable |
| training_forward_2x4x8x4_f64/kernix | 2.250 / 2.416 | 2.159 / 2.282 | 17 | 1032 | unavailable |
| training_record_2x4x8x4_f64/candle | 6.208 / 6.750 | 5.402 / 6.240 | 149 | 11544 | unavailable |
| training_record_2x4x8x4_f64/kernix | 3.167 / 4.125 | 3.078 / 5.831 | 22 | 6832 | unavailable |
| training_sgd_2x4x8x4_f64/kernix | 0.709 / 0.792 | 0.730 / 0.736 | 5 | 688 | unavailable |

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
