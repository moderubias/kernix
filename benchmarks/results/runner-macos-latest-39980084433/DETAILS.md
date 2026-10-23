# Clean CPU evidence: runner-macos-latest-39980084433

Measured source `9b7ef492c99bf2202e7eb3b07a79ef8f108991c7`, tree `3bfcaf4212c3091bc37e737a1d4d86760b373f94`; dirty before/after: false.
Library source: workspace 9b7ef492c99bf2202e7eb3b07a79ef8f108991c7. CPU affinity: not pinned. Dtype explicitly named per workload; row-major inputs.

| Workload / library | Per-call p50 / p95 (us) | Batch-mean p50 / p95 (us) | Rust allocations | Peak additional requested bytes | Process peak RSS (MiB) |
|---|---:|---:|---:|---:|---:|
| cholesky_factor_128_f64/faer | 49.500 / 166.125 | 47.750 / 66.240 | 16 | 149504 | unavailable |
| cholesky_factor_128_f64/kernix | 167.667 / 183.750 | 187.301 / 260.397 | 1 | 131072 | unavailable |
| cholesky_factor_128_f64/nalgebra | 104.000 / 147.041 | 106.887 / 126.109 | 1 | 131072 | unavailable |
| cholesky_factor_128_f64/numpy | 108.167 / 210.708 | 92.233 / 613.495 | unavailable | unavailable | unavailable |
| cholesky_factor_256_f64/faer | 250.708 / 272.958 | 264.920 / 271.941 | 76 | 592384 | unavailable |
| cholesky_factor_256_f64/kernix | 1322.125 / 3395.792 | 1498.381 / 3595.514 | 1 | 524288 | unavailable |
| cholesky_factor_256_f64/nalgebra | 730.125 / 805.250 | 756.000 / 1699.028 | 1 | 524288 | unavailable |
| cholesky_factor_256_f64/numpy | 330.125 / 401.458 | 327.579 / 371.034 | unavailable | unavailable | unavailable |
| cholesky_factor_64_f64/faer | 10.334 / 12.458 | 10.343 / 11.235 | 2 | 33280 | unavailable |
| cholesky_factor_64_f64/kernix | 22.292 / 29.792 | 23.217 / 26.002 | 2 | 33280 | unavailable |
| cholesky_factor_64_f64/nalgebra | 21.042 / 26.084 | 22.899 / 32.390 | 1 | 32768 | unavailable |
| cholesky_factor_64_f64/numpy | 15.083 / 17.875 | 15.427 / 18.578 | unavailable | unavailable | unavailable |
| create_ones_64x64_f32/candle | 0.417 / 0.500 | 0.408 / 1.032 | 7 | 16696 | unavailable |
| create_ones_64x64_f32/faer | 0.333 / 0.334 | 0.346 / 0.410 | 1 | 16384 | unavailable |
| create_ones_64x64_f32/kernix | 0.292 / 0.375 | 0.312 / 0.363 | 1 | 16384 | unavailable |
| create_ones_64x64_f32/ndarray | 0.333 / 0.334 | 0.269 / 0.300 | 1 | 16384 | unavailable |
| create_ones_64x64_f32/numpy | 1.125 / 1.208 | 1.300 / 1.564 | unavailable | unavailable | unavailable |
| create_zeros_64x64_f32/candle | 0.458 / 0.459 | 0.369 / 0.436 | 5 | 16648 | unavailable |
| create_zeros_64x64_f32/faer | 0.291 / 0.292 | 0.263 / 0.292 | 1 | 16384 | unavailable |
| create_zeros_64x64_f32/kernix | 0.750 / 0.792 | 0.363 / 0.513 | 1 | 16384 | unavailable |
| create_zeros_64x64_f32/ndarray | 0.291 / 0.292 | 0.274 / 0.361 | 1 | 16384 | unavailable |
| create_zeros_64x64_f32/numpy | 0.542 / 0.750 | 0.776 / 1.683 | unavailable | unavailable | unavailable |
| matmul_128_f32/candle | 52.916 / 62.792 | 54.160 / 59.342 | 10 | 327768 | unavailable |
| matmul_128_f32/faer | 55.500 / 58.208 | 56.822 / 88.659 | 3 | 393224 | unavailable |
| matmul_128_f32/kernix | 52.459 / 57.167 | 54.371 / 72.739 | 3 | 163840 | unavailable |
| matmul_128_f32/nalgebra | 51.208 / 58.500 | 54.427 / 66.752 | 2 | 163840 | unavailable |
| matmul_128_f32/ndarray | 47.834 / 51.125 | 52.746 / 71.475 | 2 | 163840 | unavailable |
| matmul_128_f32/numpy | 6.125 / 6.875 | 6.700 / 8.813 | unavailable | unavailable | unavailable |
| matmul_128_f64/candle | 97.750 / 111.250 | 107.117 / 124.599 | 10 | 655448 | unavailable |
| matmul_128_f64/faer | 108.500 / 318.959 | 111.915 / 223.223 | 3 | 786440 | unavailable |
| matmul_128_f64/kernix | 102.541 / 113.042 | 105.463 / 111.751 | 3 | 327680 | unavailable |
| matmul_128_f64/nalgebra | 103.667 / 127.709 | 125.461 / 211.355 | 2 | 327680 | unavailable |
| matmul_128_f64/ndarray | 109.792 / 218.750 | 113.377 / 141.197 | 2 | 327680 | unavailable |
| matmul_128_f64/numpy | 17.667 / 20.208 | 21.470 / 30.069 | unavailable | unavailable | unavailable |
| matmul_256_f32/candle | 408.125 / 500.541 | 415.543 / 727.134 | 10 | 786520 | unavailable |
| matmul_256_f32/faer | 458.125 / 652.125 | 785.042 / 956.902 | 3 | 1048584 | unavailable |
| matmul_256_f32/kernix | 372.500 / 424.334 | 410.683 / 455.067 | 3 | 589824 | unavailable |
| matmul_256_f32/nalgebra | 384.375 / 419.166 | 398.019 / 417.345 | 2 | 589824 | unavailable |
| matmul_256_f32/ndarray | 390.542 / 514.417 | 430.233 / 651.599 | 2 | 589824 | unavailable |
| matmul_256_f32/numpy | 39.916 / 144.959 | 36.326 / 115.132 | unavailable | unavailable | unavailable |
| matmul_256_f64/candle | 883.000 / 2436.166 | 935.045 / 1361.185 | 10 | 1572952 | unavailable |
| matmul_256_f64/faer | 822.750 / 925.125 | 848.121 / 1008.617 | 3 | 1572872 | unavailable |
| matmul_256_f64/kernix | 845.417 / 2354.375 | 1177.926 / 2282.958 | 3 | 1179648 | unavailable |
| matmul_256_f64/nalgebra | 1022.417 / 2404.209 | 1144.903 / 2389.867 | 2 | 1179648 | unavailable |
| matmul_256_f64/ndarray | 816.333 / 2280.459 | 1017.008 / 1905.785 | 2 | 1179648 | unavailable |
| matmul_256_f64/numpy | 120.834 / 537.208 | 170.076 / 447.176 | unavailable | unavailable | unavailable |
| matmul_32_f64/candle | 2.583 / 2.709 | 2.551 / 2.661 | 10 | 18512 | unavailable |
| matmul_32_f64/faer | 2.417 / 2.459 | 2.496 / 3.254 | 3 | 26624 | unavailable |
| matmul_32_f64/kernix | 2.250 / 2.542 | 2.305 / 2.408 | 3 | 24576 | unavailable |
| matmul_32_f64/nalgebra | 2.083 / 2.084 | 2.102 / 2.259 | 2 | 24576 | unavailable |
| matmul_32_f64/ndarray | 2.083 / 2.208 | 2.184 / 3.076 | 2 | 24576 | unavailable |
| matmul_32_f64/numpy | 1.541 / 1.792 | 1.693 / 2.038 | unavailable | unavailable | unavailable |
| matmul_64_f32/candle | 7.916 / 8.042 | 8.360 / 10.584 | 10 | 33872 | unavailable |
| matmul_64_f32/faer | 8.209 / 11.416 | 9.005 / 12.315 | 3 | 50176 | unavailable |
| matmul_64_f32/kernix | 10.500 / 52.500 | 8.742 / 22.290 | 3 | 49152 | unavailable |
| matmul_64_f32/nalgebra | 6.917 / 8.667 | 7.636 / 14.515 | 2 | 49152 | unavailable |
| matmul_64_f32/ndarray | 6.959 / 8.792 | 7.597 / 10.122 | 2 | 49152 | unavailable |
| matmul_64_f32/numpy | 1.916 / 2.083 | 1.905 / 2.133 | unavailable | unavailable | unavailable |
| matmul_64_f64/candle | 14.500 / 18.250 | 13.795 / 15.255 | 10 | 66640 | unavailable |
| matmul_64_f64/faer | 14.709 / 15.084 | 14.863 / 15.395 | 3 | 99328 | unavailable |
| matmul_64_f64/kernix | 12.959 / 14.000 | 13.168 / 14.786 | 3 | 98304 | unavailable |
| matmul_64_f64/nalgebra | 15.584 / 44.208 | 18.062 / 25.289 | 2 | 98304 | unavailable |
| matmul_64_f64/ndarray | 13.375 / 16.917 | 13.601 / 23.082 | 2 | 98304 | unavailable |
| matmul_64_f64/numpy | 3.500 / 3.708 | 3.660 / 3.822 | unavailable | unavailable | unavailable |
| matmul_batched_4x16x32x16_f64/candle | 3.583 / 3.625 | 3.512 / 3.722 | 25 | 27232 | unavailable |
| matmul_batched_4x16x32x16_f64/kernix | 2.875 / 3.084 | 3.009 / 3.147 | 7 | 16392 | unavailable |
| matmul_batched_4x16x32x16_f64/numpy | 1.791 / 2.125 | 2.061 / 2.394 | unavailable | unavailable | unavailable |
| matmul_sum_backward_32x16x8_f64/candle | 19.416 / 20.375 | 7.066 / 16.375 | 113 | 27980 | unavailable |
| matmul_sum_backward_32x16x8_f64/kernix | 8.042 / 8.541 | 5.632 / 10.593 | 16 | 18376 | unavailable |
| min_norm_32x64_rank32_f64/kernix | 95.917 / 226.542 | 92.720 / 152.907 | 28 | 102144 | unavailable |
| min_norm_32x64_rank32_f64/nalgebra | 152.958 / 166.083 | 156.192 / 161.128 | 15 | 42232 | unavailable |
| min_norm_32x64_rank32_f64/numpy | 136.125 / 159.250 | 159.075 / 275.063 | unavailable | unavailable | unavailable |
| min_norm_64x32_rank16_f64/kernix | 51.250 / 57.042 | 60.610 / 238.261 | 28 | 74752 | unavailable |
| min_norm_64x32_rank16_f64/nalgebra | 69.834 / 77.541 | 81.891 / 372.602 | 15 | 41976 | unavailable |
| min_norm_64x32_rank16_f64/numpy | 81.000 / 95.041 | 90.148 / 111.726 | unavailable | unavailable | unavailable |
| pivoted_qr_256x64_f64/faer | 309.791 / 346.042 | 437.822 / 846.465 | 58 | 336904 | unavailable |
| pivoted_qr_256x64_f64/kernix | 605.417 / 1319.458 | 914.701 / 1287.641 | 14 | 559104 | unavailable |
| pivoted_qr_256x64_f64/nalgebra | 1009.125 / 1170.250 | 1044.892 / 1112.662 | 7 | 296960 | unavailable |
| pivoted_qr_96x32_f64/faer | 40.167 / 46.083 | 42.622 / 47.147 | 32 | 94728 | unavailable |
| pivoted_qr_96x32_f64/kernix | 73.250 / 89.291 | 96.020 / 128.271 | 14 | 107520 | unavailable |
| pivoted_qr_96x32_f64/nalgebra | 96.375 / 106.000 | 100.821 / 109.734 | 7 | 58368 | unavailable |
| qr_256x64_f64/faer | 303.167 / 644.708 | 321.482 / 431.594 | 83 | 335880 | unavailable |
| qr_256x64_f64/kernix | 497.917 / 567.083 | 515.435 / 820.438 | 6 | 426496 | unavailable |
| qr_256x64_f64/nalgebra | 356.042 / 451.542 | 356.180 / 728.228 | 4 | 295424 | unavailable |
| qr_256x64_f64/numpy | 364.458 / 390.208 | 377.623 / 406.080 | unavailable | unavailable | unavailable |
| qr_96x32_f64/faer | 50.541 / 76.666 | 45.383 / 49.002 | 41 | 94216 | unavailable |
| qr_96x32_f64/kernix | 56.792 / 132.875 | 62.440 / 85.696 | 6 | 82176 | unavailable |
| qr_96x32_f64/nalgebra | 43.542 / 253.000 | 61.168 / 139.194 | 4 | 57600 | unavailable |
| qr_96x32_f64/numpy | 47.500 / 58.250 | 50.853 / 63.125 | unavailable | unavailable | unavailable |
| softmax_256x64_f64/candle | 65.000 / 70.083 | 69.118 / 72.368 | 37 | 265200 | unavailable |
| softmax_256x64_f64/kernix | 78.917 / 94.166 | 83.250 / 86.877 | 1 | 131072 | unavailable |
| softmax_256x64_f64/ndarray | 65.250 / 76.708 | 65.770 / 140.656 | 1 | 131072 | unavailable |
| softmax_256x64_f64/numpy | 76.667 / 94.458 | 80.806 / 144.447 | unavailable | unavailable | unavailable |
| training_backward_2x4x8x4_f64/candle | 20.959 / 23.875 | 19.711 / 28.842 | 476 | 21284 | unavailable |
| training_backward_2x4x8x4_f64/kernix | 3.750 / 11.209 | 3.303 / 4.163 | 23 | 4224 | unavailable |
| training_forward_2x4x8x4_f64/candle | 5.458 / 6.834 | 4.660 / 5.643 | 126 | 9568 | unavailable |
| training_forward_2x4x8x4_f64/kernix | 2.250 / 2.875 | 2.528 / 3.832 | 17 | 1032 | unavailable |
| training_record_2x4x8x4_f64/candle | 5.625 / 5.834 | 5.339 / 5.518 | 149 | 11544 | unavailable |
| training_record_2x4x8x4_f64/kernix | 2.875 / 2.917 | 2.762 / 7.468 | 22 | 6832 | unavailable |
| training_sgd_2x4x8x4_f64/kernix | 0.750 / 0.791 | 0.785 / 0.818 | 5 | 688 | unavailable |

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
