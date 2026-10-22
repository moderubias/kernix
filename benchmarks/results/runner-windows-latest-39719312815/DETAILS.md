# Clean CPU evidence: runner-windows-latest-39719312815

Measured source `227f339974f9c31fcf6bd37ccccbda3eedc99591`, tree `556afc7a18c74320ec378c75bbefd4422f3d3ef3`; dirty before/after: false.
Library source: workspace 227f339974f9c31fcf6bd37ccccbda3eedc99591. One pinned CPU; dtype explicitly named per workload; row-major inputs.

| Workload / library | Per-call p50 / p95 (us) | Batch-mean p50 / p95 (us) | Rust allocations | Peak additional requested bytes | Process peak RSS (MiB) |
|---|---:|---:|---:|---:|---:|
| cholesky_factor_128_f64/faer | 59.200 / 62.200 | 60.588 / 65.983 | 2 | 132096 | unavailable |
| cholesky_factor_128_f64/kernix | 221.700 / 355.200 | 229.839 / 297.938 | 1 | 131072 | unavailable |
| cholesky_factor_128_f64/nalgebra | 112.500 / 138.800 | 119.008 / 120.057 | 1 | 131072 | unavailable |
| cholesky_factor_128_f64/numpy | 113.700 / 136.600 | 116.241 / 123.490 | unavailable | unavailable | unavailable |
| cholesky_factor_256_f64/faer | 313.500 / 349.800 | 323.281 / 491.419 | 2 | 526336 | unavailable |
| cholesky_factor_256_f64/kernix | 1012.900 / 1507.900 | 1011.340 / 1058.400 | 1 | 524288 | unavailable |
| cholesky_factor_256_f64/nalgebra | 859.800 / 956.900 | 887.292 / 1269.911 | 1 | 524288 | unavailable |
| cholesky_factor_256_f64/numpy | 948.800 / 1121.900 | 940.364 / 1103.680 | unavailable | unavailable | unavailable |
| cholesky_factor_64_f64/faer | 14.500 / 14.600 | 14.668 / 15.124 | 2 | 33280 | unavailable |
| cholesky_factor_64_f64/kernix | 35.800 / 70.800 | 36.666 / 70.837 | 2 | 33280 | unavailable |
| cholesky_factor_64_f64/nalgebra | 20.100 / 20.900 | 20.854 / 20.971 | 1 | 32768 | unavailable |
| cholesky_factor_64_f64/numpy | 21.200 / 38.500 | 22.663 / 34.572 | unavailable | unavailable | unavailable |
| create_ones_64x64_f32/candle | 1.300 / 1.400 | 0.734 / 1.363 | 7 | 16696 | unavailable |
| create_ones_64x64_f32/faer | 0.500 / 0.500 | 0.452 / 0.456 | 1 | 16384 | unavailable |
| create_ones_64x64_f32/kernix | 0.500 / 0.500 | 0.491 / 0.524 | 1 | 16384 | unavailable |
| create_ones_64x64_f32/ndarray | 0.400 / 0.400 | 0.388 / 0.396 | 1 | 16384 | unavailable |
| create_ones_64x64_f32/numpy | 2.100 / 2.200 | 2.255 / 2.343 | unavailable | unavailable | unavailable |
| create_zeros_64x64_f32/candle | 0.500 / 0.700 | 0.488 / 0.493 | 5 | 16648 | unavailable |
| create_zeros_64x64_f32/faer | 0.500 / 0.500 | 0.485 / 0.646 | 1 | 16384 | unavailable |
| create_zeros_64x64_f32/kernix | 0.400 / 0.400 | 0.365 / 0.367 | 1 | 16384 | unavailable |
| create_zeros_64x64_f32/ndarray | 0.300 / 0.300 | 0.283 / 0.305 | 1 | 16384 | unavailable |
| create_zeros_64x64_f32/numpy | 0.800 / 0.900 | 0.948 / 0.975 | unavailable | unavailable | unavailable |
| matmul_128_f32/candle | 72.400 / 119.400 | 65.552 / 85.222 | 10 | 65816 | unavailable |
| matmul_128_f32/faer | 50.200 / 52.400 | 51.377 / 51.945 | 1 | 65536 | unavailable |
| matmul_128_f32/kernix | 76.000 / 125.000 | 73.582 / 133.532 | 3 | 65544 | unavailable |
| matmul_128_f32/nalgebra | 53.700 / 54.500 | 54.689 / 91.810 | 2 | 163840 | unavailable |
| matmul_128_f32/ndarray | 115.400 / 152.400 | 117.309 / 126.870 | 2 | 163840 | unavailable |
| matmul_128_f32/numpy | 56.200 / 59.800 | 57.485 / 63.715 | unavailable | unavailable | unavailable |
| matmul_128_f64/candle | 136.900 / 170.700 | 138.451 / 145.632 | 10 | 131352 | unavailable |
| matmul_128_f64/faer | 116.700 / 153.200 | 116.291 / 117.134 | 1 | 131072 | unavailable |
| matmul_128_f64/kernix | 137.700 / 168.900 | 141.755 / 159.803 | 3 | 131080 | unavailable |
| matmul_128_f64/nalgebra | 115.900 / 130.300 | 117.557 / 124.268 | 2 | 327680 | unavailable |
| matmul_128_f64/ndarray | 115.300 / 139.400 | 117.081 / 123.785 | 2 | 327680 | unavailable |
| matmul_128_f64/numpy | 100.800 / 129.900 | 103.255 / 108.985 | unavailable | unavailable | unavailable |
| matmul_256_f32/candle | 432.300 / 469.800 | 436.661 / 471.027 | 10 | 262424 | unavailable |
| matmul_256_f32/faer | 424.900 / 693.200 | 427.658 / 462.859 | 1 | 262144 | unavailable |
| matmul_256_f32/kernix | 440.400 / 482.500 | 446.013 / 491.948 | 3 | 262152 | unavailable |
| matmul_256_f32/nalgebra | 399.700 / 435.400 | 404.628 / 408.576 | 2 | 589824 | unavailable |
| matmul_256_f32/ndarray | 397.500 / 686.500 | 401.616 / 431.746 | 2 | 589824 | unavailable |
| matmul_256_f32/numpy | 391.500 / 509.800 | 397.450 / 432.450 | unavailable | unavailable | unavailable |
| matmul_256_f64/candle | 867.900 / 907.100 | 877.183 / 891.508 | 10 | 524568 | unavailable |
| matmul_256_f64/faer | 1088.100 / 1123.900 | 1096.940 / 1128.544 | 1 | 524288 | unavailable |
| matmul_256_f64/kernix | 843.800 / 883.600 | 840.758 / 874.367 | 3 | 524296 | unavailable |
| matmul_256_f64/nalgebra | 908.200 / 937.000 | 912.145 / 934.682 | 2 | 1179648 | unavailable |
| matmul_256_f64/ndarray | 1164.600 / 1214.500 | 1161.733 / 1191.078 | 2 | 1179648 | unavailable |
| matmul_256_f64/numpy | 757.500 / 797.500 | 771.831 / 840.358 | unavailable | unavailable | unavailable |
| matmul_32_f64/candle | 2.800 / 3.100 | 2.632 / 3.014 | 10 | 8528 | unavailable |
| matmul_32_f64/faer | 2.500 / 2.600 | 2.468 / 2.683 | 1 | 8192 | unavailable |
| matmul_32_f64/kernix | 2.800 / 2.800 | 2.819 / 2.852 | 3 | 24576 | unavailable |
| matmul_32_f64/nalgebra | 2.300 / 2.400 | 2.412 / 4.184 | 2 | 24576 | unavailable |
| matmul_32_f64/ndarray | 2.400 / 2.400 | 2.455 / 2.800 | 2 | 24576 | unavailable |
| matmul_32_f64/numpy | 4.200 / 4.300 | 4.373 / 6.262 | unavailable | unavailable | unavailable |
| matmul_64_f32/candle | 7.500 / 7.600 | 7.284 / 7.970 | 10 | 16664 | unavailable |
| matmul_64_f32/faer | 7.100 / 7.200 | 7.367 / 11.956 | 1 | 16384 | unavailable |
| matmul_64_f32/kernix | 7.000 / 7.100 | 6.878 / 7.042 | 3 | 16512 | unavailable |
| matmul_64_f32/nalgebra | 27.300 / 35.600 | 9.591 / 29.455 | 2 | 49152 | unavailable |
| matmul_64_f32/ndarray | 7.900 / 7.900 | 8.015 / 22.818 | 2 | 49152 | unavailable |
| matmul_64_f32/numpy | 10.400 / 11.200 | 10.883 / 17.798 | unavailable | unavailable | unavailable |
| matmul_64_f64/candle | 27.900 / 29.200 | 14.589 / 27.843 | 10 | 33048 | unavailable |
| matmul_64_f64/faer | 14.500 / 14.700 | 14.887 / 20.426 | 1 | 32768 | unavailable |
| matmul_64_f64/kernix | 13.500 / 13.600 | 13.626 / 23.944 | 3 | 32896 | unavailable |
| matmul_64_f64/nalgebra | 50.800 / 66.900 | 29.713 / 46.186 | 2 | 98304 | unavailable |
| matmul_64_f64/ndarray | 53.700 / 73.100 | 55.314 / 64.310 | 2 | 98304 | unavailable |
| matmul_64_f64/numpy | 15.800 / 16.500 | 16.347 / 16.920 | unavailable | unavailable | unavailable |
| matmul_batched_4x16x32x16_f64/candle | 4.000 / 4.100 | 4.393 / 4.405 | 26 | 25440 | unavailable |
| matmul_batched_4x16x32x16_f64/kernix | 3.400 / 5.600 | 3.779 / 5.182 | 7 | 16392 | unavailable |
| matmul_batched_4x16x32x16_f64/numpy | 5.100 / 5.200 | 5.291 / 6.104 | unavailable | unavailable | unavailable |
| matmul_sum_backward_32x16x8_f64/candle | 10.200 / 10.900 | 10.780 / 18.556 | 116 | 27988 | unavailable |
| matmul_sum_backward_32x16x8_f64/kernix | 5.100 / 5.200 | 4.583 / 8.318 | 16 | 18376 | unavailable |
| qr_256x64_f64/faer | 439.400 / 572.800 | 416.944 / 467.927 | 7 | 331776 | unavailable |
| qr_256x64_f64/kernix | 810.100 / 1212.200 | 805.015 / 913.936 | 6 | 426496 | unavailable |
| qr_256x64_f64/nalgebra | 564.800 / 610.500 | 578.217 / 648.806 | 4 | 295424 | unavailable |
| qr_256x64_f64/numpy | 552.600 / 726.900 | 556.647 / 577.728 | unavailable | unavailable | unavailable |
| qr_96x32_f64/faer | 83.700 / 109.500 | 68.999 / 88.856 | 7 | 67584 | unavailable |
| qr_96x32_f64/kernix | 105.200 / 137.800 | 96.771 / 105.848 | 6 | 82176 | unavailable |
| qr_96x32_f64/nalgebra | 79.600 / 94.700 | 80.411 / 98.600 | 4 | 57600 | unavailable |
| qr_96x32_f64/numpy | 77.000 / 87.600 | 79.243 / 86.390 | unavailable | unavailable | unavailable |
| softmax_256x64_f64/candle | 155.000 / 219.800 | 128.665 / 162.669 | 37 | 265200 | unavailable |
| softmax_256x64_f64/kernix | 112.500 / 142.000 | 117.244 / 140.986 | 1 | 131072 | unavailable |
| softmax_256x64_f64/ndarray | 162.900 / 287.700 | 166.008 / 171.437 | 1 | 131072 | unavailable |
| softmax_256x64_f64/numpy | 134.500 / 154.600 | 136.596 / 141.207 | unavailable | unavailable | unavailable |
| training_backward_2x4x8x4_f64/candle | 31.500 / 32.200 | 31.890 / 38.128 | 474 | 8332 | unavailable |
| training_backward_2x4x8x4_f64/kernix | 6.000 / 6.100 | 6.027 / 6.086 | 23 | 4224 | unavailable |
| training_forward_2x4x8x4_f64/candle | 8.200 / 8.400 | 8.220 / 8.604 | 127 | 3912 | unavailable |
| training_forward_2x4x8x4_f64/kernix | 3.800 / 4.000 | 3.670 / 5.343 | 17 | 1032 | unavailable |
| training_record_2x4x8x4_f64/candle | 10.600 / 27.300 | 10.535 / 12.039 | 150 | 10712 | unavailable |
| training_record_2x4x8x4_f64/kernix | 5.000 / 5.100 | 4.886 / 4.979 | 22 | 6832 | unavailable |
| training_sgd_2x4x8x4_f64/kernix | 1.000 / 1.000 | 1.011 / 1.039 | 5 | 688 | unavailable |

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
