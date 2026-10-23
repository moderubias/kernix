# Clean CPU evidence: 0.9.0-final

Measured source `4e40ea5a98e7f98505da7cc3dd8a2d80c1a4bd92`, tree `885fd4628515aa5df5de2609d5ec4ca9ab82ef63`; dirty before/after: false.
Library source: workspace 4e40ea5a98e7f98505da7cc3dd8a2d80c1a4bd92. One pinned CPU; dtype explicitly named per workload; row-major inputs.

| Workload / library | Per-call p50 / p95 (us) | Batch-mean p50 / p95 (us) | Rust allocations | Peak additional requested bytes | Process peak RSS (MiB) |
|---|---:|---:|---:|---:|---:|
| cholesky_factor_128_f64/faer | 37.323 / 39.965 | 37.979 / 39.174 | 2 | 132096 | 5.04 |
| cholesky_factor_128_f64/kernix | 122.243 / 164.045 | 122.309 / 125.365 | 1 | 131072 | 4.34 |
| cholesky_factor_128_f64/nalgebra | 96.829 / 108.260 | 100.227 / 115.237 | 1 | 131072 | 4.18 |
| cholesky_factor_128_f64/numpy | 68.058 / 81.081 | 71.060 / 72.520 | unavailable | unavailable | 36.73 |
| cholesky_factor_256_f64/faer | 191.370 / 211.082 | 195.984 / 274.523 | 2 | 526336 | 6.00 |
| cholesky_factor_256_f64/kernix | 619.867 / 660.239 | 625.229 / 645.360 | 1 | 524288 | 5.57 |
| cholesky_factor_256_f64/nalgebra | 639.843 / 705.145 | 665.143 / 724.438 | 1 | 524288 | 4.88 |
| cholesky_factor_256_f64/numpy | 510.375 / 664.620 | 536.078 / 626.821 | unavailable | unavailable | 40.82 |
| cholesky_factor_64_f64/faer | 8.396 / 8.503 | 8.466 / 8.611 | 2 | 33280 | 4.49 |
| cholesky_factor_64_f64/kernix | 20.684 / 24.251 | 20.600 / 21.934 | 2 | 33280 | 4.15 |
| cholesky_factor_64_f64/nalgebra | 13.137 / 14.324 | 14.202 / 14.642 | 1 | 32768 | 3.95 |
| cholesky_factor_64_f64/numpy | 10.231 / 12.001 | 10.978 / 11.557 | unavailable | unavailable | 36.13 |
| create_ones_64x64_f32/candle | 0.237 / 0.251 | 0.232 / 0.242 | 7 | 16696 | 4.77 |
| create_ones_64x64_f32/faer | 0.184 / 0.188 | 0.171 / 0.176 | 1 | 16384 | 3.86 |
| create_ones_64x64_f32/kernix | 0.203 / 0.210 | 0.206 / 0.215 | 1 | 16384 | 3.96 |
| create_ones_64x64_f32/ndarray | 0.154 / 0.156 | 0.147 / 0.158 | 1 | 16384 | 3.98 |
| create_ones_64x64_f32/numpy | 1.348 / 1.410 | 1.429 / 1.819 | unavailable | unavailable | 35.11 |
| create_zeros_64x64_f32/candle | 0.155 / 0.169 | 0.165 / 0.174 | 5 | 16648 | 4.58 |
| create_zeros_64x64_f32/faer | 0.141 / 0.163 | 0.180 / 0.269 | 1 | 16384 | 4.49 |
| create_zeros_64x64_f32/kernix | 0.133 / 0.148 | 0.139 / 0.149 | 1 | 16384 | 3.91 |
| create_zeros_64x64_f32/ndarray | 0.098 / 0.100 | 0.101 / 0.103 | 1 | 16384 | 3.73 |
| create_zeros_64x64_f32/numpy | 0.353 / 0.379 | 0.399 / 0.675 | unavailable | unavailable | 35.07 |
| matmul_128_f32/candle | 34.787 / 38.817 | 36.490 / 38.173 | 9 | 65816 | 5.18 |
| matmul_128_f32/faer | 41.289 / 45.808 | 40.919 / 42.226 | 1 | 65536 | 4.71 |
| matmul_128_f32/kernix | 36.757 / 37.419 | 37.426 / 41.994 | 3 | 65544 | 4.89 |
| matmul_128_f32/nalgebra | 49.789 / 62.142 | 49.374 / 54.662 | 2 | 163840 | 4.11 |
| matmul_128_f32/ndarray | 53.503 / 63.640 | 52.432 / 57.163 | 2 | 163840 | 4.30 |
| matmul_128_f32/numpy | 36.345 / 40.499 | 38.443 / 41.564 | unavailable | unavailable | 36.77 |
| matmul_128_f64/candle | 74.702 / 79.190 | 76.356 / 77.347 | 9 | 131352 | 5.45 |
| matmul_128_f64/faer | 78.058 / 91.215 | 79.171 / 81.508 | 1 | 131072 | 4.98 |
| matmul_128_f64/kernix | 73.171 / 79.248 | 75.237 / 76.587 | 3 | 131080 | 4.91 |
| matmul_128_f64/nalgebra | 117.273 / 138.966 | 119.088 / 127.163 | 2 | 327680 | 4.23 |
| matmul_128_f64/ndarray | 119.441 / 145.415 | 114.800 / 118.293 | 2 | 327680 | 4.38 |
| matmul_128_f64/numpy | 92.307 / 113.670 | 78.330 / 79.608 | unavailable | unavailable | 36.90 |
| matmul_256_f32/candle | 276.250 / 303.301 | 279.823 / 294.387 | 9 | 262424 | 6.03 |
| matmul_256_f32/faer | 306.861 / 370.735 | 308.059 / 345.483 | 1 | 262144 | 5.19 |
| matmul_256_f32/kernix | 270.542 / 289.435 | 275.990 / 295.569 | 3 | 262152 | 5.55 |
| matmul_256_f32/nalgebra | 382.811 / 457.961 | 410.857 / 556.659 | 2 | 589824 | 4.45 |
| matmul_256_f32/ndarray | 394.088 / 447.815 | 410.405 / 442.048 | 2 | 589824 | 4.91 |
| matmul_256_f32/numpy | 285.924 / 370.581 | 284.389 / 297.413 | unavailable | unavailable | 40.89 |
| matmul_256_f64/candle | 557.263 / 629.532 | 556.656 / 583.197 | 9 | 524568 | 7.00 |
| matmul_256_f64/faer | 686.047 / 794.971 | 704.785 / 909.786 | 1 | 524288 | 5.84 |
| matmul_256_f64/kernix | 556.114 / 636.640 | 560.600 / 602.516 | 3 | 524296 | 6.06 |
| matmul_256_f64/nalgebra | 636.472 / 707.628 | 678.537 / 794.258 | 2 | 1179648 | 5.52 |
| matmul_256_f64/ndarray | 664.653 / 824.101 | 660.291 / 676.368 | 2 | 1179648 | 6.23 |
| matmul_256_f64/numpy | 597.657 / 768.129 | 610.204 / 853.491 | unavailable | unavailable | 41.66 |
| matmul_32_f64/candle | 1.420 / 1.455 | 1.534 / 1.858 | 9 | 8592 | 4.97 |
| matmul_32_f64/faer | 1.442 / 1.505 | 1.497 / 1.526 | 1 | 8192 | 4.61 |
| matmul_32_f64/kernix | 1.637 / 1.667 | 1.660 / 1.755 | 3 | 24576 | 4.48 |
| matmul_32_f64/nalgebra | 1.650 / 1.683 | 1.628 / 1.732 | 2 | 24576 | 3.86 |
| matmul_32_f64/ndarray | 1.394 / 1.417 | 1.444 / 1.510 | 2 | 24576 | 4.10 |
| matmul_32_f64/numpy | 2.388 / 2.432 | 2.542 / 2.565 | unavailable | unavailable | 35.76 |
| matmul_64_f32/candle | 4.885 / 4.953 | 4.952 / 5.148 | 9 | 16664 | 5.03 |
| matmul_64_f32/faer | 5.708 / 5.770 | 5.315 / 5.682 | 1 | 16384 | 4.49 |
| matmul_64_f32/kernix | 5.213 / 5.418 | 5.045 / 5.256 | 3 | 16544 | 4.29 |
| matmul_64_f32/nalgebra | 5.642 / 6.107 | 5.845 / 5.988 | 2 | 49152 | 4.12 |
| matmul_64_f32/ndarray | 7.242 / 8.808 | 6.277 / 7.324 | 2 | 49152 | 4.18 |
| matmul_64_f32/numpy | 6.276 / 8.692 | 6.154 / 6.616 | unavailable | unavailable | 35.76 |
| matmul_64_f64/candle | 9.957 / 10.715 | 10.824 / 15.354 | 9 | 33048 | 5.04 |
| matmul_64_f64/faer | 11.285 / 12.614 | 11.797 / 12.046 | 1 | 32768 | 4.66 |
| matmul_64_f64/kernix | 9.483 / 10.274 | 10.040 / 10.382 | 3 | 32928 | 4.70 |
| matmul_64_f64/nalgebra | 10.468 / 11.254 | 11.262 / 11.487 | 2 | 98304 | 4.07 |
| matmul_64_f64/ndarray | 11.334 / 11.429 | 11.122 / 11.691 | 2 | 98304 | 4.16 |
| matmul_64_f64/numpy | 11.158 / 13.271 | 12.042 / 12.730 | unavailable | unavailable | 35.88 |
| matmul_batched_4x16x32x16_f64/candle | 2.103 / 2.327 | 2.058 / 2.109 | 25 | 25504 | 5.64 |
| matmul_batched_4x16x32x16_f64/kernix | 2.160 / 2.190 | 2.104 / 2.504 | 7 | 16392 | 4.39 |
| matmul_batched_4x16x32x16_f64/numpy | 2.865 / 2.928 | 2.951 / 4.006 | unavailable | unavailable | 35.76 |
| matmul_sum_backward_32x16x8_f64/candle | 4.268 / 4.433 | 4.149 / 4.294 | 113 | 27988 | 5.70 |
| matmul_sum_backward_32x16x8_f64/kernix | 2.366 / 2.558 | 2.503 / 2.613 | 16 | 18376 | 3.97 |
| min_norm_32x64_rank32_f64/kernix | 76.670 / 88.084 | 76.731 / 82.793 | 28 | 102144 | 4.51 |
| min_norm_32x64_rank32_f64/nalgebra | 129.405 / 141.829 | 132.849 / 236.227 | 15 | 42232 | 4.18 |
| min_norm_32x64_rank32_f64/numpy | 115.946 / 130.093 | 119.327 / 126.840 | unavailable | unavailable | 36.50 |
| min_norm_64x32_rank16_f64/kernix | 49.903 / 69.188 | 49.194 / 67.792 | 28 | 74752 | 4.57 |
| min_norm_64x32_rank16_f64/nalgebra | 71.614 / 77.672 | 74.527 / 77.554 | 15 | 41976 | 4.34 |
| min_norm_64x32_rank16_f64/numpy | 63.224 / 67.274 | 64.998 / 66.938 | unavailable | unavailable | 36.54 |
| pivoted_qr_256x64_f64/faer | 466.149 / 523.328 | 476.826 / 505.015 | 10 | 333312 | 5.02 |
| pivoted_qr_256x64_f64/kernix | 511.262 / 620.625 | 536.674 / 613.555 | 14 | 559104 | 4.97 |
| pivoted_qr_256x64_f64/nalgebra | 781.969 / 875.930 | 797.574 / 851.342 | 7 | 296960 | 4.63 |
| pivoted_qr_96x32_f64/faer | 71.206 / 100.180 | 76.219 / 80.203 | 10 | 68352 | 5.04 |
| pivoted_qr_96x32_f64/kernix | 56.637 / 63.551 | 55.175 / 66.674 | 14 | 107520 | 4.44 |
| pivoted_qr_96x32_f64/nalgebra | 78.306 / 93.541 | 81.547 / 103.406 | 7 | 58368 | 4.29 |
| qr_256x64_f64/faer | 398.048 / 433.225 | 433.546 / 488.396 | 7 | 331776 | 5.19 |
| qr_256x64_f64/kernix | 513.125 / 588.185 | 510.624 / 601.060 | 6 | 426496 | 4.94 |
| qr_256x64_f64/nalgebra | 365.251 / 390.806 | 368.000 / 383.677 | 4 | 295424 | 4.51 |
| qr_256x64_f64/numpy | 340.891 / 372.451 | 343.109 / 353.363 | unavailable | unavailable | 37.55 |
| qr_96x32_f64/faer | 69.820 / 77.063 | 71.718 / 95.368 | 7 | 67584 | 5.03 |
| qr_96x32_f64/kernix | 49.547 / 54.341 | 48.734 / 50.590 | 6 | 82176 | 4.46 |
| qr_96x32_f64/nalgebra | 36.100 / 38.569 | 37.318 / 37.928 | 4 | 57600 | 4.11 |
| qr_96x32_f64/numpy | 38.052 / 43.856 | 39.252 / 40.290 | unavailable | unavailable | 36.71 |
| softmax_256x64_f64/candle | 80.473 / 130.369 | 78.092 / 93.843 | 37 | 265200 | 5.93 |
| softmax_256x64_f64/kernix | 83.691 / 116.297 | 85.256 / 97.132 | 1 | 131072 | 4.52 |
| softmax_256x64_f64/ndarray | 71.240 / 78.014 | 73.463 / 75.798 | 1 | 131072 | 4.47 |
| softmax_256x64_f64/numpy | 99.419 / 145.188 | 100.274 / 119.479 | unavailable | unavailable | 35.70 |
| training_backward_2x4x8x4_f64/candle | 13.387 / 14.535 | 14.002 / 14.799 | 472 | 8332 | 6.21 |
| training_backward_2x4x8x4_f64/kernix | 3.213 / 3.562 | 3.182 / 3.477 | 23 | 4224 | 4.56 |
| training_forward_2x4x8x4_f64/candle | 3.533 / 3.701 | 3.465 / 4.087 | 126 | 3912 | 5.71 |
| training_forward_2x4x8x4_f64/kernix | 2.092 / 3.242 | 2.114 / 2.432 | 17 | 1032 | 4.64 |
| training_record_2x4x8x4_f64/candle | 5.141 / 5.368 | 5.198 / 5.334 | 149 | 10712 | 5.77 |
| training_record_2x4x8x4_f64/kernix | 2.656 / 2.719 | 2.688 / 2.800 | 22 | 6832 | 4.52 |
| training_sgd_2x4x8x4_f64/kernix | 0.552 / 0.575 | 0.586 / 0.595 | 5 | 688 | 4.69 |

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
- A shared laptop has uncontrolled load/frequency/cache effects. Report comparator wins and losses; no universal ranking follows. The source archive comes from committed bytes, with result directories excluded. Library dependencies are pinned by the saved driver lockfile.
