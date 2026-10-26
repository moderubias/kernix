# Clean CPU evidence: runner-ubuntu-latest-40270611933

Measured source `90cbbff60e5d7458ef9f89918c99a6cca94b51d1`, tree `d806b9704a44b4bf2c4c04e664fe3c60eeebc177`; dirty before/after: false.
Library source: workspace 90cbbff60e5d7458ef9f89918c99a6cca94b51d1. CPU affinity: 0. Dtype explicitly named per workload; row-major inputs.

| Workload / library | Per-call p50 / p95 (us) | Batch-mean p50 / p95 (us) | Rust allocations | Peak additional requested bytes | Process peak RSS (MiB) |
|---|---:|---:|---:|---:|---:|
| cholesky_factor_128_f64/faer | 27.211 / 28.222 | 26.878 / 27.337 | 2 | 132096 | 4.51 |
| cholesky_factor_128_f64/kernix | 97.536 / 103.496 | 97.787 / 98.257 | 1 | 131072 | 4.16 |
| cholesky_factor_128_f64/nalgebra | 84.276 / 91.197 | 84.224 / 85.050 | 1 | 131072 | 3.73 |
| cholesky_factor_128_f64/numpy | 88.853 / 96.415 | 89.622 / 92.515 | unavailable | unavailable | 47.18 |
| cholesky_factor_256_f64/faer | 144.758 / 152.679 | 142.720 / 145.578 | 2 | 526336 | 5.34 |
| cholesky_factor_256_f64/kernix | 426.441 / 437.688 | 428.281 / 437.280 | 49 | 645120 | 5.41 |
| cholesky_factor_256_f64/nalgebra | 562.625 / 575.875 | 561.266 / 572.312 | 1 | 524288 | 4.59 |
| cholesky_factor_256_f64/numpy | 831.750 / 848.575 | 827.859 / 844.541 | unavailable | unavailable | 50.71 |
| cholesky_factor_512_f64/faer | 901.274 / 921.524 | 918.066 / 925.344 | 2 | 2101248 | 8.97 |
| cholesky_factor_512_f64/kernix | 2326.847 / 2384.303 | 2339.647 / 2393.380 | 161 | 2269184 | 9.72 |
| cholesky_factor_512_f64/nalgebra | 4206.351 / 4291.769 | 4190.477 / 4428.938 | 1 | 2097152 | 7.55 |
| cholesky_factor_512_f64/numpy | 4302.975 / 4374.282 | 4272.452 / 4349.562 | unavailable | unavailable | 63.29 |
| cholesky_factor_64_f64/faer | 6.920 / 6.951 | 6.902 / 7.083 | 2 | 33280 | 4.07 |
| cholesky_factor_64_f64/kernix | 16.114 / 17.036 | 16.284 / 16.392 | 2 | 33280 | 3.54 |
| cholesky_factor_64_f64/nalgebra | 14.312 / 14.392 | 14.407 / 14.530 | 1 | 32768 | 3.47 |
| cholesky_factor_64_f64/numpy | 10.436 / 10.556 | 10.523 / 10.717 | unavailable | unavailable | 46.28 |
| cholesky_rank_downdate_256_f64/kernix | 90.446 / 101.904 | 91.200 / 92.452 | 3 | 526336 | 5.46 |
| cholesky_rank_downdate_256_f64/nalgebra | 17.066 / 17.416 | 17.425 / 17.668 | 2 | 526336 | 6.22 |
| cholesky_rank_downdate_256_f64/refactor | 436.576 / 447.012 | 435.859 / 597.412 | 49 | 645120 | 6.25 |
| cholesky_rank_update_256_f64/kernix | 92.739 / 104.847 | 93.564 / 94.840 | 3 | 526336 | 5.48 |
| cholesky_rank_update_256_f64/nalgebra | 18.007 / 18.548 | 18.254 / 18.513 | 2 | 526336 | 6.15 |
| cholesky_rank_update_256_f64/refactor | 435.535 / 446.440 | 433.445 / 437.549 | 49 | 645120 | 6.29 |
| cholesky_rank_update_512_f64/kernix | 399.350 / 415.985 | 402.182 / 418.612 | 3 | 2101248 | 10.06 |
| cholesky_rank_update_512_f64/nalgebra | 85.518 / 92.799 | 86.347 / 87.258 | 2 | 2101248 | 13.73 |
| cholesky_rank_update_512_f64/refactor | 2321.798 / 2372.695 | 2300.941 / 2400.669 | 161 | 2269184 | 12.39 |
| create_ones_64x64_f32/candle | 0.241 / 0.251 | 0.252 / 0.254 | 7 | 16696 | 3.80 |
| create_ones_64x64_f32/faer | 0.180 / 0.190 | 0.168 / 0.171 | 1 | 16384 | 3.80 |
| create_ones_64x64_f32/kernix | 0.220 / 0.221 | 0.222 / 0.224 | 1 | 16384 | 3.59 |
| create_ones_64x64_f32/ndarray | 0.150 / 0.160 | 0.153 / 0.155 | 1 | 16384 | 3.63 |
| create_ones_64x64_f32/numpy | 1.252 / 1.282 | 1.321 / 1.357 | unavailable | unavailable | 41.70 |
| create_zeros_64x64_f32/candle | 0.151 / 0.161 | 0.156 / 0.166 | 5 | 16648 | 4.25 |
| create_zeros_64x64_f32/faer | 0.140 / 0.150 | 0.134 / 0.219 | 1 | 16384 | 3.87 |
| create_zeros_64x64_f32/kernix | 0.150 / 0.160 | 0.155 / 0.158 | 1 | 16384 | 3.70 |
| create_zeros_64x64_f32/ndarray | 0.090 / 0.091 | 0.093 / 0.094 | 1 | 16384 | 3.58 |
| create_zeros_64x64_f32/numpy | 0.321 / 0.361 | 0.363 / 0.371 | unavailable | unavailable | 41.71 |
| matmul_128_f32/candle | 33.841 / 37.145 | 33.869 / 34.098 | 9 | 65816 | 5.12 |
| matmul_128_f32/faer | 19.660 / 19.800 | 19.525 / 19.842 | 1 | 65536 | 4.30 |
| matmul_128_f32/kernix | 34.182 / 34.362 | 34.005 / 34.412 | 3 | 65544 | 4.09 |
| matmul_128_f32/nalgebra | 40.181 / 48.052 | 41.696 / 42.282 | 2 | 163840 | 3.89 |
| matmul_128_f32/ndarray | 40.241 / 47.932 | 42.121 / 43.181 | 2 | 163840 | 4.03 |
| matmul_128_f32/numpy | 20.420 / 20.511 | 20.850 / 20.936 | unavailable | unavailable | 45.32 |
| matmul_128_f64/candle | 67.671 / 73.400 | 67.954 / 68.651 | 9 | 131352 | 5.26 |
| matmul_128_f64/faer | 38.879 / 54.141 | 40.723 / 66.729 | 1 | 131072 | 4.40 |
| matmul_128_f64/kernix | 67.381 / 72.990 | 67.433 / 68.278 | 3 | 131080 | 4.31 |
| matmul_128_f64/nalgebra | 117.857 / 123.947 | 116.393 / 117.170 | 2 | 327680 | 4.01 |
| matmul_128_f64/ndarray | 118.989 / 126.020 | 117.823 / 119.257 | 2 | 327680 | 4.09 |
| matmul_128_f64/numpy | 48.092 / 53.821 | 48.598 / 48.937 | unavailable | unavailable | 45.42 |
| matmul_256_f32/candle | 262.364 / 270.536 | 265.669 / 361.381 | 9 | 262424 | 5.88 |
| matmul_256_f32/faer | 146.280 / 153.030 | 148.659 / 154.276 | 1 | 262144 | 4.86 |
| matmul_256_f32/kernix | 262.855 / 270.617 | 265.108 / 267.619 | 3 | 262152 | 5.04 |
| matmul_256_f32/nalgebra | 325.859 / 336.946 | 324.470 / 326.908 | 2 | 589824 | 4.37 |
| matmul_256_f32/ndarray | 335.454 / 345.699 | 332.760 / 334.160 | 2 | 589824 | 4.60 |
| matmul_256_f32/numpy | 148.143 / 338.910 | 144.788 / 220.036 | unavailable | unavailable | 48.99 |
| matmul_256_f64/candle | 545.750 / 552.200 | 547.697 / 561.568 | 9 | 524568 | 6.61 |
| matmul_256_f64/faer | 336.866 / 345.940 | 340.328 / 343.488 | 1 | 524288 | 5.61 |
| matmul_256_f64/kernix | 535.845 / 545.009 | 542.970 / 553.183 | 3 | 524296 | 5.80 |
| matmul_256_f64/nalgebra | 333.922 / 343.506 | 336.742 / 343.998 | 2 | 1179648 | 5.28 |
| matmul_256_f64/ndarray | 328.163 / 361.923 | 327.680 / 328.544 | 2 | 1179648 | 5.82 |
| matmul_256_f64/numpy | 355.204 / 365.359 | 357.044 / 362.843 | unavailable | unavailable | 49.67 |
| matmul_32_f64/candle | 1.402 / 1.432 | 1.419 / 1.433 | 9 | 8528 | 4.86 |
| matmul_32_f64/faer | 0.751 / 0.771 | 0.745 / 0.759 | 1 | 8192 | 4.28 |
| matmul_32_f64/kernix | 1.121 / 1.141 | 1.134 / 1.167 | 3 | 24576 | 3.95 |
| matmul_32_f64/nalgebra | 0.982 / 0.992 | 1.009 / 1.018 | 2 | 24576 | 3.69 |
| matmul_32_f64/ndarray | 0.961 / 0.972 | 0.958 / 0.965 | 2 | 24576 | 3.75 |
| matmul_32_f64/numpy | 1.382 / 1.402 | 1.447 / 1.477 | unavailable | unavailable | 42.91 |
| matmul_64_f32/candle | 4.387 / 4.416 | 4.472 / 4.505 | 9 | 16664 | 5.01 |
| matmul_64_f32/faer | 2.333 / 2.344 | 2.318 / 2.331 | 1 | 16384 | 4.18 |
| matmul_64_f32/kernix | 4.306 / 4.327 | 4.436 / 5.496 | 3 | 16512 | 4.36 |
| matmul_64_f32/nalgebra | 3.495 / 3.516 | 3.489 / 3.591 | 2 | 49152 | 3.52 |
| matmul_64_f32/ndarray | 3.425 / 3.436 | 3.414 / 3.431 | 2 | 49152 | 3.78 |
| matmul_64_f32/numpy | 2.894 / 2.915 | 2.960 / 2.975 | unavailable | unavailable | 43.23 |
| matmul_64_f64/candle | 8.823 / 8.863 | 8.864 / 8.929 | 9 | 33048 | 4.25 |
| matmul_64_f64/faer | 4.737 / 4.787 | 4.770 / 4.829 | 1 | 32768 | 4.25 |
| matmul_64_f64/kernix | 8.963 / 8.993 | 8.953 / 9.057 | 3 | 32896 | 4.59 |
| matmul_64_f64/nalgebra | 6.329 / 6.380 | 6.301 / 6.434 | 2 | 98304 | 3.78 |
| matmul_64_f64/ndarray | 6.189 / 6.210 | 6.252 / 6.419 | 2 | 98304 | 3.78 |
| matmul_64_f64/numpy | 4.997 / 5.028 | 5.003 / 5.072 | unavailable | unavailable | 43.21 |
| matmul_ab_128_f32/kernix | 33.009 / 33.130 | 33.405 / 35.397 | 3 | 65544 | 4.64 |
| matmul_ab_128_f32/numpy | 20.912 / 21.042 | 21.018 / 21.250 | unavailable | unavailable | 45.03 |
| matmul_ab_128_f64/kernix | 67.341 / 73.190 | 67.517 / 68.460 | 3 | 131080 | 4.77 |
| matmul_ab_128_f64/numpy | 47.972 / 54.202 | 48.665 / 48.905 | unavailable | unavailable | 45.26 |
| matmul_batched_4x16x32x16_f32/candle | 1.332 / 1.372 | 1.349 / 1.358 | 25 | 13152 | 5.13 |
| matmul_batched_4x16x32x16_f32/kernix | 0.752 / 0.771 | 0.761 / 0.787 | 3 | 4104 | 4.07 |
| matmul_batched_4x16x32x16_f32/numpy | 1.442 / 1.472 | 1.505 / 1.524 | unavailable | unavailable | 42.96 |
| matmul_batched_4x16x32x16_f64/candle | 1.953 / 1.983 | 1.946 / 1.995 | 25 | 25440 | 5.00 |
| matmul_batched_4x16x32x16_f64/kernix | 1.302 / 1.332 | 1.346 / 1.369 | 3 | 8200 | 3.99 |
| matmul_batched_4x16x32x16_f64/numpy | 1.602 / 1.632 | 1.681 / 1.703 | unavailable | unavailable | 42.96 |
| matmul_sum_backward_32x16x8_f64/candle | 3.836 / 3.926 | 3.790 / 3.947 | 113 | 27988 | 5.16 |
| matmul_sum_backward_32x16x8_f64/kernix | 2.133 / 2.163 | 2.143 / 2.181 | 15 | 18376 | 3.96 |
| matmul_transposed_128_f32/kernix | 37.456 / 44.707 | 39.883 / 40.295 | 3 | 163840 | 4.44 |
| matmul_transposed_128_f32/numpy | 21.182 / 21.272 | 21.244 / 21.363 | unavailable | unavailable | 45.09 |
| matmul_transposed_128_f64/kernix | 119.139 / 126.681 | 117.378 / 119.476 | 3 | 327680 | 4.69 |
| matmul_transposed_128_f64/numpy | 47.982 / 53.941 | 48.408 / 49.389 | unavailable | unavailable | 45.24 |
| min_norm_32x64_rank32_f64/kernix | 47.371 / 52.969 | 47.918 / 49.218 | 20 | 59136 | 4.40 |
| min_norm_32x64_rank32_f64/nalgebra | 125.458 / 131.738 | 125.418 / 127.041 | 15 | 42232 | 4.03 |
| min_norm_32x64_rank32_f64/numpy | 115.984 / 123.084 | 116.717 / 117.696 | unavailable | unavailable | 48.29 |
| min_norm_64x32_rank16_f64/kernix | 35.784 / 36.795 | 36.390 / 79.701 | 20 | 74752 | 4.41 |
| min_norm_64x32_rank16_f64/nalgebra | 68.203 / 74.031 | 68.931 / 70.084 | 15 | 41976 | 4.07 |
| min_norm_64x32_rank16_f64/numpy | 61.152 / 67.051 | 61.994 / 64.302 | unavailable | unavailable | 47.39 |
| parallel_matmul_into_128_f32/kernix | 24.807 / 25.078 | 24.352 / 24.679 | 2 | 16 | 4.77 |
| parallel_matmul_into_128_f64/kernix | 43.585 / 47.402 | 43.383 / 64.453 | 2 | 16 | 4.97 |
| parallel_matmul_into_256_f64/kernix | 283.426 / 297.958 | 284.259 / 287.431 | 2 | 16 | 6.66 |
| parallel_matmul_into_64_f64/kernix | 9.344 / 9.494 | 10.101 / 10.368 | 2 | 98304 | 3.96 |
| pivoted_qr_256x64_f64/faer | 217.167 / 227.492 | 215.352 / 222.281 | 10 | 333312 | 5.17 |
| pivoted_qr_256x64_f64/kernix | 524.989 / 538.469 | 524.170 / 551.050 | 14 | 559104 | 4.77 |
| pivoted_qr_256x64_f64/nalgebra | 599.630 / 611.279 | 594.939 / 600.005 | 7 | 296960 | 4.32 |
| pivoted_qr_96x32_f64/faer | 23.425 / 23.716 | 23.785 / 24.112 | 10 | 68352 | 4.86 |
| pivoted_qr_96x32_f64/kernix | 46.029 / 50.946 | 47.004 / 47.415 | 14 | 107520 | 4.14 |
| pivoted_qr_96x32_f64/nalgebra | 60.451 / 66.420 | 61.062 / 62.166 | 7 | 58368 | 3.91 |
| qr_1024x256_f64/faer | 6576.743 / 6778.696 | 6532.100 / 6669.703 | 7 | 5308416 | 12.03 |
| qr_1024x256_f64/kernix | 13868.677 / 14131.442 | 13759.383 / 14011.401 | 82 | 7145480 | 17.30 |
| qr_1024x256_f64/nalgebra | 21889.282 / 22581.322 | 22083.304 / 22545.538 | 4 | 4720640 | 10.09 |
| qr_1024x256_f64/numpy | 11171.564 / 11616.973 | 11210.202 / 11419.467 | unavailable | unavailable | 63.02 |
| qr_256x64_f64/faer | 233.611 / 249.284 | 234.771 / 289.123 | 7 | 331776 | 5.18 |
| qr_256x64_f64/kernix | 488.284 / 501.123 | 493.067 / 497.655 | 6 | 426496 | 4.42 |
| qr_256x64_f64/nalgebra | 350.937 / 360.562 | 347.710 / 351.316 | 4 | 295424 | 4.11 |
| qr_256x64_f64/numpy | 314.573 / 327.863 | 311.713 / 319.127 | unavailable | unavailable | 49.45 |
| qr_512x128_f64/faer | 1210.359 / 1224.179 | 1206.943 / 1580.973 | 7 | 1343488 | 6.61 |
| qr_512x128_f64/kernix | 2381.458 / 2435.389 | 2393.054 / 2421.024 | 49 | 2294792 | 7.69 |
| qr_512x128_f64/nalgebra | 2880.959 / 3007.909 | 2910.425 / 2989.689 | 4 | 1180672 | 4.90 |
| qr_512x128_f64/numpy | 2084.652 / 2158.413 | 2150.323 / 2177.810 | unavailable | unavailable | 53.12 |
| qr_96x32_f64/faer | 28.523 / 28.723 | 28.775 / 29.735 | 7 | 67584 | 4.88 |
| qr_96x32_f64/kernix | 40.851 / 41.643 | 40.807 / 41.369 | 6 | 82176 | 3.67 |
| qr_96x32_f64/nalgebra | 33.710 / 36.034 | 33.877 / 46.641 | 4 | 57600 | 3.76 |
| qr_96x32_f64/numpy | 34.822 / 35.503 | 35.017 / 35.270 | unavailable | unavailable | 48.36 |
| softmax_256x64_f64/candle | 44.346 / 45.609 | 44.710 / 44.999 | 37 | 265200 | 5.99 |
| softmax_256x64_f64/kernix | 68.864 / 74.432 | 69.323 / 71.051 | 1 | 131072 | 4.12 |
| softmax_256x64_f64/ndarray | 42.865 / 44.247 | 43.864 / 44.460 | 1 | 131072 | 4.05 |
| softmax_256x64_f64/numpy | 44.747 / 48.293 | 43.767 / 45.306 | unavailable | unavailable | 43.21 |
| solve_cholesky_128x1_f64/faer | 3.565 / 3.615 | 3.595 / 3.619 | 1 | 1024 | 4.89 |
| solve_cholesky_128x1_f64/kernix | 8.923 / 9.625 | 9.320 / 9.491 | 1 | 1024 | 4.07 |
| solve_cholesky_128x1_f64/nalgebra | 3.645 / 3.656 | 3.687 / 3.699 | 1 | 1024 | 3.99 |
| solve_cholesky_256x64_f64/faer | 122.384 / 129.665 | 122.359 / 126.993 | 1 | 131072 | 6.45 |
| solve_cholesky_256x64_f64/kernix | 853.993 / 872.060 | 845.639 / 849.233 | 1 | 131072 | 5.41 |
| solve_cholesky_256x64_f64/nalgebra | 725.109 / 738.480 | 725.061 / 729.944 | 1 | 131072 | 5.29 |
| solve_cholesky_512x128_f64/faer | 1113.754 / 1218.561 | 1341.759 / 1754.441 | 1 | 524288 | 10.95 |
| solve_cholesky_512x128_f64/kernix | 6608.068 / 7429.072 | 6621.543 / 6799.621 | 1 | 524288 | 9.69 |
| solve_cholesky_512x128_f64/nalgebra | 5987.768 / 6069.610 | 6022.980 / 6065.855 | 1 | 524288 | 9.86 |
| solve_lu_256x64_f64/faer | 187.782 / 199.299 | 187.177 / 192.897 | 2 | 262144 | 6.43 |
| solve_lu_256x64_f64/kernix | 842.977 / 866.633 | 845.247 / 858.962 | 2 | 262144 | 5.43 |
| solve_lu_256x64_f64/nalgebra | 675.134 / 689.656 | 669.330 / 677.283 | 1 | 131072 | 5.27 |
| tensordot_16x32x8_32x8x16_f64/kernix | 30.997 / 31.157 | 31.458 / 31.697 | 9 | 2166 | 3.94 |
| tensordot_16x32x8_32x8x16_f64/numpy | 4.888 / 4.977 | 4.990 / 5.106 | unavailable | unavailable | 42.94 |
| training_backward_2x4x8x4_f64/candle | 12.739 / 13.010 | 12.998 / 13.220 | 472 | 8332 | 5.82 |
| training_backward_2x4x8x4_f64/kernix | 3.095 / 3.155 | 3.096 / 3.135 | 23 | 4480 | 4.24 |
| training_forward_2x4x8x4_f64/candle | 2.924 / 2.995 | 2.955 / 3.008 | 126 | 3912 | 5.94 |
| training_forward_2x4x8x4_f64/kernix | 1.933 / 2.463 | 1.945 / 1.969 | 17 | 1288 | 4.30 |
| training_record_2x4x8x4_f64/candle | 4.277 / 4.467 | 4.317 / 4.358 | 149 | 10712 | 5.91 |
| training_record_2x4x8x4_f64/kernix | 2.443 / 2.494 | 2.446 / 2.485 | 22 | 6832 | 3.91 |
| training_sgd_2x4x8x4_f64/kernix | 0.541 / 0.561 | 0.549 / 0.568 | 5 | 688 | 4.40 |
| training_shared_backward_64x32_f64/candle | 3.105 / 3.205 | 3.143 / 3.178 | 58 | 100004 | 5.10 |
| training_shared_backward_64x32_f64/kernix | 5.929 / 5.959 | 5.881 / 5.964 | 8 | 49768 | 4.14 |

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
