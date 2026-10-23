# Clean CPU evidence: runner-windows-latest-39980084433

Measured source `9b7ef492c99bf2202e7eb3b07a79ef8f108991c7`, tree `3bfcaf4212c3091bc37e737a1d4d86760b373f94`; dirty before/after: false.
Library source: workspace 9b7ef492c99bf2202e7eb3b07a79ef8f108991c7. CPU affinity: not pinned. Dtype explicitly named per workload; row-major inputs.

| Workload / library | Per-call p50 / p95 (us) | Batch-mean p50 / p95 (us) | Rust allocations | Peak additional requested bytes | Process peak RSS (MiB) |
|---|---:|---:|---:|---:|---:|
| cholesky_factor_128_f64/faer | 48.400 / 51.800 | 49.100 / 54.808 | 2 | 132096 | unavailable |
| cholesky_factor_128_f64/kernix | 139.600 / 156.200 | 141.594 / 157.272 | 1 | 131072 | unavailable |
| cholesky_factor_128_f64/nalgebra | 94.400 / 107.100 | 95.652 / 135.599 | 1 | 131072 | unavailable |
| cholesky_factor_128_f64/numpy | 86.000 / 90.600 | 87.466 / 110.015 | unavailable | unavailable | unavailable |
| cholesky_factor_256_f64/faer | 264.700 / 286.200 | 268.750 / 272.886 | 2 | 526336 | unavailable |
| cholesky_factor_256_f64/kernix | 809.900 / 842.400 | 809.638 / 822.608 | 1 | 524288 | unavailable |
| cholesky_factor_256_f64/nalgebra | 733.700 / 1069.400 | 732.021 / 755.521 | 1 | 524288 | unavailable |
| cholesky_factor_256_f64/numpy | 742.000 / 789.100 | 749.536 / 781.677 | unavailable | unavailable | unavailable |
| cholesky_factor_64_f64/faer | 12.100 / 12.200 | 12.133 / 12.274 | 2 | 33280 | unavailable |
| cholesky_factor_64_f64/kernix | 22.300 / 22.600 | 22.538 / 23.125 | 2 | 33280 | unavailable |
| cholesky_factor_64_f64/nalgebra | 16.900 / 17.000 | 17.224 / 25.090 | 1 | 32768 | unavailable |
| cholesky_factor_64_f64/numpy | 18.300 / 18.900 | 18.803 / 22.946 | unavailable | unavailable | unavailable |
| create_ones_64x64_f32/candle | 0.600 / 0.600 | 0.580 / 0.588 | 7 | 16696 | unavailable |
| create_ones_64x64_f32/faer | 0.400 / 0.400 | 0.382 / 0.388 | 1 | 16384 | unavailable |
| create_ones_64x64_f32/kernix | 0.400 / 0.500 | 0.420 / 0.430 | 1 | 16384 | unavailable |
| create_ones_64x64_f32/ndarray | 0.300 / 0.400 | 0.330 / 0.333 | 1 | 16384 | unavailable |
| create_ones_64x64_f32/numpy | 1.600 / 1.700 | 1.730 / 1.752 | unavailable | unavailable | unavailable |
| create_zeros_64x64_f32/candle | 0.400 / 0.400 | 0.398 / 0.403 | 5 | 16648 | unavailable |
| create_zeros_64x64_f32/faer | 0.400 / 0.400 | 0.340 / 0.374 | 1 | 16384 | unavailable |
| create_zeros_64x64_f32/kernix | 0.300 / 0.300 | 0.299 / 0.328 | 1 | 16384 | unavailable |
| create_zeros_64x64_f32/ndarray | 0.200 / 0.300 | 0.216 / 0.218 | 1 | 16384 | unavailable |
| create_zeros_64x64_f32/numpy | 0.600 / 0.700 | 0.653 / 0.676 | unavailable | unavailable | unavailable |
| matmul_128_f32/candle | 68.900 / 72.300 | 60.383 / 69.963 | 10 | 65816 | unavailable |
| matmul_128_f32/faer | 39.200 / 43.500 | 39.602 / 39.913 | 1 | 65536 | unavailable |
| matmul_128_f32/kernix | 40.500 / 41.500 | 49.622 / 50.743 | 3 | 65544 | unavailable |
| matmul_128_f32/nalgebra | 44.500 / 48.300 | 45.181 / 47.667 | 2 | 163840 | unavailable |
| matmul_128_f32/ndarray | 110.000 / 124.700 | 110.619 / 123.155 | 2 | 163840 | unavailable |
| matmul_128_f32/numpy | 46.900 / 50.600 | 47.550 / 56.672 | unavailable | unavailable | unavailable |
| matmul_128_f64/candle | 133.200 / 153.000 | 134.961 / 142.520 | 10 | 131352 | unavailable |
| matmul_128_f64/faer | 76.800 / 97.800 | 77.984 / 86.330 | 1 | 131072 | unavailable |
| matmul_128_f64/kernix | 80.300 / 91.400 | 81.292 / 81.821 | 3 | 131080 | unavailable |
| matmul_128_f64/nalgebra | 83.100 / 97.000 | 84.089 / 84.507 | 2 | 327680 | unavailable |
| matmul_128_f64/ndarray | 83.400 / 90.600 | 84.975 / 85.644 | 2 | 327680 | unavailable |
| matmul_128_f64/numpy | 102.100 / 125.700 | 90.567 / 92.685 | unavailable | unavailable | unavailable |
| matmul_256_f32/candle | 303.700 / 334.400 | 309.618 / 313.684 | 10 | 262424 | unavailable |
| matmul_256_f32/faer | 301.300 / 321.500 | 304.518 / 307.794 | 1 | 262144 | unavailable |
| matmul_256_f32/kernix | 302.800 / 328.600 | 307.085 / 309.048 | 3 | 262152 | unavailable |
| matmul_256_f32/nalgebra | 322.200 / 345.300 | 326.358 / 329.700 | 2 | 589824 | unavailable |
| matmul_256_f32/ndarray | 322.600 / 346.200 | 327.061 / 352.548 | 2 | 589824 | unavailable |
| matmul_256_f32/numpy | 334.700 / 356.000 | 338.843 / 350.003 | unavailable | unavailable | unavailable |
| matmul_256_f64/candle | 608.100 / 649.400 | 618.876 / 659.188 | 10 | 524568 | unavailable |
| matmul_256_f64/faer | 612.400 / 643.700 | 619.206 / 622.888 | 1 | 524288 | unavailable |
| matmul_256_f64/kernix | 608.400 / 1127.500 | 616.847 / 621.318 | 3 | 524296 | unavailable |
| matmul_256_f64/nalgebra | 638.700 / 676.700 | 648.275 / 688.280 | 2 | 1179648 | unavailable |
| matmul_256_f64/ndarray | 891.700 / 946.900 | 933.173 / 981.000 | 2 | 1179648 | unavailable |
| matmul_256_f64/numpy | 672.200 / 698.200 | 678.613 / 908.042 | unavailable | unavailable | unavailable |
| matmul_32_f64/candle | 2.100 / 3.200 | 2.143 / 2.504 | 10 | 8784 | unavailable |
| matmul_32_f64/faer | 1.600 / 1.600 | 1.587 / 1.695 | 1 | 8192 | unavailable |
| matmul_32_f64/kernix | 2.200 / 2.300 | 2.250 / 2.271 | 3 | 24576 | unavailable |
| matmul_32_f64/nalgebra | 1.900 / 2.500 | 1.948 / 1.968 | 2 | 24576 | unavailable |
| matmul_32_f64/ndarray | 1.900 / 2.000 | 1.927 / 1.947 | 2 | 24576 | unavailable |
| matmul_32_f64/numpy | 2.700 / 2.800 | 2.819 / 2.954 | unavailable | unavailable | unavailable |
| matmul_64_f32/candle | 6.200 / 6.200 | 6.073 / 6.143 | 10 | 16720 | unavailable |
| matmul_64_f32/faer | 5.500 / 5.600 | 5.587 / 5.608 | 1 | 16384 | unavailable |
| matmul_64_f32/kernix | 5.700 / 5.700 | 5.715 / 5.783 | 3 | 16640 | unavailable |
| matmul_64_f32/nalgebra | 26.100 / 27.200 | 7.613 / 21.854 | 2 | 49152 | unavailable |
| matmul_64_f32/ndarray | 26.300 / 27.400 | 9.447 / 57.612 | 2 | 49152 | unavailable |
| matmul_64_f32/numpy | 6.100 / 6.400 | 6.262 / 6.282 | unavailable | unavailable | unavailable |
| matmul_64_f64/candle | 11.800 / 11.900 | 11.729 / 11.922 | 10 | 33104 | unavailable |
| matmul_64_f64/faer | 10.600 / 10.700 | 10.711 / 11.752 | 1 | 32768 | unavailable |
| matmul_64_f64/kernix | 11.400 / 11.600 | 11.489 / 11.627 | 3 | 33024 | unavailable |
| matmul_64_f64/nalgebra | 50.300 / 60.500 | 21.024 / 39.253 | 2 | 98304 | unavailable |
| matmul_64_f64/ndarray | 50.900 / 61.800 | 44.717 / 53.160 | 2 | 98304 | unavailable |
| matmul_64_f64/numpy | 10.800 / 11.000 | 11.049 / 11.125 | unavailable | unavailable | unavailable |
| matmul_batched_4x16x32x16_f64/candle | 3.400 / 3.600 | 3.637 / 3.778 | 26 | 25696 | unavailable |
| matmul_batched_4x16x32x16_f64/kernix | 2.900 / 2.900 | 2.918 / 2.946 | 7 | 16392 | unavailable |
| matmul_batched_4x16x32x16_f64/numpy | 2.900 / 3.000 | 3.090 / 3.118 | unavailable | unavailable | unavailable |
| matmul_sum_backward_32x16x8_f64/candle | 8.000 / 8.500 | 8.112 / 8.191 | 116 | 27988 | unavailable |
| matmul_sum_backward_32x16x8_f64/kernix | 5.600 / 6.200 | 3.630 / 4.792 | 16 | 18376 | unavailable |
| min_norm_32x64_rank32_f64/kernix | 153.400 / 171.100 | 92.739 / 106.253 | 28 | 102144 | unavailable |
| min_norm_32x64_rank32_f64/nalgebra | 138.700 / 158.300 | 140.506 / 142.194 | 15 | 42232 | unavailable |
| min_norm_32x64_rank32_f64/numpy | 159.000 / 178.600 | 162.295 / 166.389 | unavailable | unavailable | unavailable |
| min_norm_64x32_rank16_f64/kernix | 90.800 / 107.400 | 59.008 / 67.807 | 28 | 74752 | unavailable |
| min_norm_64x32_rank16_f64/nalgebra | 83.800 / 151.200 | 84.214 / 90.313 | 15 | 41976 | unavailable |
| min_norm_64x32_rank16_f64/numpy | 85.600 / 100.800 | 87.062 / 87.872 | unavailable | unavailable | unavailable |
| pivoted_qr_256x64_f64/faer | 320.200 / 346.300 | 282.392 / 318.538 | 10 | 333312 | unavailable |
| pivoted_qr_256x64_f64/kernix | 752.700 / 790.000 | 736.357 / 764.493 | 14 | 559104 | unavailable |
| pivoted_qr_256x64_f64/nalgebra | 863.400 / 1007.200 | 856.242 / 865.258 | 7 | 296960 | unavailable |
| pivoted_qr_96x32_f64/faer | 61.000 / 76.000 | 34.002 / 61.967 | 10 | 68352 | unavailable |
| pivoted_qr_96x32_f64/kernix | 107.600 / 125.000 | 110.152 / 120.195 | 14 | 107520 | unavailable |
| pivoted_qr_96x32_f64/nalgebra | 97.000 / 112.200 | 94.200 / 101.649 | 7 | 58368 | unavailable |
| qr_256x64_f64/faer | 288.300 / 322.100 | 307.067 / 314.644 | 7 | 331776 | unavailable |
| qr_256x64_f64/kernix | 666.600 / 712.400 | 670.133 / 677.987 | 6 | 426496 | unavailable |
| qr_256x64_f64/nalgebra | 449.200 / 488.000 | 460.905 / 561.044 | 4 | 295424 | unavailable |
| qr_256x64_f64/numpy | 376.800 / 424.700 | 384.712 / 492.743 | unavailable | unavailable | unavailable |
| qr_96x32_f64/faer | 63.800 / 67.400 | 46.293 / 64.768 | 7 | 67584 | unavailable |
| qr_96x32_f64/kernix | 98.000 / 123.300 | 84.933 / 91.963 | 6 | 82176 | unavailable |
| qr_96x32_f64/nalgebra | 69.200 / 71.800 | 63.583 / 71.276 | 4 | 57600 | unavailable |
| qr_96x32_f64/numpy | 55.500 / 67.600 | 57.262 / 59.257 | unavailable | unavailable | unavailable |
| softmax_256x64_f64/candle | 143.500 / 167.600 | 95.333 / 103.907 | 37 | 265200 | unavailable |
| softmax_256x64_f64/kernix | 79.500 / 91.700 | 80.570 / 81.348 | 1 | 131072 | unavailable |
| softmax_256x64_f64/ndarray | 73.200 / 84.700 | 74.341 / 74.892 | 1 | 131072 | unavailable |
| softmax_256x64_f64/numpy | 92.500 / 113.700 | 94.021 / 95.654 | unavailable | unavailable | unavailable |
| training_backward_2x4x8x4_f64/candle | 25.500 / 32.700 | 25.487 / 26.023 | 474 | 9004 | unavailable |
| training_backward_2x4x8x4_f64/kernix | 4.700 / 5.000 | 4.752 / 4.877 | 23 | 4480 | unavailable |
| training_forward_2x4x8x4_f64/candle | 6.600 / 6.700 | 6.540 / 10.687 | 127 | 3912 | unavailable |
| training_forward_2x4x8x4_f64/kernix | 2.800 / 2.900 | 2.840 / 3.514 | 17 | 1288 | unavailable |
| training_record_2x4x8x4_f64/candle | 8.300 / 8.700 | 8.442 / 8.550 | 150 | 10712 | unavailable |
| training_record_2x4x8x4_f64/kernix | 3.800 / 3.900 | 3.867 / 3.937 | 22 | 6832 | unavailable |
| training_sgd_2x4x8x4_f64/kernix | 0.800 / 0.900 | 0.789 / 0.801 | 5 | 688 | unavailable |

