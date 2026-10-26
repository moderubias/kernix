# Clean CPU evidence: baseline-windows-latest-40289723446

Measured source `7c11b2165b0cb7b523db4d938a36f1396396a138`, tree `cc7ac522fccee7e655adb7ca8d3644d95344fcb1`; dirty before/after: false.
Library source: git v0.12.0 90cbbff60e5d7458ef9f89918c99a6cca94b51d1. CPU affinity: not pinned. Dtype explicitly named per workload; row-major inputs.

| Workload / library | Per-call p50 / p95 (us) | Batch-mean p50 / p95 (us) | Rust allocations | Peak additional requested bytes | Process peak RSS (MiB) |
|---|---:|---:|---:|---:|---:|
| cholesky_factor_128_f64/faer | 59.400 / 74.100 | 60.297 / 60.812 | 2 | 132096 | unavailable |
| cholesky_factor_128_f64/kernix | 221.900 / 249.500 | 224.780 / 231.830 | 1 | 131072 | unavailable |
| cholesky_factor_128_f64/nalgebra | 121.800 / 139.200 | 124.735 / 213.077 | 1 | 131072 | unavailable |
| cholesky_factor_128_f64/numpy | 112.900 / 142.700 | 115.205 / 132.525 | unavailable | unavailable | unavailable |
| cholesky_factor_256_f64/faer | 311.400 / 391.700 | 318.141 / 334.507 | 2 | 526336 | unavailable |
| cholesky_factor_256_f64/kernix | 697.000 / 1120.500 | 699.433 / 708.187 | 49 | 645120 | unavailable |
| cholesky_factor_256_f64/nalgebra | 873.500 / 914.400 | 878.867 / 920.400 | 1 | 524288 | unavailable |
| cholesky_factor_256_f64/numpy | 988.400 / 1052.800 | 1002.160 / 1164.367 | unavailable | unavailable | unavailable |
| cholesky_factor_512_f64/faer | 2890.900 / 3011.300 | 2888.300 / 2985.175 | 2 | 2101248 | unavailable |
| cholesky_factor_512_f64/kernix | 4636.600 / 6584.200 | 4595.800 / 5441.000 | 161 | 2269184 | unavailable |
| cholesky_factor_512_f64/nalgebra | 8270.200 / 8764.600 | 8307.850 / 12125.500 | 1 | 2097152 | unavailable |
| cholesky_factor_512_f64/numpy | 6183.100 / 6764.800 | 6198.350 / 6292.050 | unavailable | unavailable | unavailable |
| cholesky_factor_64_f64/faer | 14.600 / 14.700 | 14.711 / 14.829 | 2 | 33280 | unavailable |
| cholesky_factor_64_f64/kernix | 35.800 / 36.100 | 36.116 / 38.516 | 2 | 33280 | unavailable |
| cholesky_factor_64_f64/nalgebra | 20.600 / 20.700 | 21.003 / 22.115 | 1 | 32768 | unavailable |
| cholesky_factor_64_f64/numpy | 20.800 / 27.000 | 21.489 / 22.283 | unavailable | unavailable | unavailable |
| cholesky_rank_downdate_256_f64/kernix | 168.100 / 198.400 | 171.792 / 295.344 | 3 | 526336 | unavailable |
| cholesky_rank_downdate_256_f64/nalgebra | 399.600 / 436.500 | 405.884 / 553.221 | 2 | 526336 | unavailable |
| cholesky_rank_downdate_256_f64/refactor | 1112.300 / 1271.400 | 978.518 / 1073.990 | 49 | 645120 | unavailable |
| cholesky_rank_update_256_f64/kernix | 171.400 / 203.800 | 175.366 / 179.286 | 3 | 526336 | unavailable |
| cholesky_rank_update_256_f64/nalgebra | 419.400 / 455.300 | 429.721 / 505.045 | 2 | 526336 | unavailable |
| cholesky_rank_update_256_f64/refactor | 1118.000 / 1173.700 | 1011.730 / 1084.140 | 49 | 645120 | unavailable |
| cholesky_rank_update_512_f64/kernix | 1351.300 / 1720.000 | 1351.600 / 1404.250 | 3 | 2101248 | unavailable |
| cholesky_rank_update_512_f64/nalgebra | 559.800 / 603.500 | 569.572 / 581.106 | 2 | 2101248 | unavailable |
| cholesky_rank_update_512_f64/refactor | 4671.400 / 6593.200 | 4623.667 / 4674.667 | 161 | 2269184 | unavailable |
| create_ones_64x64_f32/candle | 0.700 / 0.700 | 0.700 / 0.715 | 7 | 16696 | unavailable |
| create_ones_64x64_f32/faer | 0.500 / 0.500 | 0.455 / 0.462 | 1 | 16384 | unavailable |
| create_ones_64x64_f32/kernix | 0.500 / 0.500 | 0.476 / 0.477 | 1 | 16384 | unavailable |
| create_ones_64x64_f32/ndarray | 0.400 / 0.400 | 0.388 / 0.397 | 1 | 16384 | unavailable |
| create_ones_64x64_f32/numpy | 2.100 / 2.200 | 2.207 / 2.549 | unavailable | unavailable | unavailable |
| create_zeros_64x64_f32/candle | 0.500 / 0.500 | 0.477 / 0.806 | 5 | 16648 | unavailable |
| create_zeros_64x64_f32/faer | 0.500 / 0.500 | 0.452 / 0.458 | 1 | 16384 | unavailable |
| create_zeros_64x64_f32/kernix | 0.400 / 0.400 | 0.372 / 0.381 | 1 | 16384 | unavailable |
| create_zeros_64x64_f32/ndarray | 0.300 / 0.500 | 0.282 / 27.856 | 1 | 16384 | unavailable |
| create_zeros_64x64_f32/numpy | 0.800 / 0.900 | 0.899 / 0.915 | unavailable | unavailable | unavailable |
| matmul_128_f32/candle | 75.100 / 79.100 | 67.729 / 85.564 | 10 | 65816 | unavailable |
| matmul_128_f32/faer | 50.100 / 52.700 | 51.252 / 54.406 | 1 | 65536 | unavailable |
| matmul_128_f32/kernix | 75.500 / 94.200 | 76.801 / 122.444 | 3 | 65544 | unavailable |
| matmul_128_f32/nalgebra | 53.500 / 63.400 | 54.303 / 79.416 | 2 | 163840 | unavailable |
| matmul_128_f32/ndarray | 123.600 / 142.100 | 126.866 / 134.189 | 2 | 163840 | unavailable |
| matmul_128_f32/numpy | 56.100 / 67.500 | 57.381 / 57.949 | unavailable | unavailable | unavailable |
| matmul_128_f64/candle | 141.200 / 175.800 | 144.381 / 149.769 | 10 | 131352 | unavailable |
| matmul_128_f64/faer | 103.100 / 130.600 | 107.860 / 180.762 | 1 | 131072 | unavailable |
| matmul_128_f64/kernix | 96.000 / 123.800 | 97.704 / 98.215 | 3 | 131080 | unavailable |
| matmul_128_f64/nalgebra | 115.700 / 141.000 | 117.481 / 120.887 | 2 | 327680 | unavailable |
| matmul_128_f64/ndarray | 114.600 / 140.200 | 116.506 / 120.987 | 2 | 327680 | unavailable |
| matmul_128_f64/numpy | 100.000 / 117.200 | 102.710 / 185.820 | unavailable | unavailable | unavailable |
| matmul_256_f32/candle | 427.600 / 475.500 | 437.239 / 744.407 | 10 | 262424 | unavailable |
| matmul_256_f32/faer | 409.800 / 689.600 | 416.292 / 437.400 | 1 | 262144 | unavailable |
| matmul_256_f32/kernix | 427.300 / 479.300 | 433.538 / 440.596 | 3 | 262152 | unavailable |
| matmul_256_f32/nalgebra | 395.000 / 463.200 | 401.572 / 407.104 | 2 | 589824 | unavailable |
| matmul_256_f32/ndarray | 389.700 / 423.000 | 396.054 / 401.652 | 2 | 589824 | unavailable |
| matmul_256_f32/numpy | 387.100 / 419.500 | 392.027 / 419.946 | unavailable | unavailable | unavailable |
| matmul_256_f64/candle | 870.600 / 911.100 | 876.558 / 1122.067 | 10 | 524568 | unavailable |
| matmul_256_f64/faer | 1086.100 / 1197.400 | 1091.320 / 1262.875 | 1 | 524288 | unavailable |
| matmul_256_f64/kernix | 728.400 / 767.700 | 754.179 / 875.108 | 3 | 524296 | unavailable |
| matmul_256_f64/nalgebra | 909.600 / 934.600 | 909.425 / 974.864 | 2 | 1179648 | unavailable |
| matmul_256_f64/ndarray | 1171.600 / 1201.200 | 1169.400 / 1207.567 | 2 | 1179648 | unavailable |
| matmul_256_f64/numpy | 758.000 / 794.200 | 769.369 / 1069.080 | unavailable | unavailable | unavailable |
| matmul_32_f64/candle | 2.600 / 2.700 | 2.604 / 2.691 | 10 | 8528 | unavailable |
| matmul_32_f64/faer | 2.500 / 2.600 | 2.450 / 3.197 | 1 | 8192 | unavailable |
| matmul_32_f64/kernix | 2.800 / 2.800 | 2.821 / 2.848 | 3 | 24576 | unavailable |
| matmul_32_f64/nalgebra | 2.300 / 2.400 | 2.368 / 2.435 | 2 | 24576 | unavailable |
| matmul_32_f64/ndarray | 2.400 / 2.400 | 2.387 / 2.400 | 2 | 24576 | unavailable |
| matmul_32_f64/numpy | 4.200 / 4.400 | 4.408 / 4.508 | unavailable | unavailable | unavailable |
| matmul_64_f32/candle | 7.400 / 7.500 | 7.237 / 7.391 | 10 | 16664 | unavailable |
| matmul_64_f32/faer | 7.100 / 7.200 | 7.347 / 7.707 | 1 | 16384 | unavailable |
| matmul_64_f32/kernix | 7.000 / 7.200 | 7.065 / 7.153 | 3 | 16512 | unavailable |
| matmul_64_f32/nalgebra | 7.700 / 7.800 | 8.734 / 22.881 | 2 | 49152 | unavailable |
| matmul_64_f32/ndarray | 29.800 / 32.000 | 9.571 / 31.639 | 2 | 49152 | unavailable |
| matmul_64_f32/numpy | 10.500 / 10.600 | 10.675 / 11.185 | unavailable | unavailable | unavailable |
| matmul_64_f64/candle | 14.200 / 14.400 | 14.227 / 14.637 | 10 | 33048 | unavailable |
| matmul_64_f64/faer | 14.500 / 14.700 | 14.606 / 15.093 | 1 | 32768 | unavailable |
| matmul_64_f64/kernix | 13.200 / 13.400 | 13.498 / 25.753 | 3 | 32896 | unavailable |
| matmul_64_f64/nalgebra | 57.800 / 60.200 | 33.031 / 50.252 | 2 | 98304 | unavailable |
| matmul_64_f64/ndarray | 58.200 / 69.300 | 59.214 / 60.047 | 2 | 98304 | unavailable |
| matmul_64_f64/numpy | 16.000 / 16.100 | 16.419 / 17.503 | unavailable | unavailable | unavailable |
| matmul_ab_128_f32/kernix | 75.500 / 80.000 | 77.097 / 83.351 | 3 | 65544 | unavailable |
| matmul_ab_128_f32/numpy | 56.400 / 59.900 | 57.916 / 62.221 | unavailable | unavailable | unavailable |
| matmul_ab_128_f64/kernix | 141.800 / 161.700 | 144.760 / 146.249 | 3 | 131080 | unavailable |
| matmul_ab_128_f64/numpy | 101.200 / 118.700 | 103.906 / 104.865 | unavailable | unavailable | unavailable |
| matmul_batched_4x16x32x16_f32/candle | 2.900 / 3.000 | 3.282 / 3.347 | 26 | 13152 | unavailable |
| matmul_batched_4x16x32x16_f32/kernix | 1.400 / 1.500 | 1.509 / 1.526 | 3 | 4104 | unavailable |
| matmul_batched_4x16x32x16_f32/numpy | 4.800 / 4.900 | 5.016 / 5.164 | unavailable | unavailable | unavailable |
| matmul_batched_4x16x32x16_f64/candle | 4.200 / 4.400 | 4.494 / 4.658 | 26 | 25440 | unavailable |
| matmul_batched_4x16x32x16_f64/kernix | 2.400 / 2.500 | 2.417 / 2.636 | 3 | 8200 | unavailable |
| matmul_batched_4x16x32x16_f64/numpy | 5.000 / 5.100 | 5.265 / 5.405 | unavailable | unavailable | unavailable |
| matmul_sum_backward_32x16x8_f64/candle | 9.900 / 10.200 | 9.713 / 10.290 | 116 | 27988 | unavailable |
| matmul_sum_backward_32x16x8_f64/kernix | 4.600 / 4.800 | 4.434 / 391.643 | 15 | 18376 | unavailable |
| matmul_transposed_128_f32/kernix | 98.700 / 153.000 | 89.747 / 135.772 | 4 | 458760 | unavailable |
| matmul_transposed_128_f32/numpy | 60.900 / 73.800 | 62.217 / 67.154 | unavailable | unavailable | unavailable |
| matmul_transposed_128_f64/kernix | 198.800 / 232.000 | 201.834 / 220.546 | 4 | 917512 | unavailable |
| matmul_transposed_128_f64/numpy | 106.800 / 124.500 | 111.149 / 192.000 | unavailable | unavailable | unavailable |
| min_norm_32x64_rank32_f64/kernix | 114.900 / 135.700 | 86.223 / 118.696 | 20 | 59136 | unavailable |
| min_norm_32x64_rank32_f64/nalgebra | 174.800 / 194.000 | 177.772 / 180.293 | 15 | 42232 | unavailable |
| min_norm_32x64_rank32_f64/numpy | 204.300 / 254.600 | 215.287 / 261.782 | unavailable | unavailable | unavailable |
| min_norm_64x32_rank16_f64/kernix | 96.600 / 125.800 | 64.938 / 79.510 | 20 | 74752 | unavailable |
| min_norm_64x32_rank16_f64/nalgebra | 103.500 / 132.500 | 105.362 / 106.783 | 15 | 41976 | unavailable |
| min_norm_64x32_rank16_f64/numpy | 112.700 / 147.400 | 116.497 / 122.014 | unavailable | unavailable | unavailable |
| parallel_matmul_into_128_f32/kernix | 56.900 / 79.900 | 60.400 / 66.074 | 2 | 16 | unavailable |
| parallel_matmul_into_128_f64/kernix | 82.600 / 134.700 | 106.848 / 118.311 | 2 | 16 | unavailable |
| parallel_matmul_into_256_f64/kernix | 441.100 / 772.800 | 518.975 / 792.062 | 2 | 16 | unavailable |
| parallel_matmul_into_64_f64/kernix | 56.100 / 122.500 | 70.452 / 101.552 | 2 | 98304 | unavailable |
| pivoted_qr_256x64_f64/faer | 413.400 / 457.400 | 366.379 / 663.075 | 10 | 333312 | unavailable |
| pivoted_qr_256x64_f64/kernix | 925.800 / 971.600 | 905.258 / 1316.987 | 14 | 559104 | unavailable |
| pivoted_qr_256x64_f64/nalgebra | 906.000 / 949.800 | 914.118 / 925.045 | 7 | 296960 | unavailable |
| pivoted_qr_96x32_f64/faer | 71.900 / 94.200 | 56.079 / 112.854 | 10 | 68352 | unavailable |
| pivoted_qr_96x32_f64/kernix | 125.000 / 144.100 | 132.791 / 139.132 | 14 | 107520 | unavailable |
| pivoted_qr_96x32_f64/nalgebra | 108.300 / 137.900 | 106.949 / 113.367 | 7 | 58368 | unavailable |
| qr_1024x256_f64/faer | 12819.100 / 13513.000 | 12817.200 / 13081.400 | 7 | 5308416 | unavailable |
| qr_1024x256_f64/kernix | 20948.700 / 22269.000 | 20451.500 / 20922.600 | 82 | 7145480 | unavailable |
| qr_1024x256_f64/nalgebra | 32593.900 / 33555.100 | 32532.900 / 33168.600 | 4 | 4720640 | unavailable |
| qr_1024x256_f64/numpy | 21414.100 / 22007.900 | 21610.700 / 22711.300 | unavailable | unavailable | unavailable |
| qr_256x64_f64/faer | 386.700 / 431.000 | 416.472 / 678.940 | 7 | 331776 | unavailable |
| qr_256x64_f64/kernix | 751.000 / 796.400 | 764.364 / 771.254 | 6 | 426496 | unavailable |
| qr_256x64_f64/nalgebra | 650.300 / 722.000 | 645.237 / 655.119 | 4 | 295424 | unavailable |
| qr_256x64_f64/numpy | 546.600 / 600.900 | 561.917 / 599.988 | unavailable | unavailable | unavailable |
| qr_512x128_f64/faer | 2094.900 / 2167.200 | 2057.480 / 2164.760 | 7 | 1343488 | unavailable |
| qr_512x128_f64/kernix | 3989.600 / 4237.800 | 3695.200 / 4034.167 | 49 | 2294792 | unavailable |
| qr_512x128_f64/nalgebra | 4291.300 / 4424.700 | 4281.400 / 4466.833 | 4 | 1180672 | unavailable |
| qr_512x128_f64/numpy | 4035.300 / 4406.900 | 4028.500 / 4304.500 | unavailable | unavailable | unavailable |
| qr_96x32_f64/faer | 85.900 / 107.100 | 74.261 / 96.180 | 7 | 67584 | unavailable |
| qr_96x32_f64/kernix | 118.500 / 164.800 | 110.003 / 165.500 | 6 | 82176 | unavailable |
| qr_96x32_f64/nalgebra | 82.400 / 89.300 | 79.248 / 84.038 | 4 | 57600 | unavailable |
| qr_96x32_f64/numpy | 76.500 / 98.300 | 79.032 / 94.176 | unavailable | unavailable | unavailable |
| softmax_256x64_f64/candle | 167.900 / 204.800 | 124.772 / 149.359 | 37 | 265200 | unavailable |
| softmax_256x64_f64/kernix | 112.600 / 127.900 | 114.459 / 116.419 | 1 | 131072 | unavailable |
| softmax_256x64_f64/ndarray | 162.800 / 284.900 | 165.392 / 167.028 | 1 | 131072 | unavailable |
| softmax_256x64_f64/numpy | 132.500 / 156.600 | 135.920 / 264.600 | unavailable | unavailable | unavailable |
| solve_cholesky_128x1_f64/faer | 6.500 / 6.600 | 6.598 / 6.624 | 1 | 1024 | unavailable |
| solve_cholesky_128x1_f64/kernix | 18.500 / 18.700 | 19.004 / 19.362 | 1 | 1024 | unavailable |
| solve_cholesky_128x1_f64/nalgebra | 6.400 / 6.500 | 6.005 / 6.100 | 1 | 1024 | unavailable |
| solve_cholesky_256x64_f64/faer | 326.000 / 361.500 | 332.858 / 360.043 | 1 | 131072 | unavailable |
| solve_cholesky_256x64_f64/kernix | 1296.300 / 1326.900 | 1295.425 / 1668.800 | 1 | 131072 | unavailable |
| solve_cholesky_256x64_f64/nalgebra | 1320.100 / 1382.900 | 1321.050 / 1385.075 | 1 | 131072 | unavailable |
| solve_cholesky_512x128_f64/faer | 2264.300 / 2403.000 | 2259.580 / 2297.140 | 1 | 524288 | unavailable |
| solve_cholesky_512x128_f64/kernix | 9565.900 / 10087.800 | 9642.250 / 10309.600 | 1 | 524288 | unavailable |
| solve_cholesky_512x128_f64/nalgebra | 10219.600 / 10593.100 | 10361.900 / 13412.500 | 1 | 524288 | unavailable |
| solve_lu_256x64_f64/faer | 372.100 / 428.500 | 381.141 / 394.862 | 2 | 262144 | unavailable |
| solve_lu_256x64_f64/kernix | 1306.100 / 1362.400 | 1297.088 / 1328.775 | 2 | 262144 | unavailable |
| solve_lu_256x64_f64/nalgebra | 1154.800 / 1668.100 | 1149.967 / 1254.825 | 1 | 131072 | unavailable |
| sparse_auto_512_f64/kernix-dense | 382.100 / 419.500 | 373.285 / 393.404 | 1 | 2097152 | unavailable |
| sparse_auto_512_f64/numpy | 392.400 / 477.400 | 398.235 / 404.748 | unavailable | unavailable | unavailable |
| sparse_auto_512_f64/scipy | 1468.300 / 1717.600 | 1523.743 / 1825.183 | unavailable | unavailable | unavailable |
| sparse_matmul_512x32_f32/faer | 64.100 / 78.000 | 75.617 / 79.453 | 1 | 65536 | unavailable |
| sparse_matmul_512x32_f32/kernix-dense | 332.100 / 366.700 | 338.840 / 349.134 | 3 | 163840 | unavailable |
| sparse_matmul_512x32_f32/numpy | 215.800 / 251.300 | 219.846 / 238.948 | unavailable | unavailable | unavailable |
| sparse_matmul_512x32_f32/scipy | 26.500 / 28.800 | 27.335 / 32.171 | unavailable | unavailable | unavailable |
| sparse_matmul_512x32_f64/faer | 67.700 / 70.900 | 68.918 / 70.842 | 1 | 131072 | unavailable |
| sparse_matmul_512x32_f64/kernix-dense | 626.300 / 681.800 | 630.388 / 660.250 | 3 | 327680 | unavailable |
| sparse_matmul_512x32_f64/numpy | 466.300 / 502.100 | 469.423 / 474.282 | unavailable | unavailable | unavailable |
| sparse_matmul_512x32_f64/scipy | 41.600 / 49.000 | 42.738 / 43.950 | unavailable | unavailable | unavailable |
| sparse_matvec_2048_f64/faer | 7.900 / 8.000 | 8.092 / 8.145 | 1 | 16384 | unavailable |
| sparse_matvec_2048_f64/kernix-dense | 2669.800 / 2856.500 | 2570.975 / 2708.675 | 3 | 155648 | unavailable |
| sparse_matvec_2048_f64/numpy | 655.900 / 715.100 | 730.443 / 755.150 | unavailable | unavailable | unavailable |
| sparse_matvec_2048_f64/scipy | 11.300 / 12.200 | 11.187 / 11.875 | unavailable | unavailable | unavailable |
| sparse_product_256_f64/kernix-dense | 745.700 / 788.200 | 764.200 / 902.983 | 3 | 524296 | unavailable |
| sparse_product_256_f64/numpy | 760.000 / 801.700 | 769.415 / 801.977 | unavailable | unavailable | unavailable |
| sparse_product_256_f64/scipy | 66.800 / 80.000 | 68.847 / 71.270 | unavailable | unavailable | unavailable |
| tensordot_16x32x8_32x8x16_f64/kernix | 64.000 / 64.900 | 65.156 / 65.970 | 9 | 2166 | unavailable |
| tensordot_16x32x8_32x8x16_f64/numpy | 12.100 / 12.900 | 12.428 / 21.424 | unavailable | unavailable | unavailable |
| training_backward_2x4x8x4_f64/candle | 31.900 / 34.500 | 32.307 / 33.191 | 474 | 8332 | unavailable |
| training_backward_2x4x8x4_f64/kernix | 6.100 / 6.200 | 5.815 / 5.855 | 23 | 4224 | unavailable |
| training_forward_2x4x8x4_f64/candle | 8.100 / 8.300 | 8.150 / 8.234 | 127 | 3912 | unavailable |
| training_forward_2x4x8x4_f64/kernix | 3.500 / 3.600 | 3.581 / 3.742 | 17 | 1032 | unavailable |
| training_record_2x4x8x4_f64/candle | 10.100 / 10.700 | 10.092 / 11.161 | 150 | 10712 | unavailable |
| training_record_2x4x8x4_f64/kernix | 4.900 / 5.100 | 4.804 / 4.828 | 22 | 6832 | unavailable |
| training_sgd_2x4x8x4_f64/kernix | 1.000 / 1.800 | 1.009 / 1.018 | 5 | 688 | unavailable |
| training_shared_backward_64x32_f64/candle | 58.300 / 72.900 | 6.948 / 7.007 | 58 | 100004 | unavailable |
| training_shared_backward_64x32_f64/kernix | 9.700 / 9.900 | 9.401 / 9.821 | 8 | 49768 | unavailable |

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
