# Clean CPU evidence: runner-ubuntu-latest-40059566395

Measured source `18750693908110110d85d02d0f11af8d58bfae8b`, tree `75403e48db8556f95e014ec2c5b16b30f9fa781e`; dirty before/after: false.
Library source: workspace 18750693908110110d85d02d0f11af8d58bfae8b. CPU affinity: 0. Dtype explicitly named per workload; row-major inputs.

| Workload / library | Per-call p50 / p95 (us) | Batch-mean p50 / p95 (us) | Rust allocations | Peak additional requested bytes | Process peak RSS (MiB) |
|---|---:|---:|---:|---:|---:|
| cholesky_factor_128_f64/faer | 57.718 / 67.326 | 58.589 / 63.042 | 2 | 132096 | 5.89 |
| cholesky_factor_128_f64/kernix | 169.948 / 180.317 | 171.992 / 173.192 | 1 | 131072 | 3.51 |
| cholesky_factor_128_f64/nalgebra | 119.193 / 129.593 | 124.959 / 126.228 | 1 | 131072 | 3.43 |
| cholesky_factor_128_f64/numpy | 108.113 / 122.450 | 109.765 / 111.196 | unavailable | unavailable | 40.34 |
| cholesky_factor_256_f64/faer | 313.717 / 328.725 | 317.749 / 319.950 | 2 | 526336 | 6.72 |
| cholesky_factor_256_f64/kernix | 686.484 / 699.590 | 688.131 / 698.998 | 49 | 645120 | 5.08 |
| cholesky_factor_256_f64/nalgebra | 788.336 / 803.194 | 817.975 / 826.110 | 1 | 524288 | 4.07 |
| cholesky_factor_256_f64/numpy | 1070.464 / 1103.796 | 1070.740 / 1083.170 | unavailable | unavailable | 43.44 |
| cholesky_factor_512_f64/faer | 2584.958 / 2649.059 | 2581.394 / 2633.526 | 2 | 2101248 | 9.62 |
| cholesky_factor_512_f64/kernix | 3886.384 / 5599.900 | 3881.935 / 5393.124 | 161 | 2269184 | 9.63 |
| cholesky_factor_512_f64/nalgebra | 7356.338 / 7524.292 | 7523.786 / 7582.942 | 1 | 2097152 | 7.13 |
| cholesky_factor_512_f64/numpy | 6194.474 / 6245.405 | 6213.515 / 6301.525 | unavailable | unavailable | 54.02 |
| cholesky_factor_64_f64/faer | 14.097 / 14.177 | 14.275 / 17.766 | 2 | 33280 | 3.55 |
| cholesky_factor_64_f64/kernix | 24.777 / 25.297 | 24.958 / 25.745 | 2 | 33280 | 3.24 |
| cholesky_factor_64_f64/nalgebra | 20.910 / 21.150 | 21.601 / 21.719 | 1 | 32768 | 3.18 |
| cholesky_factor_64_f64/numpy | 18.174 / 19.156 | 18.878 / 19.051 | unavailable | unavailable | 39.75 |
| create_ones_64x64_f32/candle | 0.471 / 0.472 | 0.477 / 0.491 | 7 | 16696 | 3.73 |
| create_ones_64x64_f32/faer | 0.391 / 0.401 | 0.383 / 0.385 | 1 | 16384 | 3.27 |
| create_ones_64x64_f32/kernix | 0.431 / 0.431 | 0.441 / 0.445 | 1 | 16384 | 3.26 |
| create_ones_64x64_f32/ndarray | 0.351 / 0.361 | 0.360 / 0.362 | 1 | 16384 | 3.21 |
| create_ones_64x64_f32/numpy | 2.094 / 2.154 | 2.212 / 2.251 | unavailable | unavailable | 34.48 |
| create_zeros_64x64_f32/candle | 0.301 / 0.330 | 0.315 / 0.321 | 5 | 16648 | 3.48 |
| create_zeros_64x64_f32/faer | 0.311 / 0.311 | 0.273 / 0.316 | 1 | 16384 | 3.39 |
| create_zeros_64x64_f32/kernix | 0.320 / 0.321 | 0.338 / 0.345 | 1 | 16384 | 3.16 |
| create_zeros_64x64_f32/ndarray | 0.250 / 0.251 | 0.249 / 0.249 | 1 | 16384 | 3.12 |
| create_zeros_64x64_f32/numpy | 0.742 / 0.782 | 0.839 / 0.846 | unavailable | unavailable | 34.61 |
| matmul_128_f32/candle | 46.186 / 50.394 | 46.867 / 47.184 | 9 | 65816 | 4.23 |
| matmul_128_f32/faer | 51.556 / 59.261 | 52.137 / 52.468 | 1 | 65536 | 5.84 |
| matmul_128_f32/kernix | 45.836 / 55.064 | 46.396 / 47.400 | 3 | 65544 | 4.04 |
| matmul_128_f32/nalgebra | 73.688 / 102.652 | 77.267 / 83.514 | 2 | 163840 | 3.48 |
| matmul_128_f32/ndarray | 74.700 / 90.890 | 78.095 / 78.442 | 2 | 163840 | 3.53 |
| matmul_128_f32/numpy | 49.011 / 59.161 | 49.718 / 50.483 | unavailable | unavailable | 38.27 |
| matmul_128_f64/candle | 92.934 / 103.514 | 94.060 / 94.657 | 9 | 131352 | 4.46 |
| matmul_128_f64/faer | 100.428 / 111.539 | 102.116 / 110.608 | 1 | 131072 | 5.87 |
| matmul_128_f64/kernix | 93.986 / 104.585 | 95.483 / 100.032 | 3 | 131080 | 4.23 |
| matmul_128_f64/nalgebra | 204.954 / 229.609 | 206.588 / 208.483 | 2 | 327680 | 3.67 |
| matmul_128_f64/ndarray | 204.002 / 221.635 | 204.665 / 206.245 | 2 | 327680 | 3.59 |
| matmul_128_f64/numpy | 102.031 / 113.051 | 103.611 / 104.732 | unavailable | unavailable | 38.48 |
| matmul_256_f32/candle | 353.150 / 384.540 | 356.831 / 392.568 | 9 | 262424 | 4.89 |
| matmul_256_f32/faer | 418.683 / 435.676 | 422.605 / 425.118 | 1 | 262144 | 6.03 |
| matmul_256_f32/kernix | 352.229 / 363.311 | 356.351 / 358.274 | 3 | 262152 | 4.54 |
| matmul_256_f32/nalgebra | 600.013 / 614.120 | 598.206 / 601.930 | 2 | 589824 | 3.93 |
| matmul_256_f32/ndarray | 600.354 / 616.654 | 599.024 / 602.696 | 2 | 589824 | 4.21 |
| matmul_256_f32/numpy | 361.626 / 376.816 | 365.389 / 371.294 | unavailable | unavailable | 41.70 |
| matmul_256_f64/candle | 738.492 / 774.138 | 737.909 / 819.940 | 9 | 524568 | 5.83 |
| matmul_256_f64/faer | 969.514 / 989.382 | 969.348 / 972.202 | 1 | 524288 | 6.44 |
| matmul_256_f64/kernix | 730.869 / 746.278 | 730.196 / 738.122 | 3 | 524296 | 5.34 |
| matmul_256_f64/nalgebra | 904.363 / 916.075 | 905.976 / 907.790 | 2 | 1179648 | 4.77 |
| matmul_256_f64/ndarray | 910.935 / 933.667 | 911.797 / 929.027 | 2 | 1179648 | 5.26 |
| matmul_256_f64/numpy | 766.815 / 789.948 | 766.871 / 772.815 | unavailable | unavailable | 42.14 |
| matmul_32_f64/candle | 2.214 / 2.245 | 2.250 / 2.282 | 9 | 8472 | 4.02 |
| matmul_32_f64/faer | 2.044 / 2.074 | 2.030 / 2.063 | 1 | 8192 | 5.36 |
| matmul_32_f64/kernix | 2.524 / 2.555 | 2.554 / 2.566 | 3 | 24576 | 3.57 |
| matmul_32_f64/nalgebra | 2.124 / 2.144 | 2.152 / 2.171 | 2 | 24576 | 3.24 |
| matmul_32_f64/ndarray | 2.224 / 2.255 | 2.228 / 2.243 | 2 | 24576 | 3.24 |
| matmul_32_f64/numpy | 3.677 / 3.747 | 3.812 / 3.838 | unavailable | unavailable | 37.30 |
| matmul_64_f32/candle | 6.583 / 6.653 | 6.683 / 6.766 | 9 | 16664 | 4.05 |
| matmul_64_f32/faer | 6.512 / 6.592 | 6.563 / 6.600 | 1 | 16384 | 5.51 |
| matmul_64_f32/kernix | 6.612 / 6.692 | 6.688 / 7.651 | 3 | 16448 | 3.80 |
| matmul_64_f32/nalgebra | 7.354 / 7.434 | 7.427 / 7.445 | 2 | 49152 | 3.25 |
| matmul_64_f32/ndarray | 7.454 / 9.588 | 7.524 / 7.586 | 2 | 49152 | 3.34 |
| matmul_64_f32/numpy | 8.105 / 8.216 | 8.310 / 8.375 | unavailable | unavailable | 37.39 |
| matmul_64_f64/candle | 12.844 / 12.944 | 13.067 / 13.994 | 9 | 33048 | 4.12 |
| matmul_64_f64/faer | 13.455 / 13.735 | 13.764 / 13.945 | 1 | 32768 | 5.55 |
| matmul_64_f64/kernix | 12.724 / 12.854 | 12.923 / 14.844 | 3 | 32832 | 4.11 |
| matmul_64_f64/nalgebra | 14.958 / 15.048 | 15.130 / 16.451 | 2 | 98304 | 3.33 |
| matmul_64_f64/ndarray | 15.509 / 15.609 | 15.657 / 15.722 | 2 | 98304 | 3.24 |
| matmul_64_f64/numpy | 15.148 / 15.339 | 15.487 / 17.801 | unavailable | unavailable | 37.57 |
| matmul_ab_128_f32/kernix | 45.996 / 48.581 | 46.545 / 46.826 | 3 | 65544 | 4.05 |
| matmul_ab_128_f32/numpy | 49.803 / 59.882 | 50.585 / 51.114 | unavailable | unavailable | 38.12 |
| matmul_ab_128_f64/kernix | 94.016 / 104.685 | 95.192 / 95.908 | 3 | 131080 | 4.23 |
| matmul_ab_128_f64/numpy | 102.652 / 113.442 | 103.446 / 105.467 | unavailable | unavailable | 38.36 |
| matmul_batched_4x16x32x16_f64/candle | 3.247 / 6.101 | 3.262 / 3.301 | 25 | 25384 | 4.19 |
| matmul_batched_4x16x32x16_f64/kernix | 3.126 / 3.176 | 3.182 / 3.201 | 7 | 16392 | 3.45 |
| matmul_batched_4x16x32x16_f64/numpy | 4.188 / 4.278 | 4.349 / 4.985 | unavailable | unavailable | 37.18 |
| matmul_sum_backward_32x16x8_f64/candle | 7.725 / 8.075 | 7.654 / 7.826 | 113 | 27988 | 4.50 |
| matmul_sum_backward_32x16x8_f64/kernix | 4.067 / 4.138 | 4.100 / 4.129 | 16 | 18376 | 3.65 |
| min_norm_32x64_rank32_f64/kernix | 84.257 / 93.304 | 85.058 / 85.725 | 20 | 59136 | 3.78 |
| min_norm_32x64_rank32_f64/nalgebra | 178.364 / 188.443 | 180.718 / 182.038 | 15 | 42232 | 3.51 |
| min_norm_32x64_rank32_f64/numpy | 188.583 / 215.193 | 192.473 / 228.492 | unavailable | unavailable | 38.50 |
| min_norm_64x32_rank16_f64/kernix | 62.256 / 71.264 | 62.975 / 63.314 | 20 | 74752 | 3.94 |
| min_norm_64x32_rank16_f64/nalgebra | 102.482 / 112.070 | 103.979 / 104.353 | 15 | 41976 | 3.53 |
| min_norm_64x32_rank16_f64/numpy | 103.403 / 123.251 | 105.716 / 107.102 | unavailable | unavailable | 38.38 |
| parallel_matmul_into_128_f32/kernix | 34.084 / 37.209 | 34.416 / 34.912 | 2 | 16 | 4.41 |
| parallel_matmul_into_128_f64/kernix | 56.566 / 66.795 | 59.781 / 60.349 | 2 | 16 | 4.49 |
| parallel_matmul_into_256_f64/kernix | 393.688 / 409.527 | 399.180 / 406.104 | 2 | 16 | 6.18 |
| parallel_matmul_into_64_f64/kernix | 18.966 / 19.216 | 19.161 / 21.209 | 2 | 98304 | 3.79 |
| pivoted_qr_256x64_f64/faer | 343.322 / 378.659 | 352.160 / 380.334 | 10 | 333312 | 6.16 |
| pivoted_qr_256x64_f64/kernix | 851.052 / 879.506 | 818.781 / 869.201 | 14 | 559104 | 4.18 |
| pivoted_qr_256x64_f64/nalgebra | 767.306 / 785.490 | 766.552 / 774.075 | 7 | 296960 | 3.80 |
| pivoted_qr_96x32_f64/faer | 42.480 / 47.138 | 42.995 / 49.818 | 10 | 68352 | 6.06 |
| pivoted_qr_96x32_f64/kernix | 78.687 / 90.550 | 80.184 / 82.146 | 14 | 107520 | 3.72 |
| pivoted_qr_96x32_f64/nalgebra | 78.216 / 87.163 | 78.956 / 80.434 | 7 | 58368 | 3.56 |
| qr_256x64_f64/faer | 418.203 / 442.689 | 421.534 / 423.783 | 7 | 331776 | 6.38 |
| qr_256x64_f64/kernix | 780.099 / 850.101 | 783.666 / 801.402 | 6 | 426496 | 4.09 |
| qr_256x64_f64/nalgebra | 570.307 / 592.299 | 574.772 / 717.521 | 4 | 295424 | 3.58 |
| qr_256x64_f64/numpy | 540.582 / 675.013 | 535.606 / 603.707 | unavailable | unavailable | 39.00 |
| qr_96x32_f64/faer | 54.262 / 63.750 | 54.878 / 71.741 | 7 | 67584 | 5.88 |
| qr_96x32_f64/kernix | 67.256 / 76.493 | 68.518 / 69.006 | 6 | 82176 | 3.59 |
| qr_96x32_f64/nalgebra | 58.970 / 67.116 | 59.533 / 60.435 | 4 | 57600 | 3.52 |
| qr_96x32_f64/numpy | 70.692 / 91.281 | 72.147 / 87.290 | unavailable | unavailable | 38.47 |
| softmax_256x64_f64/candle | 115.145 / 128.460 | 116.734 / 117.948 | 37 | 265200 | 4.80 |
| softmax_256x64_f64/kernix | 119.533 / 128.480 | 120.937 / 147.407 | 1 | 131072 | 3.70 |
| softmax_256x64_f64/ndarray | 110.877 / 120.796 | 114.976 / 117.304 | 1 | 131072 | 3.63 |
| softmax_256x64_f64/numpy | 132.998 / 156.804 | 132.248 / 164.164 | unavailable | unavailable | 35.23 |
| tensordot_16x32x8_32x8x16_f64/kernix | 63.067 / 71.774 | 63.686 / 74.106 | 9 | 2166 | 3.29 |
| tensordot_16x32x8_32x8x16_f64/numpy | 11.331 / 11.772 | 11.606 / 11.774 | unavailable | unavailable | 37.03 |
| training_backward_2x4x8x4_f64/candle | 20.689 / 21.369 | 20.552 / 20.856 | 472 | 8332 | 5.02 |
| training_backward_2x4x8x4_f64/kernix | 5.099 / 5.280 | 5.215 / 5.306 | 23 | 4224 | 3.79 |
| training_forward_2x4x8x4_f64/candle | 5.030 / 5.260 | 5.072 / 5.129 | 126 | 3912 | 4.94 |
| training_forward_2x4x8x4_f64/kernix | 3.226 / 3.286 | 3.272 / 3.301 | 17 | 1032 | 3.78 |
| training_record_2x4x8x4_f64/candle | 6.201 / 6.522 | 6.276 / 6.310 | 149 | 10712 | 4.70 |
| training_record_2x4x8x4_f64/kernix | 4.098 / 4.188 | 4.202 / 4.237 | 22 | 6832 | 3.66 |
| training_sgd_2x4x8x4_f64/kernix | 0.802 / 0.812 | 0.817 / 0.824 | 5 | 688 | 3.97 |
| training_shared_backward_64x32_f64/candle | 4.939 / 5.050 | 4.954 / 5.023 | 58 | 100004 | 4.05 |
| training_shared_backward_64x32_f64/kernix | 10.410 / 10.470 | 10.412 / 10.545 | 8 | 49768 | 3.55 |

