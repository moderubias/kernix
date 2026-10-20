# Clean CPU evidence: 0.7.3-clean-bc459f4

Measured source `bc459f44b276e86275cc4e431e4946443aefb15f`, tree `8f1c1dad8523b125c5d444ddddcb3b84f34180f1`; dirty before/after: false.
Library source: workspace bc459f44b276e86275cc4e431e4946443aefb15f. One pinned CPU; dtype explicitly named per workload; row-major inputs.

| Workload / library | Per-call p50 / p95 (us) | Batch-mean p50 / p95 (us) | Rust allocations | Peak additional requested bytes | Process peak RSS (MiB) |
|---|---:|---:|---:|---:|---:|
| cholesky_factor_64_f64/faer | 8.459 / 11.350 | 8.334 / 8.598 | 2 | 33280 | 4.34 |
| cholesky_factor_64_f64/kernix | 25.132 / 29.908 | 23.824 / 26.077 | 2 | 65536 | 4.18 |
| cholesky_factor_64_f64/nalgebra | 14.198 / 23.259 | 14.605 / 14.922 | 1 | 32768 | 3.95 |
| cholesky_factor_64_f64/numpy | 10.865 / 11.527 | 11.075 / 12.230 | unavailable | unavailable | 32.83 |
| create_ones_64x64_f32/candle | 0.231 / 0.244 | 0.239 / 0.340 | 7 | 16696 | 4.70 |
| create_ones_64x64_f32/faer | 0.181 / 0.184 | 0.177 / 0.194 | 1 | 16384 | 4.09 |
| create_ones_64x64_f32/kernix | 0.185 / 0.190 | 0.200 / 0.207 | 1 | 16384 | 3.98 |
| create_ones_64x64_f32/ndarray | 0.144 / 0.145 | 0.151 / 0.182 | 1 | 16384 | 3.96 |
| create_ones_64x64_f32/numpy | 1.390 / 1.455 | 1.393 / 1.519 | unavailable | unavailable | 32.37 |
| create_zeros_64x64_f32/candle | 0.170 / 0.171 | 0.165 / 0.172 | 5 | 16648 | 4.43 |
| create_zeros_64x64_f32/faer | 0.155 / 0.165 | 0.193 / 0.203 | 1 | 16384 | 4.06 |
| create_zeros_64x64_f32/kernix | 0.143 / 0.146 | 0.145 / 0.167 | 1 | 16384 | 4.00 |
| create_zeros_64x64_f32/ndarray | 0.096 / 0.097 | 0.095 / 0.101 | 1 | 16384 | 3.89 |
| create_zeros_64x64_f32/numpy | 0.413 / 0.450 | 0.442 / 0.500 | unavailable | unavailable | 32.37 |
| matmul_128_f32/candle | 36.865 / 40.513 | 36.942 / 37.422 | 9 | 65816 | 5.30 |
| matmul_128_f32/faer | 40.606 / 42.481 | 41.702 / 43.818 | 1 | 65536 | 4.59 |
| matmul_128_f32/kernix | 34.749 / 43.578 | 36.847 / 37.495 | 3 | 65544 | 4.75 |
| matmul_128_f32/nalgebra | 49.771 / 60.873 | 50.001 / 53.177 | 2 | 163840 | 4.37 |
| matmul_128_f32/ndarray | 52.912 / 60.579 | 49.772 / 51.353 | 2 | 163840 | 4.18 |
| matmul_128_f32/numpy | 39.432 / 49.651 | 39.208 / 40.658 | unavailable | unavailable | 33.78 |
| matmul_128_f64/candle | 73.614 / 81.407 | 76.116 / 79.524 | 9 | 131352 | 5.56 |
| matmul_128_f64/faer | 82.494 / 86.900 | 83.670 / 85.564 | 1 | 131072 | 4.67 |
| matmul_128_f64/kernix | 73.624 / 80.074 | 76.029 / 77.215 | 3 | 131080 | 4.78 |
| matmul_128_f64/nalgebra | 115.569 / 142.450 | 118.061 / 136.201 | 2 | 327680 | 4.36 |
| matmul_128_f64/ndarray | 117.855 / 132.839 | 118.606 / 121.745 | 2 | 327680 | 4.31 |
| matmul_128_f64/numpy | 78.457 / 82.676 | 80.619 / 81.876 | unavailable | unavailable | 33.78 |
| matmul_256_f32/candle | 271.808 / 288.577 | 289.093 / 352.176 | 9 | 262424 | 6.04 |
| matmul_256_f32/faer | 310.024 / 323.077 | 312.615 / 319.833 | 1 | 262144 | 5.01 |
| matmul_256_f32/kernix | 270.684 / 301.835 | 278.458 / 285.456 | 3 | 262152 | 5.18 |
| matmul_256_f32/nalgebra | 386.854 / 425.066 | 405.541 / 431.505 | 2 | 589824 | 4.81 |
| matmul_256_f32/ndarray | 384.756 / 439.810 | 399.246 / 444.864 | 2 | 589824 | 4.88 |
| matmul_256_f32/numpy | 286.792 / 310.526 | 289.149 / 314.350 | unavailable | unavailable | 36.80 |
| matmul_256_f64/candle | 575.843 / 635.523 | 576.111 / 613.909 | 9 | 524568 | 7.16 |
| matmul_256_f64/faer | 722.614 / 751.776 | 724.448 / 741.527 | 1 | 524288 | 5.80 |
| matmul_256_f64/kernix | 574.036 / 603.405 | 580.756 / 652.008 | 3 | 524296 | 5.90 |
| matmul_256_f64/nalgebra | 662.924 / 704.866 | 664.984 / 688.683 | 2 | 1179648 | 5.53 |
| matmul_256_f64/ndarray | 665.964 / 744.432 | 666.414 / 687.929 | 2 | 1179648 | 6.01 |
| matmul_256_f64/numpy | 612.150 / 672.551 | 613.727 / 667.925 | unavailable | unavailable | 37.53 |
| matmul_32_f64/candle | 1.525 / 1.839 | 1.609 / 1.663 | 9 | 8592 | 5.09 |
| matmul_32_f64/faer | 1.600 / 1.868 | 1.581 / 1.792 | 1 | 8192 | 4.38 |
| matmul_32_f64/kernix | 1.535 / 1.918 | 1.669 / 1.712 | 3 | 24576 | 4.20 |
| matmul_32_f64/nalgebra | 1.612 / 1.777 | 1.531 / 1.607 | 2 | 24576 | 4.03 |
| matmul_32_f64/ndarray | 1.492 / 1.520 | 1.520 / 1.608 | 2 | 24576 | 4.06 |
| matmul_32_f64/numpy | 2.808 / 3.638 | 2.730 / 2.813 | unavailable | unavailable | 32.55 |
| matmul_64_f32/candle | 4.998 / 5.237 | 5.250 / 6.120 | 9 | 16664 | 5.11 |
| matmul_64_f32/faer | 5.676 / 6.469 | 5.521 / 5.683 | 1 | 16384 | 4.17 |
| matmul_64_f32/kernix | 6.441 / 8.550 | 4.936 / 5.323 | 3 | 16544 | 4.48 |
| matmul_64_f32/nalgebra | 5.791 / 5.876 | 5.855 / 6.122 | 2 | 49152 | 4.00 |
| matmul_64_f32/ndarray | 6.001 / 6.926 | 5.904 / 6.292 | 2 | 49152 | 4.04 |
| matmul_64_f32/numpy | 6.464 / 8.479 | 6.314 / 6.567 | unavailable | unavailable | 32.79 |
| matmul_64_f64/candle | 9.671 / 12.059 | 10.483 / 12.897 | 9 | 33048 | 5.27 |
| matmul_64_f64/faer | 10.701 / 11.584 | 11.870 / 12.036 | 1 | 32768 | 4.37 |
| matmul_64_f64/kernix | 9.162 / 11.080 | 10.448 / 10.931 | 3 | 32928 | 4.36 |
| matmul_64_f64/nalgebra | 11.075 / 14.950 | 11.298 / 11.677 | 2 | 98304 | 4.09 |
| matmul_64_f64/ndarray | 11.312 / 11.392 | 11.347 / 12.888 | 2 | 98304 | 4.15 |
| matmul_64_f64/numpy | 12.827 / 14.200 | 12.653 / 13.132 | unavailable | unavailable | 32.87 |
| matmul_batched_4x16x32x16_f64/candle | 1.995 / 2.232 | 2.208 / 2.898 | 25 | 25504 | 5.34 |
| matmul_batched_4x16x32x16_f64/kernix | 1.881 / 2.323 | 2.105 / 2.240 | 7 | 16392 | 4.45 |
| matmul_batched_4x16x32x16_f64/numpy | 3.391 / 4.634 | 3.201 / 3.388 | unavailable | unavailable | 32.54 |
| matmul_sum_backward_32x16x8_f64/candle | 4.273 / 5.261 | 4.267 / 4.759 | 113 | 27988 | 5.73 |
| matmul_sum_backward_32x16x8_f64/kernix | 2.378 / 2.802 | 3.946 / 4.638 | 16 | 18376 | 4.38 |
| qr_256x64_f64/faer | 423.772 / 465.185 | 408.792 / 435.627 | 7 | 331776 | 5.12 |
| qr_256x64_f64/kernix | 506.246 / 610.673 | 507.811 / 557.195 | 6 | 557568 | 4.88 |
| qr_256x64_f64/nalgebra | 368.816 / 393.340 | 378.203 / 570.741 | 4 | 295424 | 4.24 |
| qr_256x64_f64/numpy | 340.898 / 365.342 | 348.898 / 388.790 | unavailable | unavailable | 33.69 |
| qr_96x32_f64/faer | 70.197 / 74.963 | 67.703 / 72.590 | 7 | 67584 | 4.55 |
| qr_96x32_f64/kernix | 42.494 / 54.030 | 44.177 / 47.994 | 6 | 106752 | 4.14 |
| qr_96x32_f64/nalgebra | 36.530 / 38.232 | 37.775 / 40.850 | 4 | 57600 | 3.91 |
| qr_96x32_f64/numpy | 38.405 / 44.404 | 40.109 / 42.248 | unavailable | unavailable | 32.87 |
| softmax_256x64_f64/candle | 77.918 / 81.688 | 80.403 / 83.720 | 37 | 265200 | 5.69 |
| softmax_256x64_f64/kernix | 84.139 / 92.996 | 86.812 / 93.044 | 1 | 131072 | 4.33 |
| softmax_256x64_f64/ndarray | 68.781 / 80.569 | 70.443 / 74.978 | 1 | 131072 | 4.29 |
| softmax_256x64_f64/numpy | 102.145 / 111.305 | 101.446 / 104.477 | unavailable | unavailable | 32.79 |
| training_backward_2x4x8x4_f64/candle | 14.881 / 15.434 | 14.795 / 15.821 | 472 | 8332 | 6.20 |
| training_backward_2x4x8x4_f64/kernix | 3.160 / 3.308 | 3.363 / 3.484 | 23 | 4224 | 4.58 |
| training_forward_2x4x8x4_f64/candle | 3.582 / 3.793 | 3.716 / 4.222 | 126 | 3912 | 5.95 |
| training_forward_2x4x8x4_f64/kernix | 2.029 / 3.064 | 2.285 / 2.684 | 17 | 1032 | 4.46 |
| training_record_2x4x8x4_f64/candle | 5.252 / 7.515 | 5.409 / 5.944 | 149 | 10712 | 5.88 |
| training_record_2x4x8x4_f64/kernix | 2.699 / 2.773 | 2.828 / 2.938 | 22 | 6832 | 4.33 |
| training_sgd_2x4x8x4_f64/kernix | 0.528 / 0.539 | 0.549 / 0.574 | 5 | 688 | 4.46 |

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