## Method and limits

- Per-call quantiles use nearest ranks over 256 individual warmed calls; batch quantiles use 15 means over separate 10-ms batches. They are separate distributions. Raw samples remain in raw.json.
- Eight warmup calls follow validation. Timing includes allocation, dispatch, computation and output destruction. The first call field excludes output destruction. Inputs and library/runtime imports are prepared outside timing.
- Three extra processes record their first operation calls. These are first-use observations, not cache-flushed or device-startup latency. Timer-pair overhead is reported without subtraction; tiny calls are clock-sensitive.
- ndarray and Candle softmax are explicit stable compositions, not claims about a native softmax API.
- Constructor cases allocate owned 64x64 f32 tensors. Pre-0.6 registry baselines use full(value), explicitly marked in metadata; current workspace runs use zeros/ones. No nested literal reinterpretation is involved.
- All GEMM inputs are C-order and match elementwise. Native output layout differs by library. QR materializes thin Q and R in every implementation. Cholesky returns reusable factors (or NumPy L); no extra factor materialization is timed.
- Pivoted QR materializes thin Q/R and an owned permutation in Kernix, nalgebra and faer. Minimum-norm cases include factorization and four RHS: Kernix uses pivoted QR plus right QR with relative cutoff 1e-12, NumPy uses native lstsq with rcond=1e-12, and nalgebra uses native SVD plus solve with absolute cutoff 1e-10. These algorithms have different costs and truncation policies. Fixtures have a wide rank gap; independent null-space orthogonality and normal-residual checks must both pass 2e-10 before timing. Results do not establish equivalence near a rank threshold or general SVD performance.
- Kernix checks finite values/symmetry. Some competitors validate a narrower contract. No finite-data precondition check was removed to improve timing. Accuracy results and budgets are retained per worker.
- The backward case uses an actual reverse-mode graph in Kernix and Candle for sum(A @ W), with A constant and W trainable. Both copy input/weight data and returned gradients; no manual derivative is used as a framework ranking.
- Allocations are measured in a separately compiled instrumented binary after warmup, outside all timing loops. Counts cover Rust global-allocator requests, not native allocator internals, stack memory or all NumPy allocations. NumPy allocation counts are unavailable, not zero.
- Native engines may retain thread-local packing buffers. Warmed allocation probes exclude first-use allocations and retained scratch; first-use timing and process RSS are separate observations. See the measured library numerical contract for that version.
- Requested-byte peak is additional live Rust allocation during one call, including output; it is not scratch-only memory or peak RSS. RSS is the process high-water mark including fixtures, validation and runtime, read from Linux VmHWM. Other platforms may report unavailable.
- The published cases are allocation-inclusive CPU APIs and graph workloads. Pure internal kernel-only and end-to-end application/model latency are not measured. No workload result implies a complete application frame rate.
- faer is explicitly serial; Rayon and BLAS thread counts are one. No GPU comparison. Python/NumPy timings include Python dispatch. Backend and dependency versions/configuration, governor and RAM are recorded.
- Training stages use batched matmul + bias + layer_norm + tanh + step-two feature slice + MSE (Candle index_select). Forward excludes graph recording; record includes parameter/input/target snapshots and eager forward/loss; backward reuses a prepared graph and returns fresh derivatives. Allocation probes measure the corresponding stage, including graph values/scratch, not graph-container-only bytes. SGD uses fixed precomputed real gradients and includes transactional validation/parameter updates; it is not an end-to-end converging training loop. Candle normalization is an explicit population-statistics composition; its checks/rounding differ. Historical registry baselines without these Tape operations omit the stages, recorded in metadata, rather than substitute manual derivatives.
- Host load/frequency/cache effects are uncontrolled. Report comparator wins and losses; no universal ranking follows. The source archive comes from committed bytes, with result directories excluded. Library dependencies are pinned by the saved driver lockfile.
