# Clean CPU evidence: baseline-macos-latest-40059566395

Measured source `18750693908110110d85d02d0f11af8d58bfae8b`, tree `75403e48db8556f95e014ec2c5b16b30f9fa781e`; dirty before/after: false.
Library source: registry 0.10.0. CPU affinity: not pinned. Dtype explicitly named per workload; row-major inputs.

| Workload / library | Per-call p50 / p95 (us) | Batch-mean p50 / p95 (us) | Rust allocations | Peak additional requested bytes | Process peak RSS (MiB) |
|---|---:|---:|---:|---:|---:|
| cholesky_factor_128_f64/faer | 50.042 / 67.125 | 58.954 / 93.247 | 16 | 149504 | unavailable |
| cholesky_factor_128_f64/kernix | 197.250 / 601.750 | 262.257 / 375.082 | 1 | 131072 | unavailable |
| cholesky_factor_128_f64/nalgebra | 99.083 / 109.208 | 99.780 / 104.192 | 1 | 131072 | unavailable |
| cholesky_factor_128_f64/numpy | 51.125 / 52.166 | 58.046 / 77.746 | unavailable | unavailable | unavailable |
| cholesky_factor_256_f64/faer | 266.584 / 306.958 | 262.400 / 301.479 | 76 | 592384 | unavailable |
| cholesky_factor_256_f64/kernix | 1116.750 / 1245.958 | 1202.019 / 1269.448 | 1 | 524288 | unavailable |
| cholesky_factor_256_f64/nalgebra | 696.042 / 798.875 | 748.214 / 855.014 | 1 | 524288 | unavailable |
| cholesky_factor_256_f64/numpy | 309.542 / 413.583 | 308.956 / 341.467 | unavailable | unavailable | unavailable |
| cholesky_factor_512_f64/faer | 1815.166 / 2259.917 | 1689.229 / 1859.417 | 388 | 2625544 | unavailable |
| cholesky_factor_512_f64/kernix | 9378.625 / 16598.833 | 9262.646 / 10042.333 | 1 | 2097152 | unavailable |
| cholesky_factor_512_f64/nalgebra | 5356.917 / 6204.917 | 5971.520 / 7531.729 | 1 | 2097152 | unavailable |
| cholesky_factor_512_f64/numpy | 2404.042 / 2732.000 | 2370.417 / 9627.694 | unavailable | unavailable | unavailable |
| cholesky_factor_64_f64/faer | 11.084 / 13.500 | 11.786 / 12.266 | 2 | 33280 | unavailable |
| cholesky_factor_64_f64/kernix | 20.167 / 20.333 | 22.086 / 25.875 | 2 | 33280 | unavailable |
| cholesky_factor_64_f64/nalgebra | 22.000 / 30.167 | 22.505 / 23.539 | 1 | 32768 | unavailable |
| cholesky_factor_64_f64/numpy | 16.084 / 16.708 | 16.209 / 24.999 | unavailable | unavailable | unavailable |
| create_ones_64x64_f32/candle | 0.417 / 0.417 | 0.377 / 0.432 | 7 | 16696 | unavailable |
| create_ones_64x64_f32/faer | 0.792 / 0.834 | 0.817 / 0.828 | 1 | 16384 | unavailable |
| create_ones_64x64_f32/kernix | 0.333 / 0.416 | 0.327 / 0.374 | 1 | 16384 | unavailable |
| create_ones_64x64_f32/ndarray | 0.333 / 0.416 | 0.272 / 0.307 | 1 | 16384 | unavailable |
| create_ones_64x64_f32/numpy | 1.084 / 1.375 | 1.131 / 1.231 | unavailable | unavailable | unavailable |
| create_zeros_64x64_f32/candle | 0.375 / 0.416 | 0.383 / 0.427 | 5 | 16648 | unavailable |
| create_zeros_64x64_f32/faer | 0.291 / 0.292 | 0.257 / 0.263 | 1 | 16384 | unavailable |
| create_zeros_64x64_f32/kernix | 0.292 / 0.333 | 0.289 / 0.308 | 1 | 16384 | unavailable |
| create_zeros_64x64_f32/ndarray | 0.250 / 0.667 | 0.282 / 0.347 | 1 | 16384 | unavailable |
| create_zeros_64x64_f32/numpy | 0.625 / 0.750 | 0.617 / 1.013 | unavailable | unavailable | unavailable |
| matmul_128_f32/candle | 65.208 / 80.250 | 67.840 / 110.230 | 10 | 327768 | unavailable |
| matmul_128_f32/faer | 55.458 / 65.667 | 55.730 / 61.993 | 3 | 393224 | unavailable |
| matmul_128_f32/kernix | 50.625 / 58.083 | 56.980 / 60.808 | 3 | 163840 | unavailable |
| matmul_128_f32/nalgebra | 49.625 / 51.958 | 49.421 / 50.300 | 2 | 163840 | unavailable |
| matmul_128_f32/ndarray | 51.459 / 61.375 | 53.654 / 63.923 | 2 | 163840 | unavailable |
| matmul_128_f32/numpy | 6.334 / 7.542 | 6.023 / 6.551 | unavailable | unavailable | unavailable |
| matmul_128_f64/candle | 98.208 / 115.333 | 108.463 / 116.091 | 10 | 655448 | unavailable |
| matmul_128_f64/faer | 101.208 / 105.625 | 99.174 / 104.419 | 3 | 786440 | unavailable |
| matmul_128_f64/kernix | 95.750 / 106.667 | 97.876 / 112.045 | 3 | 327680 | unavailable |
| matmul_128_f64/nalgebra | 113.208 / 316.583 | 109.235 / 188.775 | 2 | 327680 | unavailable |
| matmul_128_f64/ndarray | 97.583 / 103.375 | 104.260 / 110.245 | 2 | 327680 | unavailable |
| matmul_128_f64/numpy | 16.125 / 16.583 | 17.086 / 43.882 | unavailable | unavailable | unavailable |
| matmul_256_f32/candle | 373.459 / 405.500 | 371.566 / 385.905 | 10 | 786520 | unavailable |
| matmul_256_f32/faer | 422.917 / 469.833 | 434.104 / 466.989 | 3 | 1048584 | unavailable |
| matmul_256_f32/kernix | 405.709 / 466.125 | 422.293 / 739.214 | 3 | 589824 | unavailable |
| matmul_256_f32/nalgebra | 410.542 / 451.791 | 412.123 / 428.495 | 2 | 589824 | unavailable |
| matmul_256_f32/ndarray | 422.250 / 1166.708 | 479.087 / 882.038 | 2 | 589824 | unavailable |
| matmul_256_f32/numpy | 34.750 / 37.375 | 33.009 / 37.133 | unavailable | unavailable | unavailable |
| matmul_256_f64/candle | 790.792 / 904.542 | 894.462 / 1164.338 | 10 | 1572952 | unavailable |
| matmul_256_f64/faer | 887.584 / 2370.334 | 900.017 / 1019.375 | 3 | 1572872 | unavailable |
| matmul_256_f64/kernix | 854.500 / 962.666 | 786.622 / 1380.719 | 3 | 1179648 | unavailable |
| matmul_256_f64/nalgebra | 772.958 / 861.875 | 834.118 / 963.239 | 2 | 1179648 | unavailable |
| matmul_256_f64/ndarray | 853.417 / 956.666 | 877.417 / 1128.380 | 2 | 1179648 | unavailable |
| matmul_256_f64/numpy | 123.083 / 151.958 | 122.304 / 175.610 | unavailable | unavailable | unavailable |
| matmul_32_f64/candle | 2.750 / 2.834 | 2.722 / 2.968 | 10 | 18512 | unavailable |
| matmul_32_f64/faer | 2.417 / 2.583 | 2.585 / 4.300 | 3 | 26624 | unavailable |
| matmul_32_f64/kernix | 2.750 / 4.167 | 2.070 / 2.096 | 3 | 24576 | unavailable |
| matmul_32_f64/nalgebra | 2.167 / 2.750 | 2.190 / 2.335 | 2 | 24576 | unavailable |
| matmul_32_f64/ndarray | 6.000 / 7.000 | 2.576 / 5.033 | 2 | 24576 | unavailable |
| matmul_32_f64/numpy | 1.292 / 1.833 | 1.538 / 1.675 | unavailable | unavailable | unavailable |
| matmul_64_f32/candle | 8.000 / 10.125 | 7.827 / 12.382 | 10 | 33872 | unavailable |
| matmul_64_f32/faer | 10.625 / 13.167 | 11.025 / 13.596 | 3 | 50176 | unavailable |
| matmul_64_f32/kernix | 6.750 / 8.416 | 7.024 / 7.144 | 3 | 49152 | unavailable |
| matmul_64_f32/nalgebra | 7.250 / 9.125 | 7.025 / 7.644 | 2 | 49152 | unavailable |
| matmul_64_f32/ndarray | 6.667 / 6.917 | 6.750 / 6.908 | 2 | 49152 | unavailable |
| matmul_64_f32/numpy | 1.750 / 1.958 | 1.899 / 1.972 | unavailable | unavailable | unavailable |
| matmul_64_f64/candle | 13.584 / 14.000 | 13.850 / 14.191 | 10 | 66640 | unavailable |
| matmul_64_f64/faer | 14.708 / 17.917 | 15.134 / 15.613 | 3 | 99328 | unavailable |
| matmul_64_f64/kernix | 13.417 / 14.166 | 13.498 / 13.697 | 3 | 98304 | unavailable |
| matmul_64_f64/nalgebra | 12.667 / 13.583 | 12.852 / 13.216 | 2 | 98304 | unavailable |
| matmul_64_f64/ndarray | 13.750 / 17.209 | 14.555 / 15.052 | 2 | 98304 | unavailable |
| matmul_64_f64/numpy | 3.500 / 3.917 | 3.636 / 3.962 | unavailable | unavailable | unavailable |
| matmul_ab_128_f32/kernix | 63.375 / 75.583 | 60.556 / 101.380 | 3 | 163840 | unavailable |
| matmul_ab_128_f32/numpy | 5.209 / 5.458 | 5.633 / 6.665 | unavailable | unavailable | unavailable |
| matmul_ab_128_f64/kernix | 100.417 / 113.208 | 100.739 / 117.753 | 3 | 327680 | unavailable |
| matmul_ab_128_f64/numpy | 16.000 / 16.750 | 16.550 / 17.655 | unavailable | unavailable | unavailable |
| matmul_batched_4x16x32x16_f64/candle | 3.791 / 4.708 | 4.214 / 9.434 | 25 | 27232 | unavailable |
| matmul_batched_4x16x32x16_f64/kernix | 2.542 / 2.584 | 2.716 / 4.825 | 7 | 16392 | unavailable |
| matmul_batched_4x16x32x16_f64/numpy | 2.167 / 2.500 | 2.374 / 2.456 | unavailable | unavailable | unavailable |
| matmul_sum_backward_32x16x8_f64/candle | 7.208 / 9.125 | 6.931 / 7.629 | 113 | 27980 | unavailable |
| matmul_sum_backward_32x16x8_f64/kernix | 3.292 / 3.417 | 3.022 / 3.352 | 16 | 18376 | unavailable |
| min_norm_32x64_rank32_f64/kernix | 66.375 / 76.167 | 67.229 / 68.490 | 20 | 59136 | unavailable |
| min_norm_32x64_rank32_f64/nalgebra | 152.375 / 169.916 | 156.374 / 220.470 | 15 | 42232 | unavailable |
| min_norm_32x64_rank32_f64/numpy | 143.292 / 166.125 | 143.213 / 146.031 | unavailable | unavailable | unavailable |
| min_norm_64x32_rank16_f64/kernix | 45.750 / 46.291 | 50.194 / 91.270 | 20 | 74752 | unavailable |
| min_norm_64x32_rank16_f64/nalgebra | 75.833 / 89.125 | 96.471 / 186.859 | 15 | 41976 | unavailable |
| min_norm_64x32_rank16_f64/numpy | 79.875 / 120.167 | 117.336 / 245.779 | unavailable | unavailable | unavailable |
| parallel_matmul_into_128_f32/kernix | 52.375 / 121.250 | 56.699 / 91.529 | 4 | 163840 | unavailable |
| parallel_matmul_into_128_f64/kernix | 141.291 / 250.167 | 87.103 / 179.172 | 4 | 327680 | unavailable |
| parallel_matmul_into_256_f64/kernix | 454.917 / 508.500 | 485.064 / 758.601 | 8 | 1179648 | unavailable |
| parallel_matmul_into_64_f64/kernix | 38.250 / 67.417 | 46.306 / 52.198 | 2 | 98304 | unavailable |
| pivoted_qr_256x64_f64/faer | 327.292 / 426.625 | 315.762 / 466.042 | 58 | 336904 | unavailable |
| pivoted_qr_256x64_f64/kernix | 597.625 / 688.708 | 595.618 / 627.372 | 14 | 559104 | unavailable |
| pivoted_qr_256x64_f64/nalgebra | 1054.208 / 1191.625 | 955.443 / 1093.125 | 7 | 296960 | unavailable |
| pivoted_qr_96x32_f64/faer | 39.791 / 41.625 | 41.236 / 50.540 | 32 | 94728 | unavailable |
| pivoted_qr_96x32_f64/kernix | 68.833 / 82.583 | 72.699 / 92.066 | 14 | 107520 | unavailable |
| pivoted_qr_96x32_f64/nalgebra | 92.958 / 102.584 | 93.204 / 100.255 | 7 | 58368 | unavailable |
| qr_256x64_f64/faer | 332.917 / 424.542 | 339.944 / 348.010 | 83 | 335880 | unavailable |
| qr_256x64_f64/kernix | 557.541 / 637.417 | 569.039 / 597.735 | 6 | 426496 | unavailable |
| qr_256x64_f64/nalgebra | 359.708 / 471.375 | 341.335 / 366.762 | 4 | 295424 | unavailable |
| qr_256x64_f64/numpy | 416.083 / 510.708 | 428.875 / 723.107 | unavailable | unavailable | unavailable |
| qr_96x32_f64/faer | 45.041 / 47.083 | 50.141 / 57.590 | 41 | 94216 | unavailable |
| qr_96x32_f64/kernix | 57.417 / 64.667 | 56.000 / 72.539 | 6 | 82176 | unavailable |
| qr_96x32_f64/nalgebra | 44.917 / 52.542 | 45.852 / 89.562 | 4 | 57600 | unavailable |
| qr_96x32_f64/numpy | 52.958 / 66.167 | 52.706 / 54.303 | unavailable | unavailable | unavailable |
| softmax_256x64_f64/candle | 76.250 / 87.250 | 78.449 / 81.535 | 37 | 265200 | unavailable |
| softmax_256x64_f64/kernix | 85.958 / 101.208 | 83.186 / 95.386 | 1 | 131072 | unavailable |
| softmax_256x64_f64/ndarray | 65.208 / 76.708 | 60.754 / 68.499 | 1 | 131072 | unavailable |
| softmax_256x64_f64/numpy | 74.416 / 88.084 | 70.821 / 77.607 | unavailable | unavailable | unavailable |
| tensordot_16x32x8_32x8x16_f64/kernix | 62.625 / 74.750 | 61.470 / 64.629 | 9 | 2166 | unavailable |
| tensordot_16x32x8_32x8x16_f64/numpy | 5.583 / 5.958 | 5.305 / 6.371 | unavailable | unavailable | unavailable |
| training_backward_2x4x8x4_f64/candle | 18.958 / 20.500 | 16.825 / 19.296 | 476 | 21284 | unavailable |
| training_backward_2x4x8x4_f64/kernix | 3.209 / 3.417 | 2.929 / 3.232 | 23 | 4224 | unavailable |
| training_forward_2x4x8x4_f64/candle | 5.708 / 7.458 | 5.276 / 6.731 | 126 | 9568 | unavailable |
| training_forward_2x4x8x4_f64/kernix | 2.417 / 3.000 | 2.233 / 2.715 | 17 | 1032 | unavailable |
| training_record_2x4x8x4_f64/candle | 6.375 / 8.334 | 6.032 / 8.296 | 149 | 11544 | unavailable |
| training_record_2x4x8x4_f64/kernix | 3.208 / 3.375 | 3.211 / 4.774 | 22 | 6832 | unavailable |
| training_sgd_2x4x8x4_f64/kernix | 0.708 / 0.709 | 0.692 / 0.778 | 5 | 688 | unavailable |
| training_shared_backward_64x32_f64/candle | 4.000 / 4.375 | 3.853 / 4.954 | 58 | 99996 | unavailable |
| training_shared_backward_64x32_f64/kernix | 8.000 / 8.292 | 7.978 / 8.737 | 8 | 49768 | unavailable |

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
