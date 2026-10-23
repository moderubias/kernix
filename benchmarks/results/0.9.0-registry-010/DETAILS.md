# Clean CPU evidence: 0.9.0-registry-010

Measured source `fcd9fdb97ff307e9c50d0526e8b4e1a580cb44a9`, tree `519899f0fd6fef0f3c289100267dcaf2a518fe7e`; dirty before/after: false.
Library source: registry 0.9.0. CPU affinity: 0. Dtype explicitly named per workload; row-major inputs.

| Workload / library | Per-call p50 / p95 (us) | Batch-mean p50 / p95 (us) | Rust allocations | Peak additional requested bytes | Process peak RSS (MiB) |
|---|---:|---:|---:|---:|---:|
| cholesky_factor_128_f64/faer | 39.611 / 42.451 | 40.495 / 41.479 | 2 | 132096 | 4.85 |
| cholesky_factor_128_f64/kernix | 120.034 / 125.915 | 124.220 / 131.090 | 1 | 131072 | 4.52 |
| cholesky_factor_128_f64/nalgebra | 111.245 / 123.561 | 113.386 / 119.971 | 1 | 131072 | 4.21 |
| cholesky_factor_128_f64/numpy | 74.878 / 80.064 | 75.935 / 91.609 | unavailable | unavailable | 36.78 |
| cholesky_factor_256_f64/faer | 206.834 / 217.272 | 209.752 / 217.844 | 2 | 526336 | 5.88 |
| cholesky_factor_256_f64/kernix | 685.247 / 758.056 | 701.730 / 782.928 | 1 | 524288 | 5.57 |
| cholesky_factor_256_f64/nalgebra | 752.003 / 913.637 | 752.593 / 822.554 | 1 | 524288 | 4.97 |
| cholesky_factor_256_f64/numpy | 545.514 / 798.498 | 578.123 / 629.694 | unavailable | unavailable | 40.97 |
| cholesky_factor_64_f64/faer | 8.936 / 9.013 | 8.997 / 9.226 | 2 | 33280 | 4.39 |
| cholesky_factor_64_f64/kernix | 21.573 / 23.349 | 21.785 / 22.057 | 2 | 33280 | 4.21 |
| cholesky_factor_64_f64/nalgebra | 15.131 / 18.442 | 15.496 / 18.645 | 1 | 32768 | 3.90 |
| cholesky_factor_64_f64/numpy | 11.778 / 14.231 | 12.035 / 13.267 | unavailable | unavailable | 36.13 |
| create_ones_64x64_f32/candle | 0.225 / 0.227 | 0.244 / 0.256 | 7 | 16696 | 4.78 |
| create_ones_64x64_f32/faer | 0.196 / 0.198 | 0.185 / 0.198 | 1 | 16384 | 4.10 |
| create_ones_64x64_f32/kernix | 0.205 / 0.208 | 0.204 / 0.218 | 1 | 16384 | 3.89 |
| create_ones_64x64_f32/ndarray | 0.154 / 0.156 | 0.148 / 0.153 | 1 | 16384 | 3.92 |
| create_ones_64x64_f32/numpy | 1.295 / 1.343 | 1.371 / 1.404 | unavailable | unavailable | 35.04 |
| create_zeros_64x64_f32/candle | 0.176 / 0.178 | 0.173 / 0.181 | 5 | 16648 | 4.47 |
| create_zeros_64x64_f32/faer | 0.160 / 0.171 | 0.197 / 0.204 | 1 | 16384 | 4.37 |
| create_zeros_64x64_f32/kernix | 0.156 / 0.157 | 0.154 / 0.159 | 1 | 16384 | 3.97 |
| create_zeros_64x64_f32/ndarray | 0.090 / 0.091 | 0.098 / 0.102 | 1 | 16384 | 3.89 |
| create_zeros_64x64_f32/numpy | 0.396 / 0.520 | 0.437 / 0.947 | unavailable | unavailable | 35.12 |
| matmul_128_f32/candle | 39.756 / 43.183 | 39.437 / 41.026 | 9 | 65816 | 5.41 |
| matmul_128_f32/faer | 50.973 / 54.410 | 51.305 / 65.002 | 1 | 65536 | 4.55 |
| matmul_128_f32/kernix | 38.634 / 44.078 | 39.944 / 40.776 | 3 | 65544 | 4.89 |
| matmul_128_f32/nalgebra | 50.480 / 61.766 | 54.920 / 58.970 | 2 | 163840 | 3.93 |
| matmul_128_f32/ndarray | 52.083 / 57.584 | 51.885 / 52.992 | 2 | 163840 | 4.37 |
| matmul_128_f32/numpy | 40.227 / 41.947 | 41.645 / 42.795 | unavailable | unavailable | 36.80 |
| matmul_128_f64/candle | 78.204 / 83.584 | 80.460 / 81.873 | 9 | 131352 | 5.53 |
| matmul_128_f64/faer | 86.726 / 101.386 | 87.657 / 88.681 | 1 | 131072 | 4.78 |
| matmul_128_f64/kernix | 83.207 / 88.688 | 85.480 / 89.073 | 3 | 131080 | 5.23 |
| matmul_128_f64/nalgebra | 120.325 / 155.375 | 124.254 / 128.494 | 2 | 327680 | 4.34 |
| matmul_128_f64/ndarray | 121.834 / 141.925 | 124.776 / 130.898 | 2 | 327680 | 4.43 |
| matmul_128_f64/numpy | 82.343 / 92.127 | 83.692 / 87.515 | unavailable | unavailable | 37.05 |
| matmul_256_f32/candle | 304.379 / 341.017 | 306.333 / 327.934 | 9 | 262424 | 6.09 |
| matmul_256_f32/faer | 343.122 / 362.234 | 345.206 / 365.219 | 1 | 262144 | 5.20 |
| matmul_256_f32/kernix | 294.932 / 319.111 | 303.834 / 322.345 | 3 | 262152 | 5.49 |
| matmul_256_f32/nalgebra | 412.324 / 435.053 | 418.402 / 446.470 | 2 | 589824 | 4.45 |
| matmul_256_f32/ndarray | 410.973 / 490.331 | 428.408 / 641.199 | 2 | 589824 | 5.01 |
| matmul_256_f32/numpy | 301.429 / 326.870 | 305.566 / 327.369 | unavailable | unavailable | 40.93 |
| matmul_256_f64/candle | 603.518 / 703.979 | 618.770 / 691.320 | 9 | 524568 | 7.01 |
| matmul_256_f64/faer | 758.898 / 795.954 | 757.601 / 767.896 | 1 | 524288 | 5.54 |
| matmul_256_f64/kernix | 647.477 / 743.727 | 675.090 / 706.792 | 3 | 524296 | 6.34 |
| matmul_256_f64/nalgebra | 680.500 / 787.900 | 714.643 / 740.315 | 2 | 1179648 | 5.77 |
| matmul_256_f64/ndarray | 685.012 / 800.147 | 726.086 / 762.726 | 2 | 1179648 | 6.16 |
| matmul_256_f64/numpy | 668.632 / 901.176 | 717.030 / 780.035 | unavailable | unavailable | 41.82 |
| matmul_32_f64/candle | 1.574 / 1.599 | 1.632 / 1.692 | 9 | 8592 | 5.03 |
| matmul_32_f64/faer | 1.561 / 1.713 | 1.644 / 2.334 | 1 | 8192 | 4.46 |
| matmul_32_f64/kernix | 1.687 / 1.980 | 1.718 / 1.743 | 3 | 24576 | 4.27 |
| matmul_32_f64/nalgebra | 1.481 / 1.498 | 1.538 / 1.572 | 2 | 24576 | 3.95 |
| matmul_32_f64/ndarray | 1.526 / 1.550 | 1.528 / 1.570 | 2 | 24576 | 4.00 |
| matmul_32_f64/numpy | 2.788 / 2.863 | 2.817 / 3.225 | unavailable | unavailable | 35.81 |
| matmul_64_f32/candle | 5.767 / 5.967 | 5.661 / 5.999 | 9 | 16664 | 5.20 |
| matmul_64_f32/faer | 5.624 / 5.724 | 5.598 / 5.772 | 1 | 16384 | 4.48 |
| matmul_64_f32/kernix | 5.081 / 5.215 | 5.106 / 5.178 | 3 | 16544 | 4.88 |
| matmul_64_f32/nalgebra | 6.239 / 6.432 | 6.602 / 6.853 | 2 | 49152 | 4.00 |
| matmul_64_f32/ndarray | 6.127 / 6.173 | 6.229 / 7.564 | 2 | 49152 | 4.15 |
| matmul_64_f32/numpy | 7.539 / 9.992 | 6.767 / 7.029 | unavailable | unavailable | 35.80 |
| matmul_64_f64/candle | 11.379 / 11.548 | 11.142 / 12.795 | 9 | 33048 | 5.19 |
| matmul_64_f64/faer | 11.187 / 12.169 | 11.703 / 11.915 | 1 | 32768 | 4.50 |
| matmul_64_f64/kernix | 17.926 / 18.370 | 10.910 / 18.380 | 3 | 32928 | 4.93 |
| matmul_64_f64/nalgebra | 11.195 / 12.103 | 11.414 / 11.736 | 2 | 98304 | 4.26 |
| matmul_64_f64/ndarray | 11.608 / 11.796 | 11.716 / 12.374 | 2 | 98304 | 4.25 |
| matmul_64_f64/numpy | 12.204 / 12.590 | 12.835 / 13.251 | unavailable | unavailable | 36.00 |
| matmul_batched_4x16x32x16_f64/candle | 4.315 / 4.917 | 2.604 / 4.046 | 25 | 25504 | 5.64 |
| matmul_batched_4x16x32x16_f64/kernix | 2.073 / 2.143 | 2.163 / 3.399 | 7 | 16392 | 4.53 |
| matmul_batched_4x16x32x16_f64/numpy | 3.491 / 3.585 | 3.867 / 4.258 | unavailable | unavailable | 35.70 |
| matmul_sum_backward_32x16x8_f64/candle | 4.364 / 4.641 | 4.488 / 5.099 | 113 | 27988 | 5.79 |
| matmul_sum_backward_32x16x8_f64/kernix | 2.518 / 2.580 | 2.659 / 2.838 | 16 | 18376 | 4.30 |
| min_norm_32x64_rank32_f64/kernix | 81.587 / 88.347 | 81.264 / 83.755 | 28 | 102144 | 4.59 |
| min_norm_32x64_rank32_f64/nalgebra | 142.115 / 192.377 | 221.281 / 240.614 | 15 | 42232 | 3.84 |
| min_norm_32x64_rank32_f64/numpy | 129.445 / 140.980 | 130.840 / 141.050 | unavailable | unavailable | 36.62 |
| min_norm_64x32_rank16_f64/kernix | 51.783 / 56.676 | 53.416 / 55.990 | 28 | 74752 | 4.58 |
| min_norm_64x32_rank16_f64/nalgebra | 78.173 / 88.316 | 79.322 / 83.094 | 15 | 41976 | 4.37 |
| min_norm_64x32_rank16_f64/numpy | 71.772 / 98.745 | 73.290 / 101.332 | unavailable | unavailable | 36.64 |
| parallel_matmul_into_128_f32/kernix | 29.303 / 35.440 | 31.810 / 67.671 | 4 | 163840 | 4.98 |
| parallel_matmul_into_128_f64/kernix | 105.080 / 129.501 | 77.623 / 133.470 | 4 | 327680 | 5.32 |
| parallel_matmul_into_256_f64/kernix | 423.627 / 543.582 | 423.605 / 719.649 | 8 | 1179648 | 7.56 |
| parallel_matmul_into_64_f64/kernix | 10.422 / 18.261 | 10.856 / 12.764 | 2 | 98304 | 4.62 |
| pivoted_qr_256x64_f64/faer | 485.354 / 537.792 | 495.330 / 695.130 | 10 | 333312 | 5.37 |
| pivoted_qr_256x64_f64/kernix | 599.809 / 677.285 | 606.725 / 634.422 | 14 | 559104 | 5.15 |
| pivoted_qr_256x64_f64/nalgebra | 843.150 / 1028.937 | 816.449 / 832.830 | 7 | 296960 | 4.66 |
| pivoted_qr_96x32_f64/faer | 78.191 / 83.780 | 81.269 / 88.039 | 10 | 68352 | 4.93 |
| pivoted_qr_96x32_f64/kernix | 58.492 / 82.369 | 58.601 / 63.265 | 14 | 107520 | 4.68 |
| pivoted_qr_96x32_f64/nalgebra | 79.440 / 86.797 | 80.237 / 81.133 | 7 | 58368 | 4.23 |
| qr_256x64_f64/faer | 437.890 / 486.147 | 428.358 / 443.107 | 7 | 331776 | 5.43 |
| qr_256x64_f64/kernix | 531.300 / 656.406 | 529.836 / 558.182 | 6 | 426496 | 5.03 |
| qr_256x64_f64/nalgebra | 395.774 / 541.958 | 405.579 / 424.833 | 4 | 295424 | 4.43 |
| qr_256x64_f64/numpy | 374.776 / 414.731 | 402.197 / 503.762 | unavailable | unavailable | 37.59 |
| qr_96x32_f64/faer | 69.889 / 74.223 | 71.239 / 80.432 | 7 | 67584 | 4.96 |
| qr_96x32_f64/kernix | 47.578 / 55.708 | 49.401 / 64.822 | 6 | 82176 | 4.38 |
| qr_96x32_f64/nalgebra | 40.857 / 43.235 | 40.991 / 43.495 | 4 | 57600 | 4.12 |
| qr_96x32_f64/numpy | 43.608 / 46.945 | 43.739 / 46.500 | unavailable | unavailable | 36.77 |
| softmax_256x64_f64/candle | 93.798 / 98.375 | 97.877 / 110.294 | 37 | 265200 | 5.82 |
| softmax_256x64_f64/kernix | 93.662 / 103.512 | 93.330 / 96.827 | 1 | 131072 | 4.52 |
| softmax_256x64_f64/ndarray | 85.808 / 96.042 | 87.460 / 91.622 | 1 | 131072 | 4.31 |
| softmax_256x64_f64/numpy | 110.451 / 118.564 | 114.811 / 127.311 | unavailable | unavailable | 35.61 |
| tensordot_16x32x8_32x8x16_f64/kernix | 691.795 / 733.280 | 707.567 / 751.203 | 12 | 2230 | 4.14 |
| tensordot_16x32x8_32x8x16_f64/numpy | 12.340 / 15.680 | 8.522 / 10.739 | unavailable | unavailable | 35.66 |
| training_backward_2x4x8x4_f64/candle | 14.863 / 15.317 | 15.190 / 16.630 | 472 | 8332 | 6.37 |
| training_backward_2x4x8x4_f64/kernix | 3.311 / 4.254 | 3.380 / 3.543 | 23 | 4224 | 4.68 |
| training_forward_2x4x8x4_f64/candle | 3.880 / 4.082 | 3.741 / 3.962 | 126 | 3912 | 6.20 |
| training_forward_2x4x8x4_f64/kernix | 2.572 / 2.689 | 2.460 / 2.659 | 17 | 1032 | 4.54 |
| training_record_2x4x8x4_f64/candle | 5.435 / 5.752 | 5.616 / 5.874 | 149 | 10712 | 5.93 |
| training_record_2x4x8x4_f64/kernix | 2.926 / 3.009 | 3.013 / 3.116 | 22 | 6832 | 4.21 |
| training_sgd_2x4x8x4_f64/kernix | 0.585 / 0.621 | 0.597 / 0.647 | 5 | 688 | 4.84 |
| training_shared_backward_64x32_f64/candle | 3.961 / 4.117 | 4.118 / 4.239 | 58 | 100004 | 5.26 |
| training_shared_backward_64x32_f64/kernix | 6.488 / 7.580 | 6.660 / 6.945 | 10 | 66152 | 4.48 |

