# Clean CPU evidence: runner-windows-latest-40059566395

Measured source `18750693908110110d85d02d0f11af8d58bfae8b`, tree `75403e48db8556f95e014ec2c5b16b30f9fa781e`; dirty before/after: false.
Library source: workspace 18750693908110110d85d02d0f11af8d58bfae8b. CPU affinity: not pinned. Dtype explicitly named per workload; row-major inputs.

| Workload / library | Per-call p50 / p95 (us) | Batch-mean p50 / p95 (us) | Rust allocations | Peak additional requested bytes | Process peak RSS (MiB) |
|---|---:|---:|---:|---:|---:|
| cholesky_factor_128_f64/faer | 59.400 / 76.700 | 61.679 / 91.495 | 2 | 132096 | unavailable |
| cholesky_factor_128_f64/kernix | 222.500 / 252.600 | 226.322 / 234.688 | 1 | 131072 | unavailable |
| cholesky_factor_128_f64/nalgebra | 112.800 / 141.700 | 114.434 / 124.564 | 1 | 131072 | unavailable |
| cholesky_factor_128_f64/numpy | 113.300 / 144.800 | 115.119 / 121.163 | unavailable | unavailable | unavailable |
| cholesky_factor_256_f64/faer | 315.300 / 354.600 | 323.042 / 402.280 | 2 | 526336 | unavailable |
| cholesky_factor_256_f64/kernix | 737.700 / 1553.300 | 1043.345 / 1525.838 | 49 | 645120 | unavailable |
| cholesky_factor_256_f64/nalgebra | 842.000 / 885.300 | 843.358 / 909.973 | 1 | 524288 | unavailable |
| cholesky_factor_256_f64/numpy | 985.100 / 1044.000 | 999.591 / 1179.489 | unavailable | unavailable | unavailable |
| cholesky_factor_512_f64/faer | 3037.000 / 3495.900 | 3132.850 / 3832.333 | 2 | 2101248 | unavailable |
| cholesky_factor_512_f64/kernix | 4728.400 / 6812.600 | 4652.433 / 5013.650 | 161 | 2269184 | unavailable |
| cholesky_factor_512_f64/nalgebra | 8226.900 / 8871.700 | 8097.450 / 11147.600 | 1 | 2097152 | unavailable |
| cholesky_factor_512_f64/numpy | 6494.200 / 6960.700 | 6400.350 / 6644.600 | unavailable | unavailable | unavailable |
| cholesky_factor_64_f64/faer | 14.600 / 18.500 | 14.840 / 15.632 | 2 | 33280 | unavailable |
| cholesky_factor_64_f64/kernix | 35.900 / 69.100 | 45.756 / 73.199 | 2 | 33280 | unavailable |
| cholesky_factor_64_f64/nalgebra | 19.800 / 19.900 | 19.969 / 20.041 | 1 | 32768 | unavailable |
| cholesky_factor_64_f64/numpy | 21.500 / 23.700 | 22.428 / 36.582 | unavailable | unavailable | unavailable |
| create_ones_64x64_f32/candle | 0.700 / 0.700 | 0.696 / 0.742 | 7 | 16696 | unavailable |
| create_ones_64x64_f32/faer | 0.500 / 0.500 | 0.450 / 0.587 | 1 | 16384 | unavailable |
| create_ones_64x64_f32/kernix | 0.600 / 0.600 | 0.500 / 0.553 | 1 | 16384 | unavailable |
| create_ones_64x64_f32/ndarray | 0.400 / 0.400 | 0.388 / 0.424 | 1 | 16384 | unavailable |
| create_ones_64x64_f32/numpy | 2.100 / 2.200 | 2.236 / 2.281 | unavailable | unavailable | unavailable |
| create_zeros_64x64_f32/candle | 0.800 / 0.800 | 0.486 / 0.799 | 5 | 16648 | unavailable |
| create_zeros_64x64_f32/faer | 0.500 / 0.500 | 0.483 / 0.501 | 1 | 16384 | unavailable |
| create_zeros_64x64_f32/kernix | 0.400 / 0.400 | 0.377 / 0.386 | 1 | 16384 | unavailable |
| create_zeros_64x64_f32/ndarray | 0.300 / 0.300 | 0.283 / 0.312 | 1 | 16384 | unavailable |
| create_zeros_64x64_f32/numpy | 0.800 / 0.900 | 0.939 / 1.110 | unavailable | unavailable | unavailable |
| matmul_128_f32/candle | 75.800 / 91.400 | 67.847 / 78.926 | 10 | 65816 | unavailable |
| matmul_128_f32/faer | 72.000 / 106.900 | 53.031 / 75.996 | 1 | 65536 | unavailable |
| matmul_128_f32/kernix | 47.400 / 67.500 | 57.793 / 78.950 | 3 | 65544 | unavailable |
| matmul_128_f32/nalgebra | 53.600 / 57.700 | 55.878 / 66.959 | 2 | 163840 | unavailable |
| matmul_128_f32/ndarray | 123.800 / 159.000 | 126.772 / 196.259 | 2 | 163840 | unavailable |
| matmul_128_f32/numpy | 56.200 / 70.100 | 57.425 / 58.754 | unavailable | unavailable | unavailable |
| matmul_128_f64/candle | 141.300 / 173.800 | 143.214 / 157.227 | 10 | 131352 | unavailable |
| matmul_128_f64/faer | 120.300 / 369.100 | 119.846 / 159.740 | 1 | 131072 | unavailable |
| matmul_128_f64/kernix | 141.000 / 199.500 | 143.789 / 151.088 | 3 | 131080 | unavailable |
| matmul_128_f64/nalgebra | 116.000 / 141.100 | 117.922 / 126.036 | 2 | 327680 | unavailable |
| matmul_128_f64/ndarray | 128.400 / 168.600 | 123.282 / 190.249 | 2 | 327680 | unavailable |
| matmul_128_f64/numpy | 101.300 / 119.000 | 103.536 / 179.566 | unavailable | unavailable | unavailable |
| matmul_256_f32/candle | 431.500 / 626.000 | 437.096 / 449.813 | 10 | 262424 | unavailable |
| matmul_256_f32/faer | 415.800 / 469.700 | 424.633 / 679.773 | 1 | 262144 | unavailable |
| matmul_256_f32/kernix | 430.500 / 471.100 | 437.800 / 481.576 | 3 | 262152 | unavailable |
| matmul_256_f32/nalgebra | 396.300 / 427.600 | 402.032 / 404.420 | 2 | 589824 | unavailable |
| matmul_256_f32/ndarray | 394.800 / 425.600 | 402.540 / 426.637 | 2 | 589824 | unavailable |
| matmul_256_f32/numpy | 389.500 / 423.100 | 395.419 / 438.991 | unavailable | unavailable | unavailable |
| matmul_256_f64/candle | 860.000 / 940.600 | 862.175 / 1501.500 | 10 | 524568 | unavailable |
| matmul_256_f64/faer | 1069.100 / 1403.700 | 1121.067 / 1600.657 | 1 | 524288 | unavailable |
| matmul_256_f64/kernix | 900.100 / 1291.600 | 873.092 / 953.209 | 3 | 524296 | unavailable |
| matmul_256_f64/nalgebra | 918.300 / 975.800 | 916.845 / 957.373 | 2 | 1179648 | unavailable |
| matmul_256_f64/ndarray | 1176.400 / 1244.500 | 1184.333 / 1339.925 | 2 | 1179648 | unavailable |
| matmul_256_f64/numpy | 769.700 / 805.800 | 775.469 / 954.155 | unavailable | unavailable | unavailable |
| matmul_32_f64/candle | 2.600 / 4.400 | 2.692 / 2.725 | 10 | 8528 | unavailable |
| matmul_32_f64/faer | 2.500 / 2.600 | 2.888 / 3.802 | 1 | 8192 | unavailable |
| matmul_32_f64/kernix | 2.800 / 2.800 | 2.826 / 3.008 | 3 | 24576 | unavailable |
| matmul_32_f64/nalgebra | 2.300 / 2.400 | 2.394 / 4.064 | 2 | 24576 | unavailable |
| matmul_32_f64/ndarray | 2.400 / 2.500 | 2.411 / 4.245 | 2 | 24576 | unavailable |
| matmul_32_f64/numpy | 4.100 / 4.200 | 4.367 / 6.624 | unavailable | unavailable | unavailable |
| matmul_64_f32/candle | 7.400 / 7.600 | 7.298 / 8.547 | 10 | 16664 | unavailable |
| matmul_64_f32/faer | 7.100 / 7.200 | 7.296 / 7.328 | 1 | 16384 | unavailable |
| matmul_64_f32/kernix | 7.100 / 10.700 | 7.176 / 9.083 | 3 | 16512 | unavailable |
| matmul_64_f32/nalgebra | 29.700 / 30.900 | 9.622 / 30.255 | 2 | 49152 | unavailable |
| matmul_64_f32/ndarray | 30.200 / 48.500 | 11.090 / 39.427 | 2 | 49152 | unavailable |
| matmul_64_f32/numpy | 10.400 / 10.600 | 11.374 / 17.579 | unavailable | unavailable | unavailable |
| matmul_64_f64/candle | 14.100 / 17.400 | 14.282 / 20.526 | 10 | 33048 | unavailable |
| matmul_64_f64/faer | 14.600 / 14.800 | 14.866 / 25.049 | 1 | 32768 | unavailable |
| matmul_64_f64/kernix | 13.600 / 13.700 | 13.712 / 14.652 | 3 | 32896 | unavailable |
| matmul_64_f64/nalgebra | 58.600 / 88.500 | 33.240 / 51.055 | 2 | 98304 | unavailable |
| matmul_64_f64/ndarray | 60.900 / 76.800 | 61.846 / 86.588 | 2 | 98304 | unavailable |
| matmul_64_f64/numpy | 15.900 / 16.700 | 16.801 / 25.816 | unavailable | unavailable | unavailable |
| matmul_ab_128_f32/kernix | 75.200 / 79.800 | 76.366 / 77.582 | 3 | 65544 | unavailable |
| matmul_ab_128_f32/numpy | 56.400 / 70.200 | 57.869 / 59.253 | unavailable | unavailable | unavailable |
| matmul_ab_128_f64/kernix | 142.400 / 175.100 | 144.441 / 162.176 | 3 | 131080 | unavailable |
| matmul_ab_128_f64/numpy | 101.500 / 130.200 | 104.142 / 107.471 | unavailable | unavailable | unavailable |
| matmul_batched_4x16x32x16_f64/candle | 4.000 / 4.100 | 4.326 / 4.354 | 26 | 25440 | unavailable |
| matmul_batched_4x16x32x16_f64/kernix | 5.200 / 5.500 | 4.984 / 5.924 | 7 | 16392 | unavailable |
| matmul_batched_4x16x32x16_f64/numpy | 5.000 / 5.100 | 5.380 / 7.113 | unavailable | unavailable | unavailable |
| matmul_sum_backward_32x16x8_f64/candle | 9.700 / 10.000 | 17.313 / 36.669 | 116 | 27988 | unavailable |
| matmul_sum_backward_32x16x8_f64/kernix | 4.800 / 11.800 | 5.174 / 6.749 | 16 | 18376 | unavailable |
| min_norm_32x64_rank32_f64/kernix | 115.700 / 141.400 | 87.299 / 115.534 | 20 | 59136 | unavailable |
| min_norm_32x64_rank32_f64/nalgebra | 175.200 / 208.900 | 187.878 / 198.147 | 15 | 42232 | unavailable |
| min_norm_32x64_rank32_f64/numpy | 204.200 / 258.100 | 228.786 / 390.265 | unavailable | unavailable | unavailable |
| min_norm_64x32_rank16_f64/kernix | 101.800 / 172.500 | 66.611 / 111.793 | 20 | 74752 | unavailable |
| min_norm_64x32_rank16_f64/nalgebra | 105.100 / 132.300 | 106.067 / 110.292 | 15 | 41976 | unavailable |
| min_norm_64x32_rank16_f64/numpy | 113.300 / 156.100 | 120.535 / 177.925 | unavailable | unavailable | unavailable |
| parallel_matmul_into_128_f32/kernix | 57.500 / 84.100 | 61.088 / 65.260 | 2 | 16 | unavailable |
| parallel_matmul_into_128_f64/kernix | 84.100 / 103.900 | 99.153 / 126.016 | 2 | 16 | unavailable |
| parallel_matmul_into_256_f64/kernix | 441.000 / 771.000 | 658.112 / 804.877 | 2 | 16 | unavailable |
| parallel_matmul_into_64_f64/kernix | 39.500 / 55.200 | 60.494 / 70.787 | 2 | 98304 | unavailable |
| pivoted_qr_256x64_f64/faer | 411.800 / 623.600 | 372.678 / 424.500 | 10 | 333312 | unavailable |
| pivoted_qr_256x64_f64/kernix | 907.700 / 1386.400 | 893.033 / 1430.857 | 14 | 559104 | unavailable |
| pivoted_qr_256x64_f64/nalgebra | 915.600 / 1463.200 | 988.045 / 1658.886 | 7 | 296960 | unavailable |
| pivoted_qr_96x32_f64/faer | 78.000 / 101.400 | 60.738 / 89.315 | 10 | 68352 | unavailable |
| pivoted_qr_96x32_f64/kernix | 125.100 / 157.500 | 128.065 / 138.985 | 14 | 107520 | unavailable |
| pivoted_qr_96x32_f64/nalgebra | 107.700 / 128.100 | 109.757 / 125.416 | 7 | 58368 | unavailable |
| qr_256x64_f64/faer | 392.700 / 635.400 | 420.658 / 435.543 | 7 | 331776 | unavailable |
| qr_256x64_f64/kernix | 802.400 / 1165.400 | 809.631 / 852.475 | 6 | 426496 | unavailable |
| qr_256x64_f64/nalgebra | 565.900 / 607.100 | 586.078 / 731.693 | 4 | 295424 | unavailable |
| qr_256x64_f64/numpy | 545.200 / 597.300 | 558.739 / 593.388 | unavailable | unavailable | unavailable |
| qr_96x32_f64/faer | 86.300 / 126.700 | 67.111 / 88.462 | 7 | 67584 | unavailable |
| qr_96x32_f64/kernix | 113.300 / 146.400 | 105.722 / 151.556 | 6 | 82176 | unavailable |
| qr_96x32_f64/nalgebra | 81.900 / 100.300 | 83.487 / 114.202 | 4 | 57600 | unavailable |
| qr_96x32_f64/numpy | 152.100 / 276.700 | 80.123 / 105.211 | unavailable | unavailable | unavailable |
| softmax_256x64_f64/candle | 154.700 / 216.400 | 124.919 / 150.416 | 37 | 265200 | unavailable |
| softmax_256x64_f64/kernix | 114.100 / 140.300 | 116.642 / 124.837 | 1 | 131072 | unavailable |
| softmax_256x64_f64/ndarray | 153.900 / 183.700 | 156.431 / 165.536 | 1 | 131072 | unavailable |
| softmax_256x64_f64/numpy | 130.800 / 176.200 | 136.893 / 149.616 | unavailable | unavailable | unavailable |
| tensordot_16x32x8_32x8x16_f64/kernix | 63.900 / 89.000 | 64.895 / 67.395 | 9 | 2166 | unavailable |
| tensordot_16x32x8_32x8x16_f64/numpy | 11.900 / 12.800 | 12.274 / 12.389 | unavailable | unavailable | unavailable |
| training_backward_2x4x8x4_f64/candle | 31.500 / 32.100 | 32.844 / 58.323 | 474 | 8332 | unavailable |
| training_backward_2x4x8x4_f64/kernix | 5.700 / 5.800 | 5.924 / 7.601 | 23 | 4224 | unavailable |
| training_forward_2x4x8x4_f64/candle | 8.200 / 8.300 | 8.276 / 8.531 | 127 | 3912 | unavailable |
| training_forward_2x4x8x4_f64/kernix | 3.500 / 3.700 | 3.560 / 4.023 | 17 | 1032 | unavailable |
| training_record_2x4x8x4_f64/candle | 10.100 / 10.600 | 10.194 / 17.044 | 150 | 10712 | unavailable |
| training_record_2x4x8x4_f64/kernix | 4.800 / 4.900 | 4.706 / 6.305 | 22 | 6832 | unavailable |
| training_sgd_2x4x8x4_f64/kernix | 1.000 / 1.000 | 0.999 / 1.221 | 5 | 688 | unavailable |
| training_shared_backward_64x32_f64/candle | 59.400 / 72.600 | 7.315 / 7.994 | 58 | 100004 | unavailable |
| training_shared_backward_64x32_f64/kernix | 13.700 / 13.800 | 10.181 / 14.458 | 8 | 49768 | unavailable |

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
