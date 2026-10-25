# Clean CPU evidence: runner-ubuntu-latest-40161751367

Measured source `450df128c1ffb55a9a861230edb6b9a5ac3ef5b4`, tree `957542f0c3a8c779782e0b662134bab34e1572a9`; dirty before/after: false.
Library source: workspace 450df128c1ffb55a9a861230edb6b9a5ac3ef5b4. CPU affinity: 0. Dtype explicitly named per workload; row-major inputs.

| Workload / library | Per-call p50 / p95 (us) | Batch-mean p50 / p95 (us) | Rust allocations | Peak additional requested bytes | Process peak RSS (MiB) |
|---|---:|---:|---:|---:|---:|
| cholesky_factor_128_f64/faer | 59.321 / 69.300 | 60.102 / 60.553 | 2 | 132096 | 4.36 |
| cholesky_factor_128_f64/kernix | 170.820 / 180.728 | 172.721 / 174.330 | 1 | 131072 | 3.94 |
| cholesky_factor_128_f64/nalgebra | 129.342 / 140.312 | 117.600 / 152.029 | 1 | 131072 | 3.80 |
| cholesky_factor_128_f64/numpy | 107.972 / 122.549 | 109.773 / 111.127 | unavailable | unavailable | 45.89 |
| cholesky_factor_256_f64/faer | 312.645 / 330.528 | 318.009 / 320.472 | 2 | 526336 | 5.39 |
| cholesky_factor_256_f64/kernix | 690.612 / 708.456 | 691.664 / 699.407 | 49 | 645120 | 5.50 |
| cholesky_factor_256_f64/nalgebra | 895.356 / 917.828 | 797.016 / 813.219 | 1 | 524288 | 4.52 |
| cholesky_factor_256_f64/numpy | 1119.364 / 1148.319 | 1117.844 / 1142.953 | unavailable | unavailable | 49.43 |
| cholesky_factor_512_f64/faer | 2072.588 / 2106.101 | 2075.203 / 2101.112 | 2 | 2101248 | 8.71 |
| cholesky_factor_512_f64/kernix | 3905.588 / 3994.114 | 3886.920 / 3974.357 | 161 | 2269184 | 10.09 |
| cholesky_factor_512_f64/nalgebra | 7720.137 / 8079.469 | 7335.537 / 7644.234 | 1 | 2097152 | 7.50 |
| cholesky_factor_512_f64/numpy | 6344.763 / 6461.441 | 6337.830 / 6404.449 | unavailable | unavailable | 60.00 |
| cholesky_factor_64_f64/faer | 14.557 / 14.647 | 14.687 / 14.777 | 2 | 33280 | 3.96 |
| cholesky_factor_64_f64/kernix | 24.796 / 25.067 | 25.045 / 25.251 | 2 | 33280 | 3.77 |
| cholesky_factor_64_f64/nalgebra | 22.282 / 22.643 | 20.602 / 20.816 | 1 | 32768 | 3.61 |
| cholesky_factor_64_f64/numpy | 18.063 / 25.136 | 18.365 / 19.087 | unavailable | unavailable | 45.16 |
| create_ones_64x64_f32/candle | 0.441 / 0.461 | 0.450 / 0.453 | 7 | 16696 | 4.28 |
| create_ones_64x64_f32/faer | 0.391 / 0.391 | 0.379 / 0.385 | 1 | 16384 | 4.00 |
| create_ones_64x64_f32/kernix | 0.431 / 0.441 | 0.438 / 0.440 | 1 | 16384 | 3.52 |
| create_ones_64x64_f32/ndarray | 0.351 / 0.360 | 0.357 / 0.359 | 1 | 16384 | 3.60 |
| create_ones_64x64_f32/numpy | 2.064 / 2.144 | 2.148 / 2.348 | unavailable | unavailable | 41.59 |
| create_zeros_64x64_f32/candle | 0.280 / 0.281 | 0.279 / 0.286 | 5 | 16648 | 4.05 |
| create_zeros_64x64_f32/faer | 0.311 / 0.320 | 0.289 / 0.292 | 1 | 16384 | 3.91 |
| create_zeros_64x64_f32/kernix | 0.290 / 0.301 | 0.294 / 0.297 | 1 | 16384 | 3.75 |
| create_zeros_64x64_f32/ndarray | 0.200 / 0.201 | 0.205 / 0.208 | 1 | 16384 | 3.61 |
| create_zeros_64x64_f32/numpy | 0.711 / 0.771 | 0.812 / 1.019 | unavailable | unavailable | 41.57 |
| matmul_128_f32/candle | 45.815 / 59.802 | 46.331 / 47.481 | 9 | 65816 | 4.75 |
| matmul_128_f32/faer | 51.096 / 59.341 | 51.504 / 51.939 | 1 | 65536 | 4.34 |
| matmul_128_f32/kernix | 45.325 / 54.612 | 45.893 / 46.441 | 3 | 65544 | 4.48 |
| matmul_128_f32/nalgebra | 74.019 / 93.255 | 77.586 / 78.406 | 2 | 163840 | 3.97 |
| matmul_128_f32/ndarray | 73.899 / 94.637 | 77.477 / 78.021 | 2 | 163840 | 3.85 |
| matmul_128_f32/numpy | 49.012 / 59.322 | 49.769 / 50.471 | unavailable | unavailable | 44.02 |
| matmul_128_f64/candle | 92.883 / 103.964 | 94.167 / 95.350 | 9 | 131352 | 5.41 |
| matmul_128_f64/faer | 113.432 / 131.055 | 116.965 / 119.091 | 1 | 131072 | 4.37 |
| matmul_128_f64/kernix | 93.225 / 103.504 | 94.401 / 95.251 | 3 | 131080 | 4.71 |
| matmul_128_f64/nalgebra | 209.532 / 224.570 | 209.662 / 210.776 | 2 | 327680 | 3.84 |
| matmul_128_f64/ndarray | 211.866 / 232.716 | 212.935 / 216.304 | 2 | 327680 | 4.17 |
| matmul_128_f64/numpy | 100.959 / 113.933 | 102.724 / 104.838 | unavailable | unavailable | 44.21 |
| matmul_256_f32/candle | 353.602 / 371.535 | 357.757 / 360.048 | 9 | 262424 | 5.92 |
| matmul_256_f32/faer | 419.114 / 438.621 | 422.176 / 430.236 | 1 | 262144 | 4.79 |
| matmul_256_f32/kernix | 358.421 / 378.188 | 361.310 / 365.415 | 3 | 262152 | 4.66 |
| matmul_256_f32/nalgebra | 618.407 / 668.952 | 617.493 / 623.000 | 2 | 589824 | 4.26 |
| matmul_256_f32/ndarray | 610.403 / 629.528 | 610.348 / 616.459 | 2 | 589824 | 4.19 |
| matmul_256_f32/numpy | 360.454 / 380.182 | 365.747 / 369.008 | unavailable | unavailable | 47.73 |
| matmul_256_f64/candle | 738.682 / 758.640 | 737.823 / 746.425 | 9 | 524568 | 6.96 |
| matmul_256_f64/faer | 970.046 / 993.720 | 970.013 / 1021.804 | 1 | 524288 | 5.56 |
| matmul_256_f64/kernix | 742.720 / 759.712 | 742.619 / 746.843 | 3 | 524296 | 5.96 |
| matmul_256_f64/nalgebra | 902.158 / 920.333 | 903.815 / 905.100 | 2 | 1179648 | 5.31 |
| matmul_256_f64/ndarray | 910.995 / 921.344 | 912.604 / 917.491 | 2 | 1179648 | 5.70 |
| matmul_256_f64/numpy | 765.102 / 798.424 | 768.429 / 783.711 | unavailable | unavailable | 48.46 |
| matmul_32_f64/candle | 2.044 / 2.094 | 2.068 / 2.091 | 9 | 8472 | 4.23 |
| matmul_32_f64/faer | 2.144 / 2.175 | 2.045 / 2.060 | 1 | 8192 | 4.11 |
| matmul_32_f64/kernix | 2.425 / 2.464 | 2.465 / 2.476 | 3 | 24576 | 3.98 |
| matmul_32_f64/nalgebra | 2.124 / 2.194 | 2.161 / 2.172 | 2 | 24576 | 3.71 |
| matmul_32_f64/ndarray | 2.154 / 2.194 | 2.182 / 5.105 | 2 | 24576 | 3.70 |
| matmul_32_f64/numpy | 3.626 / 3.707 | 3.786 / 3.827 | unavailable | unavailable | 42.91 |
| matmul_64_f32/candle | 6.452 / 6.533 | 6.545 / 6.588 | 9 | 16664 | 4.95 |
| matmul_64_f32/faer | 6.512 / 6.592 | 6.599 / 6.638 | 1 | 16384 | 4.07 |
| matmul_64_f32/kernix | 6.282 / 6.361 | 6.405 / 6.458 | 3 | 16448 | 3.88 |
| matmul_64_f32/nalgebra | 7.454 / 9.257 | 7.532 / 7.708 | 2 | 49152 | 3.83 |
| matmul_64_f32/ndarray | 7.354 / 7.444 | 7.420 / 7.464 | 2 | 49152 | 3.76 |
| matmul_64_f32/numpy | 8.085 / 8.196 | 8.294 / 8.409 | unavailable | unavailable | 43.02 |
| matmul_64_f64/candle | 12.724 / 12.804 | 12.885 / 13.081 | 9 | 33048 | 5.02 |
| matmul_64_f64/faer | 13.625 / 13.746 | 13.821 / 14.027 | 1 | 32768 | 4.16 |
| matmul_64_f64/kernix | 12.513 / 12.644 | 12.653 / 12.741 | 3 | 32832 | 4.48 |
| matmul_64_f64/nalgebra | 15.179 / 15.359 | 15.421 / 15.479 | 2 | 98304 | 3.74 |
| matmul_64_f64/ndarray | 15.279 / 15.399 | 15.478 / 15.722 | 2 | 98304 | 3.83 |
| matmul_64_f64/numpy | 14.919 / 15.129 | 15.194 / 15.421 | unavailable | unavailable | 43.08 |
| matmul_ab_128_f32/kernix | 45.635 / 48.210 | 46.246 / 46.904 | 3 | 65544 | 4.39 |
| matmul_ab_128_f32/numpy | 49.052 / 59.812 | 50.120 / 53.215 | unavailable | unavailable | 43.95 |
| matmul_ab_128_f64/kernix | 94.016 / 104.826 | 95.792 / 107.429 | 3 | 131080 | 4.78 |
| matmul_ab_128_f64/numpy | 103.043 / 115.045 | 104.907 / 106.803 | unavailable | unavailable | 44.18 |
| matmul_batched_4x16x32x16_f32/candle | 2.044 / 2.104 | 2.075 / 2.117 | 25 | 13096 | 4.97 |
| matmul_batched_4x16x32x16_f32/kernix | 1.202 / 1.212 | 1.201 / 1.219 | 3 | 4104 | 3.96 |
| matmul_batched_4x16x32x16_f32/numpy | 3.317 / 3.396 | 3.464 / 3.506 | unavailable | unavailable | 42.89 |
| matmul_batched_4x16x32x16_f64/candle | 3.036 / 3.126 | 3.101 / 6.274 | 25 | 25384 | 5.36 |
| matmul_batched_4x16x32x16_f64/kernix | 1.994 / 2.044 | 2.046 / 2.090 | 3 | 8200 | 3.87 |
| matmul_batched_4x16x32x16_f64/numpy | 4.178 / 4.248 | 4.345 / 4.396 | unavailable | unavailable | 42.79 |
| matmul_sum_backward_32x16x8_f64/candle | 5.511 / 5.651 | 5.598 / 5.688 | 113 | 27988 | 5.56 |
| matmul_sum_backward_32x16x8_f64/kernix | 3.416 / 3.497 | 3.492 / 3.519 | 15 | 18376 | 4.03 |
| min_norm_32x64_rank32_f64/kernix | 84.268 / 93.886 | 85.265 / 86.865 | 20 | 59136 | 4.43 |
| min_norm_32x64_rank32_f64/nalgebra | 177.401 / 187.741 | 178.824 / 180.886 | 15 | 42232 | 4.03 |
| min_norm_32x64_rank32_f64/numpy | 182.752 / 203.691 | 187.540 / 191.392 | unavailable | unavailable | 47.26 |
| min_norm_64x32_rank16_f64/kernix | 63.749 / 72.586 | 65.233 / 65.623 | 20 | 74752 | 4.32 |
| min_norm_64x32_rank16_f64/nalgebra | 103.955 / 112.751 | 105.467 / 106.433 | 15 | 41976 | 4.07 |
| min_norm_64x32_rank16_f64/numpy | 103.593 / 123.452 | 106.356 / 107.775 | unavailable | unavailable | 46.22 |
| parallel_matmul_into_128_f32/kernix | 31.168 / 32.110 | 31.134 / 31.517 | 2 | 16 | 4.70 |
| parallel_matmul_into_128_f64/kernix | 56.275 / 67.697 | 56.944 / 58.111 | 2 | 16 | 4.93 |
| parallel_matmul_into_256_f64/kernix | 401.331 / 429.634 | 401.166 / 405.932 | 2 | 16 | 6.70 |
| parallel_matmul_into_64_f64/kernix | 15.689 / 16.210 | 16.085 / 16.150 | 2 | 98304 | 4.25 |
| pivoted_qr_256x64_f64/faer | 383.948 / 413.163 | 385.983 / 389.023 | 10 | 333312 | 4.81 |
| pivoted_qr_256x64_f64/kernix | 810.206 / 827.218 | 813.883 / 825.072 | 14 | 559104 | 4.79 |
| pivoted_qr_256x64_f64/nalgebra | 749.562 / 764.039 | 752.016 / 807.514 | 7 | 296960 | 4.15 |
| pivoted_qr_96x32_f64/faer | 42.590 / 43.441 | 43.125 / 43.293 | 10 | 68352 | 3.93 |
| pivoted_qr_96x32_f64/kernix | 78.938 / 88.546 | 79.785 / 80.582 | 14 | 107520 | 4.09 |
| pivoted_qr_96x32_f64/nalgebra | 78.347 / 86.752 | 79.111 / 79.532 | 7 | 58368 | 3.93 |
| qr_256x64_f64/faer | 423.843 / 460.612 | 429.518 / 435.766 | 7 | 331776 | 4.98 |
| qr_256x64_f64/kernix | 758.811 / 781.423 | 759.661 / 777.789 | 6 | 426496 | 4.39 |
| qr_256x64_f64/nalgebra | 566.359 / 592.569 | 569.832 / 574.576 | 4 | 295424 | 4.07 |
| qr_256x64_f64/numpy | 537.225 / 553.797 | 533.106 / 536.891 | unavailable | unavailable | 48.10 |
| qr_96x32_f64/faer | 55.474 / 64.721 | 56.208 / 56.965 | 7 | 67584 | 4.52 |
| qr_96x32_f64/kernix | 66.223 / 87.534 | 68.321 / 69.289 | 6 | 82176 | 3.90 |
| qr_96x32_f64/nalgebra | 58.550 / 67.245 | 59.238 / 59.893 | 4 | 57600 | 3.75 |
| qr_96x32_f64/numpy | 69.430 / 89.017 | 71.128 / 72.045 | unavailable | unavailable | 47.22 |
| softmax_256x64_f64/candle | 84.839 / 95.158 | 86.054 / 86.764 | 37 | 265200 | 5.75 |
| softmax_256x64_f64/kernix | 94.146 / 103.704 | 95.387 / 96.671 | 1 | 131072 | 4.27 |
| softmax_256x64_f64/ndarray | 80.751 / 89.758 | 81.989 / 82.478 | 1 | 131072 | 4.08 |
| softmax_256x64_f64/numpy | 119.814 / 132.468 | 121.310 / 123.495 | unavailable | unavailable | 42.09 |
| tensordot_16x32x8_32x8x16_f64/kernix | 63.519 / 71.734 | 64.091 / 64.657 | 9 | 2166 | 3.80 |
| tensordot_16x32x8_32x8x16_f64/numpy | 11.391 / 11.672 | 11.681 / 12.063 | unavailable | unavailable | 42.86 |
| training_backward_2x4x8x4_f64/candle | 16.661 / 17.122 | 16.873 / 23.172 | 472 | 8332 | 5.72 |
| training_backward_2x4x8x4_f64/kernix | 4.819 / 7.674 | 4.838 / 4.925 | 23 | 4224 | 4.18 |
| training_forward_2x4x8x4_f64/candle | 4.298 / 4.458 | 4.371 / 4.522 | 126 | 3912 | 5.71 |
| training_forward_2x4x8x4_f64/kernix | 2.966 / 3.026 | 3.003 / 3.041 | 17 | 1032 | 4.35 |
| training_record_2x4x8x4_f64/candle | 6.191 / 6.452 | 6.294 / 12.817 | 149 | 10712 | 5.77 |
| training_record_2x4x8x4_f64/kernix | 3.848 / 3.947 | 3.892 / 4.124 | 22 | 6832 | 4.08 |
| training_sgd_2x4x8x4_f64/kernix | 0.781 / 0.792 | 0.795 / 0.808 | 5 | 688 | 4.39 |
| training_shared_backward_64x32_f64/candle | 4.548 / 4.658 | 4.594 / 4.641 | 58 | 100004 | 5.24 |
| training_shared_backward_64x32_f64/kernix | 10.590 / 10.640 | 10.713 / 10.795 | 8 | 49768 | 4.09 |

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
