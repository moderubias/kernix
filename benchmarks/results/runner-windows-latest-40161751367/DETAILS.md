# Clean CPU evidence: runner-windows-latest-40161751367

Measured source `450df128c1ffb55a9a861230edb6b9a5ac3ef5b4`, tree `957542f0c3a8c779782e0b662134bab34e1572a9`; dirty before/after: false.
Library source: workspace 450df128c1ffb55a9a861230edb6b9a5ac3ef5b4. CPU affinity: not pinned. Dtype explicitly named per workload; row-major inputs.

| Workload / library | Per-call p50 / p95 (us) | Batch-mean p50 / p95 (us) | Rust allocations | Peak additional requested bytes | Process peak RSS (MiB) |
|---|---:|---:|---:|---:|---:|
| cholesky_factor_128_f64/faer | 59.200 / 67.500 | 64.667 / 95.356 | 2 | 132096 | unavailable |
| cholesky_factor_128_f64/kernix | 223.500 / 242.000 | 227.062 / 235.267 | 1 | 131072 | unavailable |
| cholesky_factor_128_f64/nalgebra | 115.200 / 143.700 | 115.348 / 139.736 | 1 | 131072 | unavailable |
| cholesky_factor_128_f64/numpy | 113.500 / 148.400 | 118.081 / 144.517 | unavailable | unavailable | unavailable |
| cholesky_factor_256_f64/faer | 316.800 / 356.200 | 321.778 / 503.540 | 2 | 526336 | unavailable |
| cholesky_factor_256_f64/kernix | 703.500 / 745.300 | 708.120 / 739.857 | 49 | 645120 | unavailable |
| cholesky_factor_256_f64/nalgebra | 832.800 / 869.200 | 848.625 / 1302.950 | 1 | 524288 | unavailable |
| cholesky_factor_256_f64/numpy | 1002.000 / 1202.700 | 1085.590 / 1222.300 | unavailable | unavailable | unavailable |
| cholesky_factor_512_f64/faer | 2951.900 / 4048.000 | 3019.850 / 3930.633 | 2 | 2101248 | unavailable |
| cholesky_factor_512_f64/kernix | 4658.800 / 6496.100 | 4577.400 / 4637.433 | 161 | 2269184 | unavailable |
| cholesky_factor_512_f64/nalgebra | 8255.400 / 9850.000 | 8187.350 / 9038.950 | 1 | 2097152 | unavailable |
| cholesky_factor_512_f64/numpy | 6514.300 / 7116.200 | 6337.800 / 6829.700 | unavailable | unavailable | unavailable |
| cholesky_factor_64_f64/faer | 14.700 / 14.800 | 14.956 / 18.779 | 2 | 33280 | unavailable |
| cholesky_factor_64_f64/kernix | 36.400 / 37.000 | 36.644 / 38.988 | 2 | 33280 | unavailable |
| cholesky_factor_64_f64/nalgebra | 19.900 / 20.800 | 20.241 / 23.715 | 1 | 32768 | unavailable |
| cholesky_factor_64_f64/numpy | 20.900 / 25.500 | 26.595 / 38.484 | unavailable | unavailable | unavailable |
| create_ones_64x64_f32/candle | 0.700 / 0.700 | 0.691 / 0.728 | 7 | 16696 | unavailable |
| create_ones_64x64_f32/faer | 0.500 / 0.500 | 0.453 / 0.469 | 1 | 16384 | unavailable |
| create_ones_64x64_f32/kernix | 0.600 / 0.700 | 0.531 / 0.598 | 1 | 16384 | unavailable |
| create_ones_64x64_f32/ndarray | 0.400 / 0.400 | 0.390 / 0.467 | 1 | 16384 | unavailable |
| create_ones_64x64_f32/numpy | 2.100 / 2.500 | 2.294 / 2.521 | unavailable | unavailable | unavailable |
| create_zeros_64x64_f32/candle | 0.500 / 0.500 | 0.531 / 0.655 | 5 | 16648 | unavailable |
| create_zeros_64x64_f32/faer | 0.500 / 0.500 | 0.518 / 0.768 | 1 | 16384 | unavailable |
| create_zeros_64x64_f32/kernix | 0.400 / 0.400 | 0.385 / 0.412 | 1 | 16384 | unavailable |
| create_zeros_64x64_f32/ndarray | 0.300 / 0.300 | 0.276 / 0.288 | 1 | 16384 | unavailable |
| create_zeros_64x64_f32/numpy | 1.300 / 1.400 | 0.896 / 1.426 | unavailable | unavailable | unavailable |
| matmul_128_f32/candle | 75.800 / 85.200 | 67.901 / 77.901 | 10 | 65816 | unavailable |
| matmul_128_f32/faer | 52.100 / 53.300 | 53.373 / 57.611 | 1 | 65536 | unavailable |
| matmul_128_f32/kernix | 47.000 / 60.600 | 56.452 / 57.739 | 3 | 65544 | unavailable |
| matmul_128_f32/nalgebra | 53.600 / 54.300 | 58.635 / 74.261 | 2 | 163840 | unavailable |
| matmul_128_f32/ndarray | 113.000 / 142.800 | 127.597 / 135.328 | 2 | 163840 | unavailable |
| matmul_128_f32/numpy | 56.300 / 68.400 | 57.545 / 64.443 | unavailable | unavailable | unavailable |
| matmul_128_f64/candle | 137.300 / 171.400 | 141.817 / 150.907 | 10 | 131352 | unavailable |
| matmul_128_f64/faer | 110.200 / 142.100 | 112.334 / 112.884 | 1 | 131072 | unavailable |
| matmul_128_f64/kernix | 155.200 / 189.600 | 158.644 / 160.822 | 3 | 131080 | unavailable |
| matmul_128_f64/nalgebra | 116.000 / 140.700 | 118.024 / 186.617 | 2 | 327680 | unavailable |
| matmul_128_f64/ndarray | 114.400 / 128.600 | 116.492 / 186.576 | 2 | 327680 | unavailable |
| matmul_128_f64/numpy | 101.200 / 128.500 | 104.013 / 115.221 | unavailable | unavailable | unavailable |
| matmul_256_f32/candle | 448.200 / 824.700 | 484.824 / 667.653 | 10 | 262424 | unavailable |
| matmul_256_f32/faer | 428.900 / 480.200 | 430.717 / 433.275 | 1 | 262144 | unavailable |
| matmul_256_f32/kernix | 358.300 / 726.800 | 366.936 / 444.839 | 3 | 262152 | unavailable |
| matmul_256_f32/nalgebra | 400.200 / 495.900 | 407.248 / 419.996 | 2 | 589824 | unavailable |
| matmul_256_f32/ndarray | 395.300 / 708.900 | 397.619 / 426.254 | 2 | 589824 | unavailable |
| matmul_256_f32/numpy | 395.100 / 467.800 | 394.846 / 397.523 | unavailable | unavailable | unavailable |
| matmul_256_f64/candle | 869.400 / 924.000 | 864.675 / 926.975 | 10 | 524568 | unavailable |
| matmul_256_f64/faer | 1104.100 / 1267.000 | 1099.850 / 1159.211 | 1 | 524288 | unavailable |
| matmul_256_f64/kernix | 859.200 / 927.600 | 853.625 / 866.983 | 3 | 524296 | unavailable |
| matmul_256_f64/nalgebra | 926.500 / 972.400 | 919.018 / 973.591 | 2 | 1179648 | unavailable |
| matmul_256_f64/ndarray | 1201.100 / 1334.900 | 1204.478 / 1343.550 | 2 | 1179648 | unavailable |
| matmul_256_f64/numpy | 749.900 / 785.600 | 754.579 / 1341.450 | unavailable | unavailable | unavailable |
| matmul_32_f64/candle | 2.600 / 2.600 | 2.770 / 4.560 | 10 | 8528 | unavailable |
| matmul_32_f64/faer | 2.500 / 2.600 | 2.466 / 2.760 | 1 | 8192 | unavailable |
| matmul_32_f64/kernix | 2.800 / 3.000 | 2.928 / 5.090 | 3 | 24576 | unavailable |
| matmul_32_f64/nalgebra | 2.400 / 2.400 | 2.496 / 4.318 | 2 | 24576 | unavailable |
| matmul_32_f64/ndarray | 2.400 / 2.400 | 2.605 / 3.064 | 2 | 24576 | unavailable |
| matmul_32_f64/numpy | 4.200 / 4.300 | 4.365 / 4.398 | unavailable | unavailable | unavailable |
| matmul_64_f32/candle | 7.400 / 7.500 | 7.331 / 12.255 | 10 | 16664 | unavailable |
| matmul_64_f32/faer | 7.000 / 7.100 | 7.223 / 7.860 | 1 | 16384 | unavailable |
| matmul_64_f32/kernix | 14.000 / 14.200 | 6.877 / 13.984 | 3 | 16512 | unavailable |
| matmul_64_f32/nalgebra | 7.800 / 7.900 | 8.950 / 21.289 | 2 | 49152 | unavailable |
| matmul_64_f32/ndarray | 29.000 / 45.900 | 9.494 / 31.765 | 2 | 49152 | unavailable |
| matmul_64_f32/numpy | 10.500 / 10.600 | 10.767 / 13.936 | unavailable | unavailable | unavailable |
| matmul_64_f64/candle | 14.200 / 14.300 | 14.196 / 15.832 | 10 | 33048 | unavailable |
| matmul_64_f64/faer | 14.600 / 14.700 | 14.994 / 25.813 | 1 | 32768 | unavailable |
| matmul_64_f64/kernix | 13.500 / 13.600 | 13.636 / 14.499 | 3 | 32896 | unavailable |
| matmul_64_f64/nalgebra | 58.500 / 73.600 | 33.496 / 52.012 | 2 | 98304 | unavailable |
| matmul_64_f64/ndarray | 61.500 / 79.400 | 58.647 / 65.145 | 2 | 98304 | unavailable |
| matmul_64_f64/numpy | 16.000 / 25.200 | 16.491 / 19.635 | unavailable | unavailable | unavailable |
| matmul_ab_128_f32/kernix | 71.200 / 85.100 | 74.690 / 85.064 | 3 | 65544 | unavailable |
| matmul_ab_128_f32/numpy | 56.400 / 86.000 | 58.063 / 100.586 | unavailable | unavailable | unavailable |
| matmul_ab_128_f64/kernix | 141.800 / 179.900 | 147.909 / 166.079 | 3 | 131080 | unavailable |
| matmul_ab_128_f64/numpy | 104.300 / 125.100 | 105.059 / 106.795 | unavailable | unavailable | unavailable |
| matmul_batched_4x16x32x16_f32/candle | 2.900 / 3.000 | 3.287 / 3.938 | 26 | 13152 | unavailable |
| matmul_batched_4x16x32x16_f32/kernix | 1.400 / 1.500 | 1.455 / 1.464 | 3 | 4104 | unavailable |
| matmul_batched_4x16x32x16_f32/numpy | 4.800 / 5.200 | 5.045 / 5.311 | unavailable | unavailable | unavailable |
| matmul_batched_4x16x32x16_f64/candle | 4.000 / 4.100 | 4.387 / 4.437 | 26 | 25440 | unavailable |
| matmul_batched_4x16x32x16_f64/kernix | 2.800 / 2.900 | 2.410 / 4.838 | 3 | 8200 | unavailable |
| matmul_batched_4x16x32x16_f64/numpy | 5.000 / 5.100 | 5.246 / 5.467 | unavailable | unavailable | unavailable |
| matmul_sum_backward_32x16x8_f64/candle | 10.200 / 19.500 | 9.832 / 11.214 | 116 | 27988 | unavailable |
| matmul_sum_backward_32x16x8_f64/kernix | 4.100 / 4.200 | 4.161 / 4.373 | 15 | 18376 | unavailable |
| min_norm_32x64_rank32_f64/kernix | 118.200 / 151.000 | 87.869 / 121.814 | 20 | 59136 | unavailable |
| min_norm_32x64_rank32_f64/nalgebra | 164.800 / 193.700 | 167.638 / 168.155 | 15 | 42232 | unavailable |
| min_norm_32x64_rank32_f64/numpy | 205.700 / 247.900 | 216.628 / 233.370 | unavailable | unavailable | unavailable |
| min_norm_64x32_rank16_f64/kernix | 94.900 / 125.600 | 65.016 / 82.769 | 20 | 74752 | unavailable |
| min_norm_64x32_rank16_f64/nalgebra | 105.300 / 132.600 | 105.708 / 181.775 | 15 | 41976 | unavailable |
| min_norm_64x32_rank16_f64/numpy | 114.600 / 234.300 | 123.015 / 188.921 | unavailable | unavailable | unavailable |
| parallel_matmul_into_128_f32/kernix | 58.600 / 89.600 | 63.596 / 74.694 | 2 | 16 | unavailable |
| parallel_matmul_into_128_f64/kernix | 85.000 / 126.600 | 92.053 / 124.633 | 2 | 16 | unavailable |
| parallel_matmul_into_256_f64/kernix | 501.600 / 787.100 | 724.986 / 820.562 | 2 | 16 | unavailable |
| parallel_matmul_into_64_f64/kernix | 32.700 / 61.300 | 64.129 / 80.716 | 2 | 98304 | unavailable |
| pivoted_qr_256x64_f64/faer | 413.300 / 471.100 | 397.281 / 621.024 | 10 | 333312 | unavailable |
| pivoted_qr_256x64_f64/kernix | 918.800 / 1479.900 | 1170.644 / 10716.900 | 14 | 559104 | unavailable |
| pivoted_qr_256x64_f64/nalgebra | 923.600 / 1243.000 | 1076.710 / 1770.367 | 7 | 296960 | unavailable |
| pivoted_qr_96x32_f64/faer | 78.100 / 108.300 | 57.040 / 81.790 | 10 | 68352 | unavailable |
| pivoted_qr_96x32_f64/kernix | 125.700 / 217.000 | 131.786 / 209.227 | 14 | 107520 | unavailable |
| pivoted_qr_96x32_f64/nalgebra | 105.500 / 136.600 | 105.421 / 112.227 | 7 | 58368 | unavailable |
| qr_256x64_f64/faer | 391.700 / 463.200 | 415.068 / 475.186 | 7 | 331776 | unavailable |
| qr_256x64_f64/kernix | 824.200 / 1320.500 | 778.369 / 1045.390 | 6 | 426496 | unavailable |
| qr_256x64_f64/nalgebra | 569.700 / 734.000 | 583.606 / 645.156 | 4 | 295424 | unavailable |
| qr_256x64_f64/numpy | 545.900 / 597.100 | 563.072 / 599.188 | unavailable | unavailable | unavailable |
| qr_96x32_f64/faer | 84.300 / 116.900 | 71.092 / 87.435 | 7 | 67584 | unavailable |
| qr_96x32_f64/kernix | 100.600 / 122.400 | 108.712 / 119.690 | 6 | 82176 | unavailable |
| qr_96x32_f64/nalgebra | 78.300 / 96.000 | 78.399 / 83.605 | 4 | 57600 | unavailable |
| qr_96x32_f64/numpy | 76.800 / 110.400 | 105.323 / 167.050 | unavailable | unavailable | unavailable |
| softmax_256x64_f64/candle | 187.100 / 305.800 | 159.949 / 231.493 | 37 | 265200 | unavailable |
| softmax_256x64_f64/kernix | 114.400 / 142.600 | 116.847 / 227.700 | 1 | 131072 | unavailable |
| softmax_256x64_f64/ndarray | 153.700 / 187.800 | 156.984 / 174.428 | 1 | 131072 | unavailable |
| softmax_256x64_f64/numpy | 133.100 / 163.800 | 136.772 / 142.383 | unavailable | unavailable | unavailable |
| tensordot_16x32x8_32x8x16_f64/kernix | 63.800 / 65.100 | 64.999 / 68.796 | 9 | 2166 | unavailable |
| tensordot_16x32x8_32x8x16_f64/numpy | 12.100 / 14.000 | 12.757 / 20.301 | unavailable | unavailable | unavailable |
| training_backward_2x4x8x4_f64/candle | 32.700 / 34.300 | 33.118 / 43.702 | 474 | 8332 | unavailable |
| training_backward_2x4x8x4_f64/kernix | 5.800 / 5.900 | 5.671 / 5.872 | 23 | 4224 | unavailable |
| training_forward_2x4x8x4_f64/candle | 8.400 / 8.800 | 9.391 / 10.109 | 127 | 3912 | unavailable |
| training_forward_2x4x8x4_f64/kernix | 3.500 / 3.700 | 3.627 / 3.675 | 17 | 1032 | unavailable |
| training_record_2x4x8x4_f64/candle | 10.100 / 11.300 | 10.737 / 14.457 | 150 | 10712 | unavailable |
| training_record_2x4x8x4_f64/kernix | 5.000 / 5.100 | 4.966 / 8.648 | 22 | 6832 | unavailable |
| training_sgd_2x4x8x4_f64/kernix | 1.000 / 1.000 | 0.997 / 1.073 | 5 | 688 | unavailable |
| training_shared_backward_64x32_f64/candle | 50.700 / 70.900 | 7.022 / 7.195 | 58 | 100004 | unavailable |
| training_shared_backward_64x32_f64/kernix | 9.800 / 10.300 | 10.048 / 11.105 | 8 | 49768 | unavailable |

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
