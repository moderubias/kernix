# Clean CPU evidence: 0.12.0-final

Measured source `90cbbff60e5d7458ef9f89918c99a6cca94b51d1`, tree `d806b9704a44b4bf2c4c04e664fe3c60eeebc177`; dirty before/after: false.
Library source: workspace 90cbbff60e5d7458ef9f89918c99a6cca94b51d1. CPU affinity: 0. Dtype explicitly named per workload; row-major inputs.

| Workload / library | Per-call p50 / p95 (us) | Batch-mean p50 / p95 (us) | Rust allocations | Peak additional requested bytes | Process peak RSS (MiB) |
|---|---:|---:|---:|---:|---:|
| cholesky_factor_128_f64/faer | 37.224 / 39.274 | 35.492 / 37.637 | 2 | 132096 | 5.00 |
| cholesky_factor_128_f64/kernix | 112.836 / 126.148 | 118.306 / 127.543 | 1 | 131072 | 4.64 |
| cholesky_factor_128_f64/nalgebra | 96.496 / 101.773 | 97.121 / 103.222 | 1 | 131072 | 4.16 |
| cholesky_factor_128_f64/numpy | 68.714 / 80.034 | 71.380 / 74.386 | unavailable | unavailable | 37.55 |
| cholesky_factor_256_f64/faer | 194.053 / 227.653 | 196.472 / 236.269 | 2 | 526336 | 6.04 |
| cholesky_factor_256_f64/kernix | 485.969 / 541.520 | 492.488 / 509.268 | 49 | 645120 | 6.05 |
| cholesky_factor_256_f64/nalgebra | 913.281 / 1148.698 | 654.977 / 974.736 | 1 | 524288 | 5.02 |
| cholesky_factor_256_f64/numpy | 514.590 / 692.064 | 558.955 / 636.564 | unavailable | unavailable | 41.58 |
| cholesky_factor_512_f64/faer | 1381.789 / 2423.240 | 1437.961 / 1934.782 | 2 | 2101248 | 9.06 |
| cholesky_factor_512_f64/kernix | 3174.989 / 4721.258 | 3162.653 / 6234.752 | 161 | 2269184 | 10.73 |
| cholesky_factor_512_f64/nalgebra | 5227.821 / 6668.463 | 5375.809 / 6580.449 | 1 | 2097152 | 7.87 |
| cholesky_factor_512_f64/numpy | 3828.883 / 6668.674 | 3890.936 / 5118.977 | unavailable | unavailable | 53.41 |
| cholesky_factor_64_f64/faer | 8.370 / 9.632 | 8.557 / 8.708 | 2 | 33280 | 4.42 |
| cholesky_factor_64_f64/kernix | 21.617 / 25.419 | 21.176 / 22.136 | 2 | 33280 | 4.51 |
| cholesky_factor_64_f64/nalgebra | 14.128 / 16.959 | 15.032 / 17.395 | 1 | 32768 | 3.96 |
| cholesky_factor_64_f64/numpy | 17.409 / 27.231 | 11.081 / 21.074 | unavailable | unavailable | 36.86 |
| cholesky_rank_downdate_256_f64/kernix | 119.132 / 130.547 | 125.515 / 142.822 | 3 | 526336 | 5.93 |
| cholesky_rank_downdate_256_f64/nalgebra | 23.870 / 26.050 | 24.664 / 26.516 | 2 | 526336 | 6.46 |
| cholesky_rank_downdate_256_f64/refactor | 540.500 / 833.870 | 520.266 / 575.194 | 49 | 645120 | 6.66 |
| cholesky_rank_update_256_f64/kernix | 112.750 / 138.039 | 116.316 / 166.780 | 3 | 526336 | 5.96 |
| cholesky_rank_update_256_f64/nalgebra | 21.713 / 26.833 | 22.273 / 24.184 | 2 | 526336 | 6.67 |
| cholesky_rank_update_256_f64/refactor | 529.381 / 740.421 | 518.905 / 545.198 | 49 | 645120 | 6.68 |
| cholesky_rank_update_512_f64/kernix | 722.691 / 1000.623 | 705.249 / 740.118 | 3 | 2101248 | 10.56 |
| cholesky_rank_update_512_f64/nalgebra | 147.525 / 179.300 | 153.687 / 202.071 | 2 | 2101248 | 13.59 |
| cholesky_rank_update_512_f64/refactor | 2818.152 / 3450.630 | 2927.258 / 3482.776 | 161 | 2269184 | 12.76 |
| create_ones_64x64_f32/candle | 0.218 / 0.240 | 0.227 / 0.230 | 7 | 16696 | 4.68 |
| create_ones_64x64_f32/faer | 0.203 / 0.206 | 0.187 / 0.193 | 1 | 16384 | 4.36 |
| create_ones_64x64_f32/kernix | 0.201 / 0.217 | 0.196 / 0.286 | 1 | 16384 | 4.12 |
| create_ones_64x64_f32/ndarray | 0.140 / 0.142 | 0.145 / 0.157 | 1 | 16384 | 4.04 |
| create_ones_64x64_f32/numpy | 1.331 / 1.422 | 1.290 / 1.420 | unavailable | unavailable | 35.77 |
| create_zeros_64x64_f32/candle | 0.168 / 0.170 | 0.176 / 0.260 | 5 | 16648 | 4.54 |
| create_zeros_64x64_f32/faer | 0.143 / 0.155 | 0.179 / 0.185 | 1 | 16384 | 4.36 |
| create_zeros_64x64_f32/kernix | 0.141 / 0.143 | 0.145 / 0.150 | 1 | 16384 | 4.15 |
| create_zeros_64x64_f32/ndarray | 0.098 / 0.100 | 0.103 / 0.104 | 1 | 16384 | 4.12 |
| create_zeros_64x64_f32/numpy | 0.376 / 0.465 | 0.418 / 0.462 | unavailable | unavailable | 35.77 |
| matmul_128_f32/candle | 34.972 / 38.135 | 36.327 / 37.537 | 9 | 65816 | 5.23 |
| matmul_128_f32/faer | 37.779 / 41.022 | 38.522 / 41.722 | 1 | 65536 | 4.83 |
| matmul_128_f32/kernix | 34.163 / 37.007 | 34.823 / 36.185 | 3 | 65544 | 5.04 |
| matmul_128_f32/nalgebra | 50.392 / 64.433 | 53.635 / 61.577 | 2 | 163840 | 4.24 |
| matmul_128_f32/ndarray | 48.903 / 57.421 | 49.894 / 53.588 | 2 | 163840 | 4.42 |
| matmul_128_f32/numpy | 36.352 / 40.135 | 38.236 / 41.270 | unavailable | unavailable | 37.37 |
| matmul_128_f64/candle | 68.889 / 76.756 | 70.178 / 75.778 | 9 | 131352 | 5.48 |
| matmul_128_f64/faer | 71.690 / 79.827 | 72.699 / 75.130 | 1 | 131072 | 4.91 |
| matmul_128_f64/kernix | 74.135 / 85.373 | 75.945 / 80.437 | 3 | 131080 | 5.02 |
| matmul_128_f64/nalgebra | 119.011 / 143.676 | 121.695 / 127.281 | 2 | 327680 | 4.54 |
| matmul_128_f64/ndarray | 119.173 / 130.898 | 122.630 / 161.785 | 2 | 327680 | 4.57 |
| matmul_128_f64/numpy | 77.964 / 83.480 | 76.542 / 78.689 | unavailable | unavailable | 37.71 |
| matmul_256_f32/candle | 271.568 / 310.768 | 287.813 / 300.190 | 9 | 262424 | 5.90 |
| matmul_256_f32/faer | 305.567 / 334.745 | 311.456 / 315.841 | 1 | 262144 | 5.31 |
| matmul_256_f32/kernix | 271.677 / 295.961 | 266.574 / 281.515 | 3 | 262152 | 5.18 |
| matmul_256_f32/nalgebra | 387.147 / 444.057 | 393.105 / 417.558 | 2 | 589824 | 4.80 |
| matmul_256_f32/ndarray | 413.609 / 560.595 | 418.210 / 482.486 | 2 | 589824 | 4.78 |
| matmul_256_f32/numpy | 280.681 / 320.326 | 290.887 / 408.704 | unavailable | unavailable | 40.61 |
| matmul_256_f64/candle | 563.193 / 607.054 | 570.802 / 606.701 | 9 | 524568 | 6.86 |
| matmul_256_f64/faer | 679.925 / 768.621 | 690.586 / 747.927 | 1 | 524288 | 5.96 |
| matmul_256_f64/kernix | 585.325 / 717.360 | 589.274 / 645.877 | 3 | 524296 | 6.39 |
| matmul_256_f64/nalgebra | 631.138 / 674.865 | 633.328 / 659.571 | 2 | 1179648 | 5.77 |
| matmul_256_f64/ndarray | 666.647 / 844.421 | 783.539 / 1169.088 | 2 | 1179648 | 6.23 |
| matmul_256_f64/numpy | 557.881 / 599.869 | 572.207 / 584.786 | unavailable | unavailable | 41.62 |
| matmul_32_f64/candle | 1.400 / 1.432 | 1.451 / 1.556 | 9 | 8592 | 4.91 |
| matmul_32_f64/faer | 1.541 / 1.635 | 1.585 / 1.620 | 1 | 8192 | 4.59 |
| matmul_32_f64/kernix | 1.566 / 1.596 | 1.659 / 1.846 | 3 | 24576 | 4.51 |
| matmul_32_f64/nalgebra | 1.392 / 1.420 | 1.421 / 1.565 | 2 | 24576 | 4.15 |
| matmul_32_f64/ndarray | 1.501 / 1.761 | 1.529 / 1.578 | 2 | 24576 | 4.05 |
| matmul_32_f64/numpy | 2.355 / 2.397 | 2.435 / 2.570 | unavailable | unavailable | 36.38 |
| matmul_64_f32/candle | 4.890 / 5.088 | 5.062 / 5.260 | 9 | 16664 | 5.07 |
| matmul_64_f32/faer | 4.910 / 5.193 | 4.939 / 5.247 | 1 | 16384 | 4.36 |
| matmul_64_f32/kernix | 4.502 / 4.890 | 4.850 / 5.124 | 3 | 16544 | 4.74 |
| matmul_64_f32/nalgebra | 6.083 / 6.274 | 5.741 / 6.547 | 2 | 49152 | 4.27 |
| matmul_64_f32/ndarray | 6.152 / 6.306 | 5.970 / 6.269 | 2 | 49152 | 3.91 |
| matmul_64_f32/numpy | 5.775 / 6.202 | 5.919 / 6.237 | unavailable | unavailable | 36.57 |
| matmul_64_f64/candle | 9.478 / 9.575 | 9.617 / 10.039 | 9 | 33048 | 5.02 |
| matmul_64_f64/faer | 10.286 / 10.416 | 10.451 / 11.327 | 1 | 32768 | 4.71 |
| matmul_64_f64/kernix | 10.140 / 12.902 | 10.644 / 10.871 | 3 | 32928 | 4.83 |
| matmul_64_f64/nalgebra | 11.275 / 15.090 | 10.662 / 11.069 | 2 | 98304 | 4.33 |
| matmul_64_f64/ndarray | 10.543 / 11.382 | 10.796 / 11.247 | 2 | 98304 | 4.25 |
| matmul_64_f64/numpy | 10.994 / 11.978 | 11.198 / 11.530 | unavailable | unavailable | 36.69 |
| matmul_ab_128_f32/kernix | 36.784 / 37.656 | 36.389 / 41.868 | 3 | 65544 | 5.01 |
| matmul_ab_128_f32/numpy | 39.052 / 48.388 | 37.393 / 42.238 | unavailable | unavailable | 37.32 |
| matmul_ab_128_f64/kernix | 69.063 / 74.353 | 70.732 / 101.533 | 3 | 131080 | 5.23 |
| matmul_ab_128_f64/numpy | 82.268 / 104.058 | 82.953 / 89.860 | unavailable | unavailable | 37.70 |
| matmul_batched_4x16x32x16_f32/candle | 1.371 / 1.485 | 1.433 / 3.513 | 25 | 13216 | 5.61 |
| matmul_batched_4x16x32x16_f32/kernix | 0.961 / 1.025 | 0.974 / 1.015 | 3 | 4104 | 4.56 |
| matmul_batched_4x16x32x16_f32/numpy | 2.032 / 2.059 | 3.066 / 5.775 | unavailable | unavailable | 36.34 |
| matmul_batched_4x16x32x16_f64/candle | 2.222 / 2.275 | 2.263 / 2.399 | 25 | 25504 | 5.62 |
| matmul_batched_4x16x32x16_f64/kernix | 2.005 / 2.416 | 2.021 / 2.081 | 3 | 8200 | 4.43 |
| matmul_batched_4x16x32x16_f64/numpy | 2.756 / 2.828 | 2.825 / 2.966 | unavailable | unavailable | 36.34 |
| matmul_sum_backward_32x16x8_f64/candle | 4.153 / 4.464 | 4.288 / 4.340 | 113 | 27988 | 5.50 |
| matmul_sum_backward_32x16x8_f64/kernix | 2.254 / 2.293 | 2.304 / 2.497 | 15 | 18376 | 4.32 |
| matmul_transposed_128_f32/kernix | 49.265 / 60.611 | 50.386 / 53.100 | 3 | 163840 | 4.92 |
| matmul_transposed_128_f32/numpy | 36.336 / 252.605 | 39.341 / 83.382 | unavailable | unavailable | 37.24 |
| matmul_transposed_128_f64/kernix | 122.731 / 151.118 | 125.217 / 131.540 | 3 | 327680 | 5.29 |
| matmul_transposed_128_f64/numpy | 78.575 / 507.037 | 91.151 / 147.334 | unavailable | unavailable | 37.70 |
| min_norm_32x64_rank32_f64/kernix | 65.960 / 105.197 | 56.445 / 58.106 | 20 | 59136 | 4.79 |
| min_norm_32x64_rank32_f64/nalgebra | 124.527 / 136.170 | 129.738 / 141.189 | 15 | 42232 | 4.52 |
| min_norm_32x64_rank32_f64/numpy | 114.402 / 130.350 | 116.002 / 126.165 | unavailable | unavailable | 37.38 |
| min_norm_64x32_rank16_f64/kernix | 41.648 / 46.666 | 42.389 / 45.834 | 20 | 74752 | 4.88 |
| min_norm_64x32_rank16_f64/nalgebra | 73.977 / 78.444 | 72.691 / 75.047 | 15 | 41976 | 4.52 |
| min_norm_64x32_rank16_f64/numpy | 66.909 / 110.222 | 66.422 / 74.764 | unavailable | unavailable | 37.43 |
| parallel_matmul_into_128_f32/kernix | 22.457 / 28.450 | 23.887 / 42.185 | 2 | 8 | 5.19 |
| parallel_matmul_into_128_f64/kernix | 41.991 / 55.976 | 51.079 / 117.959 | 2 | 8 | 5.14 |
| parallel_matmul_into_256_f64/kernix | 306.270 / 601.852 | 310.495 / 321.547 | 2 | 16 | 7.37 |
| parallel_matmul_into_64_f64/kernix | 9.656 / 10.534 | 10.124 / 10.414 | 2 | 98304 | 4.85 |
| pivoted_qr_256x64_f64/faer | 472.299 / 556.986 | 490.873 / 550.192 | 10 | 333312 | 5.34 |
| pivoted_qr_256x64_f64/kernix | 542.780 / 750.377 | 522.826 / 576.449 | 14 | 559104 | 5.09 |
| pivoted_qr_256x64_f64/nalgebra | 785.617 / 856.188 | 796.224 / 852.418 | 7 | 296960 | 4.87 |
| pivoted_qr_96x32_f64/faer | 70.533 / 78.742 | 72.492 / 76.630 | 10 | 68352 | 4.96 |
| pivoted_qr_96x32_f64/kernix | 52.032 / 58.530 | 53.360 / 92.731 | 14 | 107520 | 4.69 |
| pivoted_qr_96x32_f64/nalgebra | 83.223 / 87.319 | 83.418 / 85.419 | 7 | 58368 | 4.38 |
| qr_1024x256_f64/faer | 13100.052 / 15755.556 | 12679.620 / 14802.021 | 7 | 5308416 | 12.26 |
| qr_1024x256_f64/kernix | 17883.006 / 25954.038 | 19495.450 / 29412.811 | 82 | 7145480 | 17.80 |
| qr_1024x256_f64/nalgebra | 26987.916 / 29510.434 | 27874.920 / 31124.998 | 4 | 4720640 | 10.71 |
| qr_1024x256_f64/numpy | 13946.581 / 15986.645 | 13270.319 / 17278.241 | unavailable | unavailable | 52.88 |
| qr_256x64_f64/faer | 426.410 / 475.797 | 447.752 / 493.472 | 7 | 331776 | 5.32 |
| qr_256x64_f64/kernix | 515.956 / 643.692 | 796.287 / 837.419 | 6 | 426496 | 5.00 |
| qr_256x64_f64/nalgebra | 372.457 / 405.190 | 417.375 / 711.103 | 4 | 295424 | 4.59 |
| qr_256x64_f64/numpy | 316.185 / 364.151 | 322.427 / 337.553 | unavailable | unavailable | 38.21 |
| qr_512x128_f64/faer | 2187.147 / 2859.386 | 2733.287 / 4267.495 | 7 | 1343488 | 6.80 |
| qr_512x128_f64/kernix | 2667.134 / 3670.294 | 2679.416 / 3083.748 | 49 | 2294792 | 8.24 |
| qr_512x128_f64/nalgebra | 3130.708 / 4487.725 | 3114.576 / 3435.276 | 4 | 1180672 | 5.79 |
| qr_512x128_f64/numpy | 2526.361 / 3721.977 | 2551.097 / 5525.643 | unavailable | unavailable | 41.71 |
| qr_96x32_f64/faer | 65.215 / 71.613 | 67.556 / 70.575 | 7 | 67584 | 5.02 |
| qr_96x32_f64/kernix | 47.241 / 55.792 | 47.301 / 49.592 | 6 | 82176 | 4.59 |
| qr_96x32_f64/nalgebra | 34.400 / 38.145 | 35.294 / 36.516 | 4 | 57600 | 4.29 |
| qr_96x32_f64/numpy | 39.234 / 41.173 | 39.862 / 40.985 | unavailable | unavailable | 37.46 |
| softmax_256x64_f64/candle | 80.789 / 88.483 | 80.558 / 84.979 | 37 | 265200 | 5.89 |
| softmax_256x64_f64/kernix | 75.698 / 88.985 | 78.047 / 80.810 | 1 | 131072 | 4.75 |
| softmax_256x64_f64/ndarray | 69.113 / 84.146 | 71.223 / 79.564 | 1 | 131072 | 4.42 |
| softmax_256x64_f64/numpy | 100.295 / 131.614 | 102.451 / 107.981 | unavailable | unavailable | 36.27 |
| solve_cholesky_128x1_f64/faer | 3.941 / 4.048 | 3.769 / 3.872 | 1 | 1024 | 5.21 |
| solve_cholesky_128x1_f64/kernix | 9.304 / 9.619 | 9.768 / 10.284 | 1 | 1024 | 4.81 |
| solve_cholesky_128x1_f64/nalgebra | 4.044 / 4.854 | 3.951 / 4.163 | 1 | 1024 | 4.53 |
| solve_cholesky_256x64_f64/faer | 199.222 / 228.009 | 211.410 / 237.993 | 1 | 131072 | 6.17 |
| solve_cholesky_256x64_f64/kernix | 1007.406 / 1125.253 | 1096.590 / 1145.401 | 1 | 131072 | 6.02 |
| solve_cholesky_256x64_f64/nalgebra | 769.663 / 912.036 | 774.891 / 847.780 | 1 | 131072 | 5.73 |
| solve_cholesky_512x128_f64/faer | 1658.008 / 3079.083 | 1738.218 / 2187.038 | 1 | 524288 | 11.01 |
| solve_cholesky_512x128_f64/kernix | 9067.001 / 10327.385 | 8880.355 / 9226.060 | 1 | 524288 | 10.23 |
| solve_cholesky_512x128_f64/nalgebra | 6934.999 / 8957.689 | 8208.026 / 10404.204 | 1 | 524288 | 10.22 |
| solve_lu_256x64_f64/faer | 239.854 / 298.068 | 245.737 / 278.556 | 2 | 262144 | 7.24 |
| solve_lu_256x64_f64/kernix | 1103.033 / 1206.253 | 1101.387 / 1372.276 | 2 | 262144 | 5.47 |
| solve_lu_256x64_f64/nalgebra | 754.701 / 859.504 | 765.980 / 1027.619 | 1 | 131072 | 5.38 |
| tensordot_16x32x8_32x8x16_f64/kernix | 35.885 / 37.012 | 33.722 / 44.974 | 9 | 2166 | 4.52 |
| tensordot_16x32x8_32x8x16_f64/numpy | 7.201 / 7.476 | 7.486 / 8.222 | unavailable | unavailable | 36.34 |
| training_backward_2x4x8x4_f64/candle | 14.463 / 14.996 | 14.215 / 15.225 | 472 | 8332 | 5.57 |
| training_backward_2x4x8x4_f64/kernix | 2.943 / 3.258 | 3.085 / 3.242 | 23 | 4224 | 4.58 |
| training_forward_2x4x8x4_f64/candle | 3.451 / 3.568 | 3.593 / 3.621 | 126 | 3912 | 5.98 |
| training_forward_2x4x8x4_f64/kernix | 2.188 / 2.252 | 2.275 / 2.454 | 17 | 1032 | 4.82 |
| training_record_2x4x8x4_f64/candle | 5.060 / 6.082 | 5.053 / 5.230 | 149 | 10712 | 5.86 |
| training_record_2x4x8x4_f64/kernix | 2.700 / 2.761 | 2.774 / 2.817 | 22 | 6832 | 4.71 |
| training_sgd_2x4x8x4_f64/kernix | 0.510 / 0.540 | 0.517 / 0.532 | 5 | 688 | 4.79 |
| training_shared_backward_64x32_f64/candle | 3.870 / 3.999 | 3.590 / 5.186 | 58 | 100004 | 5.47 |
| training_shared_backward_64x32_f64/kernix | 6.062 / 6.685 | 6.498 / 6.755 | 8 | 49768 | 4.81 |

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
