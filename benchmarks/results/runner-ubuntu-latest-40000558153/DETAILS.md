# Clean CPU evidence: runner-ubuntu-latest-40000558153

Measured source `1c53d4663a48de5a588809c2b69c3a6a5bdad7b2`, tree `d5b67d509ef5c0b81991af0933b482b13552047e`; dirty before/after: false.
Library source: workspace 1c53d4663a48de5a588809c2b69c3a6a5bdad7b2. CPU affinity: 0. Dtype explicitly named per workload; row-major inputs.

| Workload / library | Per-call p50 / p95 (us) | Batch-mean p50 / p95 (us) | Rust allocations | Peak additional requested bytes | Process peak RSS (MiB) |
|---|---:|---:|---:|---:|---:|
| cholesky_factor_128_f64/faer | 58.149 / 68.548 | 58.873 / 59.185 | 2 | 132096 | 5.97 |
| cholesky_factor_128_f64/kernix | 170.349 / 179.736 | 171.860 / 172.758 | 1 | 131072 | 3.48 |
| cholesky_factor_128_f64/nalgebra | 120.695 / 130.583 | 115.978 / 117.359 | 1 | 131072 | 3.32 |
| cholesky_factor_128_f64/numpy | 106.949 / 120.304 | 108.971 / 111.367 | unavailable | unavailable | 40.35 |
| cholesky_factor_256_f64/faer | 312.673 / 329.906 | 317.127 / 319.701 | 2 | 526336 | 6.68 |
| cholesky_factor_256_f64/kernix | 1029.482 / 1044.119 | 1031.730 / 1044.062 | 1 | 524288 | 4.67 |
| cholesky_factor_256_f64/nalgebra | 821.313 / 833.597 | 785.168 / 806.900 | 1 | 524288 | 4.03 |
| cholesky_factor_256_f64/numpy | 1050.872 / 1077.772 | 1053.405 / 1062.103 | unavailable | unavailable | 43.23 |
| cholesky_factor_64_f64/faer | 14.177 / 14.247 | 14.322 / 14.468 | 2 | 33280 | 3.49 |
| cholesky_factor_64_f64/kernix | 25.017 / 25.308 | 25.287 / 25.477 | 2 | 33280 | 3.36 |
| cholesky_factor_64_f64/nalgebra | 20.218 / 20.528 | 20.269 / 20.865 | 1 | 32768 | 3.14 |
| cholesky_factor_64_f64/numpy | 18.144 / 20.538 | 18.637 / 21.340 | unavailable | unavailable | 39.72 |
| create_ones_64x64_f32/candle | 0.461 / 1.022 | 0.475 / 0.506 | 7 | 16696 | 3.64 |
| create_ones_64x64_f32/faer | 0.391 / 0.401 | 0.382 / 0.384 | 1 | 16384 | 3.45 |
| create_ones_64x64_f32/kernix | 0.431 / 0.441 | 0.441 / 0.443 | 1 | 16384 | 3.12 |
| create_ones_64x64_f32/ndarray | 0.361 / 0.361 | 0.366 / 0.368 | 1 | 16384 | 3.20 |
| create_ones_64x64_f32/numpy | 2.084 / 2.244 | 2.220 / 2.267 | unavailable | unavailable | 34.33 |
| create_zeros_64x64_f32/candle | 0.301 / 0.311 | 0.312 / 0.316 | 5 | 16648 | 3.45 |
| create_zeros_64x64_f32/faer | 0.341 / 0.341 | 0.296 / 0.299 | 1 | 16384 | 3.29 |
| create_zeros_64x64_f32/kernix | 0.320 / 0.321 | 0.328 / 0.333 | 1 | 16384 | 3.17 |
| create_zeros_64x64_f32/ndarray | 0.240 / 0.241 | 0.245 / 0.248 | 1 | 16384 | 3.16 |
| create_zeros_64x64_f32/numpy | 0.731 / 0.762 | 0.837 / 1.092 | unavailable | unavailable | 34.42 |
| matmul_128_f32/candle | 46.025 / 46.948 | 46.713 / 47.577 | 9 | 65816 | 4.32 |
| matmul_128_f32/faer | 51.026 / 59.090 | 51.471 / 51.731 | 1 | 65536 | 5.64 |
| matmul_128_f32/kernix | 46.327 / 55.294 | 46.769 / 47.027 | 3 | 65544 | 4.05 |
| matmul_128_f32/nalgebra | 73.387 / 91.361 | 76.912 / 77.532 | 2 | 163840 | 3.42 |
| matmul_128_f32/ndarray | 74.980 / 91.141 | 78.365 / 78.960 | 2 | 163840 | 3.50 |
| matmul_128_f32/numpy | 49.122 / 59.200 | 49.929 / 50.349 | unavailable | unavailable | 38.05 |
| matmul_128_f64/candle | 93.715 / 104.035 | 94.899 / 95.717 | 9 | 131352 | 4.51 |
| matmul_128_f64/faer | 100.477 / 118.371 | 101.139 / 101.713 | 1 | 131072 | 5.84 |
| matmul_128_f64/kernix | 93.655 / 104.525 | 94.714 / 96.024 | 3 | 131080 | 4.17 |
| matmul_128_f64/nalgebra | 202.298 / 226.643 | 204.342 / 208.865 | 2 | 327680 | 3.58 |
| matmul_128_f64/ndarray | 205.072 / 222.275 | 205.965 / 207.728 | 2 | 327680 | 3.67 |
| matmul_128_f64/numpy | 101.269 / 112.360 | 102.858 / 103.653 | unavailable | unavailable | 38.36 |
| matmul_256_f32/candle | 355.113 / 370.281 | 358.091 / 360.093 | 9 | 262424 | 4.96 |
| matmul_256_f32/faer | 413.121 / 428.239 | 416.057 / 423.049 | 1 | 262144 | 5.93 |
| matmul_256_f32/kernix | 355.824 / 369.640 | 359.523 / 370.244 | 3 | 262152 | 4.57 |
| matmul_256_f32/nalgebra | 597.285 / 612.584 | 597.199 / 599.776 | 2 | 589824 | 3.91 |
| matmul_256_f32/ndarray | 603.857 / 647.157 | 599.840 / 602.382 | 2 | 589824 | 4.15 |
| matmul_256_f32/numpy | 364.320 / 377.776 | 367.347 / 369.879 | unavailable | unavailable | 42.02 |
| matmul_256_f64/candle | 740.312 / 761.071 | 740.430 / 746.189 | 9 | 524568 | 5.90 |
| matmul_256_f64/faer | 970.091 / 989.427 | 971.028 / 978.698 | 1 | 524288 | 6.49 |
| matmul_256_f64/kernix | 732.798 / 748.607 | 733.164 / 737.889 | 3 | 524296 | 5.38 |
| matmul_256_f64/nalgebra | 902.021 / 912.661 | 905.043 / 971.800 | 2 | 1179648 | 4.83 |
| matmul_256_f64/ndarray | 908.375 / 922.683 | 917.080 / 922.445 | 2 | 1179648 | 5.30 |
| matmul_256_f64/numpy | 766.902 / 788.062 | 764.848 / 772.082 | unavailable | unavailable | 42.67 |
| matmul_32_f64/candle | 2.174 / 2.205 | 2.205 / 2.222 | 9 | 8472 | 4.06 |
| matmul_32_f64/faer | 2.054 / 2.074 | 2.031 / 2.050 | 1 | 8192 | 5.54 |
| matmul_32_f64/kernix | 2.505 / 2.545 | 2.542 / 2.553 | 3 | 24576 | 3.55 |
| matmul_32_f64/nalgebra | 2.113 / 2.134 | 2.142 / 2.155 | 2 | 24576 | 3.20 |
| matmul_32_f64/ndarray | 2.184 / 2.205 | 2.192 / 2.203 | 2 | 24576 | 3.27 |
| matmul_32_f64/numpy | 3.657 / 3.738 | 3.820 / 3.885 | unavailable | unavailable | 37.05 |
| matmul_64_f32/candle | 6.592 / 6.693 | 6.692 / 6.745 | 9 | 16664 | 4.21 |
| matmul_64_f32/faer | 6.582 / 6.662 | 6.640 / 6.660 | 1 | 16384 | 5.59 |
| matmul_64_f32/kernix | 6.602 / 6.682 | 6.690 / 6.711 | 3 | 16448 | 3.84 |
| matmul_64_f32/nalgebra | 7.354 / 7.424 | 7.478 / 7.815 | 2 | 49152 | 3.24 |
| matmul_64_f32/ndarray | 7.534 / 7.624 | 7.606 / 7.686 | 2 | 49152 | 3.30 |
| matmul_64_f32/numpy | 8.135 / 8.236 | 8.307 / 8.467 | unavailable | unavailable | 37.35 |
| matmul_64_f64/candle | 12.974 / 13.074 | 13.115 / 13.186 | 9 | 33048 | 4.09 |
| matmul_64_f64/faer | 13.625 / 13.836 | 13.641 / 13.882 | 1 | 32768 | 5.64 |
| matmul_64_f64/kernix | 12.834 / 12.974 | 13.030 / 13.145 | 3 | 32832 | 3.95 |
| matmul_64_f64/nalgebra | 14.948 / 15.048 | 15.119 / 15.219 | 2 | 98304 | 3.41 |
| matmul_64_f64/ndarray | 15.539 / 15.639 | 15.722 / 15.804 | 2 | 98304 | 3.43 |
| matmul_64_f64/numpy | 14.998 / 15.318 | 15.272 / 15.304 | unavailable | unavailable | 37.46 |
| matmul_batched_4x16x32x16_f64/candle | 3.276 / 3.327 | 3.294 / 3.324 | 25 | 25384 | 4.21 |
| matmul_batched_4x16x32x16_f64/kernix | 3.086 / 3.156 | 3.132 / 3.161 | 7 | 16392 | 3.48 |
| matmul_batched_4x16x32x16_f64/numpy | 4.197 / 4.268 | 4.364 / 5.043 | unavailable | unavailable | 37.21 |
| matmul_sum_backward_32x16x8_f64/candle | 7.584 / 7.915 | 7.639 / 7.811 | 113 | 27988 | 4.47 |
| matmul_sum_backward_32x16x8_f64/kernix | 4.107 / 4.188 | 4.150 / 4.189 | 16 | 18376 | 3.61 |
| min_norm_32x64_rank32_f64/kernix | 86.080 / 95.408 | 87.326 / 88.351 | 20 | 59136 | 3.88 |
| min_norm_32x64_rank32_f64/nalgebra | 178.463 / 188.101 | 180.264 / 182.435 | 15 | 42232 | 3.36 |
| min_norm_32x64_rank32_f64/numpy | 183.693 / 208.269 | 187.332 / 191.000 | unavailable | unavailable | 38.39 |
| min_norm_64x32_rank16_f64/kernix | 62.777 / 72.005 | 63.542 / 64.044 | 20 | 74752 | 3.88 |
| min_norm_64x32_rank16_f64/nalgebra | 103.583 / 114.013 | 104.937 / 105.425 | 15 | 41976 | 3.45 |
| min_norm_64x32_rank16_f64/numpy | 102.742 / 121.717 | 105.305 / 106.516 | unavailable | unavailable | 38.31 |
| parallel_matmul_into_128_f32/kernix | 31.619 / 34.475 | 32.125 / 38.222 | 2 | 16 | 4.43 |
| parallel_matmul_into_128_f64/kernix | 57.177 / 69.119 | 58.074 / 59.337 | 2 | 16 | 4.70 |
| parallel_matmul_into_256_f64/kernix | 395.459 / 411.719 | 399.259 / 401.158 | 2 | 16 | 6.20 |
| parallel_matmul_into_64_f64/kernix | 16.601 / 17.041 | 18.581 / 20.288 | 2 | 98304 | 3.93 |
| pivoted_qr_256x64_f64/faer | 346.206 / 381.893 | 356.648 / 360.486 | 10 | 333312 | 6.25 |
| pivoted_qr_256x64_f64/kernix | 851.550 / 871.656 | 821.577 / 849.982 | 14 | 559104 | 4.12 |
| pivoted_qr_256x64_f64/nalgebra | 756.301 / 769.185 | 757.507 / 763.091 | 7 | 296960 | 3.76 |
| pivoted_qr_96x32_f64/faer | 43.501 / 52.838 | 43.933 / 44.319 | 10 | 68352 | 6.04 |
| pivoted_qr_96x32_f64/kernix | 78.687 / 88.455 | 80.077 / 80.626 | 14 | 107520 | 3.52 |
| pivoted_qr_96x32_f64/nalgebra | 76.964 / 85.901 | 77.585 / 78.084 | 7 | 58368 | 3.30 |
| qr_256x64_f64/faer | 417.629 / 440.122 | 420.682 / 422.879 | 7 | 331776 | 6.25 |
| qr_256x64_f64/kernix | 770.688 / 794.243 | 769.467 / 812.521 | 6 | 426496 | 4.05 |
| qr_256x64_f64/nalgebra | 557.621 / 573.521 | 558.888 / 564.163 | 4 | 295424 | 3.57 |
| qr_256x64_f64/numpy | 537.844 / 555.166 | 534.339 / 539.453 | unavailable | unavailable | 38.97 |
| qr_96x32_f64/faer | 54.903 / 64.671 | 55.567 / 56.359 | 7 | 67584 | 5.90 |
| qr_96x32_f64/kernix | 65.522 / 74.419 | 66.652 / 67.058 | 6 | 82176 | 3.50 |
| qr_96x32_f64/nalgebra | 57.677 / 66.084 | 58.396 / 58.679 | 4 | 57600 | 3.32 |
| qr_96x32_f64/numpy | 69.850 / 91.711 | 72.116 / 74.372 | unavailable | unavailable | 38.36 |
| softmax_256x64_f64/candle | 115.445 / 126.616 | 116.921 / 118.142 | 37 | 265200 | 4.80 |
| softmax_256x64_f64/kernix | 120.205 / 129.642 | 121.522 / 121.998 | 1 | 131072 | 3.58 |
| softmax_256x64_f64/ndarray | 111.358 / 124.212 | 112.731 / 116.352 | 1 | 131072 | 3.66 |
| softmax_256x64_f64/numpy | 132.688 / 144.781 | 134.644 / 135.798 | unavailable | unavailable | 35.15 |
| tensordot_16x32x8_32x8x16_f64/kernix | 63.479 / 71.734 | 64.008 / 64.315 | 9 | 2166 | 3.27 |
| tensordot_16x32x8_32x8x16_f64/numpy | 11.261 / 11.651 | 11.446 / 11.921 | unavailable | unavailable | 36.92 |
| training_backward_2x4x8x4_f64/candle | 20.879 / 21.620 | 20.960 / 21.404 | 472 | 8332 | 4.98 |
| training_backward_2x4x8x4_f64/kernix | 5.139 / 5.370 | 5.238 / 5.311 | 23 | 4224 | 3.80 |
| training_forward_2x4x8x4_f64/candle | 4.979 / 5.200 | 5.063 / 5.450 | 126 | 3912 | 4.71 |
| training_forward_2x4x8x4_f64/kernix | 3.236 / 3.306 | 3.273 / 3.311 | 17 | 1032 | 3.79 |
| training_record_2x4x8x4_f64/candle | 6.202 / 6.752 | 6.299 / 6.390 | 149 | 10712 | 4.73 |
| training_record_2x4x8x4_f64/kernix | 4.147 / 4.228 | 4.236 / 4.265 | 22 | 6832 | 3.77 |
| training_sgd_2x4x8x4_f64/kernix | 0.801 / 0.802 | 0.810 / 0.818 | 5 | 688 | 3.96 |
| training_shared_backward_64x32_f64/candle | 4.869 / 4.969 | 4.893 / 4.950 | 58 | 100004 | 4.29 |
| training_shared_backward_64x32_f64/kernix | 10.219 / 10.290 | 10.353 / 15.699 | 8 | 49768 | 3.60 |

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
