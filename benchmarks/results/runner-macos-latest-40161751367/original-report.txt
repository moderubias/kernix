# Clean CPU evidence: runner-macos-latest-40161751367

Measured source `450df128c1ffb55a9a861230edb6b9a5ac3ef5b4`, tree `957542f0c3a8c779782e0b662134bab34e1572a9`; dirty before/after: false.
Library source: workspace 450df128c1ffb55a9a861230edb6b9a5ac3ef5b4. CPU affinity: not pinned. Dtype explicitly named per workload; row-major inputs.

| Workload / library | Per-call p50 / p95 (us) | Batch-mean p50 / p95 (us) | Rust allocations | Peak additional requested bytes | Process peak RSS (MiB) |
|---|---:|---:|---:|---:|---:|
| cholesky_factor_128_f64/faer | 47.333 / 57.500 | 50.617 / 56.278 | 16 | 149504 | unavailable |
| cholesky_factor_128_f64/kernix | 187.458 / 229.750 | 191.182 / 220.196 | 1 | 131072 | unavailable |
| cholesky_factor_128_f64/nalgebra | 114.083 / 137.958 | 106.189 / 118.067 | 1 | 131072 | unavailable |
| cholesky_factor_128_f64/numpy | 51.292 / 98.916 | 57.644 / 101.478 | unavailable | unavailable | unavailable |
| cholesky_factor_256_f64/faer | 269.916 / 346.583 | 277.939 / 351.122 | 76 | 592384 | unavailable |
| cholesky_factor_256_f64/kernix | 759.250 / 969.083 | 782.526 / 855.809 | 49 | 645120 | unavailable |
| cholesky_factor_256_f64/nalgebra | 1094.583 / 1413.167 | 1216.593 / 1723.354 | 1 | 524288 | unavailable |
| cholesky_factor_256_f64/numpy | 431.625 / 632.500 | 421.606 / 921.330 | unavailable | unavailable | unavailable |
| cholesky_factor_512_f64/faer | 2441.750 / 4851.250 | 2275.567 / 3654.000 | 388 | 2625544 | unavailable |
| cholesky_factor_512_f64/kernix | 3338.125 / 3996.084 | 3337.819 / 3842.695 | 161 | 2269184 | unavailable |
| cholesky_factor_512_f64/nalgebra | 7143.375 / 10562.708 | 6748.188 / 7353.333 | 1 | 2097152 | unavailable |
| cholesky_factor_512_f64/numpy | 2748.042 / 4912.667 | 2893.229 / 4248.847 | unavailable | unavailable | unavailable |
| cholesky_factor_64_f64/faer | 10.833 / 11.333 | 11.581 / 12.526 | 2 | 33280 | unavailable |
| cholesky_factor_64_f64/kernix | 22.292 / 28.833 | 30.367 / 41.573 | 2 | 33280 | unavailable |
| cholesky_factor_64_f64/nalgebra | 20.917 / 24.958 | 25.881 / 26.781 | 1 | 32768 | unavailable |
| cholesky_factor_64_f64/numpy | 16.833 / 48.792 | 19.146 / 29.254 | unavailable | unavailable | unavailable |
| create_ones_64x64_f32/candle | 0.500 / 0.833 | 0.561 / 0.718 | 7 | 16696 | unavailable |
| create_ones_64x64_f32/faer | 0.334 / 0.417 | 0.349 / 0.446 | 1 | 16384 | unavailable |
| create_ones_64x64_f32/kernix | 0.417 / 0.625 | 0.458 / 0.632 | 1 | 16384 | unavailable |
| create_ones_64x64_f32/ndarray | 0.334 / 0.667 | 0.407 / 0.502 | 1 | 16384 | unavailable |
| create_ones_64x64_f32/numpy | 1.084 / 1.167 | 1.244 / 1.413 | unavailable | unavailable | unavailable |
| create_zeros_64x64_f32/candle | 1.208 / 1.500 | 1.067 / 1.337 | 5 | 16648 | unavailable |
| create_zeros_64x64_f32/faer | 0.291 / 0.292 | 0.638 / 0.830 | 1 | 16384 | unavailable |
| create_zeros_64x64_f32/kernix | 0.334 / 0.584 | 0.331 / 0.728 | 1 | 16384 | unavailable |
| create_zeros_64x64_f32/ndarray | 0.791 / 1.000 | 0.268 / 0.464 | 1 | 16384 | unavailable |
| create_zeros_64x64_f32/numpy | 1.375 / 1.583 | 1.106 / 1.393 | unavailable | unavailable | unavailable |
| matmul_128_f32/candle | 63.792 / 124.375 | 61.383 / 70.629 | 10 | 327768 | unavailable |
| matmul_128_f32/faer | 60.792 / 96.042 | 61.528 / 67.911 | 3 | 393224 | unavailable |
| matmul_128_f32/kernix | 55.334 / 100.625 | 60.946 / 87.136 | 3 | 163840 | unavailable |
| matmul_128_f32/kernix-accelerate | 6.333 / 17.583 | 6.921 / 9.332 | 1 | 65536 | unavailable |
| matmul_128_f32/nalgebra | 55.042 / 121.500 | 66.600 / 70.034 | 2 | 163840 | unavailable |
| matmul_128_f32/ndarray | 49.458 / 51.209 | 55.630 / 66.371 | 2 | 163840 | unavailable |
| matmul_128_f32/numpy | 5.959 / 6.292 | 6.730 / 9.796 | unavailable | unavailable | unavailable |
| matmul_128_f64/candle | 112.084 / 196.416 | 121.635 / 186.156 | 10 | 655448 | unavailable |
| matmul_128_f64/faer | 113.542 / 183.625 | 113.803 / 134.713 | 3 | 786440 | unavailable |
| matmul_128_f64/kernix | 115.500 / 186.875 | 145.362 / 194.725 | 3 | 327680 | unavailable |
| matmul_128_f64/kernix-accelerate | 18.000 / 18.958 | 19.141 / 20.728 | 1 | 131072 | unavailable |
| matmul_128_f64/nalgebra | 114.500 / 195.084 | 130.524 / 175.956 | 2 | 327680 | unavailable |
| matmul_128_f64/ndarray | 116.500 / 266.250 | 131.851 / 274.857 | 2 | 327680 | unavailable |
| matmul_128_f64/numpy | 19.500 / 93.666 | 22.602 / 39.813 | unavailable | unavailable | unavailable |
| matmul_256_f32/candle | 433.917 / 946.333 | 517.410 / 728.556 | 10 | 786520 | unavailable |
| matmul_256_f32/faer | 401.458 / 478.375 | 414.577 / 510.729 | 3 | 1048584 | unavailable |
| matmul_256_f32/kernix | 442.083 / 574.042 | 508.002 / 552.667 | 3 | 589824 | unavailable |
| matmul_256_f32/kernix-accelerate | 35.792 / 37.834 | 36.211 / 66.683 | 1 | 262144 | unavailable |
| matmul_256_f32/nalgebra | 416.042 / 778.917 | 457.549 / 646.177 | 2 | 589824 | unavailable |
| matmul_256_f32/ndarray | 391.375 / 577.042 | 392.391 / 423.536 | 2 | 589824 | unavailable |
| matmul_256_f32/numpy | 34.916 / 145.125 | 39.786 / 43.594 | unavailable | unavailable | unavailable |
| matmul_256_f64/candle | 934.500 / 1773.875 | 966.027 / 1254.486 | 10 | 1572952 | unavailable |
| matmul_256_f64/faer | 1071.208 / 2049.542 | 1205.366 / 2236.508 | 3 | 1572872 | unavailable |
| matmul_256_f64/kernix | 1005.250 / 2269.000 | 1305.526 / 2648.094 | 3 | 1179648 | unavailable |
| matmul_256_f64/kernix-accelerate | 124.458 / 177.250 | 153.912 / 284.318 | 1 | 524288 | unavailable |
| matmul_256_f64/nalgebra | 821.583 / 971.667 | 813.747 / 889.434 | 2 | 1179648 | unavailable |
| matmul_256_f64/ndarray | 989.959 / 1402.292 | 964.140 / 1039.235 | 2 | 1179648 | unavailable |
| matmul_256_f64/numpy | 125.250 / 338.542 | 146.774 / 271.329 | unavailable | unavailable | unavailable |
| matmul_32_f64/candle | 2.709 / 3.584 | 3.159 / 4.177 | 10 | 18512 | unavailable |
| matmul_32_f64/faer | 2.500 / 6.583 | 3.005 / 5.859 | 3 | 26624 | unavailable |
| matmul_32_f64/kernix | 2.250 / 3.000 | 2.738 / 4.165 | 3 | 24576 | unavailable |
| matmul_32_f64/nalgebra | 2.125 / 2.750 | 2.574 / 2.871 | 2 | 24576 | unavailable |
| matmul_32_f64/ndarray | 5.792 / 6.292 | 3.122 / 4.138 | 2 | 24576 | unavailable |
| matmul_32_f64/numpy | 1.459 / 1.625 | 1.555 / 1.596 | unavailable | unavailable | unavailable |
| matmul_64_f32/candle | 8.167 / 23.625 | 8.013 / 8.416 | 10 | 33872 | unavailable |
| matmul_64_f32/faer | 8.292 / 8.958 | 8.913 / 11.240 | 3 | 50176 | unavailable |
| matmul_64_f32/kernix | 7.333 / 9.209 | 8.406 / 12.855 | 3 | 49152 | unavailable |
| matmul_64_f32/nalgebra | 6.917 / 8.625 | 8.503 / 10.753 | 2 | 49152 | unavailable |
| matmul_64_f32/ndarray | 6.959 / 20.750 | 8.783 / 9.310 | 2 | 49152 | unavailable |
| matmul_64_f32/numpy | 1.958 / 9.167 | 2.355 / 3.101 | unavailable | unavailable | unavailable |
| matmul_64_f64/candle | 14.750 / 41.958 | 19.626 / 38.785 | 10 | 66640 | unavailable |
| matmul_64_f64/faer | 15.916 / 31.542 | 21.372 / 31.288 | 3 | 99328 | unavailable |
| matmul_64_f64/kernix | 14.125 / 24.417 | 16.592 / 20.438 | 3 | 98304 | unavailable |
| matmul_64_f64/nalgebra | 14.792 / 18.084 | 16.183 / 22.728 | 2 | 98304 | unavailable |
| matmul_64_f64/ndarray | 13.333 / 13.500 | 15.490 / 27.592 | 2 | 98304 | unavailable |
| matmul_64_f64/numpy | 3.667 / 4.291 | 4.536 / 5.006 | unavailable | unavailable | unavailable |
| matmul_ab_128_f32/kernix | 67.791 / 162.334 | 74.231 / 85.254 | 3 | 163840 | unavailable |
| matmul_ab_128_f32/numpy | 6.083 / 6.292 | 6.016 / 11.939 | unavailable | unavailable | unavailable |
| matmul_ab_128_f64/kernix | 117.958 / 306.041 | 154.256 / 171.412 | 3 | 327680 | unavailable |
| matmul_ab_128_f64/numpy | 17.583 / 72.875 | 27.430 / 48.239 | unavailable | unavailable | unavailable |
| matmul_batched_4x16x32x16_f32/candle | 2.666 / 3.417 | 2.474 / 2.639 | 25 | 14944 | unavailable |
| matmul_batched_4x16x32x16_f32/kernix | 2.167 / 3.375 | 2.100 / 3.083 | 7 | 8200 | unavailable |
| matmul_batched_4x16x32x16_f32/numpy | 2.208 / 2.458 | 2.374 / 2.557 | unavailable | unavailable | unavailable |
| matmul_batched_4x16x32x16_f64/candle | 3.791 / 20.625 | 4.342 / 7.098 | 25 | 27232 | unavailable |
| matmul_batched_4x16x32x16_f64/kernix | 2.833 / 2.917 | 2.849 / 4.786 | 7 | 16392 | unavailable |
| matmul_batched_4x16x32x16_f64/numpy | 2.208 / 2.542 | 2.328 / 3.104 | unavailable | unavailable | unavailable |
| matmul_sum_backward_32x16x8_f64/candle | 9.250 / 12.375 | 10.455 / 16.502 | 113 | 27980 | unavailable |
| matmul_sum_backward_32x16x8_f64/kernix | 3.084 / 3.958 | 3.056 / 3.806 | 16 | 18376 | unavailable |
| min_norm_32x64_rank32_f64/kernix | 73.000 / 80.834 | 71.835 / 76.126 | 20 | 59136 | unavailable |
| min_norm_32x64_rank32_f64/nalgebra | 195.000 / 318.125 | 235.275 / 378.640 | 15 | 42232 | unavailable |
| min_norm_32x64_rank32_f64/numpy | 186.333 / 480.333 | 213.078 / 323.753 | unavailable | unavailable | unavailable |
| min_norm_64x32_rank16_f64/kernix | 51.625 / 131.666 | 52.097 / 68.751 | 20 | 74752 | unavailable |
| min_norm_64x32_rank16_f64/nalgebra | 74.917 / 192.875 | 88.590 / 189.841 | 15 | 41976 | unavailable |
| min_norm_64x32_rank16_f64/numpy | 94.333 / 170.083 | 94.038 / 155.834 | unavailable | unavailable | unavailable |
| parallel_matmul_into_128_f32/kernix | 99.541 / 151.542 | 80.749 / 112.351 | 2 | 196608 | unavailable |
| parallel_matmul_into_128_f64/kernix | 157.750 / 250.750 | 117.683 / 189.002 | 2 | 393216 | unavailable |
| parallel_matmul_into_256_f64/kernix | 605.917 / 1041.125 | 465.593 / 737.580 | 2 | 655360 | unavailable |
| parallel_matmul_into_64_f64/kernix | 26.708 / 66.000 | 33.506 / 42.974 | 2 | 98304 | unavailable |
| pivoted_qr_256x64_f64/faer | 359.500 / 592.459 | 375.630 / 457.542 | 58 | 336904 | unavailable |
| pivoted_qr_256x64_f64/kernix | 727.625 / 1379.625 | 808.144 / 1034.825 | 14 | 559104 | unavailable |
| pivoted_qr_256x64_f64/nalgebra | 1057.666 / 1723.667 | 1081.079 / 1370.989 | 7 | 296960 | unavailable |
| pivoted_qr_96x32_f64/faer | 41.667 / 50.542 | 54.499 / 58.044 | 32 | 94728 | unavailable |
| pivoted_qr_96x32_f64/kernix | 73.084 / 163.375 | 91.893 / 141.565 | 14 | 107520 | unavailable |
| pivoted_qr_96x32_f64/nalgebra | 110.000 / 232.083 | 109.167 / 150.022 | 7 | 58368 | unavailable |
| qr_256x64_f64/faer | 316.084 / 943.583 | 381.327 / 679.200 | 83 | 335880 | unavailable |
| qr_256x64_f64/kernix | 622.667 / 1161.667 | 684.273 / 762.479 | 6 | 426496 | unavailable |
| qr_256x64_f64/nalgebra | 395.792 / 577.750 | 428.378 / 457.438 | 4 | 295424 | unavailable |
| qr_256x64_f64/numpy | 400.958 / 472.333 | 402.905 / 459.706 | unavailable | unavailable | unavailable |
| qr_96x32_f64/faer | 51.958 / 144.792 | 64.464 / 81.461 | 41 | 94216 | unavailable |
| qr_96x32_f64/kernix | 63.625 / 133.084 | 62.381 / 66.899 | 6 | 82176 | unavailable |
| qr_96x32_f64/nalgebra | 42.000 / 51.166 | 50.329 / 54.654 | 4 | 57600 | unavailable |
| qr_96x32_f64/numpy | 61.166 / 107.875 | 57.811 / 81.211 | unavailable | unavailable | unavailable |
| softmax_256x64_f64/candle | 82.417 / 105.416 | 88.764 / 186.938 | 37 | 265200 | unavailable |
| softmax_256x64_f64/kernix | 92.042 / 204.417 | 99.053 / 137.081 | 1 | 131072 | unavailable |
| softmax_256x64_f64/ndarray | 67.500 / 143.958 | 81.514 / 89.353 | 1 | 131072 | unavailable |
| softmax_256x64_f64/numpy | 89.166 / 140.584 | 90.610 / 100.133 | unavailable | unavailable | unavailable |
| tensordot_16x32x8_32x8x16_f64/kernix | 69.208 / 85.000 | 69.566 / 96.542 | 9 | 2166 | unavailable |
| tensordot_16x32x8_32x8x16_f64/numpy | 5.542 / 5.834 | 5.781 / 6.303 | unavailable | unavailable | unavailable |
| training_backward_2x4x8x4_f64/candle | 23.667 / 29.250 | 21.837 / 24.486 | 476 | 21284 | unavailable |
| training_backward_2x4x8x4_f64/kernix | 8.917 / 9.333 | 3.878 / 4.827 | 23 | 4224 | unavailable |
| training_forward_2x4x8x4_f64/candle | 15.916 / 33.625 | 9.391 / 12.046 | 126 | 9568 | unavailable |
| training_forward_2x4x8x4_f64/kernix | 2.459 / 3.042 | 2.315 / 2.938 | 17 | 1032 | unavailable |
| training_record_2x4x8x4_f64/candle | 6.958 / 9.917 | 8.749 / 16.923 | 149 | 11544 | unavailable |
| training_record_2x4x8x4_f64/kernix | 4.792 / 13.875 | 3.979 / 5.190 | 22 | 6832 | unavailable |
| training_sgd_2x4x8x4_f64/kernix | 0.750 / 0.959 | 0.817 / 1.014 | 5 | 688 | unavailable |
| training_shared_backward_64x32_f64/candle | 4.334 / 5.500 | 5.866 / 9.435 | 58 | 99996 | unavailable |
| training_shared_backward_64x32_f64/kernix | 6.042 / 7.709 | 6.843 / 7.444 | 8 | 49768 | unavailable |

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
