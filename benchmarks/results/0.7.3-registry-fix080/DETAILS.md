# Clean CPU evidence: 0.7.3-registry-fix080

Measured source `2d62febf07666079a81043b287fe67cdadd2429e`, tree `43ff97aa0002f41890bc1f0ddeb1ffc83b4868a8`; dirty before/after: false.
Library source: registry 0.7.3. One pinned CPU; dtype explicitly named per workload; row-major inputs.

| Workload / library | Per-call p50 / p95 (us) | Batch-mean p50 / p95 (us) | Rust allocations | Peak additional requested bytes | Process peak RSS (MiB) |
|---|---:|---:|---:|---:|---:|
| cholesky_factor_128_f64/faer | 39.410 / 42.380 | 40.066 / 43.548 | 2 | 132096 | 4.88 |
| cholesky_factor_128_f64/kernix | 168.412 / 182.446 | 182.001 / 191.889 | 2 | 262144 | 4.56 |
| cholesky_factor_128_f64/nalgebra | 104.188 / 114.397 | 98.047 / 104.772 | 1 | 131072 | 4.24 |
| cholesky_factor_128_f64/numpy | 73.713 / 97.617 | 73.889 / 75.719 | unavailable | unavailable | 41.85 |
| cholesky_factor_256_f64/faer | 199.262 / 221.896 | 201.370 / 211.821 | 2 | 526336 | 5.80 |
| cholesky_factor_256_f64/kernix | 1219.916 / 1327.383 | 1252.604 / 1361.546 | 2 | 1048576 | 6.00 |
| cholesky_factor_256_f64/nalgebra | 709.381 / 969.988 | 677.320 / 688.046 | 1 | 524288 | 4.97 |
| cholesky_factor_256_f64/numpy | 510.372 / 561.332 | 545.840 / 641.012 | unavailable | unavailable | 45.85 |
| cholesky_factor_64_f64/faer | 8.426 / 9.490 | 8.523 / 8.736 | 2 | 33280 | 4.44 |
| cholesky_factor_64_f64/kernix | 25.208 / 26.064 | 24.953 / 26.347 | 2 | 65536 | 4.16 |
| cholesky_factor_64_f64/nalgebra | 15.047 / 15.173 | 14.904 / 19.843 | 1 | 32768 | 3.87 |
| cholesky_factor_64_f64/numpy | 10.863 / 11.111 | 11.089 / 11.201 | unavailable | unavailable | 41.15 |
| create_ones_64x64_f32/candle | 0.230 / 0.244 | 0.244 / 0.280 | 7 | 16696 | 4.64 |
| create_ones_64x64_f32/faer | 0.189 / 0.201 | 0.178 / 0.181 | 1 | 16384 | 4.11 |
| create_ones_64x64_f32/kernix | 0.204 / 0.207 | 0.204 / 0.214 | 1 | 16384 | 4.07 |
| create_ones_64x64_f32/ndarray | 0.141 / 0.142 | 0.144 / 0.151 | 1 | 16384 | 3.99 |
| create_ones_64x64_f32/numpy | 1.333 / 1.422 | 1.381 / 1.447 | unavailable | unavailable | 40.50 |
| create_zeros_64x64_f32/candle | 0.155 / 0.157 | 0.159 / 0.163 | 5 | 16648 | 4.40 |
| create_zeros_64x64_f32/faer | 0.167 / 0.169 | 0.217 / 0.234 | 1 | 16384 | 4.14 |
| create_zeros_64x64_f32/kernix | 0.146 / 0.148 | 0.151 / 0.171 | 1 | 16384 | 3.86 |
| create_zeros_64x64_f32/ndarray | 0.099 / 0.100 | 0.094 / 0.097 | 1 | 16384 | 3.56 |
| create_zeros_64x64_f32/numpy | 0.389 / 0.423 | 0.402 / 0.412 | unavailable | unavailable | 40.46 |
| matmul_128_f32/candle | 36.484 / 38.617 | 37.713 / 38.784 | 9 | 65816 | 5.16 |
| matmul_128_f32/faer | 41.142 / 47.049 | 42.315 / 43.239 | 1 | 65536 | 4.52 |
| matmul_128_f32/kernix | 50.087 / 62.859 | 49.827 / 59.814 | 3 | 65544 | 4.85 |
| matmul_128_f32/nalgebra | 49.492 / 56.659 | 50.811 / 54.319 | 2 | 163840 | 3.96 |
| matmul_128_f32/ndarray | 49.696 / 57.956 | 51.526 / 52.146 | 2 | 163840 | 4.38 |
| matmul_128_f32/numpy | 39.181 / 43.555 | 41.123 / 44.830 | unavailable | unavailable | 41.78 |
| matmul_128_f64/candle | 76.270 / 97.745 | 76.122 / 79.012 | 9 | 131352 | 5.58 |
| matmul_128_f64/faer | 85.352 / 91.525 | 84.811 / 89.858 | 1 | 131072 | 4.80 |
| matmul_128_f64/kernix | 73.407 / 79.125 | 74.680 / 75.995 | 3 | 131080 | 5.08 |
| matmul_128_f64/nalgebra | 117.090 / 125.140 | 118.883 / 121.800 | 2 | 327680 | 4.30 |
| matmul_128_f64/ndarray | 119.253 / 142.681 | 120.148 / 161.631 | 2 | 327680 | 4.33 |
| matmul_128_f64/numpy | 78.251 / 81.833 | 78.622 / 81.153 | unavailable | unavailable | 41.93 |
| matmul_256_f32/candle | 293.756 / 351.271 | 304.972 / 389.647 | 9 | 262424 | 5.95 |
| matmul_256_f32/faer | 335.476 / 345.639 | 333.625 / 346.320 | 1 | 262144 | 5.25 |
| matmul_256_f32/kernix | 279.287 / 301.625 | 286.798 / 300.705 | 3 | 262152 | 5.30 |
| matmul_256_f32/nalgebra | 424.814 / 475.409 | 436.595 / 557.580 | 2 | 589824 | 4.75 |
| matmul_256_f32/ndarray | 387.241 / 444.442 | 509.062 / 605.653 | 2 | 589824 | 5.18 |
| matmul_256_f32/numpy | 288.021 / 316.664 | 296.234 / 309.700 | unavailable | unavailable | 46.00 |
| matmul_256_f64/candle | 569.457 / 610.440 | 576.774 / 627.376 | 9 | 524568 | 6.97 |
| matmul_256_f64/faer | 750.506 / 888.482 | 764.051 / 984.611 | 1 | 524288 | 6.07 |
| matmul_256_f64/kernix | 570.453 / 666.927 | 585.603 / 695.173 | 3 | 524296 | 6.29 |
| matmul_256_f64/nalgebra | 680.841 / 714.629 | 703.265 / 849.517 | 2 | 1179648 | 5.71 |
| matmul_256_f64/ndarray | 664.428 / 942.223 | 653.466 / 714.492 | 2 | 1179648 | 6.25 |
| matmul_256_f64/numpy | 610.098 / 648.069 | 616.402 / 636.348 | unavailable | unavailable | 45.86 |
| matmul_32_f64/candle | 1.497 / 1.527 | 1.577 / 1.633 | 9 | 8592 | 5.09 |
| matmul_32_f64/faer | 1.708 / 1.861 | 1.539 / 1.621 | 1 | 8192 | 4.55 |
| matmul_32_f64/kernix | 1.737 / 1.766 | 1.763 / 1.821 | 3 | 24576 | 4.41 |
| matmul_32_f64/nalgebra | 1.495 / 1.913 | 1.656 / 1.861 | 2 | 24576 | 3.91 |
| matmul_32_f64/ndarray | 1.467 / 1.486 | 1.530 / 1.562 | 2 | 24576 | 4.07 |
| matmul_32_f64/numpy | 2.599 / 4.089 | 2.601 / 2.664 | unavailable | unavailable | 40.76 |
| matmul_64_f32/candle | 5.163 / 5.415 | 5.230 / 5.583 | 9 | 16664 | 5.16 |
| matmul_64_f32/faer | 5.295 / 5.469 | 5.144 / 5.264 | 1 | 16384 | 4.39 |
| matmul_64_f32/kernix | 5.094 / 5.359 | 5.095 / 5.336 | 3 | 16544 | 4.49 |
| matmul_64_f32/nalgebra | 6.069 / 6.520 | 6.060 / 6.204 | 2 | 49152 | 4.23 |
| matmul_64_f32/ndarray | 6.968 / 7.853 | 7.131 / 8.152 | 2 | 49152 | 4.18 |
| matmul_64_f32/numpy | 6.179 / 6.281 | 6.262 / 6.610 | unavailable | unavailable | 41.00 |
| matmul_64_f64/candle | 10.108 / 10.525 | 10.497 / 10.796 | 9 | 33048 | 5.13 |
| matmul_64_f64/faer | 10.842 / 11.091 | 11.182 / 11.606 | 1 | 32768 | 4.51 |
| matmul_64_f64/kernix | 9.665 / 11.121 | 9.982 / 11.177 | 3 | 32928 | 4.77 |
| matmul_64_f64/nalgebra | 11.250 / 11.865 | 11.502 / 12.830 | 2 | 98304 | 4.18 |
| matmul_64_f64/ndarray | 10.600 / 11.444 | 11.014 / 11.222 | 2 | 98304 | 4.23 |
| matmul_64_f64/numpy | 11.917 / 15.155 | 12.689 / 13.492 | unavailable | unavailable | 41.08 |
| matmul_batched_4x16x32x16_f64/candle | 2.194 / 21.138 | 2.249 / 2.309 | 25 | 25504 | 5.59 |
| matmul_batched_4x16x32x16_f64/kernix | 1.964 / 2.003 | 2.048 / 2.095 | 7 | 16392 | 4.49 |
| matmul_batched_4x16x32x16_f64/numpy | 3.070 / 4.694 | 3.249 / 4.139 | unavailable | unavailable | 40.74 |
| matmul_sum_backward_32x16x8_f64/candle | 8.284 / 10.460 | 5.971 / 7.910 | 113 | 27988 | 5.78 |
| matmul_sum_backward_32x16x8_f64/kernix | 2.656 / 2.714 | 2.650 / 2.751 | 16 | 18376 | 4.45 |
| qr_256x64_f64/faer | 435.016 / 597.215 | 441.612 / 471.487 | 7 | 331776 | 5.11 |
| qr_256x64_f64/kernix | 514.650 / 598.272 | 512.857 / 569.300 | 6 | 557568 | 4.90 |
| qr_256x64_f64/nalgebra | 376.353 / 406.959 | 379.343 / 414.547 | 4 | 295424 | 4.40 |
| qr_256x64_f64/numpy | 348.518 / 365.182 | 349.993 / 358.007 | unavailable | unavailable | 42.49 |
| qr_96x32_f64/faer | 74.457 / 94.937 | 75.337 / 95.719 | 7 | 67584 | 4.52 |
| qr_96x32_f64/kernix | 47.046 / 61.193 | 47.694 / 50.001 | 6 | 106752 | 4.35 |
| qr_96x32_f64/nalgebra | 37.902 / 42.320 | 38.224 / 39.943 | 4 | 57600 | 4.10 |
| qr_96x32_f64/numpy | 40.026 / 56.226 | 40.666 / 43.257 | unavailable | unavailable | 41.63 |
| softmax_256x64_f64/candle | 82.419 / 89.415 | 81.255 / 85.966 | 37 | 265200 | 5.93 |
| softmax_256x64_f64/kernix | 85.337 / 92.686 | 86.186 / 86.874 | 1 | 131072 | 4.76 |
| softmax_256x64_f64/ndarray | 72.206 / 80.113 | 73.815 / 84.116 | 1 | 131072 | 4.46 |
| softmax_256x64_f64/numpy | 108.198 / 116.846 | 108.394 / 111.854 | unavailable | unavailable | 40.85 |
| training_backward_2x4x8x4_f64/candle | 14.658 / 19.148 | 14.886 / 15.208 | 472 | 8332 | 6.23 |
| training_backward_2x4x8x4_f64/kernix | 3.102 / 3.286 | 3.233 / 3.280 | 23 | 4224 | 4.30 |
| training_forward_2x4x8x4_f64/candle | 3.524 / 3.644 | 3.566 / 3.945 | 126 | 3912 | 5.89 |
| training_forward_2x4x8x4_f64/kernix | 2.238 / 2.318 | 2.395 / 2.486 | 17 | 1032 | 4.52 |
| training_record_2x4x8x4_f64/candle | 5.153 / 5.434 | 5.509 / 6.073 | 149 | 10712 | 5.93 |
| training_record_2x4x8x4_f64/kernix | 2.802 / 2.903 | 2.808 / 2.996 | 22 | 6832 | 4.49 |
| training_sgd_2x4x8x4_f64/kernix | 0.532 / 0.560 | 0.518 / 0.534 | 5 | 688 | 4.44 |

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
