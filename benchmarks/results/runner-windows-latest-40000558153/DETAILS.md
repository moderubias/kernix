# Clean CPU evidence: runner-windows-latest-40000558153

Measured source `1c53d4663a48de5a588809c2b69c3a6a5bdad7b2`, tree `d5b67d509ef5c0b81991af0933b482b13552047e`; dirty before/after: false.
Library source: workspace 1c53d4663a48de5a588809c2b69c3a6a5bdad7b2. CPU affinity: not pinned. Dtype explicitly named per workload; row-major inputs.

| Workload / library | Per-call p50 / p95 (us) | Batch-mean p50 / p95 (us) | Rust allocations | Peak additional requested bytes | Process peak RSS (MiB) |
|---|---:|---:|---:|---:|---:|
| cholesky_factor_128_f64/faer | 54.000 / 65.400 | 56.361 / 60.334 | 2 | 132096 | unavailable |
| cholesky_factor_128_f64/kernix | 166.600 / 198.600 | 173.866 / 183.627 | 1 | 131072 | unavailable |
| cholesky_factor_128_f64/nalgebra | 122.600 / 146.000 | 125.782 / 152.005 | 1 | 131072 | unavailable |
| cholesky_factor_128_f64/numpy | 121.900 / 243.300 | 124.286 / 153.765 | unavailable | unavailable | unavailable |
| cholesky_factor_256_f64/faer | 235.600 / 266.300 | 241.193 / 246.839 | 2 | 526336 | unavailable |
| cholesky_factor_256_f64/kernix | 928.100 / 1160.500 | 922.055 / 939.627 | 1 | 524288 | unavailable |
| cholesky_factor_256_f64/nalgebra | 919.200 / 973.200 | 920.664 / 962.682 | 1 | 524288 | unavailable |
| cholesky_factor_256_f64/numpy | 1008.200 / 1227.800 | 1004.700 / 1188.130 | unavailable | unavailable | unavailable |
| cholesky_factor_64_f64/faer | 17.200 / 17.600 | 13.487 / 17.327 | 2 | 33280 | unavailable |
| cholesky_factor_64_f64/kernix | 29.300 / 44.600 | 31.673 / 43.482 | 2 | 33280 | unavailable |
| cholesky_factor_64_f64/nalgebra | 19.400 / 30.600 | 19.279 / 19.639 | 1 | 32768 | unavailable |
| cholesky_factor_64_f64/numpy | 19.300 / 26.000 | 20.337 / 21.663 | unavailable | unavailable | unavailable |
| create_ones_64x64_f32/candle | 0.700 / 0.800 | 0.718 / 0.735 | 7 | 16696 | unavailable |
| create_ones_64x64_f32/faer | 0.400 / 0.500 | 0.374 / 0.380 | 1 | 16384 | unavailable |
| create_ones_64x64_f32/kernix | 0.500 / 0.600 | 0.462 / 0.463 | 1 | 16384 | unavailable |
| create_ones_64x64_f32/ndarray | 0.400 / 0.500 | 0.358 / 0.453 | 1 | 16384 | unavailable |
| create_ones_64x64_f32/numpy | 1.700 / 1.800 | 1.822 / 2.779 | unavailable | unavailable | unavailable |
| create_zeros_64x64_f32/candle | 0.500 / 0.700 | 0.511 / 0.516 | 5 | 16648 | unavailable |
| create_zeros_64x64_f32/faer | 0.500 / 0.600 | 0.512 / 0.528 | 1 | 16384 | unavailable |
| create_zeros_64x64_f32/kernix | 0.300 / 0.600 | 0.377 / 0.457 | 1 | 16384 | unavailable |
| create_zeros_64x64_f32/ndarray | 0.200 / 0.400 | 0.245 / 0.299 | 1 | 16384 | unavailable |
| create_zeros_64x64_f32/numpy | 0.700 / 0.800 | 0.766 / 1.239 | unavailable | unavailable | unavailable |
| matmul_128_f32/candle | 98.800 / 118.700 | 88.432 / 106.473 | 10 | 65816 | unavailable |
| matmul_128_f32/faer | 31.800 / 41.400 | 47.509 / 76.368 | 1 | 65536 | unavailable |
| matmul_128_f32/kernix | 54.200 / 59.800 | 70.768 / 95.535 | 3 | 65544 | unavailable |
| matmul_128_f32/nalgebra | 31.200 / 33.900 | 32.771 / 42.984 | 2 | 163840 | unavailable |
| matmul_128_f32/ndarray | 132.000 / 156.500 | 150.485 / 197.224 | 2 | 163840 | unavailable |
| matmul_128_f32/numpy | 38.200 / 47.000 | 39.952 / 42.153 | unavailable | unavailable | unavailable |
| matmul_128_f64/candle | 178.000 / 250.000 | 182.158 / 188.881 | 10 | 131352 | unavailable |
| matmul_128_f64/faer | 60.600 / 85.200 | 62.327 / 63.983 | 1 | 131072 | unavailable |
| matmul_128_f64/kernix | 106.900 / 133.100 | 109.468 / 113.552 | 3 | 131080 | unavailable |
| matmul_128_f64/nalgebra | 66.100 / 100.700 | 68.882 / 72.666 | 2 | 327680 | unavailable |
| matmul_128_f64/ndarray | 66.900 / 84.500 | 71.821 / 260.844 | 2 | 327680 | unavailable |
| matmul_128_f64/numpy | 72.200 / 106.900 | 74.710 / 77.385 | unavailable | unavailable | unavailable |
| matmul_256_f32/candle | 403.600 / 442.800 | 413.476 / 509.914 | 10 | 262424 | unavailable |
| matmul_256_f32/faer | 232.200 / 274.700 | 236.249 / 247.878 | 1 | 262144 | unavailable |
| matmul_256_f32/kernix | 403.900 / 441.500 | 411.544 / 429.796 | 3 | 262152 | unavailable |
| matmul_256_f32/nalgebra | 229.200 / 314.000 | 229.243 / 233.895 | 2 | 589824 | unavailable |
| matmul_256_f32/ndarray | 224.500 / 272.700 | 235.095 / 420.846 | 2 | 589824 | unavailable |
| matmul_256_f32/numpy | 240.900 / 334.300 | 245.880 / 254.900 | unavailable | unavailable | unavailable |
| matmul_256_f64/candle | 833.200 / 906.200 | 824.869 / 862.892 | 10 | 524568 | unavailable |
| matmul_256_f64/faer | 511.300 / 555.900 | 518.450 / 775.500 | 1 | 524288 | unavailable |
| matmul_256_f64/kernix | 827.700 / 1121.000 | 822.685 / 850.500 | 3 | 524296 | unavailable |
| matmul_256_f64/nalgebra | 524.200 / 994.400 | 525.220 / 556.489 | 2 | 1179648 | unavailable |
| matmul_256_f64/ndarray | 1056.900 / 1198.700 | 1072.560 / 1560.600 | 2 | 1179648 | unavailable |
| matmul_256_f64/numpy | 485.700 / 537.200 | 500.165 / 521.070 | unavailable | unavailable | unavailable |
| matmul_32_f64/candle | 2.700 / 2.800 | 2.761 / 2.774 | 10 | 9296 | unavailable |
| matmul_32_f64/faer | 1.900 / 2.000 | 1.922 / 1.983 | 1 | 8192 | unavailable |
| matmul_32_f64/kernix | 1.800 / 1.900 | 1.866 / 1.924 | 3 | 24576 | unavailable |
| matmul_32_f64/nalgebra | 1.400 / 1.500 | 1.509 / 2.478 | 2 | 24576 | unavailable |
| matmul_32_f64/ndarray | 1.400 / 2.300 | 1.497 / 3.568 | 2 | 24576 | unavailable |
| matmul_32_f64/numpy | 2.400 / 2.500 | 2.477 / 2.560 | unavailable | unavailable | unavailable |
| matmul_64_f32/candle | 8.100 / 8.700 | 8.039 / 8.720 | 10 | 16976 | unavailable |
| matmul_64_f32/faer | 4.700 / 4.800 | 4.672 / 4.727 | 1 | 16384 | unavailable |
| matmul_64_f32/kernix | 7.500 / 8.000 | 7.539 / 7.726 | 3 | 16896 | unavailable |
| matmul_64_f32/nalgebra | 4.800 / 5.300 | 5.937 / 26.683 | 2 | 49152 | unavailable |
| matmul_64_f32/ndarray | 35.900 / 48.100 | 8.578 / 37.967 | 2 | 49152 | unavailable |
| matmul_64_f32/numpy | 4.700 / 5.200 | 4.953 / 7.798 | unavailable | unavailable | unavailable |
| matmul_64_f64/candle | 15.600 / 16.000 | 15.551 / 15.880 | 10 | 33360 | unavailable |
| matmul_64_f64/faer | 9.600 / 9.900 | 10.147 / 17.996 | 1 | 32768 | unavailable |
| matmul_64_f64/kernix | 14.900 / 16.100 | 15.126 / 15.354 | 3 | 33280 | unavailable |
| matmul_64_f64/nalgebra | 72.100 / 98.900 | 53.832 / 75.004 | 2 | 98304 | unavailable |
| matmul_64_f64/ndarray | 74.700 / 93.400 | 75.552 / 77.641 | 2 | 98304 | unavailable |
| matmul_64_f64/numpy | 8.200 / 11.200 | 8.567 / 8.787 | unavailable | unavailable | unavailable |
| matmul_batched_4x16x32x16_f64/candle | 4.500 / 6.000 | 4.579 / 4.922 | 26 | 26208 | unavailable |
| matmul_batched_4x16x32x16_f64/kernix | 2.400 / 3.000 | 2.454 / 2.583 | 7 | 16392 | unavailable |
| matmul_batched_4x16x32x16_f64/numpy | 2.900 / 3.100 | 3.099 / 3.339 | unavailable | unavailable | unavailable |
| matmul_sum_backward_32x16x8_f64/candle | 10.300 / 10.700 | 10.485 / 10.757 | 116 | 27988 | unavailable |
| matmul_sum_backward_32x16x8_f64/kernix | 4.200 / 6.800 | 4.291 / 4.441 | 16 | 18376 | unavailable |
| min_norm_32x64_rank32_f64/kernix | 114.600 / 137.400 | 83.846 / 179.502 | 20 | 59136 | unavailable |
| min_norm_32x64_rank32_f64/nalgebra | 164.400 / 188.900 | 167.578 / 173.560 | 15 | 42232 | unavailable |
| min_norm_32x64_rank32_f64/numpy | 177.600 / 232.500 | 184.998 / 191.777 | unavailable | unavailable | unavailable |
| min_norm_64x32_rank16_f64/kernix | 102.000 / 121.400 | 65.239 / 89.325 | 20 | 74752 | unavailable |
| min_norm_64x32_rank16_f64/nalgebra | 164.100 / 185.100 | 102.601 / 150.700 | 15 | 41976 | unavailable |
| min_norm_64x32_rank16_f64/numpy | 99.400 / 139.800 | 100.299 / 101.984 | unavailable | unavailable | unavailable |
| parallel_matmul_into_128_f32/kernix | 77.800 / 106.800 | 72.793 / 81.369 | 2 | 16 | unavailable |
| parallel_matmul_into_128_f64/kernix | 114.900 / 167.600 | 130.709 / 147.618 | 2 | 16 | unavailable |
| parallel_matmul_into_256_f64/kernix | 504.100 / 875.800 | 735.893 / 914.727 | 2 | 16 | unavailable |
| parallel_matmul_into_64_f64/kernix | 106.900 / 155.400 | 108.865 / 114.717 | 2 | 98304 | unavailable |
| pivoted_qr_256x64_f64/faer | 758.700 / 1024.000 | 761.879 / 955.836 | 10 | 333312 | unavailable |
| pivoted_qr_256x64_f64/kernix | 1042.100 / 1255.000 | 993.864 / 1022.080 | 14 | 559104 | unavailable |
| pivoted_qr_256x64_f64/nalgebra | 1347.400 / 1792.700 | 1338.812 / 1351.700 | 7 | 296960 | unavailable |
| pivoted_qr_96x32_f64/faer | 147.800 / 176.400 | 134.287 / 153.029 | 10 | 68352 | unavailable |
| pivoted_qr_96x32_f64/kernix | 134.500 / 158.900 | 136.778 / 138.997 | 14 | 107520 | unavailable |
| pivoted_qr_96x32_f64/nalgebra | 142.400 / 225.900 | 149.749 / 177.098 | 7 | 58368 | unavailable |
| qr_256x64_f64/faer | 644.000 / 715.100 | 613.088 / 641.506 | 7 | 331776 | unavailable |
| qr_256x64_f64/kernix | 841.700 / 890.000 | 977.764 / 1127.560 | 6 | 426496 | unavailable |
| qr_256x64_f64/nalgebra | 734.200 / 1127.200 | 719.707 / 951.082 | 4 | 295424 | unavailable |
| qr_256x64_f64/numpy | 535.100 / 636.400 | 531.679 / 638.000 | unavailable | unavailable | unavailable |
| qr_96x32_f64/faer | 130.800 / 184.700 | 129.247 / 156.681 | 7 | 67584 | unavailable |
| qr_96x32_f64/kernix | 131.200 / 152.100 | 119.505 / 189.153 | 6 | 82176 | unavailable |
| qr_96x32_f64/nalgebra | 81.900 / 95.100 | 77.719 / 86.413 | 4 | 57600 | unavailable |
| qr_96x32_f64/numpy | 65.000 / 89.100 | 67.554 / 71.672 | unavailable | unavailable | unavailable |
| softmax_256x64_f64/candle | 205.500 / 461.000 | 166.039 / 182.251 | 37 | 265200 | unavailable |
| softmax_256x64_f64/kernix | 118.000 / 153.900 | 121.141 / 137.284 | 1 | 131072 | unavailable |
| softmax_256x64_f64/ndarray | 101.400 / 126.600 | 103.646 / 172.739 | 1 | 131072 | unavailable |
| softmax_256x64_f64/numpy | 129.200 / 277.800 | 137.377 / 197.496 | unavailable | unavailable | unavailable |
| tensordot_16x32x8_32x8x16_f64/kernix | 48.500 / 59.200 | 50.265 / 83.685 | 9 | 2166 | unavailable |
| tensordot_16x32x8_32x8x16_f64/numpy | 8.100 / 9.100 | 8.982 / 9.362 | unavailable | unavailable | unavailable |
| training_backward_2x4x8x4_f64/candle | 35.300 / 36.300 | 36.064 / 36.830 | 474 | 13100 | unavailable |
| training_backward_2x4x8x4_f64/kernix | 5.600 / 5.800 | 5.452 / 5.607 | 23 | 4480 | unavailable |
| training_forward_2x4x8x4_f64/candle | 8.800 / 9.000 | 8.889 / 9.101 | 127 | 5472 | unavailable |
| training_forward_2x4x8x4_f64/kernix | 3.400 / 3.500 | 3.505 / 3.638 | 17 | 1288 | unavailable |
| training_record_2x4x8x4_f64/candle | 11.500 / 11.800 | 11.620 / 12.664 | 150 | 10712 | unavailable |
| training_record_2x4x8x4_f64/kernix | 4.700 / 4.900 | 4.713 / 7.401 | 22 | 6832 | unavailable |
| training_sgd_2x4x8x4_f64/kernix | 0.900 / 1.000 | 0.931 / 0.940 | 5 | 688 | unavailable |
| training_shared_backward_64x32_f64/candle | 73.400 / 107.300 | 7.989 / 11.084 | 58 | 100004 | unavailable |
| training_shared_backward_64x32_f64/kernix | 10.600 / 11.500 | 10.549 / 11.634 | 8 | 49768 | unavailable |

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
