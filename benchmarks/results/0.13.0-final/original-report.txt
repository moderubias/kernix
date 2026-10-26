# Clean CPU evidence: 0.13.0-final

Measured source `7c11b2165b0cb7b523db4d938a36f1396396a138`, tree `cc7ac522fccee7e655adb7ca8d3644d95344fcb1`; dirty before/after: false.
Library source: workspace 7c11b2165b0cb7b523db4d938a36f1396396a138. CPU affinity: 0. Dtype explicitly named per workload; row-major inputs.

| Workload / library | Per-call p50 / p95 (us) | Batch-mean p50 / p95 (us) | Rust allocations | Peak additional requested bytes | Process peak RSS (MiB) |
|---|---:|---:|---:|---:|---:|
| cholesky_factor_128_f64/faer | 40.685 / 54.652 | 41.295 / 45.681 | 2 | 132096 | 4.50 |
| cholesky_factor_128_f64/kernix | 124.647 / 150.760 | 128.162 / 131.843 | 1 | 131072 | 4.81 |
| cholesky_factor_128_f64/nalgebra | 97.744 / 102.974 | 95.706 / 97.018 | 1 | 131072 | 4.24 |
| cholesky_factor_128_f64/numpy | 72.767 / 79.062 | 73.319 / 76.417 | unavailable | unavailable | 37.64 |
| cholesky_factor_256_f64/faer | 211.038 / 242.028 | 217.128 / 234.310 | 2 | 526336 | 5.87 |
| cholesky_factor_256_f64/kernix | 532.935 / 610.260 | 546.813 / 562.725 | 49 | 645120 | 6.08 |
| cholesky_factor_256_f64/nalgebra | 721.137 / 806.053 | 741.685 / 784.497 | 1 | 524288 | 4.97 |
| cholesky_factor_256_f64/numpy | 538.751 / 702.225 | 557.197 / 588.562 | unavailable | unavailable | 41.59 |
| cholesky_factor_512_f64/faer | 1501.998 / 1758.776 | 1520.431 / 1598.440 | 2 | 2101248 | 9.45 |
| cholesky_factor_512_f64/kernix | 3120.465 / 3852.521 | 3115.432 / 3529.379 | 161 | 2269184 | 10.72 |
| cholesky_factor_512_f64/nalgebra | 5562.141 / 6245.141 | 5566.124 / 5952.788 | 1 | 2097152 | 7.96 |
| cholesky_factor_512_f64/numpy | 4526.544 / 12226.069 | 7144.703 / 11469.055 | unavailable | unavailable | 53.44 |
| cholesky_factor_64_f64/faer | 9.145 / 9.240 | 9.207 / 9.882 | 2 | 33280 | 4.59 |
| cholesky_factor_64_f64/kernix | 20.337 / 22.575 | 21.423 / 23.337 | 2 | 33280 | 4.30 |
| cholesky_factor_64_f64/nalgebra | 14.147 / 17.490 | 14.543 / 15.401 | 1 | 32768 | 4.05 |
| cholesky_factor_64_f64/numpy | 11.160 / 12.321 | 11.352 / 12.063 | unavailable | unavailable | 36.88 |
| cholesky_rank_downdate_256_f64/kernix | 116.086 / 138.751 | 118.031 / 124.378 | 3 | 526336 | 6.01 |
| cholesky_rank_downdate_256_f64/nalgebra | 25.679 / 30.561 | 27.609 / 33.284 | 2 | 526336 | 6.68 |
| cholesky_rank_downdate_256_f64/refactor | 560.177 / 703.439 | 544.570 / 593.986 | 49 | 645120 | 6.74 |
| cholesky_rank_update_256_f64/kernix | 125.068 / 159.027 | 126.359 / 136.631 | 3 | 526336 | 5.57 |
| cholesky_rank_update_256_f64/nalgebra | 24.466 / 25.185 | 25.308 / 26.673 | 2 | 526336 | 6.59 |
| cholesky_rank_update_256_f64/refactor | 514.278 / 539.200 | 515.077 / 535.089 | 49 | 645120 | 6.94 |
| cholesky_rank_update_512_f64/kernix | 767.096 / 1390.923 | 801.661 / 1040.823 | 3 | 2101248 | 10.08 |
| cholesky_rank_update_512_f64/nalgebra | 152.543 / 320.891 | 194.293 / 300.069 | 2 | 2101248 | 14.09 |
| cholesky_rank_update_512_f64/refactor | 3179.033 / 4636.311 | 3211.693 / 3900.278 | 161 | 2269184 | 12.84 |
| create_ones_64x64_f32/candle | 0.249 / 0.262 | 0.253 / 0.273 | 7 | 16696 | 4.70 |
| create_ones_64x64_f32/faer | 0.204 / 0.219 | 0.191 / 0.199 | 1 | 16384 | 4.32 |
| create_ones_64x64_f32/kernix | 0.191 / 0.194 | 0.197 / 0.204 | 1 | 16384 | 3.98 |
| create_ones_64x64_f32/ndarray | 0.154 / 0.155 | 0.155 / 0.165 | 1 | 16384 | 4.01 |
| create_ones_64x64_f32/numpy | 1.330 / 1.371 | 1.376 / 1.462 | unavailable | unavailable | 35.95 |
| create_zeros_64x64_f32/candle | 0.161 / 0.215 | 0.165 / 0.173 | 5 | 16648 | 4.56 |
| create_zeros_64x64_f32/faer | 0.165 / 0.177 | 0.204 / 0.212 | 1 | 16384 | 3.83 |
| create_zeros_64x64_f32/kernix | 0.148 / 0.151 | 0.152 / 0.163 | 1 | 16384 | 4.26 |
| create_zeros_64x64_f32/ndarray | 0.098 / 0.100 | 0.104 / 0.112 | 1 | 16384 | 3.96 |
| create_zeros_64x64_f32/numpy | 0.388 / 0.416 | 0.413 / 0.426 | unavailable | unavailable | 35.96 |
| matmul_128_f32/candle | 39.874 / 44.781 | 40.758 / 43.275 | 9 | 65816 | 4.82 |
| matmul_128_f32/faer | 42.827 / 47.013 | 43.451 / 45.063 | 1 | 65536 | 4.54 |
| matmul_128_f32/kernix | 38.804 / 47.911 | 40.076 / 41.924 | 3 | 65544 | 5.04 |
| matmul_128_f32/nalgebra | 54.080 / 63.182 | 54.922 / 59.499 | 2 | 163840 | 4.27 |
| matmul_128_f32/ndarray | 53.745 / 94.859 | 57.447 / 61.074 | 2 | 163840 | 4.54 |
| matmul_128_f32/numpy | 39.017 / 42.834 | 39.690 / 40.272 | unavailable | unavailable | 37.68 |
| matmul_128_f64/candle | 79.583 / 103.685 | 82.325 / 105.639 | 9 | 131352 | 5.55 |
| matmul_128_f64/faer | 87.443 / 92.952 | 89.206 / 93.540 | 1 | 131072 | 4.78 |
| matmul_128_f64/kernix | 78.164 / 84.553 | 80.829 / 85.650 | 3 | 131080 | 5.34 |
| matmul_128_f64/nalgebra | 118.692 / 148.172 | 121.488 / 125.879 | 2 | 327680 | 4.44 |
| matmul_128_f64/ndarray | 130.215 / 172.883 | 134.876 / 149.988 | 2 | 327680 | 4.46 |
| matmul_128_f64/numpy | 83.555 / 103.294 | 86.108 / 92.550 | unavailable | unavailable | 37.82 |
| matmul_256_f32/candle | 303.014 / 326.927 | 310.155 / 334.233 | 9 | 262424 | 6.11 |
| matmul_256_f32/faer | 341.562 / 374.572 | 342.340 / 363.501 | 1 | 262144 | 5.26 |
| matmul_256_f32/kernix | 284.651 / 293.351 | 286.080 / 292.171 | 3 | 262152 | 5.53 |
| matmul_256_f32/nalgebra | 419.397 / 496.109 | 458.882 / 546.445 | 2 | 589824 | 4.91 |
| matmul_256_f32/ndarray | 426.304 / 510.200 | 443.363 / 568.890 | 2 | 589824 | 5.02 |
| matmul_256_f32/numpy | 300.744 / 353.905 | 305.252 / 323.150 | unavailable | unavailable | 41.71 |
| matmul_256_f64/candle | 576.856 / 598.462 | 580.649 / 592.317 | 9 | 524568 | 7.01 |
| matmul_256_f64/faer | 762.531 / 924.687 | 764.043 / 801.922 | 1 | 524288 | 5.88 |
| matmul_256_f64/kernix | 619.081 / 746.090 | 626.978 / 669.937 | 3 | 524296 | 5.91 |
| matmul_256_f64/nalgebra | 662.704 / 696.277 | 670.942 / 731.194 | 2 | 1179648 | 5.79 |
| matmul_256_f64/ndarray | 692.011 / 850.881 | 710.101 / 745.720 | 2 | 1179648 | 6.26 |
| matmul_256_f64/numpy | 640.902 / 761.889 | 648.067 / 692.009 | unavailable | unavailable | 42.55 |
| matmul_32_f64/candle | 1.570 / 1.873 | 1.636 / 1.700 | 9 | 8592 | 5.18 |
| matmul_32_f64/faer | 1.483 / 1.613 | 1.640 / 1.740 | 1 | 8192 | 4.39 |
| matmul_32_f64/kernix | 1.763 / 1.861 | 1.855 / 2.951 | 3 | 24576 | 4.57 |
| matmul_32_f64/nalgebra | 1.669 / 2.139 | 1.554 / 1.627 | 2 | 24576 | 4.11 |
| matmul_32_f64/ndarray | 1.610 / 1.969 | 1.663 / 1.797 | 2 | 24576 | 4.12 |
| matmul_32_f64/numpy | 2.748 / 2.823 | 2.860 / 3.056 | unavailable | unavailable | 36.64 |
| matmul_64_f32/candle | 4.990 / 5.068 | 5.226 / 5.501 | 9 | 16664 | 4.58 |
| matmul_64_f32/faer | 5.556 / 5.909 | 5.698 / 6.537 | 1 | 16384 | 4.55 |
| matmul_64_f32/kernix | 5.167 / 5.285 | 5.521 / 6.195 | 3 | 16544 | 4.76 |
| matmul_64_f32/nalgebra | 5.638 / 6.071 | 5.852 / 6.800 | 2 | 49152 | 4.04 |
| matmul_64_f32/ndarray | 6.203 / 6.908 | 6.223 / 6.384 | 2 | 49152 | 4.09 |
| matmul_64_f32/numpy | 6.631 / 7.250 | 6.831 / 7.159 | unavailable | unavailable | 36.66 |
| matmul_64_f64/candle | 10.478 / 12.046 | 10.921 / 11.238 | 9 | 33048 | 5.36 |
| matmul_64_f64/faer | 11.485 / 12.638 | 11.917 / 12.299 | 1 | 32768 | 4.57 |
| matmul_64_f64/kernix | 10.383 / 12.457 | 10.921 / 11.353 | 3 | 32928 | 5.00 |
| matmul_64_f64/nalgebra | 11.009 / 12.179 | 11.764 / 12.235 | 2 | 98304 | 4.37 |
| matmul_64_f64/ndarray | 11.606 / 11.683 | 12.307 / 12.831 | 2 | 98304 | 4.38 |
| matmul_64_f64/numpy | 12.269 / 13.248 | 12.871 / 13.122 | unavailable | unavailable | 36.75 |
| matmul_ab_128_f32/kernix | 37.833 / 40.210 | 38.958 / 40.347 | 3 | 65544 | 5.14 |
| matmul_ab_128_f32/numpy | 40.065 / 43.288 | 41.184 / 43.641 | unavailable | unavailable | 37.38 |
| matmul_ab_128_f64/kernix | 72.444 / 77.832 | 74.391 / 75.256 | 3 | 131080 | 5.12 |
| matmul_ab_128_f64/numpy | 76.145 / 86.355 | 77.836 / 79.454 | unavailable | unavailable | 37.82 |
| matmul_batched_4x16x32x16_f32/candle | 1.462 / 1.527 | 1.530 / 1.764 | 25 | 13216 | 5.59 |
| matmul_batched_4x16x32x16_f32/kernix | 1.067 / 1.084 | 1.093 / 1.195 | 3 | 4104 | 4.53 |
| matmul_batched_4x16x32x16_f32/numpy | 2.575 / 4.979 | 2.581 / 2.701 | unavailable | unavailable | 36.41 |
| matmul_batched_4x16x32x16_f64/candle | 2.358 / 2.435 | 2.379 / 2.504 | 25 | 25504 | 5.57 |
| matmul_batched_4x16x32x16_f64/kernix | 1.786 / 1.802 | 1.820 / 1.954 | 3 | 8200 | 4.52 |
| matmul_batched_4x16x32x16_f64/numpy | 3.274 / 5.476 | 3.424 / 3.677 | unavailable | unavailable | 36.41 |
| matmul_sum_backward_32x16x8_f64/candle | 4.177 / 4.467 | 4.268 / 4.777 | 113 | 27988 | 5.73 |
| matmul_sum_backward_32x16x8_f64/kernix | 2.397 / 2.484 | 2.531 / 2.886 | 15 | 18376 | 4.39 |
| matmul_transposed_128_f32/kernix | 51.166 / 61.321 | 52.880 / 57.941 | 3 | 163840 | 4.88 |
| matmul_transposed_128_f32/numpy | 40.474 / 42.961 | 42.369 / 45.068 | unavailable | unavailable | 37.36 |
| matmul_transposed_128_f64/kernix | 114.249 / 130.178 | 117.204 / 148.944 | 3 | 327680 | 5.09 |
| matmul_transposed_128_f64/numpy | 78.262 / 97.093 | 81.181 / 84.963 | unavailable | unavailable | 37.79 |
| min_norm_32x64_rank32_f64/kernix | 63.897 / 70.423 | 64.058 / 66.719 | 20 | 59136 | 4.84 |
| min_norm_32x64_rank32_f64/nalgebra | 142.803 / 152.535 | 144.447 / 153.605 | 15 | 42232 | 4.63 |
| min_norm_32x64_rank32_f64/numpy | 122.814 / 129.650 | 125.328 / 130.335 | unavailable | unavailable | 37.55 |
| min_norm_64x32_rank16_f64/kernix | 48.342 / 53.372 | 48.490 / 52.054 | 20 | 74752 | 4.93 |
| min_norm_64x32_rank16_f64/nalgebra | 80.777 / 132.699 | 83.507 / 93.353 | 15 | 41976 | 4.52 |
| min_norm_64x32_rank16_f64/numpy | 68.057 / 105.224 | 74.630 / 111.336 | unavailable | unavailable | 37.43 |
| parallel_matmul_into_128_f32/kernix | 41.307 / 48.962 | 24.329 / 33.792 | 2 | 8 | 5.27 |
| parallel_matmul_into_128_f64/kernix | 41.924 / 85.093 | 47.244 / 85.964 | 2 | 16 | 5.62 |
| parallel_matmul_into_256_f64/kernix | 318.499 / 653.522 | 327.298 / 375.811 | 2 | 16 | 7.30 |
| parallel_matmul_into_64_f64/kernix | 14.817 / 15.201 | 10.252 / 12.322 | 2 | 98304 | 4.91 |
| pivoted_qr_256x64_f64/faer | 456.588 / 482.098 | 460.693 / 487.292 | 10 | 333312 | 5.49 |
| pivoted_qr_256x64_f64/kernix | 594.535 / 809.778 | 597.580 / 659.895 | 14 | 559104 | 5.36 |
| pivoted_qr_256x64_f64/nalgebra | 846.397 / 909.889 | 889.440 / 953.986 | 7 | 296960 | 4.82 |
| pivoted_qr_96x32_f64/faer | 73.177 / 82.377 | 73.098 / 75.686 | 10 | 68352 | 5.11 |
| pivoted_qr_96x32_f64/kernix | 57.355 / 61.144 | 58.360 / 59.381 | 14 | 107520 | 4.65 |
| pivoted_qr_96x32_f64/nalgebra | 83.846 / 90.014 | 85.566 / 89.566 | 7 | 58368 | 3.89 |
| qr_1024x256_f64/faer | 14910.293 / 18372.092 | 14712.116 / 19070.696 | 7 | 5308416 | 12.15 |
| qr_1024x256_f64/kernix | 18244.794 / 24318.128 | 18513.283 / 21463.224 | 82 | 7145480 | 17.91 |
| qr_1024x256_f64/nalgebra | 31419.420 / 33626.676 | 30640.456 / 32167.151 | 4 | 4720640 | 10.57 |
| qr_1024x256_f64/numpy | 13106.153 / 15094.960 | 13246.999 / 14855.916 | unavailable | unavailable | 53.07 |
| qr_256x64_f64/faer | 424.417 / 501.775 | 433.993 / 477.080 | 7 | 331776 | 5.39 |
| qr_256x64_f64/kernix | 510.159 / 574.472 | 536.054 / 591.806 | 6 | 426496 | 4.85 |
| qr_256x64_f64/nalgebra | 393.179 / 414.387 | 385.244 / 394.321 | 4 | 295424 | 4.13 |
| qr_256x64_f64/numpy | 407.643 / 445.797 | 408.162 / 555.606 | unavailable | unavailable | 38.46 |
| qr_512x128_f64/faer | 2074.102 / 2228.504 | 2106.834 / 2241.844 | 7 | 1343488 | 6.91 |
| qr_512x128_f64/kernix | 2710.334 / 4282.129 | 2816.296 / 3541.212 | 49 | 2294792 | 8.21 |
| qr_512x128_f64/nalgebra | 3063.173 / 3286.480 | 3059.703 / 3334.573 | 4 | 1180672 | 5.72 |
| qr_512x128_f64/numpy | 2591.262 / 3312.594 | 2637.996 / 2917.631 | unavailable | unavailable | 41.67 |
| qr_96x32_f64/faer | 66.514 / 87.614 | 68.421 / 73.384 | 7 | 67584 | 4.55 |
| qr_96x32_f64/kernix | 51.792 / 55.847 | 53.783 / 92.883 | 6 | 82176 | 4.58 |
| qr_96x32_f64/nalgebra | 39.913 / 41.145 | 41.239 / 46.909 | 4 | 57600 | 3.83 |
| qr_96x32_f64/numpy | 42.941 / 46.534 | 44.407 / 53.865 | unavailable | unavailable | 37.66 |
| softmax_256x64_f64/candle | 87.264 / 97.822 | 87.952 / 90.980 | 37 | 265200 | 6.24 |
| softmax_256x64_f64/kernix | 93.389 / 99.312 | 94.707 / 96.821 | 1 | 131072 | 4.72 |
| softmax_256x64_f64/ndarray | 77.091 / 81.775 | 81.563 / 89.718 | 1 | 131072 | 4.58 |
| softmax_256x64_f64/numpy | 105.888 / 116.996 | 109.065 / 167.252 | unavailable | unavailable | 36.41 |
| solve_cholesky_128x1_f64/faer | 4.335 / 4.436 | 4.360 / 6.339 | 1 | 1024 | 5.26 |
| solve_cholesky_128x1_f64/kernix | 10.516 / 10.607 | 10.887 / 11.274 | 1 | 1024 | 4.65 |
| solve_cholesky_128x1_f64/nalgebra | 3.939 / 4.259 | 4.159 / 4.294 | 1 | 1024 | 4.46 |
| solve_cholesky_256x64_f64/faer | 227.128 / 278.947 | 231.328 / 244.955 | 1 | 131072 | 6.87 |
| solve_cholesky_256x64_f64/kernix | 1233.339 / 1304.494 | 1228.527 / 1306.899 | 1 | 131072 | 5.91 |
| solve_cholesky_256x64_f64/nalgebra | 855.367 / 888.157 | 844.613 / 868.065 | 1 | 131072 | 5.79 |
| solve_cholesky_512x128_f64/faer | 1652.097 / 2425.226 | 1708.460 / 2343.573 | 1 | 524288 | 11.03 |
| solve_cholesky_512x128_f64/kernix | 8879.196 / 9474.883 | 8797.893 / 9090.772 | 1 | 524288 | 10.09 |
| solve_cholesky_512x128_f64/nalgebra | 8215.864 / 10641.391 | 8700.683 / 11767.178 | 1 | 524288 | 10.37 |
| solve_lu_256x64_f64/faer | 268.121 / 343.718 | 273.733 / 326.419 | 2 | 262144 | 7.32 |
| solve_lu_256x64_f64/kernix | 1157.877 / 1260.919 | 1160.885 / 1270.519 | 2 | 262144 | 6.05 |
| solve_lu_256x64_f64/nalgebra | 882.351 / 916.451 | 874.908 / 894.732 | 1 | 131072 | 5.80 |
| sparse_auto_512_f64/kernix | 534.670 / 997.726 | 598.961 / 686.991 | 4 | 2142216 | 10.38 |
| sparse_auto_512_f64/kernix-dense | 116.698 / 520.665 | 105.431 / 158.803 | 1 | 2097152 | 16.21 |
| sparse_auto_512_f64/numpy | 81.466 / 263.057 | 100.354 / 161.765 | unavailable | unavailable | 63.02 |
| sparse_auto_512_f64/scipy | 941.507 / 1569.635 | 979.005 / 1204.264 | unavailable | unavailable | 63.01 |
| sparse_matmul_512x32_f32/faer | 38.575 / 39.815 | 39.021 / 40.873 | 1 | 65536 | 5.80 |
| sparse_matmul_512x32_f32/kernix | 16.813 / 17.306 | 17.193 / 18.019 | 1 | 65536 | 4.88 |
| sparse_matmul_512x32_f32/kernix-dense | 212.719 / 275.740 | 220.766 / 258.483 | 3 | 163840 | 6.89 |
| sparse_matmul_512x32_f32/numpy | 169.153 / 181.756 | 172.200 / 180.520 | unavailable | unavailable | 54.27 |
| sparse_matmul_512x32_f32/scipy | 15.043 / 15.374 | 15.282 / 16.910 | unavailable | unavailable | 53.41 |
| sparse_matmul_512x32_f64/faer | 42.111 / 43.555 | 44.325 / 51.833 | 1 | 131072 | 6.96 |
| sparse_matmul_512x32_f64/kernix | 24.034 / 25.457 | 25.223 / 26.044 | 1 | 131072 | 5.96 |
| sparse_matmul_512x32_f64/kernix-dense | 421.951 / 564.467 | 449.686 / 487.405 | 3 | 327680 | 9.13 |
| sparse_matmul_512x32_f64/numpy | 428.464 / 694.880 | 447.682 / 573.394 | unavailable | unavailable | 55.57 |
| sparse_matmul_512x32_f64/scipy | 22.263 / 39.159 | 22.808 / 24.659 | unavailable | unavailable | 54.67 |
| sparse_matvec_2048_f64/faer | 4.938 / 5.073 | 5.160 / 5.525 | 1 | 16384 | 13.23 |
| sparse_matvec_2048_f64/kernix | 11.708 / 11.967 | 12.623 / 21.420 | 1 | 16384 | 12.37 |
| sparse_matvec_2048_f64/kernix-dense | 3233.102 / 4617.031 | 3376.965 / 4546.640 | 3 | 155648 | 45.04 |
| sparse_matvec_2048_f64/numpy | 421.739 / 1421.228 | 480.654 / 921.683 | unavailable | unavailable | 60.64 |
| sparse_matvec_2048_f64/scipy | 7.040 / 8.430 | 7.515 / 7.848 | unavailable | unavailable | 60.73 |
| sparse_product_256_f64/kernix | 86.747 / 91.036 | 86.546 / 87.060 | 4 | 67752 | 4.90 |
| sparse_product_256_f64/kernix-dense | 573.091 / 654.802 | 572.331 / 613.594 | 3 | 524296 | 8.17 |
| sparse_product_256_f64/numpy | 635.812 / 876.020 | 659.379 / 824.135 | unavailable | unavailable | 56.29 |
| sparse_product_256_f64/scipy | 42.096 / 44.025 | 42.568 / 50.932 | unavailable | unavailable | 54.52 |
| tensordot_16x32x8_32x8x16_f64/kernix | 37.966 / 48.017 | 38.256 / 43.572 | 9 | 2166 | 4.18 |
| tensordot_16x32x8_32x8x16_f64/numpy | 8.467 / 9.298 | 8.770 / 9.675 | unavailable | unavailable | 36.39 |
| training_backward_2x4x8x4_f64/candle | 15.270 / 15.659 | 15.507 / 16.616 | 472 | 8332 | 6.15 |
| training_backward_2x4x8x4_f64/kernix | 3.259 / 3.439 | 3.434 / 3.782 | 23 | 4224 | 4.55 |
| training_forward_2x4x8x4_f64/candle | 3.880 / 4.062 | 3.820 / 4.536 | 126 | 3912 | 5.45 |
| training_forward_2x4x8x4_f64/kernix | 2.219 / 2.274 | 2.249 / 2.707 | 17 | 1032 | 4.86 |
| training_record_2x4x8x4_f64/candle | 5.374 / 6.984 | 5.659 / 9.806 | 149 | 10712 | 6.05 |
| training_record_2x4x8x4_f64/kernix | 2.934 / 3.017 | 2.941 / 3.130 | 22 | 6832 | 4.82 |
| training_sgd_2x4x8x4_f64/kernix | 0.553 / 0.582 | 0.584 / 1.037 | 5 | 688 | 5.18 |
| training_shared_backward_64x32_f64/candle | 4.211 / 4.697 | 4.526 / 4.744 | 58 | 100004 | 5.46 |
| training_shared_backward_64x32_f64/kernix | 6.659 / 6.906 | 6.635 / 6.699 | 8 | 49768 | 4.78 |

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
