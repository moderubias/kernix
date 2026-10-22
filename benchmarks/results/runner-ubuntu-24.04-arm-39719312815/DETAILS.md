# Clean CPU evidence: runner-ubuntu-24.04-arm-39719312815

Measured source `227f339974f9c31fcf6bd37ccccbda3eedc99591`, tree `556afc7a18c74320ec378c75bbefd4422f3d3ef3`; dirty before/after: false.
Library source: workspace 227f339974f9c31fcf6bd37ccccbda3eedc99591. One pinned CPU; dtype explicitly named per workload; row-major inputs.

| Workload / library | Per-call p50 / p95 (us) | Batch-mean p50 / p95 (us) | Rust allocations | Peak additional requested bytes | Process peak RSS (MiB) |
|---|---:|---:|---:|---:|---:|
| cholesky_factor_128_f64/faer | 65.012 / 67.060 | 65.098 / 66.025 | 16 | 148736 | 3.38 |
| cholesky_factor_128_f64/kernix | 279.557 / 282.781 | 278.709 / 279.290 | 1 | 131072 | 3.01 |
| cholesky_factor_128_f64/nalgebra | 128.695 / 132.759 | 130.158 / 130.582 | 1 | 131072 | 2.82 |
| cholesky_factor_128_f64/numpy | 85.178 / 102.641 | 87.063 / 87.653 | unavailable | unavailable | 37.14 |
| cholesky_factor_256_f64/faer | 404.461 / 417.420 | 408.433 / 410.837 | 76 | 592000 | 4.29 |
| cholesky_factor_256_f64/kernix | 2015.201 / 2031.848 | 2006.780 / 2018.334 | 1 | 524288 | 4.14 |
| cholesky_factor_256_f64/nalgebra | 1029.683 / 1036.739 | 1032.847 / 1040.282 | 1 | 524288 | 3.57 |
| cholesky_factor_256_f64/numpy | 821.793 / 837.624 | 819.619 / 822.667 | unavailable | unavailable | 39.98 |
| cholesky_factor_64_f64/faer | 13.231 / 13.255 | 13.277 / 13.297 | 2 | 33280 | 2.79 |
| cholesky_factor_64_f64/kernix | 27.326 / 28.126 | 27.372 / 27.559 | 2 | 33280 | 2.72 |
| cholesky_factor_64_f64/nalgebra | 20.359 / 20.975 | 20.517 / 20.691 | 1 | 32768 | 2.63 |
| cholesky_factor_64_f64/numpy | 18.143 / 18.751 | 18.491 / 18.515 | unavailable | unavailable | 36.98 |
| create_ones_64x64_f32/candle | 0.344 / 0.352 | 0.352 / 0.355 | 7 | 16696 | 3.04 |
| create_ones_64x64_f32/faer | 0.240 / 0.248 | 0.233 / 0.234 | 1 | 16384 | 2.72 |
| create_ones_64x64_f32/kernix | 0.256 / 0.272 | 0.258 / 0.263 | 1 | 16384 | 2.64 |
| create_ones_64x64_f32/ndarray | 0.208 / 0.224 | 0.216 / 0.216 | 1 | 16384 | 2.64 |
| create_ones_64x64_f32/numpy | 1.528 / 1.576 | 1.602 / 1.608 | unavailable | unavailable | 34.62 |
| create_zeros_64x64_f32/candle | 0.240 / 0.256 | 0.249 / 0.253 | 5 | 16648 | 2.91 |
| create_zeros_64x64_f32/faer | 0.320 / 0.328 | 0.313 / 0.317 | 1 | 16384 | 2.72 |
| create_zeros_64x64_f32/kernix | 0.184 / 0.200 | 0.186 / 0.188 | 1 | 16384 | 2.64 |
| create_zeros_64x64_f32/ndarray | 0.136 / 0.144 | 0.140 / 0.143 | 1 | 16384 | 2.64 |
| create_zeros_64x64_f32/numpy | 0.504 / 0.536 | 0.548 / 0.556 | unavailable | unavailable | 37.30 |
| matmul_128_f32/candle | 85.922 / 87.826 | 86.055 / 86.209 | 10 | 131160 | 3.58 |
| matmul_128_f32/faer | 87.203 / 89.122 | 87.385 / 87.589 | 3 | 196616 | 3.21 |
| matmul_128_f32/kernix | 97.833 / 110.857 | 100.201 / 101.023 | 3 | 163840 | 3.13 |
| matmul_128_f32/nalgebra | 97.426 / 110.600 | 99.656 / 100.392 | 2 | 163840 | 2.91 |
| matmul_128_f32/ndarray | 97.337 / 110.200 | 98.968 / 99.544 | 2 | 163840 | 2.91 |
| matmul_128_f32/numpy | 99.833 / 103.442 | 100.499 / 100.867 | unavailable | unavailable | 37.23 |
| matmul_128_f64/candle | 168.765 / 173.069 | 169.527 / 169.941 | 10 | 262232 | 3.92 |
| matmul_128_f64/faer | 169.485 / 173.044 | 169.945 / 170.261 | 3 | 393224 | 3.65 |
| matmul_128_f64/kernix | 225.569 / 236.928 | 225.638 / 228.544 | 3 | 327680 | 3.26 |
| matmul_128_f64/nalgebra | 225.025 / 241.784 | 224.901 / 226.345 | 2 | 327680 | 2.94 |
| matmul_128_f64/ndarray | 224.513 / 234.384 | 224.264 / 224.646 | 2 | 327680 | 3.01 |
| matmul_128_f64/numpy | 194.635 / 199.179 | 196.126 / 196.667 | unavailable | unavailable | 36.45 |
| matmul_256_f32/candle | 724.783 / 766.092 | 727.650 / 738.528 | 10 | 524376 | 4.35 |
| matmul_256_f32/faer | 677.707 / 683.418 | 674.067 / 681.867 | 3 | 786440 | 4.21 |
| matmul_256_f32/kernix | 802.778 / 813.297 | 800.714 / 803.178 | 3 | 589824 | 3.76 |
| matmul_256_f32/nalgebra | 795.611 / 810.466 | 794.073 / 798.200 | 2 | 589824 | 3.38 |
| matmul_256_f32/ndarray | 799.795 / 810.113 | 797.627 / 805.909 | 2 | 589824 | 3.55 |
| matmul_256_f32/numpy | 727.023 / 735.727 | 727.703 / 731.214 | unavailable | unavailable | 40.95 |
| matmul_256_f64/candle | 1321.904 / 1338.455 | 1323.647 / 1327.128 | 10 | 1048664 | 5.53 |
| matmul_256_f64/faer | 1621.235 / 1640.409 | 1616.588 / 1623.275 | 3 | 1048584 | 5.53 |
| matmul_256_f64/kernix | 1332.135 / 1346.430 | 1331.017 / 1337.006 | 3 | 1179648 | 4.95 |
| matmul_256_f64/nalgebra | 1329.142 / 1341.526 | 1329.525 / 1335.130 | 2 | 1179648 | 4.32 |
| matmul_256_f64/ndarray | 1324.464 / 1339.679 | 1322.621 / 1327.681 | 2 | 1179648 | 4.75 |
| matmul_256_f64/numpy | 1463.590 / 1477.621 | 1463.340 / 1470.602 | unavailable | unavailable | 41.25 |
| matmul_32_f64/candle | 3.440 / 3.472 | 3.454 / 3.468 | 10 | 16976 | 3.31 |
| matmul_32_f64/faer | 3.511 / 3.528 | 3.491 / 3.501 | 3 | 25088 | 3.09 |
| matmul_32_f64/kernix | 3.240 / 3.272 | 3.244 / 3.302 | 3 | 24576 | 2.86 |
| matmul_32_f64/nalgebra | 3.216 / 3.247 | 3.236 / 3.278 | 2 | 24576 | 2.72 |
| matmul_32_f64/ndarray | 3.048 / 3.080 | 3.059 / 3.064 | 2 | 24576 | 2.67 |
| matmul_32_f64/numpy | 8.239 / 8.295 | 8.322 / 8.351 | unavailable | unavailable | 35.20 |
| matmul_64_f32/candle | 12.000 / 12.047 | 12.035 / 12.061 | 10 | 33104 | 3.31 |
| matmul_64_f32/faer | 12.287 / 12.311 | 12.314 / 12.329 | 3 | 49408 | 3.05 |
| matmul_64_f32/kernix | 11.528 / 11.559 | 11.558 / 11.576 | 3 | 49152 | 2.89 |
| matmul_64_f32/nalgebra | 11.280 / 11.303 | 11.301 / 11.327 | 2 | 49152 | 2.75 |
| matmul_64_f32/ndarray | 11.263 / 11.295 | 11.293 / 11.298 | 2 | 49152 | 2.71 |
| matmul_64_f32/numpy | 20.118 / 20.703 | 20.315 / 20.364 | unavailable | unavailable | 35.44 |
| matmul_64_f64/candle | 22.559 / 22.607 | 22.618 / 22.659 | 10 | 65872 | 3.46 |
| matmul_64_f64/faer | 22.630 / 22.687 | 22.683 / 22.700 | 3 | 98560 | 3.18 |
| matmul_64_f64/kernix | 22.174 / 22.238 | 22.215 / 22.237 | 3 | 98304 | 3.00 |
| matmul_64_f64/nalgebra | 22.486 / 22.646 | 22.550 / 22.667 | 2 | 98304 | 2.81 |
| matmul_64_f64/ndarray | 21.799 / 21.855 | 21.858 / 21.879 | 2 | 98304 | 2.79 |
| matmul_64_f64/numpy | 31.022 / 32.142 | 31.336 / 31.406 | unavailable | unavailable | 36.31 |
| matmul_batched_4x16x32x16_f64/candle | 4.464 / 4.519 | 4.503 / 4.518 | 25 | 25696 | 3.50 |
| matmul_batched_4x16x32x16_f64/kernix | 3.831 / 3.872 | 3.848 / 3.860 | 7 | 16392 | 2.82 |
| matmul_batched_4x16x32x16_f64/numpy | 12.999 / 13.135 | 13.157 / 13.199 | unavailable | unavailable | 35.23 |
| matmul_sum_backward_32x16x8_f64/candle | 7.223 / 7.495 | 7.157 / 7.204 | 113 | 27980 | 3.64 |
| matmul_sum_backward_32x16x8_f64/kernix | 3.527 / 3.592 | 3.508 / 3.555 | 16 | 18376 | 2.85 |
| qr_256x64_f64/faer | 448.938 / 455.697 | 450.742 / 451.737 | 83 | 331776 | 3.85 |
| qr_256x64_f64/kernix | 917.547 / 925.858 | 918.836 / 921.301 | 6 | 426496 | 3.38 |
| qr_256x64_f64/nalgebra | 593.232 / 607.759 | 592.841 / 594.423 | 4 | 295424 | 3.04 |
| qr_256x64_f64/numpy | 572.866 / 585.049 | 568.133 / 572.067 | unavailable | unavailable | 38.28 |
| qr_96x32_f64/faer | 65.395 / 67.891 | 65.471 / 65.614 | 41 | 67584 | 3.44 |
| qr_96x32_f64/kernix | 89.546 / 91.745 | 89.667 / 90.168 | 6 | 82176 | 2.89 |
| qr_96x32_f64/nalgebra | 60.876 / 62.996 | 61.054 / 61.157 | 4 | 57600 | 2.72 |
| qr_96x32_f64/numpy | 65.963 / 78.594 | 67.579 / 68.000 | unavailable | unavailable | 37.80 |
| softmax_256x64_f64/candle | 110.969 / 113.592 | 111.284 / 111.420 | 37 | 265200 | 3.95 |
| softmax_256x64_f64/kernix | 113.168 / 115.168 | 113.443 / 113.696 | 1 | 131072 | 3.05 |
| softmax_256x64_f64/ndarray | 97.410 / 99.865 | 97.704 / 98.320 | 1 | 131072 | 2.88 |
| softmax_256x64_f64/numpy | 135.151 / 145.118 | 136.781 / 137.276 | unavailable | unavailable | 35.25 |
| training_backward_2x4x8x4_f64/candle | 21.134 / 21.590 | 21.115 / 21.166 | 476 | 8996 | 4.02 |
| training_backward_2x4x8x4_f64/kernix | 4.256 / 4.415 | 4.272 / 4.301 | 23 | 4224 | 3.02 |
| training_forward_2x4x8x4_f64/candle | 5.320 / 5.464 | 5.289 / 5.312 | 126 | 3912 | 3.86 |
| training_forward_2x4x8x4_f64/kernix | 3.023 / 3.088 | 3.119 / 3.125 | 17 | 1032 | 2.95 |
| training_record_2x4x8x4_f64/candle | 7.584 / 7.784 | 7.441 / 7.604 | 149 | 10712 | 3.87 |
| training_record_2x4x8x4_f64/kernix | 3.856 / 3.920 | 3.901 / 3.912 | 22 | 6832 | 2.95 |
| training_sgd_2x4x8x4_f64/kernix | 0.728 / 0.760 | 0.735 / 0.737 | 5 | 688 | 3.02 |

