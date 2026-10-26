# Clean CPU evidence: baseline-macos-latest-40289723446

Measured source `7c11b2165b0cb7b523db4d938a36f1396396a138`, tree `cc7ac522fccee7e655adb7ca8d3644d95344fcb1`; dirty before/after: false.
Library source: git v0.12.0 90cbbff60e5d7458ef9f89918c99a6cca94b51d1. CPU affinity: not pinned. Dtype explicitly named per workload; row-major inputs.

| Workload / library | Per-call p50 / p95 (us) | Batch-mean p50 / p95 (us) | Rust allocations | Peak additional requested bytes | Process peak RSS (MiB) |
|---|---:|---:|---:|---:|---:|
| cholesky_factor_128_f64/faer | 47.209 / 48.208 | 47.578 / 48.558 | 16 | 149504 | unavailable |
| cholesky_factor_128_f64/kernix | 171.708 / 182.667 | 172.906 / 181.253 | 1 | 131072 | unavailable |
| cholesky_factor_128_f64/nalgebra | 102.916 / 126.416 | 112.524 / 137.134 | 1 | 131072 | unavailable |
| cholesky_factor_128_f64/numpy | 47.041 / 49.792 | 47.726 / 67.520 | unavailable | unavailable | unavailable |
| cholesky_factor_256_f64/faer | 266.625 / 321.875 | 271.329 / 323.239 | 76 | 592384 | unavailable |
| cholesky_factor_256_f64/kernix | 616.792 / 655.708 | 618.103 / 625.836 | 49 | 645120 | unavailable |
| cholesky_factor_256_f64/nalgebra | 752.666 / 1211.875 | 781.064 / 1720.618 | 1 | 524288 | unavailable |
| cholesky_factor_256_f64/numpy | 336.750 / 453.833 | 368.080 / 435.110 | unavailable | unavailable | unavailable |
| cholesky_factor_512_f64/faer | 1921.250 / 2500.667 | 1951.229 / 2722.183 | 388 | 2625544 | unavailable |
| cholesky_factor_512_f64/kernix | 3570.042 / 5844.958 | 5497.236 / 7859.646 | 161 | 2269184 | unavailable |
| cholesky_factor_512_f64/nalgebra | 5303.917 / 5662.709 | 5368.770 / 5565.521 | 1 | 2097152 | unavailable |
| cholesky_factor_512_f64/numpy | 2228.167 / 2538.792 | 2237.500 / 2506.365 | unavailable | unavailable | unavailable |
| cholesky_factor_64_f64/faer | 10.500 / 10.917 | 11.009 / 11.761 | 2 | 33280 | unavailable |
| cholesky_factor_64_f64/kernix | 21.917 / 55.417 | 22.023 / 24.228 | 2 | 33280 | unavailable |
| cholesky_factor_64_f64/nalgebra | 20.750 / 21.583 | 20.876 / 21.256 | 1 | 32768 | unavailable |
| cholesky_factor_64_f64/numpy | 15.417 / 22.917 | 16.029 / 20.835 | unavailable | unavailable | unavailable |
| cholesky_rank_downdate_256_f64/kernix | 114.209 / 122.167 | 115.694 / 125.043 | 3 | 526336 | unavailable |
| cholesky_rank_downdate_256_f64/nalgebra | 21.750 / 23.041 | 24.000 / 29.185 | 2 | 526336 | unavailable |
| cholesky_rank_downdate_256_f64/refactor | 654.375 / 790.750 | 660.260 / 752.714 | 49 | 645120 | unavailable |
| cholesky_rank_update_256_f64/kernix | 119.792 / 127.167 | 119.816 / 123.532 | 3 | 526336 | unavailable |
| cholesky_rank_update_256_f64/nalgebra | 24.083 / 31.542 | 23.959 / 28.153 | 2 | 526336 | unavailable |
| cholesky_rank_update_256_f64/refactor | 612.833 / 636.875 | 609.350 / 649.422 | 49 | 645120 | unavailable |
| cholesky_rank_update_512_f64/kernix | 465.584 / 488.833 | 467.771 / 517.358 | 3 | 2101248 | unavailable |
| cholesky_rank_update_512_f64/nalgebra | 89.750 / 92.750 | 90.128 / 90.721 | 2 | 2101248 | unavailable |
| cholesky_rank_update_512_f64/refactor | 3314.750 / 3496.583 | 3310.198 / 3397.903 | 161 | 2269184 | unavailable |
| create_ones_64x64_f32/candle | 0.375 / 0.458 | 0.405 / 0.592 | 7 | 16696 | unavailable |
| create_ones_64x64_f32/faer | 0.708 / 0.750 | 0.351 / 0.404 | 1 | 16384 | unavailable |
| create_ones_64x64_f32/kernix | 0.333 / 0.334 | 0.291 / 0.327 | 1 | 16384 | unavailable |
| create_ones_64x64_f32/ndarray | 0.333 / 0.334 | 0.269 / 0.281 | 1 | 16384 | unavailable |
| create_ones_64x64_f32/numpy | 1.042 / 1.084 | 1.127 / 1.140 | unavailable | unavailable | unavailable |
| create_zeros_64x64_f32/candle | 0.375 / 0.417 | 0.349 / 0.390 | 5 | 16648 | unavailable |
| create_zeros_64x64_f32/faer | 0.292 / 0.375 | 0.278 / 0.326 | 1 | 16384 | unavailable |
| create_zeros_64x64_f32/kernix | 0.375 / 0.625 | 0.339 / 0.608 | 1 | 16384 | unavailable |
| create_zeros_64x64_f32/ndarray | 0.333 / 0.334 | 0.265 / 0.334 | 1 | 16384 | unavailable |
| create_zeros_64x64_f32/numpy | 0.500 / 0.542 | 0.554 / 0.587 | unavailable | unavailable | unavailable |
| matmul_128_f32/candle | 52.792 / 62.125 | 54.226 / 58.468 | 10 | 327768 | unavailable |
| matmul_128_f32/faer | 55.292 / 68.000 | 57.665 / 94.609 | 3 | 393224 | unavailable |
| matmul_128_f32/kernix | 52.458 / 59.292 | 52.240 / 52.952 | 3 | 163840 | unavailable |
| matmul_128_f32/kernix-accelerate | 5.709 / 5.917 | 6.122 / 6.292 | 1 | 65536 | unavailable |
| matmul_128_f32/nalgebra | 51.250 / 60.792 | 52.610 / 60.743 | 2 | 163840 | unavailable |
| matmul_128_f32/ndarray | 51.458 / 58.708 | 51.646 / 54.760 | 2 | 163840 | unavailable |
| matmul_128_f32/numpy | 6.000 / 22.667 | 6.363 / 7.222 | unavailable | unavailable | unavailable |
| matmul_128_f64/candle | 100.500 / 110.542 | 98.427 / 102.732 | 10 | 655448 | unavailable |
| matmul_128_f64/faer | 107.875 / 129.000 | 110.618 / 119.818 | 3 | 786440 | unavailable |
| matmul_128_f64/kernix | 108.292 / 193.542 | 111.256 / 173.193 | 3 | 327680 | unavailable |
| matmul_128_f64/kernix-accelerate | 18.250 / 19.333 | 18.962 / 48.723 | 1 | 131072 | unavailable |
| matmul_128_f64/nalgebra | 118.500 / 304.542 | 119.472 / 200.241 | 2 | 327680 | unavailable |
| matmul_128_f64/ndarray | 101.458 / 124.250 | 103.175 / 112.097 | 2 | 327680 | unavailable |
| matmul_128_f64/numpy | 17.875 / 24.750 | 19.519 / 21.323 | unavailable | unavailable | unavailable |
| matmul_256_f32/candle | 426.083 / 1190.333 | 426.260 / 597.169 | 10 | 786520 | unavailable |
| matmul_256_f32/faer | 409.917 / 438.708 | 435.759 / 473.258 | 3 | 1048584 | unavailable |
| matmul_256_f32/kernix | 389.333 / 443.666 | 408.795 / 466.161 | 3 | 589824 | unavailable |
| matmul_256_f32/kernix-accelerate | 35.667 / 36.917 | 32.913 / 35.132 | 1 | 262144 | unavailable |
| matmul_256_f32/nalgebra | 384.209 / 396.750 | 385.897 / 391.811 | 2 | 589824 | unavailable |
| matmul_256_f32/ndarray | 370.542 / 388.166 | 374.049 / 425.983 | 2 | 589824 | unavailable |
| matmul_256_f32/numpy | 32.208 / 34.583 | 34.240 / 144.368 | unavailable | unavailable | unavailable |
| matmul_256_f64/candle | 830.000 / 896.000 | 829.510 / 1525.060 | 10 | 1572952 | unavailable |
| matmul_256_f64/faer | 807.917 / 852.459 | 814.805 / 956.254 | 3 | 1572872 | unavailable |
| matmul_256_f64/kernix | 777.000 / 871.542 | 835.948 / 1257.630 | 3 | 1179648 | unavailable |
| matmul_256_f64/kernix-accelerate | 119.750 / 135.500 | 121.429 / 125.190 | 1 | 524288 | unavailable |
| matmul_256_f64/nalgebra | 762.000 / 842.917 | 754.684 / 764.589 | 2 | 1179648 | unavailable |
| matmul_256_f64/ndarray | 784.166 / 836.209 | 795.965 / 821.369 | 2 | 1179648 | unavailable |
| matmul_256_f64/numpy | 116.959 / 121.833 | 116.632 / 121.805 | unavailable | unavailable | unavailable |
| matmul_32_f64/candle | 2.542 / 2.584 | 2.397 / 2.768 | 10 | 18512 | unavailable |
| matmul_32_f64/faer | 2.417 / 2.459 | 2.440 / 2.523 | 3 | 26624 | unavailable |
| matmul_32_f64/kernix | 2.250 / 2.791 | 2.698 / 3.724 | 3 | 24576 | unavailable |
| matmul_32_f64/nalgebra | 2.250 / 2.875 | 2.295 / 3.200 | 2 | 24576 | unavailable |
| matmul_32_f64/ndarray | 2.000 / 2.750 | 2.079 / 2.680 | 2 | 24576 | unavailable |
| matmul_32_f64/numpy | 1.500 / 1.875 | 1.967 / 4.650 | unavailable | unavailable | unavailable |
| matmul_64_f32/candle | 7.958 / 10.208 | 8.361 / 11.029 | 10 | 33872 | unavailable |
| matmul_64_f32/faer | 8.208 / 10.416 | 8.493 / 9.350 | 3 | 50176 | unavailable |
| matmul_64_f32/kernix | 7.208 / 8.709 | 7.308 / 7.789 | 3 | 49152 | unavailable |
| matmul_64_f32/nalgebra | 6.667 / 6.917 | 7.029 / 7.287 | 2 | 49152 | unavailable |
| matmul_64_f32/ndarray | 6.917 / 8.958 | 6.960 / 7.139 | 2 | 49152 | unavailable |
| matmul_64_f32/numpy | 1.709 / 1.875 | 1.969 / 2.404 | unavailable | unavailable | unavailable |
| matmul_64_f64/candle | 14.833 / 19.042 | 16.459 / 19.940 | 10 | 66640 | unavailable |
| matmul_64_f64/faer | 14.750 / 15.125 | 14.553 / 15.218 | 3 | 99328 | unavailable |
| matmul_64_f64/kernix | 14.333 / 18.000 | 15.196 / 33.577 | 3 | 98304 | unavailable |
| matmul_64_f64/nalgebra | 16.583 / 45.334 | 17.442 / 26.903 | 2 | 98304 | unavailable |
| matmul_64_f64/ndarray | 12.792 / 14.458 | 12.948 / 13.856 | 2 | 98304 | unavailable |
| matmul_64_f64/numpy | 3.250 / 3.500 | 3.343 / 3.573 | unavailable | unavailable | unavailable |
| matmul_ab_128_f32/kernix | 51.000 / 53.000 | 52.001 / 53.325 | 3 | 163840 | unavailable |
| matmul_ab_128_f32/numpy | 6.000 / 8.750 | 6.463 / 8.675 | unavailable | unavailable | unavailable |
| matmul_ab_128_f64/kernix | 101.917 / 112.292 | 102.580 / 106.187 | 3 | 327680 | unavailable |
| matmul_ab_128_f64/numpy | 17.375 / 18.958 | 17.568 / 18.272 | unavailable | unavailable | unavailable |
| matmul_batched_4x16x32x16_f32/candle | 2.625 / 2.791 | 2.599 / 3.282 | 25 | 14944 | unavailable |
| matmul_batched_4x16x32x16_f32/kernix | 1.834 / 2.292 | 1.911 / 2.028 | 7 | 8200 | unavailable |
| matmul_batched_4x16x32x16_f32/numpy | 2.208 / 2.375 | 2.373 / 3.913 | unavailable | unavailable | unavailable |
| matmul_batched_4x16x32x16_f64/candle | 3.625 / 4.459 | 3.664 / 5.056 | 25 | 27232 | unavailable |
| matmul_batched_4x16x32x16_f64/kernix | 2.833 / 2.958 | 2.704 / 2.908 | 7 | 16392 | unavailable |
| matmul_batched_4x16x32x16_f64/numpy | 2.084 / 2.333 | 2.157 / 2.673 | unavailable | unavailable | unavailable |
| matmul_sum_backward_32x16x8_f64/candle | 6.167 / 6.458 | 6.130 / 6.532 | 113 | 27980 | unavailable |
| matmul_sum_backward_32x16x8_f64/kernix | 3.250 / 4.167 | 3.455 / 5.459 | 16 | 18376 | unavailable |
| matmul_transposed_128_f32/kernix | 51.041 / 51.666 | 50.742 / 54.390 | 3 | 163840 | unavailable |
| matmul_transposed_128_f32/numpy | 5.083 / 5.334 | 5.243 / 7.945 | unavailable | unavailable | unavailable |
| matmul_transposed_128_f64/kernix | 102.625 / 107.916 | 103.552 / 108.284 | 3 | 327680 | unavailable |
| matmul_transposed_128_f64/numpy | 15.708 / 16.292 | 16.461 / 18.999 | unavailable | unavailable | unavailable |
| min_norm_32x64_rank32_f64/kernix | 65.250 / 74.542 | 65.463 / 69.014 | 20 | 59136 | unavailable |
| min_norm_32x64_rank32_f64/nalgebra | 155.833 / 175.917 | 163.782 / 175.475 | 15 | 42232 | unavailable |
| min_norm_32x64_rank32_f64/numpy | 145.750 / 159.208 | 146.318 / 163.142 | unavailable | unavailable | unavailable |
| min_norm_64x32_rank16_f64/kernix | 47.291 / 49.500 | 49.521 / 50.239 | 20 | 74752 | unavailable |
| min_norm_64x32_rank16_f64/nalgebra | 72.209 / 78.542 | 73.402 / 74.083 | 15 | 41976 | unavailable |
| min_norm_64x32_rank16_f64/numpy | 83.916 / 95.750 | 83.680 / 140.834 | unavailable | unavailable | unavailable |
| parallel_matmul_into_128_f32/kernix | 57.250 / 156.542 | 78.300 / 137.846 | 2 | 196608 | unavailable |
| parallel_matmul_into_128_f64/kernix | 125.792 / 226.375 | 126.297 / 193.154 | 3 | 393216 | unavailable |
| parallel_matmul_into_256_f64/kernix | 490.000 / 1146.250 | 736.566 / 2760.619 | 2 | 1310720 | unavailable |
| parallel_matmul_into_64_f64/kernix | 20.917 / 39.708 | 24.011 / 25.707 | 2 | 98304 | unavailable |
| pivoted_qr_256x64_f64/faer | 299.791 / 320.958 | 302.534 / 368.924 | 58 | 336904 | unavailable |
| pivoted_qr_256x64_f64/kernix | 542.667 / 558.708 | 549.719 / 575.627 | 14 | 559104 | unavailable |
| pivoted_qr_256x64_f64/nalgebra | 1036.750 / 1390.042 | 1088.067 / 1224.727 | 7 | 296960 | unavailable |
| pivoted_qr_96x32_f64/faer | 42.000 / 50.916 | 43.083 / 45.067 | 32 | 94728 | unavailable |
| pivoted_qr_96x32_f64/kernix | 67.000 / 70.792 | 66.725 / 67.572 | 14 | 107520 | unavailable |
| pivoted_qr_96x32_f64/nalgebra | 99.917 / 118.500 | 104.880 / 121.360 | 7 | 58368 | unavailable |
| qr_1024x256_f64/faer | 10970.291 / 12142.083 | 10957.584 / 11765.333 | 764 | 9166852 | unavailable |
| qr_1024x256_f64/kernix | 14539.000 / 15047.959 | 14671.875 / 14845.542 | 51 | 7276544 | unavailable |
| qr_1024x256_f64/nalgebra | 23042.542 / 24598.375 | 22894.791 / 24174.375 | 4 | 4720640 | unavailable |
| qr_1024x256_f64/numpy | 11414.375 / 12781.000 | 11838.667 / 13470.375 | unavailable | unavailable | unavailable |
| qr_256x64_f64/faer | 293.625 / 315.917 | 297.320 / 302.873 | 83 | 335880 | unavailable |
| qr_256x64_f64/kernix | 548.750 / 610.041 | 546.886 / 720.759 | 6 | 426496 | unavailable |
| qr_256x64_f64/nalgebra | 382.583 / 797.125 | 427.554 / 630.432 | 4 | 295424 | unavailable |
| qr_256x64_f64/numpy | 395.500 / 503.083 | 415.360 / 484.401 | unavailable | unavailable | unavailable |
| qr_512x128_f64/faer | 1698.417 / 1766.125 | 1728.778 / 1851.188 | 360 | 3334148 | unavailable |
| qr_512x128_f64/kernix | 2620.708 / 3120.209 | 2636.312 / 2910.583 | 31 | 2098176 | unavailable |
| qr_512x128_f64/nalgebra | 3168.125 / 3859.792 | 3393.264 / 3971.833 | 4 | 1180672 | unavailable |
| qr_512x128_f64/numpy | 2729.250 / 5695.458 | 3454.597 / 5401.188 | unavailable | unavailable | unavailable |
| qr_96x32_f64/faer | 45.792 / 47.000 | 46.156 / 53.830 | 41 | 94216 | unavailable |
| qr_96x32_f64/kernix | 58.292 / 63.833 | 64.882 / 127.146 | 6 | 82176 | unavailable |
| qr_96x32_f64/nalgebra | 48.042 / 52.292 | 44.902 / 47.771 | 4 | 57600 | unavailable |
| qr_96x32_f64/numpy | 58.250 / 173.375 | 52.091 / 95.621 | unavailable | unavailable | unavailable |
| softmax_256x64_f64/candle | 82.834 / 275.333 | 74.867 / 106.911 | 37 | 265200 | unavailable |
| softmax_256x64_f64/kernix | 83.709 / 99.542 | 88.583 / 149.938 | 1 | 131072 | unavailable |
| softmax_256x64_f64/ndarray | 61.125 / 64.875 | 59.609 / 63.187 | 1 | 131072 | unavailable |
| softmax_256x64_f64/numpy | 76.167 / 92.042 | 79.945 / 86.572 | unavailable | unavailable | unavailable |
| solve_cholesky_128x1_f64/faer | 4.709 / 4.875 | 4.750 / 4.839 | 1 | 1024 | unavailable |
| solve_cholesky_128x1_f64/kernix | 15.708 / 19.542 | 16.476 / 18.449 | 1 | 1024 | unavailable |
| solve_cholesky_128x1_f64/nalgebra | 4.791 / 6.000 | 5.013 / 5.167 | 1 | 1024 | unavailable |
| solve_cholesky_256x64_f64/faer | 291.083 / 439.750 | 292.670 / 354.369 | 53 | 786440 | unavailable |
| solve_cholesky_256x64_f64/kernix | 1343.459 / 1991.834 | 1662.024 / 2667.521 | 1 | 131072 | unavailable |
| solve_cholesky_256x64_f64/nalgebra | 1037.833 / 1303.917 | 1027.175 / 1230.454 | 1 | 131072 | unavailable |
| solve_cholesky_512x128_f64/faer | 2329.209 / 3293.750 | 2141.892 / 2180.858 | 205 | 1572872 | unavailable |
| solve_cholesky_512x128_f64/kernix | 9704.709 / 9985.417 | 9736.500 / 9901.875 | 1 | 524288 | unavailable |
| solve_cholesky_512x128_f64/nalgebra | 7494.500 / 8901.667 | 7501.062 / 10082.709 | 1 | 524288 | unavailable |
| solve_lu_256x64_f64/faer | 272.208 / 325.042 | 290.100 / 396.468 | 46 | 786440 | unavailable |
| solve_lu_256x64_f64/kernix | 1148.833 / 2217.875 | 1200.833 / 1688.618 | 2 | 262144 | unavailable |
| solve_lu_256x64_f64/nalgebra | 943.834 / 1852.958 | 939.008 / 1041.192 | 1 | 131072 | unavailable |
| sparse_auto_512_f64/kernix-dense | 47.500 / 54.459 | 49.311 / 52.962 | 1 | 2097152 | unavailable |
| sparse_auto_512_f64/numpy | 126.334 / 165.250 | 131.055 / 140.076 | unavailable | unavailable | unavailable |
| sparse_auto_512_f64/scipy | 972.625 / 1344.709 | 1024.138 / 1467.530 | unavailable | unavailable | unavailable |
| sparse_matmul_512x32_f32/faer | 45.958 / 47.541 | 46.089 / 49.525 | 1 | 65536 | unavailable |
| sparse_matmul_512x32_f32/kernix-dense | 237.042 / 713.958 | 242.675 / 257.656 | 3 | 163840 | unavailable |
| sparse_matmul_512x32_f32/numpy | 30.458 / 31.166 | 30.486 / 32.972 | unavailable | unavailable | unavailable |
| sparse_matmul_512x32_f32/scipy | 13.875 / 32.750 | 14.204 / 15.231 | unavailable | unavailable | unavailable |
| sparse_matmul_512x32_f64/faer | 53.167 / 56.250 | 53.091 / 55.609 | 1 | 131072 | unavailable |
| sparse_matmul_512x32_f64/kernix-dense | 438.875 / 608.250 | 440.665 / 484.802 | 3 | 327680 | unavailable |
| sparse_matmul_512x32_f64/numpy | 86.750 / 107.167 | 86.780 / 92.463 | unavailable | unavailable | unavailable |
| sparse_matmul_512x32_f64/scipy | 49.666 / 52.292 | 21.762 / 29.086 | unavailable | unavailable | unavailable |
| sparse_matvec_2048_f64/faer | 6.042 / 6.583 | 6.081 / 6.529 | 1 | 16384 | unavailable |
| sparse_matvec_2048_f64/kernix-dense | 2396.500 / 4667.750 | 3451.361 / 5041.806 | 3 | 155648 | unavailable |
| sparse_matvec_2048_f64/numpy | 1267.250 / 1462.708 | 1283.656 / 1587.857 | unavailable | unavailable | unavailable |
| sparse_matvec_2048_f64/scipy | 8.375 / 10.500 | 13.018 / 20.448 | unavailable | unavailable | unavailable |
| sparse_product_256_f64/kernix-dense | 820.334 / 1486.958 | 790.244 / 867.382 | 3 | 1179648 | unavailable |
| sparse_product_256_f64/numpy | 124.500 / 153.958 | 109.503 / 116.078 | unavailable | unavailable | unavailable |
| sparse_product_256_f64/scipy | 43.958 / 53.250 | 49.383 / 62.706 | unavailable | unavailable | unavailable |
| tensordot_16x32x8_32x8x16_f64/kernix | 61.875 / 72.167 | 61.993 / 72.627 | 9 | 2166 | unavailable |
| tensordot_16x32x8_32x8x16_f64/numpy | 5.167 / 5.375 | 5.484 / 5.836 | unavailable | unavailable | unavailable |
| training_backward_2x4x8x4_f64/candle | 21.708 / 28.833 | 20.794 / 31.018 | 476 | 21284 | unavailable |
| training_backward_2x4x8x4_f64/kernix | 3.416 / 3.541 | 3.338 / 3.839 | 23 | 4224 | unavailable |
| training_forward_2x4x8x4_f64/candle | 16.959 / 17.875 | 5.689 / 9.324 | 126 | 9568 | unavailable |
| training_forward_2x4x8x4_f64/kernix | 2.292 / 2.917 | 2.273 / 2.723 | 17 | 1032 | unavailable |
| training_record_2x4x8x4_f64/candle | 6.250 / 7.917 | 5.316 / 6.922 | 149 | 11544 | unavailable |
| training_record_2x4x8x4_f64/kernix | 3.291 / 3.459 | 3.354 / 5.637 | 22 | 6832 | unavailable |
| training_sgd_2x4x8x4_f64/kernix | 0.750 / 0.750 | 0.859 / 1.357 | 5 | 688 | unavailable |
| training_shared_backward_64x32_f64/candle | 4.125 / 4.250 | 3.852 / 4.201 | 58 | 99996 | unavailable |
| training_shared_backward_64x32_f64/kernix | 6.000 / 7.709 | 6.043 / 7.023 | 8 | 49768 | unavailable |

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
