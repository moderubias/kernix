# Clean CPU evidence: baseline-macos-latest-40161751367

Measured source `450df128c1ffb55a9a861230edb6b9a5ac3ef5b4`, tree `957542f0c3a8c779782e0b662134bab34e1572a9`; dirty before/after: false.
Library source: registry 0.11.0. CPU affinity: not pinned. Dtype explicitly named per workload; row-major inputs.

| Workload / library | Per-call p50 / p95 (us) | Batch-mean p50 / p95 (us) | Rust allocations | Peak additional requested bytes | Process peak RSS (MiB) |
|---|---:|---:|---:|---:|---:|
| cholesky_factor_128_f64/faer | 47.500 / 54.334 | 48.253 / 49.260 | 16 | 149504 | unavailable |
| cholesky_factor_128_f64/kernix | 197.416 / 217.958 | 200.514 / 209.494 | 1 | 131072 | unavailable |
| cholesky_factor_128_f64/nalgebra | 102.416 / 115.209 | 103.333 / 104.101 | 1 | 131072 | unavailable |
| cholesky_factor_128_f64/numpy | 47.333 / 51.500 | 47.509 / 48.613 | unavailable | unavailable | unavailable |
| cholesky_factor_256_f64/faer | 267.083 / 300.542 | 269.395 / 278.828 | 76 | 592384 | unavailable |
| cholesky_factor_256_f64/kernix | 652.166 / 743.750 | 622.924 / 635.062 | 49 | 645120 | unavailable |
| cholesky_factor_256_f64/nalgebra | 712.625 / 853.291 | 719.589 / 820.208 | 1 | 524288 | unavailable |
| cholesky_factor_256_f64/numpy | 311.959 / 337.167 | 312.745 / 469.250 | unavailable | unavailable | unavailable |
| cholesky_factor_512_f64/faer | 1645.500 / 1721.916 | 1649.024 / 1765.201 | 388 | 2625544 | unavailable |
| cholesky_factor_512_f64/kernix | 3354.333 / 3512.667 | 3336.945 / 3497.861 | 161 | 2269184 | unavailable |
| cholesky_factor_512_f64/nalgebra | 5327.584 / 5481.666 | 5380.271 / 5509.354 | 1 | 2097152 | unavailable |
| cholesky_factor_512_f64/numpy | 1747.916 / 2204.500 | 2322.433 / 2414.408 | unavailable | unavailable | unavailable |
| cholesky_factor_64_f64/faer | 10.417 / 12.000 | 10.459 / 10.844 | 2 | 33280 | unavailable |
| cholesky_factor_64_f64/kernix | 22.042 / 24.708 | 22.652 / 23.030 | 2 | 33280 | unavailable |
| cholesky_factor_64_f64/nalgebra | 20.625 / 21.167 | 20.270 / 20.819 | 1 | 32768 | unavailable |
| cholesky_factor_64_f64/numpy | 14.917 / 15.416 | 14.872 / 15.147 | unavailable | unavailable | unavailable |
| create_ones_64x64_f32/candle | 0.459 / 0.583 | 0.433 / 0.454 | 7 | 16696 | unavailable |
| create_ones_64x64_f32/faer | 0.375 / 0.458 | 0.372 / 0.386 | 1 | 16384 | unavailable |
| create_ones_64x64_f32/kernix | 0.292 / 0.333 | 0.300 / 0.330 | 1 | 16384 | unavailable |
| create_ones_64x64_f32/ndarray | 0.333 / 0.459 | 0.271 / 0.313 | 1 | 16384 | unavailable |
| create_ones_64x64_f32/numpy | 1.042 / 1.167 | 1.200 / 1.246 | unavailable | unavailable | unavailable |
| create_zeros_64x64_f32/candle | 0.333 / 0.334 | 0.362 / 0.384 | 5 | 16648 | unavailable |
| create_zeros_64x64_f32/faer | 0.250 / 0.292 | 0.266 / 0.277 | 1 | 16384 | unavailable |
| create_zeros_64x64_f32/kernix | 0.333 / 0.334 | 0.317 / 0.351 | 1 | 16384 | unavailable |
| create_zeros_64x64_f32/ndarray | 0.333 / 0.458 | 0.277 / 0.306 | 1 | 16384 | unavailable |
| create_zeros_64x64_f32/numpy | 0.541 / 0.667 | 0.582 / 0.610 | unavailable | unavailable | unavailable |
| matmul_128_f32/candle | 52.833 / 53.708 | 53.923 / 59.536 | 10 | 327768 | unavailable |
| matmul_128_f32/faer | 64.292 / 76.792 | 63.105 / 65.528 | 3 | 393224 | unavailable |
| matmul_128_f32/kernix | 56.708 / 67.291 | 61.190 / 91.286 | 3 | 163840 | unavailable |
| matmul_128_f32/kernix-accelerate | 5.958 / 6.334 | 5.989 / 6.102 | 1 | 65536 | unavailable |
| matmul_128_f32/nalgebra | 53.167 / 67.333 | 59.135 / 87.882 | 2 | 163840 | unavailable |
| matmul_128_f32/ndarray | 49.417 / 51.208 | 49.634 / 50.383 | 2 | 163840 | unavailable |
| matmul_128_f32/numpy | 5.750 / 6.125 | 5.731 / 5.825 | unavailable | unavailable | unavailable |
| matmul_128_f64/candle | 98.000 / 116.958 | 98.509 / 123.905 | 10 | 655448 | unavailable |
| matmul_128_f64/faer | 105.334 / 112.125 | 106.335 / 106.936 | 3 | 786440 | unavailable |
| matmul_128_f64/kernix | 103.208 / 111.167 | 103.939 / 106.945 | 3 | 327680 | unavailable |
| matmul_128_f64/kernix-accelerate | 17.292 / 18.291 | 17.363 / 17.850 | 1 | 131072 | unavailable |
| matmul_128_f64/nalgebra | 98.292 / 115.667 | 100.850 / 126.993 | 2 | 327680 | unavailable |
| matmul_128_f64/ndarray | 101.708 / 107.916 | 102.722 / 103.290 | 2 | 327680 | unavailable |
| matmul_128_f64/numpy | 16.542 / 17.542 | 16.755 / 17.254 | unavailable | unavailable | unavailable |
| matmul_256_f32/candle | 392.417 / 411.291 | 396.090 / 406.200 | 10 | 786520 | unavailable |
| matmul_256_f32/faer | 412.375 / 435.500 | 416.158 / 422.363 | 3 | 1048584 | unavailable |
| matmul_256_f32/kernix | 389.250 / 407.708 | 392.897 / 402.412 | 3 | 589824 | unavailable |
| matmul_256_f32/kernix-accelerate | 34.125 / 36.375 | 34.527 / 34.989 | 1 | 262144 | unavailable |
| matmul_256_f32/nalgebra | 387.208 / 408.333 | 391.256 / 404.167 | 2 | 589824 | unavailable |
| matmul_256_f32/ndarray | 412.292 / 464.583 | 411.082 / 427.076 | 2 | 589824 | unavailable |
| matmul_256_f32/numpy | 32.917 / 35.125 | 33.151 / 34.119 | unavailable | unavailable | unavailable |
| matmul_256_f64/candle | 792.000 / 957.250 | 854.611 / 934.542 | 10 | 1572952 | unavailable |
| matmul_256_f64/faer | 812.416 / 845.250 | 814.487 / 823.881 | 3 | 1572872 | unavailable |
| matmul_256_f64/kernix | 809.500 / 877.958 | 795.010 / 811.048 | 3 | 1179648 | unavailable |
| matmul_256_f64/kernix-accelerate | 121.792 / 133.250 | 119.576 / 121.028 | 1 | 524288 | unavailable |
| matmul_256_f64/nalgebra | 778.167 / 793.083 | 771.535 / 791.494 | 2 | 1179648 | unavailable |
| matmul_256_f64/ndarray | 789.083 / 845.375 | 789.410 / 792.740 | 2 | 1179648 | unavailable |
| matmul_256_f64/numpy | 108.167 / 110.542 | 113.031 / 122.278 | unavailable | unavailable | unavailable |
| matmul_32_f64/candle | 2.625 / 3.041 | 2.497 / 2.643 | 10 | 18512 | unavailable |
| matmul_32_f64/faer | 2.417 / 2.542 | 2.443 / 2.498 | 3 | 26624 | unavailable |
| matmul_32_f64/kernix | 2.250 / 3.000 | 2.475 / 2.838 | 3 | 24576 | unavailable |
| matmul_32_f64/nalgebra | 2.083 / 2.250 | 2.128 / 2.181 | 2 | 24576 | unavailable |
| matmul_32_f64/ndarray | 2.041 / 2.042 | 2.045 / 2.137 | 2 | 24576 | unavailable |
| matmul_32_f64/numpy | 1.458 / 1.625 | 1.540 / 1.682 | unavailable | unavailable | unavailable |
| matmul_64_f32/candle | 7.875 / 8.166 | 7.904 / 8.007 | 10 | 33872 | unavailable |
| matmul_64_f32/faer | 8.167 / 9.917 | 8.315 / 8.599 | 3 | 50176 | unavailable |
| matmul_64_f32/kernix | 7.625 / 8.042 | 7.367 / 7.434 | 3 | 49152 | unavailable |
| matmul_64_f32/nalgebra | 7.000 / 8.958 | 7.229 / 7.898 | 2 | 49152 | unavailable |
| matmul_64_f32/ndarray | 6.958 / 8.875 | 7.475 / 7.615 | 2 | 49152 | unavailable |
| matmul_64_f32/numpy | 1.916 / 2.250 | 2.194 / 2.234 | unavailable | unavailable | unavailable |
| matmul_64_f64/candle | 14.708 / 17.875 | 14.777 / 14.909 | 10 | 66640 | unavailable |
| matmul_64_f64/faer | 14.750 / 15.166 | 14.922 / 15.324 | 3 | 99328 | unavailable |
| matmul_64_f64/kernix | 13.875 / 16.792 | 14.270 / 15.465 | 3 | 98304 | unavailable |
| matmul_64_f64/nalgebra | 13.542 / 13.584 | 14.224 / 14.556 | 2 | 98304 | unavailable |
| matmul_64_f64/ndarray | 13.833 / 37.917 | 14.816 / 16.333 | 2 | 98304 | unavailable |
| matmul_64_f64/numpy | 3.250 / 3.417 | 3.360 / 3.510 | unavailable | unavailable | unavailable |
| matmul_ab_128_f32/kernix | 52.875 / 66.250 | 55.711 / 57.744 | 3 | 163840 | unavailable |
| matmul_ab_128_f32/numpy | 6.042 / 6.666 | 6.262 / 6.491 | unavailable | unavailable | unavailable |
| matmul_ab_128_f64/kernix | 103.500 / 112.667 | 102.408 / 105.425 | 3 | 327680 | unavailable |
| matmul_ab_128_f64/numpy | 17.250 / 17.666 | 17.615 / 19.204 | unavailable | unavailable | unavailable |
| matmul_batched_4x16x32x16_f32/candle | 2.500 / 3.125 | 2.456 / 3.231 | 25 | 14944 | unavailable |
| matmul_batched_4x16x32x16_f32/kernix | 1.750 / 2.208 | 1.739 / 1.798 | 7 | 8200 | unavailable |
| matmul_batched_4x16x32x16_f32/numpy | 2.209 / 3.959 | 2.153 / 2.405 | unavailable | unavailable | unavailable |
| matmul_batched_4x16x32x16_f64/candle | 3.458 / 3.500 | 3.169 / 3.462 | 25 | 27232 | unavailable |
| matmul_batched_4x16x32x16_f64/kernix | 2.833 / 3.500 | 2.832 / 2.949 | 7 | 16392 | unavailable |
| matmul_batched_4x16x32x16_f64/numpy | 1.958 / 2.125 | 2.094 / 2.174 | unavailable | unavailable | unavailable |
| matmul_sum_backward_32x16x8_f64/candle | 6.458 / 8.000 | 5.939 / 6.119 | 113 | 27980 | unavailable |
| matmul_sum_backward_32x16x8_f64/kernix | 3.208 / 3.250 | 3.095 / 3.252 | 16 | 18376 | unavailable |
| min_norm_32x64_rank32_f64/kernix | 65.333 / 80.792 | 68.099 / 73.621 | 20 | 59136 | unavailable |
| min_norm_32x64_rank32_f64/nalgebra | 153.916 / 164.792 | 155.383 / 160.684 | 15 | 42232 | unavailable |
| min_norm_32x64_rank32_f64/numpy | 147.125 / 157.584 | 146.711 / 150.226 | unavailable | unavailable | unavailable |
| min_norm_64x32_rank16_f64/kernix | 49.250 / 53.334 | 49.846 / 50.087 | 20 | 74752 | unavailable |
| min_norm_64x32_rank16_f64/nalgebra | 69.625 / 73.666 | 69.964 / 70.678 | 15 | 41976 | unavailable |
| min_norm_64x32_rank16_f64/numpy | 84.458 / 91.208 | 82.915 / 84.065 | unavailable | unavailable | unavailable |
| parallel_matmul_into_128_f32/kernix | 45.000 / 61.333 | 46.354 / 5057.479 | 2 | 98304 | unavailable |
| parallel_matmul_into_128_f64/kernix | 69.125 / 77.416 | 70.441 / 74.490 | 2 | 393216 | unavailable |
| parallel_matmul_into_256_f64/kernix | 420.083 / 438.333 | 420.366 / 446.817 | 2 | 1310720 | unavailable |
| parallel_matmul_into_64_f64/kernix | 20.042 / 32.958 | 21.400 / 24.103 | 2 | 98304 | unavailable |
| pivoted_qr_256x64_f64/faer | 289.250 / 300.833 | 290.699 / 293.621 | 58 | 336904 | unavailable |
| pivoted_qr_256x64_f64/kernix | 568.042 / 606.375 | 571.294 / 582.625 | 14 | 559104 | unavailable |
| pivoted_qr_256x64_f64/nalgebra | 998.417 / 1044.750 | 1001.263 / 1012.587 | 7 | 296960 | unavailable |
| pivoted_qr_96x32_f64/faer | 41.333 / 50.709 | 42.636 / 44.822 | 32 | 94728 | unavailable |
| pivoted_qr_96x32_f64/kernix | 66.208 / 85.083 | 66.885 / 69.449 | 14 | 107520 | unavailable |
| pivoted_qr_96x32_f64/nalgebra | 99.583 / 108.125 | 99.891 / 100.948 | 7 | 58368 | unavailable |
| qr_256x64_f64/faer | 304.666 / 324.334 | 307.529 / 320.387 | 83 | 335880 | unavailable |
| qr_256x64_f64/kernix | 522.958 / 554.791 | 519.288 / 530.428 | 6 | 426496 | unavailable |
| qr_256x64_f64/nalgebra | 378.791 / 413.667 | 378.147 / 391.503 | 4 | 295424 | unavailable |
| qr_256x64_f64/numpy | 385.750 / 462.000 | 385.564 / 394.231 | unavailable | unavailable | unavailable |
| qr_96x32_f64/faer | 46.833 / 52.209 | 48.186 / 53.404 | 41 | 94216 | unavailable |
| qr_96x32_f64/kernix | 57.583 / 60.084 | 58.179 / 58.666 | 6 | 82176 | unavailable |
| qr_96x32_f64/nalgebra | 42.625 / 51.917 | 44.515 / 45.746 | 4 | 57600 | unavailable |
| qr_96x32_f64/numpy | 49.291 / 53.292 | 49.040 / 50.254 | unavailable | unavailable | unavailable |
| softmax_256x64_f64/candle | 67.417 / 68.000 | 67.688 / 68.167 | 37 | 265200 | unavailable |
| softmax_256x64_f64/kernix | 79.125 / 83.542 | 83.126 / 83.515 | 1 | 131072 | unavailable |
| softmax_256x64_f64/ndarray | 61.208 / 67.083 | 61.874 / 80.640 | 1 | 131072 | unavailable |
| softmax_256x64_f64/numpy | 74.416 / 79.417 | 75.489 / 76.074 | unavailable | unavailable | unavailable |
| tensordot_16x32x8_32x8x16_f64/kernix | 62.750 / 70.792 | 63.392 / 63.949 | 9 | 2166 | unavailable |
| tensordot_16x32x8_32x8x16_f64/numpy | 5.208 / 5.500 | 5.615 / 7.006 | unavailable | unavailable | unavailable |
| training_backward_2x4x8x4_f64/candle | 20.166 / 24.417 | 17.734 / 19.809 | 476 | 21284 | unavailable |
| training_backward_2x4x8x4_f64/kernix | 3.583 / 3.750 | 3.101 / 3.519 | 23 | 4224 | unavailable |
| training_forward_2x4x8x4_f64/candle | 5.375 / 5.916 | 4.417 / 5.068 | 126 | 9568 | unavailable |
| training_forward_2x4x8x4_f64/kernix | 2.375 / 2.500 | 2.402 / 3.119 | 17 | 1032 | unavailable |
| training_record_2x4x8x4_f64/candle | 7.625 / 9.250 | 6.144 / 8.248 | 149 | 11544 | unavailable |
| training_record_2x4x8x4_f64/kernix | 3.333 / 4.875 | 3.331 / 4.228 | 22 | 6832 | unavailable |
| training_sgd_2x4x8x4_f64/kernix | 0.750 / 0.792 | 0.716 / 0.776 | 5 | 688 | unavailable |
| training_shared_backward_64x32_f64/candle | 4.083 / 4.250 | 3.833 / 4.083 | 58 | 99996 | unavailable |
| training_shared_backward_64x32_f64/kernix | 5.792 / 6.125 | 5.758 / 6.054 | 8 | 49768 | unavailable |

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
