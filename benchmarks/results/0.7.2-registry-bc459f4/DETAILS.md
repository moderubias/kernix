# Clean CPU evidence: 0.7.2-registry-bc459f4

Measured source `bc459f44b276e86275cc4e431e4946443aefb15f`, tree `8f1c1dad8523b125c5d444ddddcb3b84f34180f1`; dirty before/after: false.
Library source: registry 0.7.2. One pinned CPU; dtype explicitly named per workload; row-major inputs.

| Workload / library | Per-call p50 / p95 (us) | Batch-mean p50 / p95 (us) | Rust allocations | Peak additional requested bytes | Process peak RSS (MiB) |
|---|---:|---:|---:|---:|---:|
| cholesky_factor_64_f64/faer | 8.176 / 10.647 | 8.522 / 9.104 | 2 | 33280 | 4.25 |
| cholesky_factor_64_f64/kernix | 22.798 / 26.039 | 24.272 / 27.226 | 2 | 65536 | 4.09 |
| cholesky_factor_64_f64/nalgebra | 13.105 / 14.152 | 14.331 / 15.444 | 1 | 32768 | 4.12 |
| cholesky_factor_64_f64/numpy | 10.755 / 11.350 | 10.916 / 11.626 | unavailable | unavailable | 33.19 |
| create_ones_64x64_f32/candle | 0.237 / 0.250 | 0.236 / 0.249 | 7 | 16696 | 4.02 |
| create_ones_64x64_f32/faer | 0.202 / 0.218 | 0.180 / 0.189 | 1 | 16384 | 4.05 |
| create_ones_64x64_f32/kernix | 0.201 / 0.204 | 0.200 / 0.207 | 1 | 16384 | 4.02 |
| create_ones_64x64_f32/ndarray | 0.154 / 0.156 | 0.150 / 0.160 | 1 | 16384 | 4.01 |
| create_ones_64x64_f32/numpy | 1.345 / 1.471 | 1.393 / 1.483 | unavailable | unavailable | 32.80 |
| create_zeros_64x64_f32/candle | 0.169 / 0.187 | 0.168 / 0.187 | 5 | 16648 | 4.55 |
| create_zeros_64x64_f32/faer | 0.143 / 0.152 | 0.191 / 0.195 | 1 | 16384 | 4.25 |
| create_zeros_64x64_f32/kernix | 0.135 / 0.137 | 0.141 / 0.150 | 1 | 16384 | 4.00 |
| create_zeros_64x64_f32/ndarray | 0.090 / 0.091 | 0.096 / 0.102 | 1 | 16384 | 4.03 |
| create_zeros_64x64_f32/numpy | 0.396 / 0.579 | 0.413 / 0.425 | unavailable | unavailable | 32.80 |
| matmul_128_f32/candle | 34.791 / 37.557 | 37.191 / 38.405 | 9 | 65816 | 5.27 |
| matmul_128_f32/faer | 41.712 / 51.437 | 43.030 / 44.915 | 1 | 65536 | 4.62 |
| matmul_128_f32/kernix | 49.181 / 58.117 | 51.086 / 53.128 | 3 | 163840 | 4.55 |
| matmul_128_f32/nalgebra | 53.199 / 62.834 | 50.121 / 51.727 | 2 | 163840 | 4.11 |
| matmul_128_f32/ndarray | 51.443 / 56.024 | 50.652 / 61.056 | 2 | 163840 | 4.40 |
| matmul_128_f32/numpy | 39.220 / 47.080 | 39.730 / 42.039 | unavailable | unavailable | 34.08 |
| matmul_128_f64/candle | 73.442 / 81.443 | 76.868 / 84.825 | 9 | 131352 | 5.53 |
| matmul_128_f64/faer | 86.168 / 91.407 | 86.447 / 88.606 | 1 | 131072 | 4.54 |
| matmul_128_f64/kernix | 73.035 / 78.616 | 77.268 / 80.300 | 3 | 131080 | 5.09 |
| matmul_128_f64/nalgebra | 118.439 / 126.247 | 120.279 / 124.540 | 2 | 327680 | 4.43 |
| matmul_128_f64/ndarray | 119.839 / 140.183 | 120.581 / 142.366 | 2 | 327680 | 3.96 |
| matmul_128_f64/numpy | 79.676 / 93.339 | 81.960 / 84.332 | unavailable | unavailable | 34.23 |
| matmul_256_f32/candle | 278.711 / 310.369 | 285.533 / 298.897 | 9 | 262424 | 5.96 |
| matmul_256_f32/faer | 312.433 / 322.659 | 317.879 / 323.317 | 1 | 262144 | 5.10 |
| matmul_256_f32/kernix | 393.444 / 438.747 | 405.056 / 636.399 | 3 | 589824 | 5.41 |
| matmul_256_f32/nalgebra | 398.043 / 447.180 | 398.651 / 408.852 | 2 | 589824 | 4.63 |
| matmul_256_f32/ndarray | 391.705 / 435.273 | 395.338 / 410.939 | 2 | 589824 | 5.14 |
| matmul_256_f32/numpy | 286.706 / 300.815 | 295.073 / 410.927 | unavailable | unavailable | 38.22 |
| matmul_256_f64/candle | 579.849 / 621.375 | 589.677 / 602.346 | 9 | 524568 | 6.90 |
| matmul_256_f64/faer | 742.204 / 824.670 | 751.808 / 804.566 | 1 | 524288 | 5.82 |
| matmul_256_f64/kernix | 572.709 / 603.191 | 584.403 / 681.384 | 3 | 524296 | 6.09 |
| matmul_256_f64/nalgebra | 682.415 / 772.165 | 679.543 / 795.243 | 2 | 1179648 | 5.69 |
| matmul_256_f64/ndarray | 702.672 / 944.662 | 683.473 / 701.408 | 2 | 1179648 | 6.22 |
| matmul_256_f64/numpy | 608.764 / 642.114 | 616.483 / 639.827 | unavailable | unavailable | 38.11 |
| matmul_32_f64/candle | 1.500 / 1.771 | 1.570 / 1.666 | 9 | 8592 | 4.89 |
| matmul_32_f64/faer | 1.875 / 2.584 | 1.584 / 1.656 | 1 | 8192 | 4.44 |
| matmul_32_f64/kernix | 1.589 / 1.847 | 1.746 / 1.889 | 3 | 24576 | 4.09 |
| matmul_32_f64/nalgebra | 1.498 / 1.537 | 1.542 / 1.616 | 2 | 24576 | 4.00 |
| matmul_32_f64/ndarray | 1.412 / 1.667 | 1.535 / 1.661 | 2 | 24576 | 4.05 |
| matmul_32_f64/numpy | 2.822 / 4.820 | 2.755 / 2.927 | unavailable | unavailable | 33.06 |
| matmul_64_f32/candle | 4.779 / 5.071 | 4.903 / 5.047 | 9 | 16664 | 5.13 |
| matmul_64_f32/faer | 5.350 / 7.619 | 5.277 / 5.418 | 1 | 16384 | 4.43 |
| matmul_64_f32/kernix | 5.884 / 6.345 | 6.013 / 7.080 | 3 | 49152 | 4.40 |
| matmul_64_f32/nalgebra | 6.161 / 6.235 | 5.945 / 6.332 | 2 | 49152 | 4.09 |
| matmul_64_f32/ndarray | 5.847 / 6.039 | 5.881 / 6.325 | 2 | 49152 | 4.13 |
| matmul_64_f32/numpy | 6.338 / 6.436 | 6.227 / 6.567 | unavailable | unavailable | 33.28 |
| matmul_64_f64/candle | 9.862 / 16.213 | 10.787 / 12.110 | 9 | 33048 | 5.12 |
| matmul_64_f64/faer | 10.916 / 11.386 | 11.812 / 12.301 | 1 | 32768 | 4.38 |
| matmul_64_f64/kernix | 9.138 / 10.188 | 10.350 / 10.762 | 3 | 32928 | 4.18 |
| matmul_64_f64/nalgebra | 11.299 / 11.739 | 11.461 / 11.676 | 2 | 98304 | 4.18 |
| matmul_64_f64/ndarray | 11.253 / 11.400 | 11.402 / 15.019 | 2 | 98304 | 4.21 |
| matmul_64_f64/numpy | 13.142 / 14.270 | 12.737 / 13.709 | unavailable | unavailable | 33.35 |
| matmul_batched_4x16x32x16_f64/candle | 2.078 / 2.373 | 2.308 / 2.395 | 25 | 25504 | 5.66 |
| matmul_batched_4x16x32x16_f64/kernix | 1.936 / 2.414 | 2.173 / 2.388 | 7 | 16392 | 4.40 |
| matmul_batched_4x16x32x16_f64/numpy | 3.332 / 3.425 | 3.279 / 3.399 | unavailable | unavailable | 33.04 |
| matmul_sum_backward_32x16x8_f64/candle | 4.231 / 6.465 | 4.191 / 4.777 | 113 | 27988 | 5.57 |
| matmul_sum_backward_32x16x8_f64/kernix | 2.423 / 2.878 | 2.559 / 2.826 | 16 | 18376 | 4.45 |
| qr_256x64_f64/faer | 407.851 / 436.131 | 415.909 / 488.996 | 7 | 331776 | 5.16 |
| qr_256x64_f64/kernix | 496.680 / 553.499 | 501.144 / 522.536 | 6 | 557568 | 4.84 |
| qr_256x64_f64/nalgebra | 371.777 / 409.641 | 370.722 / 384.748 | 4 | 295424 | 4.37 |
| qr_256x64_f64/numpy | 348.219 / 483.559 | 344.258 / 349.315 | unavailable | unavailable | 34.12 |
| qr_96x32_f64/faer | 64.469 / 72.317 | 66.509 / 73.221 | 7 | 67584 | 4.39 |
| qr_96x32_f64/kernix | 42.746 / 47.850 | 45.930 / 48.415 | 6 | 106752 | 4.36 |
| qr_96x32_f64/nalgebra | 37.668 / 53.666 | 37.770 / 38.522 | 4 | 57600 | 4.00 |
| qr_96x32_f64/numpy | 38.141 / 40.672 | 40.118 / 51.041 | unavailable | unavailable | 33.30 |
| softmax_256x64_f64/candle | 80.122 / 97.638 | 81.793 / 82.779 | 37 | 265200 | 5.79 |
| softmax_256x64_f64/kernix | 85.344 / 100.913 | 88.177 / 94.354 | 1 | 131072 | 4.03 |
| softmax_256x64_f64/ndarray | 71.614 / 80.433 | 74.009 / 101.382 | 1 | 131072 | 3.79 |
| softmax_256x64_f64/numpy | 99.838 / 109.743 | 104.199 / 108.758 | unavailable | unavailable | 33.20 |
| training_backward_2x4x8x4_f64/candle | 14.451 / 17.490 | 14.665 / 15.364 | 472 | 8332 | 6.16 |
| training_backward_2x4x8x4_f64/kernix | 3.194 / 3.279 | 3.411 / 4.010 | 23 | 4224 | 4.48 |
| training_forward_2x4x8x4_f64/candle | 3.486 / 4.221 | 3.647 / 3.743 | 126 | 3912 | 5.39 |
| training_forward_2x4x8x4_f64/kernix | 2.001 / 2.348 | 2.300 / 2.386 | 17 | 1032 | 4.48 |
| training_record_2x4x8x4_f64/candle | 5.198 / 5.786 | 5.362 / 5.532 | 149 | 10712 | 6.04 |
| training_record_2x4x8x4_f64/kernix | 2.776 / 2.872 | 2.843 / 2.874 | 22 | 6832 | 4.43 |
| training_sgd_2x4x8x4_f64/kernix | 0.514 / 0.549 | 0.549 / 0.604 | 5 | 688 | 4.38 |

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
