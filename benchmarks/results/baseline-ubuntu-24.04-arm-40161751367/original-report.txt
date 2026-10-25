# Clean CPU evidence: baseline-ubuntu-24.04-arm-40161751367

Measured source `450df128c1ffb55a9a861230edb6b9a5ac3ef5b4`, tree `957542f0c3a8c779782e0b662134bab34e1572a9`; dirty before/after: false.
Library source: registry 0.11.0. CPU affinity: 0. Dtype explicitly named per workload; row-major inputs.

| Workload / library | Per-call p50 / p95 (us) | Batch-mean p50 / p95 (us) | Rust allocations | Peak additional requested bytes | Process peak RSS (MiB) |
|---|---:|---:|---:|---:|---:|
| cholesky_factor_128_f64/faer | 64.942 / 67.054 | 65.147 / 65.252 | 16 | 148736 | 3.45 |
| cholesky_factor_128_f64/kernix | 280.737 / 283.256 | 277.465 / 279.935 | 1 | 131072 | 2.95 |
| cholesky_factor_128_f64/nalgebra | 129.037 / 133.597 | 129.931 / 130.828 | 1 | 131072 | 2.82 |
| cholesky_factor_128_f64/numpy | 84.206 / 98.941 | 85.885 / 86.431 | unavailable | unavailable | 35.45 |
| cholesky_factor_256_f64/faer | 405.645 / 415.333 | 408.002 / 410.566 | 76 | 592000 | 4.36 |
| cholesky_factor_256_f64/kernix | 768.276 / 776.331 | 768.891 / 779.315 | 49 | 645120 | 4.44 |
| cholesky_factor_256_f64/nalgebra | 1037.404 / 1046.604 | 1043.098 / 1053.484 | 1 | 524288 | 3.57 |
| cholesky_factor_256_f64/numpy | 748.650 / 772.802 | 744.215 / 748.793 | unavailable | unavailable | 38.32 |
| cholesky_factor_512_f64/faer | 2643.087 / 2665.174 | 2637.183 / 2645.126 | 388 | 2297864 | 9.31 |
| cholesky_factor_512_f64/kernix | 4348.512 / 4397.750 | 4325.248 / 4348.136 | 161 | 2269184 | 9.02 |
| cholesky_factor_512_f64/nalgebra | 8577.060 / 8680.970 | 8512.438 / 8614.159 | 1 | 2097152 | 6.57 |
| cholesky_factor_512_f64/numpy | 3888.242 / 3945.879 | 3856.501 / 3909.190 | unavailable | unavailable | 47.82 |
| cholesky_factor_64_f64/faer | 13.208 / 13.232 | 13.252 / 13.307 | 2 | 33280 | 2.72 |
| cholesky_factor_64_f64/kernix | 27.287 / 28.111 | 27.259 / 27.411 | 2 | 33280 | 2.79 |
| cholesky_factor_64_f64/nalgebra | 20.431 / 20.967 | 20.500 / 20.601 | 1 | 32768 | 2.63 |
| cholesky_factor_64_f64/numpy | 18.088 / 18.592 | 18.265 / 18.399 | unavailable | unavailable | 34.69 |
| create_ones_64x64_f32/candle | 0.344 / 0.352 | 0.349 / 0.350 | 7 | 16696 | 3.17 |
| create_ones_64x64_f32/faer | 0.240 / 0.248 | 0.234 / 0.235 | 1 | 16384 | 2.71 |
| create_ones_64x64_f32/kernix | 0.256 / 0.264 | 0.262 / 0.263 | 1 | 16384 | 2.65 |
| create_ones_64x64_f32/ndarray | 0.208 / 0.216 | 0.216 / 0.217 | 1 | 16384 | 2.65 |
| create_ones_64x64_f32/numpy | 1.528 / 1.576 | 1.579 / 1.592 | unavailable | unavailable | 33.58 |
| create_zeros_64x64_f32/candle | 0.240 / 0.248 | 0.247 / 0.253 | 5 | 16648 | 2.98 |
| create_zeros_64x64_f32/faer | 0.320 / 0.328 | 0.311 / 0.318 | 1 | 16384 | 2.73 |
| create_zeros_64x64_f32/kernix | 0.200 / 0.224 | 0.210 / 0.213 | 1 | 16384 | 2.70 |
| create_zeros_64x64_f32/ndarray | 0.136 / 0.144 | 0.143 / 0.146 | 1 | 16384 | 2.65 |
| create_zeros_64x64_f32/numpy | 0.512 / 0.552 | 0.552 / 0.560 | unavailable | unavailable | 33.58 |
| matmul_128_f32/candle | 85.837 / 88.118 | 86.009 / 86.333 | 10 | 131160 | 3.59 |
| matmul_128_f32/faer | 86.942 / 89.133 | 87.228 / 87.824 | 3 | 196616 | 3.16 |
| matmul_128_f32/kernix | 85.301 / 87.190 | 85.559 / 85.776 | 4 | 131080 | 3.41 |
| matmul_128_f32/nalgebra | 93.861 / 101.518 | 95.410 / 95.825 | 2 | 163840 | 2.85 |
| matmul_128_f32/ndarray | 93.702 / 101.157 | 95.210 / 96.097 | 2 | 163840 | 2.91 |
| matmul_128_f32/numpy | 99.885 / 103.757 | 100.479 / 100.843 | unavailable | unavailable | 35.33 |
| matmul_128_f64/candle | 168.420 / 174.308 | 168.970 / 171.436 | 10 | 262232 | 3.93 |
| matmul_128_f64/faer | 169.027 / 173.971 | 169.312 / 169.707 | 3 | 393224 | 3.59 |
| matmul_128_f64/kernix | 167.995 / 176.139 | 168.688 / 168.875 | 4 | 262152 | 3.73 |
| matmul_128_f64/nalgebra | 210.899 / 221.490 | 211.528 / 212.373 | 2 | 327680 | 2.95 |
| matmul_128_f64/ndarray | 206.538 / 215.466 | 205.924 / 207.013 | 2 | 327680 | 3.07 |
| matmul_128_f64/numpy | 195.537 / 200.394 | 196.484 / 197.303 | unavailable | unavailable | 35.58 |
| matmul_256_f32/candle | 757.364 / 770.467 | 757.625 / 759.067 | 10 | 524376 | 4.43 |
| matmul_256_f32/faer | 675.365 / 681.413 | 674.049 / 680.457 | 3 | 786440 | 4.16 |
| matmul_256_f32/kernix | 660.774 / 666.390 | 658.777 / 660.935 | 4 | 524296 | 4.23 |
| matmul_256_f32/nalgebra | 757.890 / 768.273 | 756.884 / 758.101 | 2 | 589824 | 3.32 |
| matmul_256_f32/ndarray | 759.795 / 766.578 | 758.686 / 763.914 | 2 | 589824 | 3.58 |
| matmul_256_f32/numpy | 724.059 / 732.323 | 722.933 / 725.906 | unavailable | unavailable | 39.46 |
| matmul_256_f64/candle | 1320.265 / 1333.792 | 1320.565 / 1324.147 | 10 | 1048664 | 5.53 |
| matmul_256_f64/faer | 1559.604 / 1587.724 | 1533.014 / 1536.647 | 3 | 1048584 | 5.47 |
| matmul_256_f64/kernix | 1319.501 / 1333.204 | 1319.864 / 1330.696 | 4 | 1048584 | 5.29 |
| matmul_256_f64/nalgebra | 1327.050 / 1336.674 | 1327.485 / 1330.915 | 2 | 1179648 | 4.31 |
| matmul_256_f64/ndarray | 1320.836 / 1333.714 | 1320.707 / 1324.638 | 2 | 1179648 | 4.83 |
| matmul_256_f64/numpy | 1463.844 / 1482.348 | 1463.423 / 1474.887 | unavailable | unavailable | 39.96 |
| matmul_32_f64/candle | 3.448 / 3.464 | 3.457 / 3.481 | 10 | 16976 | 3.33 |
| matmul_32_f64/faer | 3.495 / 3.512 | 3.485 / 3.509 | 3 | 25088 | 3.02 |
| matmul_32_f64/kernix | 3.232 / 3.264 | 3.248 / 3.268 | 3 | 24576 | 2.93 |
| matmul_32_f64/nalgebra | 3.216 / 3.248 | 3.233 / 3.253 | 2 | 24576 | 2.72 |
| matmul_32_f64/ndarray | 3.048 / 3.079 | 3.057 / 3.071 | 2 | 24576 | 2.73 |
| matmul_32_f64/numpy | 8.208 / 8.272 | 8.296 / 8.309 | unavailable | unavailable | 34.25 |
| matmul_64_f32/candle | 11.960 / 12.000 | 11.990 / 12.027 | 10 | 33104 | 3.32 |
| matmul_64_f32/faer | 12.304 / 12.328 | 12.327 / 12.408 | 3 | 49408 | 2.98 |
| matmul_64_f32/kernix | 11.752 / 11.815 | 11.789 / 11.813 | 4 | 33024 | 3.13 |
| matmul_64_f32/nalgebra | 11.264 / 11.288 | 11.296 / 11.323 | 2 | 49152 | 2.70 |
| matmul_64_f32/ndarray | 11.263 / 11.287 | 11.291 / 11.375 | 2 | 49152 | 2.71 |
| matmul_64_f32/numpy | 20.119 / 22.431 | 20.310 / 20.370 | unavailable | unavailable | 34.47 |
| matmul_64_f64/candle | 22.527 / 22.600 | 22.393 / 22.621 | 10 | 65872 | 3.46 |
| matmul_64_f64/faer | 22.648 / 22.696 | 22.669 / 22.756 | 3 | 98560 | 3.12 |
| matmul_64_f64/kernix | 22.247 / 22.312 | 22.300 / 22.339 | 4 | 65792 | 3.26 |
| matmul_64_f64/nalgebra | 22.464 / 22.504 | 22.523 / 22.579 | 2 | 98304 | 2.80 |
| matmul_64_f64/ndarray | 21.823 / 21.879 | 21.858 / 21.895 | 2 | 98304 | 2.85 |
| matmul_64_f64/numpy | 31.063 / 31.783 | 31.258 / 31.510 | unavailable | unavailable | 34.53 |
| matmul_ab_128_f32/kernix | 85.582 / 87.790 | 85.858 / 86.083 | 4 | 131080 | 3.60 |
| matmul_ab_128_f32/numpy | 99.885 / 103.686 | 100.380 / 101.538 | unavailable | unavailable | 35.32 |
| matmul_ab_128_f64/kernix | 168.795 / 177.739 | 169.515 / 170.659 | 4 | 262152 | 3.85 |
| matmul_ab_128_f64/numpy | 196.274 / 202.474 | 197.543 / 198.001 | unavailable | unavailable | 35.58 |
| matmul_batched_4x16x32x16_f32/candle | 2.880 / 2.912 | 2.895 / 2.900 | 25 | 13408 | 3.45 |
| matmul_batched_4x16x32x16_f32/kernix | 2.384 / 2.416 | 2.405 / 2.415 | 7 | 8200 | 2.85 |
| matmul_batched_4x16x32x16_f32/numpy | 5.160 / 5.200 | 5.226 / 5.266 | unavailable | unavailable | 34.22 |
| matmul_batched_4x16x32x16_f64/candle | 4.448 / 4.488 | 4.481 / 4.517 | 25 | 25696 | 3.49 |
| matmul_batched_4x16x32x16_f64/kernix | 3.864 / 3.928 | 3.868 / 3.897 | 7 | 16392 | 2.88 |
| matmul_batched_4x16x32x16_f64/numpy | 12.975 / 13.103 | 13.064 / 13.081 | unavailable | unavailable | 34.25 |
| matmul_sum_backward_32x16x8_f64/candle | 7.200 / 7.472 | 7.201 / 7.255 | 113 | 27980 | 3.71 |
| matmul_sum_backward_32x16x8_f64/kernix | 3.504 / 3.568 | 3.515 / 3.547 | 16 | 18376 | 2.85 |
| min_norm_32x64_rank32_f64/kernix | 96.374 / 98.357 | 96.558 / 96.751 | 20 | 59136 | 3.11 |
| min_norm_32x64_rank32_f64/nalgebra | 218.194 / 221.730 | 218.582 / 219.931 | 15 | 42232 | 2.95 |
| min_norm_32x64_rank32_f64/numpy | 176.267 / 192.042 | 179.809 / 180.441 | unavailable | unavailable | 35.25 |
| min_norm_64x32_rank16_f64/kernix | 72.550 / 74.702 | 72.731 / 72.837 | 20 | 74752 | 3.12 |
| min_norm_64x32_rank16_f64/nalgebra | 116.829 / 122.237 | 117.057 / 118.158 | 15 | 41976 | 2.96 |
| min_norm_64x32_rank16_f64/numpy | 93.797 / 107.245 | 95.701 / 96.248 | unavailable | unavailable | 35.12 |
| parallel_matmul_into_128_f32/kernix | 48.895 / 51.679 | 48.971 / 49.433 | 4 | 65552 | 3.77 |
| parallel_matmul_into_128_f64/kernix | 92.141 / 98.549 | 93.986 / 95.244 | 4 | 131088 | 4.04 |
| parallel_matmul_into_256_f64/kernix | 700.219 / 712.410 | 699.133 / 708.257 | 4 | 524304 | 6.25 |
| parallel_matmul_into_64_f64/kernix | 16.472 / 16.984 | 16.733 / 16.841 | 2 | 98304 | 3.22 |
| pivoted_qr_256x64_f64/faer | 473.123 / 478.963 | 504.570 / 507.800 | 58 | 333312 | 4.03 |
| pivoted_qr_256x64_f64/kernix | 786.083 / 798.610 | 769.761 / 783.886 | 14 | 559104 | 3.48 |
| pivoted_qr_256x64_f64/nalgebra | 1150.751 / 1154.775 | 1150.341 / 1157.610 | 7 | 296960 | 3.23 |
| pivoted_qr_96x32_f64/faer | 62.254 / 64.374 | 62.320 / 62.621 | 32 | 68352 | 3.43 |
| pivoted_qr_96x32_f64/kernix | 88.806 / 91.078 | 89.007 / 89.435 | 14 | 107520 | 2.92 |
| pivoted_qr_96x32_f64/nalgebra | 114.221 / 116.652 | 114.472 / 115.340 | 7 | 58368 | 2.80 |
| qr_256x64_f64/faer | 446.563 / 454.850 | 447.989 / 449.282 | 83 | 331776 | 3.92 |
| qr_256x64_f64/kernix | 695.382 / 706.822 | 695.720 / 698.782 | 6 | 426496 | 3.45 |
| qr_256x64_f64/nalgebra | 585.640 / 595.639 | 584.895 / 589.553 | 4 | 295424 | 3.09 |
| qr_256x64_f64/numpy | 569.305 / 588.121 | 566.186 / 569.337 | unavailable | unavailable | 35.92 |
| qr_96x32_f64/faer | 65.359 / 67.630 | 65.563 / 65.777 | 41 | 67584 | 3.48 |
| qr_96x32_f64/kernix | 71.790 / 74.214 | 71.897 / 72.016 | 6 | 82176 | 2.91 |
| qr_96x32_f64/nalgebra | 61.327 / 63.342 | 61.480 / 61.576 | 4 | 57600 | 2.79 |
| qr_96x32_f64/numpy | 66.574 / 84.061 | 68.227 / 69.033 | unavailable | unavailable | 35.22 |
| softmax_256x64_f64/candle | 110.485 / 113.956 | 110.756 / 110.822 | 37 | 265200 | 4.07 |
| softmax_256x64_f64/kernix | 113.093 / 116.085 | 113.338 / 113.740 | 1 | 131072 | 3.00 |
| softmax_256x64_f64/ndarray | 97.357 / 100.070 | 97.594 / 98.030 | 1 | 131072 | 2.93 |
| softmax_256x64_f64/numpy | 134.725 / 141.733 | 135.934 / 137.379 | unavailable | unavailable | 34.21 |
| tensordot_16x32x8_32x8x16_f64/kernix | 46.759 / 48.599 | 46.808 / 46.976 | 9 | 2166 | 2.82 |
| tensordot_16x32x8_32x8x16_f64/numpy | 16.488 / 16.832 | 16.668 / 16.710 | unavailable | unavailable | 34.12 |
| training_backward_2x4x8x4_f64/candle | 21.408 / 21.847 | 21.410 / 21.629 | 476 | 8996 | 4.10 |
| training_backward_2x4x8x4_f64/kernix | 4.184 / 4.304 | 4.212 / 4.240 | 23 | 4224 | 3.09 |
| training_forward_2x4x8x4_f64/candle | 5.368 / 5.528 | 5.344 / 5.399 | 126 | 3912 | 3.98 |
| training_forward_2x4x8x4_f64/kernix | 3.071 / 3.136 | 3.110 / 3.142 | 17 | 1032 | 3.08 |
| training_record_2x4x8x4_f64/candle | 7.656 / 7.792 | 7.511 / 7.669 | 149 | 10712 | 4.01 |
| training_record_2x4x8x4_f64/kernix | 3.824 / 3.888 | 3.847 / 3.864 | 22 | 6832 | 3.02 |
| training_sgd_2x4x8x4_f64/kernix | 0.728 / 0.759 | 0.738 / 0.741 | 5 | 688 | 3.15 |
| training_shared_backward_64x32_f64/candle | 4.968 / 5.032 | 4.958 / 5.020 | 58 | 99996 | 3.60 |
| training_shared_backward_64x32_f64/kernix | 8.415 / 8.464 | 8.445 / 8.471 | 8 | 49768 | 3.05 |

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
