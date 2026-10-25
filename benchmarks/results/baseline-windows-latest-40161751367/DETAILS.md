# Clean CPU evidence: baseline-windows-latest-40161751367

Measured source `450df128c1ffb55a9a861230edb6b9a5ac3ef5b4`, tree `957542f0c3a8c779782e0b662134bab34e1572a9`; dirty before/after: false.
Library source: registry 0.11.0. CPU affinity: not pinned. Dtype explicitly named per workload; row-major inputs.

| Workload / library | Per-call p50 / p95 (us) | Batch-mean p50 / p95 (us) | Rust allocations | Peak additional requested bytes | Process peak RSS (MiB) |
|---|---:|---:|---:|---:|---:|
| cholesky_factor_128_f64/faer | 59.400 / 62.200 | 60.732 / 70.431 | 2 | 132096 | unavailable |
| cholesky_factor_128_f64/kernix | 223.600 / 242.700 | 228.443 / 486.662 | 1 | 131072 | unavailable |
| cholesky_factor_128_f64/nalgebra | 112.400 / 144.800 | 196.145 / 10226.650 | 1 | 131072 | unavailable |
| cholesky_factor_128_f64/numpy | 115.100 / 137.600 | 117.302 / 153.465 | unavailable | unavailable | unavailable |
| cholesky_factor_256_f64/faer | 314.100 / 353.700 | 321.225 / 354.459 | 2 | 526336 | unavailable |
| cholesky_factor_256_f64/kernix | 699.400 / 742.900 | 698.460 / 717.114 | 49 | 645120 | unavailable |
| cholesky_factor_256_f64/nalgebra | 863.400 / 946.000 | 846.050 / 1319.750 | 1 | 524288 | unavailable |
| cholesky_factor_256_f64/numpy | 943.400 / 1100.300 | 939.364 / 960.945 | unavailable | unavailable | unavailable |
| cholesky_factor_512_f64/faer | 3098.600 / 3619.300 | 3109.950 / 3476.267 | 2 | 2101248 | unavailable |
| cholesky_factor_512_f64/kernix | 4684.900 / 6388.900 | 4638.100 / 4856.133 | 161 | 2269184 | unavailable |
| cholesky_factor_512_f64/nalgebra | 8036.900 / 8789.800 | 8092.000 / 12522.300 | 1 | 2097152 | unavailable |
| cholesky_factor_512_f64/numpy | 6134.100 / 6755.600 | 6101.900 / 6558.800 | unavailable | unavailable | unavailable |
| cholesky_factor_64_f64/faer | 14.600 / 14.700 | 14.832 / 15.634 | 2 | 33280 | unavailable |
| cholesky_factor_64_f64/kernix | 36.000 / 55.600 | 58.292 / 89.018 | 2 | 33280 | unavailable |
| cholesky_factor_64_f64/nalgebra | 19.700 / 19.800 | 20.405 / 36.035 | 1 | 32768 | unavailable |
| cholesky_factor_64_f64/numpy | 21.100 / 23.700 | 21.951 / 34.671 | unavailable | unavailable | unavailable |
| create_ones_64x64_f32/candle | 0.700 / 0.800 | 0.725 / 0.839 | 7 | 16696 | unavailable |
| create_ones_64x64_f32/faer | 0.500 / 0.500 | 0.450 / 0.452 | 1 | 16384 | unavailable |
| create_ones_64x64_f32/kernix | 0.500 / 0.500 | 0.491 / 0.506 | 1 | 16384 | unavailable |
| create_ones_64x64_f32/ndarray | 0.500 / 0.600 | 0.389 / 0.475 | 1 | 16384 | unavailable |
| create_ones_64x64_f32/numpy | 2.100 / 2.200 | 2.252 / 4.350 | unavailable | unavailable | unavailable |
| create_zeros_64x64_f32/candle | 0.500 / 0.500 | 0.477 / 0.492 | 5 | 16648 | unavailable |
| create_zeros_64x64_f32/faer | 0.500 / 0.500 | 0.461 / 0.468 | 1 | 16384 | unavailable |
| create_zeros_64x64_f32/kernix | 0.400 / 0.400 | 0.380 / 0.388 | 1 | 16384 | unavailable |
| create_zeros_64x64_f32/ndarray | 0.300 / 0.300 | 0.283 / 3.082 | 1 | 16384 | unavailable |
| create_zeros_64x64_f32/numpy | 0.800 / 0.900 | 0.905 / 0.939 | unavailable | unavailable | unavailable |
| matmul_128_f32/candle | 72.800 / 92.400 | 66.595 / 77.846 | 10 | 65816 | unavailable |
| matmul_128_f32/faer | 52.200 / 55.000 | 53.607 / 91.695 | 1 | 65536 | unavailable |
| matmul_128_f32/kernix | 71.500 / 97.800 | 71.501 / 81.341 | 3 | 65544 | unavailable |
| matmul_128_f32/nalgebra | 53.800 / 54.600 | 54.550 / 58.428 | 2 | 163840 | unavailable |
| matmul_128_f32/ndarray | 119.900 / 195.900 | 122.439 / 607.514 | 2 | 163840 | unavailable |
| matmul_128_f32/numpy | 56.000 / 58.900 | 57.154 / 61.981 | unavailable | unavailable | unavailable |
| matmul_128_f64/candle | 137.600 / 162.600 | 140.094 / 144.440 | 10 | 131352 | unavailable |
| matmul_128_f64/faer | 101.300 / 125.700 | 106.810 / 117.384 | 1 | 131072 | unavailable |
| matmul_128_f64/kernix | 147.500 / 181.200 | 148.504 / 156.756 | 3 | 131080 | unavailable |
| matmul_128_f64/nalgebra | 115.900 / 168.800 | 117.835 / 118.581 | 2 | 327680 | unavailable |
| matmul_128_f64/ndarray | 114.300 / 140.400 | 116.583 / 185.794 | 2 | 327680 | unavailable |
| matmul_128_f64/numpy | 101.300 / 119.900 | 105.026 / 181.964 | unavailable | unavailable | unavailable |
| matmul_256_f32/candle | 436.500 / 608.300 | 432.812 / 461.409 | 10 | 262424 | unavailable |
| matmul_256_f32/faer | 433.700 / 705.000 | 433.621 / 464.532 | 1 | 262144 | unavailable |
| matmul_256_f32/kernix | 356.800 / 391.400 | 363.175 / 472.127 | 3 | 262152 | unavailable |
| matmul_256_f32/nalgebra | 399.000 / 429.600 | 405.916 / 452.057 | 2 | 589824 | unavailable |
| matmul_256_f32/ndarray | 398.300 / 435.900 | 402.784 / 423.825 | 2 | 589824 | unavailable |
| matmul_256_f32/numpy | 386.400 / 415.600 | 394.642 / 696.900 | unavailable | unavailable | unavailable |
| matmul_256_f64/candle | 869.700 / 1158.800 | 870.642 / 980.255 | 10 | 524568 | unavailable |
| matmul_256_f64/faer | 1092.100 / 1167.900 | 1095.060 / 1183.778 | 1 | 524288 | unavailable |
| matmul_256_f64/kernix | 855.400 / 913.400 | 859.950 / 1407.537 | 3 | 524296 | unavailable |
| matmul_256_f64/nalgebra | 907.500 / 947.400 | 909.325 / 965.636 | 2 | 1179648 | unavailable |
| matmul_256_f64/ndarray | 1151.500 / 1332.700 | 1165.422 / 1287.963 | 2 | 1179648 | unavailable |
| matmul_256_f64/numpy | 773.900 / 1326.300 | 771.654 / 787.854 | unavailable | unavailable | unavailable |
| matmul_32_f64/candle | 2.600 / 2.600 | 2.621 / 2.860 | 10 | 8528 | unavailable |
| matmul_32_f64/faer | 4.700 / 5.800 | 2.630 / 93.430 | 1 | 8192 | unavailable |
| matmul_32_f64/kernix | 2.800 / 2.800 | 2.825 / 2.909 | 3 | 24576 | unavailable |
| matmul_32_f64/nalgebra | 2.400 / 2.400 | 2.425 / 2.548 | 2 | 24576 | unavailable |
| matmul_32_f64/ndarray | 2.400 / 2.500 | 2.409 / 2.669 | 2 | 24576 | unavailable |
| matmul_32_f64/numpy | 4.100 / 4.300 | 4.389 / 7.054 | unavailable | unavailable | unavailable |
| matmul_64_f32/candle | 7.500 / 43.700 | 7.326 / 14.427 | 10 | 16664 | unavailable |
| matmul_64_f32/faer | 7.000 / 11.600 | 7.292 / 9.470 | 1 | 16384 | unavailable |
| matmul_64_f32/kernix | 6.900 / 7.000 | 7.001 / 7.541 | 3 | 16512 | unavailable |
| matmul_64_f32/nalgebra | 8.000 / 8.100 | 8.786 / 20.669 | 2 | 49152 | unavailable |
| matmul_64_f32/ndarray | 30.700 / 42.300 | 9.634 / 31.996 | 2 | 49152 | unavailable |
| matmul_64_f32/numpy | 10.300 / 10.500 | 10.697 / 11.434 | unavailable | unavailable | unavailable |
| matmul_64_f64/candle | 14.100 / 14.800 | 14.179 / 16.116 | 10 | 33048 | unavailable |
| matmul_64_f64/faer | 14.500 / 14.700 | 14.725 / 15.013 | 1 | 32768 | unavailable |
| matmul_64_f64/kernix | 13.500 / 13.600 | 13.576 / 13.730 | 3 | 32896 | unavailable |
| matmul_64_f64/nalgebra | 15.700 / 28.600 | 28.352 / 43.529 | 2 | 98304 | unavailable |
| matmul_64_f64/ndarray | 59.400 / 89.200 | 58.394 / 64.525 | 2 | 98304 | unavailable |
| matmul_64_f64/numpy | 16.000 / 17.200 | 16.418 / 16.619 | unavailable | unavailable | unavailable |
| matmul_ab_128_f32/kernix | 71.700 / 120.200 | 75.535 / 125.950 | 3 | 65544 | unavailable |
| matmul_ab_128_f32/numpy | 56.400 / 69.100 | 57.615 / 57.947 | unavailable | unavailable | unavailable |
| matmul_ab_128_f64/kernix | 140.000 / 173.100 | 142.017 / 156.302 | 3 | 131080 | unavailable |
| matmul_ab_128_f64/numpy | 101.400 / 132.500 | 104.708 / 115.074 | unavailable | unavailable | unavailable |
| matmul_batched_4x16x32x16_f32/candle | 3.200 / 3.300 | 3.475 / 4.514 | 26 | 13152 | unavailable |
| matmul_batched_4x16x32x16_f32/kernix | 2.200 / 2.300 | 2.235 / 2.259 | 7 | 8200 | unavailable |
| matmul_batched_4x16x32x16_f32/numpy | 4.800 / 4.900 | 5.031 / 6.336 | unavailable | unavailable | unavailable |
| matmul_batched_4x16x32x16_f64/candle | 4.000 / 4.100 | 4.365 / 4.678 | 26 | 25440 | unavailable |
| matmul_batched_4x16x32x16_f64/kernix | 3.400 / 3.400 | 3.441 / 3.462 | 7 | 16392 | unavailable |
| matmul_batched_4x16x32x16_f64/numpy | 5.100 / 5.200 | 5.315 / 6.637 | unavailable | unavailable | unavailable |
| matmul_sum_backward_32x16x8_f64/candle | 9.800 / 10.100 | 9.738 / 16.626 | 116 | 27988 | unavailable |
| matmul_sum_backward_32x16x8_f64/kernix | 4.900 / 5.000 | 4.437 / 6.536 | 16 | 18376 | unavailable |
| min_norm_32x64_rank32_f64/kernix | 116.900 / 157.200 | 88.210 / 123.526 | 20 | 59136 | unavailable |
| min_norm_32x64_rank32_f64/nalgebra | 164.300 / 187.100 | 168.083 / 174.009 | 15 | 42232 | unavailable |
| min_norm_32x64_rank32_f64/numpy | 204.800 / 256.100 | 216.089 / 254.195 | unavailable | unavailable | unavailable |
| min_norm_64x32_rank16_f64/kernix | 88.500 / 118.600 | 64.876 / 81.700 | 20 | 74752 | unavailable |
| min_norm_64x32_rank16_f64/nalgebra | 104.900 / 130.200 | 106.463 / 113.198 | 15 | 41976 | unavailable |
| min_norm_64x32_rank16_f64/numpy | 112.300 / 148.000 | 117.330 / 121.122 | unavailable | unavailable | unavailable |
| parallel_matmul_into_128_f32/kernix | 57.400 / 86.300 | 60.624 / 64.267 | 2 | 16 | unavailable |
| parallel_matmul_into_128_f64/kernix | 125.300 / 177.300 | 130.914 / 144.747 | 2 | 16 | unavailable |
| parallel_matmul_into_256_f64/kernix | 592.800 / 838.000 | 611.118 / 879.558 | 2 | 16 | unavailable |
| parallel_matmul_into_64_f64/kernix | 46.500 / 89.500 | 80.451 / 1460.178 | 2 | 98304 | unavailable |
| pivoted_qr_256x64_f64/faer | 397.600 / 443.200 | 363.889 / 404.076 | 10 | 333312 | unavailable |
| pivoted_qr_256x64_f64/kernix | 886.700 / 950.200 | 864.800 / 919.745 | 14 | 559104 | unavailable |
| pivoted_qr_256x64_f64/nalgebra | 925.900 / 1136.700 | 920.700 / 982.364 | 7 | 296960 | unavailable |
| pivoted_qr_96x32_f64/faer | 68.700 / 89.200 | 51.330 / 69.699 | 10 | 68352 | unavailable |
| pivoted_qr_96x32_f64/kernix | 118.900 / 154.500 | 124.148 / 137.403 | 14 | 107520 | unavailable |
| pivoted_qr_96x32_f64/nalgebra | 107.900 / 131.400 | 103.748 / 115.471 | 7 | 58368 | unavailable |
| qr_256x64_f64/faer | 406.200 / 690.000 | 406.152 / 435.529 | 7 | 331776 | unavailable |
| qr_256x64_f64/kernix | 757.100 / 818.400 | 785.131 / 866.342 | 6 | 426496 | unavailable |
| qr_256x64_f64/nalgebra | 641.300 / 713.600 | 640.300 / 658.294 | 4 | 295424 | unavailable |
| qr_256x64_f64/numpy | 545.800 / 592.300 | 551.874 / 556.778 | unavailable | unavailable | unavailable |
| qr_96x32_f64/faer | 86.600 / 115.900 | 66.601 / 97.316 | 7 | 67584 | unavailable |
| qr_96x32_f64/kernix | 101.500 / 143.300 | 104.259 / 115.977 | 6 | 82176 | unavailable |
| qr_96x32_f64/nalgebra | 82.800 / 112.100 | 114.591 / 943.336 | 4 | 57600 | unavailable |
| qr_96x32_f64/numpy | 77.900 / 107.900 | 80.292 / 83.403 | unavailable | unavailable | unavailable |
| softmax_256x64_f64/candle | 181.800 / 217.300 | 134.059 / 151.608 | 37 | 265200 | unavailable |
| softmax_256x64_f64/kernix | 114.200 / 140.000 | 116.280 / 121.488 | 1 | 131072 | unavailable |
| softmax_256x64_f64/ndarray | 153.700 / 181.100 | 156.078 / 156.945 | 1 | 131072 | unavailable |
| softmax_256x64_f64/numpy | 133.100 / 168.800 | 137.315 / 250.045 | unavailable | unavailable | unavailable |
| tensordot_16x32x8_32x8x16_f64/kernix | 63.700 / 79.100 | 65.862 / 181.607 | 9 | 2166 | unavailable |
| tensordot_16x32x8_32x8x16_f64/numpy | 12.200 / 13.700 | 12.501 / 12.860 | unavailable | unavailable | unavailable |
| training_backward_2x4x8x4_f64/candle | 31.700 / 54.300 | 32.045 / 32.556 | 474 | 8332 | unavailable |
| training_backward_2x4x8x4_f64/kernix | 5.900 / 6.000 | 5.692 / 5.780 | 23 | 4224 | unavailable |
| training_forward_2x4x8x4_f64/candle | 8.200 / 8.300 | 8.195 / 8.641 | 127 | 3912 | unavailable |
| training_forward_2x4x8x4_f64/kernix | 3.500 / 3.600 | 3.613 / 3.662 | 17 | 1032 | unavailable |
| training_record_2x4x8x4_f64/candle | 10.200 / 10.700 | 10.155 / 12.744 | 150 | 10712 | unavailable |
| training_record_2x4x8x4_f64/kernix | 4.900 / 5.100 | 4.788 / 4.855 | 22 | 6832 | unavailable |
| training_sgd_2x4x8x4_f64/kernix | 1.000 / 1.100 | 1.016 / 1.159 | 5 | 688 | unavailable |
| training_shared_backward_64x32_f64/candle | 50.100 / 66.400 | 7.019 / 7.125 | 58 | 100004 | unavailable |
| training_shared_backward_64x32_f64/kernix | 9.800 / 10.300 | 10.009 / 12.400 | 8 | 49768 | unavailable |

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
