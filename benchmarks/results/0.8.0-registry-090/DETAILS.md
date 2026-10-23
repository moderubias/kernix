# Clean CPU evidence: 0.8.0-registry-090

Measured source `281af4b51aa0fe6f354770c2d4f3d18544586377`, tree `aa9c5efb0388fe82ae48455bbeb9b3d0de582268`; dirty before/after: false.
Library source: registry 0.8.0. One pinned CPU; dtype explicitly named per workload; row-major inputs.

Unavailable historical Kernix workloads: pivoted_qr_96x32_f64, pivoted_qr_256x64_f64, min_norm_64x32_rank16_f64, min_norm_32x64_rank32_f64. registry lacks the corresponding Tape or pivoted/minimum-norm APIs; no substitute algorithms used as historical Kernix.

| Workload / library | Per-call p50 / p95 (us) | Batch-mean p50 / p95 (us) | Rust allocations | Peak additional requested bytes | Process peak RSS (MiB) |
|---|---:|---:|---:|---:|---:|
| cholesky_factor_128_f64/faer | 39.205 / 44.944 | 39.469 / 46.137 | 2 | 132096 | 4.98 |
| cholesky_factor_128_f64/kernix | 110.993 / 128.299 | 114.978 / 122.976 | 1 | 131072 | 4.46 |
| cholesky_factor_128_f64/nalgebra | 94.382 / 106.013 | 96.598 / 116.974 | 1 | 131072 | 3.72 |
| cholesky_factor_128_f64/numpy | 71.079 / 76.547 | 72.164 / 73.795 | unavailable | unavailable | 36.95 |
| cholesky_factor_256_f64/faer | 194.519 / 217.262 | 196.025 / 204.205 | 2 | 526336 | 5.68 |
| cholesky_factor_256_f64/kernix | 614.523 / 677.045 | 645.648 / 716.808 | 1 | 524288 | 5.08 |
| cholesky_factor_256_f64/nalgebra | 642.426 / 682.793 | 661.353 / 794.950 | 1 | 524288 | 5.08 |
| cholesky_factor_256_f64/numpy | 512.960 / 630.834 | 531.913 / 573.565 | unavailable | unavailable | 40.89 |
| cholesky_factor_64_f64/faer | 8.489 / 9.520 | 8.407 / 9.745 | 2 | 33280 | 4.37 |
| cholesky_factor_64_f64/kernix | 21.199 / 28.994 | 21.117 / 21.953 | 2 | 33280 | 4.15 |
| cholesky_factor_64_f64/nalgebra | 14.344 / 21.506 | 14.889 / 15.636 | 1 | 32768 | 4.06 |
| cholesky_factor_64_f64/numpy | 10.184 / 12.348 | 10.888 / 12.481 | unavailable | unavailable | 36.18 |
| create_ones_64x64_f32/candle | 0.226 / 0.261 | 0.238 / 0.299 | 7 | 16696 | 4.75 |
| create_ones_64x64_f32/faer | 0.268 / 0.422 | 0.178 / 0.187 | 1 | 16384 | 4.38 |
| create_ones_64x64_f32/kernix | 0.203 / 0.206 | 0.196 / 0.205 | 1 | 16384 | 3.93 |
| create_ones_64x64_f32/ndarray | 0.140 / 0.142 | 0.146 / 0.170 | 1 | 16384 | 4.08 |
| create_ones_64x64_f32/numpy | 1.336 / 1.424 | 1.304 / 1.360 | unavailable | unavailable | 35.16 |
| create_zeros_64x64_f32/candle | 0.167 / 0.171 | 0.156 / 0.170 | 5 | 16648 | 4.49 |
| create_zeros_64x64_f32/faer | 0.156 / 0.167 | 0.190 / 0.194 | 1 | 16384 | 4.16 |
| create_zeros_64x64_f32/kernix | 0.136 / 0.138 | 0.140 / 0.145 | 1 | 16384 | 3.94 |
| create_zeros_64x64_f32/ndarray | 0.120 / 0.148 | 0.097 / 0.108 | 1 | 16384 | 4.06 |
| create_zeros_64x64_f32/numpy | 0.389 / 0.691 | 0.404 / 0.413 | unavailable | unavailable | 35.11 |
| matmul_128_f32/candle | 35.161 / 39.329 | 37.414 / 43.144 | 9 | 65816 | 5.29 |
| matmul_128_f32/faer | 40.147 / 42.897 | 40.454 / 42.257 | 1 | 65536 | 4.57 |
| matmul_128_f32/kernix | 34.962 / 39.757 | 37.157 / 44.387 | 3 | 65544 | 4.49 |
| matmul_128_f32/nalgebra | 47.523 / 54.573 | 50.476 / 53.510 | 2 | 163840 | 3.96 |
| matmul_128_f32/ndarray | 49.395 / 62.700 | 52.783 / 63.768 | 2 | 163840 | 4.31 |
| matmul_128_f32/numpy | 39.079 / 43.386 | 39.626 / 40.728 | unavailable | unavailable | 36.94 |
| matmul_128_f64/candle | 71.942 / 76.843 | 74.138 / 79.172 | 9 | 131352 | 5.44 |
| matmul_128_f64/faer | 83.957 / 92.742 | 84.781 / 87.721 | 1 | 131072 | 4.88 |
| matmul_128_f64/kernix | 72.804 / 87.523 | 74.066 / 89.493 | 3 | 131080 | 5.07 |
| matmul_128_f64/nalgebra | 113.800 / 135.072 | 119.446 / 126.971 | 2 | 327680 | 4.44 |
| matmul_128_f64/ndarray | 112.933 / 132.227 | 116.929 / 120.788 | 2 | 327680 | 4.49 |
| matmul_128_f64/numpy | 74.198 / 83.774 | 77.074 / 80.285 | unavailable | unavailable | 37.17 |
| matmul_256_f32/candle | 280.492 / 293.650 | 286.474 / 291.218 | 9 | 262424 | 6.08 |
| matmul_256_f32/faer | 327.257 / 347.867 | 334.305 / 409.147 | 1 | 262144 | 5.06 |
| matmul_256_f32/kernix | 273.282 / 289.791 | 280.783 / 411.628 | 3 | 262152 | 5.47 |
| matmul_256_f32/nalgebra | 394.214 / 442.376 | 411.445 / 674.669 | 2 | 589824 | 4.88 |
| matmul_256_f32/ndarray | 386.693 / 443.397 | 404.826 / 438.536 | 2 | 589824 | 5.10 |
| matmul_256_f32/numpy | 278.882 / 311.146 | 285.177 / 393.718 | unavailable | unavailable | 41.03 |
| matmul_256_f64/candle | 561.715 / 628.837 | 573.899 / 592.472 | 9 | 524568 | 6.82 |
| matmul_256_f64/faer | 747.131 / 800.262 | 774.516 / 898.999 | 1 | 524288 | 6.01 |
| matmul_256_f64/kernix | 550.760 / 645.864 | 567.746 / 610.621 | 3 | 524296 | 6.30 |
| matmul_256_f64/nalgebra | 662.855 / 822.371 | 671.988 / 710.912 | 2 | 1179648 | 5.68 |
| matmul_256_f64/ndarray | 661.684 / 821.148 | 685.995 / 747.860 | 2 | 1179648 | 6.20 |
| matmul_256_f64/numpy | 611.331 / 839.871 | 627.040 / 752.328 | unavailable | unavailable | 41.93 |
| matmul_32_f64/candle | 1.434 / 1.546 | 1.561 / 2.772 | 9 | 8592 | 5.10 |
| matmul_32_f64/faer | 1.780 / 1.884 | 1.708 / 1.766 | 1 | 8192 | 4.50 |
| matmul_32_f64/kernix | 1.529 / 1.653 | 1.708 / 1.790 | 3 | 24576 | 4.49 |
| matmul_32_f64/nalgebra | 1.384 / 1.402 | 1.450 / 1.526 | 2 | 24576 | 3.79 |
| matmul_32_f64/ndarray | 1.517 / 1.641 | 1.460 / 1.723 | 2 | 24576 | 4.13 |
| matmul_32_f64/numpy | 2.519 / 2.762 | 2.649 / 3.548 | unavailable | unavailable | 35.82 |
| matmul_64_f32/candle | 4.928 / 5.042 | 5.120 / 5.330 | 9 | 16664 | 5.05 |
| matmul_64_f32/faer | 5.643 / 6.303 | 5.541 / 6.581 | 1 | 16384 | 4.41 |
| matmul_64_f32/kernix | 4.712 / 4.911 | 4.952 / 5.647 | 3 | 16544 | 4.67 |
| matmul_64_f32/nalgebra | 5.660 / 6.099 | 5.974 / 6.282 | 2 | 49152 | 4.17 |
| matmul_64_f32/ndarray | 5.697 / 6.149 | 5.822 / 6.021 | 2 | 49152 | 4.18 |
| matmul_64_f32/numpy | 5.767 / 5.833 | 6.181 / 6.446 | unavailable | unavailable | 35.96 |
| matmul_64_f64/candle | 9.673 / 10.159 | 9.963 / 10.161 | 9 | 33048 | 5.09 |
| matmul_64_f64/faer | 10.484 / 11.408 | 10.897 / 15.280 | 1 | 32768 | 4.53 |
| matmul_64_f64/kernix | 9.853 / 10.774 | 9.859 / 10.028 | 3 | 32928 | 4.76 |
| matmul_64_f64/nalgebra | 10.516 / 11.345 | 11.170 / 11.862 | 2 | 98304 | 4.22 |
| matmul_64_f64/ndarray | 10.659 / 11.604 | 11.197 / 13.166 | 2 | 98304 | 4.20 |
| matmul_64_f64/numpy | 11.954 / 13.872 | 12.379 / 12.659 | unavailable | unavailable | 36.14 |
| matmul_batched_4x16x32x16_f64/candle | 2.153 / 2.223 | 2.196 / 2.507 | 25 | 25504 | 5.45 |
| matmul_batched_4x16x32x16_f64/kernix | 2.018 / 2.064 | 1.956 / 1.986 | 7 | 16392 | 4.29 |
| matmul_batched_4x16x32x16_f64/numpy | 2.945 / 3.016 | 2.994 / 3.071 | unavailable | unavailable | 35.93 |
| matmul_sum_backward_32x16x8_f64/candle | 3.958 / 4.121 | 4.144 / 4.405 | 113 | 27988 | 5.62 |
| matmul_sum_backward_32x16x8_f64/kernix | 2.592 / 2.672 | 2.672 / 5.068 | 16 | 18376 | 4.43 |
| qr_256x64_f64/faer | 399.014 / 437.021 | 404.667 / 428.118 | 7 | 331776 | 5.35 |
| qr_256x64_f64/kernix | 480.702 / 521.164 | 478.811 / 498.683 | 6 | 426496 | 4.72 |
| qr_256x64_f64/nalgebra | 368.639 / 386.032 | 368.927 / 379.283 | 4 | 295424 | 4.46 |
| qr_256x64_f64/numpy | 336.724 / 401.526 | 340.420 / 423.559 | unavailable | unavailable | 37.73 |
| qr_96x32_f64/faer | 68.343 / 73.685 | 69.406 / 71.401 | 7 | 67584 | 4.98 |
| qr_96x32_f64/kernix | 45.978 / 49.543 | 47.308 / 57.217 | 6 | 82176 | 4.41 |
| qr_96x32_f64/nalgebra | 36.882 / 57.341 | 38.133 / 46.397 | 4 | 57600 | 4.06 |
| qr_96x32_f64/numpy | 38.230 / 41.052 | 40.246 / 43.594 | unavailable | unavailable | 36.84 |
| softmax_256x64_f64/candle | 77.915 / 88.013 | 78.008 / 107.915 | 37 | 265200 | 5.88 |
| softmax_256x64_f64/kernix | 83.640 / 89.749 | 84.545 / 99.289 | 1 | 131072 | 4.70 |
| softmax_256x64_f64/ndarray | 68.899 / 111.796 | 70.635 / 79.258 | 1 | 131072 | 4.28 |
| softmax_256x64_f64/numpy | 102.018 / 109.292 | 103.139 / 121.434 | unavailable | unavailable | 35.66 |
| training_backward_2x4x8x4_f64/candle | 13.914 / 14.805 | 14.310 / 14.547 | 472 | 8332 | 5.91 |
| training_backward_2x4x8x4_f64/kernix | 3.082 / 3.277 | 3.190 / 3.714 | 23 | 4224 | 4.53 |
| training_forward_2x4x8x4_f64/candle | 3.391 / 3.564 | 3.492 / 3.743 | 126 | 3912 | 6.05 |
| training_forward_2x4x8x4_f64/kernix | 2.136 / 2.207 | 2.191 / 3.217 | 17 | 1032 | 4.27 |
| training_record_2x4x8x4_f64/candle | 5.017 / 5.254 | 5.039 / 5.149 | 149 | 10712 | 5.82 |
| training_record_2x4x8x4_f64/kernix | 2.582 / 2.643 | 2.682 / 2.839 | 22 | 6832 | 4.41 |
| training_sgd_2x4x8x4_f64/kernix | 0.498 / 0.518 | 0.514 / 0.525 | 5 | 688 | 4.44 |

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
