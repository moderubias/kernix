# Clean CPU evidence: 0.10.0-clean

Measured source `fcd9fdb97ff307e9c50d0526e8b4e1a580cb44a9`, tree `519899f0fd6fef0f3c289100267dcaf2a518fe7e`; dirty before/after: false.
Library source: workspace fcd9fdb97ff307e9c50d0526e8b4e1a580cb44a9. CPU affinity: 0. Dtype explicitly named per workload; row-major inputs.

| Workload / library | Per-call p50 / p95 (us) | Batch-mean p50 / p95 (us) | Rust allocations | Peak additional requested bytes | Process peak RSS (MiB) |
|---|---:|---:|---:|---:|---:|
| cholesky_factor_128_f64/faer | 40.517 / 43.038 | 41.626 / 42.359 | 2 | 132096 | 5.14 |
| cholesky_factor_128_f64/kernix | 124.276 / 135.865 | 124.491 / 126.345 | 1 | 131072 | 4.16 |
| cholesky_factor_128_f64/nalgebra | 110.192 / 120.971 | 107.815 / 115.638 | 1 | 131072 | 4.07 |
| cholesky_factor_128_f64/numpy | 75.570 / 80.469 | 78.120 / 81.565 | unavailable | unavailable | 36.74 |
| cholesky_factor_256_f64/faer | 228.287 / 276.646 | 225.547 / 238.762 | 2 | 526336 | 5.94 |
| cholesky_factor_256_f64/kernix | 670.643 / 742.819 | 671.566 / 700.507 | 1 | 524288 | 5.39 |
| cholesky_factor_256_f64/nalgebra | 748.822 / 853.596 | 734.646 / 809.104 | 1 | 524288 | 4.95 |
| cholesky_factor_256_f64/numpy | 534.461 / 746.688 | 548.249 / 660.344 | unavailable | unavailable | 40.65 |
| cholesky_factor_64_f64/faer | 9.006 / 9.440 | 9.067 / 9.232 | 2 | 33280 | 4.52 |
| cholesky_factor_64_f64/kernix | 23.868 / 30.234 | 22.280 / 23.497 | 2 | 33280 | 4.06 |
| cholesky_factor_64_f64/nalgebra | 14.759 / 17.349 | 15.097 / 15.653 | 1 | 32768 | 3.95 |
| cholesky_factor_64_f64/numpy | 11.489 / 16.882 | 11.999 / 12.713 | unavailable | unavailable | 35.93 |
| create_ones_64x64_f32/candle | 0.231 / 0.244 | 0.241 / 0.261 | 7 | 16696 | 4.88 |
| create_ones_64x64_f32/faer | 0.193 / 0.203 | 0.184 / 0.187 | 1 | 16384 | 3.98 |
| create_ones_64x64_f32/kernix | 0.196 / 0.207 | 0.205 / 0.218 | 1 | 16384 | 3.97 |
| create_ones_64x64_f32/ndarray | 0.154 / 0.156 | 0.159 / 0.171 | 1 | 16384 | 3.98 |
| create_ones_64x64_f32/numpy | 1.406 / 2.236 | 1.430 / 1.476 | unavailable | unavailable | 35.00 |
| create_zeros_64x64_f32/candle | 0.172 / 0.173 | 0.171 / 0.179 | 5 | 16648 | 4.55 |
| create_zeros_64x64_f32/faer | 0.201 / 0.277 | 0.206 / 0.241 | 1 | 16384 | 4.10 |
| create_zeros_64x64_f32/kernix | 0.141 / 0.143 | 0.151 / 0.159 | 1 | 16384 | 4.09 |
| create_zeros_64x64_f32/ndarray | 0.092 / 0.093 | 0.096 / 0.099 | 1 | 16384 | 3.92 |
| create_zeros_64x64_f32/numpy | 0.389 / 0.430 | 0.423 / 0.440 | unavailable | unavailable | 34.93 |
| matmul_128_f32/candle | 39.425 / 50.227 | 43.428 / 63.914 | 9 | 65816 | 5.31 |
| matmul_128_f32/faer | 50.298 / 64.765 | 50.511 / 53.518 | 1 | 65536 | 4.59 |
| matmul_128_f32/kernix | 40.614 / 42.531 | 41.225 / 43.906 | 3 | 65544 | 4.95 |
| matmul_128_f32/nalgebra | 52.313 / 70.273 | 55.588 / 69.527 | 2 | 163840 | 4.18 |
| matmul_128_f32/ndarray | 51.054 / 63.284 | 53.608 / 55.813 | 2 | 163840 | 4.36 |
| matmul_128_f32/numpy | 39.237 / 48.925 | 42.704 / 43.648 | unavailable | unavailable | 36.53 |
| matmul_128_f64/candle | 83.928 / 89.874 | 83.351 / 87.843 | 9 | 131352 | 5.64 |
| matmul_128_f64/faer | 88.627 / 94.442 | 88.183 / 92.569 | 1 | 131072 | 4.85 |
| matmul_128_f64/kernix | 84.128 / 104.859 | 85.823 / 95.553 | 3 | 131080 | 4.87 |
| matmul_128_f64/nalgebra | 122.242 / 154.730 | 127.544 / 137.027 | 2 | 327680 | 4.39 |
| matmul_128_f64/ndarray | 120.902 / 133.070 | 123.687 / 128.669 | 2 | 327680 | 4.54 |
| matmul_128_f64/numpy | 79.292 / 87.095 | 83.022 / 85.009 | unavailable | unavailable | 36.89 |
| matmul_256_f32/candle | 300.116 / 317.465 | 301.561 / 312.064 | 9 | 262424 | 5.91 |
| matmul_256_f32/faer | 345.495 / 364.352 | 345.366 / 377.693 | 1 | 262144 | 5.13 |
| matmul_256_f32/kernix | 301.732 / 415.792 | 307.227 / 319.410 | 3 | 262152 | 5.38 |
| matmul_256_f32/nalgebra | 415.266 / 493.437 | 424.512 / 438.485 | 2 | 589824 | 4.77 |
| matmul_256_f32/ndarray | 414.735 / 489.895 | 423.024 / 449.206 | 2 | 589824 | 5.05 |
| matmul_256_f32/numpy | 302.060 / 334.918 | 308.939 / 315.126 | unavailable | unavailable | 40.77 |
| matmul_256_f64/candle | 606.776 / 709.591 | 629.647 / 698.732 | 9 | 524568 | 6.42 |
| matmul_256_f64/faer | 880.524 / 1108.456 | 887.191 / 942.362 | 1 | 524288 | 5.93 |
| matmul_256_f64/kernix | 650.087 / 789.764 | 665.494 / 707.758 | 3 | 524296 | 6.18 |
| matmul_256_f64/nalgebra | 693.950 / 800.531 | 705.345 / 895.536 | 2 | 1179648 | 5.62 |
| matmul_256_f64/ndarray | 683.995 / 850.312 | 714.611 / 851.909 | 2 | 1179648 | 6.00 |
| matmul_256_f64/numpy | 667.864 / 788.368 | 686.140 / 745.642 | unavailable | unavailable | 41.50 |
| matmul_32_f64/candle | 1.582 / 1.610 | 1.638 / 1.703 | 9 | 8592 | 5.04 |
| matmul_32_f64/faer | 1.528 / 1.574 | 1.639 / 1.699 | 1 | 8192 | 4.50 |
| matmul_32_f64/kernix | 1.837 / 1.888 | 1.899 / 1.968 | 3 | 24576 | 4.30 |
| matmul_32_f64/nalgebra | 1.498 / 1.587 | 1.533 / 1.614 | 2 | 24576 | 4.02 |
| matmul_32_f64/ndarray | 1.463 / 1.483 | 1.530 / 1.579 | 2 | 24576 | 3.99 |
| matmul_32_f64/numpy | 2.746 / 2.806 | 2.802 / 2.898 | unavailable | unavailable | 35.55 |
| matmul_64_f32/candle | 5.410 / 6.954 | 5.683 / 6.183 | 9 | 16664 | 5.02 |
| matmul_64_f32/faer | 5.562 / 5.639 | 5.694 / 6.386 | 1 | 16384 | 4.36 |
| matmul_64_f32/kernix | 5.146 / 5.290 | 5.188 / 5.472 | 3 | 16544 | 4.67 |
| matmul_64_f32/nalgebra | 6.191 / 6.244 | 6.663 / 6.984 | 2 | 49152 | 4.07 |
| matmul_64_f32/ndarray | 6.229 / 7.603 | 6.390 / 6.812 | 2 | 49152 | 4.16 |
| matmul_64_f32/numpy | 6.525 / 7.308 | 6.651 / 7.221 | unavailable | unavailable | 35.71 |
| matmul_64_f64/candle | 10.449 / 10.774 | 10.897 / 11.686 | 9 | 33048 | 5.04 |
| matmul_64_f64/faer | 11.511 / 13.426 | 11.929 / 12.386 | 1 | 32768 | 4.66 |
| matmul_64_f64/kernix | 10.905 / 11.033 | 10.922 / 11.485 | 3 | 32928 | 4.62 |
| matmul_64_f64/nalgebra | 10.727 / 11.958 | 11.370 / 11.694 | 2 | 98304 | 4.11 |
| matmul_64_f64/ndarray | 11.592 / 13.269 | 11.965 / 15.154 | 2 | 98304 | 4.26 |
| matmul_64_f64/numpy | 12.507 / 13.181 | 13.083 / 14.227 | unavailable | unavailable | 35.82 |
| matmul_batched_4x16x32x16_f64/candle | 2.439 / 2.487 | 2.466 / 2.557 | 25 | 25504 | 5.57 |
| matmul_batched_4x16x32x16_f64/kernix | 2.204 / 2.245 | 2.298 / 2.374 | 7 | 16392 | 4.36 |
| matmul_batched_4x16x32x16_f64/numpy | 3.399 / 3.473 | 3.580 / 3.918 | unavailable | unavailable | 35.35 |
| matmul_sum_backward_32x16x8_f64/candle | 4.581 / 6.378 | 4.679 / 5.137 | 113 | 27988 | 5.87 |
| matmul_sum_backward_32x16x8_f64/kernix | 2.582 / 2.640 | 2.680 / 2.833 | 16 | 18376 | 4.38 |
| min_norm_32x64_rank32_f64/kernix | 61.880 / 81.284 | 62.447 / 64.532 | 20 | 59136 | 4.11 |
| min_norm_32x64_rank32_f64/nalgebra | 140.636 / 150.876 | 142.762 / 150.566 | 15 | 42232 | 4.32 |
| min_norm_32x64_rank32_f64/numpy | 129.431 / 137.262 | 132.467 / 137.223 | unavailable | unavailable | 36.43 |
| min_norm_64x32_rank16_f64/kernix | 48.841 / 51.776 | 49.866 / 51.194 | 20 | 74752 | 4.54 |
| min_norm_64x32_rank16_f64/nalgebra | 78.119 / 85.030 | 79.947 / 84.373 | 15 | 41976 | 4.17 |
| min_norm_64x32_rank16_f64/numpy | 69.879 / 76.040 | 70.991 / 80.726 | unavailable | unavailable | 36.37 |
| parallel_matmul_into_128_f32/kernix | 22.644 / 23.446 | 24.586 / 46.076 | 2 | 16 | 5.11 |
| parallel_matmul_into_128_f64/kernix | 81.816 / 98.297 | 47.048 / 76.053 | 2 | 16 | 5.24 |
| parallel_matmul_into_256_f64/kernix | 307.403 / 540.992 | 328.645 / 377.661 | 2 | 16 | 7.00 |
| parallel_matmul_into_64_f64/kernix | 9.832 / 15.644 | 14.046 / 32.924 | 2 | 98304 | 4.59 |
| pivoted_qr_256x64_f64/faer | 480.146 / 675.202 | 490.342 / 631.596 | 10 | 333312 | 4.99 |
| pivoted_qr_256x64_f64/kernix | 610.066 / 684.604 | 617.455 / 864.722 | 14 | 559104 | 5.04 |
| pivoted_qr_256x64_f64/nalgebra | 807.347 / 866.323 | 820.072 / 841.540 | 7 | 296960 | 4.70 |
| pivoted_qr_96x32_f64/faer | 77.385 / 83.516 | 77.608 / 78.756 | 10 | 68352 | 4.81 |
| pivoted_qr_96x32_f64/kernix | 58.549 / 64.702 | 61.915 / 68.473 | 14 | 107520 | 4.40 |
| pivoted_qr_96x32_f64/nalgebra | 81.493 / 87.753 | 81.919 / 83.387 | 7 | 58368 | 4.27 |
| qr_256x64_f64/faer | 431.156 / 472.390 | 438.584 / 480.499 | 7 | 331776 | 5.41 |
| qr_256x64_f64/kernix | 539.290 / 579.173 | 547.711 / 577.774 | 6 | 426496 | 4.78 |
| qr_256x64_f64/nalgebra | 388.185 / 427.291 | 395.450 / 414.732 | 4 | 295424 | 4.42 |
| qr_256x64_f64/numpy | 375.857 / 428.737 | 380.872 / 408.448 | unavailable | unavailable | 37.42 |
| qr_96x32_f64/faer | 69.955 / 79.743 | 71.192 / 73.247 | 7 | 67584 | 4.93 |
| qr_96x32_f64/kernix | 49.589 / 56.063 | 50.512 / 51.591 | 6 | 82176 | 4.29 |
| qr_96x32_f64/nalgebra | 39.704 / 44.598 | 40.871 / 44.757 | 4 | 57600 | 4.09 |
| qr_96x32_f64/numpy | 42.526 / 47.577 | 43.809 / 48.331 | unavailable | unavailable | 36.56 |
| softmax_256x64_f64/candle | 89.845 / 106.239 | 92.027 / 99.158 | 37 | 265200 | 5.75 |
| softmax_256x64_f64/kernix | 93.964 / 105.462 | 92.439 / 98.227 | 1 | 131072 | 3.98 |
| softmax_256x64_f64/ndarray | 85.585 / 93.528 | 86.381 / 87.653 | 1 | 131072 | 4.40 |
| softmax_256x64_f64/numpy | 110.693 / 116.480 | 112.719 / 117.104 | unavailable | unavailable | 35.48 |
| tensordot_16x32x8_32x8x16_f64/kernix | 39.133 / 56.259 | 43.675 / 46.766 | 9 | 2166 | 4.12 |
| tensordot_16x32x8_32x8x16_f64/numpy | 8.329 / 9.035 | 8.643 / 8.794 | unavailable | unavailable | 35.48 |
| training_backward_2x4x8x4_f64/candle | 15.595 / 16.607 | 15.531 / 19.095 | 472 | 8332 | 6.07 |
| training_backward_2x4x8x4_f64/kernix | 3.194 / 3.263 | 3.416 / 3.615 | 23 | 4224 | 4.67 |
| training_forward_2x4x8x4_f64/candle | 3.932 / 4.156 | 3.797 / 4.045 | 126 | 3912 | 5.95 |
| training_forward_2x4x8x4_f64/kernix | 2.334 / 3.404 | 2.372 / 2.457 | 17 | 1032 | 4.70 |
| training_record_2x4x8x4_f64/candle | 5.592 / 5.930 | 5.685 / 6.442 | 149 | 10712 | 6.00 |
| training_record_2x4x8x4_f64/kernix | 2.774 / 2.826 | 2.928 / 3.345 | 22 | 6832 | 4.52 |
| training_sgd_2x4x8x4_f64/kernix | 0.584 / 0.617 | 0.593 / 0.666 | 5 | 688 | 4.72 |
| training_shared_backward_64x32_f64/candle | 4.158 / 5.453 | 4.207 / 5.346 | 58 | 100004 | 5.58 |
| training_shared_backward_64x32_f64/kernix | 6.501 / 7.140 | 6.407 / 6.794 | 8 | 49768 | 4.63 |

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
