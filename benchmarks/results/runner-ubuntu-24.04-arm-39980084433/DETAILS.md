# Clean CPU evidence: runner-ubuntu-24.04-arm-39980084433

Measured source `9b7ef492c99bf2202e7eb3b07a79ef8f108991c7`, tree `3bfcaf4212c3091bc37e737a1d4d86760b373f94`; dirty before/after: false.
Library source: workspace 9b7ef492c99bf2202e7eb3b07a79ef8f108991c7. CPU affinity: 0. Dtype explicitly named per workload; row-major inputs.

| Workload / library | Per-call p50 / p95 (us) | Batch-mean p50 / p95 (us) | Rust allocations | Peak additional requested bytes | Process peak RSS (MiB) |
|---|---:|---:|---:|---:|---:|
| cholesky_factor_128_f64/faer | 64.996 / 66.963 | 65.118 / 65.455 | 16 | 148736 | 3.38 |
| cholesky_factor_128_f64/kernix | 280.789 / 286.077 | 278.573 / 280.940 | 1 | 131072 | 3.01 |
| cholesky_factor_128_f64/nalgebra | 128.775 / 132.247 | 129.976 / 130.399 | 1 | 131072 | 2.88 |
| cholesky_factor_128_f64/numpy | 82.746 / 93.434 | 84.289 / 85.478 | unavailable | unavailable | 35.16 |
| cholesky_factor_256_f64/faer | 407.037 / 419.684 | 410.331 / 414.445 | 76 | 592000 | 4.29 |
| cholesky_factor_256_f64/kernix | 1994.912 / 2011.119 | 1991.633 / 2008.225 | 1 | 524288 | 4.14 |
| cholesky_factor_256_f64/nalgebra | 1031.706 / 1042.321 | 1035.349 / 1044.509 | 1 | 524288 | 3.63 |
| cholesky_factor_256_f64/numpy | 751.805 / 773.211 | 748.325 / 751.618 | unavailable | unavailable | 38.00 |
| cholesky_factor_64_f64/faer | 13.256 / 13.287 | 13.281 / 13.371 | 2 | 33280 | 2.79 |
| cholesky_factor_64_f64/kernix | 27.334 / 28.310 | 27.304 / 27.494 | 2 | 33280 | 2.78 |
| cholesky_factor_64_f64/nalgebra | 20.350 / 21.055 | 20.565 / 20.691 | 1 | 32768 | 2.69 |
| cholesky_factor_64_f64/numpy | 18.031 / 19.095 | 18.187 / 18.292 | unavailable | unavailable | 34.38 |
| create_ones_64x64_f32/candle | 0.344 / 0.352 | 0.349 / 0.351 | 7 | 16696 | 3.16 |
| create_ones_64x64_f32/faer | 0.240 / 0.248 | 0.233 / 0.234 | 1 | 16384 | 2.66 |
| create_ones_64x64_f32/kernix | 0.248 / 0.272 | 0.258 / 0.259 | 1 | 16384 | 2.58 |
| create_ones_64x64_f32/ndarray | 0.208 / 0.216 | 0.216 / 0.217 | 1 | 16384 | 2.58 |
| create_ones_64x64_f32/numpy | 1.520 / 1.560 | 1.578 / 1.589 | unavailable | unavailable | 33.25 |
| create_zeros_64x64_f32/candle | 0.256 / 0.272 | 0.247 / 0.254 | 5 | 16648 | 2.97 |
| create_zeros_64x64_f32/faer | 0.328 / 0.328 | 0.316 / 0.336 | 1 | 16384 | 2.66 |
| create_zeros_64x64_f32/kernix | 0.184 / 0.208 | 0.196 / 0.206 | 1 | 16384 | 2.58 |
| create_zeros_64x64_f32/ndarray | 0.136 / 0.152 | 0.141 / 0.144 | 1 | 16384 | 2.58 |
| create_zeros_64x64_f32/numpy | 0.512 / 0.536 | 0.556 / 0.572 | unavailable | unavailable | 33.26 |
| matmul_128_f32/candle | 85.819 / 88.362 | 86.019 / 86.489 | 10 | 131160 | 3.64 |
| matmul_128_f32/faer | 87.051 / 89.706 | 87.331 / 88.025 | 3 | 196616 | 3.15 |
| matmul_128_f32/kernix | 94.506 / 105.848 | 96.629 / 97.141 | 3 | 163840 | 3.13 |
| matmul_128_f32/nalgebra | 94.962 / 106.793 | 97.320 / 97.787 | 2 | 163840 | 2.91 |
| matmul_128_f32/ndarray | 93.594 / 100.633 | 95.534 / 97.266 | 2 | 163840 | 2.97 |
| matmul_128_f32/numpy | 100.065 / 104.281 | 100.787 / 101.775 | unavailable | unavailable | 35.03 |
| matmul_128_f64/candle | 169.268 / 175.044 | 170.047 / 170.908 | 10 | 262232 | 3.91 |
| matmul_128_f64/faer | 169.821 / 177.364 | 170.456 / 171.302 | 3 | 393224 | 3.52 |
| matmul_128_f64/kernix | 210.410 / 219.545 | 210.113 / 211.989 | 3 | 327680 | 3.13 |
| matmul_128_f64/nalgebra | 208.642 / 220.649 | 208.617 / 209.131 | 2 | 327680 | 2.88 |
| matmul_128_f64/ndarray | 207.458 / 219.993 | 206.817 / 207.291 | 2 | 327680 | 2.93 |
| matmul_128_f64/numpy | 196.011 / 201.322 | 197.181 / 198.366 | unavailable | unavailable | 35.16 |
| matmul_256_f32/candle | 757.933 / 771.651 | 755.684 / 763.814 | 10 | 524376 | 4.42 |
| matmul_256_f32/faer | 674.018 / 686.073 | 675.260 / 685.784 | 3 | 786440 | 4.15 |
| matmul_256_f32/kernix | 770.612 / 788.667 | 773.040 / 782.761 | 3 | 589824 | 3.70 |
| matmul_256_f32/nalgebra | 761.412 / 775.363 | 760.008 / 762.532 | 2 | 589824 | 3.32 |
| matmul_256_f32/ndarray | 765.316 / 777.075 | 765.473 / 770.391 | 2 | 589824 | 3.57 |
| matmul_256_f32/numpy | 729.143 / 736.189 | 735.772 / 738.498 | unavailable | unavailable | 38.86 |
| matmul_256_f64/candle | 1324.023 / 1339.525 | 1325.221 / 1336.553 | 10 | 1048664 | 5.53 |
| matmul_256_f64/faer | 1548.982 / 1567.765 | 1551.846 / 1558.088 | 3 | 1048584 | 5.47 |
| matmul_256_f64/kernix | 1335.542 / 1347.445 | 1337.238 / 1341.592 | 3 | 1179648 | 4.89 |
| matmul_256_f64/nalgebra | 1334.373 / 1344.084 | 1332.547 / 1335.848 | 2 | 1179648 | 4.20 |
| matmul_256_f64/ndarray | 1326.502 / 1342.692 | 1324.880 / 1330.513 | 2 | 1179648 | 4.70 |
| matmul_256_f64/numpy | 1466.035 / 1476.891 | 1464.488 / 1472.112 | unavailable | unavailable | 39.14 |
| matmul_32_f64/candle | 3.424 / 3.456 | 3.438 / 3.455 | 10 | 16976 | 3.32 |
| matmul_32_f64/faer | 3.504 / 3.536 | 3.490 / 3.513 | 3 | 25088 | 2.96 |
| matmul_32_f64/kernix | 3.239 / 3.280 | 3.251 / 3.265 | 3 | 24576 | 2.80 |
| matmul_32_f64/nalgebra | 3.216 / 3.248 | 3.231 / 3.239 | 2 | 24576 | 2.60 |
| matmul_32_f64/ndarray | 3.056 / 3.080 | 3.065 / 3.096 | 2 | 24576 | 2.61 |
| matmul_32_f64/numpy | 8.200 / 8.287 | 8.282 / 8.357 | unavailable | unavailable | 33.93 |
| matmul_64_f32/candle | 11.951 / 11.983 | 11.977 / 12.043 | 10 | 33104 | 3.38 |
| matmul_64_f32/faer | 12.295 / 12.327 | 12.318 / 12.387 | 3 | 49408 | 3.00 |
| matmul_64_f32/kernix | 11.504 / 11.543 | 11.556 / 11.575 | 3 | 49152 | 2.91 |
| matmul_64_f32/nalgebra | 11.271 / 11.288 | 11.298 / 11.336 | 2 | 49152 | 2.75 |
| matmul_64_f32/ndarray | 11.247 / 11.279 | 11.279 / 11.337 | 2 | 49152 | 2.77 |
| matmul_64_f32/numpy | 20.119 / 20.334 | 20.265 / 20.302 | unavailable | unavailable | 34.16 |
| matmul_64_f64/candle | 22.574 / 22.639 | 22.616 / 22.673 | 10 | 65872 | 3.46 |
| matmul_64_f64/faer | 22.631 / 22.710 | 22.668 / 22.724 | 3 | 98560 | 3.05 |
| matmul_64_f64/kernix | 22.174 / 22.239 | 22.220 / 22.292 | 3 | 98304 | 2.93 |
| matmul_64_f64/nalgebra | 22.463 / 22.511 | 22.518 / 22.640 | 2 | 98304 | 2.69 |
| matmul_64_f64/ndarray | 21.799 / 21.854 | 21.848 / 21.927 | 2 | 98304 | 2.72 |
| matmul_64_f64/numpy | 31.118 / 31.902 | 31.293 / 31.352 | unavailable | unavailable | 34.21 |
| matmul_batched_4x16x32x16_f64/candle | 4.448 / 4.496 | 4.489 / 4.532 | 25 | 25696 | 3.55 |
| matmul_batched_4x16x32x16_f64/kernix | 3.856 / 3.912 | 3.877 / 3.898 | 7 | 16392 | 2.82 |
| matmul_batched_4x16x32x16_f64/numpy | 12.975 / 13.239 | 13.086 / 13.136 | unavailable | unavailable | 33.94 |
| matmul_sum_backward_32x16x8_f64/candle | 7.128 / 7.400 | 7.144 / 7.221 | 113 | 27980 | 3.83 |
| matmul_sum_backward_32x16x8_f64/kernix | 3.512 / 3.576 | 3.512 / 3.543 | 16 | 18376 | 2.83 |
| min_norm_32x64_rank32_f64/kernix | 114.616 / 117.784 | 114.812 / 115.020 | 28 | 102144 | 2.96 |
| min_norm_32x64_rank32_f64/nalgebra | 217.986 / 221.753 | 218.283 / 218.544 | 15 | 42232 | 2.77 |
| min_norm_32x64_rank32_f64/numpy | 176.460 / 192.563 | 180.044 / 182.035 | unavailable | unavailable | 34.93 |
| min_norm_64x32_rank16_f64/kernix | 76.059 / 78.522 | 76.279 / 76.870 | 28 | 74752 | 2.93 |
| min_norm_64x32_rank16_f64/nalgebra | 116.800 / 119.792 | 117.059 / 117.392 | 15 | 41976 | 2.76 |
| min_norm_64x32_rank16_f64/numpy | 93.706 / 105.329 | 95.408 / 95.605 | unavailable | unavailable | 34.81 |
| pivoted_qr_256x64_f64/faer | 474.936 / 482.120 | 508.300 / 515.031 | 58 | 333312 | 4.03 |
| pivoted_qr_256x64_f64/kernix | 786.763 / 800.530 | 768.559 / 784.382 | 14 | 559104 | 3.48 |
| pivoted_qr_256x64_f64/nalgebra | 1151.545 / 1157.857 | 1151.333 / 1153.683 | 7 | 296960 | 3.16 |
| pivoted_qr_96x32_f64/faer | 61.988 / 64.076 | 62.107 / 62.503 | 32 | 68352 | 3.37 |
| pivoted_qr_96x32_f64/kernix | 88.146 / 91.370 | 88.356 / 88.739 | 14 | 107520 | 2.93 |
| pivoted_qr_96x32_f64/nalgebra | 114.304 / 118.552 | 114.552 / 114.988 | 7 | 58368 | 2.73 |
| qr_256x64_f64/faer | 443.554 / 450.697 | 444.625 / 447.693 | 83 | 331776 | 3.78 |
| qr_256x64_f64/kernix | 690.305 / 705.064 | 689.621 / 692.597 | 6 | 426496 | 3.32 |
| qr_256x64_f64/nalgebra | 588.263 / 599.439 | 589.055 / 595.162 | 4 | 295424 | 3.04 |
| qr_256x64_f64/numpy | 576.105 / 591.056 | 570.264 / 595.994 | unavailable | unavailable | 35.62 |
| qr_96x32_f64/faer | 65.356 / 67.891 | 65.525 / 65.672 | 41 | 67584 | 3.29 |
| qr_96x32_f64/kernix | 70.155 / 72.355 | 70.269 / 70.317 | 6 | 82176 | 2.83 |
| qr_96x32_f64/nalgebra | 61.388 / 63.804 | 61.540 / 61.660 | 4 | 57600 | 2.72 |
| qr_96x32_f64/numpy | 65.963 / 81.026 | 67.308 / 68.116 | unavailable | unavailable | 34.89 |
| softmax_256x64_f64/candle | 111.553 / 115.424 | 111.812 / 112.797 | 37 | 265200 | 4.08 |
| softmax_256x64_f64/kernix | 113.000 / 116.544 | 113.318 / 113.787 | 1 | 131072 | 3.00 |
| softmax_256x64_f64/ndarray | 97.266 / 101.105 | 97.544 / 98.274 | 1 | 131072 | 2.88 |
| softmax_256x64_f64/numpy | 134.894 / 145.382 | 136.510 / 137.029 | unavailable | unavailable | 33.89 |
| training_backward_2x4x8x4_f64/candle | 21.263 / 21.686 | 21.358 / 21.539 | 476 | 8996 | 4.09 |
| training_backward_2x4x8x4_f64/kernix | 4.192 / 4.336 | 4.251 / 4.255 | 23 | 4224 | 2.96 |
| training_forward_2x4x8x4_f64/candle | 5.279 / 5.423 | 5.327 / 5.340 | 126 | 3912 | 3.98 |
| training_forward_2x4x8x4_f64/kernix | 3.087 / 3.152 | 3.096 / 3.120 | 17 | 1032 | 2.93 |
| training_record_2x4x8x4_f64/candle | 7.615 / 7.775 | 7.504 / 7.692 | 149 | 10712 | 4.00 |
| training_record_2x4x8x4_f64/kernix | 3.823 / 3.895 | 3.840 / 3.862 | 22 | 6832 | 2.88 |
| training_sgd_2x4x8x4_f64/kernix | 0.736 / 0.768 | 0.753 / 0.757 | 5 | 688 | 3.03 |

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
