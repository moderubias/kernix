# Clean CPU evidence: runner-ubuntu-24.04-arm-40000558153

Measured source `1c53d4663a48de5a588809c2b69c3a6a5bdad7b2`, tree `d5b67d509ef5c0b81991af0933b482b13552047e`; dirty before/after: false.
Library source: workspace 1c53d4663a48de5a588809c2b69c3a6a5bdad7b2. CPU affinity: 0. Dtype explicitly named per workload; row-major inputs.

| Workload / library | Per-call p50 / p95 (us) | Batch-mean p50 / p95 (us) | Rust allocations | Peak additional requested bytes | Process peak RSS (MiB) |
|---|---:|---:|---:|---:|---:|
| cholesky_factor_128_f64/faer | 65.426 / 67.362 | 65.427 / 66.929 | 16 | 148736 | 3.45 |
| cholesky_factor_128_f64/kernix | 277.953 / 281.696 | 275.657 / 279.155 | 1 | 131072 | 3.01 |
| cholesky_factor_128_f64/nalgebra | 128.828 / 133.501 | 129.868 / 130.683 | 1 | 131072 | 2.87 |
| cholesky_factor_128_f64/numpy | 85.762 / 98.307 | 87.334 / 87.939 | unavailable | unavailable | 35.31 |
| cholesky_factor_256_f64/faer | 404.125 / 415.085 | 406.362 / 407.751 | 76 | 592000 | 4.36 |
| cholesky_factor_256_f64/kernix | 1970.346 / 1995.643 | 1969.262 / 1981.949 | 1 | 524288 | 4.14 |
| cholesky_factor_256_f64/nalgebra | 1034.459 / 1042.803 | 1038.394 / 1046.413 | 1 | 524288 | 3.63 |
| cholesky_factor_256_f64/numpy | 756.351 / 772.680 | 752.943 / 756.723 | unavailable | unavailable | 38.18 |
| cholesky_factor_64_f64/faer | 13.344 / 13.392 | 13.375 / 13.418 | 2 | 33280 | 2.78 |
| cholesky_factor_64_f64/kernix | 27.337 / 28.121 | 27.266 / 27.474 | 2 | 33280 | 2.79 |
| cholesky_factor_64_f64/nalgebra | 20.321 / 21.009 | 20.525 / 20.722 | 1 | 32768 | 2.68 |
| cholesky_factor_64_f64/numpy | 17.792 / 18.888 | 18.115 / 18.212 | unavailable | unavailable | 34.56 |
| create_ones_64x64_f32/candle | 0.344 / 0.352 | 0.349 / 0.350 | 7 | 16696 | 3.11 |
| create_ones_64x64_f32/faer | 0.240 / 0.248 | 0.235 / 0.236 | 1 | 16384 | 2.66 |
| create_ones_64x64_f32/kernix | 0.248 / 0.272 | 0.258 / 0.259 | 1 | 16384 | 2.59 |
| create_ones_64x64_f32/ndarray | 0.208 / 0.224 | 0.216 / 0.216 | 1 | 16384 | 2.59 |
| create_ones_64x64_f32/numpy | 1.536 / 1.576 | 1.599 / 1.611 | unavailable | unavailable | 33.43 |
| create_zeros_64x64_f32/candle | 0.232 / 0.248 | 0.246 / 0.251 | 5 | 16648 | 2.91 |
| create_zeros_64x64_f32/faer | 0.320 / 0.328 | 0.310 / 0.313 | 1 | 16384 | 2.66 |
| create_zeros_64x64_f32/kernix | 0.200 / 0.224 | 0.221 / 0.226 | 1 | 16384 | 2.65 |
| create_zeros_64x64_f32/ndarray | 0.136 / 0.144 | 0.142 / 0.144 | 1 | 16384 | 2.59 |
| create_zeros_64x64_f32/numpy | 0.512 / 0.544 | 0.560 / 0.569 | unavailable | unavailable | 33.45 |
| matmul_128_f32/candle | 86.067 / 88.747 | 86.160 / 86.524 | 10 | 131160 | 3.71 |
| matmul_128_f32/faer | 87.011 / 89.219 | 87.215 / 88.099 | 3 | 196616 | 3.21 |
| matmul_128_f32/kernix | 94.356 / 104.107 | 96.269 / 96.688 | 3 | 163840 | 3.07 |
| matmul_128_f32/nalgebra | 93.755 / 103.219 | 95.783 / 96.278 | 2 | 163840 | 2.85 |
| matmul_128_f32/ndarray | 93.875 / 102.339 | 95.271 / 95.525 | 2 | 163840 | 2.91 |
| matmul_128_f32/numpy | 99.836 / 103.403 | 100.424 / 100.781 | unavailable | unavailable | 35.19 |
| matmul_128_f64/candle | 168.285 / 171.013 | 168.729 / 169.089 | 10 | 262232 | 3.93 |
| matmul_128_f64/faer | 169.742 / 177.798 | 170.328 / 170.580 | 3 | 393224 | 3.53 |
| matmul_128_f64/kernix | 208.887 / 223.088 | 209.675 / 210.506 | 3 | 327680 | 3.20 |
| matmul_128_f64/nalgebra | 208.679 / 222.423 | 209.258 / 210.031 | 2 | 327680 | 2.80 |
| matmul_128_f64/ndarray | 207.823 / 221.520 | 208.387 / 208.853 | 2 | 327680 | 2.95 |
| matmul_128_f64/numpy | 195.222 / 199.790 | 196.129 / 196.769 | unavailable | unavailable | 35.44 |
| matmul_256_f32/candle | 766.771 / 779.411 | 764.203 / 768.060 | 10 | 524376 | 4.49 |
| matmul_256_f32/faer | 675.088 / 681.608 | 673.767 / 682.661 | 3 | 786440 | 4.22 |
| matmul_256_f32/kernix | 773.531 / 786.812 | 771.002 / 773.284 | 3 | 589824 | 3.70 |
| matmul_256_f32/nalgebra | 763.616 / 775.216 | 761.667 / 763.669 | 2 | 589824 | 3.38 |
| matmul_256_f32/ndarray | 767.258 / 780.202 | 766.826 / 769.489 | 2 | 589824 | 3.58 |
| matmul_256_f32/numpy | 727.864 / 737.896 | 726.820 / 729.750 | unavailable | unavailable | 38.41 |
| matmul_256_f64/candle | 1328.690 / 1342.868 | 1331.315 / 1335.432 | 10 | 1048664 | 5.54 |
| matmul_256_f64/faer | 1559.577 / 1587.297 | 1551.924 / 1564.256 | 3 | 1048584 | 5.48 |
| matmul_256_f64/kernix | 1335.439 / 1348.536 | 1337.953 / 1342.740 | 3 | 1179648 | 4.88 |
| matmul_256_f64/nalgebra | 1342.730 / 1354.251 | 1344.369 / 1347.402 | 2 | 1179648 | 4.13 |
| matmul_256_f64/ndarray | 1328.439 / 1339.336 | 1327.349 / 1337.360 | 2 | 1179648 | 4.63 |
| matmul_256_f64/numpy | 1481.796 / 1498.005 | 1479.197 / 1483.973 | unavailable | unavailable | 38.85 |
| matmul_32_f64/candle | 3.432 / 3.456 | 3.443 / 3.468 | 10 | 16976 | 3.31 |
| matmul_32_f64/faer | 3.504 / 3.528 | 3.494 / 3.516 | 3 | 25088 | 2.96 |
| matmul_32_f64/kernix | 3.240 / 3.272 | 3.251 / 3.272 | 3 | 24576 | 2.80 |
| matmul_32_f64/nalgebra | 3.216 / 3.248 | 3.234 / 3.254 | 2 | 24576 | 2.54 |
| matmul_32_f64/ndarray | 3.056 / 3.088 | 3.061 / 3.068 | 2 | 24576 | 2.55 |
| matmul_32_f64/numpy | 8.224 / 8.264 | 8.306 / 8.323 | unavailable | unavailable | 34.11 |
| matmul_64_f32/candle | 11.961 / 12.008 | 11.999 / 12.051 | 10 | 33104 | 3.45 |
| matmul_64_f32/faer | 12.321 / 12.353 | 12.353 / 12.409 | 3 | 49408 | 3.05 |
| matmul_64_f32/kernix | 11.513 / 11.552 | 11.570 / 11.595 | 3 | 49152 | 2.84 |
| matmul_64_f32/nalgebra | 11.272 / 11.296 | 11.306 / 11.400 | 2 | 49152 | 2.70 |
| matmul_64_f32/ndarray | 11.257 / 11.288 | 11.300 / 11.366 | 2 | 49152 | 2.71 |
| matmul_64_f32/numpy | 20.120 / 20.592 | 20.304 / 20.361 | unavailable | unavailable | 34.34 |
| matmul_64_f64/candle | 22.616 / 22.689 | 22.448 / 22.740 | 10 | 65872 | 3.46 |
| matmul_64_f64/faer | 22.641 / 22.712 | 22.698 / 22.771 | 3 | 98560 | 3.05 |
| matmul_64_f64/kernix | 22.160 / 22.265 | 22.222 / 22.381 | 3 | 98304 | 2.95 |
| matmul_64_f64/nalgebra | 22.480 / 22.521 | 22.532 / 22.729 | 2 | 98304 | 2.62 |
| matmul_64_f64/ndarray | 21.808 / 21.889 | 21.849 / 21.951 | 2 | 98304 | 2.66 |
| matmul_64_f64/numpy | 31.065 / 32.177 | 31.289 / 31.378 | unavailable | unavailable | 34.39 |
| matmul_batched_4x16x32x16_f64/candle | 4.441 / 4.488 | 4.490 / 4.532 | 25 | 25696 | 3.61 |
| matmul_batched_4x16x32x16_f64/kernix | 3.840 / 3.880 | 3.861 / 3.872 | 7 | 16392 | 2.88 |
| matmul_batched_4x16x32x16_f64/numpy | 12.984 / 13.120 | 13.095 / 13.107 | unavailable | unavailable | 34.11 |
| matmul_sum_backward_32x16x8_f64/candle | 7.232 / 7.473 | 7.232 / 7.313 | 113 | 27980 | 3.84 |
| matmul_sum_backward_32x16x8_f64/kernix | 3.496 / 3.560 | 3.505 / 3.524 | 16 | 18376 | 2.90 |
| min_norm_32x64_rank32_f64/kernix | 96.523 / 98.931 | 96.749 / 97.627 | 20 | 59136 | 3.05 |
| min_norm_32x64_rank32_f64/nalgebra | 218.351 / 222.615 | 218.756 / 218.864 | 15 | 42232 | 2.89 |
| min_norm_32x64_rank32_f64/numpy | 176.404 / 194.212 | 179.331 / 180.150 | unavailable | unavailable | 35.10 |
| min_norm_64x32_rank16_f64/kernix | 72.306 / 75.002 | 72.498 / 73.864 | 20 | 74752 | 3.06 |
| min_norm_64x32_rank16_f64/nalgebra | 116.779 / 120.683 | 116.958 / 117.338 | 15 | 41976 | 2.89 |
| min_norm_64x32_rank16_f64/numpy | 93.731 / 106.868 | 95.626 / 96.190 | unavailable | unavailable | 34.99 |
| parallel_matmul_into_128_f32/kernix | 49.722 / 51.898 | 49.880 / 50.110 | 4 | 163840 | 3.48 |
| parallel_matmul_into_128_f64/kernix | 97.835 / 103.596 | 100.358 / 100.563 | 4 | 327680 | 3.83 |
| parallel_matmul_into_256_f64/kernix | 841.275 / 862.843 | 839.272 / 852.090 | 8 | 1179648 | 6.15 |
| parallel_matmul_into_64_f64/kernix | 16.832 / 17.233 | 16.906 / 16.968 | 2 | 98304 | 3.22 |
| pivoted_qr_256x64_f64/faer | 474.078 / 482.079 | 512.646 / 515.422 | 58 | 333312 | 4.07 |
| pivoted_qr_256x64_f64/kernix | 792.708 / 806.860 | 773.468 / 793.054 | 14 | 559104 | 3.48 |
| pivoted_qr_256x64_f64/nalgebra | 1150.531 / 1153.948 | 1149.736 / 1151.031 | 7 | 296960 | 3.23 |
| pivoted_qr_96x32_f64/faer | 62.130 / 64.547 | 62.285 / 64.409 | 32 | 68352 | 3.43 |
| pivoted_qr_96x32_f64/kernix | 88.260 / 90.932 | 88.462 / 88.562 | 14 | 107520 | 2.93 |
| pivoted_qr_96x32_f64/nalgebra | 114.211 / 116.596 | 114.470 / 114.563 | 7 | 58368 | 2.80 |
| qr_256x64_f64/faer | 447.087 / 453.672 | 448.344 / 449.165 | 83 | 331776 | 3.84 |
| qr_256x64_f64/kernix | 699.895 / 713.001 | 699.757 / 702.104 | 6 | 426496 | 3.45 |
| qr_256x64_f64/nalgebra | 590.322 / 604.499 | 591.620 / 593.605 | 4 | 295424 | 3.10 |
| qr_256x64_f64/numpy | 570.707 / 584.484 | 566.951 / 570.795 | unavailable | unavailable | 35.79 |
| qr_96x32_f64/faer | 65.330 / 67.906 | 65.452 / 65.557 | 41 | 67584 | 3.41 |
| qr_96x32_f64/kernix | 71.659 / 73.690 | 71.764 / 71.969 | 6 | 82176 | 2.91 |
| qr_96x32_f64/nalgebra | 61.347 / 63.258 | 61.495 / 61.539 | 4 | 57600 | 2.79 |
| qr_96x32_f64/numpy | 66.314 / 86.795 | 67.844 / 68.358 | unavailable | unavailable | 35.08 |
| softmax_256x64_f64/candle | 110.508 / 115.708 | 110.788 / 111.031 | 37 | 265200 | 4.14 |
| softmax_256x64_f64/kernix | 113.012 / 115.812 | 113.209 / 113.380 | 1 | 131072 | 3.07 |
| softmax_256x64_f64/ndarray | 97.315 / 99.947 | 97.549 / 97.755 | 1 | 131072 | 2.95 |
| softmax_256x64_f64/numpy | 134.508 / 144.325 | 136.086 / 136.460 | unavailable | unavailable | 34.09 |
| tensordot_16x32x8_32x8x16_f64/kernix | 46.698 / 48.506 | 46.828 / 46.876 | 9 | 2166 | 2.82 |
| tensordot_16x32x8_32x8x16_f64/numpy | 16.472 / 16.936 | 16.672 / 16.771 | unavailable | unavailable | 33.97 |
| training_backward_2x4x8x4_f64/candle | 21.400 / 21.816 | 21.384 / 21.455 | 476 | 8996 | 4.23 |
| training_backward_2x4x8x4_f64/kernix | 4.200 / 4.352 | 4.231 / 4.240 | 23 | 4224 | 3.09 |
| training_forward_2x4x8x4_f64/candle | 5.296 / 5.384 | 5.322 / 5.348 | 126 | 3912 | 4.10 |
| training_forward_2x4x8x4_f64/kernix | 3.024 / 3.096 | 3.134 / 3.140 | 17 | 1032 | 3.07 |
| training_record_2x4x8x4_f64/candle | 7.664 / 7.840 | 7.437 / 7.673 | 149 | 10712 | 4.07 |
| training_record_2x4x8x4_f64/kernix | 3.817 / 3.889 | 3.858 / 3.873 | 22 | 6832 | 3.02 |
| training_sgd_2x4x8x4_f64/kernix | 0.720 / 0.752 | 0.734 / 0.737 | 5 | 688 | 3.15 |
| training_shared_backward_64x32_f64/candle | 4.984 / 5.040 | 4.959 / 5.030 | 58 | 99996 | 3.66 |
| training_shared_backward_64x32_f64/kernix | 8.432 / 8.481 | 8.455 / 8.475 | 8 | 49768 | 3.04 |

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