## Method and limits

- Rows explicitly marked two workers use a private Kernix executor with two workers and reusable output allocated before the first call. RefCell borrowing is included; pool construction is excluded. These rows compare Kernix versions, with no single-thread peer score. Linux uses up to two allowed CPUs, preferring distinct reported physical cores; Windows/macOS affinity is not pinned. Exact per-workload CPU sets are in explicit_execution metadata. Ordinary cases retain one BLAS/Rayon thread and the original single-CPU Linux policy.
- Coalesced tensordot contracts [16,32,8] with [32,8,16] over two adjacent axes using the actual Kernix/NumPy tensor APIs. Shared-variable backward uses a prepared real graph of sum(X*X), 64x32 f64, and fresh gradient stores; recording and input construction are excluded in both frameworks.
- Per-call quantiles use nearest ranks over 256 individual warmed calls; batch quantiles use 15 means over separate 10-ms batches. They are separate distributions. Raw samples remain in raw.json.
- Eight warmup calls follow validation. Ordinary timing includes allocation, dispatch, computation and output destruction; explicitly marked reusable-output cases reuse their fixture buffer. The first call field excludes output destruction. Inputs and library/runtime imports are prepared outside timing.
- Three extra processes record their first operation calls. These are first-use observations, not cache-flushed or device-startup latency. Timer-pair overhead is reported without subtraction; tiny calls are clock-sensitive.
- ndarray and Candle softmax are explicit stable compositions, not claims about a native softmax API.
- Constructor cases allocate owned 64x64 f32 tensors. Pre-0.6 registry baselines use full(value), explicitly marked in metadata; current workspace runs use zeros/ones. No nested literal reinterpretation is involved.
- All GEMM inputs are C-order and match elementwise. Native output layout differs by library. QR materializes thin Q and R in every implementation. Cholesky returns reusable factors (or NumPy L); no extra factor materialization is timed.
- Pivoted QR materializes thin Q/R and an owned permutation in Kernix, nalgebra and faer. Minimum-norm cases include factorization and four RHS: Kernix uses pivoted QR plus scaled right QR with relative cutoff 1e-12, NumPy uses native lstsq with rcond=1e-12, and nalgebra uses native SVD plus solve with absolute cutoff 1e-10. These algorithms have different costs and truncation policies. Fixtures have a wide rank gap; independent null-space orthogonality and normal-residual checks must both pass 2e-10 before timing. Results do not establish equivalence near a rank threshold or general SVD performance.
- Kernix checks finite values/symmetry. Some competitors validate a narrower contract. No finite-data precondition check was removed to improve timing. Accuracy results and budgets are retained per worker.
- The backward case uses an actual reverse-mode graph in Kernix and Candle for sum(A @ W), with A constant and W trainable. Both copy input/weight data and returned gradients; no manual derivative is used as a framework ranking.
- Allocations are measured in a separately compiled instrumented binary after warmup, outside all timing loops. Counts cover Rust global-allocator requests, not native allocator internals, stack memory or all NumPy allocations. NumPy allocation counts are unavailable, not zero.
- Native engines may retain thread-local packing buffers. Warmed allocation probes exclude first-use allocations and retained scratch; first-use timing and process RSS are separate observations. See the measured library numerical contract for that version.
- Requested-byte peak is additional live Rust allocation during one call, including output; it is not scratch-only memory or peak RSS. RSS is the process high-water mark including fixtures, validation and runtime, read from Linux VmHWM. Other platforms may report unavailable.
- The published cases are allocation-inclusive CPU APIs and graph workloads. Pure internal kernel-only and end-to-end application/model latency are not measured. No workload result implies a complete application frame rate.
- faer is explicitly serial; Rayon and BLAS thread counts are one. No GPU comparison. Python/NumPy timings include Python dispatch. Backend and dependency versions/configuration, governor and RAM are recorded.
- Training stages use batched matmul + bias + layer_norm + tanh + step-two feature slice + MSE (Candle index_select). Forward excludes graph recording; record includes parameter/input/target snapshots and eager forward/loss; backward reuses a prepared graph and returns fresh derivatives. Allocation probes measure the corresponding stage, including graph values/scratch, not graph-container-only bytes. SGD uses fixed precomputed real gradients and includes transactional validation/parameter updates; it is not an end-to-end converging training loop. Candle normalization is an explicit population-statistics composition; its checks/rounding differ. Historical registry baselines without these Tape operations omit the stages, recorded in metadata, rather than substitute manual derivatives.
- Host load/frequency/cache effects are uncontrolled. Report comparator wins and losses; no universal ranking follows. The source archive comes from committed bytes, with result directories excluded. Library dependencies are pinned by the saved driver lockfile.
