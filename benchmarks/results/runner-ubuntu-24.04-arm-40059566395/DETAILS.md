# Clean CPU evidence: runner-ubuntu-24.04-arm-40059566395

Measured source `18750693908110110d85d02d0f11af8d58bfae8b`, tree `75403e48db8556f95e014ec2c5b16b30f9fa781e`; dirty before/after: false.
Library source: workspace 18750693908110110d85d02d0f11af8d58bfae8b. CPU affinity: 0. Dtype explicitly named per workload; row-major inputs.

| Workload / library | Per-call p50 / p95 (us) | Batch-mean p50 / p95 (us) | Rust allocations | Peak additional requested bytes | Process peak RSS (MiB) |
|---|---:|---:|---:|---:|---:|
| cholesky_factor_128_f64/faer | 64.950 / 67.446 | 65.172 / 65.215 | 16 | 148736 | 3.39 |
| cholesky_factor_128_f64/kernix | 280.005 / 283.741 | 278.340 / 280.574 | 1 | 131072 | 3.07 |
| cholesky_factor_128_f64/nalgebra | 128.682 / 131.698 | 130.451 / 131.205 | 1 | 131072 | 2.95 |
| cholesky_factor_128_f64/numpy | 86.549 / 101.524 | 88.295 / 88.706 | unavailable | unavailable | 35.42 |
| cholesky_factor_256_f64/faer | 407.640 / 420.423 | 409.629 / 410.827 | 76 | 592000 | 4.30 |
| cholesky_factor_256_f64/kernix | 771.122 / 781.162 | 770.851 / 774.616 | 49 | 645120 | 4.57 |
| cholesky_factor_256_f64/nalgebra | 1035.745 / 1045.465 | 1035.952 / 1042.947 | 1 | 524288 | 3.70 |
| cholesky_factor_256_f64/numpy | 767.587 / 785.875 | 762.149 / 766.216 | unavailable | unavailable | 38.28 |
| cholesky_factor_512_f64/faer | 2673.472 / 2697.855 | 2672.724 / 2681.324 | 388 | 2297864 | 9.25 |
| cholesky_factor_512_f64/kernix | 4366.255 / 4397.902 | 4362.458 / 4407.027 | 161 | 2269184 | 9.16 |
| cholesky_factor_512_f64/nalgebra | 9204.768 / 9313.076 | 9132.874 / 9293.364 | 1 | 2097152 | 6.69 |
| cholesky_factor_512_f64/numpy | 4027.441 / 4134.093 | 3982.555 / 4018.644 | unavailable | unavailable | 47.78 |
| cholesky_factor_64_f64/faer | 13.255 / 13.280 | 13.294 / 13.329 | 2 | 33280 | 2.85 |
| cholesky_factor_64_f64/kernix | 27.703 / 28.175 | 27.554 / 27.704 | 2 | 33280 | 2.85 |
| cholesky_factor_64_f64/nalgebra | 20.311 / 20.927 | 20.615 / 20.751 | 1 | 32768 | 2.74 |
| cholesky_factor_64_f64/numpy | 18.039 / 19.079 | 18.270 / 18.385 | unavailable | unavailable | 34.66 |
| create_ones_64x64_f32/candle | 0.344 / 0.352 | 0.348 / 0.349 | 7 | 16696 | 3.17 |
| create_ones_64x64_f32/faer | 0.240 / 0.248 | 0.234 / 0.235 | 1 | 16384 | 2.73 |
| create_ones_64x64_f32/kernix | 0.248 / 0.272 | 0.260 / 0.263 | 1 | 16384 | 2.65 |
| create_ones_64x64_f32/ndarray | 0.208 / 0.216 | 0.216 / 0.221 | 1 | 16384 | 2.64 |
| create_ones_64x64_f32/numpy | 1.520 / 1.568 | 1.580 / 1.589 | unavailable | unavailable | 33.54 |
| create_zeros_64x64_f32/candle | 0.232 / 0.248 | 0.245 / 0.253 | 5 | 16648 | 2.98 |
| create_zeros_64x64_f32/faer | 0.360 / 0.368 | 0.311 / 0.351 | 1 | 16384 | 2.72 |
| create_zeros_64x64_f32/kernix | 0.200 / 0.216 | 0.213 / 0.236 | 1 | 16384 | 2.71 |
| create_zeros_64x64_f32/ndarray | 0.136 / 0.136 | 0.145 / 0.160 | 1 | 16384 | 2.64 |
| create_zeros_64x64_f32/numpy | 0.512 / 0.544 | 0.544 / 0.566 | unavailable | unavailable | 33.55 |
| matmul_128_f32/candle | 85.788 / 88.037 | 85.996 / 87.368 | 10 | 131160 | 3.71 |
| matmul_128_f32/faer | 87.141 / 89.092 | 87.377 / 87.594 | 3 | 196616 | 3.21 |
| matmul_128_f32/kernix | 85.557 / 88.596 | 85.834 / 86.080 | 4 | 131080 | 3.53 |
| matmul_128_f32/nalgebra | 93.509 / 102.732 | 95.505 / 95.921 | 2 | 163840 | 2.97 |
| matmul_128_f32/ndarray | 93.285 / 100.508 | 95.241 / 95.677 | 2 | 163840 | 3.04 |
| matmul_128_f32/numpy | 99.772 / 103.004 | 100.205 / 100.369 | unavailable | unavailable | 35.29 |
| matmul_128_f64/candle | 168.273 / 176.185 | 168.998 / 169.580 | 10 | 262232 | 3.92 |
| matmul_128_f64/faer | 169.098 / 173.657 | 169.610 / 169.882 | 3 | 393224 | 3.53 |
| matmul_128_f64/kernix | 167.786 / 171.273 | 168.247 / 168.360 | 4 | 262152 | 3.72 |
| matmul_128_f64/nalgebra | 208.808 / 218.935 | 208.998 / 209.329 | 2 | 327680 | 2.95 |
| matmul_128_f64/ndarray | 208.280 / 217.479 | 207.942 / 208.842 | 2 | 327680 | 3.07 |
| matmul_128_f64/numpy | 194.280 / 198.136 | 195.019 / 195.874 | unavailable | unavailable | 35.54 |
| matmul_256_f32/candle | 757.955 / 769.371 | 756.717 / 758.471 | 10 | 524376 | 4.55 |
| matmul_256_f32/faer | 678.132 / 683.036 | 678.405 / 690.221 | 3 | 786440 | 4.21 |
| matmul_256_f32/kernix | 665.766 / 671.565 | 664.218 / 666.130 | 4 | 524296 | 4.36 |
| matmul_256_f32/nalgebra | 766.243 / 777.059 | 765.267 / 766.329 | 2 | 589824 | 3.45 |
| matmul_256_f32/ndarray | 767.138 / 779.714 | 765.787 / 768.755 | 2 | 589824 | 3.70 |
| matmul_256_f32/numpy | 728.777 / 738.792 | 726.622 / 729.220 | unavailable | unavailable | 38.54 |
| matmul_256_f64/candle | 1330.557 / 1343.204 | 1329.484 / 1334.373 | 10 | 1048664 | 5.52 |
| matmul_256_f64/faer | 1551.065 / 1566.561 | 1552.209 / 1555.968 | 3 | 1048584 | 5.48 |
| matmul_256_f64/kernix | 1332.074 / 1350.217 | 1335.271 / 1341.318 | 4 | 1048584 | 5.29 |
| matmul_256_f64/nalgebra | 1343.020 / 1354.852 | 1342.884 / 1347.764 | 2 | 1179648 | 4.26 |
| matmul_256_f64/ndarray | 1332.212 / 1345.436 | 1333.601 / 1338.833 | 2 | 1179648 | 4.77 |
| matmul_256_f64/numpy | 1475.672 / 1490.351 | 1476.347 / 1488.774 | unavailable | unavailable | 38.97 |
| matmul_32_f64/candle | 3.440 / 3.480 | 3.460 / 3.508 | 10 | 16976 | 3.33 |
| matmul_32_f64/faer | 3.504 / 3.527 | 3.498 / 3.505 | 3 | 25088 | 2.95 |
| matmul_32_f64/kernix | 3.248 / 3.288 | 3.262 / 3.267 | 3 | 24576 | 2.92 |
| matmul_32_f64/nalgebra | 3.224 / 3.256 | 3.238 / 3.262 | 2 | 24576 | 2.66 |
| matmul_32_f64/ndarray | 3.048 / 3.064 | 3.064 / 3.080 | 2 | 24576 | 2.67 |
| matmul_32_f64/numpy | 8.207 / 8.255 | 8.290 / 8.302 | unavailable | unavailable | 34.21 |
| matmul_64_f32/candle | 11.968 / 12.015 | 12.011 / 12.054 | 10 | 33104 | 3.45 |
| matmul_64_f32/faer | 12.303 / 12.327 | 12.321 / 12.362 | 3 | 49408 | 3.06 |
| matmul_64_f32/kernix | 11.751 / 11.792 | 11.792 / 11.820 | 4 | 33024 | 3.26 |
| matmul_64_f32/nalgebra | 11.279 / 11.295 | 11.301 / 11.355 | 2 | 49152 | 2.82 |
| matmul_64_f32/ndarray | 11.272 / 11.304 | 11.297 / 11.349 | 2 | 49152 | 2.82 |
| matmul_64_f32/numpy | 20.112 / 20.311 | 20.254 / 20.299 | unavailable | unavailable | 34.44 |
| matmul_64_f64/candle | 22.543 / 22.599 | 22.594 / 22.710 | 10 | 65872 | 3.46 |
| matmul_64_f64/faer | 22.647 / 22.711 | 22.675 / 22.704 | 3 | 98560 | 3.04 |
| matmul_64_f64/kernix | 22.247 / 22.327 | 22.308 / 22.448 | 4 | 65792 | 3.24 |
| matmul_64_f64/nalgebra | 22.471 / 22.559 | 22.528 / 22.766 | 2 | 98304 | 2.76 |
| matmul_64_f64/ndarray | 21.808 / 21.863 | 21.856 / 21.908 | 2 | 98304 | 2.79 |
| matmul_64_f64/numpy | 31.111 / 31.591 | 31.232 / 31.330 | unavailable | unavailable | 34.50 |
| matmul_ab_128_f32/kernix | 86.965 / 90.077 | 87.181 / 87.592 | 4 | 131080 | 3.60 |
| matmul_ab_128_f32/numpy | 99.957 / 103.765 | 100.581 / 100.934 | unavailable | unavailable | 35.29 |
| matmul_ab_128_f64/kernix | 169.458 / 174.113 | 170.127 / 170.574 | 4 | 262152 | 3.85 |
| matmul_ab_128_f64/numpy | 196.488 / 203.096 | 197.697 / 198.668 | unavailable | unavailable | 35.55 |
| matmul_batched_4x16x32x16_f64/candle | 4.416 / 4.464 | 4.459 / 4.487 | 25 | 25696 | 3.62 |
| matmul_batched_4x16x32x16_f64/kernix | 3.880 / 3.928 | 3.887 / 3.893 | 7 | 16392 | 3.01 |
| matmul_batched_4x16x32x16_f64/numpy | 12.999 / 13.136 | 13.095 / 13.119 | unavailable | unavailable | 34.23 |
| matmul_sum_backward_32x16x8_f64/candle | 7.288 / 7.535 | 7.323 / 7.385 | 113 | 27980 | 3.84 |
| matmul_sum_backward_32x16x8_f64/kernix | 3.496 / 3.552 | 3.493 / 3.526 | 16 | 18376 | 3.10 |
| min_norm_32x64_rank32_f64/kernix | 96.173 / 98.301 | 96.431 / 96.495 | 20 | 59136 | 3.17 |
| min_norm_32x64_rank32_f64/nalgebra | 218.537 / 220.946 | 218.854 / 220.972 | 15 | 42232 | 2.89 |
| min_norm_32x64_rank32_f64/numpy | 176.609 / 192.568 | 179.529 / 180.555 | unavailable | unavailable | 35.22 |
| min_norm_64x32_rank16_f64/kernix | 72.397 / 74.629 | 72.593 / 72.680 | 20 | 74752 | 3.18 |
| min_norm_64x32_rank16_f64/nalgebra | 116.964 / 120.147 | 117.209 / 117.333 | 15 | 41976 | 2.88 |
| min_norm_64x32_rank16_f64/numpy | 93.901 / 109.509 | 95.965 / 96.293 | unavailable | unavailable | 35.09 |
| parallel_matmul_into_128_f32/kernix | 50.829 / 52.758 | 51.233 / 52.188 | 4 | 65552 | 3.70 |
| parallel_matmul_into_128_f64/kernix | 92.685 / 97.988 | 95.119 / 95.382 | 4 | 131088 | 3.96 |
| parallel_matmul_into_256_f64/kernix | 697.427 / 710.027 | 697.889 / 701.454 | 4 | 524304 | 6.18 |
| parallel_matmul_into_64_f64/kernix | 18.887 / 19.576 | 17.341 / 19.028 | 2 | 98304 | 3.29 |
| pivoted_qr_256x64_f64/faer | 473.974 / 482.398 | 506.656 / 508.339 | 58 | 333312 | 4.16 |
| pivoted_qr_256x64_f64/kernix | 793.153 / 805.536 | 777.696 / 794.741 | 14 | 559104 | 3.59 |
| pivoted_qr_256x64_f64/nalgebra | 1150.714 / 1155.778 | 1150.512 / 1152.522 | 7 | 296960 | 3.29 |
| pivoted_qr_96x32_f64/faer | 62.118 / 64.109 | 62.248 / 62.353 | 32 | 68352 | 3.44 |
| pivoted_qr_96x32_f64/kernix | 88.533 / 91.012 | 88.692 / 88.885 | 14 | 107520 | 3.05 |
| pivoted_qr_96x32_f64/nalgebra | 114.244 / 116.867 | 114.481 / 114.597 | 7 | 58368 | 2.86 |
| qr_256x64_f64/faer | 447.455 / 456.311 | 449.265 / 450.059 | 83 | 331776 | 3.79 |
| qr_256x64_f64/kernix | 695.948 / 709.044 | 697.108 / 697.836 | 6 | 426496 | 3.36 |
| qr_256x64_f64/nalgebra | 586.681 / 597.257 | 586.644 / 587.848 | 4 | 295424 | 3.10 |
| qr_256x64_f64/numpy | 573.546 / 589.826 | 569.535 / 575.723 | unavailable | unavailable | 35.91 |
| qr_96x32_f64/faer | 65.373 / 67.701 | 65.517 / 66.323 | 41 | 67584 | 3.36 |
| qr_96x32_f64/kernix | 72.093 / 74.509 | 72.169 / 72.286 | 6 | 82176 | 2.91 |
| qr_96x32_f64/nalgebra | 61.309 / 63.102 | 61.449 / 61.595 | 4 | 57600 | 2.79 |
| qr_96x32_f64/numpy | 66.293 / 84.581 | 67.914 / 68.168 | unavailable | unavailable | 35.19 |
| softmax_256x64_f64/candle | 110.555 / 116.523 | 110.730 / 110.934 | 37 | 265200 | 4.14 |
| softmax_256x64_f64/kernix | 113.236 / 115.764 | 113.453 / 113.740 | 1 | 131072 | 3.14 |
| softmax_256x64_f64/ndarray | 97.389 / 100.300 | 97.665 / 97.711 | 1 | 131072 | 3.01 |
| softmax_256x64_f64/numpy | 133.851 / 139.227 | 134.607 / 135.309 | unavailable | unavailable | 34.19 |
| tensordot_16x32x8_32x8x16_f64/kernix | 46.742 / 48.510 | 46.832 / 46.890 | 9 | 2166 | 2.89 |
| tensordot_16x32x8_32x8x16_f64/numpy | 16.472 / 16.663 | 16.624 / 16.697 | unavailable | unavailable | 34.10 |
| training_backward_2x4x8x4_f64/candle | 21.136 / 21.488 | 21.088 / 21.186 | 476 | 8996 | 4.22 |
| training_backward_2x4x8x4_f64/kernix | 4.175 / 4.320 | 4.229 / 4.251 | 23 | 4224 | 3.21 |
| training_forward_2x4x8x4_f64/candle | 5.287 / 5.384 | 5.307 / 5.344 | 126 | 3912 | 4.11 |
| training_forward_2x4x8x4_f64/kernix | 3.064 / 3.136 | 3.114 / 3.124 | 17 | 1032 | 3.20 |
| training_record_2x4x8x4_f64/candle | 7.640 / 7.816 | 7.473 / 7.655 | 149 | 10712 | 4.13 |
| training_record_2x4x8x4_f64/kernix | 3.848 / 3.919 | 3.894 / 3.913 | 22 | 6832 | 3.14 |
| training_sgd_2x4x8x4_f64/kernix | 0.720 / 0.752 | 0.742 / 0.743 | 5 | 688 | 3.27 |
| training_shared_backward_64x32_f64/candle | 5.007 / 5.072 | 4.995 / 5.043 | 58 | 99996 | 3.73 |
| training_shared_backward_64x32_f64/kernix | 8.368 / 8.432 | 8.405 / 8.421 | 8 | 49768 | 3.18 |

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
