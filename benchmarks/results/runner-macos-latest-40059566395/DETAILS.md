# Clean CPU evidence: runner-macos-latest-40059566395

Measured source `18750693908110110d85d02d0f11af8d58bfae8b`, tree `75403e48db8556f95e014ec2c5b16b30f9fa781e`; dirty before/after: false.
Library source: workspace 18750693908110110d85d02d0f11af8d58bfae8b. CPU affinity: not pinned. Dtype explicitly named per workload; row-major inputs.

| Workload / library | Per-call p50 / p95 (us) | Batch-mean p50 / p95 (us) | Rust allocations | Peak additional requested bytes | Process peak RSS (MiB) |
|---|---:|---:|---:|---:|---:|
| cholesky_factor_128_f64/faer | 47.083 / 54.041 | 50.072 / 75.879 | 16 | 149504 | unavailable |
| cholesky_factor_128_f64/kernix | 166.459 / 182.708 | 166.834 / 168.883 | 1 | 131072 | unavailable |
| cholesky_factor_128_f64/nalgebra | 113.792 / 257.458 | 179.672 / 816.883 | 1 | 131072 | unavailable |
| cholesky_factor_128_f64/numpy | 51.250 / 52.250 | 52.283 / 54.128 | unavailable | unavailable | unavailable |
| cholesky_factor_256_f64/faer | 275.084 / 520.125 | 274.695 / 388.907 | 76 | 592384 | unavailable |
| cholesky_factor_256_f64/kernix | 646.875 / 731.875 | 644.865 / 878.889 | 49 | 645120 | unavailable |
| cholesky_factor_256_f64/nalgebra | 734.250 / 835.500 | 753.884 / 769.365 | 1 | 524288 | unavailable |
| cholesky_factor_256_f64/numpy | 317.000 / 449.125 | 290.306 / 315.267 | unavailable | unavailable | unavailable |
| cholesky_factor_512_f64/faer | 1658.792 / 2038.875 | 2030.933 / 2689.700 | 388 | 2625544 | unavailable |
| cholesky_factor_512_f64/kernix | 3496.208 / 7314.625 | 3589.167 / 4173.695 | 161 | 2269184 | unavailable |
| cholesky_factor_512_f64/nalgebra | 5620.458 / 7343.250 | 5589.979 / 6151.938 | 1 | 2097152 | unavailable |
| cholesky_factor_512_f64/numpy | 1888.375 / 2210.125 | 1890.882 / 2188.942 | unavailable | unavailable | unavailable |
| cholesky_factor_64_f64/faer | 10.292 / 10.625 | 10.368 / 10.724 | 2 | 33280 | unavailable |
| cholesky_factor_64_f64/kernix | 22.708 / 28.125 | 23.889 / 28.496 | 2 | 33280 | unavailable |
| cholesky_factor_64_f64/nalgebra | 21.084 / 27.084 | 24.350 / 30.925 | 1 | 32768 | unavailable |
| cholesky_factor_64_f64/numpy | 16.042 / 16.459 | 16.392 / 21.143 | unavailable | unavailable | unavailable |
| create_ones_64x64_f32/candle | 0.416 / 0.500 | 0.372 / 0.403 | 7 | 16696 | unavailable |
| create_ones_64x64_f32/faer | 0.334 / 0.458 | 0.382 / 0.428 | 1 | 16384 | unavailable |
| create_ones_64x64_f32/kernix | 0.250 / 0.292 | 0.279 / 0.296 | 1 | 16384 | unavailable |
| create_ones_64x64_f32/ndarray | 0.292 / 0.334 | 0.247 / 0.259 | 1 | 16384 | unavailable |
| create_ones_64x64_f32/numpy | 1.209 / 1.583 | 1.299 / 1.360 | unavailable | unavailable | unavailable |
| create_zeros_64x64_f32/candle | 0.417 / 0.667 | 0.420 / 0.707 | 5 | 16648 | unavailable |
| create_zeros_64x64_f32/faer | 0.292 / 0.375 | 0.286 / 0.611 | 1 | 16384 | unavailable |
| create_zeros_64x64_f32/kernix | 0.333 / 0.500 | 0.327 / 0.579 | 1 | 16384 | unavailable |
| create_zeros_64x64_f32/ndarray | 0.333 / 0.541 | 0.277 / 0.307 | 1 | 16384 | unavailable |
| create_zeros_64x64_f32/numpy | 0.541 / 0.708 | 0.596 / 0.642 | unavailable | unavailable | unavailable |
| matmul_128_f32/candle | 54.000 / 64.583 | 56.863 / 61.241 | 10 | 327768 | unavailable |
| matmul_128_f32/faer | 57.292 / 66.000 | 57.199 / 98.267 | 3 | 393224 | unavailable |
| matmul_128_f32/kernix | 48.875 / 53.458 | 53.746 / 56.040 | 3 | 163840 | unavailable |
| matmul_128_f32/kernix-accelerate | 6.500 / 8.041 | 6.274 / 22.668 | 1 | 65536 | unavailable |
| matmul_128_f32/nalgebra | 55.500 / 122.708 | 59.195 / 106.425 | 2 | 163840 | unavailable |
| matmul_128_f32/ndarray | 51.333 / 57.500 | 53.820 / 154.854 | 2 | 163840 | unavailable |
| matmul_128_f32/numpy | 5.250 / 5.959 | 5.550 / 6.149 | unavailable | unavailable | unavailable |
| matmul_128_f64/candle | 108.542 / 126.166 | 98.299 / 106.446 | 10 | 655448 | unavailable |
| matmul_128_f64/faer | 100.917 / 109.084 | 102.002 / 110.520 | 3 | 786440 | unavailable |
| matmul_128_f64/kernix | 111.833 / 131.792 | 112.215 / 124.664 | 3 | 327680 | unavailable |
| matmul_128_f64/kernix-accelerate | 16.167 / 16.375 | 15.800 / 15.936 | 1 | 131072 | unavailable |
| matmul_128_f64/nalgebra | 95.125 / 111.584 | 97.054 / 100.549 | 2 | 327680 | unavailable |
| matmul_128_f64/ndarray | 110.583 / 120.708 | 106.089 / 111.915 | 2 | 327680 | unavailable |
| matmul_128_f64/numpy | 17.291 / 17.917 | 16.952 / 47.692 | unavailable | unavailable | unavailable |
| matmul_256_f32/candle | 400.541 / 435.291 | 404.922 / 426.741 | 10 | 786520 | unavailable |
| matmul_256_f32/faer | 436.833 / 1077.625 | 511.681 / 761.095 | 3 | 1048584 | unavailable |
| matmul_256_f32/kernix | 398.125 / 446.875 | 430.807 / 542.221 | 3 | 589824 | unavailable |
| matmul_256_f32/kernix-accelerate | 42.292 / 171.875 | 65.177 / 154.198 | 1 | 262144 | unavailable |
| matmul_256_f32/nalgebra | 366.917 / 425.125 | 387.466 / 421.689 | 2 | 589824 | unavailable |
| matmul_256_f32/ndarray | 424.042 / 871.584 | 513.273 / 801.942 | 2 | 589824 | unavailable |
| matmul_256_f32/numpy | 35.834 / 55.334 | 34.018 / 45.619 | unavailable | unavailable | unavailable |
| matmul_256_f64/candle | 753.166 / 829.625 | 778.670 / 853.469 | 10 | 1572952 | unavailable |
| matmul_256_f64/faer | 871.209 / 1511.250 | 1414.047 / 2935.938 | 3 | 1572872 | unavailable |
| matmul_256_f64/kernix | 753.208 / 873.083 | 744.756 / 1065.767 | 3 | 1179648 | unavailable |
| matmul_256_f64/kernix-accelerate | 101.958 / 105.584 | 102.342 / 131.210 | 1 | 524288 | unavailable |
| matmul_256_f64/nalgebra | 804.417 / 898.792 | 759.562 / 801.401 | 2 | 1179648 | unavailable |
| matmul_256_f64/ndarray | 792.250 / 890.417 | 752.066 / 786.683 | 2 | 1179648 | unavailable |
| matmul_256_f64/numpy | 115.875 / 149.833 | 117.500 / 118.749 | unavailable | unavailable | unavailable |
| matmul_32_f64/candle | 2.542 / 3.167 | 2.589 / 3.169 | 10 | 18512 | unavailable |
| matmul_32_f64/faer | 2.458 / 3.041 | 2.577 / 5.112 | 3 | 26624 | unavailable |
| matmul_32_f64/kernix | 2.291 / 2.333 | 2.341 / 2.619 | 3 | 24576 | unavailable |
| matmul_32_f64/nalgebra | 2.125 / 2.708 | 2.204 / 2.298 | 2 | 24576 | unavailable |
| matmul_32_f64/ndarray | 2.083 / 2.584 | 2.189 / 2.496 | 2 | 24576 | unavailable |
| matmul_32_f64/numpy | 1.375 / 1.542 | 1.555 / 1.934 | unavailable | unavailable | unavailable |
| matmul_64_f32/candle | 8.833 / 10.917 | 7.608 / 93.853 | 10 | 33872 | unavailable |
| matmul_64_f32/faer | 22.250 / 22.625 | 8.932 / 10.166 | 3 | 50176 | unavailable |
| matmul_64_f32/kernix | 7.584 / 8.083 | 7.819 / 10.144 | 3 | 49152 | unavailable |
| matmul_64_f32/nalgebra | 7.166 / 8.959 | 7.728 / 10.359 | 2 | 49152 | unavailable |
| matmul_64_f32/ndarray | 6.875 / 8.709 | 7.403 / 7.858 | 2 | 49152 | unavailable |
| matmul_64_f32/numpy | 1.916 / 2.250 | 2.436 / 4.860 | unavailable | unavailable | unavailable |
| matmul_64_f64/candle | 15.167 / 18.542 | 15.660 / 16.062 | 10 | 66640 | unavailable |
| matmul_64_f64/faer | 14.208 / 15.334 | 14.396 / 16.329 | 3 | 99328 | unavailable |
| matmul_64_f64/kernix | 15.000 / 18.792 | 16.654 / 33.255 | 3 | 98304 | unavailable |
| matmul_64_f64/nalgebra | 13.625 / 17.042 | 14.386 / 15.404 | 2 | 98304 | unavailable |
| matmul_64_f64/ndarray | 41.542 / 41.958 | 20.330 / 42.589 | 2 | 98304 | unavailable |
| matmul_64_f64/numpy | 3.500 / 3.750 | 3.707 / 4.472 | unavailable | unavailable | unavailable |
| matmul_ab_128_f32/kernix | 52.916 / 68.833 | 53.737 / 60.878 | 3 | 163840 | unavailable |
| matmul_ab_128_f32/numpy | 6.000 / 6.541 | 6.105 / 13.329 | unavailable | unavailable | unavailable |
| matmul_ab_128_f64/kernix | 101.750 / 111.250 | 102.227 / 105.869 | 3 | 327680 | unavailable |
| matmul_ab_128_f64/numpy | 16.542 / 18.834 | 16.619 / 24.474 | unavailable | unavailable | unavailable |
| matmul_batched_4x16x32x16_f64/candle | 3.666 / 4.583 | 3.331 / 3.904 | 25 | 27232 | unavailable |
| matmul_batched_4x16x32x16_f64/kernix | 2.834 / 3.542 | 2.961 / 7.184 | 7 | 16392 | unavailable |
| matmul_batched_4x16x32x16_f64/numpy | 2.125 / 2.417 | 2.360 / 2.530 | unavailable | unavailable | unavailable |
| matmul_sum_backward_32x16x8_f64/candle | 6.541 / 8.416 | 6.024 / 6.943 | 113 | 27980 | unavailable |
| matmul_sum_backward_32x16x8_f64/kernix | 3.250 / 4.584 | 3.226 / 6.182 | 16 | 18376 | unavailable |
| min_norm_32x64_rank32_f64/kernix | 65.584 / 182.875 | 67.756 / 93.698 | 20 | 59136 | unavailable |
| min_norm_32x64_rank32_f64/nalgebra | 147.375 / 158.542 | 147.651 / 156.826 | 15 | 42232 | unavailable |
| min_norm_32x64_rank32_f64/numpy | 158.833 / 238.458 | 159.767 / 164.352 | unavailable | unavailable | unavailable |
| min_norm_64x32_rank16_f64/kernix | 49.208 / 61.333 | 51.080 / 53.489 | 20 | 74752 | unavailable |
| min_norm_64x32_rank16_f64/nalgebra | 72.291 / 82.042 | 73.621 / 86.465 | 15 | 41976 | unavailable |
| min_norm_64x32_rank16_f64/numpy | 84.625 / 103.875 | 87.102 / 122.051 | unavailable | unavailable | unavailable |
| parallel_matmul_into_128_f32/kernix | 55.291 / 124.042 | 96.163 / 112.117 | 2 | 196608 | unavailable |
| parallel_matmul_into_128_f64/kernix | 65.666 / 77.375 | 69.525 / 71.170 | 2 | 393216 | unavailable |
| parallel_matmul_into_256_f64/kernix | 447.583 / 817.458 | 797.673 / 825.683 | 2 | 1310720 | unavailable |
| parallel_matmul_into_64_f64/kernix | 22.625 / 42.125 | 37.195 / 10118.792 | 2 | 98304 | unavailable |
| pivoted_qr_256x64_f64/faer | 301.209 / 347.333 | 293.599 / 310.105 | 58 | 336904 | unavailable |
| pivoted_qr_256x64_f64/kernix | 578.208 / 727.333 | 615.083 / 788.888 | 14 | 559104 | unavailable |
| pivoted_qr_256x64_f64/nalgebra | 1061.250 / 2633.750 | 1069.588 / 1877.264 | 7 | 296960 | unavailable |
| pivoted_qr_96x32_f64/faer | 41.917 / 59.958 | 42.166 / 45.829 | 32 | 94728 | unavailable |
| pivoted_qr_96x32_f64/kernix | 65.208 / 85.375 | 71.132 / 206.799 | 14 | 107520 | unavailable |
| pivoted_qr_96x32_f64/nalgebra | 99.334 / 112.666 | 100.504 / 103.403 | 7 | 58368 | unavailable |
| qr_256x64_f64/faer | 326.667 / 372.875 | 334.906 / 347.066 | 83 | 335880 | unavailable |
| qr_256x64_f64/kernix | 577.750 / 862.375 | 557.123 / 646.831 | 6 | 426496 | unavailable |
| qr_256x64_f64/nalgebra | 328.375 / 341.167 | 325.985 / 367.460 | 4 | 295424 | unavailable |
| qr_256x64_f64/numpy | 401.708 / 955.166 | 419.491 / 865.632 | unavailable | unavailable | unavailable |
| qr_96x32_f64/faer | 52.416 / 153.458 | 53.037 / 79.307 | 41 | 94216 | unavailable |
| qr_96x32_f64/kernix | 61.958 / 132.292 | 64.462 / 69.503 | 6 | 82176 | unavailable |
| qr_96x32_f64/nalgebra | 44.000 / 51.166 | 46.778 / 54.409 | 4 | 57600 | unavailable |
| qr_96x32_f64/numpy | 48.959 / 57.459 | 49.082 / 51.623 | unavailable | unavailable | unavailable |
| softmax_256x64_f64/candle | 66.167 / 67.416 | 70.778 / 73.288 | 37 | 265200 | unavailable |
| softmax_256x64_f64/kernix | 76.292 / 79.917 | 79.187 / 83.828 | 1 | 131072 | unavailable |
| softmax_256x64_f64/ndarray | 58.875 / 64.833 | 57.407 / 62.133 | 1 | 131072 | unavailable |
| softmax_256x64_f64/numpy | 83.041 / 270.250 | 112.577 / 239.099 | unavailable | unavailable | unavailable |
| tensordot_16x32x8_32x8x16_f64/kernix | 63.000 / 74.083 | 68.654 / 91.454 | 9 | 2166 | unavailable |
| tensordot_16x32x8_32x8x16_f64/numpy | 5.584 / 6.042 | 5.707 / 8.624 | unavailable | unavailable | unavailable |
| training_backward_2x4x8x4_f64/candle | 20.958 / 25.542 | 20.172 / 52.343 | 476 | 21284 | unavailable |
| training_backward_2x4x8x4_f64/kernix | 3.250 / 3.292 | 2.910 / 3.203 | 23 | 4224 | unavailable |
| training_forward_2x4x8x4_f64/candle | 5.958 / 7.500 | 4.673 / 6.113 | 126 | 9568 | unavailable |
| training_forward_2x4x8x4_f64/kernix | 2.458 / 3.125 | 2.466 / 3.440 | 17 | 1032 | unavailable |
| training_record_2x4x8x4_f64/candle | 6.208 / 8.083 | 5.629 / 6.496 | 149 | 11544 | unavailable |
| training_record_2x4x8x4_f64/kernix | 3.583 / 4.500 | 3.407 / 3.812 | 22 | 6832 | unavailable |
| training_sgd_2x4x8x4_f64/kernix | 1.875 / 2.000 | 0.825 / 1.306 | 5 | 688 | unavailable |
| training_shared_backward_64x32_f64/candle | 4.250 / 5.500 | 5.746 / 10.796 | 58 | 99996 | unavailable |
| training_shared_backward_64x32_f64/kernix | 6.125 / 7.584 | 5.902 / 6.398 | 8 | 49768 | unavailable |

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