## Method and limits

- Rows explicitly marked two workers use a private Kernix executor with two workers and reusable output allocated before the first call. RefCell borrowing is included; pool construction is excluded. These rows compare Kernix versions, with no single-thread peer score. Linux uses up to two allowed CPUs, preferring distinct reported physical cores; Windows/macOS affinity is not pinned. Exact per-workload CPU sets are in explicit_execution metadata. Ordinary cases retain one BLAS/Rayon thread and the original single-CPU Linux policy.
- The optional Kernix Accelerate variant is measured only on macOS 15+ when compiled with the explicit feature. SDK calls request one BLAS thread and restore the caller policy. Rust probes exclude native vendor workspace. Registry versions before 0.11 lack this variant; no substitute is measured as historical Kernix.
- Coalesced tensordot contracts [16,32,8] with [32,8,16] over two adjacent axes using the actual Kernix/NumPy tensor APIs. Shared-variable backward uses a prepared real graph of sum(X*X), 64x32 f64, and fresh gradient stores; recording and input construction are excluded in both frameworks.
- Per-call quantiles use nearest ranks over 256 individual warmed calls; batch quantiles use 15 means over separate 10-ms batches. They are separate distributions. Raw samples remain in raw.json.
- Eight warmup calls follow validation. Ordinary timing includes allocation, dispatch, computation and output destruction; explicitly marked reusable-output cases reuse their fixture buffer. The first call field excludes output destruction. Inputs and library/runtime imports are prepared outside timing.
- Three extra processes record their first operation calls. These are first-use observations, not cache-flushed or device-startup latency. Timer-pair overhead is reported without subtraction; tiny calls are clock-sensitive.
- ndarray and Candle softmax are explicit stable compositions, not claims about a native softmax API.
- Constructor cases allocate owned 64x64 f32 tensors. Pre-0.6 registry baselines use full(value), explicitly marked in metadata; current workspace runs use zeros/ones. No nested literal reinterpretation is involved.
- All GEMM inputs are C-order and match elementwise. Native output layout differs by library. QR materializes thin Q and R in every implementation. Cholesky returns reusable factors (or NumPy L); no extra factor materialization is timed.
- Pivoted QR materializes thin Q/R and an owned permutation in Kernix, nalgebra and faer. Minimum-norm cases include factorization and four RHS: Kernix uses pivoted QR plus scaled implicit RZ with relative cutoff 1e-12, NumPy uses native lstsq with rcond=1e-12, and nalgebra uses native SVD plus solve with absolute cutoff 1e-10. These algorithms have different costs and truncation policies. Fixtures have a wide rank gap; independent null-space orthogonality and normal-residual checks must both pass 2e-10 before timing. Results do not establish equivalence near a rank threshold or general SVD performance.
- Kernix checks finite values/symmetry. Some competitors validate a narrower contract. No finite-data precondition check was removed to improve timing. Accuracy results and budgets are retained per worker.
- The backward case uses an actual reverse-mode graph in Kernix and Candle for sum(A @ W), with A constant and W trainable. Both copy input/weight data and returned gradients; no manual derivative is used as a framework ranking.
- Allocations are measured in a separately compiled instrumented binary after warmup, outside all timing loops. Counts cover Rust global-allocator requests, not native allocator internals, stack memory or all NumPy allocations. NumPy allocation counts are unavailable, not zero.
- Native engines may retain thread-local packing buffers. Warmed allocation probes exclude first-use allocations and retained scratch; first-use timing and process RSS are separate observations. See the measured library numerical contract for that version.
- Requested-byte peak is additional live Rust allocation during one call, including output; it is not scratch-only memory or peak RSS. RSS is the process high-water mark including fixtures, validation and runtime, read from Linux VmHWM. Other platforms may report unavailable.
- The published cases are allocation-inclusive CPU APIs and graph workloads. Pure internal kernel-only and end-to-end application/model latency are not measured. No workload result implies a complete application frame rate.
- faer is explicitly serial; Rayon and BLAS thread counts are one. No GPU comparison. Python/NumPy timings include Python dispatch. Backend and dependency versions/configuration, governor and RAM are recorded.
- Training stages use batched matmul + bias + layer_norm + tanh + step-two feature slice + MSE (Candle index_select). Forward excludes graph recording; record includes parameter/input/target snapshots and eager forward/loss; backward reuses a prepared graph and returns fresh derivatives. Allocation probes measure the corresponding stage, including graph values/scratch, not graph-container-only bytes. SGD uses fixed precomputed real gradients and includes transactional validation/parameter updates; it is not an end-to-end converging training loop. Candle normalization is an explicit population-statistics composition; its checks/rounding differ. Historical registry baselines without these Tape operations omit the stages, recorded in metadata, rather than substitute manual derivatives.
- Host load/frequency/cache effects are uncontrolled. Report comparator wins and losses; no universal ranking follows. The source archive comes from committed bytes, with result directories excluded. Library dependencies are pinned by the saved driver lockfile.
