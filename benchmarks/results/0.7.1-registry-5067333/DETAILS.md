# Clean CPU evidence: 0.7.1-registry-5067333

Measured source `50673332cd664b98fad90a24febce1fd8d6395b0`, tree `9dc8cc2cdabf93638a96e4ea87d759c65106acb1`; dirty before/after: false.
Library source: registry 0.7.1. One pinned CPU; f64 operations and explicitly named f32 constructors; row-major inputs.

| Workload / library | Per-call p50 / p95 (us) | Batch-mean p50 / p95 (us) | Rust allocations | Peak additional requested bytes | Process peak RSS (MiB) |
|---|---:|---:|---:|---:|---:|
| cholesky_factor_64_f64/faer | 8.147 / 9.733 | 8.663 / 9.394 | 2 | 33280 | 4.14 |
| cholesky_factor_64_f64/kernix | 22.754 / 28.134 | 23.828 / 25.925 | 2 | 65536 | 4.17 |
| cholesky_factor_64_f64/nalgebra | 13.762 / 16.127 | 14.128 / 15.075 | 1 | 32768 | 3.50 |
| cholesky_factor_64_f64/numpy | 10.397 / 12.994 | 11.050 / 14.373 | unavailable | unavailable | 33.12 |
| create_ones_64x64_f32/candle | 0.225 / 0.238 | 0.238 / 0.243 | 7 | 16696 | 4.67 |
| create_ones_64x64_f32/faer | 0.179 / 0.188 | 0.178 / 0.182 | 1 | 16384 | 4.05 |
| create_ones_64x64_f32/kernix | 0.193 / 0.196 | 0.200 / 0.237 | 1 | 16384 | 3.96 |
| create_ones_64x64_f32/ndarray | 0.147 / 0.148 | 0.146 / 0.152 | 1 | 16384 | 3.87 |
| create_ones_64x64_f32/numpy | 1.352 / 3.084 | 1.396 / 1.492 | unavailable | unavailable | 32.80 |
| create_zeros_64x64_f32/candle | 0.169 / 0.171 | 0.162 / 0.166 | 5 | 16648 | 4.41 |
| create_zeros_64x64_f32/faer | 0.189 / 0.197 | 0.174 / 0.184 | 1 | 16384 | 4.06 |
| create_zeros_64x64_f32/kernix | 0.148 / 0.151 | 0.145 / 0.150 | 1 | 16384 | 3.83 |
| create_zeros_64x64_f32/ndarray | 0.101 / 0.102 | 0.098 / 0.106 | 1 | 16384 | 3.86 |
| create_zeros_64x64_f32/numpy | 0.434 / 0.506 | 0.423 / 0.433 | unavailable | unavailable | 32.77 |
| matmul_128_f64/candle | 74.649 / 80.475 | 77.327 / 80.384 | 9 | 131352 | 5.36 |
| matmul_128_f64/faer | 85.355 / 92.573 | 86.114 / 87.973 | 1 | 131072 | 4.54 |
| matmul_128_f64/kernix | 120.337 / 139.128 | 122.893 / 136.615 | 3 | 327680 | 4.52 |
| matmul_128_f64/nalgebra | 117.145 / 148.170 | 118.950 / 123.768 | 2 | 327680 | 4.36 |
| matmul_128_f64/ndarray | 119.341 / 126.898 | 120.267 / 123.004 | 2 | 327680 | 4.11 |
| matmul_128_f64/numpy | 77.968 / 84.409 | 80.915 / 85.093 | unavailable | unavailable | 34.19 |
| matmul_256_f64/candle | 578.302 / 625.762 | 609.930 / 673.541 | 9 | 524568 | 6.84 |
| matmul_256_f64/faer | 744.669 / 963.055 | 763.170 / 849.508 | 1 | 524288 | 6.05 |
| matmul_256_f64/kernix | 674.471 / 775.854 | 676.017 / 738.430 | 3 | 1179648 | 6.29 |
| matmul_256_f64/nalgebra | 662.539 / 732.872 | 663.313 / 676.860 | 2 | 1179648 | 5.55 |
| matmul_256_f64/ndarray | 665.291 / 766.643 | 666.773 / 705.241 | 2 | 1179648 | 5.95 |
| matmul_256_f64/numpy | 607.544 / 634.983 | 625.425 / 670.380 | unavailable | unavailable | 38.98 |
| matmul_32_f64/candle | 1.533 / 1.784 | 1.596 / 1.675 | 9 | 8592 | 4.38 |
| matmul_32_f64/faer | 1.616 / 1.866 | 1.596 / 1.619 | 1 | 8192 | 4.45 |
| matmul_32_f64/kernix | 1.648 / 1.670 | 1.682 / 1.697 | 3 | 24576 | 3.99 |
| matmul_32_f64/nalgebra | 1.529 / 2.006 | 1.515 / 1.572 | 2 | 24576 | 3.94 |
| matmul_32_f64/ndarray | 1.396 / 1.434 | 1.530 / 1.570 | 2 | 24576 | 3.97 |
| matmul_32_f64/numpy | 2.813 / 3.081 | 2.744 / 2.832 | unavailable | unavailable | 33.02 |
| matmul_64_f64/candle | 10.010 / 11.500 | 10.425 / 10.656 | 9 | 33048 | 5.03 |
| matmul_64_f64/faer | 10.951 / 11.882 | 11.569 / 11.843 | 1 | 32768 | 4.48 |
| matmul_64_f64/kernix | 11.918 / 13.035 | 12.260 / 15.017 | 3 | 98304 | 4.23 |
| matmul_64_f64/nalgebra | 11.308 / 12.587 | 11.448 / 11.997 | 2 | 98304 | 4.01 |
| matmul_64_f64/ndarray | 11.235 / 11.685 | 11.455 / 14.425 | 2 | 98304 | 3.98 |
| matmul_64_f64/numpy | 12.842 / 14.127 | 12.672 / 13.385 | unavailable | unavailable | 33.30 |
| matmul_batched_4x16x32x16_f64/candle | 2.084 / 2.375 | 2.273 / 2.410 | 25 | 25504 | 5.34 |
| matmul_batched_4x16x32x16_f64/kernix | 1.993 / 2.658 | 2.203 / 2.573 | 7 | 16392 | 4.09 |
| matmul_batched_4x16x32x16_f64/numpy | 3.263 / 3.431 | 3.220 / 3.329 | unavailable | unavailable | 33.00 |
| matmul_sum_backward_32x16x8_f64/candle | 4.326 / 10.945 | 4.324 / 4.504 | 113 | 27988 | 5.47 |
| matmul_sum_backward_32x16x8_f64/kernix | 2.583 / 3.064 | 2.523 / 2.575 | 16 | 18376 | 4.24 |
| qr_256x64_f64/faer | 419.047 / 458.444 | 400.748 / 415.880 | 7 | 331776 | 4.80 |
| qr_256x64_f64/kernix | 495.470 / 549.705 | 509.057 / 530.726 | 6 | 557568 | 4.83 |
| qr_256x64_f64/nalgebra | 369.394 / 393.127 | 373.487 / 419.352 | 4 | 295424 | 4.16 |
| qr_256x64_f64/numpy | 346.813 / 365.088 | 348.347 / 427.545 | unavailable | unavailable | 34.05 |
| qr_96x32_f64/faer | 64.548 / 69.724 | 67.281 / 75.993 | 7 | 67584 | 4.35 |
| qr_96x32_f64/kernix | 41.627 / 48.361 | 44.276 / 46.236 | 6 | 106752 | 4.25 |
| qr_96x32_f64/nalgebra | 36.761 / 42.103 | 37.634 / 38.050 | 4 | 57600 | 3.95 |
| qr_96x32_f64/numpy | 39.043 / 44.861 | 40.397 / 46.767 | unavailable | unavailable | 33.21 |
| softmax_256x64_f64/candle | 77.958 / 87.607 | 78.275 / 79.101 | 37 | 265200 | 5.86 |
| softmax_256x64_f64/kernix | 83.192 / 88.084 | 84.139 / 86.350 | 1 | 131072 | 4.28 |
| softmax_256x64_f64/ndarray | 72.227 / 77.826 | 75.651 / 76.398 | 1 | 131072 | 4.30 |
| softmax_256x64_f64/numpy | 100.630 / 107.145 | 101.475 / 102.290 | unavailable | unavailable | 33.15 |
| training_backward_2x4x8x4_f64/candle | 14.247 / 15.234 | 15.202 / 15.563 | 472 | 8332 | 5.79 |
| training_backward_2x4x8x4_f64/kernix | 3.232 / 3.315 | 3.373 / 3.486 | 23 | 4224 | 4.33 |
| training_forward_2x4x8x4_f64/candle | 3.523 / 4.421 | 3.827 / 4.063 | 126 | 3912 | 5.61 |
| training_forward_2x4x8x4_f64/kernix | 2.063 / 2.509 | 2.362 / 2.537 | 17 | 1032 | 4.40 |
| training_record_2x4x8x4_f64/candle | 5.375 / 6.070 | 5.528 / 6.189 | 149 | 10712 | 5.71 |
| training_record_2x4x8x4_f64/kernix | 2.563 / 2.714 | 2.915 / 3.026 | 22 | 6832 | 4.29 |
| training_sgd_2x4x8x4_f64/kernix | 0.596 / 0.635 | 0.566 / 0.948 | 5 | 688 | 4.11 |

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
