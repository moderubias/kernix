# Clean CPU evidence: baseline-ubuntu-latest-40289723446

Measured source `7c11b2165b0cb7b523db4d938a36f1396396a138`, tree `cc7ac522fccee7e655adb7ca8d3644d95344fcb1`; dirty before/after: false.
Library source: git v0.12.0 90cbbff60e5d7458ef9f89918c99a6cca94b51d1. CPU affinity: 0. Dtype explicitly named per workload; row-major inputs.

| Workload / library | Per-call p50 / p95 (us) | Batch-mean p50 / p95 (us) | Rust allocations | Peak additional requested bytes | Process peak RSS (MiB) |
|---|---:|---:|---:|---:|---:|
| cholesky_factor_128_f64/faer | 57.689 / 67.096 | 58.555 / 59.009 | 2 | 132096 | 4.57 |
| cholesky_factor_128_f64/kernix | 170.329 / 180.319 | 172.095 / 173.419 | 1 | 131072 | 4.38 |
| cholesky_factor_128_f64/nalgebra | 120.926 / 130.625 | 121.648 / 122.935 | 1 | 131072 | 3.84 |
| cholesky_factor_128_f64/numpy | 105.979 / 118.863 | 107.366 / 108.274 | unavailable | unavailable | 46.35 |
| cholesky_factor_256_f64/faer | 309.661 / 329.699 | 313.360 / 317.221 | 2 | 526336 | 5.46 |
| cholesky_factor_256_f64/kernix | 680.406 / 694.472 | 681.411 / 689.801 | 49 | 645120 | 5.60 |
| cholesky_factor_256_f64/nalgebra | 819.506 / 834.775 | 818.568 / 844.304 | 1 | 524288 | 4.64 |
| cholesky_factor_256_f64/numpy | 1088.511 / 1119.609 | 1096.966 / 1104.739 | unavailable | unavailable | 49.95 |
| cholesky_factor_512_f64/faer | 2079.300 / 2119.265 | 2078.929 / 2096.348 | 2 | 2101248 | 8.92 |
| cholesky_factor_512_f64/kernix | 3834.198 / 3882.138 | 3836.536 / 3884.259 | 161 | 2269184 | 10.28 |
| cholesky_factor_512_f64/nalgebra | 7191.831 / 7333.557 | 7191.821 / 7234.171 | 1 | 2097152 | 7.20 |
| cholesky_factor_512_f64/numpy | 6265.265 / 6336.679 | 6262.765 / 6299.198 | unavailable | unavailable | 60.36 |
| cholesky_factor_64_f64/faer | 14.167 / 14.257 | 14.297 / 14.473 | 2 | 33280 | 4.04 |
| cholesky_factor_64_f64/kernix | 25.036 / 25.197 | 25.289 / 25.547 | 2 | 33280 | 4.03 |
| cholesky_factor_64_f64/nalgebra | 20.468 / 20.919 | 20.751 / 20.903 | 1 | 32768 | 3.68 |
| cholesky_factor_64_f64/numpy | 17.974 / 18.975 | 18.496 / 18.879 | unavailable | unavailable | 45.57 |
| cholesky_rank_downdate_256_f64/kernix | 169.928 / 180.148 | 171.562 / 175.966 | 3 | 526336 | 5.61 |
| cholesky_rank_downdate_256_f64/nalgebra | 39.334 / 43.993 | 34.239 / 38.566 | 2 | 526336 | 6.15 |
| cholesky_rank_downdate_256_f64/refactor | 682.172 / 695.076 | 680.530 / 687.855 | 49 | 645120 | 6.39 |
| cholesky_rank_update_256_f64/kernix | 171.351 / 182.763 | 172.320 / 173.903 | 3 | 526336 | 5.64 |
| cholesky_rank_update_256_f64/nalgebra | 37.370 / 44.243 | 33.990 / 37.785 | 2 | 526336 | 5.66 |
| cholesky_rank_update_256_f64/refactor | 683.452 / 693.761 | 680.936 / 689.031 | 49 | 645120 | 6.53 |
| cholesky_rank_update_512_f64/kernix | 892.053 / 917.891 | 869.021 / 969.459 | 3 | 2101248 | 10.13 |
| cholesky_rank_update_512_f64/nalgebra | 158.106 / 172.153 | 136.882 / 141.663 | 2 | 2101248 | 13.73 |
| cholesky_rank_update_512_f64/refactor | 3843.366 / 3943.263 | 3842.628 / 3872.080 | 161 | 2269184 | 12.15 |
| create_ones_64x64_f32/candle | 0.441 / 0.451 | 0.456 / 0.470 | 7 | 16696 | 4.54 |
| create_ones_64x64_f32/faer | 0.391 / 0.391 | 0.380 / 0.383 | 1 | 16384 | 3.96 |
| create_ones_64x64_f32/kernix | 0.431 / 0.431 | 0.439 / 0.445 | 1 | 16384 | 3.67 |
| create_ones_64x64_f32/ndarray | 0.351 / 0.351 | 0.357 / 0.360 | 1 | 16384 | 3.72 |
| create_ones_64x64_f32/numpy | 2.043 / 2.094 | 2.162 / 2.188 | unavailable | unavailable | 42.05 |
| create_zeros_64x64_f32/candle | 0.271 / 0.281 | 0.285 / 0.347 | 5 | 16648 | 4.16 |
| create_zeros_64x64_f32/faer | 0.321 / 0.331 | 0.282 / 0.288 | 1 | 16384 | 3.95 |
| create_zeros_64x64_f32/kernix | 0.281 / 0.291 | 0.292 / 0.295 | 1 | 16384 | 3.73 |
| create_zeros_64x64_f32/ndarray | 0.200 / 0.201 | 0.205 / 0.206 | 1 | 16384 | 3.67 |
| create_zeros_64x64_f32/numpy | 0.721 / 1.173 | 0.820 / 0.836 | unavailable | unavailable | 42.04 |
| matmul_128_f32/candle | 45.446 / 51.908 | 45.990 / 46.339 | 9 | 65816 | 5.12 |
| matmul_128_f32/faer | 52.118 / 60.634 | 52.528 / 53.483 | 1 | 65536 | 4.32 |
| matmul_128_f32/kernix | 45.475 / 47.830 | 45.966 / 46.485 | 3 | 65544 | 4.87 |
| matmul_128_f32/nalgebra | 75.812 / 91.321 | 78.697 / 79.196 | 2 | 163840 | 4.07 |
| matmul_128_f32/ndarray | 81.684 / 108.263 | 76.937 / 100.326 | 2 | 163840 | 4.12 |
| matmul_128_f32/numpy | 48.731 / 58.549 | 49.481 / 50.176 | unavailable | unavailable | 44.38 |
| matmul_128_f64/candle | 92.593 / 102.602 | 93.770 / 94.657 | 9 | 131352 | 5.34 |
| matmul_128_f64/faer | 113.984 / 125.124 | 114.410 / 115.214 | 1 | 131072 | 4.49 |
| matmul_128_f64/kernix | 92.794 / 103.243 | 93.943 / 94.295 | 3 | 131080 | 4.82 |
| matmul_128_f64/nalgebra | 210.555 / 236.623 | 212.231 / 213.199 | 2 | 327680 | 4.16 |
| matmul_128_f64/ndarray | 210.775 / 226.915 | 210.605 / 212.329 | 2 | 327680 | 4.21 |
| matmul_128_f64/numpy | 100.008 / 113.593 | 101.779 / 102.306 | unavailable | unavailable | 44.69 |
| matmul_256_f32/candle | 352.832 / 364.314 | 356.687 / 358.529 | 9 | 262424 | 5.83 |
| matmul_256_f32/faer | 409.107 / 425.669 | 412.976 / 416.272 | 1 | 262144 | 4.93 |
| matmul_256_f32/kernix | 352.441 / 363.482 | 356.213 / 384.084 | 3 | 262152 | 5.36 |
| matmul_256_f32/nalgebra | 609.011 / 628.468 | 607.650 / 616.740 | 2 | 589824 | 4.60 |
| matmul_256_f32/ndarray | 606.327 / 625.964 | 605.125 / 668.573 | 2 | 589824 | 4.75 |
| matmul_256_f32/numpy | 362.019 / 375.895 | 365.778 / 386.154 | unavailable | unavailable | 48.19 |
| matmul_256_f64/candle | 731.822 / 746.088 | 736.053 / 752.408 | 9 | 524568 | 6.84 |
| matmul_256_f64/faer | 975.098 / 995.406 | 975.136 / 984.832 | 1 | 524288 | 5.31 |
| matmul_256_f64/kernix | 734.798 / 752.010 | 734.254 / 743.598 | 3 | 524296 | 5.91 |
| matmul_256_f64/nalgebra | 898.725 / 914.305 | 899.327 / 903.238 | 2 | 1179648 | 5.45 |
| matmul_256_f64/ndarray | 906.509 / 918.672 | 906.970 / 914.948 | 2 | 1179648 | 5.93 |
| matmul_256_f64/numpy | 766.897 / 791.263 | 766.201 / 777.292 | unavailable | unavailable | 48.91 |
| matmul_32_f64/candle | 2.024 / 2.054 | 2.050 / 2.064 | 9 | 8472 | 4.80 |
| matmul_32_f64/faer | 2.024 / 2.064 | 2.031 / 2.039 | 1 | 8192 | 4.21 |
| matmul_32_f64/kernix | 2.395 / 2.445 | 2.430 / 2.445 | 3 | 24576 | 4.19 |
| matmul_32_f64/nalgebra | 2.134 / 2.214 | 2.204 / 2.529 | 2 | 24576 | 3.89 |
| matmul_32_f64/ndarray | 2.164 / 2.194 | 2.185 / 2.197 | 2 | 24576 | 3.62 |
| matmul_32_f64/numpy | 3.656 / 3.747 | 3.791 / 3.826 | unavailable | unavailable | 43.33 |
| matmul_64_f32/candle | 6.532 / 6.622 | 6.547 / 6.635 | 9 | 16664 | 4.86 |
| matmul_64_f32/faer | 6.603 / 6.683 | 6.674 / 6.760 | 1 | 16384 | 4.13 |
| matmul_64_f32/kernix | 6.272 / 6.342 | 6.384 / 6.481 | 3 | 16448 | 4.56 |
| matmul_64_f32/nalgebra | 7.474 / 7.555 | 7.564 / 7.628 | 2 | 49152 | 3.41 |
| matmul_64_f32/ndarray | 7.384 / 8.136 | 7.507 / 7.536 | 2 | 49152 | 3.94 |
| matmul_64_f32/numpy | 8.205 / 8.286 | 8.429 / 8.498 | unavailable | unavailable | 43.50 |
| matmul_64_f64/candle | 12.583 / 12.674 | 12.764 / 13.032 | 9 | 33048 | 4.88 |
| matmul_64_f64/faer | 13.916 / 14.096 | 13.843 / 14.264 | 1 | 32768 | 4.34 |
| matmul_64_f64/kernix | 12.413 / 12.543 | 12.544 / 12.562 | 3 | 32832 | 4.64 |
| matmul_64_f64/nalgebra | 15.169 / 15.289 | 15.372 / 15.408 | 2 | 98304 | 3.88 |
| matmul_64_f64/ndarray | 15.178 / 15.278 | 15.315 / 15.369 | 2 | 98304 | 3.96 |
| matmul_64_f64/numpy | 15.058 / 16.180 | 15.355 / 15.434 | unavailable | unavailable | 43.62 |
| matmul_ab_128_f32/kernix | 45.625 / 52.258 | 46.185 / 46.557 | 3 | 65544 | 4.88 |
| matmul_ab_128_f32/numpy | 49.233 / 58.921 | 49.919 / 50.498 | unavailable | unavailable | 44.45 |
| matmul_ab_128_f64/kernix | 93.004 / 103.244 | 94.115 / 95.777 | 3 | 131080 | 4.82 |
| matmul_ab_128_f64/numpy | 100.809 / 112.041 | 102.498 / 103.661 | unavailable | unavailable | 44.70 |
| matmul_batched_4x16x32x16_f32/candle | 2.014 / 2.084 | 2.040 / 2.060 | 25 | 13096 | 5.29 |
| matmul_batched_4x16x32x16_f32/kernix | 1.182 / 1.203 | 1.206 / 1.222 | 3 | 4104 | 4.00 |
| matmul_batched_4x16x32x16_f32/numpy | 3.266 / 3.346 | 3.406 / 3.451 | unavailable | unavailable | 43.29 |
| matmul_batched_4x16x32x16_f64/candle | 2.985 / 3.046 | 3.010 / 3.030 | 25 | 25384 | 5.11 |
| matmul_batched_4x16x32x16_f64/kernix | 2.054 / 2.074 | 2.082 / 2.104 | 3 | 8200 | 4.06 |
| matmul_batched_4x16x32x16_f64/numpy | 4.178 / 4.248 | 4.361 / 4.417 | unavailable | unavailable | 43.34 |
| matmul_sum_backward_32x16x8_f64/candle | 5.551 / 5.691 | 5.628 / 5.685 | 113 | 27988 | 5.47 |
| matmul_sum_backward_32x16x8_f64/kernix | 3.647 / 3.697 | 3.660 / 3.691 | 15 | 18376 | 3.86 |
| matmul_transposed_128_f32/kernix | 74.139 / 87.534 | 77.472 / 78.198 | 3 | 163840 | 4.67 |
| matmul_transposed_128_f32/numpy | 50.044 / 60.764 | 50.819 / 51.381 | unavailable | unavailable | 44.39 |
| matmul_transposed_128_f64/kernix | 214.111 / 228.488 | 214.149 / 215.938 | 3 | 327680 | 4.84 |
| matmul_transposed_128_f64/numpy | 100.137 / 111.540 | 101.638 / 114.327 | unavailable | unavailable | 44.73 |
| min_norm_32x64_rank32_f64/kernix | 84.068 / 93.025 | 85.081 / 86.437 | 20 | 59136 | 4.46 |
| min_norm_32x64_rank32_f64/nalgebra | 176.291 / 185.408 | 178.000 / 180.380 | 15 | 42232 | 4.24 |
| min_norm_32x64_rank32_f64/numpy | 184.937 / 205.014 | 189.234 / 194.438 | unavailable | unavailable | 47.59 |
| min_norm_64x32_rank16_f64/kernix | 62.938 / 71.394 | 63.490 / 64.251 | 20 | 74752 | 4.41 |
| min_norm_64x32_rank16_f64/nalgebra | 103.043 / 120.065 | 106.154 / 131.634 | 15 | 41976 | 4.18 |
| min_norm_64x32_rank16_f64/numpy | 100.408 / 120.105 | 103.148 / 105.481 | unavailable | unavailable | 46.53 |
| parallel_matmul_into_128_f32/kernix | 30.978 / 33.824 | 31.425 / 31.762 | 2 | 16 | 4.80 |
| parallel_matmul_into_128_f64/kernix | 56.245 / 66.174 | 57.082 / 73.308 | 2 | 16 | 5.16 |
| parallel_matmul_into_256_f64/kernix | 391.394 / 412.263 | 394.684 / 398.178 | 2 | 16 | 6.85 |
| parallel_matmul_into_64_f64/kernix | 15.720 / 15.990 | 15.810 / 25.275 | 2 | 98304 | 4.33 |
| pivoted_qr_256x64_f64/faer | 381.395 / 401.312 | 383.098 / 385.447 | 10 | 333312 | 4.82 |
| pivoted_qr_256x64_f64/kernix | 792.246 / 809.038 | 796.735 / 811.218 | 14 | 559104 | 4.82 |
| pivoted_qr_256x64_f64/nalgebra | 739.035 / 758.932 | 739.785 / 747.708 | 7 | 296960 | 3.89 |
| pivoted_qr_96x32_f64/faer | 42.830 / 45.596 | 43.274 / 43.568 | 10 | 68352 | 4.62 |
| pivoted_qr_96x32_f64/kernix | 79.068 / 87.976 | 80.435 / 80.848 | 14 | 107520 | 4.32 |
| pivoted_qr_96x32_f64/nalgebra | 76.613 / 85.280 | 77.578 / 77.979 | 7 | 58368 | 3.89 |
| qr_1024x256_f64/faer | 13530.048 / 14004.688 | 13698.023 / 14037.319 | 7 | 5308416 | 11.76 |
| qr_1024x256_f64/kernix | 20667.047 / 20958.644 | 20703.736 / 20837.837 | 80 | 7018502 | 17.16 |
| qr_1024x256_f64/nalgebra | 33082.111 / 33502.569 | 33280.875 / 33709.199 | 4 | 4720640 | 10.24 |
| qr_1024x256_f64/numpy | 18111.968 / 18317.964 | 18245.478 / 18372.466 | unavailable | unavailable | 61.14 |
| qr_256x64_f64/faer | 418.504 / 441.107 | 421.348 / 428.313 | 7 | 331776 | 4.94 |
| qr_256x64_f64/kernix | 750.187 / 774.142 | 750.003 / 754.612 | 6 | 426496 | 4.52 |
| qr_256x64_f64/nalgebra | 556.803 / 571.251 | 558.023 / 562.936 | 4 | 295424 | 4.14 |
| qr_256x64_f64/numpy | 542.467 / 561.212 | 536.731 / 541.256 | unavailable | unavailable | 48.44 |
| qr_512x128_f64/faer | 2251.642 / 2280.636 | 2255.883 / 2273.787 | 7 | 1343488 | 6.00 |
| qr_512x128_f64/kernix | 3731.610 / 3821.850 | 3730.792 / 3789.746 | 48 | 2139142 | 7.84 |
| qr_512x128_f64/nalgebra | 4475.523 / 4532.459 | 4453.061 / 4494.759 | 4 | 1180672 | 5.35 |
| qr_512x128_f64/numpy | 3420.342 / 3485.904 | 3423.868 / 3486.419 | unavailable | unavailable | 51.83 |
| qr_96x32_f64/faer | 55.154 / 64.941 | 55.836 / 56.570 | 7 | 67584 | 4.70 |
| qr_96x32_f64/kernix | 66.815 / 75.271 | 68.288 / 69.125 | 6 | 82176 | 4.18 |
| qr_96x32_f64/nalgebra | 57.639 / 66.003 | 58.439 / 58.706 | 4 | 57600 | 3.82 |
| qr_96x32_f64/numpy | 68.588 / 89.598 | 70.472 / 71.341 | unavailable | unavailable | 47.61 |
| softmax_256x64_f64/candle | 84.518 / 94.627 | 85.571 / 86.035 | 37 | 265200 | 5.81 |
| softmax_256x64_f64/kernix | 94.077 / 103.374 | 95.105 / 95.868 | 1 | 131072 | 4.38 |
| softmax_256x64_f64/ndarray | 80.982 / 89.899 | 81.872 / 82.249 | 1 | 131072 | 4.20 |
| softmax_256x64_f64/numpy | 119.875 / 132.358 | 122.046 / 123.619 | unavailable | unavailable | 42.45 |
| solve_cholesky_128x1_f64/faer | 5.851 / 5.941 | 5.921 / 5.969 | 1 | 1024 | 4.86 |
| solve_cholesky_128x1_f64/kernix | 18.886 / 18.976 | 19.060 / 19.200 | 1 | 1024 | 4.32 |
| solve_cholesky_128x1_f64/nalgebra | 5.971 / 6.081 | 6.054 / 6.089 | 1 | 1024 | 4.15 |
| solve_cholesky_256x64_f64/faer | 275.186 / 291.195 | 278.379 / 280.355 | 1 | 131072 | 6.32 |
| solve_cholesky_256x64_f64/kernix | 1219.647 / 1244.432 | 1221.702 / 1262.230 | 1 | 131072 | 5.77 |
| solve_cholesky_256x64_f64/nalgebra | 1205.710 / 1255.604 | 1214.419 / 1275.384 | 1 | 131072 | 5.39 |
| solve_cholesky_512x128_f64/faer | 2205.695 / 2242.764 | 2212.624 / 2229.493 | 1 | 524288 | 10.14 |
| solve_cholesky_512x128_f64/kernix | 9608.824 / 9718.620 | 9619.230 / 9808.069 | 1 | 524288 | 9.61 |
| solve_cholesky_512x128_f64/nalgebra | 9558.176 / 9773.699 | 9605.113 / 9706.443 | 1 | 524288 | 9.87 |
| solve_lu_256x64_f64/faer | 348.844 / 376.776 | 352.220 / 365.149 | 2 | 262144 | 6.92 |
| solve_lu_256x64_f64/kernix | 1182.177 / 1206.462 | 1184.228 / 1191.387 | 2 | 262144 | 5.59 |
| solve_lu_256x64_f64/nalgebra | 1132.363 / 1172.538 | 1121.592 / 1134.880 | 1 | 131072 | 5.42 |
| sparse_auto_512_f64/kernix-dense | 53.971 / 64.000 | 55.674 / 57.159 | 1 | 2097152 | 15.86 |
| sparse_auto_512_f64/numpy | 54.662 / 65.774 | 55.283 / 56.097 | unavailable | unavailable | 69.13 |
| sparse_auto_512_f64/scipy | 1237.069 / 1497.928 | 1294.920 / 1415.600 | unavailable | unavailable | 72.29 |
| sparse_matmul_512x32_f32/faer | 61.165 / 69.650 | 60.525 / 61.458 | 1 | 65536 | 5.34 |
| sparse_matmul_512x32_f32/kernix-dense | 253.525 / 265.468 | 256.203 / 260.641 | 3 | 163840 | 6.57 |
| sparse_matmul_512x32_f32/numpy | 207.649 / 225.974 | 210.424 / 214.320 | unavailable | unavailable | 61.45 |
| sparse_matmul_512x32_f32/scipy | 18.986 / 19.848 | 19.295 / 19.507 | unavailable | unavailable | 60.25 |
| sparse_matmul_512x32_f64/faer | 67.146 / 76.013 | 67.815 / 68.346 | 1 | 131072 | 6.54 |
| sparse_matmul_512x32_f64/kernix-dense | 603.211 / 626.244 | 606.347 / 611.108 | 3 | 327680 | 8.71 |
| sparse_matmul_512x32_f64/numpy | 474.049 / 484.658 | 472.766 / 481.832 | unavailable | unavailable | 62.57 |
| sparse_matmul_512x32_f64/scipy | 26.920 / 28.183 | 27.714 / 28.016 | unavailable | unavailable | 60.82 |
| sparse_matvec_2048_f64/faer | 7.063 / 7.144 | 7.148 / 7.200 | 1 | 16384 | 12.64 |
| sparse_matvec_2048_f64/kernix-dense | 2838.031 / 3111.082 | 2672.408 / 2829.660 | 3 | 155648 | 44.67 |
| sparse_matvec_2048_f64/numpy | 557.595 / 634.990 | 548.116 / 578.763 | unavailable | unavailable | 89.29 |
| sparse_matvec_2048_f64/scipy | 10.370 / 11.101 | 10.618 / 10.745 | unavailable | unavailable | 89.53 |
| sparse_product_256_f64/kernix-dense | 731.922 / 776.075 | 734.224 / 773.541 | 3 | 524296 | 7.45 |
| sparse_product_256_f64/numpy | 768.180 / 790.522 | 770.320 / 777.341 | unavailable | unavailable | 63.33 |
| sparse_product_256_f64/scipy | 65.824 / 84.879 | 66.578 / 67.979 | unavailable | unavailable | 61.63 |
| tensordot_16x32x8_32x8x16_f64/kernix | 63.388 / 71.744 | 64.126 / 64.754 | 9 | 2166 | 4.06 |
| tensordot_16x32x8_32x8x16_f64/numpy | 10.860 / 11.742 | 11.115 / 11.206 | unavailable | unavailable | 43.37 |
| training_backward_2x4x8x4_f64/candle | 16.831 / 17.502 | 17.209 / 17.279 | 472 | 8332 | 5.89 |
| training_backward_2x4x8x4_f64/kernix | 4.769 / 4.859 | 4.841 / 4.930 | 23 | 4224 | 4.39 |
| training_forward_2x4x8x4_f64/candle | 4.348 / 4.498 | 4.403 / 4.443 | 126 | 3912 | 6.00 |
| training_forward_2x4x8x4_f64/kernix | 3.026 / 5.160 | 3.078 / 3.127 | 17 | 1032 | 4.29 |
| training_record_2x4x8x4_f64/candle | 6.282 / 6.533 | 6.308 / 6.419 | 149 | 10712 | 5.94 |
| training_record_2x4x8x4_f64/kernix | 3.877 / 3.968 | 3.949 / 3.992 | 22 | 6832 | 4.17 |
| training_sgd_2x4x8x4_f64/kernix | 0.852 / 0.872 | 0.876 / 0.884 | 5 | 688 | 4.52 |
| training_shared_backward_64x32_f64/candle | 4.568 / 4.668 | 4.636 / 4.644 | 58 | 100004 | 5.08 |
| training_shared_backward_64x32_f64/kernix | 10.310 / 10.370 | 10.442 / 11.494 | 8 | 49768 | 3.88 |

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