## Method and limits

- Per-call quantiles use nearest ranks over 256 individual warmed calls; batch quantiles use 15 means over separate 10-ms batches. They are separate distributions. Raw samples remain in raw.json.
- Eight warmup calls follow validation. Timing includes allocation, dispatch, computation and output destruction. The first call field excludes output destruction. Inputs and library/runtime imports are prepared outside timing.
- Three extra processes record their first operation calls. These are first-use observations, not cache-flushed or device-startup latency. Timer-pair overhead is reported without subtraction; tiny calls are clock-sensitive.
- ndarray and Candle softmax are explicit stable compositions, not claims about a native softmax API.
- Constructor cases allocate owned 64x64 f32 tensors. Pre-0.6 registry baselines use full(value), explicitly marked in metadata; current workspace runs use zeros/ones. No nested literal reinterpretation is involved.
- All GEMM inputs are C-order and match elementwise. Native output layout differs by library. QR materializes thin Q and R in every implementation. Cholesky returns reusable factors (or NumPy L); no extra factor materialization is timed.
- Kernix checks finite values/symmetry. Some competitors validate a narrower contract. No finite-data precondition check was removed to improve timing. Accuracy results and budgets are retained per worker.
- The backward case uses an actual reverse-mode graph in Kernix and Candle for sum(A @ W), with A constant and W trainable. Both copy input/weight data and returned gradients; no manual derivative is used as a framework ranking.
- Allocations are measured in a separately compiled instrumented binary after warmup, outside all timing loops. Counts cover Rust global-allocator requests, not native allocator internals, stack memory or all NumPy allocations. NumPy allocation counts are unavailable, not zero.
- Native engines may retain thread-local packing buffers. Warmed allocation probes exclude first-use allocations and retained scratch; first-use timing and process RSS are separate observations. See the measured library numerical contract for that version.
- Requested-byte peak is additional live Rust allocation during one call, including output; it is not scratch-only memory or peak RSS. RSS is the process high-water mark including fixtures, validation and runtime, read from Linux VmHWM. Other platforms may report unavailable.
- The published cases are allocation-inclusive CPU APIs and graph workloads. Pure internal kernel-only and end-to-end application/model latency are not measured. No workload result implies a complete application frame rate.
- faer is explicitly serial; Rayon and BLAS thread counts are one. No GPU comparison. Python/NumPy timings include Python dispatch. Backend and dependency versions/configuration, governor and RAM are recorded.
- Training stages use batched matmul + bias + layer_norm + tanh + step-two feature slice + MSE (Candle index_select). Forward excludes graph recording; record includes parameter/input/target snapshots and eager forward/loss; backward reuses a prepared graph and returns fresh derivatives. Allocation probes measure the corresponding stage, including graph values/scratch, not graph-container-only bytes. SGD uses fixed precomputed real gradients and includes transactional validation/parameter updates; it is not an end-to-end converging training loop. Candle normalization is an explicit population-statistics composition; its checks/rounding differ. Historical registry baselines without these Tape operations omit the stages, recorded in metadata, rather than substitute manual derivatives.
- A shared laptop has uncontrolled load/frequency/cache effects. Report comparator wins and losses; no universal ranking follows. The source archive comes from committed bytes, with result directories excluded. Library dependencies are pinned by the saved driver lockfile.
