# Clean CPU evidence: baseline-ubuntu-latest-40270611933

Measured source `90cbbff60e5d7458ef9f89918c99a6cca94b51d1`, tree `d806b9704a44b4bf2c4c04e664fe3c60eeebc177`; dirty before/after: false.
Library source: registry 0.11.1. CPU affinity: 0. Dtype explicitly named per workload; row-major inputs.

| Workload / library | Per-call p50 / p95 (us) | Batch-mean p50 / p95 (us) | Rust allocations | Peak additional requested bytes | Process peak RSS (MiB) |
|---|---:|---:|---:|---:|---:|
| cholesky_factor_128_f64/faer | 27.792 / 28.172 | 28.015 / 28.261 | 2 | 132096 | 4.78 |
| cholesky_factor_128_f64/kernix | 97.277 / 103.065 | 98.059 / 98.539 | 1 | 131072 | 4.15 |
| cholesky_factor_128_f64/nalgebra | 85.699 / 91.608 | 86.084 / 87.445 | 1 | 131072 | 3.82 |
| cholesky_factor_128_f64/numpy | 88.803 / 96.265 | 91.083 / 92.523 | unavailable | unavailable | 47.24 |
| cholesky_factor_256_f64/faer | 142.725 / 152.660 | 144.801 / 146.832 | 2 | 526336 | 5.53 |
| cholesky_factor_256_f64/kernix | 433.551 / 446.741 | 434.030 / 437.475 | 49 | 645120 | 5.25 |
| cholesky_factor_256_f64/nalgebra | 557.468 / 569.466 | 556.056 / 560.870 | 1 | 524288 | 4.52 |
| cholesky_factor_256_f64/numpy | 821.845 / 834.343 | 824.403 / 827.939 | unavailable | unavailable | 50.71 |
| cholesky_factor_512_f64/faer | 901.335 / 916.157 | 903.143 / 921.510 | 2 | 2101248 | 8.85 |
| cholesky_factor_512_f64/kernix | 2333.015 / 2381.018 | 2336.344 / 2416.609 | 161 | 2269184 | 10.20 |
| cholesky_factor_512_f64/nalgebra | 4188.855 / 4956.288 | 4203.804 / 4300.997 | 1 | 2097152 | 7.52 |
| cholesky_factor_512_f64/numpy | 4389.956 / 4503.697 | 4464.411 / 5091.981 | unavailable | unavailable | 63.14 |
| cholesky_factor_64_f64/faer | 7.111 / 7.161 | 7.207 / 7.300 | 2 | 33280 | 4.07 |
| cholesky_factor_64_f64/kernix | 16.104 / 16.234 | 16.243 / 16.390 | 2 | 33280 | 3.96 |
| cholesky_factor_64_f64/nalgebra | 15.133 / 15.183 | 15.291 / 15.792 | 1 | 32768 | 3.54 |
| cholesky_factor_64_f64/numpy | 10.757 / 10.917 | 10.776 / 10.868 | unavailable | unavailable | 46.33 |
| cholesky_rank_downdate_256_f64/nalgebra | 17.726 / 18.017 | 18.048 / 18.169 | 2 | 526336 | 6.12 |
| cholesky_rank_downdate_256_f64/refactor | 436.506 / 448.744 | 441.167 / 706.491 | 49 | 645120 | 6.21 |
| cholesky_rank_update_256_f64/nalgebra | 17.927 / 18.358 | 17.976 / 18.238 | 2 | 526336 | 6.16 |
| cholesky_rank_update_256_f64/refactor | 429.575 / 437.137 | 432.978 / 436.761 | 49 | 645120 | 6.27 |
| cholesky_rank_update_512_f64/nalgebra | 95.133 / 292.380 | 89.086 / 144.222 | 2 | 2101248 | 13.75 |
| cholesky_rank_update_512_f64/refactor | 2325.754 / 2356.831 | 2324.773 / 2336.503 | 161 | 2269184 | 12.25 |
| create_ones_64x64_f32/candle | 0.250 / 0.251 | 0.249 / 0.256 | 7 | 16696 | 4.38 |
| create_ones_64x64_f32/faer | 0.180 / 0.181 | 0.171 / 0.233 | 1 | 16384 | 3.95 |
| create_ones_64x64_f32/kernix | 0.230 / 0.231 | 0.231 / 0.233 | 1 | 16384 | 3.73 |
| create_ones_64x64_f32/ndarray | 0.150 / 0.170 | 0.156 / 0.157 | 1 | 16384 | 3.58 |
| create_ones_64x64_f32/numpy | 1.242 / 1.272 | 1.305 / 1.340 | unavailable | unavailable | 41.73 |
| create_zeros_64x64_f32/candle | 0.160 / 0.161 | 0.157 / 0.167 | 5 | 16648 | 4.18 |
| create_zeros_64x64_f32/faer | 0.140 / 0.150 | 0.132 / 0.133 | 1 | 16384 | 3.93 |
| create_zeros_64x64_f32/kernix | 0.151 / 0.161 | 0.160 / 0.177 | 1 | 16384 | 3.84 |
| create_zeros_64x64_f32/ndarray | 0.090 / 0.101 | 0.095 / 0.097 | 1 | 16384 | 3.56 |
| create_zeros_64x64_f32/numpy | 0.340 / 0.381 | 0.393 / 0.403 | unavailable | unavailable | 41.70 |
| matmul_128_f32/candle | 34.232 / 35.072 | 34.323 / 34.699 | 9 | 65816 | 5.09 |
| matmul_128_f32/faer | 20.030 / 20.130 | 20.127 / 20.273 | 1 | 65536 | 4.54 |
| matmul_128_f32/kernix | 34.331 / 35.233 | 34.525 / 34.746 | 3 | 65544 | 4.57 |
| matmul_128_f32/nalgebra | 40.240 / 48.282 | 41.957 / 42.503 | 2 | 163840 | 3.94 |
| matmul_128_f32/ndarray | 39.760 / 49.234 | 41.757 / 42.025 | 2 | 163840 | 4.10 |
| matmul_128_f32/numpy | 21.082 / 21.182 | 21.257 / 22.052 | unavailable | unavailable | 45.21 |
| matmul_128_f64/candle | 68.863 / 74.592 | 69.465 / 70.565 | 9 | 131352 | 5.21 |
| matmul_128_f64/faer | 38.478 / 38.899 | 38.680 / 40.050 | 1 | 131072 | 4.43 |
| matmul_128_f64/kernix | 68.873 / 74.923 | 69.035 / 70.305 | 3 | 131080 | 4.80 |
| matmul_128_f64/nalgebra | 120.280 / 128.833 | 118.646 / 119.073 | 2 | 327680 | 4.03 |
| matmul_128_f64/ndarray | 121.813 / 133.561 | 121.183 / 122.666 | 2 | 327680 | 4.09 |
| matmul_128_f64/numpy | 48.262 / 54.121 | 48.771 / 49.240 | unavailable | unavailable | 45.45 |
| matmul_256_f32/candle | 263.946 / 394.343 | 265.360 / 273.039 | 9 | 262424 | 5.62 |
| matmul_256_f32/faer | 150.466 / 157.778 | 152.410 / 153.508 | 1 | 262144 | 5.02 |
| matmul_256_f32/kernix | 266.700 / 277.497 | 268.247 / 269.081 | 3 | 262152 | 5.11 |
| matmul_256_f32/nalgebra | 329.905 / 343.517 | 330.413 / 338.216 | 2 | 589824 | 4.35 |
| matmul_256_f32/ndarray | 325.860 / 342.775 | 324.958 / 327.098 | 2 | 589824 | 4.65 |
| matmul_256_f32/numpy | 145.118 / 152.930 | 146.379 / 151.990 | unavailable | unavailable | 48.99 |
| matmul_256_f64/candle | 544.648 / 555.525 | 543.139 / 548.619 | 9 | 524568 | 6.75 |
| matmul_256_f64/faer | 338.719 / 347.802 | 338.847 / 348.670 | 1 | 524288 | 5.73 |
| matmul_256_f64/kernix | 540.022 / 550.447 | 538.959 / 552.555 | 3 | 524296 | 5.91 |
| matmul_256_f64/nalgebra | 330.727 / 466.712 | 333.492 / 335.316 | 2 | 1179648 | 5.37 |
| matmul_256_f64/ndarray | 346.461 / 360.181 | 343.851 / 346.356 | 2 | 1179648 | 5.90 |
| matmul_256_f64/numpy | 358.018 / 368.453 | 361.596 / 363.534 | unavailable | unavailable | 49.77 |
| matmul_32_f64/candle | 1.392 / 1.413 | 1.418 / 1.610 | 9 | 8528 | 4.87 |
| matmul_32_f64/faer | 0.772 / 0.782 | 0.752 / 0.761 | 1 | 8192 | 4.40 |
| matmul_32_f64/kernix | 1.112 / 1.132 | 1.145 / 1.166 | 3 | 24576 | 4.09 |
| matmul_32_f64/nalgebra | 0.991 / 1.002 | 1.001 / 1.011 | 2 | 24576 | 3.68 |
| matmul_32_f64/ndarray | 0.982 / 1.011 | 0.989 / 0.993 | 2 | 24576 | 3.70 |
| matmul_32_f64/numpy | 1.412 / 1.432 | 1.485 / 1.494 | unavailable | unavailable | 42.95 |
| matmul_64_f32/candle | 4.537 / 4.567 | 4.599 / 4.611 | 9 | 16664 | 4.71 |
| matmul_64_f32/faer | 2.334 / 2.373 | 2.382 / 2.445 | 1 | 16384 | 4.25 |
| matmul_64_f32/kernix | 4.447 / 4.467 | 4.504 / 4.529 | 3 | 16512 | 4.36 |
| matmul_64_f32/nalgebra | 3.405 / 3.425 | 3.444 / 3.456 | 2 | 49152 | 3.68 |
| matmul_64_f32/ndarray | 3.435 / 3.456 | 3.458 / 3.486 | 2 | 49152 | 3.88 |
| matmul_64_f32/numpy | 2.975 / 3.024 | 3.029 / 3.068 | unavailable | unavailable | 43.16 |
| matmul_64_f64/candle | 9.044 / 9.083 | 9.131 / 9.173 | 9 | 33048 | 4.86 |
| matmul_64_f64/faer | 4.857 / 4.908 | 4.919 / 4.964 | 1 | 32768 | 4.30 |
| matmul_64_f64/kernix | 9.004 / 9.044 | 9.073 / 9.356 | 3 | 32896 | 4.55 |
| matmul_64_f64/nalgebra | 6.370 / 6.921 | 6.842 / 9.812 | 2 | 98304 | 3.76 |
| matmul_64_f64/ndarray | 6.350 / 6.379 | 6.421 / 6.484 | 2 | 98304 | 3.90 |
| matmul_64_f64/numpy | 4.978 / 5.018 | 5.117 / 5.252 | unavailable | unavailable | 43.23 |
| matmul_ab_128_f32/kernix | 33.961 / 35.483 | 34.259 / 48.367 | 3 | 65544 | 4.62 |
| matmul_ab_128_f32/numpy | 20.781 / 21.372 | 20.974 / 21.036 | unavailable | unavailable | 45.04 |
| matmul_ab_128_f64/kernix | 69.274 / 75.754 | 69.899 / 70.280 | 3 | 131080 | 4.89 |
| matmul_ab_128_f64/numpy | 48.102 / 53.660 | 48.453 / 48.608 | unavailable | unavailable | 45.25 |
| matmul_batched_4x16x32x16_f32/candle | 1.322 / 1.353 | 1.338 / 1.360 | 25 | 13152 | 4.95 |
| matmul_batched_4x16x32x16_f32/kernix | 0.751 / 0.772 | 0.774 / 0.776 | 3 | 4104 | 3.96 |
| matmul_batched_4x16x32x16_f32/numpy | 1.462 / 1.492 | 1.506 / 2.093 | unavailable | unavailable | 42.89 |
| matmul_batched_4x16x32x16_f64/candle | 1.922 / 1.953 | 1.949 / 1.961 | 25 | 25440 | 4.97 |
| matmul_batched_4x16x32x16_f64/kernix | 1.352 / 1.372 | 1.362 / 1.394 | 3 | 8200 | 3.93 |
| matmul_batched_4x16x32x16_f64/numpy | 1.632 / 3.105 | 1.659 / 3.519 | unavailable | unavailable | 43.02 |
| matmul_sum_backward_32x16x8_f64/candle | 3.826 / 3.916 | 3.801 / 3.952 | 113 | 27988 | 5.29 |
| matmul_sum_backward_32x16x8_f64/kernix | 2.053 / 2.093 | 2.067 / 2.081 | 15 | 18376 | 3.94 |
| matmul_transposed_128_f32/kernix | 38.367 / 45.919 | 40.465 / 41.060 | 3 | 163840 | 4.53 |
| matmul_transposed_128_f32/numpy | 21.021 / 22.103 | 21.331 / 21.579 | unavailable | unavailable | 45.08 |
| matmul_transposed_128_f64/kernix | 119.730 / 128.343 | 118.940 / 119.658 | 3 | 327680 | 4.67 |
| matmul_transposed_128_f64/numpy | 49.013 / 55.353 | 49.352 / 51.924 | unavailable | unavailable | 45.21 |
| min_norm_32x64_rank32_f64/kernix | 47.852 / 53.711 | 48.290 / 49.166 | 20 | 59136 | 4.37 |
| min_norm_32x64_rank32_f64/nalgebra | 123.435 / 129.735 | 124.561 / 124.953 | 15 | 42232 | 4.00 |
| min_norm_32x64_rank32_f64/numpy | 118.498 / 124.988 | 119.322 / 120.183 | unavailable | unavailable | 48.42 |
| min_norm_64x32_rank16_f64/kernix | 36.285 / 36.776 | 36.625 / 37.144 | 20 | 74752 | 4.38 |
| min_norm_64x32_rank16_f64/nalgebra | 69.334 / 74.992 | 70.123 / 70.569 | 15 | 41976 | 4.02 |
| min_norm_64x32_rank16_f64/numpy | 63.515 / 69.634 | 64.249 / 73.241 | unavailable | unavailable | 47.40 |
| parallel_matmul_into_128_f32/kernix | 24.687 / 24.938 | 24.832 / 31.510 | 2 | 16 | 4.96 |
| parallel_matmul_into_128_f64/kernix | 42.504 / 43.856 | 42.400 / 43.021 | 2 | 16 | 5.15 |
| parallel_matmul_into_256_f64/kernix | 287.842 / 448.283 | 289.565 / 301.245 | 2 | 16 | 6.70 |
| parallel_matmul_into_64_f64/kernix | 10.145 / 10.255 | 10.188 / 10.756 | 2 | 98304 | 4.32 |
| pivoted_qr_256x64_f64/faer | 216.094 / 233.821 | 217.122 / 217.818 | 10 | 333312 | 5.06 |
| pivoted_qr_256x64_f64/kernix | 531.489 / 546.641 | 576.595 / 1029.210 | 14 | 559104 | 4.79 |
| pivoted_qr_256x64_f64/nalgebra | 611.399 / 621.243 | 614.647 / 628.860 | 7 | 296960 | 4.43 |
| pivoted_qr_96x32_f64/faer | 24.476 / 24.597 | 24.624 / 24.898 | 10 | 68352 | 4.94 |
| pivoted_qr_96x32_f64/kernix | 46.029 / 51.627 | 46.496 / 47.733 | 14 | 107520 | 4.21 |
| pivoted_qr_96x32_f64/nalgebra | 59.370 / 64.788 | 60.075 / 62.130 | 7 | 58368 | 3.91 |
| qr_1024x256_f64/faer | 6692.596 / 6968.571 | 6630.432 / 6790.638 | 7 | 5308416 | 11.91 |
| qr_1024x256_f64/kernix | 25522.079 / 27253.862 | 25570.571 / 26326.638 | 6 | 6817792 | 16.01 |
| qr_1024x256_f64/nalgebra | 22188.670 / 22585.948 | 22043.582 / 22429.782 | 4 | 4720640 | 10.25 |
| qr_1024x256_f64/numpy | 11212.453 / 11544.802 | 11149.829 / 11217.952 | unavailable | unavailable | 63.11 |
| qr_256x64_f64/faer | 228.423 / 243.736 | 228.532 / 230.008 | 7 | 331776 | 5.24 |
| qr_256x64_f64/kernix | 507.753 / 522.045 | 507.917 / 515.360 | 6 | 426496 | 4.41 |
| qr_256x64_f64/nalgebra | 356.125 / 365.780 | 354.722 / 356.332 | 4 | 295424 | 4.22 |
| qr_256x64_f64/numpy | 315.955 / 328.294 | 317.225 / 320.035 | unavailable | unavailable | 49.38 |
| qr_512x128_f64/faer | 1210.388 / 1235.196 | 1222.627 / 1235.381 | 7 | 1343488 | 6.50 |
| qr_512x128_f64/kernix | 3512.107 / 3554.791 | 3511.480 / 3527.594 | 6 | 1704960 | 6.46 |
| qr_512x128_f64/nalgebra | 2920.398 / 2956.722 | 2942.914 / 2959.359 | 4 | 1180672 | 5.29 |
| qr_512x128_f64/numpy | 2099.615 / 2138.874 | 2093.890 / 2105.532 | unavailable | unavailable | 53.17 |
| qr_96x32_f64/faer | 28.984 / 29.184 | 29.113 / 29.258 | 7 | 67584 | 4.70 |
| qr_96x32_f64/kernix | 39.810 / 40.531 | 39.831 / 41.960 | 6 | 82176 | 4.16 |
| qr_96x32_f64/nalgebra | 34.672 / 34.843 | 35.092 / 35.194 | 4 | 57600 | 3.71 |
| qr_96x32_f64/numpy | 35.564 / 37.045 | 35.789 / 36.181 | unavailable | unavailable | 48.46 |
| softmax_256x64_f64/candle | 44.216 / 51.056 | 44.537 / 44.922 | 37 | 265200 | 5.68 |
| softmax_256x64_f64/kernix | 69.214 / 75.093 | 69.609 / 71.791 | 1 | 131072 | 4.21 |
| softmax_256x64_f64/ndarray | 42.944 / 48.553 | 43.157 / 43.521 | 1 | 131072 | 4.21 |
| softmax_256x64_f64/numpy | 44.307 / 50.776 | 43.611 / 45.738 | unavailable | unavailable | 43.29 |
| solve_cholesky_128x1_f64/faer | 3.676 / 3.716 | 3.701 / 6.603 | 1 | 1024 | 4.82 |
| solve_cholesky_128x1_f64/kernix | 9.314 / 9.374 | 9.227 / 9.433 | 1 | 1024 | 4.21 |
| solve_cholesky_128x1_f64/nalgebra | 3.606 / 3.625 | 3.649 / 3.674 | 1 | 1024 | 4.02 |
| solve_cholesky_256x64_f64/faer | 123.235 / 131.768 | 124.372 / 125.541 | 1 | 131072 | 6.58 |
| solve_cholesky_256x64_f64/kernix | 3845.079 / 4033.099 | 3912.906 / 4010.225 | 1 | 131072 | 5.52 |
| solve_cholesky_256x64_f64/nalgebra | 732.881 / 742.446 | 731.663 / 756.586 | 1 | 131072 | 5.33 |
| solve_cholesky_512x128_f64/faer | 1113.783 / 1622.228 | 1111.414 / 1113.390 | 1 | 524288 | 10.85 |
| solve_cholesky_512x128_f64/kernix | 42785.584 / 43939.159 | 42648.830 / 44896.878 | 1 | 524288 | 9.54 |
| solve_cholesky_512x128_f64/nalgebra | 5924.202 / 6934.930 | 5925.078 / 5975.313 | 1 | 524288 | 9.79 |
| solve_lu_256x64_f64/faer | 183.366 / 198.468 | 186.591 / 215.195 | 2 | 262144 | 7.19 |
| solve_lu_256x64_f64/kernix | 2725.745 / 2951.214 | 2775.137 / 2908.941 | 2 | 262144 | 5.31 |
| solve_lu_256x64_f64/nalgebra | 681.494 / 699.260 | 680.905 / 683.436 | 1 | 131072 | 5.27 |
| tensordot_16x32x8_32x8x16_f64/kernix | 31.628 / 31.747 | 31.874 / 32.066 | 9 | 2166 | 3.86 |
| tensordot_16x32x8_32x8x16_f64/numpy | 4.958 / 5.067 | 5.023 / 5.114 | unavailable | unavailable | 42.93 |
| training_backward_2x4x8x4_f64/candle | 12.739 / 13.010 | 12.811 / 13.025 | 472 | 8332 | 5.93 |
| training_backward_2x4x8x4_f64/kernix | 3.034 / 3.115 | 3.084 / 3.123 | 23 | 4480 | 3.95 |
| training_forward_2x4x8x4_f64/candle | 2.955 / 3.055 | 2.968 / 3.016 | 126 | 3912 | 5.58 |
| training_forward_2x4x8x4_f64/kernix | 1.893 / 1.933 | 1.910 / 1.962 | 17 | 1288 | 4.20 |
| training_record_2x4x8x4_f64/candle | 4.226 / 4.426 | 4.296 / 4.345 | 149 | 10712 | 5.64 |
| training_record_2x4x8x4_f64/kernix | 2.423 / 2.464 | 2.451 / 2.483 | 22 | 6832 | 4.11 |
| training_sgd_2x4x8x4_f64/kernix | 0.531 / 0.560 | 0.538 / 0.541 | 5 | 688 | 4.36 |
| training_shared_backward_64x32_f64/candle | 3.105 / 3.185 | 3.159 / 3.269 | 58 | 100004 | 5.16 |
| training_shared_backward_64x32_f64/kernix | 4.977 / 4.998 | 5.056 / 5.410 | 8 | 49768 | 4.19 |

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
