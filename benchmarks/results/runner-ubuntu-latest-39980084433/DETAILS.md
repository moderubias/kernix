# Clean CPU evidence: runner-ubuntu-latest-39980084433

Measured source `9b7ef492c99bf2202e7eb3b07a79ef8f108991c7`, tree `3bfcaf4212c3091bc37e737a1d4d86760b373f94`; dirty before/after: false.
Library source: workspace 9b7ef492c99bf2202e7eb3b07a79ef8f108991c7. CPU affinity: 0. Dtype explicitly named per workload; row-major inputs.

| Workload / library | Per-call p50 / p95 (us) | Batch-mean p50 / p95 (us) | Rust allocations | Peak additional requested bytes | Process peak RSS (MiB) |
|---|---:|---:|---:|---:|---:|
| cholesky_factor_128_f64/faer | 58.740 / 69.379 | 59.284 / 60.054 | 2 | 132096 | 5.97 |
| cholesky_factor_128_f64/kernix | 169.285 / 182.109 | 172.657 / 174.532 | 1 | 131072 | 3.51 |
| cholesky_factor_128_f64/nalgebra | 120.364 / 129.702 | 121.657 / 122.358 | 1 | 131072 | 3.32 |
| cholesky_factor_128_f64/numpy | 107.680 / 120.665 | 109.262 / 111.368 | unavailable | unavailable | 40.14 |
| cholesky_factor_256_f64/faer | 314.836 / 331.487 | 319.664 / 327.329 | 2 | 526336 | 4.79 |
| cholesky_factor_256_f64/kernix | 1019.479 / 1046.459 | 1022.958 / 1031.505 | 1 | 524288 | 4.59 |
| cholesky_factor_256_f64/nalgebra | 821.720 / 844.272 | 824.098 / 830.428 | 1 | 524288 | 4.07 |
| cholesky_factor_256_f64/numpy | 1065.726 / 1090.603 | 1068.185 / 1077.454 | unavailable | unavailable | 43.15 |
| cholesky_factor_64_f64/faer | 14.337 / 14.427 | 14.482 / 14.652 | 2 | 33280 | 3.37 |
| cholesky_factor_64_f64/kernix | 25.357 / 25.668 | 25.640 / 26.225 | 2 | 33280 | 3.35 |
| cholesky_factor_64_f64/nalgebra | 20.238 / 20.619 | 20.495 / 21.222 | 1 | 32768 | 3.21 |
| cholesky_factor_64_f64/numpy | 18.334 / 20.708 | 18.638 / 18.794 | unavailable | unavailable | 39.48 |
| create_ones_64x64_f32/candle | 0.471 / 0.471 | 0.487 / 0.491 | 7 | 16696 | 3.62 |
| create_ones_64x64_f32/faer | 0.391 / 0.401 | 0.382 / 0.385 | 1 | 16384 | 3.37 |
| create_ones_64x64_f32/kernix | 0.431 / 0.431 | 0.442 / 0.445 | 1 | 16384 | 3.20 |
| create_ones_64x64_f32/ndarray | 0.351 / 0.361 | 0.359 / 0.360 | 1 | 16384 | 3.15 |
| create_ones_64x64_f32/numpy | 2.053 / 2.124 | 2.173 / 2.195 | unavailable | unavailable | 34.14 |
| create_zeros_64x64_f32/candle | 0.301 / 0.311 | 0.312 / 0.316 | 5 | 16648 | 3.46 |
| create_zeros_64x64_f32/faer | 0.311 / 0.311 | 0.273 / 0.276 | 1 | 16384 | 3.31 |
| create_zeros_64x64_f32/kernix | 0.311 / 0.321 | 0.323 / 0.325 | 1 | 16384 | 3.11 |
| create_zeros_64x64_f32/ndarray | 0.240 / 0.241 | 0.247 / 0.261 | 1 | 16384 | 3.20 |
| create_zeros_64x64_f32/numpy | 0.732 / 0.792 | 0.830 / 0.857 | unavailable | unavailable | 34.30 |
| matmul_128_f32/candle | 46.116 / 48.902 | 46.586 / 46.704 | 9 | 65816 | 4.04 |
| matmul_128_f32/faer | 50.735 / 60.143 | 51.063 / 51.296 | 1 | 65536 | 5.73 |
| matmul_128_f32/kernix | 46.076 / 55.133 | 46.634 / 46.957 | 3 | 65544 | 4.09 |
| matmul_128_f32/nalgebra | 73.637 / 90.048 | 77.119 / 77.886 | 2 | 163840 | 3.49 |
| matmul_128_f32/ndarray | 74.969 / 93.895 | 78.257 / 78.862 | 2 | 163840 | 3.51 |
| matmul_128_f32/numpy | 48.992 / 58.939 | 49.769 / 50.162 | unavailable | unavailable | 38.16 |
| matmul_128_f64/candle | 93.103 / 103.914 | 94.471 / 95.559 | 9 | 131352 | 4.54 |
| matmul_128_f64/faer | 100.487 / 114.544 | 101.562 / 102.055 | 1 | 131072 | 5.71 |
| matmul_128_f64/kernix | 92.974 / 104.695 | 94.128 / 94.700 | 3 | 131080 | 4.05 |
| matmul_128_f64/nalgebra | 200.213 / 216.634 | 201.655 / 203.812 | 2 | 327680 | 3.49 |
| matmul_128_f64/ndarray | 205.613 / 226.812 | 207.425 / 219.001 | 2 | 327680 | 3.65 |
| matmul_128_f64/numpy | 101.749 / 112.991 | 103.576 / 104.723 | unavailable | unavailable | 38.24 |
| matmul_256_f32/candle | 353.688 / 373.335 | 356.631 / 360.473 | 9 | 262424 | 4.90 |
| matmul_256_f32/faer | 420.944 / 437.786 | 425.305 / 429.363 | 1 | 262144 | 5.95 |
| matmul_256_f32/kernix | 352.907 / 365.551 | 357.009 / 363.212 | 3 | 262152 | 4.59 |
| matmul_256_f32/nalgebra | 597.472 / 614.524 | 598.829 / 605.823 | 2 | 589824 | 3.96 |
| matmul_256_f32/ndarray | 598.836 / 622.159 | 598.007 / 617.338 | 2 | 589824 | 4.09 |
| matmul_256_f32/numpy | 363.327 / 379.757 | 365.475 / 369.860 | unavailable | unavailable | 42.39 |
| matmul_256_f64/candle | 738.034 / 987.990 | 737.279 / 746.947 | 9 | 524568 | 5.78 |
| matmul_256_f64/faer | 981.868 / 1006.484 | 988.467 / 1004.540 | 1 | 524288 | 6.66 |
| matmul_256_f64/kernix | 734.047 / 749.246 | 734.356 / 754.344 | 3 | 524296 | 5.30 |
| matmul_256_f64/nalgebra | 903.072 / 918.992 | 902.086 / 905.781 | 2 | 1179648 | 4.78 |
| matmul_256_f64/ndarray | 909.224 / 927.828 | 909.698 / 911.746 | 2 | 1179648 | 5.33 |
| matmul_256_f64/numpy | 781.254 / 810.700 | 774.046 / 790.221 | unavailable | unavailable | 43.03 |
| matmul_32_f64/candle | 2.164 / 2.194 | 2.244 / 2.267 | 9 | 8472 | 3.89 |
| matmul_32_f64/faer | 2.024 / 2.044 | 2.018 / 2.042 | 1 | 8192 | 5.68 |
| matmul_32_f64/kernix | 2.524 / 2.545 | 2.550 / 2.568 | 3 | 24576 | 3.51 |
| matmul_32_f64/nalgebra | 2.134 / 2.154 | 2.157 / 2.170 | 2 | 24576 | 3.25 |
| matmul_32_f64/ndarray | 2.224 / 2.244 | 2.213 / 2.224 | 2 | 24576 | 3.18 |
| matmul_32_f64/numpy | 3.656 / 3.787 | 3.853 / 3.907 | unavailable | unavailable | 36.90 |
| matmul_64_f32/candle | 6.583 / 6.663 | 6.703 / 6.744 | 9 | 16664 | 3.88 |
| matmul_64_f32/faer | 6.612 / 6.673 | 6.706 / 6.766 | 1 | 16384 | 5.68 |
| matmul_64_f32/kernix | 6.543 / 6.623 | 6.629 / 6.683 | 3 | 16448 | 3.76 |
| matmul_64_f32/nalgebra | 7.453 / 7.534 | 7.516 / 7.539 | 2 | 49152 | 3.36 |
| matmul_64_f32/ndarray | 7.624 / 7.705 | 7.681 / 7.841 | 2 | 49152 | 3.33 |
| matmul_64_f32/numpy | 8.105 / 8.235 | 8.314 / 8.397 | unavailable | unavailable | 37.04 |
| matmul_64_f64/candle | 12.874 / 13.005 | 13.024 / 13.059 | 9 | 33048 | 4.03 |
| matmul_64_f64/faer | 13.455 / 19.887 | 13.625 / 13.667 | 1 | 32768 | 5.59 |
| matmul_64_f64/kernix | 12.734 / 12.834 | 12.900 / 13.016 | 3 | 32832 | 3.75 |
| matmul_64_f64/nalgebra | 15.058 / 15.168 | 15.213 / 15.347 | 2 | 98304 | 3.40 |
| matmul_64_f64/ndarray | 15.489 / 15.579 | 15.628 / 15.684 | 2 | 98304 | 3.47 |
| matmul_64_f64/numpy | 15.008 / 15.208 | 15.333 / 15.448 | unavailable | unavailable | 37.32 |
| matmul_batched_4x16x32x16_f64/candle | 3.246 / 3.306 | 3.271 / 3.298 | 25 | 25384 | 4.26 |
| matmul_batched_4x16x32x16_f64/kernix | 3.106 / 3.165 | 3.157 / 3.182 | 7 | 16392 | 3.55 |
| matmul_batched_4x16x32x16_f64/numpy | 4.308 / 4.368 | 4.461 / 4.510 | unavailable | unavailable | 37.09 |
| matmul_sum_backward_32x16x8_f64/candle | 7.514 / 7.815 | 7.650 / 7.694 | 113 | 27988 | 4.54 |
| matmul_sum_backward_32x16x8_f64/kernix | 3.977 / 4.058 | 3.961 / 4.051 | 16 | 18376 | 3.49 |
| min_norm_32x64_rank32_f64/kernix | 106.679 / 120.063 | 107.883 / 109.234 | 28 | 102144 | 3.59 |
| min_norm_32x64_rank32_f64/nalgebra | 177.190 / 187.329 | 179.146 / 181.119 | 15 | 42232 | 3.33 |
| min_norm_32x64_rank32_f64/numpy | 185.075 / 206.134 | 189.032 / 189.915 | unavailable | unavailable | 38.20 |
| min_norm_64x32_rank16_f64/kernix | 66.464 / 75.561 | 67.314 / 69.863 | 28 | 74752 | 3.74 |
| min_norm_64x32_rank16_f64/nalgebra | 103.623 / 112.129 | 104.749 / 109.092 | 15 | 41976 | 3.29 |
| min_norm_64x32_rank16_f64/numpy | 102.662 / 122.308 | 104.875 / 106.582 | unavailable | unavailable | 38.00 |
| pivoted_qr_256x64_f64/faer | 344.391 / 385.308 | 355.907 / 385.115 | 10 | 333312 | 6.21 |
| pivoted_qr_256x64_f64/kernix | 849.293 / 867.867 | 822.208 / 852.357 | 14 | 559104 | 4.02 |
| pivoted_qr_256x64_f64/nalgebra | 750.699 / 763.773 | 752.842 / 758.311 | 7 | 296960 | 3.68 |
| pivoted_qr_96x32_f64/faer | 43.151 / 43.731 | 43.642 / 45.500 | 10 | 68352 | 5.83 |
| pivoted_qr_96x32_f64/kernix | 78.446 / 90.699 | 79.986 / 80.487 | 14 | 107520 | 3.64 |
| pivoted_qr_96x32_f64/nalgebra | 77.104 / 85.940 | 78.079 / 78.717 | 7 | 58368 | 3.20 |
| qr_256x64_f64/faer | 416.997 / 450.369 | 421.141 / 425.574 | 7 | 331776 | 6.21 |
| qr_256x64_f64/kernix | 765.747 / 779.411 | 767.083 / 802.546 | 6 | 426496 | 4.09 |
| qr_256x64_f64/nalgebra | 557.258 / 572.796 | 558.375 / 561.846 | 4 | 295424 | 3.62 |
| qr_256x64_f64/numpy | 538.543 / 557.388 | 534.994 / 538.535 | unavailable | unavailable | 38.73 |
| qr_96x32_f64/faer | 53.840 / 63.248 | 54.494 / 55.206 | 7 | 67584 | 6.04 |
| qr_96x32_f64/kernix | 67.516 / 76.142 | 68.138 / 68.522 | 6 | 82176 | 3.49 |
| qr_96x32_f64/nalgebra | 57.958 / 66.334 | 58.546 / 58.967 | 4 | 57600 | 3.21 |
| qr_96x32_f64/numpy | 70.601 / 91.060 | 72.081 / 72.799 | unavailable | unavailable | 38.05 |
| softmax_256x64_f64/candle | 115.184 / 125.514 | 116.682 / 117.942 | 37 | 265200 | 4.89 |
| softmax_256x64_f64/kernix | 119.483 / 129.311 | 121.081 / 125.106 | 1 | 131072 | 3.62 |
| softmax_256x64_f64/ndarray | 108.622 / 119.383 | 109.733 / 110.257 | 1 | 131072 | 3.57 |
| softmax_256x64_f64/numpy | 132.517 / 145.631 | 135.362 / 137.615 | unavailable | unavailable | 34.96 |
| training_backward_2x4x8x4_f64/candle | 20.538 / 21.480 | 20.391 / 20.641 | 472 | 8332 | 4.91 |
| training_backward_2x4x8x4_f64/kernix | 5.069 / 5.269 | 5.111 / 5.248 | 23 | 4224 | 3.74 |
| training_forward_2x4x8x4_f64/candle | 5.089 / 5.370 | 5.166 / 5.289 | 126 | 3912 | 4.81 |
| training_forward_2x4x8x4_f64/kernix | 3.226 / 3.276 | 3.297 / 3.309 | 17 | 1032 | 3.72 |
| training_record_2x4x8x4_f64/candle | 6.272 / 6.632 | 6.378 / 6.499 | 149 | 10712 | 4.78 |
| training_record_2x4x8x4_f64/kernix | 4.148 / 4.218 | 4.274 / 4.335 | 22 | 6832 | 3.66 |
| training_sgd_2x4x8x4_f64/kernix | 0.831 / 0.832 | 0.833 / 0.855 | 5 | 688 | 3.82 |

