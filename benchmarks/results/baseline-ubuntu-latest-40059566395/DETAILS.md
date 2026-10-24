# Clean CPU evidence: baseline-ubuntu-latest-40059566395

Measured source `18750693908110110d85d02d0f11af8d58bfae8b`, tree `75403e48db8556f95e014ec2c5b16b30f9fa781e`; dirty before/after: false.
Library source: registry 0.10.0. CPU affinity: 0. Dtype explicitly named per workload; row-major inputs.

| Workload / library | Per-call p50 / p95 (us) | Batch-mean p50 / p95 (us) | Rust allocations | Peak additional requested bytes | Process peak RSS (MiB) |
|---|---:|---:|---:|---:|---:|
| cholesky_factor_128_f64/faer | 57.919 / 67.867 | 58.571 / 58.910 | 2 | 132096 | 5.96 |
| cholesky_factor_128_f64/kernix | 169.017 / 178.194 | 172.323 / 173.090 | 1 | 131072 | 3.61 |
| cholesky_factor_128_f64/nalgebra | 119.263 / 128.761 | 118.735 / 120.320 | 1 | 131072 | 3.25 |
| cholesky_factor_128_f64/numpy | 108.032 / 121.618 | 110.706 / 111.721 | unavailable | unavailable | 40.44 |
| cholesky_factor_256_f64/faer | 312.905 / 327.333 | 317.438 / 318.825 | 2 | 526336 | 4.89 |
| cholesky_factor_256_f64/kernix | 1013.949 / 1033.305 | 1013.784 / 1019.626 | 1 | 524288 | 4.64 |
| cholesky_factor_256_f64/nalgebra | 790.921 / 804.636 | 790.174 / 794.679 | 1 | 524288 | 4.14 |
| cholesky_factor_256_f64/numpy | 1080.593 / 1116.370 | 1080.801 / 1088.312 | unavailable | unavailable | 43.43 |
| cholesky_factor_512_f64/faer | 2616.349 / 2646.456 | 2617.501 / 2629.271 | 2 | 2101248 | 9.76 |
| cholesky_factor_512_f64/kernix | 7630.374 / 7732.073 | 7630.850 / 7701.176 | 1 | 2097152 | 9.10 |
| cholesky_factor_512_f64/nalgebra | 7353.676 / 7479.272 | 7333.789 / 7396.221 | 1 | 2097152 | 7.17 |
| cholesky_factor_512_f64/numpy | 6224.590 / 6659.895 | 6244.778 / 6305.566 | unavailable | unavailable | 54.00 |
| cholesky_factor_64_f64/faer | 14.087 / 14.167 | 14.257 / 14.337 | 2 | 33280 | 3.59 |
| cholesky_factor_64_f64/kernix | 24.967 / 25.167 | 25.208 / 25.623 | 2 | 33280 | 3.20 |
| cholesky_factor_64_f64/nalgebra | 20.969 / 21.501 | 21.122 / 21.330 | 1 | 32768 | 3.18 |
| cholesky_factor_64_f64/numpy | 18.123 / 19.537 | 18.451 / 18.877 | unavailable | unavailable | 39.69 |
| create_ones_64x64_f32/candle | 0.470 / 0.471 | 0.477 / 0.482 | 7 | 16696 | 3.59 |
| create_ones_64x64_f32/faer | 0.391 / 0.401 | 0.392 / 0.397 | 1 | 16384 | 3.29 |
| create_ones_64x64_f32/kernix | 0.431 / 0.431 | 0.439 / 0.441 | 1 | 16384 | 3.00 |
| create_ones_64x64_f32/ndarray | 0.351 / 0.361 | 0.359 / 0.361 | 1 | 16384 | 3.29 |
| create_ones_64x64_f32/numpy | 2.064 / 2.124 | 2.162 / 2.211 | unavailable | unavailable | 34.62 |
| create_zeros_64x64_f32/candle | 0.301 / 0.311 | 0.311 / 0.312 | 5 | 16648 | 3.23 |
| create_zeros_64x64_f32/faer | 0.330 / 0.331 | 0.295 / 0.300 | 1 | 16384 | 3.32 |
| create_zeros_64x64_f32/kernix | 0.321 / 0.331 | 0.331 / 0.332 | 1 | 16384 | 3.24 |
| create_zeros_64x64_f32/ndarray | 0.241 / 0.251 | 0.248 / 0.249 | 1 | 16384 | 3.31 |
| create_zeros_64x64_f32/numpy | 0.742 / 0.802 | 0.833 / 0.850 | unavailable | unavailable | 34.50 |
| matmul_128_f32/candle | 46.036 / 48.541 | 46.752 / 46.928 | 9 | 65816 | 4.27 |
| matmul_128_f32/faer | 49.212 / 58.710 | 51.427 / 52.492 | 1 | 65536 | 5.77 |
| matmul_128_f32/kernix | 46.407 / 50.044 | 46.944 / 47.174 | 3 | 65544 | 3.98 |
| matmul_128_f32/nalgebra | 73.337 / 86.852 | 76.669 / 77.191 | 2 | 163840 | 3.41 |
| matmul_128_f32/ndarray | 74.670 / 90.650 | 78.078 / 85.411 | 2 | 163840 | 3.57 |
| matmul_128_f32/numpy | 49.392 / 59.411 | 50.106 / 50.339 | unavailable | unavailable | 38.31 |
| matmul_128_f64/candle | 93.174 / 106.569 | 94.369 / 95.329 | 9 | 131352 | 4.46 |
| matmul_128_f64/faer | 100.428 / 112.291 | 100.840 / 101.532 | 1 | 131072 | 5.58 |
| matmul_128_f64/kernix | 93.866 / 103.915 | 94.974 / 96.503 | 3 | 131080 | 4.22 |
| matmul_128_f64/nalgebra | 200.324 / 216.456 | 201.281 / 202.664 | 2 | 327680 | 3.63 |
| matmul_128_f64/ndarray | 204.092 / 223.629 | 205.323 / 206.187 | 2 | 327680 | 3.52 |
| matmul_128_f64/numpy | 101.881 / 113.362 | 104.123 / 105.361 | unavailable | unavailable | 38.50 |
| matmul_256_f32/candle | 352.610 / 369.622 | 356.342 / 359.192 | 9 | 262424 | 4.89 |
| matmul_256_f32/faer | 408.114 / 420.387 | 411.274 / 413.304 | 1 | 262144 | 5.99 |
| matmul_256_f32/kernix | 356.397 / 370.053 | 358.366 / 359.762 | 3 | 262152 | 4.66 |
| matmul_256_f32/nalgebra | 600.475 / 618.188 | 599.447 / 602.182 | 2 | 589824 | 3.85 |
| matmul_256_f32/ndarray | 597.340 / 614.622 | 596.978 / 606.918 | 2 | 589824 | 4.07 |
| matmul_256_f32/numpy | 362.749 / 375.673 | 367.482 / 371.193 | unavailable | unavailable | 41.65 |
| matmul_256_f64/candle | 737.180 / 749.373 | 736.511 / 739.608 | 9 | 524568 | 6.07 |
| matmul_256_f64/faer | 990.484 / 1014.349 | 987.585 / 992.882 | 1 | 524288 | 5.08 |
| matmul_256_f64/kernix | 733.843 / 746.347 | 733.806 / 742.765 | 3 | 524296 | 5.41 |
| matmul_256_f64/nalgebra | 901.147 / 911.726 | 903.583 / 907.817 | 2 | 1179648 | 4.89 |
| matmul_256_f64/ndarray | 908.020 / 920.744 | 909.876 / 914.694 | 2 | 1179648 | 5.45 |
| matmul_256_f64/numpy | 768.178 / 792.183 | 769.096 / 772.344 | unavailable | unavailable | 42.33 |
| matmul_32_f64/candle | 2.164 / 2.204 | 2.258 / 2.273 | 9 | 8472 | 4.07 |
| matmul_32_f64/faer | 2.034 / 2.064 | 2.034 / 2.061 | 1 | 8192 | 5.58 |
| matmul_32_f64/kernix | 2.515 / 2.575 | 2.547 / 2.561 | 3 | 24576 | 3.49 |
| matmul_32_f64/nalgebra | 2.124 / 2.154 | 2.142 / 2.148 | 2 | 24576 | 3.41 |
| matmul_32_f64/ndarray | 2.184 / 2.215 | 2.194 / 2.225 | 2 | 24576 | 3.24 |
| matmul_32_f64/numpy | 3.778 / 3.898 | 3.937 / 3.962 | unavailable | unavailable | 37.33 |
| matmul_64_f32/candle | 6.723 / 6.802 | 6.825 / 6.941 | 9 | 16664 | 4.06 |
| matmul_64_f32/faer | 6.502 / 6.582 | 6.561 / 6.579 | 1 | 16384 | 5.48 |
| matmul_64_f32/kernix | 6.522 / 6.612 | 6.563 / 6.629 | 3 | 16448 | 3.74 |
| matmul_64_f32/nalgebra | 7.464 / 7.554 | 7.545 / 7.578 | 2 | 49152 | 3.32 |
| matmul_64_f32/ndarray | 7.494 / 7.574 | 7.530 / 7.639 | 2 | 49152 | 3.37 |
| matmul_64_f32/numpy | 8.135 / 13.505 | 8.449 / 12.138 | unavailable | unavailable | 37.49 |
| matmul_64_f64/candle | 12.814 / 12.934 | 12.996 / 13.060 | 9 | 33048 | 4.06 |
| matmul_64_f64/faer | 13.385 / 13.726 | 13.653 / 13.687 | 1 | 32768 | 5.64 |
| matmul_64_f64/kernix | 12.754 / 12.864 | 12.920 / 13.158 | 3 | 32832 | 4.00 |
| matmul_64_f64/nalgebra | 14.988 / 15.129 | 15.207 / 15.242 | 2 | 98304 | 3.24 |
| matmul_64_f64/ndarray | 15.520 / 15.620 | 15.674 / 15.735 | 2 | 98304 | 3.47 |
| matmul_64_f64/numpy | 15.058 / 15.259 | 15.332 / 15.431 | unavailable | unavailable | 37.45 |
| matmul_ab_128_f32/kernix | 46.087 / 51.096 | 46.589 / 46.862 | 3 | 65544 | 4.11 |
| matmul_ab_128_f32/numpy | 49.573 / 60.454 | 50.347 / 50.666 | unavailable | unavailable | 38.21 |
| matmul_ab_128_f64/kernix | 93.836 / 104.706 | 94.911 / 95.902 | 3 | 131080 | 4.27 |
| matmul_ab_128_f64/numpy | 102.021 / 113.022 | 103.740 / 104.335 | unavailable | unavailable | 38.38 |
| matmul_batched_4x16x32x16_f64/candle | 3.236 / 3.337 | 3.263 / 3.298 | 25 | 25384 | 4.10 |
| matmul_batched_4x16x32x16_f64/kernix | 3.076 / 3.146 | 3.115 / 3.148 | 7 | 16392 | 3.61 |
| matmul_batched_4x16x32x16_f64/numpy | 4.218 / 4.298 | 4.379 / 4.415 | unavailable | unavailable | 37.25 |
| matmul_sum_backward_32x16x8_f64/candle | 7.715 / 8.035 | 7.779 / 7.865 | 113 | 27988 | 4.53 |
| matmul_sum_backward_32x16x8_f64/kernix | 3.877 / 4.047 | 3.954 / 3.983 | 16 | 18376 | 3.54 |
| min_norm_32x64_rank32_f64/kernix | 84.669 / 93.485 | 85.632 / 86.144 | 20 | 59136 | 3.77 |
| min_norm_32x64_rank32_f64/nalgebra | 178.555 / 187.281 | 180.678 / 185.180 | 15 | 42232 | 3.51 |
| min_norm_32x64_rank32_f64/numpy | 185.417 / 206.076 | 189.435 / 192.764 | unavailable | unavailable | 38.50 |
| min_norm_64x32_rank16_f64/kernix | 62.467 / 71.334 | 63.210 / 63.325 | 20 | 74752 | 3.80 |
| min_norm_64x32_rank16_f64/nalgebra | 103.223 / 115.657 | 104.539 / 105.519 | 15 | 41976 | 3.46 |
| min_norm_64x32_rank16_f64/numpy | 102.713 / 122.890 | 105.349 / 108.923 | unavailable | unavailable | 38.39 |
| parallel_matmul_into_128_f32/kernix | 32.000 / 33.803 | 32.439 / 34.426 | 2 | 16 | 4.36 |
| parallel_matmul_into_128_f64/kernix | 57.127 / 67.777 | 58.011 / 58.571 | 2 | 16 | 4.50 |
| parallel_matmul_into_256_f64/kernix | 392.625 / 412.662 | 394.972 / 640.582 | 2 | 16 | 6.37 |
| parallel_matmul_into_64_f64/kernix | 18.845 / 19.326 | 18.876 / 19.189 | 2 | 98304 | 4.09 |
| pivoted_qr_256x64_f64/faer | 345.727 / 377.197 | 352.160 / 358.687 | 10 | 333312 | 6.20 |
| pivoted_qr_256x64_f64/kernix | 851.164 / 873.515 | 819.241 / 850.847 | 14 | 559104 | 4.09 |
| pivoted_qr_256x64_f64/nalgebra | 762.537 / 782.935 | 761.517 / 767.489 | 7 | 296960 | 3.64 |
| pivoted_qr_96x32_f64/faer | 43.021 / 46.017 | 43.508 / 43.890 | 10 | 68352 | 5.76 |
| pivoted_qr_96x32_f64/kernix | 78.427 / 87.975 | 79.869 / 81.009 | 14 | 107520 | 3.56 |
| pivoted_qr_96x32_f64/nalgebra | 78.367 / 87.724 | 79.051 / 80.046 | 7 | 58368 | 3.48 |
| qr_256x64_f64/faer | 418.002 / 441.607 | 421.309 / 425.172 | 7 | 331776 | 6.38 |
| qr_256x64_f64/kernix | 779.228 / 793.375 | 782.071 / 807.027 | 6 | 426496 | 4.06 |
| qr_256x64_f64/nalgebra | 569.847 / 585.265 | 571.981 / 576.748 | 4 | 295424 | 3.53 |
| qr_256x64_f64/numpy | 537.937 / 553.757 | 534.405 / 540.191 | unavailable | unavailable | 39.03 |
| qr_96x32_f64/faer | 54.001 / 63.429 | 54.611 / 56.504 | 7 | 67584 | 5.99 |
| qr_96x32_f64/kernix | 66.805 / 79.018 | 68.102 / 68.761 | 6 | 82176 | 3.47 |
| qr_96x32_f64/nalgebra | 58.880 / 67.216 | 59.618 / 60.042 | 4 | 57600 | 3.35 |
| qr_96x32_f64/numpy | 69.781 / 92.153 | 71.109 / 71.887 | unavailable | unavailable | 38.52 |
| softmax_256x64_f64/candle | 115.285 / 129.642 | 116.759 / 117.795 | 37 | 265200 | 4.90 |
| softmax_256x64_f64/kernix | 119.564 / 128.701 | 120.839 / 121.388 | 1 | 131072 | 3.76 |
| softmax_256x64_f64/ndarray | 113.523 / 126.546 | 114.996 / 116.008 | 1 | 131072 | 3.66 |
| softmax_256x64_f64/numpy | 133.509 / 146.164 | 133.932 / 137.570 | unavailable | unavailable | 35.10 |
| tensordot_16x32x8_32x8x16_f64/kernix | 63.689 / 71.805 | 64.287 / 64.713 | 9 | 2166 | 3.49 |
| tensordot_16x32x8_32x8x16_f64/numpy | 11.091 / 11.531 | 11.317 / 11.534 | unavailable | unavailable | 37.21 |
| training_backward_2x4x8x4_f64/candle | 20.859 / 21.561 | 20.802 / 20.986 | 472 | 8332 | 4.80 |
| training_backward_2x4x8x4_f64/kernix | 4.969 / 5.060 | 5.082 / 5.118 | 23 | 4224 | 3.89 |
| training_forward_2x4x8x4_f64/candle | 5.010 / 5.120 | 5.073 / 5.114 | 126 | 3912 | 4.65 |
| training_forward_2x4x8x4_f64/kernix | 3.376 / 3.447 | 3.430 / 3.450 | 17 | 1032 | 3.69 |
| training_record_2x4x8x4_f64/candle | 6.773 / 7.093 | 6.868 / 6.900 | 149 | 10712 | 4.50 |
| training_record_2x4x8x4_f64/kernix | 4.168 / 4.248 | 4.260 / 4.297 | 22 | 6832 | 3.68 |
| training_sgd_2x4x8x4_f64/kernix | 0.802 / 0.812 | 0.821 / 0.825 | 5 | 688 | 4.01 |
| training_shared_backward_64x32_f64/candle | 4.949 / 5.060 | 4.972 / 5.014 | 58 | 100004 | 4.26 |
| training_shared_backward_64x32_f64/kernix | 10.540 / 10.600 | 10.606 / 10.675 | 8 | 49768 | 3.73 |

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
