# Clean CPU evidence: runner-macos-latest-40289723446

Measured source `7c11b2165b0cb7b523db4d938a36f1396396a138`, tree `cc7ac522fccee7e655adb7ca8d3644d95344fcb1`; dirty before/after: false.
Library source: workspace 7c11b2165b0cb7b523db4d938a36f1396396a138. CPU affinity: not pinned. Dtype explicitly named per workload; row-major inputs.

| Workload / library | Per-call p50 / p95 (us) | Batch-mean p50 / p95 (us) | Rust allocations | Peak additional requested bytes | Process peak RSS (MiB) |
|---|---:|---:|---:|---:|---:|
| cholesky_factor_128_f64/faer | 49.333 / 86.083 | 57.750 / 70.590 | 16 | 149504 | unavailable |
| cholesky_factor_128_f64/kernix | 192.625 / 544.209 | 214.939 / 563.178 | 1 | 131072 | unavailable |
| cholesky_factor_128_f64/nalgebra | 127.334 / 545.083 | 152.739 / 2152.675 | 1 | 131072 | unavailable |
| cholesky_factor_128_f64/numpy | 74.625 / 198.708 | 60.548 / 82.758 | unavailable | unavailable | unavailable |
| cholesky_factor_256_f64/faer | 273.833 / 328.917 | 296.936 / 443.696 | 76 | 592384 | unavailable |
| cholesky_factor_256_f64/kernix | 707.458 / 1732.750 | 1022.479 / 1614.268 | 49 | 645120 | unavailable |
| cholesky_factor_256_f64/nalgebra | 783.084 / 953.291 | 776.976 / 852.517 | 1 | 524288 | unavailable |
| cholesky_factor_256_f64/numpy | 319.458 / 401.917 | 345.451 / 382.537 | unavailable | unavailable | unavailable |
| cholesky_factor_512_f64/faer | 2105.209 / 2438.667 | 2132.600 / 2233.600 | 388 | 2625544 | unavailable |
| cholesky_factor_512_f64/kernix | 3560.667 / 4929.333 | 5773.270 / 8294.125 | 161 | 2269184 | unavailable |
| cholesky_factor_512_f64/nalgebra | 5559.250 / 6517.875 | 5394.708 / 6576.292 | 1 | 2097152 | unavailable |
| cholesky_factor_512_f64/numpy | 2385.625 / 3672.167 | 2503.117 / 2982.604 | unavailable | unavailable | unavailable |
| cholesky_factor_64_f64/faer | 13.583 / 30.750 | 13.751 / 25.272 | 2 | 33280 | unavailable |
| cholesky_factor_64_f64/kernix | 21.834 / 27.333 | 21.990 / 23.793 | 2 | 33280 | unavailable |
| cholesky_factor_64_f64/nalgebra | 22.042 / 50.541 | 20.916 / 24.283 | 1 | 32768 | unavailable |
| cholesky_factor_64_f64/numpy | 16.042 / 16.667 | 15.503 / 16.809 | unavailable | unavailable | unavailable |
| cholesky_rank_downdate_256_f64/kernix | 117.584 / 126.083 | 123.587 / 132.325 | 3 | 526336 | unavailable |
| cholesky_rank_downdate_256_f64/nalgebra | 25.208 / 29.875 | 62.293 / 130.287 | 2 | 526336 | unavailable |
| cholesky_rank_downdate_256_f64/refactor | 649.833 / 1395.708 | 700.628 / 1017.646 | 49 | 645120 | unavailable |
| cholesky_rank_update_256_f64/kernix | 118.750 / 128.875 | 121.625 / 128.074 | 3 | 526336 | unavailable |
| cholesky_rank_update_256_f64/nalgebra | 24.083 / 30.125 | 31.750 / 54.462 | 2 | 526336 | unavailable |
| cholesky_rank_update_256_f64/refactor | 639.333 / 1032.625 | 659.232 / 833.351 | 49 | 645120 | unavailable |
| cholesky_rank_update_512_f64/kernix | 510.125 / 770.500 | 538.965 / 581.259 | 3 | 2101248 | unavailable |
| cholesky_rank_update_512_f64/nalgebra | 102.709 / 139.792 | 110.037 / 117.657 | 2 | 2101248 | unavailable |
| cholesky_rank_update_512_f64/refactor | 3539.250 / 5558.792 | 3845.500 / 5766.896 | 161 | 2269184 | unavailable |
| create_ones_64x64_f32/candle | 1.000 / 1.167 | 0.443 / 0.500 | 7 | 16696 | unavailable |
| create_ones_64x64_f32/faer | 0.416 / 0.459 | 0.401 / 0.619 | 1 | 16384 | unavailable |
| create_ones_64x64_f32/kernix | 0.292 / 0.375 | 0.361 / 0.485 | 1 | 16384 | unavailable |
| create_ones_64x64_f32/ndarray | 0.375 / 0.583 | 0.321 / 0.373 | 1 | 16384 | unavailable |
| create_ones_64x64_f32/numpy | 1.250 / 3.084 | 1.246 / 1.418 | unavailable | unavailable | unavailable |
| create_zeros_64x64_f32/candle | 0.542 / 0.833 | 0.476 / 0.764 | 5 | 16648 | unavailable |
| create_zeros_64x64_f32/faer | 0.291 / 0.292 | 0.293 / 0.335 | 1 | 16384 | unavailable |
| create_zeros_64x64_f32/kernix | 0.292 / 0.334 | 0.401 / 0.523 | 1 | 16384 | unavailable |
| create_zeros_64x64_f32/ndarray | 0.334 / 0.500 | 0.396 / 0.632 | 1 | 16384 | unavailable |
| create_zeros_64x64_f32/numpy | 0.500 / 0.584 | 0.621 / 0.684 | unavailable | unavailable | unavailable |
| matmul_128_f32/candle | 52.667 / 57.209 | 53.383 / 58.704 | 10 | 327768 | unavailable |
| matmul_128_f32/faer | 55.291 / 61.125 | 55.772 / 56.021 | 3 | 393224 | unavailable |
| matmul_128_f32/kernix | 57.542 / 91.375 | 63.110 / 86.519 | 3 | 163840 | unavailable |
| matmul_128_f32/kernix-accelerate | 6.458 / 22.667 | 6.865 / 13.113 | 1 | 65536 | unavailable |
| matmul_128_f32/nalgebra | 56.916 / 107.333 | 70.040 / 115.010 | 2 | 163840 | unavailable |
| matmul_128_f32/ndarray | 51.583 / 62.416 | 58.724 / 67.208 | 2 | 163840 | unavailable |
| matmul_128_f32/numpy | 6.000 / 6.250 | 6.286 / 6.805 | unavailable | unavailable | unavailable |
| matmul_128_f64/candle | 101.417 / 114.209 | 103.454 / 136.429 | 10 | 655448 | unavailable |
| matmul_128_f64/faer | 108.250 / 126.333 | 120.290 / 154.904 | 3 | 786440 | unavailable |
| matmul_128_f64/kernix | 102.250 / 303.125 | 117.785 / 139.165 | 3 | 327680 | unavailable |
| matmul_128_f64/kernix-accelerate | 17.916 / 18.292 | 18.362 / 19.661 | 1 | 131072 | unavailable |
| matmul_128_f64/nalgebra | 111.708 / 322.792 | 121.227 / 182.587 | 2 | 327680 | unavailable |
| matmul_128_f64/ndarray | 106.583 / 123.000 | 119.786 / 162.429 | 2 | 327680 | unavailable |
| matmul_128_f64/numpy | 17.292 / 17.583 | 17.457 / 45.102 | unavailable | unavailable | unavailable |
| matmul_256_f32/candle | 390.541 / 427.208 | 393.385 / 511.535 | 10 | 786520 | unavailable |
| matmul_256_f32/faer | 427.667 / 457.208 | 442.877 / 537.570 | 3 | 1048584 | unavailable |
| matmul_256_f32/kernix | 416.000 / 461.750 | 419.547 / 461.367 | 3 | 589824 | unavailable |
| matmul_256_f32/kernix-accelerate | 35.583 / 36.334 | 44.534 / 68.320 | 1 | 262144 | unavailable |
| matmul_256_f32/nalgebra | 388.042 / 1126.375 | 387.234 / 392.343 | 2 | 589824 | unavailable |
| matmul_256_f32/ndarray | 432.208 / 1132.083 | 415.170 / 466.172 | 2 | 589824 | unavailable |
| matmul_256_f32/numpy | 32.375 / 34.542 | 33.087 / 43.701 | unavailable | unavailable | unavailable |
| matmul_256_f64/candle | 889.875 / 1106.500 | 936.670 / 1238.444 | 10 | 1572952 | unavailable |
| matmul_256_f64/faer | 811.208 / 1001.917 | 891.528 / 949.413 | 3 | 1572872 | unavailable |
| matmul_256_f64/kernix | 811.750 / 1510.417 | 843.941 / 1533.226 | 3 | 1179648 | unavailable |
| matmul_256_f64/kernix-accelerate | 120.834 / 132.125 | 123.567 / 141.558 | 1 | 524288 | unavailable |
| matmul_256_f64/nalgebra | 926.958 / 2067.291 | 1038.204 / 1371.036 | 2 | 1179648 | unavailable |
| matmul_256_f64/ndarray | 784.667 / 935.667 | 784.792 / 1326.500 | 2 | 1179648 | unavailable |
| matmul_256_f64/numpy | 124.625 / 219.250 | 135.940 / 278.846 | unavailable | unavailable | unavailable |
| matmul_32_f64/candle | 2.708 / 3.458 | 2.722 / 5.492 | 10 | 18512 | unavailable |
| matmul_32_f64/faer | 2.500 / 3.166 | 2.814 / 3.087 | 3 | 26624 | unavailable |
| matmul_32_f64/kernix | 2.250 / 8.708 | 2.438 / 2.879 | 3 | 24576 | unavailable |
| matmul_32_f64/nalgebra | 2.125 / 2.125 | 2.453 / 3.338 | 2 | 24576 | unavailable |
| matmul_32_f64/ndarray | 2.041 / 2.625 | 2.331 / 3.045 | 2 | 24576 | unavailable |
| matmul_32_f64/numpy | 4.166 / 4.334 | 1.718 / 2.511 | unavailable | unavailable | unavailable |
| matmul_64_f32/candle | 7.916 / 8.083 | 8.190 / 10.219 | 10 | 33872 | unavailable |
| matmul_64_f32/faer | 8.167 / 8.333 | 8.935 / 9.892 | 3 | 50176 | unavailable |
| matmul_64_f32/kernix | 7.250 / 9.167 | 7.819 / 11.015 | 3 | 49152 | unavailable |
| matmul_64_f32/nalgebra | 6.917 / 8.750 | 8.059 / 9.239 | 2 | 49152 | unavailable |
| matmul_64_f32/ndarray | 7.042 / 21.166 | 7.322 / 8.471 | 2 | 49152 | unavailable |
| matmul_64_f32/numpy | 5.500 / 6.333 | 2.080 / 5.775 | unavailable | unavailable | unavailable |
| matmul_64_f64/candle | 14.584 / 18.250 | 15.529 / 23.903 | 10 | 66640 | unavailable |
| matmul_64_f64/faer | 14.791 / 18.833 | 15.928 / 17.424 | 3 | 99328 | unavailable |
| matmul_64_f64/kernix | 14.084 / 39.458 | 14.404 / 15.939 | 3 | 98304 | unavailable |
| matmul_64_f64/nalgebra | 13.542 / 17.083 | 14.835 / 17.287 | 2 | 98304 | unavailable |
| matmul_64_f64/ndarray | 13.416 / 16.833 | 13.758 / 16.536 | 2 | 98304 | unavailable |
| matmul_64_f64/numpy | 3.291 / 3.625 | 3.577 / 4.465 | unavailable | unavailable | unavailable |
| matmul_ab_128_f32/kernix | 54.917 / 94.709 | 55.343 / 57.744 | 3 | 163840 | unavailable |
| matmul_ab_128_f32/numpy | 5.959 / 6.416 | 6.038 / 6.454 | unavailable | unavailable | unavailable |
| matmul_ab_128_f64/kernix | 103.750 / 134.416 | 107.006 / 113.758 | 3 | 327680 | unavailable |
| matmul_ab_128_f64/numpy | 17.459 / 18.625 | 18.139 / 18.680 | unavailable | unavailable | unavailable |
| matmul_batched_4x16x32x16_f32/candle | 2.792 / 3.583 | 2.517 / 2.996 | 25 | 14944 | unavailable |
| matmul_batched_4x16x32x16_f32/kernix | 1.750 / 5.000 | 1.725 / 2.403 | 7 | 8200 | unavailable |
| matmul_batched_4x16x32x16_f32/numpy | 2.208 / 2.417 | 2.309 / 3.862 | unavailable | unavailable | unavailable |
| matmul_batched_4x16x32x16_f64/candle | 4.666 / 4.834 | 4.032 / 4.468 | 25 | 27232 | unavailable |
| matmul_batched_4x16x32x16_f64/kernix | 2.833 / 3.709 | 3.058 / 7.360 | 7 | 16392 | unavailable |
| matmul_batched_4x16x32x16_f64/numpy | 2.042 / 2.250 | 2.300 / 2.352 | unavailable | unavailable | unavailable |
| matmul_sum_backward_32x16x8_f64/candle | 6.917 / 9.208 | 6.697 / 14.602 | 113 | 27980 | unavailable |
| matmul_sum_backward_32x16x8_f64/kernix | 3.792 / 8.000 | 3.482 / 5.584 | 16 | 18376 | unavailable |
| matmul_transposed_128_f32/kernix | 51.416 / 151.000 | 58.646 / 69.676 | 3 | 163840 | unavailable |
| matmul_transposed_128_f32/numpy | 5.250 / 6.000 | 7.207 / 19.700 | unavailable | unavailable | unavailable |
| matmul_transposed_128_f64/kernix | 112.625 / 312.708 | 117.227 / 176.817 | 3 | 327680 | unavailable |
| matmul_transposed_128_f64/numpy | 14.875 / 15.875 | 15.349 / 16.837 | unavailable | unavailable | unavailable |
| min_norm_32x64_rank32_f64/kernix | 64.959 / 72.667 | 75.070 / 83.882 | 20 | 59136 | unavailable |
| min_norm_32x64_rank32_f64/nalgebra | 152.750 / 164.542 | 155.737 / 244.370 | 15 | 42232 | unavailable |
| min_norm_32x64_rank32_f64/numpy | 159.125 / 364.958 | 152.078 / 193.117 | unavailable | unavailable | unavailable |
| min_norm_64x32_rank16_f64/kernix | 56.417 / 127.834 | 59.010 / 66.896 | 20 | 74752 | unavailable |
| min_norm_64x32_rank16_f64/nalgebra | 75.750 / 92.167 | 83.557 / 137.688 | 15 | 41976 | unavailable |
| min_norm_64x32_rank16_f64/numpy | 99.500 / 141.583 | 99.132 / 118.158 | unavailable | unavailable | unavailable |
| parallel_matmul_into_128_f32/kernix | 55.917 / 121.167 | 54.160 / 71.649 | 2 | 196608 | unavailable |
| parallel_matmul_into_128_f64/kernix | 146.875 / 202.125 | 152.372 / 182.759 | 2 | 393216 | unavailable |
| parallel_matmul_into_256_f64/kernix | 747.042 / 1195.791 | 708.517 / 880.399 | 2 | 1310720 | unavailable |
| parallel_matmul_into_64_f64/kernix | 22.708 / 36.959 | 24.342 / 41.209 | 2 | 98304 | unavailable |
| pivoted_qr_256x64_f64/faer | 322.584 / 978.375 | 449.900 / 642.734 | 58 | 336904 | unavailable |
| pivoted_qr_256x64_f64/kernix | 610.167 / 755.500 | 677.000 / 1042.558 | 14 | 559104 | unavailable |
| pivoted_qr_256x64_f64/nalgebra | 1071.417 / 1808.375 | 1100.729 / 1162.579 | 7 | 296960 | unavailable |
| pivoted_qr_96x32_f64/faer | 43.500 / 46.625 | 46.807 / 90.282 | 32 | 94728 | unavailable |
| pivoted_qr_96x32_f64/kernix | 75.875 / 180.375 | 68.039 / 77.203 | 14 | 107520 | unavailable |
| pivoted_qr_96x32_f64/nalgebra | 108.500 / 140.333 | 112.937 / 121.982 | 7 | 58368 | unavailable |
| qr_1024x256_f64/faer | 12839.417 / 16322.208 | 14365.875 / 19631.208 | 764 | 9166852 | unavailable |
| qr_1024x256_f64/kernix | 15756.625 / 24949.667 | 22513.292 / 28445.167 | 51 | 7276544 | unavailable |
| qr_1024x256_f64/nalgebra | 25029.375 / 33704.000 | 25136.334 / 27106.083 | 4 | 4720640 | unavailable |
| qr_1024x256_f64/numpy | 12603.334 / 14834.500 | 13689.209 / 14367.000 | unavailable | unavailable | unavailable |
| qr_256x64_f64/faer | 312.917 / 375.458 | 304.153 / 308.836 | 83 | 335880 | unavailable |
| qr_256x64_f64/kernix | 575.750 / 675.208 | 548.980 / 615.123 | 6 | 426496 | unavailable |
| qr_256x64_f64/nalgebra | 357.542 / 585.292 | 389.139 / 562.361 | 4 | 295424 | unavailable |
| qr_256x64_f64/numpy | 394.708 / 496.583 | 401.078 / 451.190 | unavailable | unavailable | unavailable |
| qr_512x128_f64/faer | 1981.916 / 2436.875 | 1932.521 / 2106.367 | 360 | 3334148 | unavailable |
| qr_512x128_f64/kernix | 2751.375 / 4587.750 | 2930.792 / 3694.736 | 31 | 2098176 | unavailable |
| qr_512x128_f64/nalgebra | 3003.750 / 3498.625 | 2981.834 / 4032.111 | 4 | 1180672 | unavailable |
| qr_512x128_f64/numpy | 2461.375 / 2926.334 | 2482.317 / 2597.438 | unavailable | unavailable | unavailable |
| qr_96x32_f64/faer | 47.167 / 88.625 | 51.539 / 59.213 | 41 | 94216 | unavailable |
| qr_96x32_f64/kernix | 63.208 / 94.833 | 63.347 / 135.882 | 6 | 82176 | unavailable |
| qr_96x32_f64/nalgebra | 41.459 / 51.375 | 43.362 / 79.058 | 4 | 57600 | unavailable |
| qr_96x32_f64/numpy | 50.542 / 175.916 | 53.438 / 162.199 | unavailable | unavailable | unavailable |
| softmax_256x64_f64/candle | 72.416 / 97.792 | 76.593 / 112.463 | 37 | 265200 | unavailable |
| softmax_256x64_f64/kernix | 94.459 / 141.584 | 87.897 / 134.748 | 1 | 131072 | unavailable |
| softmax_256x64_f64/ndarray | 63.625 / 75.000 | 65.720 / 67.676 | 1 | 131072 | unavailable |
| softmax_256x64_f64/numpy | 75.625 / 261.000 | 82.132 / 124.320 | unavailable | unavailable | unavailable |
| solve_cholesky_128x1_f64/faer | 4.750 / 6.167 | 5.254 / 5.469 | 1 | 1024 | unavailable |
| solve_cholesky_128x1_f64/kernix | 15.709 / 19.833 | 18.630 / 20.139 | 1 | 1024 | unavailable |
| solve_cholesky_128x1_f64/nalgebra | 4.708 / 5.917 | 4.784 / 5.581 | 1 | 1024 | unavailable |
| solve_cholesky_256x64_f64/faer | 337.000 / 584.792 | 325.558 / 465.129 | 53 | 786440 | unavailable |
| solve_cholesky_256x64_f64/kernix | 1265.292 / 1580.625 | 1358.208 / 1970.403 | 1 | 131072 | unavailable |
| solve_cholesky_256x64_f64/nalgebra | 974.459 / 1111.292 | 974.977 / 1031.546 | 1 | 131072 | unavailable |
| solve_cholesky_512x128_f64/faer | 2496.000 / 3930.125 | 2728.500 / 5248.938 | 205 | 1572872 | unavailable |
| solve_cholesky_512x128_f64/kernix | 11043.792 / 13777.875 | 12023.917 / 13563.084 | 1 | 524288 | unavailable |
| solve_cholesky_512x128_f64/nalgebra | 7446.625 / 8459.458 | 7603.604 / 7915.291 | 1 | 524288 | unavailable |
| solve_lu_256x64_f64/faer | 323.166 / 432.833 | 334.004 / 415.889 | 46 | 786440 | unavailable |
| solve_lu_256x64_f64/kernix | 1430.416 / 2195.292 | 1361.141 / 2213.692 | 2 | 262144 | unavailable |
| solve_lu_256x64_f64/nalgebra | 950.125 / 1093.125 | 956.076 / 1153.704 | 1 | 131072 | unavailable |
| sparse_auto_512_f64/kernix | 744.500 / 1087.125 | 790.407 / 1043.129 | 4 | 2142216 | unavailable |
| sparse_auto_512_f64/kernix-dense | 53.167 / 73.625 | 61.852 / 82.671 | 1 | 2097152 | unavailable |
| sparse_auto_512_f64/numpy | 180.291 / 309.958 | 146.254 / 156.917 | unavailable | unavailable | unavailable |
| sparse_auto_512_f64/scipy | 964.292 / 2236.500 | 990.292 / 1870.028 | unavailable | unavailable | unavailable |
| sparse_matmul_512x32_f32/faer | 49.584 / 79.916 | 53.363 / 72.480 | 1 | 65536 | unavailable |
| sparse_matmul_512x32_f32/kernix | 42.583 / 79.792 | 39.419 / 45.388 | 1 | 65536 | unavailable |
| sparse_matmul_512x32_f32/kernix-dense | 262.625 / 441.333 | 237.830 / 411.067 | 3 | 163840 | unavailable |
| sparse_matmul_512x32_f32/numpy | 31.458 / 45.666 | 33.170 / 61.899 | unavailable | unavailable | unavailable |
| sparse_matmul_512x32_f32/scipy | 13.792 / 32.958 | 25.081 / 33.167 | unavailable | unavailable | unavailable |
| sparse_matmul_512x32_f64/faer | 54.167 / 65.084 | 55.286 / 72.268 | 1 | 131072 | unavailable |
| sparse_matmul_512x32_f64/kernix | 29.167 / 66.500 | 31.591 / 46.851 | 1 | 131072 | unavailable |
| sparse_matmul_512x32_f64/kernix-dense | 440.417 / 462.291 | 447.321 / 896.656 | 3 | 327680 | unavailable |
| sparse_matmul_512x32_f64/numpy | 87.084 / 93.666 | 91.662 / 270.747 | unavailable | unavailable | unavailable |
| sparse_matmul_512x32_f64/scipy | 21.083 / 26.208 | 22.165 / 31.693 | unavailable | unavailable | unavailable |
| sparse_matvec_2048_f64/faer | 6.792 / 7.125 | 6.841 / 14.666 | 1 | 16384 | unavailable |
| sparse_matvec_2048_f64/kernix | 15.583 / 19.667 | 16.681 / 21.669 | 1 | 16384 | unavailable |
| sparse_matvec_2048_f64/kernix-dense | 3004.417 / 3709.583 | 2919.354 / 4205.458 | 3 | 155648 | unavailable |
| sparse_matvec_2048_f64/numpy | 1356.042 / 1747.750 | 1444.583 / 2773.209 | unavailable | unavailable | unavailable |
| sparse_matvec_2048_f64/scipy | 8.250 / 10.209 | 8.626 / 9.561 | unavailable | unavailable | unavailable |
| sparse_product_256_f64/kernix | 61.292 / 73.875 | 64.513 / 71.757 | 4 | 67752 | unavailable |
| sparse_product_256_f64/kernix-dense | 827.416 / 1243.125 | 804.987 / 875.854 | 3 | 1179648 | unavailable |
| sparse_product_256_f64/numpy | 129.208 / 553.791 | 151.939 / 194.743 | unavailable | unavailable | unavailable |
| sparse_product_256_f64/scipy | 44.958 / 74.708 | 45.046 / 83.453 | unavailable | unavailable | unavailable |
| tensordot_16x32x8_32x8x16_f64/kernix | 74.708 / 209.417 | 65.013 / 74.943 | 9 | 2166 | unavailable |
| tensordot_16x32x8_32x8x16_f64/numpy | 5.667 / 6.167 | 6.356 / 8.440 | unavailable | unavailable | unavailable |
| training_backward_2x4x8x4_f64/candle | 23.209 / 26.708 | 23.010 / 29.473 | 476 | 21284 | unavailable |
| training_backward_2x4x8x4_f64/kernix | 3.375 / 3.417 | 3.146 / 3.402 | 23 | 4224 | unavailable |
| training_forward_2x4x8x4_f64/candle | 5.792 / 7.208 | 4.467 / 5.637 | 126 | 9568 | unavailable |
| training_forward_2x4x8x4_f64/kernix | 2.250 / 2.792 | 2.096 / 2.295 | 17 | 1032 | unavailable |
| training_record_2x4x8x4_f64/candle | 6.334 / 8.375 | 6.154 / 9.179 | 149 | 11544 | unavailable |
| training_record_2x4x8x4_f64/kernix | 3.166 / 3.209 | 2.984 / 3.964 | 22 | 6832 | unavailable |
| training_sgd_2x4x8x4_f64/kernix | 0.750 / 0.917 | 0.822 / 1.150 | 5 | 688 | unavailable |
| training_shared_backward_64x32_f64/candle | 4.500 / 5.459 | 4.211 / 5.494 | 58 | 99996 | unavailable |
| training_shared_backward_64x32_f64/kernix | 6.000 / 7.584 | 6.412 / 10.538 | 8 | 49768 | unavailable |

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
