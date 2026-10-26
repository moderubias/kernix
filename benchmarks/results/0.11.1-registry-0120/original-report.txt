# Clean CPU evidence: 0.11.1-registry-0120

Measured source `90cbbff60e5d7458ef9f89918c99a6cca94b51d1`, tree `d806b9704a44b4bf2c4c04e664fe3c60eeebc177`; dirty before/after: false.
Library source: registry 0.11.1. CPU affinity: 0. Dtype explicitly named per workload; row-major inputs.

| Workload / library | Per-call p50 / p95 (us) | Batch-mean p50 / p95 (us) | Rust allocations | Peak additional requested bytes | Process peak RSS (MiB) |
|---|---:|---:|---:|---:|---:|
| cholesky_factor_128_f64/faer | 41.318 / 44.586 | 41.760 / 45.785 | 2 | 132096 | 4.97 |
| cholesky_factor_128_f64/kernix | 110.599 / 124.999 | 116.569 / 149.820 | 1 | 131072 | 4.82 |
| cholesky_factor_128_f64/nalgebra | 101.281 / 134.959 | 102.128 / 122.348 | 1 | 131072 | 4.40 |
| cholesky_factor_128_f64/numpy | 76.363 / 82.868 | 78.115 / 80.945 | unavailable | unavailable | 37.51 |
| cholesky_factor_256_f64/faer | 197.118 / 259.091 | 196.796 / 201.724 | 2 | 526336 | 5.68 |
| cholesky_factor_256_f64/kernix | 512.362 / 606.692 | 528.629 / 589.246 | 49 | 645120 | 6.29 |
| cholesky_factor_256_f64/nalgebra | 673.671 / 711.425 | 681.140 / 726.734 | 1 | 524288 | 5.02 |
| cholesky_factor_256_f64/numpy | 529.307 / 616.378 | 533.109 / 616.024 | unavailable | unavailable | 41.62 |
| cholesky_factor_512_f64/faer | 1547.853 / 2049.574 | 1545.052 / 1635.819 | 2 | 2101248 | 8.96 |
| cholesky_factor_512_f64/kernix | 2803.451 / 3134.559 | 2943.089 / 3255.693 | 161 | 2269184 | 10.86 |
| cholesky_factor_512_f64/nalgebra | 5497.489 / 6076.369 | 5530.541 / 5923.822 | 1 | 2097152 | 8.07 |
| cholesky_factor_512_f64/numpy | 3635.237 / 6396.837 | 3967.610 / 5129.617 | unavailable | unavailable | 53.40 |
| cholesky_factor_64_f64/faer | 7.904 / 8.528 | 8.179 / 8.717 | 2 | 33280 | 4.35 |
| cholesky_factor_64_f64/kernix | 20.910 / 23.081 | 20.845 / 21.180 | 2 | 33280 | 4.44 |
| cholesky_factor_64_f64/nalgebra | 13.198 / 14.426 | 13.915 / 14.343 | 1 | 32768 | 4.25 |
| cholesky_factor_64_f64/numpy | 10.804 / 14.196 | 10.864 / 11.288 | unavailable | unavailable | 36.84 |
| cholesky_rank_downdate_256_f64/nalgebra | 24.594 / 25.974 | 25.595 / 27.345 | 2 | 526336 | 6.76 |
| cholesky_rank_downdate_256_f64/refactor | 529.394 / 843.174 | 565.549 / 863.045 | 49 | 645120 | 6.84 |
| cholesky_rank_update_256_f64/nalgebra | 24.476 / 25.795 | 25.448 / 26.534 | 2 | 526336 | 6.84 |
| cholesky_rank_update_256_f64/refactor | 532.143 / 743.160 | 533.093 / 590.945 | 49 | 645120 | 6.88 |
| cholesky_rank_update_512_f64/nalgebra | 145.180 / 166.775 | 146.975 / 174.625 | 2 | 2101248 | 14.17 |
| cholesky_rank_update_512_f64/refactor | 2943.494 / 3567.089 | 2981.025 / 3394.502 | 161 | 2269184 | 12.89 |
| create_ones_64x64_f32/candle | 0.237 / 0.251 | 0.228 / 0.245 | 7 | 16696 | 4.52 |
| create_ones_64x64_f32/faer | 0.183 / 0.185 | 0.172 / 0.179 | 1 | 16384 | 4.63 |
| create_ones_64x64_f32/kernix | 0.183 / 0.185 | 0.189 / 0.192 | 1 | 16384 | 4.13 |
| create_ones_64x64_f32/ndarray | 0.153 / 0.216 | 0.144 / 0.160 | 1 | 16384 | 4.35 |
| create_ones_64x64_f32/numpy | 1.339 / 1.388 | 1.330 / 1.422 | unavailable | unavailable | 35.77 |
| create_zeros_64x64_f32/candle | 0.152 / 0.153 | 0.156 / 0.165 | 5 | 16648 | 4.44 |
| create_zeros_64x64_f32/faer | 0.155 / 0.157 | 0.191 / 0.199 | 1 | 16384 | 4.39 |
| create_zeros_64x64_f32/kernix | 0.145 / 0.146 | 0.151 / 0.164 | 1 | 16384 | 4.45 |
| create_zeros_64x64_f32/ndarray | 0.090 / 0.122 | 0.094 / 0.097 | 1 | 16384 | 4.32 |
| create_zeros_64x64_f32/numpy | 0.381 / 0.406 | 0.400 / 0.450 | unavailable | unavailable | 35.80 |
| matmul_128_f32/candle | 37.227 / 39.247 | 38.232 / 38.542 | 9 | 65816 | 4.98 |
| matmul_128_f32/faer | 40.078 / 41.113 | 40.811 / 41.536 | 1 | 65536 | 4.72 |
| matmul_128_f32/kernix | 34.995 / 36.769 | 37.068 / 49.644 | 3 | 65544 | 4.83 |
| matmul_128_f32/nalgebra | 49.852 / 59.872 | 52.072 / 60.422 | 2 | 163840 | 4.50 |
| matmul_128_f32/ndarray | 49.900 / 57.650 | 51.748 / 60.913 | 2 | 163840 | 4.59 |
| matmul_128_f32/numpy | 41.899 / 52.893 | 42.562 / 46.753 | unavailable | unavailable | 37.38 |
| matmul_128_f64/candle | 71.994 / 76.455 | 73.388 / 76.997 | 9 | 131352 | 5.35 |
| matmul_128_f64/faer | 81.445 / 86.514 | 80.726 / 81.954 | 1 | 131072 | 4.81 |
| matmul_128_f64/kernix | 78.613 / 90.293 | 79.616 / 81.216 | 3 | 131080 | 5.13 |
| matmul_128_f64/nalgebra | 110.778 / 127.902 | 118.484 / 167.469 | 2 | 327680 | 4.58 |
| matmul_128_f64/ndarray | 111.467 / 125.257 | 113.995 / 116.679 | 2 | 327680 | 4.79 |
| matmul_128_f64/numpy | 74.262 / 83.040 | 77.819 / 87.453 | unavailable | unavailable | 37.68 |
| matmul_256_f32/candle | 273.336 / 287.442 | 280.118 / 288.447 | 9 | 262424 | 5.82 |
| matmul_256_f32/faer | 311.138 / 322.459 | 318.242 / 353.434 | 1 | 262144 | 5.16 |
| matmul_256_f32/kernix | 289.080 / 319.301 | 291.961 / 300.180 | 3 | 262152 | 5.61 |
| matmul_256_f32/nalgebra | 411.441 / 493.767 | 426.370 / 472.949 | 2 | 589824 | 5.02 |
| matmul_256_f32/ndarray | 414.792 / 494.845 | 423.902 / 551.563 | 2 | 589824 | 5.20 |
| matmul_256_f32/numpy | 272.518 / 290.582 | 278.247 / 282.594 | unavailable | unavailable | 40.65 |
| matmul_256_f64/candle | 562.410 / 582.461 | 567.042 / 774.447 | 9 | 524568 | 6.99 |
| matmul_256_f64/faer | 743.656 / 778.908 | 752.645 / 809.060 | 1 | 524288 | 5.96 |
| matmul_256_f64/kernix | 558.192 / 600.640 | 559.234 / 573.524 | 3 | 524296 | 6.34 |
| matmul_256_f64/nalgebra | 658.241 / 687.987 | 650.915 / 843.551 | 2 | 1179648 | 5.88 |
| matmul_256_f64/ndarray | 639.797 / 681.235 | 639.400 / 653.208 | 2 | 1179648 | 6.46 |
| matmul_256_f64/numpy | 613.441 / 671.040 | 620.229 / 642.270 | unavailable | unavailable | 41.61 |
| matmul_32_f64/candle | 1.436 / 1.525 | 1.496 / 1.569 | 9 | 8592 | 4.57 |
| matmul_32_f64/faer | 1.554 / 1.639 | 1.589 / 1.700 | 1 | 8192 | 4.44 |
| matmul_32_f64/kernix | 1.633 / 1.650 | 1.685 / 1.805 | 3 | 24576 | 4.58 |
| matmul_32_f64/nalgebra | 1.526 / 1.553 | 1.593 / 1.749 | 2 | 24576 | 4.35 |
| matmul_32_f64/ndarray | 1.389 / 1.413 | 1.409 / 1.446 | 2 | 24576 | 4.27 |
| matmul_32_f64/numpy | 2.675 / 4.470 | 2.703 / 2.810 | unavailable | unavailable | 36.49 |
| matmul_64_f32/candle | 4.689 / 4.805 | 5.050 / 5.210 | 9 | 16664 | 5.10 |
| matmul_64_f32/faer | 5.463 / 6.971 | 5.128 / 5.232 | 1 | 16384 | 4.62 |
| matmul_64_f32/kernix | 5.156 / 5.278 | 5.384 / 6.425 | 3 | 16544 | 4.51 |
| matmul_64_f32/nalgebra | 6.088 / 6.158 | 6.164 / 6.457 | 2 | 49152 | 4.28 |
| matmul_64_f32/ndarray | 5.713 / 6.295 | 5.823 / 5.876 | 2 | 49152 | 4.28 |
| matmul_64_f32/numpy | 6.093 / 8.384 | 6.042 / 6.238 | unavailable | unavailable | 36.64 |
| matmul_64_f64/candle | 10.148 / 12.192 | 10.516 / 16.977 | 9 | 33048 | 5.07 |
| matmul_64_f64/faer | 11.082 / 14.787 | 11.343 / 12.461 | 1 | 32768 | 4.52 |
| matmul_64_f64/kernix | 9.879 / 11.192 | 10.431 / 10.761 | 3 | 32928 | 4.78 |
| matmul_64_f64/nalgebra | 11.244 / 11.384 | 11.206 / 11.568 | 2 | 98304 | 4.37 |
| matmul_64_f64/ndarray | 11.366 / 12.647 | 11.441 / 13.876 | 2 | 98304 | 4.48 |
| matmul_64_f64/numpy | 13.465 / 13.589 | 13.702 / 14.152 | unavailable | unavailable | 36.80 |
| matmul_ab_128_f32/kernix | 37.823 / 46.378 | 38.174 / 44.300 | 3 | 65544 | 5.04 |
| matmul_ab_128_f32/numpy | 36.309 / 39.377 | 37.708 / 38.719 | unavailable | unavailable | 37.27 |
| matmul_ab_128_f64/kernix | 74.171 / 78.398 | 73.065 / 78.973 | 3 | 131080 | 5.25 |
| matmul_ab_128_f64/numpy | 72.806 / 83.548 | 76.709 / 80.096 | unavailable | unavailable | 37.72 |
| matmul_batched_4x16x32x16_f32/candle | 1.456 / 1.511 | 1.490 / 1.570 | 25 | 13216 | 5.30 |
| matmul_batched_4x16x32x16_f32/kernix | 1.046 / 2.798 | 0.992 / 1.269 | 3 | 4104 | 4.45 |
| matmul_batched_4x16x32x16_f32/numpy | 2.529 / 2.576 | 2.555 / 2.660 | unavailable | unavailable | 36.33 |
| matmul_batched_4x16x32x16_f64/candle | 2.122 / 2.243 | 2.264 / 2.419 | 25 | 25504 | 5.25 |
| matmul_batched_4x16x32x16_f64/kernix | 1.493 / 1.801 | 1.623 / 1.954 | 3 | 8200 | 4.68 |
| matmul_batched_4x16x32x16_f64/numpy | 2.886 / 2.923 | 2.999 / 3.097 | unavailable | unavailable | 36.51 |
| matmul_sum_backward_32x16x8_f64/candle | 4.022 / 4.719 | 4.178 / 4.333 | 113 | 27988 | 5.58 |
| matmul_sum_backward_32x16x8_f64/kernix | 2.394 / 2.496 | 2.476 / 2.567 | 15 | 18376 | 4.61 |
| matmul_transposed_128_f32/kernix | 48.376 / 53.761 | 49.905 / 52.161 | 3 | 163840 | 4.86 |
| matmul_transposed_128_f32/numpy | 36.199 / 39.901 | 37.921 / 39.518 | unavailable | unavailable | 37.37 |
| matmul_transposed_128_f64/kernix | 125.516 / 160.723 | 126.482 / 141.436 | 3 | 327680 | 5.11 |
| matmul_transposed_128_f64/numpy | 73.017 / 79.891 | 75.741 / 79.382 | unavailable | unavailable | 37.68 |
| min_norm_32x64_rank32_f64/kernix | 54.989 / 72.414 | 55.701 / 57.193 | 20 | 59136 | 4.91 |
| min_norm_32x64_rank32_f64/nalgebra | 129.273 / 136.564 | 131.895 / 242.802 | 15 | 42232 | 4.52 |
| min_norm_32x64_rank32_f64/numpy | 124.641 / 136.846 | 120.583 / 134.232 | unavailable | unavailable | 37.37 |
| min_norm_64x32_rank16_f64/kernix | 42.882 / 45.510 | 44.456 / 47.314 | 20 | 74752 | 4.86 |
| min_norm_64x32_rank16_f64/nalgebra | 70.814 / 75.441 | 72.033 / 73.345 | 15 | 41976 | 4.57 |
| min_norm_64x32_rank16_f64/numpy | 69.822 / 79.099 | 67.393 / 71.430 | unavailable | unavailable | 37.45 |
| parallel_matmul_into_128_f32/kernix | 23.390 / 26.293 | 24.265 / 27.546 | 2 | 16 | 5.42 |
| parallel_matmul_into_128_f64/kernix | 41.455 / 60.768 | 46.702 / 80.928 | 2 | 8 | 5.72 |
| parallel_matmul_into_256_f64/kernix | 303.263 / 408.568 | 309.473 / 336.863 | 2 | 16 | 7.04 |
| parallel_matmul_into_64_f64/kernix | 10.988 / 13.470 | 10.471 / 21.808 | 2 | 98304 | 4.59 |
| pivoted_qr_256x64_f64/faer | 457.110 / 521.636 | 456.639 / 501.498 | 10 | 333312 | 5.01 |
| pivoted_qr_256x64_f64/kernix | 525.001 / 558.477 | 522.828 / 550.032 | 14 | 559104 | 5.28 |
| pivoted_qr_256x64_f64/nalgebra | 808.431 / 885.860 | 811.398 / 835.210 | 7 | 296960 | 4.88 |
| pivoted_qr_96x32_f64/faer | 75.116 / 78.795 | 76.220 / 78.235 | 10 | 68352 | 5.50 |
| pivoted_qr_96x32_f64/kernix | 53.779 / 68.478 | 55.276 / 61.920 | 14 | 107520 | 4.73 |
| pivoted_qr_96x32_f64/nalgebra | 81.169 / 89.936 | 80.500 / 84.457 | 7 | 58368 | 4.62 |
| qr_1024x256_f64/faer | 13279.983 / 16866.798 | 12652.174 / 13512.462 | 7 | 5308416 | 12.23 |
| qr_1024x256_f64/kernix | 29615.796 / 33268.546 | 31991.817 / 33974.499 | 6 | 6817792 | 16.29 |
| qr_1024x256_f64/nalgebra | 29215.404 / 31448.377 | 29269.423 / 31624.928 | 4 | 4720640 | 10.78 |
| qr_1024x256_f64/numpy | 18024.881 / 25350.572 | 22234.300 / 34570.059 | unavailable | unavailable | 52.93 |
| qr_256x64_f64/faer | 427.141 / 502.793 | 477.369 / 733.401 | 7 | 331776 | 5.29 |
| qr_256x64_f64/kernix | 477.593 / 523.540 | 480.248 / 529.857 | 6 | 426496 | 4.95 |
| qr_256x64_f64/nalgebra | 372.020 / 492.136 | 372.672 / 418.386 | 4 | 295424 | 4.56 |
| qr_256x64_f64/numpy | 365.029 / 513.165 | 368.406 / 431.248 | unavailable | unavailable | 38.38 |
| qr_512x128_f64/faer | 1998.505 / 2172.526 | 2009.590 / 2076.808 | 7 | 1343488 | 6.68 |
| qr_512x128_f64/kernix | 3593.402 / 5194.787 | 4309.550 / 5515.558 | 6 | 1704960 | 7.06 |
| qr_512x128_f64/nalgebra | 3123.168 / 3576.259 | 3141.957 / 3552.786 | 4 | 1180672 | 5.90 |
| qr_512x128_f64/numpy | 2492.424 / 2875.135 | 2511.431 / 2888.411 | unavailable | unavailable | 41.69 |
| qr_96x32_f64/faer | 69.763 / 75.506 | 70.818 / 73.058 | 7 | 67584 | 4.99 |
| qr_96x32_f64/kernix | 45.836 / 48.819 | 46.290 / 48.685 | 6 | 82176 | 4.73 |
| qr_96x32_f64/nalgebra | 35.854 / 36.484 | 36.478 / 38.250 | 4 | 57600 | 4.23 |
| qr_96x32_f64/numpy | 41.781 / 46.983 | 41.922 / 43.643 | unavailable | unavailable | 37.51 |
| softmax_256x64_f64/candle | 80.014 / 106.272 | 87.473 / 102.816 | 37 | 265200 | 5.74 |
| softmax_256x64_f64/kernix | 84.535 / 93.309 | 84.811 / 86.751 | 1 | 131072 | 4.81 |
| softmax_256x64_f64/ndarray | 73.625 / 82.112 | 73.128 / 77.316 | 1 | 131072 | 4.70 |
| softmax_256x64_f64/numpy | 101.836 / 110.837 | 108.278 / 156.659 | unavailable | unavailable | 36.25 |
| solve_cholesky_128x1_f64/faer | 3.919 / 4.037 | 4.012 / 4.054 | 1 | 1024 | 5.37 |
| solve_cholesky_128x1_f64/kernix | 11.728 / 13.303 | 12.000 / 12.825 | 1 | 1024 | 4.62 |
| solve_cholesky_128x1_f64/nalgebra | 3.966 / 4.279 | 4.144 / 4.299 | 1 | 1024 | 4.57 |
| solve_cholesky_256x64_f64/faer | 204.617 / 224.206 | 206.704 / 229.932 | 1 | 131072 | 6.69 |
| solve_cholesky_256x64_f64/kernix | 2971.787 / 3267.906 | 2956.516 / 3092.893 | 1 | 131072 | 6.07 |
| solve_cholesky_256x64_f64/nalgebra | 821.816 / 868.279 | 825.986 / 858.005 | 1 | 131072 | 5.78 |
| solve_cholesky_512x128_f64/faer | 1567.762 / 1809.364 | 1566.455 / 1710.773 | 1 | 524288 | 10.89 |
| solve_cholesky_512x128_f64/kernix | 44901.669 / 54651.265 | 42097.614 / 45289.424 | 1 | 524288 | 10.49 |
| solve_cholesky_512x128_f64/nalgebra | 7367.292 / 9625.436 | 6904.120 / 7162.739 | 1 | 524288 | 10.34 |
| solve_lu_256x64_f64/faer | 230.807 / 252.768 | 238.391 / 245.304 | 2 | 262144 | 7.32 |
| solve_lu_256x64_f64/kernix | 2417.794 / 2492.461 | 2446.960 / 2544.112 | 2 | 262144 | 5.81 |
| solve_lu_256x64_f64/nalgebra | 872.068 / 995.648 | 867.072 / 899.004 | 1 | 131072 | 5.89 |
| tensordot_16x32x8_32x8x16_f64/kernix | 36.044 / 40.168 | 36.775 / 37.323 | 9 | 2166 | 4.55 |
| tensordot_16x32x8_32x8x16_f64/numpy | 7.792 / 8.097 | 8.109 / 13.978 | unavailable | unavailable | 36.32 |
| training_backward_2x4x8x4_f64/candle | 13.517 / 13.880 | 14.071 / 14.344 | 472 | 8332 | 6.29 |
| training_backward_2x4x8x4_f64/kernix | 3.162 / 3.264 | 3.157 / 3.234 | 23 | 4224 | 4.91 |
| training_forward_2x4x8x4_f64/candle | 3.245 / 3.402 | 3.385 / 3.490 | 126 | 3912 | 6.19 |
| training_forward_2x4x8x4_f64/kernix | 2.077 / 2.153 | 2.127 / 2.162 | 17 | 1032 | 5.19 |
| training_record_2x4x8x4_f64/candle | 5.127 / 5.419 | 5.226 / 5.436 | 149 | 10712 | 6.18 |
| training_record_2x4x8x4_f64/kernix | 2.605 / 2.677 | 2.706 / 2.810 | 22 | 6832 | 4.62 |
| training_sgd_2x4x8x4_f64/kernix | 0.526 / 0.550 | 0.549 / 0.577 | 5 | 688 | 5.10 |
| training_shared_backward_64x32_f64/candle | 3.892 / 4.030 | 4.058 / 4.441 | 58 | 100004 | 5.62 |
| training_shared_backward_64x32_f64/kernix | 6.713 / 6.779 | 6.350 / 6.591 | 8 | 49768 | 4.62 |

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
