# Clean CPU evidence: runner-macos-latest-40270611933

Measured source `90cbbff60e5d7458ef9f89918c99a6cca94b51d1`, tree `d806b9704a44b4bf2c4c04e664fe3c60eeebc177`; dirty before/after: false.
Library source: workspace 90cbbff60e5d7458ef9f89918c99a6cca94b51d1. CPU affinity: not pinned. Dtype explicitly named per workload; row-major inputs.

| Workload / library | Per-call p50 / p95 (us) | Batch-mean p50 / p95 (us) | Rust allocations | Peak additional requested bytes | Process peak RSS (MiB) |
|---|---:|---:|---:|---:|---:|
| cholesky_factor_128_f64/faer | 51.542 / 66.250 | 96.540 / 145.132 | 16 | 149504 | unavailable |
| cholesky_factor_128_f64/kernix | 165.833 / 171.584 | 166.602 / 171.912 | 1 | 131072 | unavailable |
| cholesky_factor_128_f64/nalgebra | 117.000 / 258.750 | 106.644 / 146.415 | 1 | 131072 | unavailable |
| cholesky_factor_128_f64/numpy | 47.125 / 51.250 | 47.072 / 75.444 | unavailable | unavailable | unavailable |
| cholesky_factor_256_f64/faer | 278.000 / 353.958 | 281.006 / 312.956 | 76 | 592384 | unavailable |
| cholesky_factor_256_f64/kernix | 588.292 / 738.917 | 682.803 / 707.128 | 49 | 645120 | unavailable |
| cholesky_factor_256_f64/nalgebra | 669.292 / 688.917 | 670.153 / 844.269 | 1 | 524288 | unavailable |
| cholesky_factor_256_f64/numpy | 332.250 / 399.625 | 327.543 / 360.366 | unavailable | unavailable | unavailable |
| cholesky_factor_512_f64/faer | 1788.000 / 2845.708 | 2021.992 / 2908.719 | 388 | 2625544 | unavailable |
| cholesky_factor_512_f64/kernix | 3040.417 / 3413.625 | 3001.844 / 3105.969 | 161 | 2269184 | unavailable |
| cholesky_factor_512_f64/nalgebra | 5920.084 / 7968.500 | 6829.605 / 11422.584 | 1 | 2097152 | unavailable |
| cholesky_factor_512_f64/numpy | 2407.750 / 2638.000 | 2459.325 / 2694.240 | unavailable | unavailable | unavailable |
| cholesky_factor_64_f64/faer | 10.083 / 10.167 | 10.634 / 13.514 | 2 | 33280 | unavailable |
| cholesky_factor_64_f64/kernix | 20.208 / 20.916 | 20.697 / 22.602 | 2 | 33280 | unavailable |
| cholesky_factor_64_f64/nalgebra | 19.500 / 20.041 | 19.900 / 20.459 | 1 | 32768 | unavailable |
| cholesky_factor_64_f64/numpy | 13.750 / 14.000 | 15.494 / 16.386 | unavailable | unavailable | unavailable |
| cholesky_rank_downdate_256_f64/kernix | 110.625 / 116.667 | 110.469 / 120.913 | 3 | 526336 | unavailable |
| cholesky_rank_downdate_256_f64/nalgebra | 20.209 / 23.417 | 24.518 / 28.831 | 2 | 526336 | unavailable |
| cholesky_rank_downdate_256_f64/refactor | 585.125 / 699.583 | 660.174 / 714.008 | 49 | 645120 | unavailable |
| cholesky_rank_update_256_f64/kernix | 120.834 / 149.416 | 117.893 / 140.352 | 3 | 526336 | unavailable |
| cholesky_rank_update_256_f64/nalgebra | 65.209 / 71.875 | 20.623 / 29.727 | 2 | 526336 | unavailable |
| cholesky_rank_update_256_f64/refactor | 643.250 / 734.208 | 652.812 / 727.726 | 49 | 645120 | unavailable |
| cholesky_rank_update_512_f64/kernix | 558.500 / 823.417 | 538.586 / 669.722 | 3 | 2101248 | unavailable |
| cholesky_rank_update_512_f64/nalgebra | 100.916 / 105.791 | 106.644 / 116.794 | 2 | 2101248 | unavailable |
| cholesky_rank_update_512_f64/refactor | 3070.125 / 3971.500 | 3090.969 / 3675.875 | 161 | 2269184 | unavailable |
| create_ones_64x64_f32/candle | 0.542 / 0.750 | 0.504 / 1.144 | 7 | 16696 | unavailable |
| create_ones_64x64_f32/faer | 0.375 / 0.500 | 0.366 / 0.551 | 1 | 16384 | unavailable |
| create_ones_64x64_f32/kernix | 0.333 / 0.334 | 0.269 / 0.276 | 1 | 16384 | unavailable |
| create_ones_64x64_f32/ndarray | 0.250 / 0.250 | 0.270 / 0.328 | 1 | 16384 | unavailable |
| create_ones_64x64_f32/numpy | 1.292 / 2.916 | 1.426 / 2.641 | unavailable | unavailable | unavailable |
| create_zeros_64x64_f32/candle | 0.333 / 0.334 | 0.337 / 0.499 | 5 | 16648 | unavailable |
| create_zeros_64x64_f32/faer | 0.250 / 0.292 | 0.252 / 0.310 | 1 | 16384 | unavailable |
| create_zeros_64x64_f32/kernix | 0.291 / 0.292 | 0.282 / 0.364 | 1 | 16384 | unavailable |
| create_zeros_64x64_f32/ndarray | 0.333 / 0.542 | 0.407 / 0.675 | 1 | 16384 | unavailable |
| create_zeros_64x64_f32/numpy | 0.584 / 0.792 | 0.698 / 0.822 | unavailable | unavailable | unavailable |
| matmul_128_f32/candle | 51.125 / 59.500 | 53.601 / 58.670 | 10 | 327768 | unavailable |
| matmul_128_f32/faer | 51.541 / 53.416 | 56.447 / 59.304 | 3 | 393224 | unavailable |
| matmul_128_f32/kernix | 48.916 / 50.625 | 49.043 / 50.940 | 3 | 163840 | unavailable |
| matmul_128_f32/kernix-accelerate | 5.834 / 6.125 | 6.389 / 9.638 | 1 | 65536 | unavailable |
| matmul_128_f32/nalgebra | 50.875 / 60.875 | 52.207 / 53.404 | 2 | 163840 | unavailable |
| matmul_128_f32/ndarray | 49.542 / 62.166 | 56.362 / 60.118 | 2 | 163840 | unavailable |
| matmul_128_f32/numpy | 5.917 / 6.208 | 6.147 / 6.631 | unavailable | unavailable | unavailable |
| matmul_128_f64/candle | 107.042 / 162.750 | 112.007 / 119.265 | 10 | 655448 | unavailable |
| matmul_128_f64/faer | 107.083 / 125.375 | 114.981 / 118.475 | 3 | 786440 | unavailable |
| matmul_128_f64/kernix | 96.667 / 121.625 | 108.163 / 116.548 | 3 | 327680 | unavailable |
| matmul_128_f64/kernix-accelerate | 17.708 / 20.417 | 17.316 / 18.382 | 1 | 131072 | unavailable |
| matmul_128_f64/nalgebra | 102.000 / 121.208 | 100.615 / 111.864 | 2 | 327680 | unavailable |
| matmul_128_f64/ndarray | 94.083 / 98.792 | 96.251 / 100.558 | 2 | 327680 | unavailable |
| matmul_128_f64/numpy | 17.875 / 21.292 | 18.088 / 21.294 | unavailable | unavailable | unavailable |
| matmul_256_f32/candle | 428.916 / 484.125 | 394.526 / 718.321 | 10 | 786520 | unavailable |
| matmul_256_f32/faer | 436.083 / 494.125 | 438.803 / 579.646 | 3 | 1048584 | unavailable |
| matmul_256_f32/kernix | 366.667 / 379.125 | 368.610 / 397.755 | 3 | 589824 | unavailable |
| matmul_256_f32/kernix-accelerate | 35.625 / 37.208 | 33.178 / 42.663 | 1 | 262144 | unavailable |
| matmul_256_f32/nalgebra | 438.250 / 1256.250 | 554.013 / 818.990 | 2 | 589824 | unavailable |
| matmul_256_f32/ndarray | 365.334 / 394.875 | 365.920 / 402.605 | 2 | 589824 | unavailable |
| matmul_256_f32/numpy | 35.291 / 37.750 | 33.226 / 51.873 | unavailable | unavailable | unavailable |
| matmul_256_f64/candle | 876.167 / 1362.875 | 899.125 / 1031.475 | 10 | 1572952 | unavailable |
| matmul_256_f64/faer | 893.792 / 994.542 | 844.087 / 950.932 | 3 | 1572872 | unavailable |
| matmul_256_f64/kernix | 741.292 / 862.833 | 834.691 / 888.802 | 3 | 1179648 | unavailable |
| matmul_256_f64/kernix-accelerate | 123.584 / 139.709 | 126.845 / 181.831 | 1 | 524288 | unavailable |
| matmul_256_f64/nalgebra | 797.041 / 928.000 | 745.878 / 779.378 | 2 | 1179648 | unavailable |
| matmul_256_f64/ndarray | 815.042 / 900.666 | 808.420 / 838.621 | 2 | 1179648 | unavailable |
| matmul_256_f64/numpy | 117.000 / 135.333 | 119.835 / 123.280 | unavailable | unavailable | unavailable |
| matmul_32_f64/candle | 4.458 / 7.750 | 3.953 / 7.441 | 10 | 18512 | unavailable |
| matmul_32_f64/faer | 2.417 / 2.958 | 2.586 / 2.648 | 3 | 26624 | unavailable |
| matmul_32_f64/kernix | 2.166 / 2.250 | 2.317 / 2.379 | 3 | 24576 | unavailable |
| matmul_32_f64/nalgebra | 2.542 / 2.750 | 2.228 / 2.315 | 2 | 24576 | unavailable |
| matmul_32_f64/ndarray | 2.125 / 5.959 | 1.932 / 2.323 | 2 | 24576 | unavailable |
| matmul_32_f64/numpy | 1.292 / 1.750 | 1.513 / 1.751 | unavailable | unavailable | unavailable |
| matmul_64_f32/candle | 7.334 / 7.459 | 7.358 / 8.029 | 10 | 33872 | unavailable |
| matmul_64_f32/faer | 7.750 / 7.792 | 7.674 / 8.137 | 3 | 50176 | unavailable |
| matmul_64_f32/kernix | 7.708 / 10.250 | 8.089 / 16.018 | 3 | 49152 | unavailable |
| matmul_64_f32/nalgebra | 6.917 / 8.583 | 7.268 / 7.948 | 2 | 49152 | unavailable |
| matmul_64_f32/ndarray | 7.083 / 9.000 | 7.501 / 8.938 | 2 | 49152 | unavailable |
| matmul_64_f32/numpy | 1.917 / 2.166 | 2.161 / 2.535 | unavailable | unavailable | unavailable |
| matmul_64_f64/candle | 14.833 / 26.500 | 17.488 / 35.262 | 10 | 66640 | unavailable |
| matmul_64_f64/faer | 13.750 / 14.208 | 14.011 / 16.815 | 3 | 99328 | unavailable |
| matmul_64_f64/kernix | 13.959 / 17.583 | 15.677 / 16.271 | 3 | 98304 | unavailable |
| matmul_64_f64/nalgebra | 12.625 / 12.708 | 14.500 / 15.444 | 2 | 98304 | unavailable |
| matmul_64_f64/ndarray | 14.333 / 41.375 | 20.252 / 55.755 | 2 | 98304 | unavailable |
| matmul_64_f64/numpy | 3.500 / 3.708 | 3.684 / 12.502 | unavailable | unavailable | unavailable |
| matmul_ab_128_f32/kernix | 53.000 / 66.542 | 57.986 / 151.892 | 3 | 163840 | unavailable |
| matmul_ab_128_f32/numpy | 5.875 / 6.167 | 6.383 / 7.985 | unavailable | unavailable | unavailable |
| matmul_ab_128_f64/kernix | 95.792 / 113.625 | 107.656 / 132.937 | 3 | 327680 | unavailable |
| matmul_ab_128_f64/numpy | 17.667 / 18.834 | 18.243 / 69.294 | unavailable | unavailable | unavailable |
| matmul_batched_4x16x32x16_f32/candle | 2.708 / 7.500 | 2.810 / 4.754 | 25 | 14944 | unavailable |
| matmul_batched_4x16x32x16_f32/kernix | 5.167 / 5.458 | 1.998 / 3.736 | 7 | 8200 | unavailable |
| matmul_batched_4x16x32x16_f32/numpy | 2.209 / 3.333 | 2.133 / 4.141 | unavailable | unavailable | unavailable |
| matmul_batched_4x16x32x16_f64/candle | 3.750 / 4.250 | 3.266 / 3.878 | 25 | 27232 | unavailable |
| matmul_batched_4x16x32x16_f64/kernix | 2.958 / 3.750 | 3.145 / 3.411 | 7 | 16392 | unavailable |
| matmul_batched_4x16x32x16_f64/numpy | 2.084 / 2.375 | 2.380 / 3.121 | unavailable | unavailable | unavailable |
| matmul_sum_backward_32x16x8_f64/candle | 6.583 / 8.167 | 6.181 / 6.778 | 113 | 27980 | unavailable |
| matmul_sum_backward_32x16x8_f64/kernix | 3.000 / 3.042 | 2.972 / 3.584 | 16 | 18376 | unavailable |
| matmul_transposed_128_f32/kernix | 58.291 / 165.917 | 58.488 / 94.517 | 3 | 163840 | unavailable |
| matmul_transposed_128_f32/numpy | 5.042 / 5.625 | 5.240 / 9.223 | unavailable | unavailable | unavailable |
| matmul_transposed_128_f64/kernix | 105.833 / 126.834 | 97.342 / 113.404 | 3 | 327680 | unavailable |
| matmul_transposed_128_f64/numpy | 15.708 / 16.292 | 13.930 / 15.952 | unavailable | unavailable | unavailable |
| min_norm_32x64_rank32_f64/kernix | 73.583 / 200.416 | 76.491 / 173.671 | 20 | 59136 | unavailable |
| min_norm_32x64_rank32_f64/nalgebra | 155.000 / 176.167 | 168.385 / 378.302 | 15 | 42232 | unavailable |
| min_norm_32x64_rank32_f64/numpy | 136.417 / 147.542 | 134.577 / 139.435 | unavailable | unavailable | unavailable |
| min_norm_64x32_rank16_f64/kernix | 54.375 / 75.417 | 54.132 / 73.718 | 20 | 74752 | unavailable |
| min_norm_64x32_rank16_f64/nalgebra | 72.458 / 93.500 | 78.969 / 86.211 | 15 | 41976 | unavailable |
| min_norm_64x32_rank16_f64/numpy | 79.542 / 80.125 | 78.310 / 101.822 | unavailable | unavailable | unavailable |
| parallel_matmul_into_128_f32/kernix | 47.500 / 87.000 | 45.710 / 48.191 | 2 | 196608 | unavailable |
| parallel_matmul_into_128_f64/kernix | 152.791 / 305.917 | 190.371 / 206.001 | 2 | 393216 | unavailable |
| parallel_matmul_into_256_f64/kernix | 1184.417 / 2119.125 | 1026.796 / 1776.035 | 2 | 1310720 | unavailable |
| parallel_matmul_into_64_f64/kernix | 50.583 / 100.667 | 47.430 / 62.950 | 2 | 98304 | unavailable |
| pivoted_qr_256x64_f64/faer | 326.958 / 363.583 | 319.573 / 376.685 | 58 | 336904 | unavailable |
| pivoted_qr_256x64_f64/kernix | 533.333 / 547.042 | 536.439 / 538.127 | 14 | 559104 | unavailable |
| pivoted_qr_256x64_f64/nalgebra | 1064.500 / 1270.458 | 1157.764 / 1304.260 | 7 | 296960 | unavailable |
| pivoted_qr_96x32_f64/faer | 124.334 / 160.042 | 54.488 / 102.387 | 32 | 94728 | unavailable |
| pivoted_qr_96x32_f64/kernix | 77.542 / 169.917 | 88.809 / 140.072 | 14 | 107520 | unavailable |
| pivoted_qr_96x32_f64/nalgebra | 96.542 / 112.083 | 106.842 / 152.726 | 7 | 58368 | unavailable |
| qr_1024x256_f64/faer | 12638.334 / 20334.125 | 15821.041 / 28083.791 | 764 | 9166852 | unavailable |
| qr_1024x256_f64/kernix | 15596.583 / 17163.917 | 16061.000 / 16810.875 | 51 | 7276544 | unavailable |
| qr_1024x256_f64/nalgebra | 24327.084 / 29117.625 | 25845.375 / 31541.792 | 4 | 4720640 | unavailable |
| qr_1024x256_f64/numpy | 13289.792 / 14570.958 | 12485.292 / 14357.209 | unavailable | unavailable | unavailable |
| qr_256x64_f64/faer | 285.583 / 298.583 | 287.057 / 291.760 | 83 | 335880 | unavailable |
| qr_256x64_f64/kernix | 563.250 / 867.584 | 600.706 / 969.477 | 6 | 426496 | unavailable |
| qr_256x64_f64/nalgebra | 331.750 / 397.667 | 385.854 / 404.260 | 4 | 295424 | unavailable |
| qr_256x64_f64/numpy | 373.667 / 431.792 | 403.447 / 717.592 | unavailable | unavailable | unavailable |
| qr_512x128_f64/faer | 1656.917 / 1939.875 | 1771.722 / 1913.688 | 360 | 3334148 | unavailable |
| qr_512x128_f64/kernix | 2764.000 / 3043.417 | 2781.177 / 3132.438 | 31 | 2098176 | unavailable |
| qr_512x128_f64/nalgebra | 2897.042 / 3448.333 | 2822.791 / 2902.969 | 4 | 1180672 | unavailable |
| qr_512x128_f64/numpy | 2239.542 / 2617.625 | 2074.958 / 3124.010 | unavailable | unavailable | unavailable |
| qr_96x32_f64/faer | 50.208 / 63.042 | 44.226 / 48.491 | 41 | 94216 | unavailable |
| qr_96x32_f64/kernix | 55.333 / 57.167 | 56.075 / 59.242 | 6 | 82176 | unavailable |
| qr_96x32_f64/nalgebra | 45.250 / 54.958 | 45.776 / 49.208 | 4 | 57600 | unavailable |
| qr_96x32_f64/numpy | 46.958 / 60.666 | 55.022 / 61.108 | unavailable | unavailable | unavailable |
| softmax_256x64_f64/candle | 75.625 / 155.917 | 119.258 / 225.768 | 37 | 265200 | unavailable |
| softmax_256x64_f64/kernix | 84.541 / 96.792 | 74.543 / 76.407 | 1 | 131072 | unavailable |
| softmax_256x64_f64/ndarray | 66.000 / 228.875 | 74.103 / 253.602 | 1 | 131072 | unavailable |
| softmax_256x64_f64/numpy | 68.958 / 76.375 | 72.647 / 105.915 | unavailable | unavailable | unavailable |
| solve_cholesky_128x1_f64/faer | 5.666 / 16.458 | 5.546 / 8.862 | 1 | 1024 | unavailable |
| solve_cholesky_128x1_f64/kernix | 18.958 / 31.958 | 16.599 / 19.229 | 1 | 1024 | unavailable |
| solve_cholesky_128x1_f64/nalgebra | 4.417 / 5.250 | 4.912 / 5.277 | 1 | 1024 | unavailable |
| solve_cholesky_256x64_f64/faer | 297.292 / 329.583 | 310.701 / 330.978 | 53 | 786440 | unavailable |
| solve_cholesky_256x64_f64/kernix | 1252.875 / 1364.875 | 1325.297 / 1488.155 | 1 | 131072 | unavailable |
| solve_cholesky_256x64_f64/nalgebra | 930.500 / 1158.458 | 991.655 / 1094.596 | 1 | 131072 | unavailable |
| solve_cholesky_512x128_f64/faer | 2234.375 / 2557.875 | 2337.192 / 2558.800 | 205 | 1572872 | unavailable |
| solve_cholesky_512x128_f64/kernix | 13071.542 / 22318.041 | 11009.875 / 14010.708 | 1 | 524288 | unavailable |
| solve_cholesky_512x128_f64/nalgebra | 7049.125 / 8547.916 | 8450.042 / 17580.750 | 1 | 524288 | unavailable |
| solve_lu_256x64_f64/faer | 322.584 / 1030.500 | 345.083 / 461.167 | 46 | 786440 | unavailable |
| solve_lu_256x64_f64/kernix | 1370.666 / 2508.917 | 1190.634 / 1330.609 | 2 | 262144 | unavailable |
| solve_lu_256x64_f64/nalgebra | 951.000 / 1114.666 | 917.936 / 1121.273 | 1 | 131072 | unavailable |
| tensordot_16x32x8_32x8x16_f64/kernix | 64.458 / 76.917 | 68.129 / 83.509 | 9 | 2166 | unavailable |
| tensordot_16x32x8_32x8x16_f64/numpy | 5.833 / 6.292 | 5.110 / 6.150 | unavailable | unavailable | unavailable |
| training_backward_2x4x8x4_f64/candle | 19.125 / 21.292 | 16.411 / 18.531 | 476 | 21284 | unavailable |
| training_backward_2x4x8x4_f64/kernix | 3.667 / 4.584 | 3.343 / 3.819 | 23 | 4224 | unavailable |
| training_forward_2x4x8x4_f64/candle | 6.458 / 8.667 | 4.837 / 6.324 | 126 | 9568 | unavailable |
| training_forward_2x4x8x4_f64/kernix | 2.084 / 2.209 | 2.059 / 2.218 | 17 | 1032 | unavailable |
| training_record_2x4x8x4_f64/candle | 5.458 / 5.500 | 5.236 / 5.633 | 149 | 11544 | unavailable |
| training_record_2x4x8x4_f64/kernix | 3.458 / 4.292 | 3.951 / 8.275 | 22 | 6832 | unavailable |
| training_sgd_2x4x8x4_f64/kernix | 0.750 / 0.959 | 0.760 / 0.802 | 5 | 688 | unavailable |
| training_shared_backward_64x32_f64/candle | 4.541 / 5.750 | 3.784 / 4.783 | 58 | 99996 | unavailable |
| training_shared_backward_64x32_f64/kernix | 6.375 / 8.125 | 8.281 / 12.426 | 8 | 49768 | unavailable |

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
