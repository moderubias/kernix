# Clean CPU evidence: runner-macos-latest-40000558153

Measured source `1c53d4663a48de5a588809c2b69c3a6a5bdad7b2`, tree `d5b67d509ef5c0b81991af0933b482b13552047e`; dirty before/after: false.
Library source: workspace 1c53d4663a48de5a588809c2b69c3a6a5bdad7b2. CPU affinity: not pinned. Dtype explicitly named per workload; row-major inputs.

| Workload / library | Per-call p50 / p95 (us) | Batch-mean p50 / p95 (us) | Rust allocations | Peak additional requested bytes | Process peak RSS (MiB) |
|---|---:|---:|---:|---:|---:|
| cholesky_factor_128_f64/faer | 48.208 / 53.792 | 50.406 / 62.090 | 16 | 149504 | unavailable |
| cholesky_factor_128_f64/kernix | 172.250 / 208.084 | 186.051 / 194.619 | 1 | 131072 | unavailable |
| cholesky_factor_128_f64/nalgebra | 100.834 / 110.083 | 100.155 / 102.730 | 1 | 131072 | unavailable |
| cholesky_factor_128_f64/numpy | 48.250 / 54.584 | 46.507 / 151.056 | unavailable | unavailable | unavailable |
| cholesky_factor_256_f64/faer | 252.416 / 311.667 | 279.843 / 287.292 | 76 | 592384 | unavailable |
| cholesky_factor_256_f64/kernix | 1249.167 / 1367.583 | 1273.380 / 1361.776 | 1 | 524288 | unavailable |
| cholesky_factor_256_f64/nalgebra | 702.708 / 1087.166 | 744.024 / 787.994 | 1 | 524288 | unavailable |
| cholesky_factor_256_f64/numpy | 316.791 / 389.375 | 359.632 / 500.527 | unavailable | unavailable | unavailable |
| cholesky_factor_64_f64/faer | 10.459 / 12.917 | 10.937 / 12.009 | 2 | 33280 | unavailable |
| cholesky_factor_64_f64/kernix | 21.250 / 25.375 | 21.314 / 24.621 | 2 | 33280 | unavailable |
| cholesky_factor_64_f64/nalgebra | 21.000 / 26.041 | 21.476 / 22.755 | 1 | 32768 | unavailable |
| cholesky_factor_64_f64/numpy | 15.000 / 17.833 | 18.273 / 24.008 | unavailable | unavailable | unavailable |
| create_ones_64x64_f32/candle | 0.375 / 0.459 | 0.390 / 0.488 | 7 | 16696 | unavailable |
| create_ones_64x64_f32/faer | 0.333 / 0.334 | 0.339 / 2.064 | 1 | 16384 | unavailable |
| create_ones_64x64_f32/kernix | 0.333 / 0.375 | 0.307 / 0.341 | 1 | 16384 | unavailable |
| create_ones_64x64_f32/ndarray | 0.333 / 0.541 | 0.253 / 0.314 | 1 | 16384 | unavailable |
| create_ones_64x64_f32/numpy | 1.125 / 1.375 | 1.219 / 1.312 | unavailable | unavailable | unavailable |
| create_zeros_64x64_f32/candle | 0.416 / 0.709 | 0.403 / 0.593 | 5 | 16648 | unavailable |
| create_zeros_64x64_f32/faer | 0.250 / 0.292 | 0.265 / 0.288 | 1 | 16384 | unavailable |
| create_zeros_64x64_f32/kernix | 0.250 / 0.292 | 0.290 / 0.522 | 1 | 16384 | unavailable |
| create_zeros_64x64_f32/ndarray | 0.333 / 0.375 | 0.292 / 0.326 | 1 | 16384 | unavailable |
| create_zeros_64x64_f32/numpy | 0.500 / 0.542 | 0.562 / 0.875 | unavailable | unavailable | unavailable |
| matmul_128_f32/candle | 52.959 / 60.792 | 54.927 / 70.259 | 10 | 327768 | unavailable |
| matmul_128_f32/faer | 61.292 / 87.958 | 61.211 / 72.287 | 3 | 393224 | unavailable |
| matmul_128_f32/kernix | 48.875 / 49.083 | 51.304 / 59.074 | 3 | 163840 | unavailable |
| matmul_128_f32/nalgebra | 47.791 / 55.875 | 49.685 / 55.581 | 2 | 163840 | unavailable |
| matmul_128_f32/ndarray | 48.667 / 48.792 | 49.107 / 56.428 | 2 | 163840 | unavailable |
| matmul_128_f32/numpy | 5.542 / 6.042 | 5.822 / 6.277 | unavailable | unavailable | unavailable |
| matmul_128_f64/candle | 95.750 / 115.417 | 107.057 / 117.776 | 10 | 655448 | unavailable |
| matmul_128_f64/faer | 106.083 / 147.417 | 107.715 / 115.268 | 3 | 786440 | unavailable |
| matmul_128_f64/kernix | 102.541 / 117.625 | 102.163 / 112.673 | 3 | 327680 | unavailable |
| matmul_128_f64/nalgebra | 102.458 / 110.250 | 107.142 / 112.340 | 2 | 327680 | unavailable |
| matmul_128_f64/ndarray | 101.958 / 187.833 | 98.567 / 123.764 | 2 | 327680 | unavailable |
| matmul_128_f64/numpy | 19.250 / 96.667 | 18.290 / 25.181 | unavailable | unavailable | unavailable |
| matmul_256_f32/candle | 468.417 / 1298.959 | 566.694 / 809.964 | 10 | 786520 | unavailable |
| matmul_256_f32/faer | 414.000 / 579.458 | 466.909 / 554.103 | 3 | 1048584 | unavailable |
| matmul_256_f32/kernix | 392.750 / 586.041 | 414.622 / 962.468 | 3 | 589824 | unavailable |
| matmul_256_f32/nalgebra | 406.916 / 475.041 | 421.059 / 486.919 | 2 | 589824 | unavailable |
| matmul_256_f32/ndarray | 391.375 / 664.417 | 401.817 / 448.357 | 2 | 589824 | unavailable |
| matmul_256_f32/numpy | 29.500 / 30.000 | 28.989 / 33.503 | unavailable | unavailable | unavailable |
| matmul_256_f64/candle | 820.417 / 931.834 | 818.119 / 1025.233 | 10 | 1572952 | unavailable |
| matmul_256_f64/faer | 838.875 / 935.416 | 845.250 / 878.653 | 3 | 1572872 | unavailable |
| matmul_256_f64/kernix | 751.583 / 825.958 | 778.506 / 842.656 | 3 | 1179648 | unavailable |
| matmul_256_f64/nalgebra | 786.584 / 877.250 | 825.801 / 998.473 | 2 | 1179648 | unavailable |
| matmul_256_f64/ndarray | 802.791 / 910.917 | 817.465 / 856.594 | 2 | 1179648 | unavailable |
| matmul_256_f64/numpy | 110.583 / 129.666 | 115.542 / 124.665 | unavailable | unavailable | unavailable |
| matmul_32_f64/candle | 2.584 / 3.292 | 2.410 / 3.524 | 10 | 18512 | unavailable |
| matmul_32_f64/faer | 2.417 / 2.959 | 2.343 / 2.606 | 3 | 26624 | unavailable |
| matmul_32_f64/kernix | 2.250 / 2.584 | 2.308 / 2.400 | 3 | 24576 | unavailable |
| matmul_32_f64/nalgebra | 2.000 / 2.042 | 2.269 / 5.385 | 2 | 24576 | unavailable |
| matmul_32_f64/ndarray | 2.417 / 3.000 | 2.445 / 3.255 | 2 | 24576 | unavailable |
| matmul_32_f64/numpy | 1.333 / 1.708 | 1.621 / 1.869 | unavailable | unavailable | unavailable |
| matmul_64_f32/candle | 7.833 / 11.125 | 8.461 / 11.617 | 10 | 33872 | unavailable |
| matmul_64_f32/faer | 8.208 / 8.917 | 8.539 / 13.861 | 3 | 50176 | unavailable |
| matmul_64_f32/kernix | 7.250 / 8.625 | 7.550 / 7.884 | 3 | 49152 | unavailable |
| matmul_64_f32/nalgebra | 6.958 / 8.625 | 7.371 / 7.612 | 2 | 49152 | unavailable |
| matmul_64_f32/ndarray | 6.917 / 8.084 | 6.876 / 7.334 | 2 | 49152 | unavailable |
| matmul_64_f32/numpy | 1.833 / 2.042 | 1.865 / 2.147 | unavailable | unavailable | unavailable |
| matmul_64_f64/candle | 14.500 / 18.209 | 16.165 / 19.653 | 10 | 66640 | unavailable |
| matmul_64_f64/faer | 14.709 / 18.666 | 16.893 / 25.815 | 3 | 99328 | unavailable |
| matmul_64_f64/kernix | 13.167 / 15.125 | 13.469 / 15.419 | 3 | 98304 | unavailable |
| matmul_64_f64/nalgebra | 13.542 / 15.917 | 14.876 / 16.224 | 2 | 98304 | unavailable |
| matmul_64_f64/ndarray | 13.250 / 16.208 | 14.519 / 22.794 | 2 | 98304 | unavailable |
| matmul_64_f64/numpy | 3.250 / 3.458 | 3.482 / 46.524 | unavailable | unavailable | unavailable |
| matmul_batched_4x16x32x16_f64/candle | 3.792 / 5.750 | 3.608 / 19.090 | 25 | 27232 | unavailable |
| matmul_batched_4x16x32x16_f64/kernix | 2.834 / 3.584 | 2.828 / 3.088 | 7 | 16392 | unavailable |
| matmul_batched_4x16x32x16_f64/numpy | 2.000 / 2.250 | 1.972 / 2.046 | unavailable | unavailable | unavailable |
| matmul_sum_backward_32x16x8_f64/candle | 6.458 / 7.459 | 6.330 / 8.087 | 113 | 27980 | unavailable |
| matmul_sum_backward_32x16x8_f64/kernix | 2.959 / 3.459 | 3.186 / 3.404 | 16 | 18376 | unavailable |
| min_norm_32x64_rank32_f64/kernix | 68.542 / 84.542 | 71.760 / 99.128 | 20 | 59136 | unavailable |
| min_norm_32x64_rank32_f64/nalgebra | 145.042 / 155.875 | 160.192 / 168.929 | 15 | 42232 | unavailable |
| min_norm_32x64_rank32_f64/numpy | 138.459 / 153.833 | 153.134 / 170.274 | unavailable | unavailable | unavailable |
| min_norm_64x32_rank16_f64/kernix | 49.792 / 56.458 | 53.156 / 70.985 | 20 | 74752 | unavailable |
| min_norm_64x32_rank16_f64/nalgebra | 72.292 / 124.958 | 75.764 / 92.227 | 15 | 41976 | unavailable |
| min_norm_64x32_rank16_f64/numpy | 77.917 / 88.750 | 78.487 / 94.214 | unavailable | unavailable | unavailable |
| parallel_matmul_into_128_f32/kernix | 53.500 / 67.875 | 52.024 / 7301.396 | 4 | 163840 | unavailable |
| parallel_matmul_into_128_f64/kernix | 89.500 / 285.083 | 106.656 / 20136.125 | 4 | 163840 | unavailable |
| parallel_matmul_into_256_f64/kernix | 518.917 / 9988.500 | 1609.274 / 14817.500 | 8 | 1179648 | unavailable |
| parallel_matmul_into_64_f64/kernix | 24.250 / 59.709 | 23.863 / 52.885 | 2 | 98304 | unavailable |
| pivoted_qr_256x64_f64/faer | 321.250 / 350.083 | 341.476 / 403.153 | 58 | 336904 | unavailable |
| pivoted_qr_256x64_f64/kernix | 530.291 / 590.459 | 564.220 / 619.667 | 14 | 559104 | unavailable |
| pivoted_qr_256x64_f64/nalgebra | 983.750 / 1194.625 | 1142.442 / 11324.167 | 7 | 296960 | unavailable |
| pivoted_qr_96x32_f64/faer | 62.000 / 111.541 | 66.899 / 75.691 | 32 | 94728 | unavailable |
| pivoted_qr_96x32_f64/kernix | 67.959 / 122.792 | 75.263 / 637.815 | 14 | 107520 | unavailable |
| pivoted_qr_96x32_f64/nalgebra | 92.042 / 95.250 | 102.531 / 110.826 | 7 | 58368 | unavailable |
| qr_256x64_f64/faer | 285.125 / 304.708 | 330.103 / 351.394 | 83 | 335880 | unavailable |
| qr_256x64_f64/kernix | 526.250 / 590.458 | 538.097 / 562.789 | 6 | 426496 | unavailable |
| qr_256x64_f64/nalgebra | 361.417 / 467.250 | 362.225 / 436.130 | 4 | 295424 | unavailable |
| qr_256x64_f64/numpy | 345.667 / 388.291 | 441.857 / 1021.208 | unavailable | unavailable | unavailable |
| qr_96x32_f64/faer | 42.917 / 47.500 | 47.929 / 49.137 | 41 | 94216 | unavailable |
| qr_96x32_f64/kernix | 54.875 / 63.000 | 54.090 / 60.815 | 6 | 82176 | unavailable |
| qr_96x32_f64/nalgebra | 39.500 / 39.583 | 41.091 / 44.390 | 4 | 57600 | unavailable |
| qr_96x32_f64/numpy | 50.167 / 61.041 | 54.637 / 90.967 | unavailable | unavailable | unavailable |
| softmax_256x64_f64/candle | 80.167 / 112.458 | 76.298 / 116.330 | 37 | 265200 | unavailable |
| softmax_256x64_f64/kernix | 78.041 / 79.542 | 86.016 / 90.266 | 1 | 131072 | unavailable |
| softmax_256x64_f64/ndarray | 64.167 / 75.833 | 64.841 / 67.296 | 1 | 131072 | unavailable |
| softmax_256x64_f64/numpy | 68.833 / 70.959 | 79.017 / 100.862 | unavailable | unavailable | unavailable |
| tensordot_16x32x8_32x8x16_f64/kernix | 61.875 / 71.583 | 64.212 / 66.769 | 9 | 2166 | unavailable |
| tensordot_16x32x8_32x8x16_f64/numpy | 5.708 / 6.083 | 5.502 / 6.047 | unavailable | unavailable | unavailable |
| training_backward_2x4x8x4_f64/candle | 20.292 / 23.583 | 18.214 / 20.794 | 476 | 21284 | unavailable |
| training_backward_2x4x8x4_f64/kernix | 3.542 / 4.666 | 4.362 / 13.929 | 23 | 4224 | unavailable |
| training_forward_2x4x8x4_f64/candle | 6.208 / 10.458 | 5.516 / 8.986 | 126 | 9568 | unavailable |
| training_forward_2x4x8x4_f64/kernix | 2.333 / 2.750 | 2.069 / 2.333 | 17 | 1032 | unavailable |
| training_record_2x4x8x4_f64/candle | 6.125 / 7.125 | 5.803 / 7.073 | 149 | 11544 | unavailable |
| training_record_2x4x8x4_f64/kernix | 3.000 / 3.208 | 3.105 / 3.910 | 22 | 6832 | unavailable |
| training_sgd_2x4x8x4_f64/kernix | 0.708 / 0.709 | 0.707 / 0.781 | 5 | 688 | unavailable |
| training_shared_backward_64x32_f64/candle | 3.959 / 4.083 | 3.865 / 5.291 | 58 | 99996 | unavailable |
| training_shared_backward_64x32_f64/kernix | 6.917 / 7.792 | 6.324 / 37.064 | 8 | 49768 | unavailable |

## Method and limits

- Rows explicitly marked two workers use a private Kernix executor with two workers and reusable output allocated before the first call. RefCell borrowing is included; pool construction is excluded. These rows compare Kernix versions, with no single-thread peer score. Linux uses up to two allowed CPUs, preferring distinct reported physical cores; Windows/macOS affinity is not pinned. Exact per-workload CPU sets are in explicit_execution metadata. Ordinary cases retain one BLAS/Rayon thread and the original single-CPU Linux policy.
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
