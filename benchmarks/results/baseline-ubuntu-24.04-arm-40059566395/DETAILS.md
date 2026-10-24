# Clean CPU evidence: baseline-ubuntu-24.04-arm-40059566395

Measured source `18750693908110110d85d02d0f11af8d58bfae8b`, tree `75403e48db8556f95e014ec2c5b16b30f9fa781e`; dirty before/after: false.
Library source: registry 0.10.0. CPU affinity: 0. Dtype explicitly named per workload; row-major inputs.

| Workload / library | Per-call p50 / p95 (us) | Batch-mean p50 / p95 (us) | Rust allocations | Peak additional requested bytes | Process peak RSS (MiB) |
|---|---:|---:|---:|---:|---:|
| cholesky_factor_128_f64/faer | 64.939 / 66.828 | 65.018 / 65.205 | 16 | 148736 | 3.45 |
| cholesky_factor_128_f64/kernix | 282.148 / 284.907 | 278.437 / 281.636 | 1 | 131072 | 3.01 |
| cholesky_factor_128_f64/nalgebra | 128.991 / 132.087 | 130.198 / 130.694 | 1 | 131072 | 2.88 |
| cholesky_factor_128_f64/numpy | 83.980 / 98.859 | 85.973 / 86.423 | unavailable | unavailable | 35.43 |
| cholesky_factor_256_f64/faer | 405.612 / 419.986 | 408.137 / 409.972 | 76 | 592000 | 4.36 |
| cholesky_factor_256_f64/kernix | 1988.423 / 2001.438 | 1985.114 / 1990.780 | 1 | 524288 | 4.12 |
| cholesky_factor_256_f64/nalgebra | 1033.389 / 1042.236 | 1039.077 / 1045.464 | 1 | 524288 | 3.63 |
| cholesky_factor_256_f64/numpy | 771.097 / 789.360 | 767.099 / 772.292 | unavailable | unavailable | 38.30 |
| cholesky_factor_512_f64/faer | 2665.544 / 2689.798 | 2669.574 / 2679.659 | 388 | 2297864 | 9.31 |
| cholesky_factor_512_f64/kernix | 15318.115 / 15716.731 | 15280.151 / 15447.652 | 1 | 2097152 | 8.64 |
| cholesky_factor_512_f64/nalgebra | 8996.093 / 9070.767 | 8930.553 / 8991.973 | 1 | 2097152 | 6.63 |
| cholesky_factor_512_f64/numpy | 3936.927 / 4210.451 | 3935.361 / 3944.904 | unavailable | unavailable | 47.80 |
| cholesky_factor_64_f64/faer | 13.208 / 13.240 | 13.253 / 13.286 | 2 | 33280 | 2.79 |
| cholesky_factor_64_f64/kernix | 27.367 / 28.431 | 27.299 / 27.382 | 2 | 33280 | 2.79 |
| cholesky_factor_64_f64/nalgebra | 20.334 / 20.943 | 20.535 / 20.655 | 1 | 32768 | 2.70 |
| cholesky_factor_64_f64/numpy | 18.111 / 19.206 | 18.436 / 18.528 | unavailable | unavailable | 34.66 |
| create_ones_64x64_f32/candle | 0.344 / 0.360 | 0.355 / 0.356 | 7 | 16696 | 3.17 |
| create_ones_64x64_f32/faer | 0.240 / 0.248 | 0.233 / 0.234 | 1 | 16384 | 2.73 |
| create_ones_64x64_f32/kernix | 0.256 / 0.272 | 0.260 / 0.261 | 1 | 16384 | 2.64 |
| create_ones_64x64_f32/ndarray | 0.208 / 0.216 | 0.246 / 0.254 | 1 | 16384 | 2.64 |
| create_ones_64x64_f32/numpy | 1.536 / 1.583 | 1.591 / 1.603 | unavailable | unavailable | 33.57 |
| create_zeros_64x64_f32/candle | 0.232 / 0.248 | 0.249 / 0.252 | 5 | 16648 | 2.98 |
| create_zeros_64x64_f32/faer | 0.368 / 0.376 | 0.322 / 0.348 | 1 | 16384 | 2.73 |
| create_zeros_64x64_f32/kernix | 0.200 / 0.216 | 0.223 / 0.227 | 1 | 16384 | 2.71 |
| create_zeros_64x64_f32/ndarray | 0.136 / 0.144 | 0.144 / 0.153 | 1 | 16384 | 2.65 |
| create_zeros_64x64_f32/numpy | 0.504 / 0.536 | 0.545 / 0.553 | unavailable | unavailable | 33.56 |
| matmul_128_f32/candle | 85.954 / 89.058 | 86.126 / 86.211 | 10 | 131160 | 3.70 |
| matmul_128_f32/faer | 86.835 / 88.803 | 87.019 / 87.089 | 3 | 196616 | 3.22 |
| matmul_128_f32/kernix | 94.099 / 101.770 | 95.700 / 96.099 | 3 | 163840 | 3.07 |
| matmul_128_f32/nalgebra | 93.585 / 103.313 | 95.317 / 95.831 | 2 | 163840 | 2.91 |
| matmul_128_f32/ndarray | 93.634 / 101.073 | 95.001 / 95.330 | 2 | 163840 | 2.98 |
| matmul_128_f32/numpy | 99.801 / 103.881 | 100.288 / 100.411 | unavailable | unavailable | 35.30 |
| matmul_128_f64/candle | 168.398 / 174.534 | 168.890 / 169.094 | 10 | 262232 | 3.99 |
| matmul_128_f64/faer | 169.740 / 176.092 | 170.397 / 171.022 | 3 | 393224 | 3.59 |
| matmul_128_f64/kernix | 211.068 / 221.067 | 210.650 / 211.039 | 3 | 327680 | 3.19 |
| matmul_128_f64/nalgebra | 210.284 / 220.427 | 210.054 / 210.358 | 2 | 327680 | 2.95 |
| matmul_128_f64/ndarray | 205.890 / 214.474 | 205.869 / 206.875 | 2 | 327680 | 3.06 |
| matmul_128_f64/numpy | 195.517 / 199.213 | 196.146 / 196.771 | unavailable | unavailable | 35.55 |
| matmul_256_f32/candle | 757.566 / 767.150 | 756.420 / 763.296 | 10 | 524376 | 4.55 |
| matmul_256_f32/faer | 676.626 / 681.490 | 676.232 / 681.827 | 3 | 786440 | 4.22 |
| matmul_256_f32/kernix | 768.293 / 780.556 | 768.068 / 771.241 | 3 | 589824 | 3.69 |
| matmul_256_f32/nalgebra | 763.313 / 771.993 | 761.850 / 762.676 | 2 | 589824 | 3.32 |
| matmul_256_f32/ndarray | 767.441 / 783.608 | 762.496 / 765.268 | 2 | 589824 | 3.58 |
| matmul_256_f32/numpy | 722.944 / 733.319 | 723.486 / 726.182 | unavailable | unavailable | 39.45 |
| matmul_256_f64/candle | 1329.027 / 1344.777 | 1333.344 / 1337.963 | 10 | 1048664 | 5.60 |
| matmul_256_f64/faer | 1550.759 / 1563.286 | 1551.759 / 1555.118 | 3 | 1048584 | 5.47 |
| matmul_256_f64/kernix | 1339.411 / 1352.482 | 1337.351 / 1345.614 | 3 | 1179648 | 4.95 |
| matmul_256_f64/nalgebra | 1338.464 / 1352.463 | 1335.864 / 1344.367 | 2 | 1179648 | 4.32 |
| matmul_256_f64/ndarray | 1328.265 / 1340.121 | 1332.932 / 1335.815 | 2 | 1179648 | 4.82 |
| matmul_256_f64/numpy | 1476.766 / 1493.025 | 1477.154 / 1483.962 | unavailable | unavailable | 39.91 |
| matmul_32_f64/candle | 3.440 / 3.464 | 3.456 / 3.473 | 10 | 16976 | 3.39 |
| matmul_32_f64/faer | 3.495 / 3.512 | 3.480 / 3.485 | 3 | 25088 | 3.03 |
| matmul_32_f64/kernix | 3.224 / 3.256 | 3.236 / 3.263 | 3 | 24576 | 2.87 |
| matmul_32_f64/nalgebra | 3.208 / 3.224 | 3.225 / 3.235 | 2 | 24576 | 2.71 |
| matmul_32_f64/ndarray | 3.047 / 3.056 | 3.051 / 3.059 | 2 | 24576 | 2.73 |
| matmul_32_f64/numpy | 8.208 / 8.264 | 8.306 / 8.319 | unavailable | unavailable | 34.21 |
| matmul_64_f32/candle | 11.952 / 11.991 | 11.990 / 12.067 | 10 | 33104 | 3.45 |
| matmul_64_f32/faer | 12.295 / 12.319 | 12.316 / 12.350 | 3 | 49408 | 3.05 |
| matmul_64_f32/kernix | 11.504 / 11.543 | 11.541 / 11.604 | 3 | 49152 | 2.85 |
| matmul_64_f32/nalgebra | 11.279 / 11.303 | 11.307 / 11.357 | 2 | 49152 | 2.76 |
| matmul_64_f32/ndarray | 11.271 / 11.295 | 11.297 / 11.346 | 2 | 49152 | 2.75 |
| matmul_64_f32/numpy | 20.127 / 20.727 | 20.289 / 20.324 | unavailable | unavailable | 34.46 |
| matmul_64_f64/candle | 22.694 / 22.751 | 22.512 / 22.847 | 10 | 65872 | 3.52 |
| matmul_64_f64/faer | 22.638 / 22.694 | 22.657 / 22.688 | 3 | 98560 | 3.11 |
| matmul_64_f64/kernix | 22.175 / 22.231 | 22.239 / 22.345 | 3 | 98304 | 3.00 |
| matmul_64_f64/nalgebra | 22.447 / 22.495 | 22.512 / 22.610 | 2 | 98304 | 2.82 |
| matmul_64_f64/ndarray | 21.775 / 21.863 | 21.826 / 21.890 | 2 | 98304 | 2.84 |
| matmul_64_f64/numpy | 31.126 / 31.670 | 31.245 / 31.306 | unavailable | unavailable | 34.52 |
| matmul_ab_128_f32/kernix | 94.587 / 101.994 | 96.243 / 96.863 | 3 | 163840 | 3.20 |
| matmul_ab_128_f32/numpy | 100.017 / 102.433 | 100.407 / 100.824 | unavailable | unavailable | 35.29 |
| matmul_ab_128_f64/kernix | 211.489 / 223.920 | 212.155 / 214.701 | 3 | 327680 | 3.33 |
| matmul_ab_128_f64/numpy | 197.028 / 206.868 | 198.510 / 199.194 | unavailable | unavailable | 35.55 |
| matmul_batched_4x16x32x16_f64/candle | 4.439 / 4.480 | 4.464 / 4.487 | 25 | 25696 | 3.62 |
| matmul_batched_4x16x32x16_f64/kernix | 3.824 / 3.864 | 3.842 / 3.861 | 7 | 16392 | 2.88 |
| matmul_batched_4x16x32x16_f64/numpy | 12.983 / 13.071 | 13.075 / 13.116 | unavailable | unavailable | 34.24 |
| matmul_sum_backward_32x16x8_f64/candle | 7.327 / 7.567 | 7.226 / 7.334 | 113 | 27980 | 3.83 |
| matmul_sum_backward_32x16x8_f64/kernix | 3.511 / 3.576 | 3.503 / 3.539 | 16 | 18376 | 2.90 |
| min_norm_32x64_rank32_f64/kernix | 96.235 / 98.331 | 96.439 / 97.289 | 20 | 59136 | 3.11 |
| min_norm_32x64_rank32_f64/nalgebra | 219.113 / 221.345 | 219.450 / 219.779 | 15 | 42232 | 2.89 |
| min_norm_32x64_rank32_f64/numpy | 176.895 / 193.486 | 179.928 / 181.042 | unavailable | unavailable | 35.23 |
| min_norm_64x32_rank16_f64/kernix | 72.597 / 74.644 | 72.776 / 73.233 | 20 | 74752 | 3.11 |
| min_norm_64x32_rank16_f64/nalgebra | 117.073 / 120.281 | 117.212 / 117.421 | 15 | 41976 | 2.89 |
| min_norm_64x32_rank16_f64/numpy | 93.658 / 108.488 | 95.338 / 96.210 | unavailable | unavailable | 35.10 |
| parallel_matmul_into_128_f32/kernix | 49.933 / 52.685 | 49.982 / 50.265 | 4 | 163840 | 3.53 |
| parallel_matmul_into_128_f64/kernix | 97.891 / 102.554 | 99.646 / 100.138 | 4 | 327680 | 3.89 |
| parallel_matmul_into_256_f64/kernix | 841.096 / 854.167 | 844.621 / 852.210 | 8 | 1179648 | 6.21 |
| parallel_matmul_into_64_f64/kernix | 17.174 / 17.623 | 17.162 / 17.589 | 2 | 98304 | 3.29 |
| pivoted_qr_256x64_f64/faer | 473.822 / 478.750 | 505.559 / 507.500 | 58 | 333312 | 4.02 |
| pivoted_qr_256x64_f64/kernix | 790.707 / 804.115 | 774.721 / 788.905 | 14 | 559104 | 3.48 |
| pivoted_qr_256x64_f64/nalgebra | 1151.077 / 1154.933 | 1150.645 / 1151.385 | 7 | 296960 | 3.29 |
| pivoted_qr_96x32_f64/faer | 61.950 / 64.094 | 62.073 / 62.202 | 32 | 68352 | 3.37 |
| pivoted_qr_96x32_f64/kernix | 88.626 / 91.162 | 88.752 / 88.852 | 14 | 107520 | 2.93 |
| pivoted_qr_96x32_f64/nalgebra | 114.242 / 116.466 | 114.486 / 114.690 | 7 | 58368 | 2.86 |
| qr_256x64_f64/faer | 447.704 / 456.287 | 448.896 / 451.622 | 83 | 331776 | 3.79 |
| qr_256x64_f64/kernix | 694.486 / 706.173 | 694.801 / 696.572 | 6 | 426496 | 3.45 |
| qr_256x64_f64/nalgebra | 587.694 / 597.398 | 588.046 / 590.593 | 4 | 295424 | 3.04 |
| qr_256x64_f64/numpy | 573.582 / 595.908 | 568.213 / 573.141 | unavailable | unavailable | 35.92 |
| qr_96x32_f64/faer | 65.355 / 67.836 | 65.449 / 65.897 | 41 | 67584 | 3.36 |
| qr_96x32_f64/kernix | 71.739 / 74.123 | 71.909 / 71.988 | 6 | 82176 | 2.91 |
| qr_96x32_f64/nalgebra | 61.413 / 63.380 | 61.555 / 61.587 | 4 | 57600 | 2.73 |
| qr_96x32_f64/numpy | 66.630 / 79.782 | 68.134 / 68.760 | unavailable | unavailable | 35.20 |
| softmax_256x64_f64/candle | 110.490 / 115.994 | 110.682 / 111.042 | 37 | 265200 | 4.15 |
| softmax_256x64_f64/kernix | 113.033 / 115.561 | 113.263 / 113.505 | 1 | 131072 | 3.07 |
| softmax_256x64_f64/ndarray | 97.441 / 99.921 | 97.702 / 97.769 | 1 | 131072 | 3.01 |
| softmax_256x64_f64/numpy | 135.007 / 143.574 | 136.747 / 137.335 | unavailable | unavailable | 34.20 |
| tensordot_16x32x8_32x8x16_f64/kernix | 46.693 / 48.484 | 46.810 / 46.826 | 9 | 2166 | 2.76 |
| tensordot_16x32x8_32x8x16_f64/numpy | 16.439 / 16.871 | 16.576 / 16.601 | unavailable | unavailable | 34.11 |
| training_backward_2x4x8x4_f64/candle | 21.135 / 21.447 | 21.092 / 21.159 | 476 | 8996 | 4.22 |
| training_backward_2x4x8x4_f64/kernix | 4.184 / 4.328 | 4.213 / 4.234 | 23 | 4224 | 3.15 |
| training_forward_2x4x8x4_f64/candle | 5.240 / 5.352 | 5.295 / 5.331 | 126 | 3912 | 4.11 |
| training_forward_2x4x8x4_f64/kernix | 3.087 / 3.144 | 3.120 / 3.155 | 17 | 1032 | 3.14 |
| training_record_2x4x8x4_f64/candle | 7.575 / 7.759 | 7.492 / 7.592 | 149 | 10712 | 4.12 |
| training_record_2x4x8x4_f64/kernix | 3.840 / 3.904 | 3.878 / 3.893 | 22 | 6832 | 3.08 |
| training_sgd_2x4x8x4_f64/kernix | 0.744 / 0.784 | 0.766 / 0.769 | 5 | 688 | 3.21 |
| training_shared_backward_64x32_f64/candle | 5.031 / 5.112 | 5.023 / 5.080 | 58 | 99996 | 3.66 |
| training_shared_backward_64x32_f64/kernix | 8.343 / 8.392 | 8.366 / 8.408 | 8 | 49768 | 3.17 |

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
