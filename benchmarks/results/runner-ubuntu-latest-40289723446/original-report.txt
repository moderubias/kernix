# Clean CPU evidence: runner-ubuntu-latest-40289723446

Measured source `7c11b2165b0cb7b523db4d938a36f1396396a138`, tree `cc7ac522fccee7e655adb7ca8d3644d95344fcb1`; dirty before/after: false.
Library source: workspace 7c11b2165b0cb7b523db4d938a36f1396396a138. CPU affinity: 0. Dtype explicitly named per workload; row-major inputs.

| Workload / library | Per-call p50 / p95 (us) | Batch-mean p50 / p95 (us) | Rust allocations | Peak additional requested bytes | Process peak RSS (MiB) |
|---|---:|---:|---:|---:|---:|
| cholesky_factor_128_f64/faer | 58.510 / 67.847 | 59.339 / 60.089 | 2 | 132096 | 4.78 |
| cholesky_factor_128_f64/kernix | 170.089 / 182.883 | 172.909 / 173.908 | 1 | 131072 | 4.19 |
| cholesky_factor_128_f64/nalgebra | 120.896 / 130.454 | 121.419 / 122.384 | 1 | 131072 | 3.93 |
| cholesky_factor_128_f64/numpy | 106.550 / 120.296 | 108.585 / 109.725 | unavailable | unavailable | 46.41 |
| cholesky_factor_256_f64/faer | 309.591 / 324.098 | 313.451 / 316.626 | 2 | 526336 | 5.39 |
| cholesky_factor_256_f64/kernix | 689.414 / 704.031 | 686.904 / 693.313 | 49 | 645120 | 5.82 |
| cholesky_factor_256_f64/nalgebra | 837.572 / 852.159 | 832.389 / 836.527 | 1 | 524288 | 4.58 |
| cholesky_factor_256_f64/numpy | 1093.802 / 1120.031 | 1099.430 / 1104.744 | unavailable | unavailable | 49.84 |
| cholesky_factor_512_f64/faer | 2086.485 / 2111.572 | 2087.541 / 2097.730 | 2 | 2101248 | 8.57 |
| cholesky_factor_512_f64/kernix | 3890.852 / 3984.157 | 3895.641 / 3933.241 | 161 | 2269184 | 10.55 |
| cholesky_factor_512_f64/nalgebra | 7377.353 / 7556.589 | 7352.306 / 7490.345 | 1 | 2097152 | 7.30 |
| cholesky_factor_512_f64/numpy | 6308.358 / 6388.517 | 6320.539 / 6491.625 | unavailable | unavailable | 60.46 |
| cholesky_factor_64_f64/faer | 14.377 / 14.477 | 14.630 / 14.814 | 2 | 33280 | 4.05 |
| cholesky_factor_64_f64/kernix | 24.977 / 25.308 | 25.480 / 25.729 | 2 | 33280 | 4.15 |
| cholesky_factor_64_f64/nalgebra | 20.849 / 21.160 | 21.101 / 21.292 | 1 | 32768 | 3.69 |
| cholesky_factor_64_f64/numpy | 18.164 / 18.875 | 18.571 / 18.819 | unavailable | unavailable | 45.59 |
| cholesky_rank_downdate_256_f64/kernix | 160.802 / 172.394 | 162.601 / 165.571 | 3 | 526336 | 5.68 |
| cholesky_rank_downdate_256_f64/nalgebra | 39.785 / 48.030 | 35.920 / 37.545 | 2 | 526336 | 6.23 |
| cholesky_rank_downdate_256_f64/refactor | 688.853 / 700.584 | 689.967 / 694.917 | 49 | 645120 | 6.47 |
| cholesky_rank_update_256_f64/kernix | 163.848 / 175.189 | 165.256 / 168.781 | 3 | 526336 | 5.70 |
| cholesky_rank_update_256_f64/nalgebra | 36.889 / 41.959 | 35.477 / 37.193 | 2 | 526336 | 6.24 |
| cholesky_rank_update_256_f64/refactor | 684.053 / 695.764 | 682.821 / 688.784 | 49 | 645120 | 6.30 |
| cholesky_rank_update_512_f64/kernix | 851.367 / 874.922 | 823.652 / 926.953 | 3 | 2101248 | 10.05 |
| cholesky_rank_update_512_f64/nalgebra | 151.384 / 164.649 | 150.280 / 152.839 | 2 | 2101248 | 13.61 |
| cholesky_rank_update_512_f64/refactor | 3870.674 / 3947.366 | 3877.550 / 3936.981 | 161 | 2269184 | 12.37 |
| create_ones_64x64_f32/candle | 0.441 / 0.451 | 0.458 / 0.461 | 7 | 16696 | 4.55 |
| create_ones_64x64_f32/faer | 0.390 / 0.391 | 0.381 / 0.383 | 1 | 16384 | 3.93 |
| create_ones_64x64_f32/kernix | 0.431 / 0.441 | 0.449 / 0.453 | 1 | 16384 | 3.75 |
| create_ones_64x64_f32/ndarray | 0.351 / 0.351 | 0.358 / 0.360 | 1 | 16384 | 3.70 |
| create_ones_64x64_f32/numpy | 2.034 / 2.084 | 2.146 / 2.180 | unavailable | unavailable | 41.99 |
| create_zeros_64x64_f32/candle | 0.271 / 0.281 | 0.279 / 0.283 | 5 | 16648 | 3.67 |
| create_zeros_64x64_f32/faer | 0.310 / 0.311 | 0.267 / 0.269 | 1 | 16384 | 4.00 |
| create_zeros_64x64_f32/kernix | 0.281 / 0.300 | 0.296 / 0.297 | 1 | 16384 | 3.78 |
| create_zeros_64x64_f32/ndarray | 0.200 / 0.201 | 0.205 / 0.209 | 1 | 16384 | 3.80 |
| create_zeros_64x64_f32/numpy | 0.722 / 0.792 | 0.809 / 0.822 | unavailable | unavailable | 42.04 |
| matmul_128_f32/candle | 45.686 / 51.857 | 46.201 / 46.561 | 9 | 65816 | 5.14 |
| matmul_128_f32/faer | 49.834 / 59.261 | 50.449 / 50.572 | 1 | 65536 | 4.46 |
| matmul_128_f32/kernix | 45.435 / 47.790 | 45.978 / 46.291 | 3 | 65544 | 4.60 |
| matmul_128_f32/nalgebra | 75.252 / 90.791 | 77.972 / 78.636 | 2 | 163840 | 3.95 |
| matmul_128_f32/ndarray | 73.708 / 91.251 | 76.683 / 77.491 | 2 | 163840 | 4.02 |
| matmul_128_f32/numpy | 49.001 / 59.321 | 49.740 / 50.179 | unavailable | unavailable | 44.37 |
| matmul_128_f64/candle | 92.724 / 103.113 | 94.055 / 94.920 | 9 | 131352 | 5.33 |
| matmul_128_f64/faer | 113.362 / 124.554 | 114.896 / 115.746 | 1 | 131072 | 4.32 |
| matmul_128_f64/kernix | 92.404 / 103.074 | 93.635 / 95.828 | 3 | 131080 | 4.83 |
| matmul_128_f64/nalgebra | 208.542 / 232.806 | 209.604 / 210.595 | 2 | 327680 | 4.12 |
| matmul_128_f64/ndarray | 210.826 / 235.331 | 212.748 / 214.224 | 2 | 327680 | 4.18 |
| matmul_128_f64/numpy | 100.649 / 112.210 | 102.458 / 103.326 | unavailable | unavailable | 44.66 |
| matmul_256_f32/candle | 355.758 / 379.773 | 358.828 / 361.156 | 9 | 262424 | 5.74 |
| matmul_256_f32/faer | 409.348 / 430.067 | 413.697 / 416.322 | 1 | 262144 | 4.89 |
| matmul_256_f32/kernix | 353.403 / 365.576 | 357.050 / 360.433 | 3 | 262152 | 5.21 |
| matmul_256_f32/nalgebra | 609.694 / 630.624 | 610.040 / 615.007 | 2 | 589824 | 4.00 |
| matmul_256_f32/ndarray | 604.234 / 623.179 | 605.194 / 610.289 | 2 | 589824 | 4.72 |
| matmul_256_f32/numpy | 362.150 / 376.507 | 365.534 / 368.608 | unavailable | unavailable | 48.19 |
| matmul_256_f64/candle | 735.450 / 748.223 | 736.157 / 747.611 | 9 | 524568 | 6.93 |
| matmul_256_f64/faer | 971.814 / 988.956 | 981.327 / 1003.221 | 1 | 524288 | 5.64 |
| matmul_256_f64/kernix | 740.770 / 752.372 | 740.087 / 743.955 | 3 | 524296 | 6.00 |
| matmul_256_f64/nalgebra | 899.177 / 911.320 | 899.876 / 905.759 | 2 | 1179648 | 5.07 |
| matmul_256_f64/ndarray | 908.665 / 925.757 | 907.693 / 914.199 | 2 | 1179648 | 5.89 |
| matmul_256_f64/numpy | 776.227 / 809.128 | 770.689 / 773.711 | unavailable | unavailable | 48.94 |
| matmul_32_f64/candle | 2.034 / 2.074 | 2.074 / 2.094 | 9 | 8472 | 4.16 |
| matmul_32_f64/faer | 2.034 / 2.063 | 2.008 / 2.031 | 1 | 8192 | 4.07 |
| matmul_32_f64/kernix | 2.414 / 2.465 | 2.452 / 2.491 | 3 | 24576 | 4.15 |
| matmul_32_f64/nalgebra | 2.114 / 2.144 | 2.150 / 2.167 | 2 | 24576 | 3.89 |
| matmul_32_f64/ndarray | 2.154 / 2.184 | 2.154 / 2.167 | 2 | 24576 | 3.70 |
| matmul_32_f64/numpy | 3.657 / 3.727 | 3.805 / 3.845 | unavailable | unavailable | 43.34 |
| matmul_64_f32/candle | 6.442 / 6.523 | 6.521 / 6.558 | 9 | 16664 | 5.13 |
| matmul_64_f32/faer | 6.623 / 6.693 | 6.691 / 6.812 | 1 | 16384 | 4.07 |
| matmul_64_f32/kernix | 6.272 / 6.352 | 6.384 / 6.406 | 3 | 16448 | 4.41 |
| matmul_64_f32/nalgebra | 7.414 / 7.494 | 7.509 / 7.538 | 2 | 49152 | 3.93 |
| matmul_64_f32/ndarray | 7.354 / 7.424 | 7.419 / 7.552 | 2 | 49152 | 4.00 |
| matmul_64_f32/numpy | 8.105 / 8.215 | 8.334 / 8.420 | unavailable | unavailable | 43.46 |
| matmul_64_f64/candle | 12.544 / 12.654 | 12.694 / 12.848 | 9 | 33048 | 5.03 |
| matmul_64_f64/faer | 13.656 / 13.766 | 13.824 / 13.974 | 1 | 32768 | 4.17 |
| matmul_64_f64/kernix | 12.614 / 12.714 | 12.767 / 12.832 | 3 | 32832 | 4.58 |
| matmul_64_f64/nalgebra | 15.279 / 15.389 | 15.459 / 15.550 | 2 | 98304 | 3.91 |
| matmul_64_f64/ndarray | 15.199 / 15.309 | 15.427 / 15.463 | 2 | 98304 | 3.86 |
| matmul_64_f64/numpy | 14.959 / 15.159 | 15.282 / 15.341 | unavailable | unavailable | 43.55 |
| matmul_ab_128_f32/kernix | 45.695 / 48.020 | 46.218 / 46.525 | 3 | 65544 | 4.61 |
| matmul_ab_128_f32/numpy | 49.152 / 59.612 | 49.989 / 50.311 | unavailable | unavailable | 44.44 |
| matmul_ab_128_f64/kernix | 93.435 / 104.516 | 94.612 / 95.516 | 3 | 131080 | 4.70 |
| matmul_ab_128_f64/numpy | 100.458 / 111.519 | 102.065 / 102.446 | unavailable | unavailable | 44.69 |
| matmul_batched_4x16x32x16_f32/candle | 2.014 / 2.074 | 2.030 / 2.069 | 25 | 13096 | 5.25 |
| matmul_batched_4x16x32x16_f32/kernix | 1.182 / 1.203 | 1.201 / 1.222 | 3 | 4104 | 4.05 |
| matmul_batched_4x16x32x16_f32/numpy | 3.287 / 3.366 | 3.452 / 3.514 | unavailable | unavailable | 43.32 |
| matmul_batched_4x16x32x16_f64/candle | 3.016 / 3.085 | 3.062 / 3.080 | 25 | 25384 | 5.16 |
| matmul_batched_4x16x32x16_f64/kernix | 1.994 / 2.034 | 2.031 / 2.079 | 3 | 8200 | 4.13 |
| matmul_batched_4x16x32x16_f64/numpy | 4.268 / 4.339 | 4.433 / 4.521 | unavailable | unavailable | 43.28 |
| matmul_sum_backward_32x16x8_f64/candle | 5.461 / 5.580 | 5.538 / 6.135 | 113 | 27988 | 5.50 |
| matmul_sum_backward_32x16x8_f64/kernix | 3.647 / 3.707 | 3.697 / 3.729 | 15 | 18376 | 3.90 |
| matmul_transposed_128_f32/kernix | 74.360 / 88.967 | 77.474 / 78.343 | 3 | 163840 | 4.56 |
| matmul_transposed_128_f32/numpy | 50.655 / 61.175 | 51.394 / 51.739 | unavailable | unavailable | 44.43 |
| matmul_transposed_128_f64/kernix | 214.931 / 231.463 | 215.170 / 217.242 | 3 | 327680 | 4.33 |
| matmul_transposed_128_f64/numpy | 101.301 / 113.212 | 102.815 / 103.610 | unavailable | unavailable | 44.62 |
| min_norm_32x64_rank32_f64/kernix | 83.146 / 92.514 | 84.046 / 84.808 | 20 | 59136 | 4.11 |
| min_norm_32x64_rank32_f64/nalgebra | 176.441 / 185.178 | 178.977 / 180.278 | 15 | 42232 | 4.25 |
| min_norm_32x64_rank32_f64/numpy | 185.238 / 205.996 | 189.001 / 191.573 | unavailable | unavailable | 47.68 |
| min_norm_64x32_rank16_f64/kernix | 62.187 / 83.356 | 63.127 / 63.481 | 20 | 74752 | 4.50 |
| min_norm_64x32_rank16_f64/nalgebra | 103.915 / 113.613 | 105.080 / 106.235 | 15 | 41976 | 4.14 |
| min_norm_64x32_rank16_f64/numpy | 101.320 / 120.606 | 104.086 / 105.716 | unavailable | unavailable | 46.64 |
| parallel_matmul_into_128_f32/kernix | 31.218 / 40.637 | 31.375 / 31.664 | 2 | 16 | 5.05 |
| parallel_matmul_into_128_f64/kernix | 56.135 / 72.276 | 56.472 / 57.619 | 2 | 16 | 5.21 |
| parallel_matmul_into_256_f64/kernix | 393.488 / 410.360 | 397.267 / 405.017 | 2 | 16 | 6.89 |
| parallel_matmul_into_64_f64/kernix | 15.750 / 16.461 | 15.959 / 16.083 | 2 | 98304 | 4.57 |
| pivoted_qr_256x64_f64/faer | 381.826 / 401.183 | 383.822 / 387.202 | 10 | 333312 | 4.39 |
| pivoted_qr_256x64_f64/kernix | 796.976 / 817.003 | 800.051 / 815.422 | 14 | 559104 | 4.86 |
| pivoted_qr_256x64_f64/nalgebra | 755.348 / 772.449 | 754.963 / 762.117 | 7 | 296960 | 4.41 |
| pivoted_qr_96x32_f64/faer | 42.440 / 45.515 | 42.897 / 43.219 | 10 | 68352 | 4.74 |
| pivoted_qr_96x32_f64/kernix | 76.994 / 86.162 | 78.840 / 79.069 | 14 | 107520 | 4.37 |
| pivoted_qr_96x32_f64/nalgebra | 76.735 / 85.721 | 77.336 / 78.780 | 7 | 58368 | 3.97 |
| qr_1024x256_f64/faer | 13699.713 / 13926.879 | 13736.483 / 13946.067 | 7 | 5308416 | 11.67 |
| qr_1024x256_f64/kernix | 20711.954 / 21043.183 | 20869.219 / 21211.461 | 80 | 7018502 | 17.20 |
| qr_1024x256_f64/nalgebra | 33353.111 / 52081.176 | 33458.548 / 33707.496 | 4 | 4720640 | 10.23 |
| qr_1024x256_f64/numpy | 18359.428 / 18499.661 | 18432.075 / 18802.240 | unavailable | unavailable | 61.26 |
| qr_256x64_f64/faer | 422.122 / 451.427 | 428.294 / 429.269 | 7 | 331776 | 4.76 |
| qr_256x64_f64/kernix | 745.830 / 767.731 | 749.913 / 756.656 | 6 | 426496 | 4.59 |
| qr_256x64_f64/nalgebra | 554.731 / 567.885 | 557.062 / 565.872 | 4 | 295424 | 4.18 |
| qr_256x64_f64/numpy | 542.488 / 556.915 | 542.851 / 555.162 | unavailable | unavailable | 48.57 |
| qr_512x128_f64/faer | 3153.292 / 3253.227 | 3179.202 / 3225.006 | 7 | 1343488 | 6.16 |
| qr_512x128_f64/kernix | 3748.874 / 3808.906 | 3744.369 / 3802.064 | 48 | 2139142 | 7.75 |
| qr_512x128_f64/nalgebra | 4449.678 / 4506.274 | 4451.401 / 4480.379 | 4 | 1180672 | 5.27 |
| qr_512x128_f64/numpy | 3455.533 / 3544.450 | 3451.165 / 3552.576 | unavailable | unavailable | 51.73 |
| qr_96x32_f64/faer | 54.031 / 63.208 | 54.883 / 55.481 | 7 | 67584 | 4.49 |
| qr_96x32_f64/kernix | 66.866 / 75.191 | 68.182 / 69.468 | 6 | 82176 | 4.16 |
| qr_96x32_f64/nalgebra | 57.949 / 66.214 | 58.678 / 59.000 | 4 | 57600 | 3.89 |
| qr_96x32_f64/numpy | 69.130 / 87.905 | 70.914 / 71.999 | unavailable | unavailable | 47.72 |
| softmax_256x64_f64/candle | 84.519 / 94.687 | 85.630 / 86.991 | 37 | 265200 | 5.92 |
| softmax_256x64_f64/kernix | 94.247 / 103.354 | 95.392 / 95.895 | 1 | 131072 | 4.35 |
| softmax_256x64_f64/ndarray | 80.892 / 90.520 | 81.985 / 83.037 | 1 | 131072 | 4.21 |
| softmax_256x64_f64/numpy | 119.875 / 155.963 | 121.611 / 122.329 | unavailable | unavailable | 42.52 |
| solve_cholesky_128x1_f64/faer | 5.831 / 8.717 | 5.885 / 5.901 | 1 | 1024 | 4.72 |
| solve_cholesky_128x1_f64/kernix | 18.916 / 19.006 | 19.096 / 19.329 | 1 | 1024 | 4.50 |
| solve_cholesky_128x1_f64/nalgebra | 5.982 / 6.062 | 6.058 / 6.094 | 1 | 1024 | 4.03 |
| solve_cholesky_256x64_f64/faer | 275.997 / 294.062 | 279.764 / 282.852 | 1 | 131072 | 6.14 |
| solve_cholesky_256x64_f64/kernix | 1322.281 / 1354.210 | 1325.069 / 1337.339 | 1 | 131072 | 5.23 |
| solve_cholesky_256x64_f64/nalgebra | 1276.014 / 1305.009 | 1281.295 / 1299.890 | 1 | 131072 | 5.40 |
| solve_cholesky_512x128_f64/faer | 2212.311 / 2249.821 | 2212.131 / 2231.702 | 1 | 524288 | 10.70 |
| solve_cholesky_512x128_f64/kernix | 10263.337 / 10354.879 | 10251.494 / 10306.758 | 1 | 524288 | 9.77 |
| solve_cholesky_512x128_f64/nalgebra | 9838.110 / 15466.520 | 9837.439 / 10083.199 | 1 | 524288 | 9.91 |
| solve_lu_256x64_f64/faer | 349.305 / 368.401 | 355.143 / 359.616 | 2 | 262144 | 6.97 |
| solve_lu_256x64_f64/kernix | 1176.307 / 1193.399 | 1179.368 / 1190.457 | 2 | 262144 | 5.44 |
| solve_lu_256x64_f64/nalgebra | 1116.935 / 1824.052 | 1126.401 / 1132.012 | 1 | 131072 | 5.54 |
| sparse_auto_512_f64/kernix | 712.978 / 739.778 | 697.747 / 716.765 | 4 | 2142216 | 10.09 |
| sparse_auto_512_f64/kernix-dense | 70.542 / 81.423 | 71.488 / 72.639 | 1 | 2097152 | 15.89 |
| sparse_auto_512_f64/numpy | 73.218 / 83.767 | 72.180 / 72.608 | unavailable | unavailable | 69.09 |
| sparse_auto_512_f64/scipy | 1473.505 / 1514.943 | 1268.875 / 1487.587 | unavailable | unavailable | 72.26 |
| sparse_matmul_512x32_f32/faer | 61.335 / 66.794 | 60.501 / 61.454 | 1 | 65536 | 5.45 |
| sparse_matmul_512x32_f32/kernix | 20.188 / 20.338 | 20.413 / 20.528 | 1 | 65536 | 4.55 |
| sparse_matmul_512x32_f32/kernix-dense | 257.984 / 272.882 | 259.217 / 263.452 | 3 | 163840 | 6.40 |
| sparse_matmul_512x32_f32/numpy | 205.395 / 220.583 | 208.257 / 210.204 | unavailable | unavailable | 61.23 |
| sparse_matmul_512x32_f32/scipy | 18.585 / 19.417 | 18.965 / 19.198 | unavailable | unavailable | 60.29 |
| sparse_matmul_512x32_f64/faer | 66.495 / 75.722 | 67.261 / 67.969 | 1 | 131072 | 6.66 |
| sparse_matmul_512x32_f64/kernix | 34.334 / 35.156 | 34.813 / 35.583 | 1 | 131072 | 5.67 |
| sparse_matmul_512x32_f64/kernix-dense | 611.327 / 630.183 | 610.848 / 623.427 | 3 | 327680 | 8.71 |
| sparse_matmul_512x32_f64/numpy | 482.964 / 498.423 | 474.872 / 479.508 | unavailable | unavailable | 62.59 |
| sparse_matmul_512x32_f64/scipy | 27.050 / 30.167 | 27.801 / 28.661 | unavailable | unavailable | 60.94 |
| sparse_matvec_2048_f64/faer | 7.244 / 7.334 | 7.331 / 7.437 | 1 | 16384 | 12.86 |
| sparse_matvec_2048_f64/kernix | 18.394 / 18.505 | 18.619 / 20.703 | 1 | 16384 | 11.87 |
| sparse_matvec_2048_f64/kernix-dense | 2617.301 / 2812.387 | 2619.363 / 2710.248 | 3 | 155648 | 44.63 |
| sparse_matvec_2048_f64/numpy | 652.023 / 689.153 | 650.853 / 661.801 | unavailable | unavailable | 89.32 |
| sparse_matvec_2048_f64/scipy | 10.430 / 10.730 | 10.698 / 10.808 | unavailable | unavailable | 89.55 |
| sparse_product_256_f64/kernix | 166.673 / 182.943 | 165.270 / 166.574 | 4 | 67752 | 4.65 |
| sparse_product_256_f64/kernix-dense | 735.439 / 756.649 | 735.368 / 742.151 | 3 | 524296 | 7.61 |
| sparse_product_256_f64/numpy | 769.644 / 788.770 | 768.183 / 773.923 | unavailable | unavailable | 63.13 |
| sparse_product_256_f64/scipy | 67.447 / 87.454 | 68.031 / 68.773 | unavailable | unavailable | 61.69 |
| tensordot_16x32x8_32x8x16_f64/kernix | 63.459 / 71.735 | 64.063 / 64.435 | 9 | 2166 | 3.89 |
| tensordot_16x32x8_32x8x16_f64/numpy | 11.201 / 11.752 | 11.441 / 11.598 | unavailable | unavailable | 43.37 |
| training_backward_2x4x8x4_f64/candle | 16.822 / 17.443 | 16.991 / 17.179 | 472 | 8332 | 5.82 |
| training_backward_2x4x8x4_f64/kernix | 4.718 / 4.789 | 4.749 / 4.843 | 23 | 4224 | 4.39 |
| training_forward_2x4x8x4_f64/candle | 4.399 / 4.538 | 4.460 / 4.541 | 126 | 3912 | 5.75 |
| training_forward_2x4x8x4_f64/kernix | 2.985 / 3.066 | 3.023 / 3.059 | 17 | 1032 | 4.36 |
| training_record_2x4x8x4_f64/candle | 6.252 / 6.522 | 6.351 / 6.391 | 149 | 10712 | 5.89 |
| training_record_2x4x8x4_f64/kernix | 3.807 / 3.887 | 3.865 / 3.877 | 22 | 6832 | 4.50 |
| training_sgd_2x4x8x4_f64/kernix | 0.792 / 0.802 | 0.808 / 0.815 | 5 | 688 | 4.53 |
| training_shared_backward_64x32_f64/candle | 4.558 / 4.649 | 4.602 / 4.625 | 58 | 100004 | 5.06 |
| training_shared_backward_64x32_f64/kernix | 10.309 / 10.379 | 10.436 / 10.776 | 8 | 49768 | 4.36 |

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
