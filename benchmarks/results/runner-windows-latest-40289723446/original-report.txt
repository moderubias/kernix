# Clean CPU evidence: runner-windows-latest-40289723446

Measured source `7c11b2165b0cb7b523db4d938a36f1396396a138`, tree `cc7ac522fccee7e655adb7ca8d3644d95344fcb1`; dirty before/after: false.
Library source: workspace 7c11b2165b0cb7b523db4d938a36f1396396a138. CPU affinity: not pinned. Dtype explicitly named per workload; row-major inputs.

| Workload / library | Per-call p50 / p95 (us) | Batch-mean p50 / p95 (us) | Rust allocations | Peak additional requested bytes | Process peak RSS (MiB) |
|---|---:|---:|---:|---:|---:|
| cholesky_factor_128_f64/faer | 59.600 / 71.400 | 60.862 / 64.169 | 2 | 132096 | unavailable |
| cholesky_factor_128_f64/kernix | 220.600 / 249.900 | 225.944 / 328.123 | 1 | 131072 | unavailable |
| cholesky_factor_128_f64/nalgebra | 117.600 / 148.400 | 119.768 / 126.211 | 1 | 131072 | unavailable |
| cholesky_factor_128_f64/numpy | 114.200 / 156.200 | 151.567 / 172.468 | unavailable | unavailable | unavailable |
| cholesky_factor_256_f64/faer | 314.100 / 351.400 | 322.745 / 345.124 | 2 | 526336 | unavailable |
| cholesky_factor_256_f64/kernix | 705.700 / 776.800 | 712.033 / 728.571 | 49 | 645120 | unavailable |
| cholesky_factor_256_f64/nalgebra | 853.900 / 931.400 | 854.725 / 886.500 | 1 | 524288 | unavailable |
| cholesky_factor_256_f64/numpy | 947.100 / 1038.900 | 958.918 / 1141.767 | unavailable | unavailable | unavailable |
| cholesky_factor_512_f64/faer | 3030.900 / 3384.600 | 3055.400 / 3090.400 | 2 | 2101248 | unavailable |
| cholesky_factor_512_f64/kernix | 4659.200 / 6287.400 | 4640.467 / 4995.267 | 161 | 2269184 | unavailable |
| cholesky_factor_512_f64/nalgebra | 8121.800 / 8579.400 | 8131.150 / 11845.800 | 1 | 2097152 | unavailable |
| cholesky_factor_512_f64/numpy | 6826.600 / 7159.100 | 6855.450 / 7349.350 | unavailable | unavailable | unavailable |
| cholesky_factor_64_f64/faer | 14.700 / 15.400 | 14.974 / 15.369 | 2 | 33280 | unavailable |
| cholesky_factor_64_f64/kernix | 37.500 / 38.000 | 42.336 / 71.037 | 2 | 33280 | unavailable |
| cholesky_factor_64_f64/nalgebra | 21.000 / 21.200 | 21.476 / 22.119 | 1 | 32768 | unavailable |
| cholesky_factor_64_f64/numpy | 21.200 / 22.800 | 22.089 / 22.377 | unavailable | unavailable | unavailable |
| cholesky_rank_downdate_256_f64/kernix | 167.900 / 200.000 | 171.634 / 175.821 | 3 | 526336 | unavailable |
| cholesky_rank_downdate_256_f64/nalgebra | 415.000 / 463.000 | 435.687 / 576.233 | 2 | 526336 | unavailable |
| cholesky_rank_downdate_256_f64/refactor | 1105.600 / 1169.800 | 970.409 / 1067.150 | 49 | 645120 | unavailable |
| cholesky_rank_update_256_f64/kernix | 169.500 / 201.300 | 174.166 / 179.266 | 3 | 526336 | unavailable |
| cholesky_rank_update_256_f64/nalgebra | 402.100 / 439.800 | 410.812 / 414.840 | 2 | 526336 | unavailable |
| cholesky_rank_update_256_f64/refactor | 1106.100 / 1187.900 | 1028.140 / 1409.013 | 49 | 645120 | unavailable |
| cholesky_rank_update_512_f64/kernix | 1341.000 / 1426.700 | 1383.825 / 1712.050 | 3 | 2101248 | unavailable |
| cholesky_rank_update_512_f64/nalgebra | 558.800 / 598.500 | 565.022 / 571.628 | 2 | 2101248 | unavailable |
| cholesky_rank_update_512_f64/refactor | 4640.400 / 6551.500 | 4646.300 / 4783.467 | 161 | 2269184 | unavailable |
| create_ones_64x64_f32/candle | 0.700 / 0.700 | 0.698 / 0.719 | 7 | 16696 | unavailable |
| create_ones_64x64_f32/faer | 0.500 / 0.500 | 0.452 / 0.453 | 1 | 16384 | unavailable |
| create_ones_64x64_f32/kernix | 0.500 / 0.500 | 0.481 / 0.520 | 1 | 16384 | unavailable |
| create_ones_64x64_f32/ndarray | 0.400 / 0.400 | 0.389 / 0.390 | 1 | 16384 | unavailable |
| create_ones_64x64_f32/numpy | 2.100 / 2.200 | 2.246 / 2.323 | unavailable | unavailable | unavailable |
| create_zeros_64x64_f32/candle | 0.500 / 0.500 | 0.482 / 0.513 | 5 | 16648 | unavailable |
| create_zeros_64x64_f32/faer | 0.500 / 0.500 | 0.485 / 0.511 | 1 | 16384 | unavailable |
| create_zeros_64x64_f32/kernix | 0.400 / 0.400 | 0.371 / 0.439 | 1 | 16384 | unavailable |
| create_zeros_64x64_f32/ndarray | 0.300 / 0.300 | 0.276 / 0.281 | 1 | 16384 | unavailable |
| create_zeros_64x64_f32/numpy | 0.800 / 0.900 | 0.908 / 0.932 | unavailable | unavailable | unavailable |
| matmul_128_f32/candle | 75.300 / 114.300 | 68.844 / 79.006 | 10 | 65816 | unavailable |
| matmul_128_f32/faer | 50.200 / 52.700 | 52.218 / 58.625 | 1 | 65536 | unavailable |
| matmul_128_f32/kernix | 74.400 / 78.100 | 75.859 / 90.139 | 3 | 65544 | unavailable |
| matmul_128_f32/nalgebra | 53.800 / 54.600 | 54.697 / 97.435 | 2 | 163840 | unavailable |
| matmul_128_f32/ndarray | 128.200 / 197.400 | 194.413 / 1161.247 | 2 | 163840 | unavailable |
| matmul_128_f32/numpy | 56.200 / 61.000 | 57.578 / 70.240 | unavailable | unavailable | unavailable |
| matmul_128_f64/candle | 150.400 / 231.500 | 156.805 / 209.517 | 10 | 131352 | unavailable |
| matmul_128_f64/faer | 107.300 / 141.300 | 112.602 / 123.726 | 1 | 131072 | unavailable |
| matmul_128_f64/kernix | 94.900 / 122.300 | 128.728 / 205.698 | 3 | 131080 | unavailable |
| matmul_128_f64/nalgebra | 115.900 / 134.600 | 118.054 / 118.568 | 2 | 327680 | unavailable |
| matmul_128_f64/ndarray | 114.400 / 138.700 | 116.528 / 123.891 | 2 | 327680 | unavailable |
| matmul_128_f64/numpy | 101.700 / 123.300 | 104.630 / 117.860 | unavailable | unavailable | unavailable |
| matmul_256_f32/candle | 451.800 / 549.900 | 449.952 / 457.377 | 10 | 262424 | unavailable |
| matmul_256_f32/faer | 430.400 / 474.300 | 441.804 / 701.513 | 1 | 262144 | unavailable |
| matmul_256_f32/kernix | 433.300 / 488.700 | 443.491 / 732.986 | 3 | 262152 | unavailable |
| matmul_256_f32/nalgebra | 398.000 / 435.900 | 405.576 / 573.106 | 2 | 589824 | unavailable |
| matmul_256_f32/ndarray | 390.800 / 458.600 | 397.931 / 445.439 | 2 | 589824 | unavailable |
| matmul_256_f32/numpy | 388.700 / 428.800 | 395.758 / 400.632 | unavailable | unavailable | unavailable |
| matmul_256_f64/candle | 856.000 / 891.100 | 859.200 / 926.927 | 10 | 524568 | unavailable |
| matmul_256_f64/faer | 1213.600 / 1541.700 | 1269.487 / 1393.250 | 1 | 524288 | unavailable |
| matmul_256_f64/kernix | 865.000 / 952.600 | 863.900 / 912.927 | 3 | 524296 | unavailable |
| matmul_256_f64/nalgebra | 905.100 / 934.600 | 909.136 / 949.000 | 2 | 1179648 | unavailable |
| matmul_256_f64/ndarray | 1181.800 / 1237.100 | 1189.789 / 1213.078 | 2 | 1179648 | unavailable |
| matmul_256_f64/numpy | 786.400 / 1024.600 | 780.277 / 909.927 | unavailable | unavailable | unavailable |
| matmul_32_f64/candle | 4.500 / 5.000 | 4.650 / 5.097 | 10 | 8528 | unavailable |
| matmul_32_f64/faer | 2.500 / 2.600 | 2.491 / 2.759 | 1 | 8192 | unavailable |
| matmul_32_f64/kernix | 2.800 / 2.900 | 2.830 / 2.873 | 3 | 24576 | unavailable |
| matmul_32_f64/nalgebra | 3.500 / 3.700 | 2.741 / 3.760 | 2 | 24576 | unavailable |
| matmul_32_f64/ndarray | 2.400 / 2.500 | 2.417 / 2.439 | 2 | 24576 | unavailable |
| matmul_32_f64/numpy | 6.700 / 7.100 | 4.397 / 5.738 | unavailable | unavailable | unavailable |
| matmul_64_f32/candle | 7.400 / 10.500 | 7.323 / 7.933 | 10 | 16664 | unavailable |
| matmul_64_f32/faer | 7.000 / 7.100 | 7.291 / 7.495 | 1 | 16384 | unavailable |
| matmul_64_f32/kernix | 6.900 / 7.000 | 6.814 / 6.937 | 3 | 16512 | unavailable |
| matmul_64_f32/nalgebra | 30.100 / 31.900 | 9.966 / 30.630 | 2 | 49152 | unavailable |
| matmul_64_f32/ndarray | 30.000 / 32.000 | 9.742 / 30.749 | 2 | 49152 | unavailable |
| matmul_64_f32/numpy | 10.400 / 10.500 | 10.671 / 11.570 | unavailable | unavailable | unavailable |
| matmul_64_f64/candle | 14.200 / 14.400 | 14.306 / 14.573 | 10 | 33048 | unavailable |
| matmul_64_f64/faer | 14.600 / 14.700 | 14.874 / 14.949 | 1 | 32768 | unavailable |
| matmul_64_f64/kernix | 13.600 / 13.700 | 13.712 / 14.538 | 3 | 32896 | unavailable |
| matmul_64_f64/nalgebra | 57.900 / 72.100 | 33.268 / 51.469 | 2 | 98304 | unavailable |
| matmul_64_f64/ndarray | 60.000 / 61.900 | 59.907 / 68.618 | 2 | 98304 | unavailable |
| matmul_64_f64/numpy | 16.000 / 16.500 | 17.276 / 24.832 | unavailable | unavailable | unavailable |
| matmul_ab_128_f32/kernix | 74.800 / 83.400 | 76.536 / 82.762 | 3 | 65544 | unavailable |
| matmul_ab_128_f32/numpy | 56.200 / 69.200 | 57.832 / 59.438 | unavailable | unavailable | unavailable |
| matmul_ab_128_f64/kernix | 142.100 / 164.500 | 144.726 / 146.828 | 3 | 131080 | unavailable |
| matmul_ab_128_f64/numpy | 101.800 / 135.400 | 103.965 / 104.859 | unavailable | unavailable | unavailable |
| matmul_batched_4x16x32x16_f32/candle | 2.900 / 3.000 | 3.251 / 3.261 | 26 | 13152 | unavailable |
| matmul_batched_4x16x32x16_f32/kernix | 1.400 / 1.500 | 1.507 / 1.578 | 3 | 4104 | unavailable |
| matmul_batched_4x16x32x16_f32/numpy | 4.800 / 4.900 | 5.013 / 5.143 | unavailable | unavailable | unavailable |
| matmul_batched_4x16x32x16_f64/candle | 3.900 / 4.100 | 4.333 / 4.361 | 26 | 25440 | unavailable |
| matmul_batched_4x16x32x16_f64/kernix | 2.400 / 2.400 | 2.385 / 2.426 | 3 | 8200 | unavailable |
| matmul_batched_4x16x32x16_f64/numpy | 5.000 / 5.200 | 5.196 / 5.416 | unavailable | unavailable | unavailable |
| matmul_sum_backward_32x16x8_f64/candle | 9.900 / 10.200 | 9.710 / 10.087 | 116 | 27988 | unavailable |
| matmul_sum_backward_32x16x8_f64/kernix | 4.000 / 4.100 | 4.079 / 4.107 | 15 | 18376 | unavailable |
| matmul_transposed_128_f32/kernix | 98.800 / 134.000 | 87.480 / 125.478 | 4 | 458760 | unavailable |
| matmul_transposed_128_f32/numpy | 60.500 / 63.800 | 61.738 / 63.344 | unavailable | unavailable | unavailable |
| matmul_transposed_128_f64/kernix | 199.700 / 234.000 | 203.530 / 214.764 | 4 | 917512 | unavailable |
| matmul_transposed_128_f64/numpy | 107.300 / 129.200 | 115.733 / 176.547 | unavailable | unavailable | unavailable |
| min_norm_32x64_rank32_f64/kernix | 114.300 / 150.000 | 86.241 / 131.811 | 20 | 59136 | unavailable |
| min_norm_32x64_rank32_f64/nalgebra | 175.200 / 205.900 | 178.456 / 181.984 | 15 | 42232 | unavailable |
| min_norm_32x64_rank32_f64/numpy | 203.700 / 255.000 | 211.096 / 220.609 | unavailable | unavailable | unavailable |
| min_norm_64x32_rank16_f64/kernix | 96.800 / 119.900 | 64.572 / 79.402 | 20 | 74752 | unavailable |
| min_norm_64x32_rank16_f64/nalgebra | 103.500 / 129.600 | 105.463 / 195.046 | 15 | 41976 | unavailable |
| min_norm_64x32_rank16_f64/numpy | 112.800 / 159.200 | 118.180 / 200.992 | unavailable | unavailable | unavailable |
| parallel_matmul_into_128_f32/kernix | 60.300 / 97.000 | 67.495 / 71.023 | 2 | 16 | unavailable |
| parallel_matmul_into_128_f64/kernix | 83.900 / 130.000 | 118.634 / 152.705 | 2 | 16 | unavailable |
| parallel_matmul_into_256_f64/kernix | 607.900 / 810.100 | 491.591 / 808.554 | 2 | 16 | unavailable |
| parallel_matmul_into_64_f64/kernix | 41.100 / 71.700 | 62.103 / 73.686 | 2 | 98304 | unavailable |
| pivoted_qr_256x64_f64/faer | 410.900 / 452.400 | 367.579 / 393.685 | 10 | 333312 | unavailable |
| pivoted_qr_256x64_f64/kernix | 906.500 / 951.000 | 911.745 / 1411.425 | 14 | 559104 | unavailable |
| pivoted_qr_256x64_f64/nalgebra | 917.300 / 981.500 | 917.700 / 926.145 | 7 | 296960 | unavailable |
| pivoted_qr_96x32_f64/faer | 69.900 / 75.700 | 51.314 / 72.506 | 10 | 68352 | unavailable |
| pivoted_qr_96x32_f64/kernix | 184.400 / 242.100 | 130.790 / 212.319 | 14 | 107520 | unavailable |
| pivoted_qr_96x32_f64/nalgebra | 106.300 / 125.300 | 104.507 / 111.407 | 7 | 58368 | unavailable |
| qr_1024x256_f64/faer | 13013.900 / 15681.800 | 12926.700 / 19201.300 | 7 | 5308416 | unavailable |
| qr_1024x256_f64/kernix | 21273.400 / 24189.700 | 20834.300 / 21516.000 | 82 | 7145480 | unavailable |
| qr_1024x256_f64/nalgebra | 32876.900 / 40146.000 | 32737.900 / 33510.100 | 4 | 4720640 | unavailable |
| qr_1024x256_f64/numpy | 21444.000 / 22654.400 | 21682.000 / 22222.900 | unavailable | unavailable | unavailable |
| qr_256x64_f64/faer | 388.200 / 442.800 | 417.637 / 425.700 | 7 | 331776 | unavailable |
| qr_256x64_f64/kernix | 771.500 / 827.700 | 784.469 / 832.900 | 6 | 426496 | unavailable |
| qr_256x64_f64/nalgebra | 580.000 / 621.500 | 591.947 / 649.125 | 4 | 295424 | unavailable |
| qr_256x64_f64/numpy | 548.000 / 603.100 | 557.772 / 572.722 | unavailable | unavailable | unavailable |
| qr_512x128_f64/faer | 2114.100 / 2191.400 | 2081.480 / 2583.640 | 7 | 1343488 | unavailable |
| qr_512x128_f64/kernix | 4076.500 / 6042.800 | 3855.333 / 5898.050 | 49 | 2294792 | unavailable |
| qr_512x128_f64/nalgebra | 4001.900 / 4217.900 | 4201.500 / 5325.033 | 4 | 1180672 | unavailable |
| qr_512x128_f64/numpy | 4854.200 / 7045.900 | 6028.700 / 9115.300 | unavailable | unavailable | unavailable |
| qr_96x32_f64/faer | 86.000 / 98.800 | 67.210 / 88.061 | 7 | 67584 | unavailable |
| qr_96x32_f64/kernix | 100.200 / 131.100 | 106.474 / 118.864 | 6 | 82176 | unavailable |
| qr_96x32_f64/nalgebra | 79.600 / 97.600 | 80.988 / 83.338 | 4 | 57600 | unavailable |
| qr_96x32_f64/numpy | 76.700 / 119.600 | 81.493 / 101.063 | unavailable | unavailable | unavailable |
| softmax_256x64_f64/candle | 184.700 / 221.800 | 135.007 / 150.225 | 37 | 265200 | unavailable |
| softmax_256x64_f64/kernix | 112.500 / 140.900 | 114.401 / 115.623 | 1 | 131072 | unavailable |
| softmax_256x64_f64/ndarray | 153.700 / 182.700 | 156.373 / 160.084 | 1 | 131072 | unavailable |
| softmax_256x64_f64/numpy | 131.500 / 159.400 | 135.520 / 189.840 | unavailable | unavailable | unavailable |
| solve_cholesky_128x1_f64/faer | 6.500 / 10.600 | 7.328 / 14.514 | 1 | 1024 | unavailable |
| solve_cholesky_128x1_f64/kernix | 18.200 / 18.300 | 18.929 / 19.594 | 1 | 1024 | unavailable |
| solve_cholesky_128x1_f64/nalgebra | 6.300 / 6.400 | 6.010 / 6.202 | 1 | 1024 | unavailable |
| solve_cholesky_256x64_f64/faer | 328.900 / 368.400 | 344.403 / 379.721 | 1 | 131072 | unavailable |
| solve_cholesky_256x64_f64/kernix | 1395.700 / 1594.400 | 1386.975 / 1420.300 | 1 | 131072 | unavailable |
| solve_cholesky_256x64_f64/nalgebra | 1326.800 / 1366.500 | 1336.975 / 2224.240 | 1 | 131072 | unavailable |
| solve_cholesky_512x128_f64/faer | 2259.900 / 2331.900 | 2263.620 / 2334.380 | 1 | 524288 | unavailable |
| solve_cholesky_512x128_f64/kernix | 10320.400 / 10881.700 | 10329.600 / 10768.200 | 1 | 524288 | unavailable |
| solve_cholesky_512x128_f64/nalgebra | 10418.700 / 14208.600 | 10506.500 / 11545.400 | 1 | 524288 | unavailable |
| solve_lu_256x64_f64/faer | 373.900 / 425.600 | 383.007 / 549.695 | 2 | 262144 | unavailable |
| solve_lu_256x64_f64/kernix | 1302.100 / 1337.800 | 1293.162 / 1369.763 | 2 | 262144 | unavailable |
| solve_lu_256x64_f64/nalgebra | 1177.900 / 1213.700 | 1172.644 / 1461.775 | 1 | 131072 | unavailable |
| sparse_auto_512_f64/kernix | 1132.200 / 1182.700 | 1099.210 / 1431.975 | 4 | 2142216 | unavailable |
| sparse_auto_512_f64/kernix-dense | 386.400 / 431.700 | 398.788 / 533.700 | 1 | 2097152 | unavailable |
| sparse_auto_512_f64/numpy | 414.200 / 546.900 | 451.326 / 558.350 | unavailable | unavailable | unavailable |
| sparse_auto_512_f64/scipy | 1468.900 / 1576.200 | 1472.843 / 1681.283 | unavailable | unavailable | unavailable |
| sparse_matmul_512x32_f32/faer | 95.300 / 112.100 | 96.620 / 99.520 | 1 | 65536 | unavailable |
| sparse_matmul_512x32_f32/kernix | 20.800 / 20.900 | 31.889 / 40.345 | 1 | 65536 | unavailable |
| sparse_matmul_512x32_f32/kernix-dense | 333.200 / 368.100 | 346.786 / 536.158 | 3 | 163840 | unavailable |
| sparse_matmul_512x32_f32/numpy | 220.300 / 254.000 | 223.747 / 242.960 | unavailable | unavailable | unavailable |
| sparse_matmul_512x32_f32/scipy | 26.500 / 28.100 | 27.146 / 28.137 | unavailable | unavailable | unavailable |
| sparse_matmul_512x32_f64/faer | 67.500 / 93.800 | 69.753 / 71.616 | 1 | 131072 | unavailable |
| sparse_matmul_512x32_f64/kernix | 69.200 / 71.100 | 36.501 / 39.607 | 1 | 131072 | unavailable |
| sparse_matmul_512x32_f64/kernix-dense | 627.900 / 695.400 | 621.594 / 630.119 | 3 | 327680 | unavailable |
| sparse_matmul_512x32_f64/numpy | 469.900 / 515.600 | 472.095 / 497.181 | unavailable | unavailable | unavailable |
| sparse_matmul_512x32_f64/scipy | 42.500 / 56.200 | 44.331 / 55.686 | unavailable | unavailable | unavailable |
| sparse_matvec_2048_f64/faer | 7.900 / 7.900 | 8.020 / 13.282 | 1 | 16384 | unavailable |
| sparse_matvec_2048_f64/kernix | 19.300 / 19.500 | 19.728 / 20.507 | 1 | 16384 | unavailable |
| sparse_matvec_2048_f64/kernix-dense | 2588.100 / 2837.600 | 2769.450 / 2845.725 | 3 | 155648 | unavailable |
| sparse_matvec_2048_f64/numpy | 815.400 / 995.500 | 953.673 / 1220.122 | unavailable | unavailable | unavailable |
| sparse_matvec_2048_f64/scipy | 11.200 / 17.400 | 12.040 / 21.771 | unavailable | unavailable | unavailable |
| sparse_product_256_f64/kernix | 150.300 / 182.400 | 139.014 / 151.524 | 4 | 67752 | unavailable |
| sparse_product_256_f64/kernix-dense | 756.700 / 802.200 | 774.777 / 900.967 | 3 | 524296 | unavailable |
| sparse_product_256_f64/numpy | 759.300 / 1107.300 | 759.786 / 782.931 | unavailable | unavailable | unavailable |
| sparse_product_256_f64/scipy | 68.300 / 80.500 | 69.735 / 82.171 | unavailable | unavailable | unavailable |
| tensordot_16x32x8_32x8x16_f64/kernix | 63.900 / 82.100 | 70.537 / 92.978 | 9 | 2166 | unavailable |
| tensordot_16x32x8_32x8x16_f64/numpy | 12.100 / 13.500 | 12.402 / 12.783 | unavailable | unavailable | unavailable |
| training_backward_2x4x8x4_f64/candle | 32.000 / 34.400 | 32.974 / 42.976 | 474 | 8332 | unavailable |
| training_backward_2x4x8x4_f64/kernix | 6.000 / 6.100 | 5.878 / 6.128 | 23 | 4224 | unavailable |
| training_forward_2x4x8x4_f64/candle | 8.200 / 8.300 | 8.166 / 8.663 | 127 | 3912 | unavailable |
| training_forward_2x4x8x4_f64/kernix | 3.600 / 3.700 | 3.638 / 4.193 | 17 | 1032 | unavailable |
| training_record_2x4x8x4_f64/candle | 9.700 / 10.600 | 9.885 / 10.368 | 150 | 10712 | unavailable |
| training_record_2x4x8x4_f64/kernix | 5.000 / 5.100 | 4.890 / 8.750 | 22 | 6832 | unavailable |
| training_sgd_2x4x8x4_f64/kernix | 1.000 / 1.800 | 1.025 / 1.200 | 5 | 688 | unavailable |
| training_shared_backward_64x32_f64/candle | 58.000 / 72.600 | 7.073 / 7.141 | 58 | 100004 | unavailable |
| training_shared_backward_64x32_f64/kernix | 33.100 / 37.500 | 10.562 / 15.629 | 8 | 49768 | unavailable |

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
