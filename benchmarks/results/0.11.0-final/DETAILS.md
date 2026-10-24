# Clean CPU evidence: 0.11.0-apple-policy

Measured source `18750693908110110d85d02d0f11af8d58bfae8b`, tree `75403e48db8556f95e014ec2c5b16b30f9fa781e`; dirty before/after: false.
Library source: workspace 18750693908110110d85d02d0f11af8d58bfae8b. CPU affinity: 0. Dtype explicitly named per workload; row-major inputs.

| Workload / library | Per-call p50 / p95 (us) | Batch-mean p50 / p95 (us) | Rust allocations | Peak additional requested bytes | Process peak RSS (MiB) |
|---|---:|---:|---:|---:|---:|
| cholesky_factor_128_f64/faer | 47.544 / 58.402 | 43.130 / 49.857 | 2 | 132096 | 4.79 |
| cholesky_factor_128_f64/kernix | 128.368 / 156.109 | 130.674 / 151.192 | 1 | 131072 | 4.21 |
| cholesky_factor_128_f64/nalgebra | 111.377 / 131.867 | 114.126 / 121.070 | 1 | 131072 | 4.09 |
| cholesky_factor_128_f64/numpy | 77.201 / 86.589 | 79.482 / 93.997 | unavailable | unavailable | 36.89 |
| cholesky_factor_256_f64/faer | 227.006 / 307.354 | 231.818 / 256.776 | 2 | 526336 | 5.40 |
| cholesky_factor_256_f64/kernix | 579.500 / 706.029 | 605.261 / 785.119 | 49 | 645120 | 5.69 |
| cholesky_factor_256_f64/nalgebra | 774.853 / 961.316 | 818.943 / 870.852 | 1 | 524288 | 4.72 |
| cholesky_factor_256_f64/numpy | 605.474 / 921.844 | 662.464 / 845.589 | unavailable | unavailable | 40.94 |
| cholesky_factor_512_f64/faer | 2059.378 / 4044.618 | 2296.836 / 3518.216 | 2 | 2101248 | 8.55 |
| cholesky_factor_512_f64/kernix | 4062.093 / 6471.442 | 4371.989 / 4786.650 | 161 | 2269184 | 10.43 |
| cholesky_factor_512_f64/nalgebra | 8337.880 / 11804.765 | 8122.778 / 9799.542 | 1 | 2097152 | 7.81 |
| cholesky_factor_512_f64/numpy | 8994.431 / 13343.682 | 8311.565 / 19241.559 | unavailable | unavailable | 52.74 |
| cholesky_factor_64_f64/faer | 9.262 / 9.328 | 9.690 / 12.723 | 2 | 33280 | 4.12 |
| cholesky_factor_64_f64/kernix | 23.116 / 40.509 | 23.126 / 23.609 | 2 | 33280 | 3.97 |
| cholesky_factor_64_f64/nalgebra | 15.500 / 17.520 | 16.416 / 34.558 | 1 | 32768 | 3.85 |
| cholesky_factor_64_f64/numpy | 11.825 / 20.810 | 12.821 / 13.347 | unavailable | unavailable | 36.11 |
| create_ones_64x64_f32/candle | 0.250 / 0.252 | 0.263 / 0.277 | 7 | 16696 | 4.02 |
| create_ones_64x64_f32/faer | 0.207 / 0.209 | 0.197 / 0.211 | 1 | 16384 | 4.10 |
| create_ones_64x64_f32/kernix | 0.205 / 0.206 | 0.216 / 0.240 | 1 | 16384 | 3.73 |
| create_ones_64x64_f32/ndarray | 0.169 / 0.170 | 0.163 / 0.189 | 1 | 16384 | 3.95 |
| create_ones_64x64_f32/numpy | 1.399 / 1.499 | 1.514 / 1.562 | unavailable | unavailable | 35.16 |
| create_zeros_64x64_f32/candle | 0.168 / 0.170 | 0.173 / 0.178 | 5 | 16648 | 4.25 |
| create_zeros_64x64_f32/faer | 0.175 / 0.248 | 0.214 / 0.222 | 1 | 16384 | 4.21 |
| create_zeros_64x64_f32/kernix | 0.155 / 0.157 | 0.164 / 0.171 | 1 | 16384 | 3.95 |
| create_zeros_64x64_f32/ndarray | 0.195 / 0.428 | 0.104 / 0.119 | 1 | 16384 | 3.64 |
| create_zeros_64x64_f32/numpy | 0.445 / 0.643 | 0.517 / 1.075 | unavailable | unavailable | 35.18 |
| matmul_128_f32/candle | 41.717 / 50.401 | 42.758 / 46.754 | 9 | 65816 | 4.96 |
| matmul_128_f32/faer | 47.184 / 52.320 | 48.693 / 75.364 | 1 | 65536 | 4.46 |
| matmul_128_f32/kernix | 46.010 / 58.464 | 47.439 / 60.883 | 3 | 65544 | 4.70 |
| matmul_128_f32/nalgebra | 54.141 / 72.657 | 58.339 / 66.188 | 2 | 163840 | 4.11 |
| matmul_128_f32/ndarray | 54.284 / 67.556 | 58.487 / 63.263 | 2 | 163840 | 4.22 |
| matmul_128_f32/numpy | 41.294 / 47.865 | 44.648 / 60.604 | unavailable | unavailable | 36.82 |
| matmul_128_f64/candle | 91.219 / 99.933 | 91.987 / 150.627 | 9 | 131352 | 5.39 |
| matmul_128_f64/faer | 91.301 / 119.200 | 91.754 / 114.635 | 1 | 131072 | 4.53 |
| matmul_128_f64/kernix | 81.483 / 89.070 | 83.625 / 87.520 | 3 | 131080 | 4.85 |
| matmul_128_f64/nalgebra | 126.333 / 146.330 | 133.175 / 147.320 | 2 | 327680 | 4.21 |
| matmul_128_f64/ndarray | 133.454 / 166.997 | 133.534 / 146.424 | 2 | 327680 | 4.36 |
| matmul_128_f64/numpy | 94.252 / 125.817 | 99.782 / 128.973 | unavailable | unavailable | 36.93 |
| matmul_256_f32/candle | 311.220 / 333.384 | 317.764 / 338.708 | 9 | 262424 | 5.80 |
| matmul_256_f32/faer | 358.189 / 378.719 | 363.223 / 379.630 | 1 | 262144 | 4.68 |
| matmul_256_f32/kernix | 346.674 / 536.005 | 427.501 / 1270.503 | 3 | 262152 | 5.28 |
| matmul_256_f32/nalgebra | 446.141 / 560.520 | 464.930 / 486.168 | 2 | 589824 | 4.77 |
| matmul_256_f32/ndarray | 453.024 / 618.538 | 445.147 / 477.904 | 2 | 589824 | 4.80 |
| matmul_256_f32/numpy | 333.271 / 387.669 | 342.191 / 358.189 | unavailable | unavailable | 40.88 |
| matmul_256_f64/candle | 644.914 / 806.620 | 655.163 / 864.672 | 9 | 524568 | 6.80 |
| matmul_256_f64/faer | 834.945 / 1014.912 | 854.928 / 926.079 | 1 | 524288 | 5.57 |
| matmul_256_f64/kernix | 651.551 / 796.702 | 664.364 / 786.744 | 3 | 524296 | 6.22 |
| matmul_256_f64/nalgebra | 730.698 / 890.507 | 909.892 / 2227.139 | 2 | 1179648 | 5.46 |
| matmul_256_f64/ndarray | 745.908 / 1016.314 | 767.711 / 953.862 | 2 | 1179648 | 6.05 |
| matmul_256_f64/numpy | 686.909 / 1261.149 | 724.787 / 796.704 | unavailable | unavailable | 41.81 |
| matmul_32_f64/candle | 1.752 / 1.780 | 1.739 / 1.936 | 9 | 8592 | 4.82 |
| matmul_32_f64/faer | 1.646 / 2.795 | 1.838 / 1.915 | 1 | 8192 | 4.12 |
| matmul_32_f64/kernix | 1.822 / 1.980 | 1.881 / 1.986 | 3 | 24576 | 4.20 |
| matmul_32_f64/nalgebra | 1.693 / 1.729 | 1.826 / 3.407 | 2 | 24576 | 3.98 |
| matmul_32_f64/ndarray | 1.610 / 1.668 | 1.664 / 1.737 | 2 | 24576 | 4.05 |
| matmul_32_f64/numpy | 4.777 / 7.200 | 2.946 / 4.234 | unavailable | unavailable | 35.74 |
| matmul_64_f32/candle | 5.459 / 5.871 | 5.709 / 6.169 | 9 | 16664 | 5.14 |
| matmul_64_f32/faer | 5.852 / 5.929 | 6.287 / 7.147 | 1 | 16384 | 4.22 |
| matmul_64_f32/kernix | 5.239 / 5.408 | 5.587 / 6.674 | 3 | 16544 | 4.19 |
| matmul_64_f32/nalgebra | 6.643 / 7.129 | 6.981 / 7.434 | 2 | 49152 | 4.01 |
| matmul_64_f32/ndarray | 7.002 / 8.728 | 7.761 / 11.266 | 2 | 49152 | 3.77 |
| matmul_64_f32/numpy | 6.689 / 8.012 | 10.390 / 11.219 | unavailable | unavailable | 35.91 |
| matmul_64_f64/candle | 11.060 / 12.639 | 11.632 / 16.287 | 9 | 33048 | 4.99 |
| matmul_64_f64/faer | 12.064 / 13.196 | 12.871 / 13.306 | 1 | 32768 | 4.25 |
| matmul_64_f64/kernix | 10.685 / 11.412 | 11.311 / 13.617 | 3 | 32928 | 4.46 |
| matmul_64_f64/nalgebra | 11.254 / 12.510 | 12.307 / 12.907 | 2 | 98304 | 4.07 |
| matmul_64_f64/ndarray | 12.993 / 14.977 | 13.402 / 15.636 | 2 | 98304 | 3.96 |
| matmul_64_f64/numpy | 12.896 / 15.886 | 13.734 / 14.982 | unavailable | unavailable | 36.07 |
| matmul_ab_128_f32/kernix | 40.851 / 45.880 | 41.822 / 43.949 | 3 | 65544 | 4.61 |
| matmul_ab_128_f32/numpy | 41.398 / 52.118 | 43.913 / 45.824 | unavailable | unavailable | 36.60 |
| matmul_ab_128_f64/kernix | 79.522 / 101.676 | 83.005 / 152.401 | 3 | 131080 | 4.89 |
| matmul_ab_128_f64/numpy | 82.316 / 90.222 | 89.668 / 157.506 | unavailable | unavailable | 36.98 |
| matmul_batched_4x16x32x16_f64/candle | 2.293 / 3.033 | 2.403 / 2.471 | 25 | 25504 | 5.45 |
| matmul_batched_4x16x32x16_f64/kernix | 2.279 / 3.220 | 2.418 / 2.732 | 7 | 16392 | 3.86 |
| matmul_batched_4x16x32x16_f64/numpy | 3.410 / 3.547 | 3.499 / 4.033 | unavailable | unavailable | 35.80 |
| matmul_sum_backward_32x16x8_f64/candle | 4.459 / 4.980 | 4.823 / 5.273 | 113 | 27988 | 5.27 |
| matmul_sum_backward_32x16x8_f64/kernix | 2.753 / 3.330 | 2.908 / 3.486 | 16 | 18376 | 4.16 |
| min_norm_32x64_rank32_f64/kernix | 66.701 / 75.641 | 67.418 / 69.615 | 20 | 59136 | 4.45 |
| min_norm_32x64_rank32_f64/nalgebra | 152.598 / 202.442 | 156.500 / 175.357 | 15 | 42232 | 4.31 |
| min_norm_32x64_rank32_f64/numpy | 146.425 / 227.044 | 151.645 / 163.315 | unavailable | unavailable | 36.70 |
| min_norm_64x32_rank16_f64/kernix | 54.399 / 64.576 | 53.864 / 57.325 | 20 | 74752 | 4.46 |
| min_norm_64x32_rank16_f64/nalgebra | 81.705 / 89.662 | 83.892 / 92.966 | 15 | 41976 | 4.16 |
| min_norm_64x32_rank16_f64/numpy | 74.806 / 99.926 | 74.732 / 77.974 | unavailable | unavailable | 36.71 |
| parallel_matmul_into_128_f32/kernix | 41.048 / 57.737 | 28.672 / 64.946 | 2 | 16 | 5.05 |
| parallel_matmul_into_128_f64/kernix | 79.016 / 442.025 | 90.262 / 143.112 | 2 | 16 | 5.47 |
| parallel_matmul_into_256_f64/kernix | 343.076 / 787.306 | 391.437 / 1340.276 | 2 | 16 | 6.99 |
| parallel_matmul_into_64_f64/kernix | 11.276 / 15.425 | 13.233 / 31.067 | 2 | 98304 | 4.64 |
| pivoted_qr_256x64_f64/faer | 514.231 / 570.510 | 609.009 / 945.469 | 10 | 333312 | 4.70 |
| pivoted_qr_256x64_f64/kernix | 602.805 / 735.441 | 632.818 / 930.942 | 14 | 559104 | 4.84 |
| pivoted_qr_256x64_f64/nalgebra | 856.584 / 979.225 | 865.921 / 898.084 | 7 | 296960 | 4.65 |
| pivoted_qr_96x32_f64/faer | 81.252 / 88.542 | 82.826 / 93.284 | 10 | 68352 | 4.47 |
| pivoted_qr_96x32_f64/kernix | 61.850 / 73.449 | 64.591 / 78.366 | 14 | 107520 | 4.31 |
| pivoted_qr_96x32_f64/nalgebra | 84.895 / 93.042 | 84.817 / 92.317 | 7 | 58368 | 4.04 |
| qr_256x64_f64/faer | 508.628 / 1349.545 | 520.965 / 737.802 | 7 | 331776 | 4.86 |
| qr_256x64_f64/kernix | 588.471 / 1128.368 | 667.259 / 956.451 | 6 | 426496 | 4.61 |
| qr_256x64_f64/nalgebra | 434.514 / 595.765 | 445.328 / 527.401 | 4 | 295424 | 4.31 |
| qr_256x64_f64/numpy | 430.596 / 642.755 | 435.666 / 591.342 | unavailable | unavailable | 37.59 |
| qr_96x32_f64/faer | 75.509 / 89.856 | 75.454 / 85.197 | 7 | 67584 | 4.61 |
| qr_96x32_f64/kernix | 53.055 / 59.323 | 53.142 / 55.460 | 6 | 82176 | 4.21 |
| qr_96x32_f64/nalgebra | 43.968 / 60.725 | 43.654 / 48.340 | 4 | 57600 | 3.89 |
| qr_96x32_f64/numpy | 45.085 / 51.615 | 46.647 / 47.246 | unavailable | unavailable | 36.76 |
| softmax_256x64_f64/candle | 97.887 / 110.363 | 97.429 / 102.681 | 37 | 265200 | 5.74 |
| softmax_256x64_f64/kernix | 100.743 / 123.231 | 103.263 / 191.328 | 1 | 131072 | 4.41 |
| softmax_256x64_f64/ndarray | 78.637 / 102.883 | 85.622 / 118.412 | 1 | 131072 | 4.38 |
| softmax_256x64_f64/numpy | 117.684 / 166.984 | 129.998 / 172.665 | unavailable | unavailable | 35.72 |
| tensordot_16x32x8_32x8x16_f64/kernix | 41.049 / 58.821 | 45.145 / 48.312 | 9 | 2166 | 4.07 |
| tensordot_16x32x8_32x8x16_f64/numpy | 8.467 / 9.249 | 9.052 / 10.230 | unavailable | unavailable | 35.67 |
| training_backward_2x4x8x4_f64/candle | 21.894 / 73.035 | 18.775 / 25.021 | 472 | 8332 | 5.85 |
| training_backward_2x4x8x4_f64/kernix | 3.515 / 4.647 | 3.812 / 4.347 | 23 | 4224 | 4.65 |
| training_forward_2x4x8x4_f64/candle | 4.733 / 6.001 | 4.044 / 4.362 | 126 | 3912 | 6.07 |
| training_forward_2x4x8x4_f64/kernix | 4.036 / 11.077 | 2.538 / 3.221 | 17 | 1032 | 4.57 |
| training_record_2x4x8x4_f64/candle | 8.494 / 10.533 | 6.976 / 10.385 | 149 | 10712 | 5.06 |
| training_record_2x4x8x4_f64/kernix | 2.840 / 3.627 | 3.030 / 3.176 | 22 | 6832 | 4.62 |
| training_sgd_2x4x8x4_f64/kernix | 0.584 / 0.811 | 0.617 / 0.724 | 5 | 688 | 4.74 |
| training_shared_backward_64x32_f64/candle | 4.717 / 34.867 | 5.712 / 11.687 | 58 | 100004 | 4.50 |
| training_shared_backward_64x32_f64/kernix | 6.946 / 7.536 | 6.908 / 10.883 | 8 | 49768 | 4.48 |

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
