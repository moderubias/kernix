# Clean CPU evidence: 0.11.0-registry-0111

Measured source `b2c458d113aa23aa24e93fbfdabd55b55e3e6831`, tree `eb0af9dc230f1ae319802354a0b518909dbaad56`; dirty before/after: false.
Library source: registry 0.11.0. CPU affinity: 0. Dtype explicitly named per workload; row-major inputs.

| Workload / library | Per-call p50 / p95 (us) | Batch-mean p50 / p95 (us) | Rust allocations | Peak additional requested bytes | Process peak RSS (MiB) |
|---|---:|---:|---:|---:|---:|
| cholesky_factor_128_f64/faer | 39.778 / 40.669 | 40.060 / 40.852 | 2 | 132096 | 4.43 |
| cholesky_factor_128_f64/kernix | 113.100 / 121.412 | 115.358 / 122.889 | 1 | 131072 | 4.25 |
| cholesky_factor_128_f64/nalgebra | 91.868 / 96.773 | 93.538 / 96.201 | 1 | 131072 | 4.05 |
| cholesky_factor_128_f64/numpy | 68.756 / 78.852 | 70.879 / 75.998 | unavailable | unavailable | 36.80 |
| cholesky_factor_256_f64/faer | 210.253 / 218.084 | 207.490 / 211.198 | 2 | 526336 | 5.70 |
| cholesky_factor_256_f64/kernix | 503.917 / 543.889 | 499.273 / 521.128 | 49 | 645120 | 5.90 |
| cholesky_factor_256_f64/nalgebra | 695.933 / 722.296 | 702.355 / 791.649 | 1 | 524288 | 4.96 |
| cholesky_factor_256_f64/numpy | 511.409 / 558.037 | 508.279 / 523.960 | unavailable | unavailable | 40.86 |
| cholesky_factor_512_f64/faer | 1374.314 / 2007.456 | 1364.008 / 1470.496 | 2 | 2101248 | 8.87 |
| cholesky_factor_512_f64/kernix | 2866.731 / 3277.804 | 2849.890 / 3179.760 | 161 | 2269184 | 10.35 |
| cholesky_factor_512_f64/nalgebra | 5381.460 / 6208.388 | 5606.963 / 6238.879 | 1 | 2097152 | 7.56 |
| cholesky_factor_512_f64/numpy | 3400.107 / 5250.233 | 3495.466 / 4499.124 | unavailable | unavailable | 52.66 |
| cholesky_factor_64_f64/faer | 8.436 / 8.848 | 8.403 / 9.355 | 2 | 33280 | 4.29 |
| cholesky_factor_64_f64/kernix | 18.837 / 20.764 | 20.378 / 20.815 | 2 | 33280 | 4.24 |
| cholesky_factor_64_f64/nalgebra | 14.517 / 16.188 | 14.967 / 15.096 | 1 | 32768 | 4.02 |
| cholesky_factor_64_f64/numpy | 10.533 / 11.399 | 11.060 / 12.254 | unavailable | unavailable | 36.09 |
| create_ones_64x64_f32/candle | 0.243 / 0.254 | 0.239 / 0.247 | 7 | 16696 | 4.87 |
| create_ones_64x64_f32/faer | 0.188 / 0.192 | 0.173 / 0.182 | 1 | 16384 | 4.43 |
| create_ones_64x64_f32/kernix | 0.184 / 0.187 | 0.193 / 0.328 | 1 | 16384 | 4.02 |
| create_ones_64x64_f32/ndarray | 0.140 / 0.141 | 0.145 / 0.147 | 1 | 16384 | 3.97 |
| create_ones_64x64_f32/numpy | 1.278 / 1.311 | 1.327 / 1.358 | unavailable | unavailable | 35.14 |
| create_zeros_64x64_f32/candle | 0.152 / 0.153 | 0.158 / 0.161 | 5 | 16648 | 4.47 |
| create_zeros_64x64_f32/faer | 0.150 / 0.160 | 0.182 / 0.187 | 1 | 16384 | 4.43 |
| create_zeros_64x64_f32/kernix | 0.155 / 0.158 | 0.148 / 0.159 | 1 | 16384 | 4.05 |
| create_zeros_64x64_f32/ndarray | 0.090 / 0.091 | 0.094 / 0.101 | 1 | 16384 | 4.02 |
| create_zeros_64x64_f32/numpy | 0.367 / 0.410 | 0.393 / 0.427 | unavailable | unavailable | 35.13 |
| matmul_128_f32/candle | 36.027 / 40.162 | 36.371 / 37.360 | 9 | 65816 | 5.27 |
| matmul_128_f32/faer | 41.242 / 42.528 | 41.009 / 50.289 | 1 | 65536 | 4.34 |
| matmul_128_f32/kernix | 39.073 / 42.472 | 38.456 / 39.465 | 3 | 65544 | 4.86 |
| matmul_128_f32/nalgebra | 51.639 / 55.276 | 49.736 / 53.977 | 2 | 163840 | 3.89 |
| matmul_128_f32/ndarray | 48.244 / 56.665 | 53.261 / 72.220 | 2 | 163840 | 4.37 |
| matmul_128_f32/numpy | 36.543 / 41.123 | 38.672 / 40.166 | unavailable | unavailable | 36.82 |
| matmul_128_f64/candle | 70.865 / 76.039 | 72.659 / 74.355 | 9 | 131352 | 5.48 |
| matmul_128_f64/faer | 78.546 / 83.317 | 79.035 / 79.809 | 1 | 131072 | 4.71 |
| matmul_128_f64/kernix | 73.585 / 81.875 | 74.925 / 75.855 | 3 | 131080 | 5.04 |
| matmul_128_f64/nalgebra | 110.566 / 128.959 | 113.871 / 120.642 | 2 | 327680 | 4.46 |
| matmul_128_f64/ndarray | 117.842 / 123.828 | 115.074 / 122.058 | 2 | 327680 | 4.47 |
| matmul_128_f64/numpy | 82.111 / 87.426 | 84.380 / 86.338 | unavailable | unavailable | 37.00 |
| matmul_256_f32/candle | 278.865 / 301.812 | 282.031 / 299.503 | 9 | 262424 | 5.81 |
| matmul_256_f32/faer | 338.662 / 351.161 | 339.677 / 344.054 | 1 | 262144 | 4.84 |
| matmul_256_f32/kernix | 283.132 / 311.700 | 290.004 / 352.942 | 3 | 262152 | 5.49 |
| matmul_256_f32/nalgebra | 385.924 / 429.145 | 395.698 / 447.522 | 2 | 589824 | 4.82 |
| matmul_256_f32/ndarray | 393.288 / 432.892 | 394.695 / 398.940 | 2 | 589824 | 4.64 |
| matmul_256_f32/numpy | 289.770 / 310.542 | 294.614 / 300.623 | unavailable | unavailable | 41.04 |
| matmul_256_f64/candle | 560.582 / 587.816 | 561.250 / 576.628 | 9 | 524568 | 7.12 |
| matmul_256_f64/faer | 794.798 / 817.069 | 790.432 / 804.995 | 1 | 524288 | 5.93 |
| matmul_256_f64/kernix | 569.153 / 607.195 | 576.050 / 751.218 | 3 | 524296 | 6.37 |
| matmul_256_f64/nalgebra | 640.734 / 689.035 | 645.211 / 671.646 | 2 | 1179648 | 5.74 |
| matmul_256_f64/ndarray | 676.386 / 713.357 | 650.341 / 700.695 | 2 | 1179648 | 6.29 |
| matmul_256_f64/numpy | 665.089 / 685.210 | 664.682 / 679.770 | unavailable | unavailable | 41.84 |
| matmul_32_f64/candle | 1.559 / 1.580 | 1.672 / 1.722 | 9 | 8592 | 4.89 |
| matmul_32_f64/faer | 1.469 / 1.583 | 1.473 / 1.548 | 1 | 8192 | 4.37 |
| matmul_32_f64/kernix | 1.751 / 1.774 | 1.671 / 2.281 | 3 | 24576 | 4.46 |
| matmul_32_f64/nalgebra | 1.431 / 1.447 | 1.513 / 1.685 | 2 | 24576 | 4.10 |
| matmul_32_f64/ndarray | 1.466 / 1.487 | 1.539 / 1.609 | 2 | 24576 | 4.12 |
| matmul_32_f64/numpy | 2.584 / 2.734 | 2.747 / 2.983 | unavailable | unavailable | 35.70 |
| matmul_64_f32/candle | 4.578 / 4.793 | 4.772 / 4.886 | 9 | 16664 | 5.02 |
| matmul_64_f32/faer | 5.031 / 5.251 | 5.079 / 5.174 | 1 | 16384 | 4.53 |
| matmul_64_f32/kernix | 4.815 / 4.977 | 4.699 / 4.883 | 3 | 16544 | 4.32 |
| matmul_64_f32/nalgebra | 5.604 / 6.186 | 5.682 / 5.829 | 2 | 49152 | 4.21 |
| matmul_64_f32/ndarray | 5.839 / 5.894 | 6.011 / 6.170 | 2 | 49152 | 4.21 |
| matmul_64_f32/numpy | 6.599 / 6.723 | 6.667 / 6.812 | unavailable | unavailable | 35.99 |
| matmul_64_f64/candle | 9.195 / 9.519 | 9.776 / 10.145 | 9 | 33048 | 5.32 |
| matmul_64_f64/faer | 10.971 / 11.205 | 10.756 / 10.962 | 1 | 32768 | 4.42 |
| matmul_64_f64/kernix | 10.032 / 10.641 | 9.731 / 9.985 | 3 | 32928 | 4.97 |
| matmul_64_f64/nalgebra | 11.843 / 12.149 | 11.232 / 11.907 | 2 | 98304 | 4.19 |
| matmul_64_f64/ndarray | 10.526 / 11.276 | 11.355 / 11.489 | 2 | 98304 | 4.17 |
| matmul_64_f64/numpy | 12.061 / 15.555 | 12.601 / 12.857 | unavailable | unavailable | 36.05 |
| matmul_ab_128_f32/kernix | 36.784 / 40.965 | 36.525 / 37.552 | 3 | 65544 | 4.88 |
| matmul_ab_128_f32/numpy | 37.337 / 42.162 | 39.482 / 40.673 | unavailable | unavailable | 36.55 |
| matmul_ab_128_f64/kernix | 70.656 / 76.291 | 71.376 / 74.081 | 3 | 131080 | 5.03 |
| matmul_ab_128_f64/numpy | 73.091 / 84.665 | 75.801 / 79.579 | unavailable | unavailable | 36.92 |
| matmul_batched_4x16x32x16_f32/candle | 1.461 / 1.507 | 1.451 / 1.506 | 25 | 13216 | 5.63 |
| matmul_batched_4x16x32x16_f32/kernix | 1.388 / 2.283 | 1.359 / 1.420 | 7 | 8200 | 4.35 |
| matmul_batched_4x16x32x16_f32/numpy | 2.150 / 2.226 | 2.200 / 2.260 | unavailable | unavailable | 35.70 |
| matmul_batched_4x16x32x16_f64/candle | 2.074 / 2.141 | 2.090 / 2.115 | 25 | 25504 | 5.60 |
| matmul_batched_4x16x32x16_f64/kernix | 1.947 / 1.973 | 2.052 / 2.104 | 7 | 16392 | 4.41 |
| matmul_batched_4x16x32x16_f64/numpy | 3.046 / 3.095 | 3.177 / 3.278 | unavailable | unavailable | 35.78 |
| matmul_sum_backward_32x16x8_f64/candle | 4.291 / 4.500 | 4.161 / 4.397 | 113 | 27988 | 5.11 |
| matmul_sum_backward_32x16x8_f64/kernix | 2.371 / 2.431 | 2.437 / 2.492 | 16 | 18376 | 4.28 |
| min_norm_32x64_rank32_f64/kernix | 59.773 / 63.139 | 57.590 / 63.457 | 20 | 59136 | 4.70 |
| min_norm_32x64_rank32_f64/nalgebra | 129.381 / 141.067 | 134.036 / 164.411 | 15 | 42232 | 4.37 |
| min_norm_32x64_rank32_f64/numpy | 124.096 / 135.135 | 120.500 / 128.476 | unavailable | unavailable | 36.74 |
| min_norm_64x32_rank16_f64/kernix | 43.885 / 47.440 | 43.528 / 45.888 | 20 | 74752 | 4.75 |
| min_norm_64x32_rank16_f64/nalgebra | 76.916 / 90.224 | 74.395 / 75.873 | 15 | 41976 | 4.26 |
| min_norm_64x32_rank16_f64/numpy | 66.209 / 72.855 | 65.675 / 79.135 | unavailable | unavailable | 36.71 |
| parallel_matmul_into_128_f32/kernix | 45.508 / 48.292 | 29.982 / 45.963 | 2 | 16 | 5.12 |
| parallel_matmul_into_128_f64/kernix | 42.230 / 49.384 | 42.214 / 66.817 | 2 | 16 | 5.61 |
| parallel_matmul_into_256_f64/kernix | 295.463 / 618.960 | 295.555 / 303.896 | 2 | 16 | 7.21 |
| parallel_matmul_into_64_f64/kernix | 9.793 / 11.058 | 10.487 / 15.351 | 2 | 98304 | 4.33 |
| pivoted_qr_256x64_f64/faer | 473.572 / 531.311 | 454.954 / 537.995 | 10 | 333312 | 4.97 |
| pivoted_qr_256x64_f64/kernix | 542.064 / 593.747 | 540.241 / 558.660 | 14 | 559104 | 5.23 |
| pivoted_qr_256x64_f64/nalgebra | 850.813 / 920.856 | 836.638 / 877.456 | 7 | 296960 | 4.80 |
| pivoted_qr_96x32_f64/faer | 70.295 / 78.055 | 71.360 / 73.363 | 10 | 68352 | 4.68 |
| pivoted_qr_96x32_f64/kernix | 58.542 / 65.110 | 55.754 / 60.879 | 14 | 107520 | 4.64 |
| pivoted_qr_96x32_f64/nalgebra | 76.127 / 83.710 | 78.160 / 80.493 | 7 | 58368 | 4.41 |
| qr_256x64_f64/faer | 420.045 / 446.831 | 441.522 / 463.647 | 7 | 331776 | 5.22 |
| qr_256x64_f64/kernix | 472.111 / 514.641 | 478.437 / 503.921 | 6 | 426496 | 4.97 |
| qr_256x64_f64/nalgebra | 362.227 / 389.209 | 364.786 / 382.208 | 4 | 295424 | 4.57 |
| qr_256x64_f64/numpy | 368.509 / 387.482 | 368.043 / 379.834 | unavailable | unavailable | 37.64 |
| qr_96x32_f64/faer | 64.272 / 70.878 | 66.345 / 70.246 | 7 | 67584 | 5.02 |
| qr_96x32_f64/kernix | 43.466 / 49.124 | 44.796 / 45.620 | 6 | 82176 | 4.39 |
| qr_96x32_f64/nalgebra | 34.993 / 37.762 | 37.316 / 39.063 | 4 | 57600 | 4.36 |
| qr_96x32_f64/numpy | 41.820 / 52.830 | 43.550 / 44.832 | unavailable | unavailable | 36.78 |
| softmax_256x64_f64/candle | 77.863 / 82.674 | 78.473 / 80.808 | 37 | 265200 | 5.93 |
| softmax_256x64_f64/kernix | 85.747 / 91.600 | 84.504 / 86.271 | 1 | 131072 | 4.61 |
| softmax_256x64_f64/ndarray | 70.836 / 75.858 | 69.904 / 74.319 | 1 | 131072 | 4.05 |
| softmax_256x64_f64/numpy | 96.953 / 103.501 | 98.850 / 100.297 | unavailable | unavailable | 35.62 |
| tensordot_16x32x8_32x8x16_f64/kernix | 33.506 / 36.466 | 34.279 / 35.366 | 9 | 2166 | 4.31 |
| tensordot_16x32x8_32x8x16_f64/numpy | 7.787 / 11.212 | 7.846 / 8.015 | unavailable | unavailable | 35.65 |
| training_backward_2x4x8x4_f64/candle | 14.518 / 20.798 | 13.980 / 15.109 | 472 | 8332 | 5.99 |
| training_backward_2x4x8x4_f64/kernix | 3.032 / 3.397 | 3.365 / 3.604 | 23 | 4224 | 4.72 |
| training_forward_2x4x8x4_f64/candle | 3.252 / 3.410 | 3.397 / 3.483 | 126 | 3912 | 5.74 |
| training_forward_2x4x8x4_f64/kernix | 2.067 / 2.110 | 2.197 / 2.251 | 17 | 1032 | 4.91 |
| training_record_2x4x8x4_f64/candle | 4.941 / 6.110 | 5.039 / 5.175 | 149 | 10712 | 5.64 |
| training_record_2x4x8x4_f64/kernix | 2.686 / 2.741 | 2.693 / 2.806 | 22 | 6832 | 4.60 |
| training_sgd_2x4x8x4_f64/kernix | 0.560 / 0.943 | 0.532 / 0.541 | 5 | 688 | 4.94 |
| training_shared_backward_64x32_f64/candle | 3.886 / 4.742 | 3.822 / 3.972 | 58 | 100004 | 5.68 |
| training_shared_backward_64x32_f64/kernix | 6.122 / 6.173 | 6.242 / 6.531 | 8 | 49768 | 4.52 |

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
