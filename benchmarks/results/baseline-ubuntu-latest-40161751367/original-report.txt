# Clean CPU evidence: baseline-ubuntu-latest-40161751367

Measured source `450df128c1ffb55a9a861230edb6b9a5ac3ef5b4`, tree `957542f0c3a8c779782e0b662134bab34e1572a9`; dirty before/after: false.
Library source: registry 0.11.0. CPU affinity: 0. Dtype explicitly named per workload; row-major inputs.

| Workload / library | Per-call p50 / p95 (us) | Batch-mean p50 / p95 (us) | Rust allocations | Peak additional requested bytes | Process peak RSS (MiB) |
|---|---:|---:|---:|---:|---:|
| cholesky_factor_128_f64/faer | 58.660 / 68.478 | 59.525 / 60.034 | 2 | 132096 | 4.43 |
| cholesky_factor_128_f64/kernix | 169.076 / 179.846 | 172.603 / 173.579 | 1 | 131072 | 4.12 |
| cholesky_factor_128_f64/nalgebra | 124.703 / 134.020 | 128.181 / 130.011 | 1 | 131072 | 3.65 |
| cholesky_factor_128_f64/numpy | 112.010 / 126.366 | 113.558 / 117.428 | unavailable | unavailable | 45.88 |
| cholesky_factor_256_f64/faer | 315.811 / 339.395 | 320.557 / 325.828 | 2 | 526336 | 5.55 |
| cholesky_factor_256_f64/kernix | 689.701 / 704.719 | 685.275 / 692.596 | 49 | 645120 | 5.41 |
| cholesky_factor_256_f64/nalgebra | 788.375 / 811.960 | 825.168 / 871.205 | 1 | 524288 | 4.24 |
| cholesky_factor_256_f64/numpy | 1120.085 / 1141.936 | 1116.632 / 1483.101 | unavailable | unavailable | 49.43 |
| cholesky_factor_512_f64/faer | 2091.724 / 2136.867 | 2095.048 / 2191.390 | 2 | 2101248 | 8.63 |
| cholesky_factor_512_f64/kernix | 3916.288 / 4000.835 | 3912.707 / 3954.408 | 161 | 2269184 | 10.12 |
| cholesky_factor_512_f64/nalgebra | 7216.740 / 7522.031 | 7245.750 / 7359.648 | 1 | 2097152 | 7.52 |
| cholesky_factor_512_f64/numpy | 6371.631 / 6471.849 | 6375.453 / 6411.245 | unavailable | unavailable | 60.00 |
| cholesky_factor_64_f64/faer | 14.307 / 14.397 | 14.450 / 14.590 | 2 | 33280 | 4.01 |
| cholesky_factor_64_f64/kernix | 25.237 / 25.538 | 25.668 / 26.226 | 2 | 33280 | 3.98 |
| cholesky_factor_64_f64/nalgebra | 21.009 / 21.570 | 21.674 / 22.002 | 1 | 32768 | 3.58 |
| cholesky_factor_64_f64/numpy | 18.294 / 19.196 | 18.584 / 18.902 | unavailable | unavailable | 45.15 |
| create_ones_64x64_f32/candle | 0.451 / 0.451 | 0.459 / 0.464 | 7 | 16696 | 4.46 |
| create_ones_64x64_f32/faer | 0.391 / 0.391 | 0.381 / 0.384 | 1 | 16384 | 4.00 |
| create_ones_64x64_f32/kernix | 0.430 / 0.431 | 0.438 / 0.441 | 1 | 16384 | 3.62 |
| create_ones_64x64_f32/ndarray | 0.351 / 0.351 | 0.358 / 0.359 | 1 | 16384 | 3.64 |
| create_ones_64x64_f32/numpy | 2.084 / 2.154 | 2.184 / 2.226 | unavailable | unavailable | 41.59 |
| create_zeros_64x64_f32/candle | 0.261 / 0.271 | 0.277 / 0.282 | 5 | 16648 | 4.24 |
| create_zeros_64x64_f32/faer | 0.330 / 0.331 | 0.292 / 0.294 | 1 | 16384 | 3.81 |
| create_zeros_64x64_f32/kernix | 0.290 / 0.301 | 0.297 / 0.299 | 1 | 16384 | 3.84 |
| create_zeros_64x64_f32/ndarray | 0.201 / 0.211 | 0.211 / 0.213 | 1 | 16384 | 3.58 |
| create_zeros_64x64_f32/numpy | 0.731 / 0.771 | 0.814 / 0.833 | unavailable | unavailable | 41.58 |
| matmul_128_f32/candle | 46.006 / 54.792 | 46.271 / 46.862 | 9 | 65816 | 5.15 |
| matmul_128_f32/faer | 50.966 / 59.962 | 51.321 / 51.929 | 1 | 65536 | 4.25 |
| matmul_128_f32/kernix | 45.304 / 53.941 | 45.880 / 67.271 | 3 | 65544 | 4.67 |
| matmul_128_f32/nalgebra | 74.820 / 89.378 | 77.977 / 78.502 | 2 | 163840 | 3.90 |
| matmul_128_f32/ndarray | 73.788 / 87.844 | 76.635 / 77.846 | 2 | 163840 | 3.93 |
| matmul_128_f32/numpy | 49.061 / 59.391 | 49.771 / 50.315 | unavailable | unavailable | 44.14 |
| matmul_128_f64/candle | 171.971 / 213.339 | 93.890 / 94.249 | 9 | 131352 | 5.43 |
| matmul_128_f64/faer | 104.065 / 122.569 | 113.221 / 114.251 | 1 | 131072 | 4.29 |
| matmul_128_f64/kernix | 92.142 / 102.652 | 93.382 / 100.408 | 3 | 131080 | 4.89 |
| matmul_128_f64/nalgebra | 209.802 / 224.991 | 210.267 / 212.835 | 2 | 327680 | 4.02 |
| matmul_128_f64/ndarray | 210.875 / 234.889 | 212.064 / 212.437 | 2 | 327680 | 4.06 |
| matmul_128_f64/numpy | 100.848 / 111.969 | 102.508 / 103.044 | unavailable | unavailable | 44.20 |
| matmul_256_f32/candle | 356.147 / 376.525 | 360.597 / 385.935 | 9 | 262424 | 5.87 |
| matmul_256_f32/faer | 418.954 / 442.338 | 422.401 / 425.464 | 1 | 262144 | 4.72 |
| matmul_256_f32/kernix | 353.331 / 368.229 | 357.672 / 363.214 | 3 | 262152 | 5.26 |
| matmul_256_f32/nalgebra | 612.756 / 628.225 | 609.042 / 612.029 | 2 | 589824 | 4.51 |
| matmul_256_f32/ndarray | 608.288 / 625.471 | 608.105 / 642.697 | 2 | 589824 | 4.62 |
| matmul_256_f32/numpy | 362.648 / 378.859 | 366.008 / 371.399 | unavailable | unavailable | 47.70 |
| matmul_256_f64/candle | 735.035 / 756.065 | 734.289 / 814.475 | 9 | 524568 | 6.66 |
| matmul_256_f64/faer | 964.544 / 990.453 | 968.799 / 979.267 | 1 | 524288 | 5.50 |
| matmul_256_f64/kernix | 734.775 / 754.291 | 734.940 / 739.771 | 3 | 524296 | 5.95 |
| matmul_256_f64/nalgebra | 900.345 / 912.848 | 902.601 / 925.829 | 2 | 1179648 | 5.39 |
| matmul_256_f64/ndarray | 910.543 / 922.526 | 911.322 / 915.924 | 2 | 1179648 | 5.71 |
| matmul_256_f64/numpy | 764.309 / 803.543 | 764.136 / 788.467 | unavailable | unavailable | 48.52 |
| matmul_32_f64/candle | 2.044 / 2.074 | 2.081 / 2.100 | 9 | 8472 | 4.90 |
| matmul_32_f64/faer | 2.133 / 2.144 | 2.128 / 2.159 | 1 | 8192 | 4.12 |
| matmul_32_f64/kernix | 2.444 / 2.485 | 2.474 / 2.483 | 3 | 24576 | 4.03 |
| matmul_32_f64/nalgebra | 2.124 / 2.144 | 2.152 / 2.162 | 2 | 24576 | 3.62 |
| matmul_32_f64/ndarray | 2.204 / 2.295 | 2.218 / 2.261 | 2 | 24576 | 3.77 |
| matmul_32_f64/numpy | 3.637 / 3.707 | 3.783 / 3.927 | unavailable | unavailable | 42.90 |
| matmul_64_f32/candle | 6.473 / 6.573 | 6.565 / 6.672 | 9 | 16664 | 4.81 |
| matmul_64_f32/faer | 6.572 / 6.663 | 6.642 / 6.947 | 1 | 16384 | 3.96 |
| matmul_64_f32/kernix | 6.282 / 6.372 | 6.396 / 6.457 | 3 | 16448 | 4.44 |
| matmul_64_f32/nalgebra | 7.514 / 7.594 | 7.619 / 7.659 | 2 | 49152 | 3.81 |
| matmul_64_f32/ndarray | 9.297 / 9.708 | 7.475 / 7.909 | 2 | 49152 | 3.75 |
| matmul_64_f32/numpy | 8.115 / 8.245 | 8.295 / 8.397 | unavailable | unavailable | 43.12 |
| matmul_64_f64/candle | 12.594 / 12.704 | 12.763 / 12.884 | 9 | 33048 | 4.91 |
| matmul_64_f64/faer | 13.695 / 13.826 | 13.858 / 14.001 | 1 | 32768 | 4.17 |
| matmul_64_f64/kernix | 12.553 / 12.653 | 12.694 / 12.925 | 3 | 32832 | 4.56 |
| matmul_64_f64/nalgebra | 15.169 / 15.329 | 15.376 / 15.414 | 2 | 98304 | 3.81 |
| matmul_64_f64/ndarray | 15.198 / 15.329 | 15.342 / 19.807 | 2 | 98304 | 3.53 |
| matmul_64_f64/numpy | 15.028 / 15.709 | 15.351 / 15.478 | unavailable | unavailable | 43.09 |
| matmul_ab_128_f32/kernix | 45.486 / 48.030 | 46.096 / 59.580 | 3 | 65544 | 4.70 |
| matmul_ab_128_f32/numpy | 49.212 / 59.751 | 49.992 / 50.276 | unavailable | unavailable | 43.89 |
| matmul_ab_128_f64/kernix | 93.455 / 104.395 | 94.912 / 95.481 | 3 | 131080 | 4.89 |
| matmul_ab_128_f64/numpy | 101.600 / 115.266 | 103.475 / 103.871 | unavailable | unavailable | 44.20 |
| matmul_batched_4x16x32x16_f32/candle | 2.023 / 2.064 | 2.032 / 2.064 | 25 | 13096 | 5.10 |
| matmul_batched_4x16x32x16_f32/kernix | 2.064 / 2.094 | 2.097 / 2.117 | 7 | 8200 | 3.97 |
| matmul_batched_4x16x32x16_f32/numpy | 3.357 / 3.447 | 3.470 / 3.591 | unavailable | unavailable | 42.88 |
| matmul_batched_4x16x32x16_f64/candle | 3.035 / 3.096 | 3.075 / 3.104 | 25 | 25384 | 4.41 |
| matmul_batched_4x16x32x16_f64/kernix | 3.046 / 3.076 | 3.091 / 3.102 | 7 | 16392 | 4.01 |
| matmul_batched_4x16x32x16_f64/numpy | 4.148 / 4.218 | 4.311 / 4.362 | unavailable | unavailable | 42.91 |
| matmul_sum_backward_32x16x8_f64/candle | 5.431 / 5.590 | 5.491 / 6.985 | 113 | 27988 | 5.38 |
| matmul_sum_backward_32x16x8_f64/kernix | 3.676 / 3.787 | 3.722 / 3.764 | 16 | 18376 | 4.02 |
| min_norm_32x64_rank32_f64/kernix | 84.017 / 93.185 | 85.017 / 85.834 | 20 | 59136 | 4.36 |
| min_norm_32x64_rank32_f64/nalgebra | 176.290 / 186.098 | 178.973 / 180.068 | 15 | 42232 | 3.95 |
| min_norm_32x64_rank32_f64/numpy | 185.357 / 206.367 | 189.149 / 190.837 | unavailable | unavailable | 47.27 |
| min_norm_64x32_rank16_f64/kernix | 62.527 / 73.418 | 63.101 / 73.430 | 20 | 74752 | 4.29 |
| min_norm_64x32_rank16_f64/nalgebra | 102.462 / 111.158 | 103.803 / 104.426 | 15 | 41976 | 4.06 |
| min_norm_64x32_rank16_f64/numpy | 102.572 / 123.060 | 105.291 / 107.546 | unavailable | unavailable | 46.20 |
| parallel_matmul_into_128_f32/kernix | 31.189 / 32.591 | 31.221 / 31.509 | 2 | 16 | 4.71 |
| parallel_matmul_into_128_f64/kernix | 56.005 / 66.274 | 56.682 / 57.915 | 2 | 16 | 5.02 |
| parallel_matmul_into_256_f64/kernix | 391.522 / 408.364 | 395.857 / 400.454 | 2 | 16 | 6.74 |
| parallel_matmul_into_64_f64/kernix | 16.130 / 16.391 | 16.356 / 16.869 | 2 | 98304 | 4.37 |
| pivoted_qr_256x64_f64/faer | 387.886 / 446.656 | 384.102 / 398.854 | 10 | 333312 | 4.89 |
| pivoted_qr_256x64_f64/kernix | 799.817 / 862.514 | 799.664 / 2010.830 | 14 | 559104 | 4.69 |
| pivoted_qr_256x64_f64/nalgebra | 765.482 / 780.581 | 768.030 / 887.150 | 7 | 296960 | 4.38 |
| pivoted_qr_96x32_f64/faer | 42.901 / 52.188 | 43.427 / 44.737 | 10 | 68352 | 4.49 |
| pivoted_qr_96x32_f64/kernix | 78.647 / 87.664 | 79.378 / 80.175 | 14 | 107520 | 4.09 |
| pivoted_qr_96x32_f64/nalgebra | 77.114 / 86.302 | 78.171 / 166.126 | 7 | 58368 | 3.80 |
| qr_256x64_f64/faer | 426.127 / 455.793 | 429.423 / 438.477 | 7 | 331776 | 4.91 |
| qr_256x64_f64/kernix | 756.375 / 778.637 | 758.305 / 762.369 | 6 | 426496 | 4.50 |
| qr_256x64_f64/nalgebra | 571.269 / 590.996 | 571.185 / 575.532 | 4 | 295424 | 4.16 |
| qr_256x64_f64/numpy | 543.668 / 563.274 | 538.975 / 543.643 | unavailable | unavailable | 48.02 |
| qr_96x32_f64/faer | 55.404 / 65.332 | 56.842 / 114.256 | 7 | 67584 | 4.58 |
| qr_96x32_f64/kernix | 67.856 / 76.463 | 68.625 / 70.443 | 6 | 82176 | 3.99 |
| qr_96x32_f64/nalgebra | 59.661 / 68.017 | 60.488 / 60.938 | 4 | 57600 | 3.53 |
| qr_96x32_f64/numpy | 69.831 / 90.790 | 71.207 / 72.280 | unavailable | unavailable | 47.27 |
| softmax_256x64_f64/candle | 84.929 / 95.188 | 87.134 / 113.248 | 37 | 265200 | 5.62 |
| softmax_256x64_f64/kernix | 93.435 / 102.772 | 94.400 / 106.528 | 1 | 131072 | 4.34 |
| softmax_256x64_f64/ndarray | 83.135 / 92.163 | 84.284 / 85.645 | 1 | 131072 | 4.16 |
| softmax_256x64_f64/numpy | 120.315 / 133.019 | 122.475 / 124.506 | unavailable | unavailable | 42.15 |
| tensordot_16x32x8_32x8x16_f64/kernix | 63.338 / 71.564 | 63.973 / 64.371 | 9 | 2166 | 3.95 |
| tensordot_16x32x8_32x8x16_f64/numpy | 10.970 / 11.682 | 11.227 / 11.318 | unavailable | unavailable | 42.86 |
| training_backward_2x4x8x4_f64/candle | 17.042 / 17.563 | 17.358 / 17.500 | 472 | 8332 | 5.75 |
| training_backward_2x4x8x4_f64/kernix | 4.729 / 4.809 | 4.788 / 4.840 | 23 | 4224 | 4.24 |
| training_forward_2x4x8x4_f64/candle | 4.328 / 4.468 | 4.383 / 4.431 | 126 | 3912 | 5.57 |
| training_forward_2x4x8x4_f64/kernix | 2.936 / 2.996 | 2.999 / 3.403 | 17 | 1032 | 4.46 |
| training_record_2x4x8x4_f64/candle | 6.312 / 6.512 | 6.397 / 6.417 | 149 | 10712 | 5.57 |
| training_record_2x4x8x4_f64/kernix | 3.797 / 3.858 | 3.836 / 3.870 | 22 | 6832 | 4.36 |
| training_sgd_2x4x8x4_f64/kernix | 0.792 / 0.812 | 0.810 / 0.824 | 5 | 688 | 4.29 |
| training_shared_backward_64x32_f64/candle | 4.629 / 4.739 | 4.680 / 4.811 | 58 | 100004 | 5.13 |
| training_shared_backward_64x32_f64/kernix | 10.299 / 10.340 | 10.454 / 10.669 | 8 | 49768 | 3.86 |

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
