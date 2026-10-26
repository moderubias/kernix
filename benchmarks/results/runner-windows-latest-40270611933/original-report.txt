# Clean CPU evidence: runner-windows-latest-40270611933

Measured source `90cbbff60e5d7458ef9f89918c99a6cca94b51d1`, tree `d806b9704a44b4bf2c4c04e664fe3c60eeebc177`; dirty before/after: false.
Library source: workspace 90cbbff60e5d7458ef9f89918c99a6cca94b51d1. CPU affinity: not pinned. Dtype explicitly named per workload; row-major inputs.

| Workload / library | Per-call p50 / p95 (us) | Batch-mean p50 / p95 (us) | Rust allocations | Peak additional requested bytes | Process peak RSS (MiB) |
|---|---:|---:|---:|---:|---:|
| cholesky_factor_128_f64/faer | 58.800 / 61.800 | 59.939 / 67.181 | 2 | 132096 | unavailable |
| cholesky_factor_128_f64/kernix | 221.600 / 240.200 | 244.783 / 256.474 | 1 | 131072 | unavailable |
| cholesky_factor_128_f64/nalgebra | 117.900 / 144.800 | 120.183 / 121.855 | 1 | 131072 | unavailable |
| cholesky_factor_128_f64/numpy | 113.700 / 143.200 | 115.647 / 116.048 | unavailable | unavailable | unavailable |
| cholesky_factor_256_f64/faer | 317.800 / 458.300 | 357.354 / 429.833 | 2 | 526336 | unavailable |
| cholesky_factor_256_f64/kernix | 690.100 / 728.300 | 700.907 / 1100.350 | 49 | 645120 | unavailable |
| cholesky_factor_256_f64/nalgebra | 869.800 / 1149.400 | 861.550 / 1432.771 | 1 | 524288 | unavailable |
| cholesky_factor_256_f64/numpy | 937.800 / 1129.800 | 923.500 / 1023.360 | unavailable | unavailable | unavailable |
| cholesky_factor_512_f64/faer | 2907.500 / 3191.500 | 2945.925 / 3096.900 | 2 | 2101248 | unavailable |
| cholesky_factor_512_f64/kernix | 4606.700 / 6492.900 | 4540.333 / 4780.767 | 161 | 2269184 | unavailable |
| cholesky_factor_512_f64/nalgebra | 8140.700 / 8701.800 | 8151.250 / 8477.150 | 1 | 2097152 | unavailable |
| cholesky_factor_512_f64/numpy | 6342.100 / 6651.900 | 6316.500 / 6445.350 | unavailable | unavailable | unavailable |
| cholesky_factor_64_f64/faer | 14.500 / 14.700 | 14.687 / 15.150 | 2 | 33280 | unavailable |
| cholesky_factor_64_f64/kernix | 35.800 / 36.100 | 36.161 / 38.071 | 2 | 33280 | unavailable |
| cholesky_factor_64_f64/nalgebra | 20.300 / 20.400 | 20.657 / 20.932 | 1 | 32768 | unavailable |
| cholesky_factor_64_f64/numpy | 21.100 / 22.500 | 21.793 / 23.270 | unavailable | unavailable | unavailable |
| cholesky_rank_downdate_256_f64/kernix | 200.900 / 356.800 | 173.159 / 211.648 | 3 | 526336 | unavailable |
| cholesky_rank_downdate_256_f64/nalgebra | 421.900 / 563.600 | 428.808 / 582.817 | 2 | 526336 | unavailable |
| cholesky_rank_downdate_256_f64/refactor | 1098.400 / 1172.400 | 979.627 / 1061.520 | 49 | 645120 | unavailable |
| cholesky_rank_update_256_f64/kernix | 168.500 / 191.000 | 172.497 / 181.134 | 3 | 526336 | unavailable |
| cholesky_rank_update_256_f64/nalgebra | 409.700 / 451.100 | 415.876 / 439.374 | 2 | 526336 | unavailable |
| cholesky_rank_update_256_f64/refactor | 1096.800 / 1550.500 | 1130.278 / 1532.157 | 49 | 645120 | unavailable |
| cholesky_rank_update_512_f64/kernix | 1368.100 / 1952.600 | 1376.100 / 1952.617 | 3 | 2101248 | unavailable |
| cholesky_rank_update_512_f64/nalgebra | 551.800 / 603.000 | 567.222 / 580.044 | 2 | 2101248 | unavailable |
| cholesky_rank_update_512_f64/refactor | 4768.900 / 6644.900 | 4714.933 / 4809.833 | 161 | 2269184 | unavailable |
| create_ones_64x64_f32/candle | 0.700 / 0.700 | 0.700 / 0.707 | 7 | 16696 | unavailable |
| create_ones_64x64_f32/faer | 0.500 / 0.500 | 0.451 / 0.453 | 1 | 16384 | unavailable |
| create_ones_64x64_f32/kernix | 0.500 / 0.500 | 0.477 / 0.481 | 1 | 16384 | unavailable |
| create_ones_64x64_f32/ndarray | 0.400 / 0.400 | 0.391 / 0.414 | 1 | 16384 | unavailable |
| create_ones_64x64_f32/numpy | 2.100 / 2.200 | 2.249 / 3.602 | unavailable | unavailable | unavailable |
| create_zeros_64x64_f32/candle | 0.500 / 0.800 | 0.523 / 0.813 | 5 | 16648 | unavailable |
| create_zeros_64x64_f32/faer | 0.500 / 0.500 | 0.479 / 0.495 | 1 | 16384 | unavailable |
| create_zeros_64x64_f32/kernix | 0.400 / 0.400 | 0.372 / 0.414 | 1 | 16384 | unavailable |
| create_zeros_64x64_f32/ndarray | 0.300 / 0.400 | 0.277 / 0.278 | 1 | 16384 | unavailable |
| create_zeros_64x64_f32/numpy | 0.800 / 0.900 | 0.918 / 0.976 | unavailable | unavailable | unavailable |
| matmul_128_f32/candle | 72.200 / 109.400 | 67.612 / 108.811 | 10 | 65816 | unavailable |
| matmul_128_f32/faer | 52.900 / 82.700 | 57.142 / 61.903 | 1 | 65536 | unavailable |
| matmul_128_f32/kernix | 70.900 / 81.400 | 71.621 / 80.649 | 3 | 65544 | unavailable |
| matmul_128_f32/nalgebra | 53.600 / 54.500 | 54.332 / 57.768 | 2 | 163840 | unavailable |
| matmul_128_f32/ndarray | 109.900 / 140.100 | 118.792 / 193.563 | 2 | 163840 | unavailable |
| matmul_128_f32/numpy | 56.200 / 59.200 | 57.353 / 57.850 | unavailable | unavailable | unavailable |
| matmul_128_f64/candle | 139.100 / 174.100 | 140.671 / 151.561 | 10 | 131352 | unavailable |
| matmul_128_f64/faer | 111.000 / 138.200 | 113.407 / 127.306 | 1 | 131072 | unavailable |
| matmul_128_f64/kernix | 136.100 / 173.200 | 138.841 / 156.338 | 3 | 131080 | unavailable |
| matmul_128_f64/nalgebra | 113.800 / 185.600 | 115.405 / 116.947 | 2 | 327680 | unavailable |
| matmul_128_f64/ndarray | 117.400 / 320.500 | 138.400 / 214.591 | 2 | 327680 | unavailable |
| matmul_128_f64/numpy | 100.900 / 127.300 | 103.103 / 136.415 | unavailable | unavailable | unavailable |
| matmul_256_f32/candle | 451.100 / 794.300 | 463.259 / 915.918 | 10 | 262424 | unavailable |
| matmul_256_f32/faer | 421.900 / 469.200 | 426.788 / 434.333 | 1 | 262144 | unavailable |
| matmul_256_f32/kernix | 425.100 / 461.000 | 432.500 / 439.400 | 3 | 262152 | unavailable |
| matmul_256_f32/nalgebra | 394.100 / 421.700 | 400.480 / 413.556 | 2 | 589824 | unavailable |
| matmul_256_f32/ndarray | 389.100 / 415.200 | 394.669 / 434.939 | 2 | 589824 | unavailable |
| matmul_256_f32/numpy | 386.500 / 418.200 | 392.819 / 427.867 | unavailable | unavailable | unavailable |
| matmul_256_f64/candle | 855.400 / 1197.500 | 903.850 / 1223.833 | 10 | 524568 | unavailable |
| matmul_256_f64/faer | 1132.300 / 1627.300 | 1121.056 / 1373.550 | 1 | 524288 | unavailable |
| matmul_256_f64/kernix | 855.400 / 969.400 | 862.083 / 942.627 | 3 | 524296 | unavailable |
| matmul_256_f64/nalgebra | 956.200 / 1542.000 | 924.091 / 1088.240 | 2 | 1179648 | unavailable |
| matmul_256_f64/ndarray | 1144.800 / 1295.800 | 1150.933 / 1308.888 | 2 | 1179648 | unavailable |
| matmul_256_f64/numpy | 756.800 / 789.900 | 769.414 / 1290.463 | unavailable | unavailable | unavailable |
| matmul_32_f64/candle | 2.500 / 2.600 | 2.592 / 2.632 | 10 | 8528 | unavailable |
| matmul_32_f64/faer | 2.500 / 2.600 | 2.496 / 2.550 | 1 | 8192 | unavailable |
| matmul_32_f64/kernix | 2.800 / 2.900 | 2.845 / 3.697 | 3 | 24576 | unavailable |
| matmul_32_f64/nalgebra | 2.400 / 2.400 | 2.411 / 2.473 | 2 | 24576 | unavailable |
| matmul_32_f64/ndarray | 2.400 / 2.500 | 2.433 / 2.440 | 2 | 24576 | unavailable |
| matmul_32_f64/numpy | 4.200 / 4.200 | 4.362 / 4.958 | unavailable | unavailable | unavailable |
| matmul_64_f32/candle | 7.400 / 7.500 | 7.267 / 7.420 | 10 | 16664 | unavailable |
| matmul_64_f32/faer | 7.200 / 7.300 | 7.332 / 7.795 | 1 | 16384 | unavailable |
| matmul_64_f32/kernix | 6.900 / 7.000 | 6.841 / 7.381 | 3 | 16512 | unavailable |
| matmul_64_f32/nalgebra | 7.800 / 14.000 | 10.897 / 25.754 | 2 | 49152 | unavailable |
| matmul_64_f32/ndarray | 26.100 / 36.100 | 9.443 / 29.719 | 2 | 49152 | unavailable |
| matmul_64_f32/numpy | 10.400 / 10.600 | 10.690 / 11.051 | unavailable | unavailable | unavailable |
| matmul_64_f64/candle | 14.100 / 14.300 | 14.112 / 14.430 | 10 | 33048 | unavailable |
| matmul_64_f64/faer | 14.500 / 14.600 | 14.783 / 19.686 | 1 | 32768 | unavailable |
| matmul_64_f64/kernix | 13.600 / 13.800 | 13.731 / 21.498 | 3 | 32896 | unavailable |
| matmul_64_f64/nalgebra | 52.700 / 69.400 | 31.306 / 47.336 | 2 | 98304 | unavailable |
| matmul_64_f64/ndarray | 52.100 / 75.300 | 54.547 / 62.176 | 2 | 98304 | unavailable |
| matmul_64_f64/numpy | 15.900 / 17.500 | 16.655 / 19.022 | unavailable | unavailable | unavailable |
| matmul_ab_128_f32/kernix | 71.500 / 90.200 | 73.128 / 134.621 | 3 | 65544 | unavailable |
| matmul_ab_128_f32/numpy | 56.700 / 67.100 | 57.883 / 62.398 | unavailable | unavailable | unavailable |
| matmul_ab_128_f64/kernix | 136.700 / 168.900 | 138.278 / 147.565 | 3 | 131080 | unavailable |
| matmul_ab_128_f64/numpy | 101.100 / 131.200 | 104.102 / 138.392 | unavailable | unavailable | unavailable |
| matmul_batched_4x16x32x16_f32/candle | 3.000 / 3.100 | 3.428 / 4.901 | 26 | 13152 | unavailable |
| matmul_batched_4x16x32x16_f32/kernix | 1.400 / 1.500 | 1.441 / 1.570 | 3 | 4104 | unavailable |
| matmul_batched_4x16x32x16_f32/numpy | 4.800 / 4.900 | 5.011 / 5.033 | unavailable | unavailable | unavailable |
| matmul_batched_4x16x32x16_f64/candle | 7.500 / 7.700 | 4.334 / 7.736 | 26 | 25440 | unavailable |
| matmul_batched_4x16x32x16_f64/kernix | 4.500 / 4.700 | 2.404 / 3.537 | 3 | 8200 | unavailable |
| matmul_batched_4x16x32x16_f64/numpy | 5.000 / 5.100 | 5.236 / 7.659 | unavailable | unavailable | unavailable |
| matmul_sum_backward_32x16x8_f64/candle | 9.800 / 10.100 | 9.645 / 11.550 | 116 | 27988 | unavailable |
| matmul_sum_backward_32x16x8_f64/kernix | 4.200 / 4.800 | 4.648 / 7.573 | 15 | 18376 | unavailable |
| matmul_transposed_128_f32/kernix | 92.500 / 123.800 | 94.574 / 112.878 | 4 | 458760 | unavailable |
| matmul_transposed_128_f32/numpy | 60.300 / 78.500 | 62.184 / 63.047 | unavailable | unavailable | unavailable |
| matmul_transposed_128_f64/kernix | 191.000 / 227.600 | 192.854 / 200.096 | 4 | 917512 | unavailable |
| matmul_transposed_128_f64/numpy | 107.200 / 135.100 | 109.268 / 129.260 | unavailable | unavailable | unavailable |
| min_norm_32x64_rank32_f64/kernix | 110.700 / 142.300 | 86.101 / 115.753 | 20 | 59136 | unavailable |
| min_norm_32x64_rank32_f64/nalgebra | 177.300 / 207.500 | 180.682 / 190.813 | 15 | 42232 | unavailable |
| min_norm_32x64_rank32_f64/numpy | 206.100 / 253.900 | 211.329 / 225.224 | unavailable | unavailable | unavailable |
| min_norm_64x32_rank16_f64/kernix | 91.500 / 149.500 | 65.299 / 77.007 | 20 | 74752 | unavailable |
| min_norm_64x32_rank16_f64/nalgebra | 104.400 / 130.400 | 105.236 / 113.919 | 15 | 41976 | unavailable |
| min_norm_64x32_rank16_f64/numpy | 200.900 / 247.600 | 141.658 / 209.565 | unavailable | unavailable | unavailable |
| parallel_matmul_into_128_f32/kernix | 60.900 / 84.200 | 62.612 / 66.726 | 2 | 16 | unavailable |
| parallel_matmul_into_128_f64/kernix | 96.700 / 161.400 | 106.895 / 121.120 | 2 | 16 | unavailable |
| parallel_matmul_into_256_f64/kernix | 446.600 / 765.800 | 547.479 / 800.408 | 2 | 16 | unavailable |
| parallel_matmul_into_64_f64/kernix | 39.700 / 72.400 | 62.189 / 82.762 | 2 | 98304 | unavailable |
| pivoted_qr_256x64_f64/faer | 420.900 / 679.500 | 435.404 / 599.959 | 10 | 333312 | unavailable |
| pivoted_qr_256x64_f64/kernix | 913.600 / 1017.800 | 917.764 / 976.809 | 14 | 559104 | unavailable |
| pivoted_qr_256x64_f64/nalgebra | 935.100 / 997.200 | 932.500 / 978.755 | 7 | 296960 | unavailable |
| pivoted_qr_96x32_f64/faer | 69.000 / 89.900 | 64.819 / 80.678 | 10 | 68352 | unavailable |
| pivoted_qr_96x32_f64/kernix | 137.700 / 212.800 | 131.329 / 199.690 | 14 | 107520 | unavailable |
| pivoted_qr_96x32_f64/nalgebra | 105.500 / 135.900 | 107.015 / 114.343 | 7 | 58368 | unavailable |
| qr_1024x256_f64/faer | 12635.700 / 13696.900 | 12515.900 / 12577.200 | 7 | 5308416 | unavailable |
| qr_1024x256_f64/kernix | 22884.800 / 31510.100 | 20449.800 / 25854.200 | 82 | 7145480 | unavailable |
| qr_1024x256_f64/nalgebra | 33780.400 / 42830.900 | 34338.400 / 43176.700 | 4 | 4720640 | unavailable |
| qr_1024x256_f64/numpy | 20803.000 / 21819.900 | 21055.800 / 22973.500 | unavailable | unavailable | unavailable |
| qr_256x64_f64/faer | 465.200 / 770.800 | 419.100 / 596.918 | 7 | 331776 | unavailable |
| qr_256x64_f64/kernix | 845.100 / 1422.700 | 1246.356 / 2140.340 | 6 | 426496 | unavailable |
| qr_256x64_f64/nalgebra | 642.900 / 681.900 | 645.888 / 1133.578 | 4 | 295424 | unavailable |
| qr_256x64_f64/numpy | 548.600 / 676.400 | 557.211 / 785.308 | unavailable | unavailable | unavailable |
| qr_512x128_f64/faer | 2120.200 / 3110.500 | 2073.160 / 2882.125 | 7 | 1343488 | unavailable |
| qr_512x128_f64/kernix | 3975.500 / 4530.300 | 3692.267 / 4057.233 | 49 | 2294792 | unavailable |
| qr_512x128_f64/nalgebra | 4390.900 / 4527.100 | 4400.500 / 4493.567 | 4 | 1180672 | unavailable |
| qr_512x128_f64/numpy | 4093.500 / 5131.000 | 4397.667 / 5085.250 | unavailable | unavailable | unavailable |
| qr_96x32_f64/faer | 83.000 / 100.300 | 68.514 / 88.879 | 7 | 67584 | unavailable |
| qr_96x32_f64/kernix | 111.100 / 170.500 | 101.786 / 111.331 | 6 | 82176 | unavailable |
| qr_96x32_f64/nalgebra | 79.300 / 97.700 | 77.531 / 84.757 | 4 | 57600 | unavailable |
| qr_96x32_f64/numpy | 127.200 / 165.000 | 78.710 / 80.751 | unavailable | unavailable | unavailable |
| softmax_256x64_f64/candle | 181.700 / 220.800 | 131.819 / 148.200 | 37 | 265200 | unavailable |
| softmax_256x64_f64/kernix | 114.000 / 143.300 | 116.829 / 135.795 | 1 | 131072 | unavailable |
| softmax_256x64_f64/ndarray | 153.700 / 187.500 | 162.621 / 278.794 | 1 | 131072 | unavailable |
| softmax_256x64_f64/numpy | 133.200 / 232.100 | 172.722 / 231.745 | unavailable | unavailable | unavailable |
| solve_cholesky_128x1_f64/faer | 6.600 / 11.500 | 6.544 / 10.855 | 1 | 1024 | unavailable |
| solve_cholesky_128x1_f64/kernix | 18.100 / 18.600 | 18.954 / 19.481 | 1 | 1024 | unavailable |
| solve_cholesky_128x1_f64/nalgebra | 6.400 / 6.500 | 6.011 / 7.797 | 1 | 1024 | unavailable |
| solve_cholesky_256x64_f64/faer | 324.500 / 365.400 | 328.745 / 347.120 | 1 | 131072 | unavailable |
| solve_cholesky_256x64_f64/kernix | 1282.600 / 4854.800 | 1855.900 / 2525.900 | 1 | 131072 | unavailable |
| solve_cholesky_256x64_f64/nalgebra | 1390.700 / 1925.100 | 1378.412 / 2263.700 | 1 | 131072 | unavailable |
| solve_cholesky_512x128_f64/faer | 2268.500 / 3017.700 | 2270.180 / 2792.175 | 1 | 524288 | unavailable |
| solve_cholesky_512x128_f64/kernix | 9497.700 / 10193.000 | 9537.800 / 10198.500 | 1 | 524288 | unavailable |
| solve_cholesky_512x128_f64/nalgebra | 10940.400 / 14358.200 | 10598.100 / 14203.300 | 1 | 524288 | unavailable |
| solve_lu_256x64_f64/faer | 362.900 / 401.600 | 372.207 / 407.620 | 2 | 262144 | unavailable |
| solve_lu_256x64_f64/kernix | 1289.500 / 1315.000 | 1288.237 / 1425.562 | 2 | 262144 | unavailable |
| solve_lu_256x64_f64/nalgebra | 1196.900 / 1229.200 | 1190.600 / 1244.700 | 1 | 131072 | unavailable |
| tensordot_16x32x8_32x8x16_f64/kernix | 64.100 / 66.200 | 65.019 / 72.367 | 9 | 2166 | unavailable |
| tensordot_16x32x8_32x8x16_f64/numpy | 12.000 / 13.500 | 18.987 / 28.913 | unavailable | unavailable | unavailable |
| training_backward_2x4x8x4_f64/candle | 32.000 / 33.400 | 32.218 / 35.427 | 474 | 8332 | unavailable |
| training_backward_2x4x8x4_f64/kernix | 5.900 / 6.000 | 5.937 / 6.023 | 23 | 4224 | unavailable |
| training_forward_2x4x8x4_f64/candle | 15.400 / 16.100 | 8.247 / 9.972 | 127 | 3912 | unavailable |
| training_forward_2x4x8x4_f64/kernix | 3.500 / 3.600 | 3.637 / 6.006 | 17 | 1032 | unavailable |
| training_record_2x4x8x4_f64/candle | 10.200 / 10.800 | 10.207 / 10.393 | 150 | 10712 | unavailable |
| training_record_2x4x8x4_f64/kernix | 4.900 / 5.000 | 4.770 / 4.782 | 22 | 6832 | unavailable |
| training_sgd_2x4x8x4_f64/kernix | 1.000 / 1.000 | 1.001 / 1.045 | 5 | 688 | unavailable |
| training_shared_backward_64x32_f64/candle | 58.000 / 113.300 | 8.212 / 10.675 | 58 | 100004 | unavailable |
| training_shared_backward_64x32_f64/kernix | 10.500 / 10.600 | 10.569 / 10.913 | 8 | 49768 | unavailable |

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
