# Clean CPU evidence: 0.11.1-final

Measured source `b2c458d113aa23aa24e93fbfdabd55b55e3e6831`, tree `eb0af9dc230f1ae319802354a0b518909dbaad56`; dirty before/after: false.
Library source: workspace b2c458d113aa23aa24e93fbfdabd55b55e3e6831. CPU affinity: 0. Dtype explicitly named per workload; row-major inputs.

| Workload / library | Per-call p50 / p95 (us) | Batch-mean p50 / p95 (us) | Rust allocations | Peak additional requested bytes | Process peak RSS (MiB) |
|---|---:|---:|---:|---:|---:|
| cholesky_factor_128_f64/faer | 37.553 / 42.053 | 38.211 / 40.846 | 2 | 132096 | 4.92 |
| cholesky_factor_128_f64/kernix | 110.399 / 123.852 | 113.081 / 116.427 | 1 | 131072 | 4.44 |
| cholesky_factor_128_f64/nalgebra | 89.792 / 97.100 | 93.656 / 96.054 | 1 | 131072 | 4.31 |
| cholesky_factor_128_f64/numpy | 73.702 / 80.426 | 74.676 / 85.484 | unavailable | unavailable | 36.80 |
| cholesky_factor_256_f64/faer | 192.757 / 268.779 | 191.980 / 197.646 | 2 | 526336 | 5.63 |
| cholesky_factor_256_f64/kernix | 495.698 / 540.733 | 503.349 / 542.691 | 49 | 645120 | 6.00 |
| cholesky_factor_256_f64/nalgebra | 639.934 / 683.120 | 668.058 / 695.650 | 1 | 524288 | 4.81 |
| cholesky_factor_256_f64/numpy | 516.323 / 574.276 | 516.189 / 591.646 | unavailable | unavailable | 40.83 |
| cholesky_factor_512_f64/faer | 1355.642 / 1515.018 | 1368.704 / 1474.923 | 2 | 2101248 | 9.07 |
| cholesky_factor_512_f64/kernix | 2875.840 / 3451.100 | 2963.374 / 3511.464 | 161 | 2269184 | 10.59 |
| cholesky_factor_512_f64/nalgebra | 5261.263 / 6151.592 | 5306.662 / 5736.939 | 1 | 2097152 | 7.98 |
| cholesky_factor_512_f64/numpy | 3360.380 / 5198.354 | 4183.164 / 6270.412 | unavailable | unavailable | 52.66 |
| cholesky_factor_64_f64/faer | 8.439 / 8.576 | 8.141 / 8.911 | 2 | 33280 | 4.41 |
| cholesky_factor_64_f64/kernix | 18.925 / 20.977 | 20.089 / 21.798 | 2 | 33280 | 4.31 |
| cholesky_factor_64_f64/nalgebra | 13.118 / 14.240 | 13.978 / 14.712 | 1 | 32768 | 3.64 |
| cholesky_factor_64_f64/numpy | 10.374 / 11.569 | 10.601 / 10.882 | unavailable | unavailable | 36.10 |
| create_ones_64x64_f32/candle | 0.236 / 0.238 | 0.227 / 0.242 | 7 | 16696 | 4.13 |
| create_ones_64x64_f32/faer | 0.182 / 0.186 | 0.167 / 0.181 | 1 | 16384 | 4.09 |
| create_ones_64x64_f32/kernix | 0.212 / 0.217 | 0.208 / 0.223 | 1 | 16384 | 4.05 |
| create_ones_64x64_f32/ndarray | 0.140 / 0.141 | 0.144 / 0.153 | 1 | 16384 | 3.95 |
| create_ones_64x64_f32/numpy | 1.213 / 1.253 | 1.273 / 1.307 | unavailable | unavailable | 35.04 |
| create_zeros_64x64_f32/candle | 0.167 / 0.169 | 0.157 / 0.171 | 5 | 16648 | 4.41 |
| create_zeros_64x64_f32/faer | 0.144 / 0.153 | 0.175 / 0.180 | 1 | 16384 | 4.25 |
| create_zeros_64x64_f32/kernix | 0.141 / 0.142 | 0.156 / 0.170 | 1 | 16384 | 4.07 |
| create_zeros_64x64_f32/ndarray | 0.090 / 0.091 | 0.097 / 0.111 | 1 | 16384 | 3.87 |
| create_zeros_64x64_f32/numpy | 0.346 / 0.371 | 0.379 / 0.390 | unavailable | unavailable | 35.13 |
| matmul_128_f32/candle | 34.360 / 38.436 | 35.941 / 36.608 | 9 | 65816 | 5.25 |
| matmul_128_f32/faer | 39.162 / 44.550 | 40.052 / 40.830 | 1 | 65536 | 4.47 |
| matmul_128_f32/kernix | 34.201 / 36.849 | 36.230 / 38.903 | 3 | 65544 | 4.91 |
| matmul_128_f32/nalgebra | 49.338 / 56.221 | 51.253 / 54.300 | 2 | 163840 | 4.27 |
| matmul_128_f32/ndarray | 53.355 / 66.077 | 50.471 / 57.420 | 2 | 163840 | 4.32 |
| matmul_128_f32/numpy | 36.504 / 40.262 | 38.394 / 39.883 | unavailable | unavailable | 36.83 |
| matmul_128_f64/candle | 68.719 / 75.429 | 72.248 / 94.740 | 9 | 131352 | 5.48 |
| matmul_128_f64/faer | 76.996 / 80.682 | 77.465 / 78.030 | 1 | 131072 | 4.71 |
| matmul_128_f64/kernix | 70.637 / 80.177 | 71.631 / 73.384 | 3 | 131080 | 5.21 |
| matmul_128_f64/nalgebra | 110.851 / 129.748 | 113.252 / 118.276 | 2 | 327680 | 4.39 |
| matmul_128_f64/ndarray | 110.826 / 125.747 | 115.082 / 155.254 | 2 | 327680 | 4.52 |
| matmul_128_f64/numpy | 72.935 / 78.737 | 75.298 / 76.787 | unavailable | unavailable | 36.82 |
| matmul_256_f32/candle | 269.307 / 286.669 | 273.942 / 281.110 | 9 | 262424 | 5.97 |
| matmul_256_f32/faer | 302.741 / 316.057 | 304.249 / 308.995 | 1 | 262144 | 5.25 |
| matmul_256_f32/kernix | 278.325 / 304.824 | 275.725 / 292.161 | 3 | 262152 | 5.48 |
| matmul_256_f32/nalgebra | 387.295 / 440.142 | 396.488 / 420.606 | 2 | 589824 | 4.90 |
| matmul_256_f32/ndarray | 407.805 / 429.478 | 394.322 / 413.419 | 2 | 589824 | 5.13 |
| matmul_256_f32/numpy | 280.075 / 307.678 | 276.517 / 335.579 | unavailable | unavailable | 39.91 |
| matmul_256_f64/candle | 547.945 / 636.052 | 551.634 / 581.938 | 9 | 524568 | 6.86 |
| matmul_256_f64/faer | 717.663 / 741.372 | 719.064 / 730.985 | 1 | 524288 | 5.86 |
| matmul_256_f64/kernix | 573.171 / 997.326 | 571.311 / 611.845 | 3 | 524296 | 6.30 |
| matmul_256_f64/nalgebra | 617.727 / 682.017 | 639.955 / 715.078 | 2 | 1179648 | 5.69 |
| matmul_256_f64/ndarray | 618.725 / 682.882 | 639.438 / 701.159 | 2 | 1179648 | 6.19 |
| matmul_256_f64/numpy | 601.651 / 661.513 | 596.763 / 611.807 | unavailable | unavailable | 41.71 |
| matmul_32_f64/candle | 1.459 / 1.589 | 1.497 / 1.519 | 9 | 8592 | 5.21 |
| matmul_32_f64/faer | 1.569 / 1.611 | 1.526 / 1.659 | 1 | 8192 | 4.29 |
| matmul_32_f64/kernix | 1.641 / 1.676 | 1.630 / 1.728 | 3 | 24576 | 4.48 |
| matmul_32_f64/nalgebra | 1.625 / 1.774 | 1.607 / 1.633 | 2 | 24576 | 4.18 |
| matmul_32_f64/ndarray | 1.486 / 1.510 | 1.418 / 1.458 | 2 | 24576 | 4.20 |
| matmul_32_f64/numpy | 2.548 / 3.992 | 2.587 / 2.737 | unavailable | unavailable | 35.72 |
| matmul_64_f32/candle | 4.557 / 4.681 | 4.754 / 4.909 | 9 | 16664 | 5.00 |
| matmul_64_f32/faer | 5.358 / 5.468 | 5.122 / 5.913 | 1 | 16384 | 4.21 |
| matmul_64_f32/kernix | 4.525 / 4.637 | 4.584 / 4.779 | 3 | 16544 | 4.70 |
| matmul_64_f32/nalgebra | 6.065 / 6.203 | 5.861 / 6.447 | 2 | 49152 | 3.98 |
| matmul_64_f32/ndarray | 5.662 / 6.236 | 5.779 / 6.163 | 2 | 49152 | 4.23 |
| matmul_64_f32/numpy | 5.772 / 6.315 | 6.035 / 6.323 | unavailable | unavailable | 35.85 |
| matmul_64_f64/candle | 9.258 / 10.284 | 9.687 / 9.833 | 9 | 33048 | 4.65 |
| matmul_64_f64/faer | 10.277 / 11.463 | 11.046 / 11.221 | 1 | 32768 | 4.38 |
| matmul_64_f64/kernix | 9.146 / 10.106 | 9.612 / 9.823 | 3 | 32928 | 4.61 |
| matmul_64_f64/nalgebra | 11.253 / 12.078 | 10.803 / 11.383 | 2 | 98304 | 4.18 |
| matmul_64_f64/ndarray | 10.907 / 12.033 | 11.262 / 11.938 | 2 | 98304 | 4.36 |
| matmul_64_f64/numpy | 11.889 / 11.986 | 11.657 / 12.194 | unavailable | unavailable | 36.05 |
| matmul_ab_128_f32/kernix | 34.252 / 38.183 | 35.514 / 36.566 | 3 | 65544 | 4.94 |
| matmul_ab_128_f32/numpy | 39.283 / 42.839 | 39.686 / 41.625 | unavailable | unavailable | 36.61 |
| matmul_ab_128_f64/kernix | 69.083 / 84.968 | 70.862 / 75.229 | 3 | 131080 | 5.25 |
| matmul_ab_128_f64/numpy | 73.137 / 82.395 | 74.344 / 76.372 | unavailable | unavailable | 36.75 |
| matmul_batched_4x16x32x16_f32/candle | 1.377 / 1.429 | 1.437 / 1.475 | 25 | 13216 | 5.61 |
| matmul_batched_4x16x32x16_f32/kernix | 0.926 / 1.047 | 0.949 / 0.970 | 3 | 4104 | 4.28 |
| matmul_batched_4x16x32x16_f32/numpy | 2.031 / 2.064 | 2.168 / 2.241 | unavailable | unavailable | 35.61 |
| matmul_batched_4x16x32x16_f64/candle | 1.940 / 1.989 | 2.046 / 2.105 | 25 | 25504 | 5.82 |
| matmul_batched_4x16x32x16_f64/kernix | 1.406 / 1.428 | 1.813 / 2.430 | 3 | 8200 | 4.38 |
| matmul_batched_4x16x32x16_f64/numpy | 2.779 / 2.832 | 3.002 / 3.094 | unavailable | unavailable | 35.75 |
| matmul_sum_backward_32x16x8_f64/candle | 3.983 / 4.294 | 4.106 / 4.416 | 113 | 27988 | 5.88 |
| matmul_sum_backward_32x16x8_f64/kernix | 2.228 / 2.273 | 2.281 / 2.335 | 15 | 18376 | 4.50 |
| min_norm_32x64_rank32_f64/kernix | 56.680 / 63.939 | 55.632 / 58.978 | 20 | 59136 | 4.70 |
| min_norm_32x64_rank32_f64/nalgebra | 126.023 / 138.991 | 129.440 / 140.321 | 15 | 42232 | 4.34 |
| min_norm_32x64_rank32_f64/numpy | 114.961 / 128.022 | 118.699 / 121.285 | unavailable | unavailable | 36.75 |
| min_norm_64x32_rank16_f64/kernix | 44.321 / 48.209 | 45.495 / 47.125 | 20 | 74752 | 4.64 |
| min_norm_64x32_rank16_f64/nalgebra | 77.227 / 85.835 | 75.332 / 78.105 | 15 | 41976 | 4.39 |
| min_norm_64x32_rank16_f64/numpy | 66.814 / 116.318 | 69.519 / 78.584 | unavailable | unavailable | 36.68 |
| parallel_matmul_into_128_f32/kernix | 22.180 / 27.793 | 22.465 / 23.645 | 2 | 8 | 5.29 |
| parallel_matmul_into_128_f64/kernix | 41.479 / 47.855 | 41.720 / 42.722 | 2 | 16 | 5.61 |
| parallel_matmul_into_256_f64/kernix | 297.428 / 613.850 | 298.265 / 317.754 | 2 | 16 | 7.16 |
| parallel_matmul_into_64_f64/kernix | 9.501 / 9.833 | 9.881 / 10.626 | 2 | 98304 | 4.64 |
| pivoted_qr_256x64_f64/faer | 452.780 / 516.470 | 456.805 / 471.482 | 10 | 333312 | 4.96 |
| pivoted_qr_256x64_f64/kernix | 512.353 / 549.516 | 520.016 / 547.100 | 14 | 559104 | 5.05 |
| pivoted_qr_256x64_f64/nalgebra | 775.494 / 874.748 | 788.829 / 884.264 | 7 | 296960 | 4.70 |
| pivoted_qr_96x32_f64/faer | 71.704 / 81.394 | 73.504 / 76.396 | 10 | 68352 | 4.91 |
| pivoted_qr_96x32_f64/kernix | 56.430 / 64.712 | 53.946 / 57.428 | 14 | 107520 | 4.45 |
| pivoted_qr_96x32_f64/nalgebra | 76.393 / 84.817 | 75.537 / 81.694 | 7 | 58368 | 3.79 |
| qr_256x64_f64/faer | 394.945 / 426.415 | 400.269 / 425.912 | 7 | 331776 | 5.15 |
| qr_256x64_f64/kernix | 469.928 / 519.292 | 473.720 / 500.973 | 6 | 426496 | 4.86 |
| qr_256x64_f64/nalgebra | 374.716 / 417.288 | 380.933 / 406.400 | 4 | 295424 | 4.50 |
| qr_256x64_f64/numpy | 338.736 / 354.267 | 341.742 / 355.996 | unavailable | unavailable | 37.61 |
| qr_96x32_f64/faer | 62.485 / 67.259 | 64.185 / 67.606 | 7 | 67584 | 4.89 |
| qr_96x32_f64/kernix | 43.392 / 48.230 | 44.613 / 56.151 | 6 | 82176 | 4.53 |
| qr_96x32_f64/nalgebra | 34.387 / 38.992 | 36.289 / 39.515 | 4 | 57600 | 4.16 |
| qr_96x32_f64/numpy | 36.755 / 42.323 | 39.753 / 50.075 | unavailable | unavailable | 36.74 |
| softmax_256x64_f64/candle | 81.850 / 87.919 | 78.936 / 82.404 | 37 | 265200 | 5.86 |
| softmax_256x64_f64/kernix | 81.343 / 91.649 | 80.874 / 82.577 | 1 | 131072 | 4.61 |
| softmax_256x64_f64/ndarray | 68.484 / 74.382 | 70.111 / 82.459 | 1 | 131072 | 4.55 |
| softmax_256x64_f64/numpy | 97.519 / 103.920 | 101.570 / 115.413 | unavailable | unavailable | 35.66 |
| tensordot_16x32x8_32x8x16_f64/kernix | 36.005 / 39.171 | 34.575 / 36.077 | 9 | 2166 | 3.97 |
| tensordot_16x32x8_32x8x16_f64/numpy | 7.743 / 9.086 | 7.923 / 9.807 | unavailable | unavailable | 35.66 |
| training_backward_2x4x8x4_f64/candle | 14.581 / 15.990 | 14.045 / 15.963 | 472 | 8332 | 6.15 |
| training_backward_2x4x8x4_f64/kernix | 2.973 / 3.056 | 3.189 / 3.321 | 23 | 4224 | 4.66 |
| training_forward_2x4x8x4_f64/candle | 3.152 / 3.819 | 3.355 / 3.511 | 126 | 3912 | 5.91 |
| training_forward_2x4x8x4_f64/kernix | 2.151 / 2.206 | 2.202 / 2.227 | 17 | 1032 | 4.81 |
| training_record_2x4x8x4_f64/candle | 5.180 / 5.638 | 5.084 / 5.499 | 149 | 10712 | 6.04 |
| training_record_2x4x8x4_f64/kernix | 2.945 / 3.022 | 2.744 / 3.217 | 22 | 6832 | 4.60 |
| training_sgd_2x4x8x4_f64/kernix | 0.515 / 0.537 | 0.504 / 0.567 | 5 | 688 | 4.66 |
| training_shared_backward_64x32_f64/candle | 3.806 / 4.255 | 3.647 / 3.841 | 58 | 100004 | 5.48 |
| training_shared_backward_64x32_f64/kernix | 6.702 / 6.788 | 6.366 / 7.342 | 8 | 49768 | 4.62 |

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
