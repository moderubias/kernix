# Clean CPU evidence: 0.7.2-clean-5067333

Measured source `50673332cd664b98fad90a24febce1fd8d6395b0`, tree `9dc8cc2cdabf93638a96e4ea87d759c65106acb1`; dirty before/after: false.
Library source: workspace 50673332cd664b98fad90a24febce1fd8d6395b0. One pinned CPU; f64 operations and explicitly named f32 constructors; row-major inputs.

| Workload / library | Per-call p50 / p95 (us) | Batch-mean p50 / p95 (us) | Rust allocations | Peak additional requested bytes | Process peak RSS (MiB) |
|---|---:|---:|---:|---:|---:|
| cholesky_factor_64_f64/faer | 7.263 / 8.909 | 7.647 / 7.805 | 2 | 33280 | 4.13 |
| cholesky_factor_64_f64/kernix | 25.250 / 26.945 | 24.014 / 25.637 | 2 | 65536 | 4.11 |
| cholesky_factor_64_f64/nalgebra | 13.459 / 16.433 | 14.243 / 14.902 | 1 | 32768 | 3.59 |
| cholesky_factor_64_f64/numpy | 10.064 / 10.702 | 10.806 / 10.901 | unavailable | unavailable | 32.69 |
| create_ones_64x64_f32/candle | 0.237 / 0.250 | 0.236 / 0.255 | 7 | 16696 | 4.67 |
| create_ones_64x64_f32/faer | 0.201 / 0.203 | 0.174 / 0.184 | 1 | 16384 | 4.17 |
| create_ones_64x64_f32/kernix | 0.185 / 0.196 | 0.205 / 0.219 | 1 | 16384 | 3.98 |
| create_ones_64x64_f32/ndarray | 0.144 / 0.146 | 0.145 / 0.150 | 1 | 16384 | 3.89 |
| create_ones_64x64_f32/numpy | 1.234 / 2.057 | 1.355 / 1.506 | unavailable | unavailable | 32.25 |
| create_zeros_64x64_f32/candle | 0.167 / 0.171 | 0.156 / 0.171 | 5 | 16648 | 4.44 |
| create_zeros_64x64_f32/faer | 0.172 / 0.174 | 0.166 / 0.178 | 1 | 16384 | 4.12 |
| create_zeros_64x64_f32/kernix | 0.145 / 0.148 | 0.136 / 0.154 | 1 | 16384 | 3.87 |
| create_zeros_64x64_f32/ndarray | 0.089 / 0.091 | 0.095 / 0.104 | 1 | 16384 | 3.84 |
| create_zeros_64x64_f32/numpy | 0.423 / 0.491 | 0.414 / 0.620 | unavailable | unavailable | 32.13 |
| matmul_128_f64/candle | 71.202 / 74.818 | 73.802 / 78.153 | 9 | 131352 | 5.39 |
| matmul_128_f64/faer | 81.040 / 85.292 | 81.086 / 83.050 | 1 | 131072 | 4.61 |
| matmul_128_f64/kernix | 68.456 / 74.700 | 73.188 / 88.220 | 3 | 131080 | 4.76 |
| matmul_128_f64/nalgebra | 112.896 / 122.548 | 113.560 / 115.435 | 2 | 327680 | 4.27 |
| matmul_128_f64/ndarray | 115.016 / 139.905 | 117.888 / 122.722 | 2 | 327680 | 4.47 |
| matmul_128_f64/numpy | 75.573 / 83.409 | 78.142 / 79.860 | unavailable | unavailable | 33.65 |
| matmul_256_f64/candle | 560.287 / 627.886 | 569.419 / 583.603 | 9 | 524568 | 6.84 |
| matmul_256_f64/faer | 711.270 / 738.716 | 707.382 / 722.513 | 1 | 524288 | 5.80 |
| matmul_256_f64/kernix | 548.673 / 570.653 | 554.162 / 636.864 | 3 | 524296 | 6.00 |
| matmul_256_f64/nalgebra | 640.174 / 694.239 | 659.940 / 778.341 | 2 | 1179648 | 5.55 |
| matmul_256_f64/ndarray | 632.796 / 849.235 | 664.414 / 739.720 | 2 | 1179648 | 6.01 |
| matmul_256_f64/numpy | 592.335 / 627.930 | 599.822 / 680.993 | unavailable | unavailable | 38.38 |
| matmul_32_f64/candle | 1.573 / 1.859 | 1.580 / 1.730 | 9 | 8592 | 4.87 |
| matmul_32_f64/faer | 1.631 / 1.864 | 1.537 / 1.609 | 1 | 8192 | 4.36 |
| matmul_32_f64/kernix | 1.535 / 1.832 | 1.591 / 1.694 | 3 | 24576 | 4.13 |
| matmul_32_f64/nalgebra | 1.504 / 1.689 | 1.482 / 1.617 | 2 | 24576 | 3.94 |
| matmul_32_f64/ndarray | 1.402 / 1.537 | 1.437 / 1.559 | 2 | 24576 | 3.95 |
| matmul_32_f64/numpy | 2.741 / 2.903 | 2.693 / 2.768 | unavailable | unavailable | 32.41 |
| matmul_64_f64/candle | 9.915 / 10.382 | 10.041 / 10.211 | 9 | 33048 | 4.90 |
| matmul_64_f64/faer | 10.448 / 11.229 | 11.368 / 11.591 | 1 | 32768 | 4.56 |
| matmul_64_f64/kernix | 9.120 / 10.122 | 9.814 / 10.181 | 3 | 32928 | 4.47 |
| matmul_64_f64/nalgebra | 11.415 / 14.019 | 10.987 / 11.727 | 2 | 98304 | 4.04 |
| matmul_64_f64/ndarray | 10.517 / 10.602 | 10.884 / 11.092 | 2 | 98304 | 3.96 |
| matmul_64_f64/numpy | 12.512 / 12.755 | 12.191 / 13.524 | unavailable | unavailable | 32.75 |
| matmul_batched_4x16x32x16_f64/candle | 1.974 / 2.120 | 2.106 / 2.140 | 25 | 25504 | 5.52 |
| matmul_batched_4x16x32x16_f64/kernix | 1.900 / 2.358 | 2.019 / 2.059 | 7 | 16392 | 4.25 |
| matmul_batched_4x16x32x16_f64/numpy | 3.117 / 3.539 | 3.097 / 3.588 | unavailable | unavailable | 32.45 |
| matmul_sum_backward_32x16x8_f64/candle | 6.678 / 8.129 | 4.665 / 7.348 | 113 | 27988 | 5.52 |
| matmul_sum_backward_32x16x8_f64/kernix | 2.382 / 3.187 | 2.554 / 2.730 | 16 | 18376 | 4.11 |
| qr_256x64_f64/faer | 405.668 / 476.858 | 406.576 / 416.786 | 7 | 331776 | 5.04 |
| qr_256x64_f64/kernix | 487.300 / 523.138 | 493.722 / 512.017 | 6 | 557568 | 4.75 |
| qr_256x64_f64/nalgebra | 363.922 / 387.016 | 375.188 / 466.433 | 4 | 295424 | 4.32 |
| qr_256x64_f64/numpy | 341.148 / 362.789 | 348.274 / 366.508 | unavailable | unavailable | 33.58 |
| qr_96x32_f64/faer | 65.928 / 73.813 | 67.590 / 70.669 | 7 | 67584 | 4.56 |
| qr_96x32_f64/kernix | 46.649 / 49.949 | 44.850 / 47.506 | 6 | 106752 | 4.30 |
| qr_96x32_f64/nalgebra | 35.130 / 37.824 | 36.474 / 39.632 | 4 | 57600 | 3.99 |
| qr_96x32_f64/numpy | 38.736 / 56.560 | 39.178 / 41.787 | unavailable | unavailable | 32.75 |
| softmax_256x64_f64/candle | 78.182 / 85.697 | 79.102 / 80.792 | 37 | 265200 | 5.80 |
| softmax_256x64_f64/kernix | 86.258 / 115.582 | 88.117 / 90.163 | 1 | 131072 | 4.34 |
| softmax_256x64_f64/ndarray | 70.921 / 96.905 | 74.530 / 77.087 | 1 | 131072 | 4.18 |
| softmax_256x64_f64/numpy | 102.402 / 111.229 | 103.685 / 109.104 | unavailable | unavailable | 32.72 |
| training_backward_2x4x8x4_f64/candle | 14.641 / 18.222 | 14.302 / 15.583 | 472 | 8332 | 5.82 |
| training_backward_2x4x8x4_f64/kernix | 3.154 / 3.255 | 3.290 / 3.425 | 23 | 4224 | 4.51 |
| training_forward_2x4x8x4_f64/candle | 3.479 / 3.706 | 3.472 / 3.762 | 126 | 3912 | 5.73 |
| training_forward_2x4x8x4_f64/kernix | 1.966 / 2.701 | 2.133 / 2.209 | 17 | 1032 | 4.47 |
| training_record_2x4x8x4_f64/candle | 5.299 / 7.452 | 5.269 / 6.011 | 149 | 10712 | 5.84 |
| training_record_2x4x8x4_f64/kernix | 2.438 / 2.512 | 2.652 / 2.798 | 22 | 6832 | 4.33 |
| training_sgd_2x4x8x4_f64/kernix | 0.489 / 0.514 | 0.518 / 0.555 | 5 | 688 | 4.44 |

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
