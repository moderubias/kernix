# Clean CPU evidence: 0.10.0-registry-011

Measured source `9a9a0e5eec261c1ed4482fe23215c514e5b20272`, tree `f61e3b2bbbdc99c398396f089d62158fb5bd4372`; dirty before/after: false.
Library source: registry 0.10.0. CPU affinity: 0. Dtype explicitly named per workload; row-major inputs.

| Workload / library | Per-call p50 / p95 (us) | Batch-mean p50 / p95 (us) | Rust allocations | Peak additional requested bytes | Process peak RSS (MiB) |
|---|---:|---:|---:|---:|---:|
| cholesky_factor_128_f64/faer | 38.835 / 40.508 | 37.807 / 39.178 | 2 | 132096 | 4.90 |
| cholesky_factor_128_f64/kernix | 118.804 / 130.139 | 118.220 / 126.118 | 1 | 131072 | 4.18 |
| cholesky_factor_128_f64/nalgebra | 98.295 / 106.369 | 101.761 / 107.332 | 1 | 131072 | 4.16 |
| cholesky_factor_128_f64/numpy | 69.128 / 92.819 | 73.818 / 77.746 | unavailable | unavailable | 37.26 |
| cholesky_factor_256_f64/faer | 194.443 / 208.811 | 197.018 / 202.307 | 2 | 526336 | 5.67 |
| cholesky_factor_256_f64/kernix | 755.046 / 1085.827 | 760.871 / 974.486 | 1 | 524288 | 5.37 |
| cholesky_factor_256_f64/nalgebra | 691.424 / 763.786 | 696.321 / 738.712 | 1 | 524288 | 4.94 |
| cholesky_factor_256_f64/numpy | 522.822 / 744.850 | 550.816 / 646.652 | unavailable | unavailable | 41.13 |
| cholesky_factor_512_f64/faer | 1541.686 / 2147.210 | 1623.448 / 1967.895 | 2 | 2101248 | 8.59 |
| cholesky_factor_512_f64/kernix | 5157.487 / 6555.140 | 5015.426 / 5778.452 | 1 | 2097152 | 9.86 |
| cholesky_factor_512_f64/nalgebra | 6158.120 / 8158.756 | 7210.658 / 10859.170 | 1 | 2097152 | 7.59 |
| cholesky_factor_512_f64/numpy | 4039.864 / 6372.702 | 4369.648 / 6603.890 | unavailable | unavailable | 52.96 |
| cholesky_factor_64_f64/faer | 8.319 / 8.547 | 8.447 / 8.604 | 2 | 33280 | 4.23 |
| cholesky_factor_64_f64/kernix | 19.580 / 21.331 | 20.472 / 20.720 | 2 | 33280 | 4.10 |
| cholesky_factor_64_f64/nalgebra | 13.622 / 14.802 | 14.167 / 14.383 | 1 | 32768 | 4.00 |
| cholesky_factor_64_f64/numpy | 10.821 / 12.001 | 11.090 / 11.275 | unavailable | unavailable | 36.39 |
| create_ones_64x64_f32/candle | 0.216 / 0.256 | 0.226 / 0.247 | 7 | 16696 | 4.76 |
| create_ones_64x64_f32/faer | 0.201 / 0.203 | 0.179 / 0.196 | 1 | 16384 | 4.10 |
| create_ones_64x64_f32/kernix | 0.226 / 0.338 | 0.202 / 0.221 | 1 | 16384 | 3.96 |
| create_ones_64x64_f32/ndarray | 0.154 / 0.156 | 0.146 / 0.156 | 1 | 16384 | 4.02 |
| create_ones_64x64_f32/numpy | 1.281 / 1.318 | 1.338 / 1.385 | unavailable | unavailable | 35.43 |
| create_zeros_64x64_f32/candle | 0.152 / 0.154 | 0.158 / 0.167 | 5 | 16648 | 4.57 |
| create_zeros_64x64_f32/faer | 0.158 / 0.168 | 0.188 / 0.198 | 1 | 16384 | 4.33 |
| create_zeros_64x64_f32/kernix | 0.184 / 0.187 | 0.177 / 0.188 | 1 | 16384 | 4.06 |
| create_zeros_64x64_f32/ndarray | 0.099 / 0.100 | 0.097 / 0.100 | 1 | 16384 | 4.01 |
| create_zeros_64x64_f32/numpy | 0.392 / 0.432 | 0.411 / 0.437 | unavailable | unavailable | 35.43 |
| matmul_128_f32/candle | 38.177 / 54.572 | 39.077 / 40.953 | 9 | 65816 | 4.72 |
| matmul_128_f32/faer | 42.098 / 47.120 | 41.668 / 42.753 | 1 | 65536 | 4.86 |
| matmul_128_f32/kernix | 39.879 / 49.294 | 41.951 / 42.897 | 3 | 65544 | 4.73 |
| matmul_128_f32/nalgebra | 48.494 / 57.043 | 50.711 / 53.892 | 2 | 163840 | 4.14 |
| matmul_128_f32/ndarray | 51.568 / 65.312 | 52.159 / 59.818 | 2 | 163840 | 4.34 |
| matmul_128_f32/numpy | 37.724 / 42.010 | 40.744 / 43.156 | unavailable | unavailable | 37.12 |
| matmul_128_f64/candle | 78.010 / 83.807 | 81.503 / 83.980 | 9 | 131352 | 4.93 |
| matmul_128_f64/faer | 80.246 / 90.295 | 83.026 / 86.255 | 1 | 131072 | 4.59 |
| matmul_128_f64/kernix | 76.016 / 80.488 | 78.087 / 83.350 | 3 | 131080 | 5.21 |
| matmul_128_f64/nalgebra | 112.473 / 134.103 | 114.431 / 116.448 | 2 | 327680 | 3.85 |
| matmul_128_f64/ndarray | 126.261 / 167.621 | 140.973 / 195.183 | 2 | 327680 | 4.42 |
| matmul_128_f64/numpy | 82.695 / 104.757 | 88.286 / 89.910 | unavailable | unavailable | 37.31 |
| matmul_256_f32/candle | 280.464 / 330.728 | 288.857 / 300.778 | 9 | 262424 | 5.95 |
| matmul_256_f32/faer | 312.027 / 326.624 | 316.683 / 332.348 | 1 | 262144 | 5.14 |
| matmul_256_f32/kernix | 309.202 / 367.158 | 320.624 / 327.775 | 3 | 262152 | 5.34 |
| matmul_256_f32/nalgebra | 392.602 / 498.939 | 417.341 / 531.227 | 2 | 589824 | 4.95 |
| matmul_256_f32/ndarray | 407.021 / 481.852 | 409.934 / 435.230 | 2 | 589824 | 5.07 |
| matmul_256_f32/numpy | 294.419 / 329.392 | 298.310 / 305.156 | unavailable | unavailable | 41.28 |
| matmul_256_f64/candle | 566.777 / 720.698 | 582.105 / 610.360 | 9 | 524568 | 6.38 |
| matmul_256_f64/faer | 725.077 / 800.484 | 718.854 / 753.997 | 1 | 524288 | 5.95 |
| matmul_256_f64/kernix | 564.860 / 680.218 | 567.289 / 602.513 | 3 | 524296 | 6.29 |
| matmul_256_f64/nalgebra | 665.835 / 775.526 | 657.054 / 781.721 | 2 | 1179648 | 5.57 |
| matmul_256_f64/ndarray | 654.160 / 746.074 | 662.321 / 692.375 | 2 | 1179648 | 6.14 |
| matmul_256_f64/numpy | 599.005 / 729.392 | 620.304 / 744.741 | unavailable | unavailable | 41.16 |
| matmul_32_f64/candle | 1.676 / 1.822 | 1.679 / 1.785 | 9 | 8592 | 4.38 |
| matmul_32_f64/faer | 1.489 / 1.603 | 1.511 / 1.540 | 1 | 8192 | 4.35 |
| matmul_32_f64/kernix | 1.520 / 1.550 | 1.622 / 1.735 | 3 | 24576 | 4.34 |
| matmul_32_f64/nalgebra | 1.504 / 1.528 | 1.486 / 1.665 | 2 | 24576 | 3.97 |
| matmul_32_f64/ndarray | 1.509 / 1.692 | 1.554 / 1.617 | 2 | 24576 | 4.08 |
| matmul_32_f64/numpy | 2.675 / 3.309 | 2.764 / 2.843 | unavailable | unavailable | 36.05 |
| matmul_64_f32/candle | 4.719 / 4.957 | 4.934 / 5.244 | 9 | 16664 | 5.14 |
| matmul_64_f32/faer | 5.174 / 6.095 | 5.445 / 5.851 | 1 | 16384 | 4.35 |
| matmul_64_f32/kernix | 4.577 / 4.754 | 4.809 / 4.933 | 3 | 16544 | 4.12 |
| matmul_64_f32/nalgebra | 6.062 / 6.942 | 6.037 / 6.569 | 2 | 49152 | 4.03 |
| matmul_64_f32/ndarray | 6.270 / 7.343 | 6.254 / 6.482 | 2 | 49152 | 4.09 |
| matmul_64_f32/numpy | 5.806 / 6.238 | 6.188 / 7.500 | unavailable | unavailable | 36.18 |
| matmul_64_f64/candle | 10.025 / 10.251 | 10.240 / 10.725 | 9 | 33048 | 5.20 |
| matmul_64_f64/faer | 10.864 / 12.026 | 11.042 / 11.350 | 1 | 32768 | 4.46 |
| matmul_64_f64/kernix | 9.579 / 10.392 | 10.011 / 10.418 | 3 | 32928 | 4.82 |
| matmul_64_f64/nalgebra | 10.680 / 11.943 | 10.793 / 10.970 | 2 | 98304 | 4.05 |
| matmul_64_f64/ndarray | 11.013 / 11.384 | 11.654 / 12.203 | 2 | 98304 | 4.11 |
| matmul_64_f64/numpy | 11.344 / 12.003 | 12.329 / 12.781 | unavailable | unavailable | 36.29 |
| matmul_ab_128_f32/kernix | 34.236 / 37.469 | 36.795 / 51.654 | 3 | 65544 | 4.40 |
| matmul_ab_128_f32/numpy | 39.301 / 42.327 | 40.439 / 42.796 | unavailable | unavailable | 36.95 |
| matmul_ab_128_f64/kernix | 74.000 / 81.967 | 76.763 / 109.020 | 3 | 131080 | 5.05 |
| matmul_ab_128_f64/numpy | 113.635 / 126.455 | 93.311 / 116.460 | unavailable | unavailable | 37.27 |
| matmul_batched_4x16x32x16_f64/candle | 1.936 / 2.013 | 2.103 / 2.260 | 25 | 25504 | 5.35 |
| matmul_batched_4x16x32x16_f64/kernix | 1.886 / 1.939 | 2.042 / 2.167 | 7 | 16392 | 4.42 |
| matmul_batched_4x16x32x16_f64/numpy | 3.054 / 3.114 | 3.056 / 3.111 | unavailable | unavailable | 36.11 |
| matmul_sum_backward_32x16x8_f64/candle | 5.091 / 7.564 | 4.696 / 5.000 | 113 | 27988 | 5.02 |
| matmul_sum_backward_32x16x8_f64/kernix | 2.589 / 2.931 | 2.863 / 4.460 | 16 | 18376 | 4.27 |
| min_norm_32x64_rank32_f64/kernix | 56.992 / 61.867 | 57.067 / 59.289 | 20 | 59136 | 4.67 |
| min_norm_32x64_rank32_f64/nalgebra | 141.059 / 154.882 | 138.726 / 152.696 | 15 | 42232 | 4.28 |
| min_norm_32x64_rank32_f64/numpy | 119.212 / 127.101 | 122.280 / 132.111 | unavailable | unavailable | 36.96 |
| min_norm_64x32_rank16_f64/kernix | 43.150 / 48.251 | 44.123 / 45.170 | 20 | 74752 | 4.52 |
| min_norm_64x32_rank16_f64/nalgebra | 73.676 / 78.870 | 74.348 / 78.080 | 15 | 41976 | 4.11 |
| min_norm_64x32_rank16_f64/numpy | 64.624 / 71.698 | 65.976 / 74.462 | unavailable | unavailable | 37.00 |
| parallel_matmul_into_128_f32/kernix | 39.180 / 40.148 | 22.608 / 24.441 | 2 | 16 | 5.14 |
| parallel_matmul_into_128_f64/kernix | 41.284 / 80.614 | 43.297 / 60.368 | 2 | 16 | 5.39 |
| parallel_matmul_into_256_f64/kernix | 294.668 / 362.175 | 297.407 / 310.528 | 2 | 16 | 7.18 |
| parallel_matmul_into_64_f64/kernix | 9.672 / 11.066 | 10.606 / 15.188 | 2 | 98304 | 4.62 |
| pivoted_qr_256x64_f64/faer | 446.662 / 487.436 | 452.081 / 474.064 | 10 | 333312 | 4.97 |
| pivoted_qr_256x64_f64/kernix | 519.996 / 701.568 | 523.831 / 549.029 | 14 | 559104 | 4.94 |
| pivoted_qr_256x64_f64/nalgebra | 781.249 / 948.688 | 839.971 / 981.001 | 7 | 296960 | 4.57 |
| pivoted_qr_96x32_f64/faer | 71.009 / 78.478 | 74.220 / 96.599 | 10 | 68352 | 4.95 |
| pivoted_qr_96x32_f64/kernix | 53.411 / 56.965 | 54.974 / 56.579 | 14 | 107520 | 4.48 |
| pivoted_qr_96x32_f64/nalgebra | 73.743 / 81.739 | 77.260 / 81.508 | 7 | 58368 | 4.17 |
| qr_256x64_f64/faer | 406.431 / 467.579 | 425.388 / 476.560 | 7 | 331776 | 5.23 |
| qr_256x64_f64/kernix | 485.343 / 591.974 | 510.864 / 558.557 | 6 | 426496 | 4.77 |
| qr_256x64_f64/nalgebra | 374.058 / 400.490 | 377.028 / 395.604 | 4 | 295424 | 4.31 |
| qr_256x64_f64/numpy | 377.160 / 441.122 | 382.308 / 403.396 | unavailable | unavailable | 37.98 |
| qr_96x32_f64/faer | 65.412 / 74.597 | 67.956 / 74.753 | 7 | 67584 | 5.04 |
| qr_96x32_f64/kernix | 44.501 / 60.906 | 50.444 / 62.748 | 6 | 82176 | 4.23 |
| qr_96x32_f64/nalgebra | 37.932 / 45.876 | 37.845 / 39.673 | 4 | 57600 | 4.08 |
| qr_96x32_f64/numpy | 40.505 / 48.167 | 40.630 / 41.987 | unavailable | unavailable | 37.12 |
| softmax_256x64_f64/candle | 88.328 / 95.189 | 87.200 / 117.734 | 37 | 265200 | 5.97 |
| softmax_256x64_f64/kernix | 85.131 / 91.925 | 85.573 / 89.585 | 1 | 131072 | 4.41 |
| softmax_256x64_f64/ndarray | 72.742 / 107.775 | 73.820 / 79.257 | 1 | 131072 | 4.45 |
| softmax_256x64_f64/numpy | 99.236 / 106.295 | 100.268 / 108.500 | unavailable | unavailable | 35.94 |
| tensordot_16x32x8_32x8x16_f64/kernix | 35.216 / 37.843 | 35.389 / 36.193 | 9 | 2166 | 4.05 |
| tensordot_16x32x8_32x8x16_f64/numpy | 7.537 / 7.829 | 7.795 / 8.129 | unavailable | unavailable | 35.96 |
| training_backward_2x4x8x4_f64/candle | 14.237 / 14.843 | 14.515 / 16.058 | 472 | 8332 | 6.33 |
| training_backward_2x4x8x4_f64/kernix | 3.151 / 3.389 | 3.314 / 3.628 | 23 | 4224 | 4.59 |
| training_forward_2x4x8x4_f64/candle | 3.370 / 3.496 | 3.457 / 3.540 | 126 | 3912 | 6.17 |
| training_forward_2x4x8x4_f64/kernix | 2.155 / 2.316 | 2.215 / 2.364 | 17 | 1032 | 4.75 |
| training_record_2x4x8x4_f64/candle | 5.180 / 5.449 | 5.166 / 5.574 | 149 | 10712 | 5.86 |
| training_record_2x4x8x4_f64/kernix | 2.794 / 2.899 | 2.882 / 3.542 | 22 | 6832 | 4.56 |
| training_sgd_2x4x8x4_f64/kernix | 0.526 / 0.554 | 0.532 / 0.560 | 5 | 688 | 4.80 |
| training_shared_backward_64x32_f64/candle | 3.848 / 4.878 | 3.871 / 4.185 | 58 | 100004 | 5.37 |
| training_shared_backward_64x32_f64/kernix | 6.053 / 6.644 | 6.297 / 7.826 | 8 | 49768 | 4.55 |

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
