# Clean CPU evidence: baseline-windows-latest-40059566395

Measured source `18750693908110110d85d02d0f11af8d58bfae8b`, tree `75403e48db8556f95e014ec2c5b16b30f9fa781e`; dirty before/after: false.
Library source: registry 0.10.0. CPU affinity: not pinned. Dtype explicitly named per workload; row-major inputs.

| Workload / library | Per-call p50 / p95 (us) | Batch-mean p50 / p95 (us) | Rust allocations | Peak additional requested bytes | Process peak RSS (MiB) |
|---|---:|---:|---:|---:|---:|
| cholesky_factor_128_f64/faer | 59.000 / 61.000 | 59.930 / 75.077 | 2 | 132096 | unavailable |
| cholesky_factor_128_f64/kernix | 229.300 / 256.600 | 231.648 / 240.524 | 1 | 131072 | unavailable |
| cholesky_factor_128_f64/nalgebra | 120.700 / 151.700 | 117.206 / 118.068 | 1 | 131072 | unavailable |
| cholesky_factor_128_f64/numpy | 128.600 / 165.600 | 115.354 / 119.960 | unavailable | unavailable | unavailable |
| cholesky_factor_256_f64/faer | 316.200 / 347.800 | 319.756 / 349.955 | 2 | 526336 | unavailable |
| cholesky_factor_256_f64/kernix | 991.900 / 1043.500 | 1001.318 / 1053.580 | 1 | 524288 | unavailable |
| cholesky_factor_256_f64/nalgebra | 871.800 / 906.700 | 844.825 / 1499.029 | 1 | 524288 | unavailable |
| cholesky_factor_256_f64/numpy | 953.100 / 1121.400 | 952.600 / 1009.010 | unavailable | unavailable | unavailable |
| cholesky_factor_512_f64/faer | 3164.700 / 3275.700 | 3184.075 / 3267.850 | 2 | 2101248 | unavailable |
| cholesky_factor_512_f64/kernix | 8586.800 / 8906.500 | 8641.350 / 8921.700 | 1 | 2097152 | unavailable |
| cholesky_factor_512_f64/nalgebra | 8275.000 / 8524.900 | 8153.800 / 8272.450 | 1 | 2097152 | unavailable |
| cholesky_factor_512_f64/numpy | 6804.200 / 7323.300 | 6792.950 / 7506.050 | unavailable | unavailable | unavailable |
| cholesky_factor_64_f64/faer | 14.500 / 14.600 | 14.554 / 15.477 | 2 | 33280 | unavailable |
| cholesky_factor_64_f64/kernix | 35.100 / 36.600 | 46.112 / 68.522 | 2 | 33280 | unavailable |
| cholesky_factor_64_f64/nalgebra | 21.500 / 21.600 | 21.433 / 21.945 | 1 | 32768 | unavailable |
| cholesky_factor_64_f64/numpy | 21.000 / 22.200 | 21.578 / 22.171 | unavailable | unavailable | unavailable |
| create_ones_64x64_f32/candle | 0.700 / 0.700 | 0.702 / 0.770 | 7 | 16696 | unavailable |
| create_ones_64x64_f32/faer | 0.500 / 0.500 | 0.452 / 0.488 | 1 | 16384 | unavailable |
| create_ones_64x64_f32/kernix | 0.500 / 0.500 | 0.486 / 0.727 | 1 | 16384 | unavailable |
| create_ones_64x64_f32/ndarray | 0.400 / 0.400 | 0.392 / 0.398 | 1 | 16384 | unavailable |
| create_ones_64x64_f32/numpy | 2.100 / 2.200 | 2.216 / 2.270 | unavailable | unavailable | unavailable |
| create_zeros_64x64_f32/candle | 0.500 / 0.500 | 0.484 / 0.488 | 5 | 16648 | unavailable |
| create_zeros_64x64_f32/faer | 0.500 / 0.900 | 0.452 / 0.464 | 1 | 16384 | unavailable |
| create_zeros_64x64_f32/kernix | 0.600 / 0.600 | 0.380 / 0.578 | 1 | 16384 | unavailable |
| create_zeros_64x64_f32/ndarray | 0.300 / 0.300 | 0.276 / 0.278 | 1 | 16384 | unavailable |
| create_zeros_64x64_f32/numpy | 0.800 / 0.900 | 0.920 / 0.945 | unavailable | unavailable | unavailable |
| matmul_128_f32/candle | 75.500 / 95.400 | 67.858 / 77.602 | 10 | 65816 | unavailable |
| matmul_128_f32/faer | 50.400 / 97.700 | 51.621 / 88.606 | 1 | 65536 | unavailable |
| matmul_128_f32/kernix | 76.000 / 91.600 | 77.900 / 87.916 | 3 | 65544 | unavailable |
| matmul_128_f32/nalgebra | 53.600 / 54.400 | 54.376 / 58.596 | 2 | 163840 | unavailable |
| matmul_128_f32/ndarray | 122.300 / 140.700 | 125.084 / 127.196 | 2 | 163840 | unavailable |
| matmul_128_f32/numpy | 56.100 / 67.700 | 57.252 / 65.724 | unavailable | unavailable | unavailable |
| matmul_128_f64/candle | 141.300 / 173.600 | 143.837 / 159.257 | 10 | 131352 | unavailable |
| matmul_128_f64/faer | 102.200 / 127.300 | 109.604 / 123.520 | 1 | 131072 | unavailable |
| matmul_128_f64/kernix | 140.000 / 171.700 | 142.403 / 144.753 | 3 | 131080 | unavailable |
| matmul_128_f64/nalgebra | 115.900 / 141.400 | 117.852 / 124.231 | 2 | 327680 | unavailable |
| matmul_128_f64/ndarray | 114.800 / 141.100 | 116.435 / 117.851 | 2 | 327680 | unavailable |
| matmul_128_f64/numpy | 100.200 / 125.000 | 102.968 / 118.105 | unavailable | unavailable | unavailable |
| matmul_256_f32/candle | 431.600 / 823.200 | 434.891 / 487.562 | 10 | 262424 | unavailable |
| matmul_256_f32/faer | 405.200 / 436.000 | 413.324 / 673.420 | 1 | 262144 | unavailable |
| matmul_256_f32/kernix | 355.800 / 390.000 | 362.850 / 492.852 | 3 | 262152 | unavailable |
| matmul_256_f32/nalgebra | 397.900 / 425.500 | 402.448 / 403.792 | 2 | 589824 | unavailable |
| matmul_256_f32/ndarray | 393.600 / 685.700 | 397.131 / 403.936 | 2 | 589824 | unavailable |
| matmul_256_f32/numpy | 388.300 / 418.000 | 394.362 / 432.504 | unavailable | unavailable | unavailable |
| matmul_256_f64/candle | 875.300 / 911.100 | 878.733 / 1617.029 | 10 | 524568 | unavailable |
| matmul_256_f64/faer | 1081.300 / 1109.100 | 1080.590 / 1561.000 | 1 | 524288 | unavailable |
| matmul_256_f64/kernix | 868.900 / 941.100 | 868.183 / 970.473 | 3 | 524296 | unavailable |
| matmul_256_f64/nalgebra | 908.600 / 941.400 | 906.183 / 971.245 | 2 | 1179648 | unavailable |
| matmul_256_f64/ndarray | 1179.100 / 1214.600 | 1177.311 / 1266.850 | 2 | 1179648 | unavailable |
| matmul_256_f64/numpy | 762.800 / 1131.400 | 768.664 / 780.962 | unavailable | unavailable | unavailable |
| matmul_32_f64/candle | 4.300 / 4.300 | 2.779 / 2.804 | 10 | 8528 | unavailable |
| matmul_32_f64/faer | 2.500 / 2.600 | 2.518 / 2.529 | 1 | 8192 | unavailable |
| matmul_32_f64/kernix | 2.800 / 2.800 | 2.818 / 3.014 | 3 | 24576 | unavailable |
| matmul_32_f64/nalgebra | 2.300 / 2.400 | 2.396 / 3.102 | 2 | 24576 | unavailable |
| matmul_32_f64/ndarray | 2.400 / 2.400 | 2.370 / 2.520 | 2 | 24576 | unavailable |
| matmul_32_f64/numpy | 4.100 / 4.200 | 4.316 / 4.472 | unavailable | unavailable | unavailable |
| matmul_64_f32/candle | 7.400 / 7.500 | 7.241 / 7.409 | 10 | 16664 | unavailable |
| matmul_64_f32/faer | 7.000 / 7.100 | 7.127 / 7.250 | 1 | 16384 | unavailable |
| matmul_64_f32/kernix | 6.900 / 7.000 | 6.905 / 7.518 | 3 | 16512 | unavailable |
| matmul_64_f32/nalgebra | 8.200 / 8.800 | 8.738 / 22.750 | 2 | 49152 | unavailable |
| matmul_64_f32/ndarray | 30.400 / 31.200 | 9.606 / 30.796 | 2 | 49152 | unavailable |
| matmul_64_f32/numpy | 10.400 / 10.500 | 10.657 / 18.095 | unavailable | unavailable | unavailable |
| matmul_64_f64/candle | 14.100 / 14.300 | 14.127 / 14.359 | 10 | 33048 | unavailable |
| matmul_64_f64/faer | 14.700 / 14.800 | 14.754 / 16.389 | 1 | 32768 | unavailable |
| matmul_64_f64/kernix | 13.600 / 13.700 | 13.652 / 13.851 | 3 | 32896 | unavailable |
| matmul_64_f64/nalgebra | 58.500 / 76.700 | 33.357 / 52.134 | 2 | 98304 | unavailable |
| matmul_64_f64/ndarray | 62.100 / 64.200 | 60.716 / 68.244 | 2 | 98304 | unavailable |
| matmul_64_f64/numpy | 16.000 / 16.800 | 16.368 / 16.563 | unavailable | unavailable | unavailable |
| matmul_ab_128_f32/kernix | 75.200 / 80.900 | 76.343 / 85.609 | 3 | 65544 | unavailable |
| matmul_ab_128_f32/numpy | 56.700 / 59.000 | 57.855 / 58.309 | unavailable | unavailable | unavailable |
| matmul_ab_128_f64/kernix | 141.400 / 171.800 | 144.096 / 160.343 | 3 | 131080 | unavailable |
| matmul_ab_128_f64/numpy | 102.300 / 129.200 | 104.349 / 104.842 | unavailable | unavailable | unavailable |
| matmul_batched_4x16x32x16_f64/candle | 8.200 / 8.400 | 4.537 / 5.680 | 26 | 25440 | unavailable |
| matmul_batched_4x16x32x16_f64/kernix | 3.400 / 3.500 | 3.420 / 3.434 | 7 | 16392 | unavailable |
| matmul_batched_4x16x32x16_f64/numpy | 5.000 / 5.100 | 5.211 / 5.253 | unavailable | unavailable | unavailable |
| matmul_sum_backward_32x16x8_f64/candle | 10.100 / 10.800 | 10.397 / 10.545 | 116 | 27988 | unavailable |
| matmul_sum_backward_32x16x8_f64/kernix | 4.400 / 5.000 | 4.304 / 4.345 | 16 | 18376 | unavailable |
| min_norm_32x64_rank32_f64/kernix | 114.900 / 148.200 | 86.851 / 124.341 | 20 | 59136 | unavailable |
| min_norm_32x64_rank32_f64/nalgebra | 174.800 / 202.100 | 178.393 / 179.848 | 15 | 42232 | unavailable |
| min_norm_32x64_rank32_f64/numpy | 215.000 / 407.500 | 214.440 / 381.044 | unavailable | unavailable | unavailable |
| min_norm_64x32_rank16_f64/kernix | 100.600 / 130.000 | 65.839 / 94.260 | 20 | 74752 | unavailable |
| min_norm_64x32_rank16_f64/nalgebra | 104.100 / 118.600 | 105.887 / 113.865 | 15 | 41976 | unavailable |
| min_norm_64x32_rank16_f64/numpy | 114.500 / 145.000 | 118.605 / 119.132 | unavailable | unavailable | unavailable |
| parallel_matmul_into_128_f32/kernix | 59.000 / 82.200 | 64.623 / 83.427 | 2 | 16 | unavailable |
| parallel_matmul_into_128_f64/kernix | 88.400 / 148.800 | 93.344 / 120.704 | 2 | 16 | unavailable |
| parallel_matmul_into_256_f64/kernix | 438.700 / 771.800 | 486.210 / 810.969 | 2 | 16 | unavailable |
| parallel_matmul_into_64_f64/kernix | 40.100 / 66.000 | 62.939 / 88.338 | 2 | 98304 | unavailable |
| pivoted_qr_256x64_f64/faer | 413.000 / 455.600 | 375.396 / 424.617 | 10 | 333312 | unavailable |
| pivoted_qr_256x64_f64/kernix | 936.600 / 1450.600 | 899.700 / 973.145 | 14 | 559104 | unavailable |
| pivoted_qr_256x64_f64/nalgebra | 909.000 / 946.600 | 910.864 / 983.927 | 7 | 296960 | unavailable |
| pivoted_qr_96x32_f64/faer | 78.000 / 113.200 | 50.233 / 79.554 | 10 | 68352 | unavailable |
| pivoted_qr_96x32_f64/kernix | 125.000 / 157.600 | 127.329 / 129.108 | 14 | 107520 | unavailable |
| pivoted_qr_96x32_f64/nalgebra | 108.400 / 140.600 | 111.514 / 141.190 | 7 | 58368 | unavailable |
| qr_256x64_f64/faer | 385.900 / 427.200 | 414.276 / 450.670 | 7 | 331776 | unavailable |
| qr_256x64_f64/kernix | 783.800 / 828.900 | 797.392 / 834.192 | 6 | 426496 | unavailable |
| qr_256x64_f64/nalgebra | 639.600 / 683.200 | 649.500 / 655.487 | 4 | 295424 | unavailable |
| qr_256x64_f64/numpy | 546.100 / 596.300 | 554.011 / 617.835 | unavailable | unavailable | unavailable |
| qr_96x32_f64/faer | 87.600 / 118.500 | 67.530 / 91.143 | 7 | 67584 | unavailable |
| qr_96x32_f64/kernix | 104.500 / 135.900 | 106.415 / 110.714 | 6 | 82176 | unavailable |
| qr_96x32_f64/nalgebra | 78.500 / 95.600 | 79.775 / 80.771 | 4 | 57600 | unavailable |
| qr_96x32_f64/numpy | 77.200 / 92.000 | 79.666 / 134.361 | unavailable | unavailable | unavailable |
| softmax_256x64_f64/candle | 155.800 / 198.200 | 124.443 / 148.821 | 37 | 265200 | unavailable |
| softmax_256x64_f64/kernix | 114.500 / 141.600 | 116.222 / 117.486 | 1 | 131072 | unavailable |
| softmax_256x64_f64/ndarray | 153.700 / 197.600 | 156.575 / 163.374 | 1 | 131072 | unavailable |
| softmax_256x64_f64/numpy | 132.600 / 166.800 | 136.281 / 144.536 | unavailable | unavailable | unavailable |
| tensordot_16x32x8_32x8x16_f64/kernix | 63.700 / 64.500 | 64.843 / 71.270 | 9 | 2166 | unavailable |
| tensordot_16x32x8_32x8x16_f64/numpy | 11.800 / 12.500 | 12.556 / 17.798 | unavailable | unavailable | unavailable |
| training_backward_2x4x8x4_f64/candle | 31.900 / 33.200 | 32.382 / 33.503 | 474 | 8332 | unavailable |
| training_backward_2x4x8x4_f64/kernix | 5.800 / 5.900 | 5.597 / 5.818 | 23 | 4224 | unavailable |
| training_forward_2x4x8x4_f64/candle | 10.100 / 17.400 | 8.275 / 9.050 | 127 | 3912 | unavailable |
| training_forward_2x4x8x4_f64/kernix | 3.500 / 6.800 | 5.014 / 6.091 | 17 | 1032 | unavailable |
| training_record_2x4x8x4_f64/candle | 10.200 / 10.800 | 10.144 / 10.669 | 150 | 10712 | unavailable |
| training_record_2x4x8x4_f64/kernix | 4.800 / 5.000 | 4.805 / 7.011 | 22 | 6832 | unavailable |
| training_sgd_2x4x8x4_f64/kernix | 1.000 / 1.800 | 1.010 / 1.015 | 5 | 688 | unavailable |
| training_shared_backward_64x32_f64/candle | 58.600 / 70.400 | 7.018 / 7.077 | 58 | 100004 | unavailable |
| training_shared_backward_64x32_f64/kernix | 9.600 / 10.100 | 9.380 / 10.171 | 8 | 49768 | unavailable |

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