## Method and limits

- Per-call quantiles use nearest ranks over 256 individual warmed calls; batch quantiles use 15 means over separate 10-ms batches. They are separate distributions. Raw samples remain in raw.json.
- Eight warmup calls follow validation. Timing includes allocation, dispatch, computation and output destruction. The first call field excludes output destruction. Inputs and library/runtime imports are prepared outside timing.
- Three extra processes record their first operation calls. These are first-use observations, not cache-flushed or device-startup latency. Timer-pair overhead is reported without subtraction; tiny calls are clock-sensitive.
- ndarray and Candle softmax are explicit stable compositions, not claims about a native softmax API.
- Constructor cases allocate owned 64x64 f32 tensors. Pre-0.6 registry baselines use full(value), explicitly marked in metadata; current workspace runs use zeros/ones. No nested literal reinterpretation is involved.
- All GEMM inputs are C-order and match elementwise. Native output layout differs by library. QR materializes thin Q and R in every implementation. Cholesky returns reusable factors (or NumPy L); no extra factor materialization is timed.
- Pivoted QR materializes thin Q/R and an owned permutation in Kernix, nalgebra and faer. Minimum-norm cases include factorization and four RHS: Kernix uses pivoted QR plus right QR with relative cutoff 1e-12, NumPy uses native lstsq with rcond=1e-12, and nalgebra uses native SVD plus solve with absolute cutoff 1e-10. These algorithms have different costs and truncation policies. Fixtures have a wide rank gap; independent null-space orthogonality and normal-residual checks must both pass 2e-10 before timing. Results do not establish equivalence near a rank threshold or general SVD performance.
- Kernix checks finite values/symmetry. Some competitors validate a narrower contract. No finite-data precondition check was removed to improve timing. Accuracy results and budgets are retained per worker.
- The backward case uses an actual reverse-mode graph in Kernix and Candle for sum(A @ W), with A constant and W trainable. Both copy input/weight data and returned gradients; no manual derivative is used as a framework ranking.
- Allocations are measured in a separately compiled instrumented binary after warmup, outside all timing loops. Counts cover Rust global-allocator requests, not native allocator internals, stack memory or all NumPy allocations. NumPy allocation counts are unavailable, not zero.
- Native engines may retain thread-local packing buffers. Warmed allocation probes exclude first-use allocations and retained scratch; first-use timing and process RSS are separate observations. See the measured library numerical contract for that version.
- Requested-byte peak is additional live Rust allocation during one call, including output; it is not scratch-only memory or peak RSS. RSS is the process high-water mark including fixtures, validation and runtime, read from Linux VmHWM. Other platforms may report unavailable.
- The published cases are allocation-inclusive CPU APIs and graph workloads. Pure internal kernel-only and end-to-end application/model latency are not measured. No workload result implies a complete application frame rate.
- faer is explicitly serial; Rayon and BLAS thread counts are one. No GPU comparison. Python/NumPy timings include Python dispatch. Backend and dependency versions/configuration, governor and RAM are recorded.
- Training stages use batched matmul + bias + layer_norm + tanh + step-two feature slice + MSE (Candle index_select). Forward excludes graph recording; record includes parameter/input/target snapshots and eager forward/loss; backward reuses a prepared graph and returns fresh derivatives. Allocation probes measure the corresponding stage, including graph values/scratch, not graph-container-only bytes. SGD uses fixed precomputed real gradients and includes transactional validation/parameter updates; it is not an end-to-end converging training loop. Candle normalization is an explicit population-statistics composition; its checks/rounding differ. Historical registry baselines without these Tape operations omit the stages, recorded in metadata, rather than substitute manual derivatives.
- Host load/frequency/cache effects are uncontrolled. Report comparator wins and losses; no universal ranking follows. The source archive comes from committed bytes, with result directories excluded. Library dependencies are pinned by the saved driver lockfile.
