# Clean CPU evidence: runner-ubuntu-24.04-arm-40289723446

Measured source `7c11b2165b0cb7b523db4d938a36f1396396a138`, tree `cc7ac522fccee7e655adb7ca8d3644d95344fcb1`; dirty before/after: false.
Library source: workspace 7c11b2165b0cb7b523db4d938a36f1396396a138. CPU affinity: 0. Dtype explicitly named per workload; row-major inputs.

| Workload / library | Per-call p50 / p95 (us) | Batch-mean p50 / p95 (us) | Rust allocations | Peak additional requested bytes | Process peak RSS (MiB) |
|---|---:|---:|---:|---:|---:|
| cholesky_factor_128_f64/faer | 65.268 / 67.316 | 65.223 / 65.312 | 16 | 148736 | 3.38 |
| cholesky_factor_128_f64/kernix | 277.143 / 280.798 | 275.474 / 277.296 | 1 | 131072 | 3.14 |
| cholesky_factor_128_f64/nalgebra | 128.952 / 132.647 | 130.554 / 130.902 | 1 | 131072 | 2.83 |
| cholesky_factor_128_f64/numpy | 85.267 / 98.058 | 86.492 / 87.331 | unavailable | unavailable | 35.89 |
| cholesky_factor_256_f64/faer | 404.576 / 416.191 | 406.445 / 407.805 | 76 | 592000 | 4.30 |
| cholesky_factor_256_f64/kernix | 769.282 / 778.978 | 767.012 / 771.206 | 49 | 645120 | 4.52 |
| cholesky_factor_256_f64/nalgebra | 1031.412 / 1038.987 | 1038.637 / 1042.510 | 1 | 524288 | 3.58 |
| cholesky_factor_256_f64/numpy | 745.574 / 761.982 | 740.927 / 747.398 | unavailable | unavailable | 38.73 |
| cholesky_factor_512_f64/faer | 2633.094 / 2649.949 | 2629.986 / 2643.243 | 388 | 2297864 | 9.25 |
| cholesky_factor_512_f64/kernix | 4336.307 / 4365.473 | 4323.340 / 4350.399 | 161 | 2269184 | 9.06 |
| cholesky_factor_512_f64/nalgebra | 8489.056 / 8561.617 | 8276.806 / 8476.841 | 1 | 2097152 | 6.58 |
| cholesky_factor_512_f64/numpy | 3859.118 / 3881.701 | 3868.097 / 3881.088 | unavailable | unavailable | 48.23 |
| cholesky_factor_64_f64/faer | 13.256 / 13.287 | 13.281 / 13.322 | 2 | 33280 | 2.80 |
| cholesky_factor_64_f64/kernix | 27.374 / 28.143 | 27.312 / 27.427 | 2 | 33280 | 2.92 |
| cholesky_factor_64_f64/nalgebra | 20.391 / 20.911 | 20.614 / 20.666 | 1 | 32768 | 2.64 |
| cholesky_factor_64_f64/numpy | 17.671 / 18.286 | 17.856 / 17.893 | unavailable | unavailable | 35.10 |
| cholesky_rank_downdate_256_f64/kernix | 150.517 / 157.972 | 153.825 / 154.386 | 3 | 526336 | 4.40 |
| cholesky_rank_downdate_256_f64/nalgebra | 33.566 / 34.630 | 34.009 / 34.314 | 2 | 526336 | 5.03 |
| cholesky_rank_downdate_256_f64/refactor | 764.447 / 773.510 | 760.592 / 763.931 | 49 | 645120 | 5.08 |
| cholesky_rank_update_256_f64/kernix | 151.119 / 161.958 | 153.694 / 154.257 | 3 | 526336 | 4.40 |
| cholesky_rank_update_256_f64/nalgebra | 33.846 / 34.910 | 34.163 / 34.487 | 2 | 526336 | 5.03 |
| cholesky_rank_update_256_f64/refactor | 764.425 / 774.832 | 763.935 / 767.982 | 49 | 645120 | 5.08 |
| cholesky_rank_update_512_f64/kernix | 657.513 / 669.633 | 660.778 / 665.811 | 3 | 2101248 | 8.90 |
| cholesky_rank_update_512_f64/nalgebra | 148.319 / 161.199 | 149.697 / 154.684 | 2 | 2101248 | 12.53 |
| cholesky_rank_update_512_f64/refactor | 4356.637 / 4387.916 | 4333.886 / 4365.130 | 161 | 2269184 | 11.16 |
| create_ones_64x64_f32/candle | 0.336 / 0.352 | 0.348 / 0.349 | 7 | 16696 | 3.30 |
| create_ones_64x64_f32/faer | 0.240 / 0.248 | 0.235 / 0.236 | 1 | 16384 | 2.86 |
| create_ones_64x64_f32/kernix | 0.256 / 0.264 | 0.259 / 0.260 | 1 | 16384 | 2.72 |
| create_ones_64x64_f32/ndarray | 0.208 / 0.216 | 0.216 / 0.218 | 1 | 16384 | 2.72 |
| create_ones_64x64_f32/numpy | 1.528 / 1.568 | 1.595 / 1.606 | unavailable | unavailable | 33.99 |
| create_zeros_64x64_f32/candle | 0.240 / 0.256 | 0.247 / 0.250 | 5 | 16648 | 3.18 |
| create_zeros_64x64_f32/faer | 0.320 / 0.328 | 0.314 / 0.322 | 1 | 16384 | 2.86 |
| create_zeros_64x64_f32/kernix | 0.208 / 0.232 | 0.212 / 0.227 | 1 | 16384 | 2.78 |
| create_zeros_64x64_f32/ndarray | 0.136 / 0.144 | 0.146 / 0.151 | 1 | 16384 | 2.71 |
| create_zeros_64x64_f32/numpy | 0.528 / 0.560 | 0.565 / 0.570 | unavailable | unavailable | 33.98 |
| matmul_128_f32/candle | 85.867 / 88.331 | 86.075 / 86.163 | 10 | 131160 | 3.71 |
| matmul_128_f32/faer | 86.907 / 88.731 | 87.128 / 87.187 | 3 | 196616 | 3.22 |
| matmul_128_f32/kernix | 85.419 / 87.747 | 85.641 / 85.749 | 4 | 131080 | 3.48 |
| matmul_128_f32/nalgebra | 93.604 / 100.380 | 94.984 / 95.335 | 2 | 163840 | 2.91 |
| matmul_128_f32/ndarray | 93.987 / 102.210 | 95.597 / 96.110 | 2 | 163840 | 2.98 |
| matmul_128_f32/numpy | 99.849 / 104.000 | 100.515 / 101.037 | unavailable | unavailable | 35.75 |
| matmul_128_f64/candle | 168.223 / 170.902 | 168.740 / 168.951 | 10 | 262232 | 4.00 |
| matmul_128_f64/faer | 169.638 / 175.606 | 170.506 / 170.806 | 3 | 393224 | 3.54 |
| matmul_128_f64/kernix | 168.406 / 171.998 | 168.927 / 169.156 | 4 | 262152 | 3.67 |
| matmul_128_f64/nalgebra | 209.276 / 220.803 | 209.750 / 210.432 | 2 | 327680 | 3.02 |
| matmul_128_f64/ndarray | 209.087 / 215.814 | 208.859 / 209.263 | 2 | 327680 | 3.14 |
| matmul_128_f64/numpy | 196.046 / 201.349 | 197.364 / 197.750 | unavailable | unavailable | 35.89 |
| matmul_256_f32/candle | 758.614 / 769.373 | 757.537 / 758.862 | 10 | 524376 | 4.56 |
| matmul_256_f32/faer | 674.769 / 680.392 | 672.850 / 679.014 | 3 | 786440 | 4.23 |
| matmul_256_f32/kernix | 663.041 / 669.168 | 661.812 / 663.736 | 4 | 524296 | 4.30 |
| matmul_256_f32/nalgebra | 762.699 / 774.915 | 759.971 / 762.607 | 2 | 589824 | 3.46 |
| matmul_256_f32/ndarray | 763.872 / 776.848 | 761.681 / 764.654 | 2 | 589824 | 3.71 |
| matmul_256_f32/numpy | 722.714 / 734.538 | 723.928 / 727.518 | unavailable | unavailable | 39.86 |
| matmul_256_f64/candle | 1324.601 / 1337.272 | 1324.458 / 1329.627 | 10 | 1048664 | 5.60 |
| matmul_256_f64/faer | 1543.694 / 1561.390 | 1544.387 / 1551.879 | 3 | 1048584 | 5.47 |
| matmul_256_f64/kernix | 1323.383 / 1337.150 | 1322.666 / 1326.345 | 4 | 1048584 | 5.24 |
| matmul_256_f64/nalgebra | 1331.586 / 1347.521 | 1336.489 / 1340.883 | 2 | 1179648 | 4.33 |
| matmul_256_f64/ndarray | 1324.634 / 1337.114 | 1327.111 / 1333.809 | 2 | 1179648 | 4.84 |
| matmul_256_f64/numpy | 1470.338 / 1491.393 | 1468.240 / 1480.732 | unavailable | unavailable | 40.34 |
| matmul_32_f64/candle | 3.440 / 3.472 | 3.452 / 3.514 | 10 | 16976 | 3.40 |
| matmul_32_f64/faer | 3.496 / 3.519 | 3.498 / 3.508 | 3 | 25088 | 2.98 |
| matmul_32_f64/kernix | 3.248 / 3.280 | 3.263 / 3.282 | 3 | 24576 | 2.88 |
| matmul_32_f64/nalgebra | 3.208 / 3.232 | 3.232 / 3.238 | 2 | 24576 | 2.73 |
| matmul_32_f64/ndarray | 3.040 / 3.072 | 3.056 / 3.106 | 2 | 24576 | 2.74 |
| matmul_32_f64/numpy | 8.216 / 8.264 | 8.301 / 8.321 | unavailable | unavailable | 34.65 |
| matmul_64_f32/candle | 11.959 / 12.007 | 11.990 / 12.020 | 10 | 33104 | 3.45 |
| matmul_64_f32/faer | 12.295 / 12.319 | 12.315 / 12.354 | 3 | 49408 | 3.07 |
| matmul_64_f32/kernix | 11.767 / 11.823 | 11.796 / 11.840 | 4 | 33024 | 3.20 |
| matmul_64_f32/nalgebra | 11.279 / 11.303 | 11.308 / 11.335 | 2 | 49152 | 2.77 |
| matmul_64_f32/ndarray | 11.279 / 11.296 | 11.286 / 11.317 | 2 | 49152 | 2.78 |
| matmul_64_f32/numpy | 20.119 / 20.335 | 20.268 / 20.341 | unavailable | unavailable | 34.89 |
| matmul_64_f64/candle | 22.558 / 22.607 | 22.610 / 22.625 | 10 | 65872 | 3.52 |
| matmul_64_f64/faer | 22.639 / 22.695 | 22.688 / 22.714 | 3 | 98560 | 3.06 |
| matmul_64_f64/kernix | 22.182 / 22.263 | 22.248 / 22.268 | 4 | 65792 | 3.20 |
| matmul_64_f64/nalgebra | 22.463 / 22.511 | 22.521 / 22.585 | 2 | 98304 | 2.81 |
| matmul_64_f64/ndarray | 21.799 / 21.855 | 21.851 / 21.930 | 2 | 98304 | 2.86 |
| matmul_64_f64/numpy | 31.086 / 31.782 | 31.241 / 31.335 | unavailable | unavailable | 34.93 |
| matmul_ab_128_f32/kernix | 85.468 / 87.931 | 85.731 / 86.038 | 4 | 131080 | 3.61 |
| matmul_ab_128_f32/numpy | 100.042 / 102.298 | 100.390 / 100.760 | unavailable | unavailable | 35.65 |
| matmul_ab_128_f64/kernix | 168.693 / 177.619 | 169.646 / 170.002 | 4 | 262152 | 3.80 |
| matmul_ab_128_f64/numpy | 195.731 / 201.698 | 196.929 / 197.144 | unavailable | unavailable | 35.89 |
| matmul_batched_4x16x32x16_f32/candle | 2.872 / 2.911 | 2.883 / 2.900 | 25 | 13408 | 3.77 |
| matmul_batched_4x16x32x16_f32/kernix | 2.400 / 2.432 | 2.416 / 2.420 | 7 | 8200 | 2.98 |
| matmul_batched_4x16x32x16_f32/numpy | 5.151 / 5.183 | 5.225 / 5.243 | unavailable | unavailable | 34.64 |
| matmul_batched_4x16x32x16_f64/candle | 4.408 / 4.456 | 4.444 / 4.491 | 25 | 25696 | 3.68 |
| matmul_batched_4x16x32x16_f64/kernix | 3.831 / 3.880 | 3.852 / 3.858 | 7 | 16392 | 3.02 |
| matmul_batched_4x16x32x16_f64/numpy | 12.999 / 13.160 | 13.101 / 13.153 | unavailable | unavailable | 34.66 |
| matmul_sum_backward_32x16x8_f64/candle | 7.255 / 7.471 | 7.304 / 7.370 | 113 | 27980 | 3.78 |
| matmul_sum_backward_32x16x8_f64/kernix | 3.536 / 3.600 | 3.518 / 3.547 | 16 | 18376 | 2.98 |
| matmul_transposed_128_f32/kernix | 93.163 / 103.130 | 95.301 / 97.282 | 3 | 163840 | 3.28 |
| matmul_transposed_128_f32/numpy | 100.170 / 104.266 | 100.757 / 100.947 | unavailable | unavailable | 35.64 |
| matmul_transposed_128_f64/kernix | 214.161 / 229.928 | 215.152 / 216.637 | 3 | 327680 | 3.53 |
| matmul_transposed_128_f64/numpy | 195.461 / 200.461 | 196.716 / 197.681 | unavailable | unavailable | 35.89 |
| min_norm_32x64_rank32_f64/kernix | 96.540 / 100.900 | 96.707 / 97.434 | 20 | 59136 | 3.18 |
| min_norm_32x64_rank32_f64/nalgebra | 218.379 / 221.107 | 218.748 / 218.976 | 15 | 42232 | 2.90 |
| min_norm_32x64_rank32_f64/numpy | 176.124 / 191.531 | 178.991 / 179.557 | unavailable | unavailable | 35.66 |
| min_norm_64x32_rank16_f64/kernix | 72.220 / 74.547 | 72.365 / 72.477 | 20 | 74752 | 3.19 |
| min_norm_64x32_rank16_f64/nalgebra | 117.265 / 120.656 | 117.441 / 117.549 | 15 | 41976 | 2.90 |
| min_norm_64x32_rank16_f64/numpy | 93.675 / 105.762 | 95.365 / 95.672 | unavailable | unavailable | 35.54 |
| parallel_matmul_into_128_f32/kernix | 50.333 / 53.237 | 50.079 / 50.658 | 4 | 65552 | 3.90 |
| parallel_matmul_into_128_f64/kernix | 93.930 / 98.442 | 94.843 / 95.406 | 4 | 131088 | 4.11 |
| parallel_matmul_into_256_f64/kernix | 692.783 / 710.430 | 696.065 / 703.404 | 4 | 524304 | 6.30 |
| parallel_matmul_into_64_f64/kernix | 16.448 / 18.095 | 16.254 / 16.736 | 2 | 98304 | 3.29 |
| pivoted_qr_256x64_f64/faer | 473.945 / 481.361 | 513.803 / 515.968 | 58 | 333312 | 4.03 |
| pivoted_qr_256x64_f64/kernix | 782.192 / 795.048 | 766.811 / 782.932 | 14 | 559104 | 3.61 |
| pivoted_qr_256x64_f64/nalgebra | 1151.266 / 1156.794 | 1150.587 / 1152.359 | 7 | 296960 | 3.29 |
| pivoted_qr_96x32_f64/faer | 62.251 / 64.220 | 62.408 / 62.553 | 32 | 68352 | 3.44 |
| pivoted_qr_96x32_f64/kernix | 88.163 / 90.371 | 88.383 / 88.627 | 14 | 107520 | 3.00 |
| pivoted_qr_96x32_f64/nalgebra | 114.211 / 117.851 | 114.465 / 114.568 | 7 | 58368 | 2.87 |
| qr_1024x256_f64/faer | 20229.733 / 20398.659 | 20135.601 / 20322.742 | 764 | 7135236 | 13.75 |
| qr_1024x256_f64/kernix | 24134.286 / 24510.832 | 24170.208 / 24510.315 | 51 | 7276544 | 14.83 |
| qr_1024x256_f64/nalgebra | 36091.080 / 36229.409 | 36065.105 / 36240.830 | 4 | 4720640 | 9.21 |
| qr_1024x256_f64/numpy | 19809.741 / 19924.877 | 19767.881 / 19890.520 | unavailable | unavailable | 49.23 |
| qr_256x64_f64/faer | 447.474 / 456.642 | 449.384 / 450.353 | 83 | 331776 | 3.74 |
| qr_256x64_f64/kernix | 693.830 / 707.430 | 695.208 / 698.411 | 6 | 426496 | 3.58 |
| qr_256x64_f64/nalgebra | 585.304 / 595.295 | 585.615 / 587.154 | 4 | 295424 | 3.11 |
| qr_256x64_f64/numpy | 565.441 / 583.751 | 566.988 / 573.800 | unavailable | unavailable | 36.34 |
| qr_512x128_f64/faer | 3113.540 / 3131.482 | 3113.390 / 3128.762 | 360 | 1859588 | 5.68 |
| qr_512x128_f64/kernix | 4162.684 / 4205.776 | 4153.033 / 4244.453 | 31 | 2098176 | 6.21 |
| qr_512x128_f64/nalgebra | 4646.439 / 4679.532 | 4645.815 / 4678.133 | 4 | 1180672 | 4.33 |
| qr_512x128_f64/numpy | 4122.093 / 4152.596 | 4131.642 / 4150.119 | unavailable | unavailable | 39.24 |
| qr_96x32_f64/faer | 65.530 / 67.522 | 65.719 / 66.200 | 41 | 67584 | 3.30 |
| qr_96x32_f64/kernix | 73.244 / 75.260 | 73.403 / 74.274 | 6 | 82176 | 2.98 |
| qr_96x32_f64/nalgebra | 61.292 / 63.220 | 61.451 / 61.500 | 4 | 57600 | 2.80 |
| qr_96x32_f64/numpy | 66.685 / 80.804 | 68.256 / 68.634 | unavailable | unavailable | 35.63 |
| softmax_256x64_f64/candle | 110.506 / 113.114 | 110.709 / 110.831 | 37 | 265200 | 4.28 |
| softmax_256x64_f64/kernix | 113.009 / 114.914 | 113.295 / 113.360 | 1 | 131072 | 3.07 |
| softmax_256x64_f64/ndarray | 97.274 / 99.274 | 97.549 / 97.599 | 1 | 131072 | 3.02 |
| softmax_256x64_f64/numpy | 134.696 / 143.256 | 135.922 / 136.611 | unavailable | unavailable | 34.62 |
| solve_cholesky_128x1_f64/faer | 5.887 / 6.256 | 5.997 / 6.023 | 1 | 1024 | 3.63 |
| solve_cholesky_128x1_f64/kernix | 12.719 / 12.895 | 12.752 / 12.792 | 1 | 1024 | 3.14 |
| solve_cholesky_128x1_f64/nalgebra | 5.256 / 5.280 | 5.265 / 5.318 | 1 | 1024 | 3.07 |
| solve_cholesky_256x64_f64/faer | 452.655 / 463.407 | 454.255 / 454.762 | 53 | 327688 | 5.06 |
| solve_cholesky_256x64_f64/kernix | 1719.759 / 1725.799 | 1719.220 / 1719.988 | 1 | 131072 | 4.40 |
| solve_cholesky_256x64_f64/nalgebra | 1286.693 / 1297.652 | 1281.903 / 1290.161 | 1 | 131072 | 4.34 |
| solve_cholesky_512x128_f64/faer | 3193.317 / 3220.915 | 3189.851 / 3204.613 | 205 | 786440 | 9.59 |
| solve_cholesky_512x128_f64/kernix | 13394.223 / 13437.411 | 13375.329 / 13431.572 | 1 | 524288 | 8.52 |
| solve_cholesky_512x128_f64/nalgebra | 12850.165 / 12976.845 | 12843.628 / 12999.787 | 1 | 524288 | 8.83 |
| solve_lu_256x64_f64/faer | 457.732 / 471.891 | 460.378 / 465.562 | 46 | 327688 | 6.20 |
| solve_lu_256x64_f64/kernix | 1654.199 / 1663.303 | 1656.972 / 1660.235 | 2 | 262144 | 4.28 |
| solve_lu_256x64_f64/nalgebra | 1234.129 / 1248.288 | 1234.898 / 1239.085 | 1 | 131072 | 4.35 |
| sparse_auto_512_f64/kernix | 489.060 / 498.659 | 488.055 / 489.607 | 4 | 2142216 | 9.04 |
| sparse_auto_512_f64/kernix-dense | 53.397 / 69.404 | 54.580 / 54.817 | 1 | 2097152 | 14.78 |
| sparse_auto_512_f64/numpy | 55.813 / 70.708 | 57.232 / 57.576 | unavailable | unavailable | 60.73 |
| sparse_auto_512_f64/scipy | 752.028 / 774.938 | 753.843 / 760.889 | unavailable | unavailable | 63.05 |
| sparse_matmul_512x32_f32/faer | 64.019 / 66.052 | 64.140 / 65.966 | 1 | 65536 | 4.15 |
| sparse_matmul_512x32_f32/kernix | 21.079 / 21.239 | 21.175 / 21.291 | 1 | 65536 | 3.53 |
| sparse_matmul_512x32_f32/kernix-dense | 380.793 / 396.280 | 384.324 / 385.154 | 3 | 163840 | 5.33 |
| sparse_matmul_512x32_f32/numpy | 512.470 / 525.781 | 514.641 / 519.029 | unavailable | unavailable | 52.35 |
| sparse_matmul_512x32_f32/scipy | 15.343 / 15.591 | 15.391 / 15.552 | unavailable | unavailable | 51.68 |
| sparse_matmul_512x32_f64/faer | 68.052 / 70.580 | 68.266 / 68.473 | 1 | 131072 | 5.31 |
| sparse_matmul_512x32_f64/kernix | 30.454 / 31.471 | 30.494 / 30.638 | 1 | 131072 | 4.65 |
| sparse_matmul_512x32_f64/kernix-dense | 783.657 / 795.152 | 781.077 / 784.740 | 3 | 327680 | 7.66 |
| sparse_matmul_512x32_f64/numpy | 1064.764 / 1074.283 | 1066.614 / 1072.052 | unavailable | unavailable | 53.41 |
| sparse_matmul_512x32_f64/scipy | 23.047 / 23.782 | 23.247 / 23.314 | unavailable | unavailable | 52.72 |
| sparse_matvec_2048_f64/faer | 8.016 / 8.080 | 8.034 / 8.059 | 1 | 16384 | 11.52 |
| sparse_matvec_2048_f64/kernix | 18.142 / 18.287 | 18.210 / 18.255 | 1 | 16384 | 10.91 |
| sparse_matvec_2048_f64/kernix-dense | 2914.433 / 2942.582 | 2914.887 / 2932.139 | 3 | 155648 | 43.52 |
| sparse_matvec_2048_f64/numpy | 805.888 / 817.448 | 803.004 / 825.833 | unavailable | unavailable | 82.61 |
| sparse_matvec_2048_f64/scipy | 9.760 / 10.047 | 9.897 / 9.941 | unavailable | unavailable | 81.26 |
| sparse_product_256_f64/kernix | 64.339 / 66.260 | 64.519 / 64.627 | 4 | 67752 | 3.63 |
| sparse_product_256_f64/kernix-dense | 1324.545 / 1335.632 | 1325.084 / 1327.855 | 4 | 1048584 | 6.39 |
| sparse_product_256_f64/numpy | 1475.794 / 1493.632 | 1476.634 / 1480.508 | unavailable | unavailable | 53.91 |
| sparse_product_256_f64/scipy | 47.037 / 57.572 | 47.346 / 47.977 | unavailable | unavailable | 52.59 |
| tensordot_16x32x8_32x8x16_f64/kernix | 46.718 / 48.469 | 46.853 / 47.765 | 9 | 2166 | 2.77 |
| tensordot_16x32x8_32x8x16_f64/numpy | 16.415 / 16.878 | 16.645 / 16.691 | unavailable | unavailable | 34.53 |
| training_backward_2x4x8x4_f64/candle | 21.198 / 21.631 | 21.171 / 21.282 | 476 | 8996 | 4.16 |
| training_backward_2x4x8x4_f64/kernix | 4.200 / 4.360 | 4.249 / 4.328 | 23 | 4224 | 3.16 |
| training_forward_2x4x8x4_f64/candle | 5.335 / 5.407 | 5.370 / 5.416 | 126 | 3912 | 4.06 |
| training_forward_2x4x8x4_f64/kernix | 3.031 / 3.096 | 3.117 / 3.132 | 17 | 1032 | 3.14 |
| training_record_2x4x8x4_f64/candle | 7.600 / 7.776 | 7.501 / 7.642 | 149 | 10712 | 4.07 |
| training_record_2x4x8x4_f64/kernix | 3.856 / 3.920 | 3.857 / 3.882 | 22 | 6832 | 3.08 |
| training_sgd_2x4x8x4_f64/kernix | 0.728 / 0.760 | 0.740 / 0.743 | 5 | 688 | 3.23 |
| training_shared_backward_64x32_f64/candle | 4.983 / 5.064 | 4.976 / 5.033 | 58 | 99996 | 3.60 |
| training_shared_backward_64x32_f64/kernix | 8.368 / 8.440 | 8.390 / 8.441 | 8 | 49768 | 3.12 |

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
