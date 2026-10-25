# Clean CPU evidence: runner-ubuntu-24.04-arm-40161751367

Measured source `450df128c1ffb55a9a861230edb6b9a5ac3ef5b4`, tree `957542f0c3a8c779782e0b662134bab34e1572a9`; dirty before/after: false.
Library source: workspace 450df128c1ffb55a9a861230edb6b9a5ac3ef5b4. CPU affinity: 0. Dtype explicitly named per workload; row-major inputs.

| Workload / library | Per-call p50 / p95 (us) | Batch-mean p50 / p95 (us) | Rust allocations | Peak additional requested bytes | Process peak RSS (MiB) |
|---|---:|---:|---:|---:|---:|
| cholesky_factor_128_f64/faer | 65.005 / 67.718 | 65.120 / 65.732 | 16 | 148736 | 3.32 |
| cholesky_factor_128_f64/kernix | 279.317 / 281.645 | 278.167 / 278.988 | 1 | 131072 | 2.95 |
| cholesky_factor_128_f64/nalgebra | 129.027 / 132.866 | 129.835 / 130.270 | 1 | 131072 | 2.82 |
| cholesky_factor_128_f64/numpy | 82.708 / 95.821 | 84.531 / 85.037 | unavailable | unavailable | 35.43 |
| cholesky_factor_256_f64/faer | 403.409 / 411.841 | 405.377 / 408.354 | 76 | 592000 | 4.23 |
| cholesky_factor_256_f64/kernix | 768.251 / 777.178 | 766.893 / 776.390 | 49 | 645120 | 4.32 |
| cholesky_factor_256_f64/nalgebra | 1035.177 / 1044.760 | 1034.966 / 1044.798 | 1 | 524288 | 3.56 |
| cholesky_factor_256_f64/numpy | 740.594 / 761.721 | 738.029 / 742.632 | unavailable | unavailable | 38.30 |
| cholesky_factor_512_f64/faer | 2645.050 / 2671.857 | 2653.068 / 2684.140 | 388 | 2297864 | 9.18 |
| cholesky_factor_512_f64/kernix | 4344.642 / 4378.449 | 4335.852 / 4389.270 | 161 | 2269184 | 8.91 |
| cholesky_factor_512_f64/nalgebra | 8552.152 / 8646.220 | 8379.725 / 8678.410 | 1 | 2097152 | 6.57 |
| cholesky_factor_512_f64/numpy | 3955.885 / 4023.067 | 3904.296 / 3939.129 | unavailable | unavailable | 47.80 |
| cholesky_factor_64_f64/faer | 13.239 / 13.288 | 13.307 / 13.461 | 2 | 33280 | 2.73 |
| cholesky_factor_64_f64/kernix | 27.623 / 28.278 | 27.521 / 27.567 | 2 | 33280 | 2.73 |
| cholesky_factor_64_f64/nalgebra | 20.328 / 21.047 | 20.534 / 20.646 | 1 | 32768 | 2.62 |
| cholesky_factor_64_f64/numpy | 18.055 / 19.095 | 18.247 / 18.413 | unavailable | unavailable | 34.67 |
| create_ones_64x64_f32/candle | 0.344 / 0.352 | 0.351 / 0.352 | 7 | 16696 | 3.29 |
| create_ones_64x64_f32/faer | 0.240 / 0.248 | 0.234 / 0.235 | 1 | 16384 | 2.73 |
| create_ones_64x64_f32/kernix | 0.256 / 0.272 | 0.264 / 0.264 | 1 | 16384 | 2.64 |
| create_ones_64x64_f32/ndarray | 0.208 / 0.216 | 0.216 / 0.217 | 1 | 16384 | 2.65 |
| create_ones_64x64_f32/numpy | 1.528 / 1.568 | 1.591 / 1.599 | unavailable | unavailable | 33.55 |
| create_zeros_64x64_f32/candle | 0.232 / 0.248 | 0.245 / 0.248 | 5 | 16648 | 3.10 |
| create_zeros_64x64_f32/faer | 0.320 / 0.328 | 0.312 / 0.316 | 1 | 16384 | 2.73 |
| create_zeros_64x64_f32/kernix | 0.200 / 0.216 | 0.215 / 0.220 | 1 | 16384 | 2.70 |
| create_zeros_64x64_f32/ndarray | 0.136 / 0.136 | 0.141 / 0.143 | 1 | 16384 | 2.64 |
| create_zeros_64x64_f32/numpy | 0.512 / 0.536 | 0.553 / 0.565 | unavailable | unavailable | 33.55 |
| matmul_128_f32/candle | 85.965 / 89.245 | 86.158 / 87.244 | 10 | 131160 | 3.71 |
| matmul_128_f32/faer | 86.934 / 88.958 | 87.199 / 87.264 | 3 | 196616 | 3.15 |
| matmul_128_f32/kernix | 85.421 / 87.253 | 85.615 / 85.791 | 4 | 131080 | 3.41 |
| matmul_128_f32/nalgebra | 93.892 / 102.132 | 95.582 / 96.350 | 2 | 163840 | 2.79 |
| matmul_128_f32/ndarray | 94.036 / 100.787 | 95.491 / 95.871 | 2 | 163840 | 2.84 |
| matmul_128_f32/numpy | 99.900 / 104.260 | 100.622 / 100.695 | unavailable | unavailable | 35.31 |
| matmul_128_f64/candle | 168.273 / 173.569 | 168.878 / 170.369 | 10 | 262232 | 4.05 |
| matmul_128_f64/faer | 168.731 / 174.971 | 169.205 / 169.314 | 3 | 393224 | 3.59 |
| matmul_128_f64/kernix | 168.033 / 174.457 | 168.529 / 168.752 | 4 | 262152 | 3.66 |
| matmul_128_f64/nalgebra | 209.064 / 218.047 | 208.603 / 211.171 | 2 | 327680 | 2.94 |
| matmul_128_f64/ndarray | 208.863 / 219.630 | 208.666 / 211.703 | 2 | 327680 | 3.07 |
| matmul_128_f64/numpy | 194.662 / 200.079 | 196.397 / 198.634 | unavailable | unavailable | 35.56 |
| matmul_256_f32/candle | 751.156 / 761.388 | 749.868 / 752.365 | 10 | 524376 | 4.55 |
| matmul_256_f32/faer | 673.781 / 679.685 | 673.879 / 678.368 | 3 | 786440 | 4.16 |
| matmul_256_f32/kernix | 661.830 / 665.614 | 661.666 / 667.627 | 4 | 524296 | 4.23 |
| matmul_256_f32/nalgebra | 756.525 / 771.261 | 755.107 / 756.582 | 2 | 589824 | 3.32 |
| matmul_256_f32/ndarray | 766.241 / 805.999 | 760.601 / 762.875 | 2 | 589824 | 3.58 |
| matmul_256_f32/numpy | 722.591 / 730.727 | 723.702 / 729.380 | unavailable | unavailable | 39.44 |
| matmul_256_f64/candle | 1321.246 / 1335.717 | 1319.310 / 1331.719 | 10 | 1048664 | 5.66 |
| matmul_256_f64/faer | 1536.267 / 1550.762 | 1531.458 / 1538.297 | 3 | 1048584 | 5.47 |
| matmul_256_f64/kernix | 1318.796 / 1330.875 | 1320.795 / 1324.570 | 4 | 1048584 | 5.23 |
| matmul_256_f64/nalgebra | 1327.137 / 1338.985 | 1327.823 / 1339.137 | 2 | 1179648 | 4.26 |
| matmul_256_f64/ndarray | 1322.459 / 1335.242 | 1321.623 / 1332.690 | 2 | 1179648 | 4.77 |
| matmul_256_f64/numpy | 1463.074 / 1481.841 | 1467.736 / 1475.090 | unavailable | unavailable | 39.94 |
| matmul_32_f64/candle | 3.448 / 3.480 | 3.460 / 3.473 | 10 | 16976 | 3.45 |
| matmul_32_f64/faer | 3.496 / 3.520 | 3.496 / 3.531 | 3 | 25088 | 3.03 |
| matmul_32_f64/kernix | 3.248 / 3.279 | 3.256 / 3.269 | 3 | 24576 | 2.79 |
| matmul_32_f64/nalgebra | 3.223 / 3.240 | 3.230 / 3.244 | 2 | 24576 | 2.66 |
| matmul_32_f64/ndarray | 3.048 / 3.056 | 3.055 / 3.071 | 2 | 24576 | 2.67 |
| matmul_32_f64/numpy | 8.223 / 8.287 | 8.311 / 8.340 | unavailable | unavailable | 34.22 |
| matmul_64_f32/candle | 11.943 / 11.968 | 11.975 / 12.040 | 10 | 33104 | 3.45 |
| matmul_64_f32/faer | 12.288 / 12.312 | 12.318 / 12.385 | 3 | 49408 | 3.00 |
| matmul_64_f32/kernix | 11.775 / 11.839 | 11.806 / 11.902 | 4 | 33024 | 3.13 |
| matmul_64_f32/nalgebra | 11.280 / 11.304 | 11.303 / 11.413 | 2 | 49152 | 2.63 |
| matmul_64_f32/ndarray | 11.255 / 11.280 | 11.277 / 11.368 | 2 | 49152 | 2.65 |
| matmul_64_f32/numpy | 20.103 / 20.256 | 20.254 / 20.383 | unavailable | unavailable | 34.45 |
| matmul_64_f64/candle | 22.504 / 22.567 | 22.542 / 22.638 | 10 | 65872 | 3.59 |
| matmul_64_f64/faer | 22.647 / 22.696 | 22.676 / 22.844 | 3 | 98560 | 3.10 |
| matmul_64_f64/kernix | 22.215 / 22.303 | 22.262 / 22.295 | 4 | 65792 | 3.19 |
| matmul_64_f64/nalgebra | 22.455 / 22.511 | 22.507 / 22.566 | 2 | 98304 | 2.76 |
| matmul_64_f64/ndarray | 21.807 / 21.847 | 21.865 / 22.081 | 2 | 98304 | 2.79 |
| matmul_64_f64/numpy | 31.055 / 31.431 | 31.217 / 31.520 | unavailable | unavailable | 34.52 |
| matmul_ab_128_f32/kernix | 85.629 / 88.092 | 85.728 / 85.973 | 4 | 131080 | 3.53 |
| matmul_ab_128_f32/numpy | 100.061 / 104.844 | 100.745 / 100.841 | unavailable | unavailable | 35.31 |
| matmul_ab_128_f64/kernix | 169.105 / 176.313 | 169.918 / 171.464 | 4 | 262152 | 3.79 |
| matmul_ab_128_f64/numpy | 195.439 / 201.543 | 196.528 / 197.284 | unavailable | unavailable | 35.54 |
| matmul_batched_4x16x32x16_f32/candle | 2.880 / 2.912 | 2.889 / 2.913 | 25 | 13408 | 3.64 |
| matmul_batched_4x16x32x16_f32/kernix | 2.400 / 2.424 | 2.415 / 2.436 | 7 | 8200 | 2.91 |
| matmul_batched_4x16x32x16_f32/numpy | 5.144 / 5.247 | 5.221 / 5.253 | unavailable | unavailable | 34.21 |
| matmul_batched_4x16x32x16_f64/candle | 4.432 / 4.480 | 4.461 / 4.506 | 25 | 25696 | 3.69 |
| matmul_batched_4x16x32x16_f64/kernix | 3.832 / 3.880 | 3.854 / 3.862 | 7 | 16392 | 2.88 |
| matmul_batched_4x16x32x16_f64/numpy | 12.991 / 13.103 | 13.098 / 13.161 | unavailable | unavailable | 34.23 |
| matmul_sum_backward_32x16x8_f64/candle | 7.159 / 7.423 | 7.225 / 7.270 | 113 | 27980 | 3.84 |
| matmul_sum_backward_32x16x8_f64/kernix | 3.504 / 3.575 | 3.504 / 3.549 | 16 | 18376 | 2.79 |
| min_norm_32x64_rank32_f64/kernix | 96.356 / 100.956 | 96.469 / 97.091 | 20 | 59136 | 3.05 |
| min_norm_32x64_rank32_f64/nalgebra | 219.320 / 221.847 | 219.903 / 220.573 | 15 | 42232 | 2.96 |
| min_norm_32x64_rank32_f64/numpy | 176.289 / 193.280 | 179.052 / 180.935 | unavailable | unavailable | 35.24 |
| min_norm_64x32_rank16_f64/kernix | 72.413 / 74.742 | 72.561 / 72.724 | 20 | 74752 | 3.05 |
| min_norm_64x32_rank16_f64/nalgebra | 116.948 / 120.620 | 117.305 / 117.861 | 15 | 41976 | 2.96 |
| min_norm_64x32_rank16_f64/numpy | 93.668 / 108.436 | 95.562 / 96.369 | unavailable | unavailable | 35.11 |
| parallel_matmul_into_128_f32/kernix | 48.670 / 51.062 | 48.663 / 49.535 | 4 | 65552 | 3.76 |
| parallel_matmul_into_128_f64/kernix | 92.156 / 99.691 | 92.712 / 94.819 | 4 | 131088 | 4.05 |
| parallel_matmul_into_256_f64/kernix | 698.289 / 709.704 | 702.148 / 707.093 | 4 | 524304 | 6.25 |
| parallel_matmul_into_64_f64/kernix | 16.575 / 18.584 | 16.608 / 18.356 | 2 | 98304 | 3.22 |
| pivoted_qr_256x64_f64/faer | 473.365 / 482.628 | 508.497 / 514.893 | 58 | 333312 | 3.95 |
| pivoted_qr_256x64_f64/kernix | 784.075 / 795.922 | 769.867 / 782.262 | 14 | 559104 | 3.48 |
| pivoted_qr_256x64_f64/nalgebra | 1149.984 / 1156.328 | 1149.722 / 1151.398 | 7 | 296960 | 3.16 |
| pivoted_qr_96x32_f64/faer | 62.262 / 64.526 | 62.404 / 62.736 | 32 | 68352 | 3.31 |
| pivoted_qr_96x32_f64/kernix | 88.317 / 90.525 | 88.500 / 88.778 | 14 | 107520 | 2.93 |
| pivoted_qr_96x32_f64/nalgebra | 114.235 / 116.811 | 114.476 / 114.908 | 7 | 58368 | 2.73 |
| qr_256x64_f64/faer | 445.277 / 452.093 | 446.652 / 449.276 | 83 | 331776 | 3.84 |
| qr_256x64_f64/kernix | 697.636 / 709.283 | 698.707 / 709.835 | 6 | 426496 | 3.45 |
| qr_256x64_f64/nalgebra | 586.032 / 594.592 | 586.030 / 587.121 | 4 | 295424 | 3.10 |
| qr_256x64_f64/numpy | 572.585 / 586.040 | 567.752 / 573.336 | unavailable | unavailable | 35.92 |
| qr_96x32_f64/faer | 65.165 / 67.837 | 65.284 / 65.504 | 41 | 67584 | 3.41 |
| qr_96x32_f64/kernix | 71.838 / 73.886 | 71.938 / 72.149 | 6 | 82176 | 2.91 |
| qr_96x32_f64/nalgebra | 61.333 / 63.222 | 61.484 / 61.942 | 4 | 57600 | 2.79 |
| qr_96x32_f64/numpy | 66.414 / 80.077 | 67.728 / 68.199 | unavailable | unavailable | 35.20 |
| softmax_256x64_f64/candle | 110.651 / 113.867 | 110.857 / 110.968 | 37 | 265200 | 4.14 |
| softmax_256x64_f64/kernix | 113.124 / 116.532 | 113.388 / 113.507 | 1 | 131072 | 3.01 |
| softmax_256x64_f64/ndarray | 97.324 / 100.524 | 97.592 / 97.701 | 1 | 131072 | 2.95 |
| softmax_256x64_f64/numpy | 133.858 / 140.611 | 134.915 / 135.378 | unavailable | unavailable | 34.20 |
| tensordot_16x32x8_32x8x16_f64/kernix | 46.774 / 48.654 | 46.843 / 46.927 | 9 | 2166 | 2.82 |
| tensordot_16x32x8_32x8x16_f64/numpy | 16.575 / 17.287 | 16.775 / 16.848 | unavailable | unavailable | 34.10 |
| training_backward_2x4x8x4_f64/candle | 21.335 / 21.799 | 21.309 / 21.425 | 476 | 8996 | 4.28 |
| training_backward_2x4x8x4_f64/kernix | 4.184 / 4.327 | 4.221 / 4.250 | 23 | 4224 | 3.03 |
| training_forward_2x4x8x4_f64/candle | 5.368 / 5.544 | 5.354 / 5.376 | 126 | 3912 | 4.11 |
| training_forward_2x4x8x4_f64/kernix | 3.080 / 3.143 | 3.099 / 3.124 | 17 | 1032 | 3.01 |
| training_record_2x4x8x4_f64/candle | 7.672 / 7.855 | 7.512 / 7.674 | 149 | 10712 | 4.13 |
| training_record_2x4x8x4_f64/kernix | 3.856 / 3.928 | 3.889 / 3.910 | 22 | 6832 | 2.96 |
| training_sgd_2x4x8x4_f64/kernix | 0.736 / 0.768 | 0.746 / 0.750 | 5 | 688 | 3.14 |
| training_shared_backward_64x32_f64/candle | 4.976 / 5.032 | 4.955 / 5.023 | 58 | 99996 | 3.66 |
| training_shared_backward_64x32_f64/kernix | 8.352 / 8.399 | 8.380 / 8.434 | 8 | 49768 | 3.06 |

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
