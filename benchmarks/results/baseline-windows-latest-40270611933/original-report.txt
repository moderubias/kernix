# Clean CPU evidence: baseline-windows-latest-40270611933

Measured source `90cbbff60e5d7458ef9f89918c99a6cca94b51d1`, tree `d806b9704a44b4bf2c4c04e664fe3c60eeebc177`; dirty before/after: false.
Library source: registry 0.11.1. CPU affinity: not pinned. Dtype explicitly named per workload; row-major inputs.

| Workload / library | Per-call p50 / p95 (us) | Batch-mean p50 / p95 (us) | Rust allocations | Peak additional requested bytes | Process peak RSS (MiB) |
|---|---:|---:|---:|---:|---:|
| cholesky_factor_128_f64/faer | 59.100 / 59.800 | 60.201 / 60.549 | 2 | 132096 | unavailable |
| cholesky_factor_128_f64/kernix | 230.700 / 258.500 | 232.884 / 240.974 | 1 | 131072 | unavailable |
| cholesky_factor_128_f64/nalgebra | 117.600 / 145.300 | 120.056 / 130.812 | 1 | 131072 | unavailable |
| cholesky_factor_128_f64/numpy | 112.700 / 138.000 | 114.900 / 119.518 | unavailable | unavailable | unavailable |
| cholesky_factor_256_f64/faer | 313.400 / 345.000 | 318.431 / 341.937 | 2 | 526336 | unavailable |
| cholesky_factor_256_f64/kernix | 701.300 / 764.500 | 705.440 / 922.536 | 49 | 645120 | unavailable |
| cholesky_factor_256_f64/nalgebra | 853.400 / 904.800 | 862.408 / 867.000 | 1 | 524288 | unavailable |
| cholesky_factor_256_f64/numpy | 918.200 / 963.300 | 926.855 / 1121.056 | unavailable | unavailable | unavailable |
| cholesky_factor_512_f64/faer | 2817.100 / 3166.400 | 2823.525 / 2885.350 | 2 | 2101248 | unavailable |
| cholesky_factor_512_f64/kernix | 4496.800 / 6450.700 | 4434.967 / 4605.567 | 161 | 2269184 | unavailable |
| cholesky_factor_512_f64/nalgebra | 8035.300 / 8426.000 | 8046.700 / 8184.100 | 1 | 2097152 | unavailable |
| cholesky_factor_512_f64/numpy | 6167.700 / 6835.000 | 6162.650 / 6888.100 | unavailable | unavailable | unavailable |
| cholesky_factor_64_f64/faer | 14.700 / 14.800 | 14.893 / 15.156 | 2 | 33280 | unavailable |
| cholesky_factor_64_f64/kernix | 35.700 / 36.100 | 36.046 / 37.315 | 2 | 33280 | unavailable |
| cholesky_factor_64_f64/nalgebra | 20.200 / 20.300 | 20.506 / 20.622 | 1 | 32768 | unavailable |
| cholesky_factor_64_f64/numpy | 21.100 / 22.200 | 21.750 / 34.818 | unavailable | unavailable | unavailable |
| cholesky_rank_downdate_256_f64/nalgebra | 410.800 / 560.800 | 422.150 / 474.995 | 2 | 526336 | unavailable |
| cholesky_rank_downdate_256_f64/refactor | 1082.300 / 1483.900 | 972.909 / 1121.222 | 49 | 645120 | unavailable |
| cholesky_rank_update_256_f64/nalgebra | 413.200 / 460.300 | 423.092 / 444.565 | 2 | 526336 | unavailable |
| cholesky_rank_update_256_f64/refactor | 1089.000 / 1346.300 | 973.864 / 1069.390 | 49 | 645120 | unavailable |
| cholesky_rank_update_512_f64/nalgebra | 550.100 / 606.200 | 560.483 / 580.011 | 2 | 2101248 | unavailable |
| cholesky_rank_update_512_f64/refactor | 4598.000 / 6412.700 | 4567.733 / 4688.500 | 161 | 2269184 | unavailable |
| create_ones_64x64_f32/candle | 0.700 / 0.700 | 0.704 / 1.024 | 7 | 16696 | unavailable |
| create_ones_64x64_f32/faer | 0.500 / 0.500 | 0.454 / 0.456 | 1 | 16384 | unavailable |
| create_ones_64x64_f32/kernix | 0.500 / 0.500 | 0.477 / 0.483 | 1 | 16384 | unavailable |
| create_ones_64x64_f32/ndarray | 0.400 / 0.400 | 0.390 / 0.393 | 1 | 16384 | unavailable |
| create_ones_64x64_f32/numpy | 2.100 / 2.200 | 2.269 / 3.278 | unavailable | unavailable | unavailable |
| create_zeros_64x64_f32/candle | 0.500 / 0.700 | 0.481 / 0.491 | 5 | 16648 | unavailable |
| create_zeros_64x64_f32/faer | 0.500 / 0.500 | 0.455 / 0.474 | 1 | 16384 | unavailable |
| create_zeros_64x64_f32/kernix | 0.400 / 0.400 | 0.373 / 0.383 | 1 | 16384 | unavailable |
| create_zeros_64x64_f32/ndarray | 0.300 / 0.300 | 0.278 / 0.279 | 1 | 16384 | unavailable |
| create_zeros_64x64_f32/numpy | 0.800 / 1.600 | 0.911 / 0.943 | unavailable | unavailable | unavailable |
| matmul_128_f32/candle | 71.800 / 95.700 | 65.384 / 76.611 | 10 | 65816 | unavailable |
| matmul_128_f32/faer | 50.300 / 63.400 | 51.285 / 59.693 | 1 | 65536 | unavailable |
| matmul_128_f32/kernix | 70.800 / 92.900 | 70.733 / 74.497 | 3 | 65544 | unavailable |
| matmul_128_f32/nalgebra | 54.300 / 57.100 | 55.155 / 96.678 | 2 | 163840 | unavailable |
| matmul_128_f32/ndarray | 109.300 / 150.100 | 115.697 / 117.371 | 2 | 163840 | unavailable |
| matmul_128_f32/numpy | 56.300 / 70.500 | 57.476 / 62.921 | unavailable | unavailable | unavailable |
| matmul_128_f64/candle | 136.100 / 168.900 | 138.656 / 147.291 | 10 | 131352 | unavailable |
| matmul_128_f64/faer | 111.100 / 139.500 | 111.167 / 115.051 | 1 | 131072 | unavailable |
| matmul_128_f64/kernix | 135.400 / 167.100 | 137.551 / 147.944 | 3 | 131080 | unavailable |
| matmul_128_f64/nalgebra | 115.800 / 140.400 | 117.656 / 124.556 | 2 | 327680 | unavailable |
| matmul_128_f64/ndarray | 114.400 / 130.600 | 116.451 / 133.748 | 2 | 327680 | unavailable |
| matmul_128_f64/numpy | 100.300 / 129.700 | 102.721 / 182.462 | unavailable | unavailable | unavailable |
| matmul_256_f32/candle | 427.900 / 465.600 | 433.521 / 562.122 | 10 | 262424 | unavailable |
| matmul_256_f32/faer | 407.500 / 711.500 | 413.540 / 443.035 | 1 | 262144 | unavailable |
| matmul_256_f32/kernix | 357.200 / 387.500 | 367.429 / 486.738 | 3 | 262152 | unavailable |
| matmul_256_f32/nalgebra | 394.800 / 425.300 | 400.148 / 417.083 | 2 | 589824 | unavailable |
| matmul_256_f32/ndarray | 391.400 / 421.100 | 399.296 / 678.640 | 2 | 589824 | unavailable |
| matmul_256_f32/numpy | 386.600 / 421.500 | 393.165 / 425.300 | unavailable | unavailable | unavailable |
| matmul_256_f64/candle | 856.400 / 900.700 | 852.192 / 872.150 | 10 | 524568 | unavailable |
| matmul_256_f64/faer | 1070.600 / 1100.300 | 1058.180 / 1072.520 | 1 | 524288 | unavailable |
| matmul_256_f64/kernix | 725.200 / 811.100 | 744.543 / 860.633 | 3 | 524296 | unavailable |
| matmul_256_f64/nalgebra | 899.700 / 927.200 | 909.075 / 922.591 | 2 | 1179648 | unavailable |
| matmul_256_f64/ndarray | 1149.200 / 1216.700 | 1154.244 / 1738.450 | 2 | 1179648 | unavailable |
| matmul_256_f64/numpy | 755.800 / 793.000 | 762.314 / 772.792 | unavailable | unavailable | unavailable |
| matmul_32_f64/candle | 2.800 / 3.200 | 2.627 / 2.909 | 10 | 8528 | unavailable |
| matmul_32_f64/faer | 2.500 / 2.500 | 2.484 / 2.511 | 1 | 8192 | unavailable |
| matmul_32_f64/kernix | 2.800 / 2.800 | 2.820 / 2.839 | 3 | 24576 | unavailable |
| matmul_32_f64/nalgebra | 2.400 / 2.400 | 2.420 / 2.434 | 2 | 24576 | unavailable |
| matmul_32_f64/ndarray | 2.400 / 2.400 | 2.433 / 2.656 | 2 | 24576 | unavailable |
| matmul_32_f64/numpy | 4.200 / 4.300 | 4.360 / 4.573 | unavailable | unavailable | unavailable |
| matmul_64_f32/candle | 7.400 / 7.500 | 7.232 / 7.358 | 10 | 16664 | unavailable |
| matmul_64_f32/faer | 7.000 / 7.100 | 7.197 / 7.592 | 1 | 16384 | unavailable |
| matmul_64_f32/kernix | 7.700 / 11.900 | 6.849 / 6.997 | 3 | 16512 | unavailable |
| matmul_64_f32/nalgebra | 27.900 / 41.300 | 10.086 / 27.513 | 2 | 49152 | unavailable |
| matmul_64_f32/ndarray | 27.000 / 28.100 | 9.438 / 28.869 | 2 | 49152 | unavailable |
| matmul_64_f32/numpy | 10.500 / 10.600 | 10.695 / 11.822 | unavailable | unavailable | unavailable |
| matmul_64_f64/candle | 14.100 / 14.200 | 14.070 / 14.367 | 10 | 33048 | unavailable |
| matmul_64_f64/faer | 14.500 / 14.600 | 14.720 / 16.028 | 1 | 32768 | unavailable |
| matmul_64_f64/kernix | 13.500 / 13.600 | 13.625 / 14.685 | 3 | 32896 | unavailable |
| matmul_64_f64/nalgebra | 15.500 / 15.700 | 25.745 / 42.386 | 2 | 98304 | unavailable |
| matmul_64_f64/ndarray | 53.200 / 82.100 | 53.124 / 56.930 | 2 | 98304 | unavailable |
| matmul_64_f64/numpy | 16.100 / 16.200 | 16.456 / 16.587 | unavailable | unavailable | unavailable |
| matmul_ab_128_f32/kernix | 75.500 / 79.800 | 76.866 / 81.060 | 3 | 65544 | unavailable |
| matmul_ab_128_f32/numpy | 56.400 / 68.300 | 57.585 / 64.190 | unavailable | unavailable | unavailable |
| matmul_ab_128_f64/kernix | 138.200 / 173.100 | 141.313 / 147.080 | 3 | 131080 | unavailable |
| matmul_ab_128_f64/numpy | 101.800 / 120.500 | 103.916 / 113.749 | unavailable | unavailable | unavailable |
| matmul_batched_4x16x32x16_f32/candle | 4.100 / 5.400 | 3.305 / 3.459 | 26 | 13152 | unavailable |
| matmul_batched_4x16x32x16_f32/kernix | 1.400 / 1.500 | 1.446 / 1.474 | 3 | 4104 | unavailable |
| matmul_batched_4x16x32x16_f32/numpy | 4.800 / 5.000 | 5.033 / 5.049 | unavailable | unavailable | unavailable |
| matmul_batched_4x16x32x16_f64/candle | 4.000 / 4.100 | 4.364 / 4.628 | 26 | 25440 | unavailable |
| matmul_batched_4x16x32x16_f64/kernix | 2.400 / 2.400 | 2.415 / 3.789 | 3 | 8200 | unavailable |
| matmul_batched_4x16x32x16_f64/numpy | 5.000 / 5.200 | 5.215 / 5.503 | unavailable | unavailable | unavailable |
| matmul_sum_backward_32x16x8_f64/candle | 9.900 / 10.200 | 9.801 / 10.049 | 116 | 27988 | unavailable |
| matmul_sum_backward_32x16x8_f64/kernix | 4.500 / 4.600 | 4.157 / 4.304 | 15 | 18376 | unavailable |
| matmul_transposed_128_f32/kernix | 117.900 / 149.500 | 123.151 / 130.557 | 3 | 163840 | unavailable |
| matmul_transposed_128_f32/numpy | 60.600 / 76.100 | 62.138 / 86.039 | unavailable | unavailable | unavailable |
| matmul_transposed_128_f64/kernix | 235.200 / 261.900 | 243.300 / 252.155 | 3 | 327680 | unavailable |
| matmul_transposed_128_f64/numpy | 107.200 / 135.100 | 109.260 / 109.549 | unavailable | unavailable | unavailable |
| min_norm_32x64_rank32_f64/kernix | 110.200 / 144.000 | 85.623 / 116.536 | 20 | 59136 | unavailable |
| min_norm_32x64_rank32_f64/nalgebra | 176.000 / 325.300 | 179.684 / 187.452 | 15 | 42232 | unavailable |
| min_norm_32x64_rank32_f64/numpy | 204.900 / 256.000 | 213.723 / 355.852 | unavailable | unavailable | unavailable |
| min_norm_64x32_rank16_f64/kernix | 97.200 / 170.200 | 66.089 / 99.446 | 20 | 74752 | unavailable |
| min_norm_64x32_rank16_f64/nalgebra | 103.800 / 118.900 | 105.378 / 108.071 | 15 | 41976 | unavailable |
| min_norm_64x32_rank16_f64/numpy | 113.500 / 161.000 | 118.812 / 199.396 | unavailable | unavailable | unavailable |
| parallel_matmul_into_128_f32/kernix | 57.600 / 80.700 | 60.751 / 63.451 | 2 | 16 | unavailable |
| parallel_matmul_into_128_f64/kernix | 83.500 / 116.500 | 88.728 / 95.819 | 2 | 16 | unavailable |
| parallel_matmul_into_256_f64/kernix | 419.800 / 766.000 | 723.414 / 772.362 | 2 | 16 | unavailable |
| parallel_matmul_into_64_f64/kernix | 81.500 / 117.400 | 88.185 / 93.143 | 2 | 98304 | unavailable |
| pivoted_qr_256x64_f64/faer | 395.100 / 437.100 | 358.518 / 412.528 | 10 | 333312 | unavailable |
| pivoted_qr_256x64_f64/kernix | 903.100 / 966.300 | 893.958 / 917.145 | 14 | 559104 | unavailable |
| pivoted_qr_256x64_f64/nalgebra | 905.400 / 949.900 | 909.355 / 924.282 | 7 | 296960 | unavailable |
| pivoted_qr_96x32_f64/faer | 73.500 / 96.500 | 50.774 / 81.010 | 10 | 68352 | unavailable |
| pivoted_qr_96x32_f64/kernix | 118.200 / 151.900 | 121.970 / 126.257 | 14 | 107520 | unavailable |
| pivoted_qr_96x32_f64/nalgebra | 108.500 / 139.400 | 109.825 / 113.987 | 7 | 58368 | unavailable |
| qr_1024x256_f64/faer | 12592.800 / 13435.700 | 12502.400 / 13392.900 | 7 | 5308416 | unavailable |
| qr_1024x256_f64/kernix | 35900.200 / 39268.500 | 35905.600 / 36807.000 | 6 | 6817792 | unavailable |
| qr_1024x256_f64/nalgebra | 32538.700 / 34249.100 | 32414.300 / 33019.200 | 4 | 4720640 | unavailable |
| qr_1024x256_f64/numpy | 21014.000 / 22362.600 | 20914.600 / 23748.800 | unavailable | unavailable | unavailable |
| qr_256x64_f64/faer | 387.200 / 429.800 | 413.720 / 725.136 | 7 | 331776 | unavailable |
| qr_256x64_f64/kernix | 773.700 / 819.700 | 805.100 / 1263.950 | 6 | 426496 | unavailable |
| qr_256x64_f64/nalgebra | 626.800 / 690.900 | 634.819 / 684.233 | 4 | 295424 | unavailable |
| qr_256x64_f64/numpy | 553.100 / 601.900 | 562.272 / 591.841 | unavailable | unavailable | unavailable |
| qr_512x128_f64/faer | 2098.300 / 2238.400 | 2057.040 / 2227.280 | 7 | 1343488 | unavailable |
| qr_512x128_f64/kernix | 4861.300 / 4996.900 | 5016.300 / 8602.800 | 6 | 1704960 | unavailable |
| qr_512x128_f64/nalgebra | 4301.900 / 4751.600 | 4306.033 / 4678.967 | 4 | 1180672 | unavailable |
| qr_512x128_f64/numpy | 4273.200 / 4617.800 | 4276.467 / 4384.333 | unavailable | unavailable | unavailable |
| qr_96x32_f64/faer | 87.100 / 108.200 | 70.131 / 89.525 | 7 | 67584 | unavailable |
| qr_96x32_f64/kernix | 98.300 / 130.200 | 99.874 / 105.544 | 6 | 82176 | unavailable |
| qr_96x32_f64/nalgebra | 78.300 / 90.000 | 76.706 / 92.036 | 4 | 57600 | unavailable |
| qr_96x32_f64/numpy | 76.600 / 106.200 | 78.514 / 133.232 | unavailable | unavailable | unavailable |
| softmax_256x64_f64/candle | 175.000 / 216.400 | 128.009 / 149.897 | 37 | 265200 | unavailable |
| softmax_256x64_f64/kernix | 114.000 / 141.400 | 116.213 / 179.911 | 1 | 131072 | unavailable |
| softmax_256x64_f64/ndarray | 153.500 / 180.900 | 156.608 / 161.054 | 1 | 131072 | unavailable |
| softmax_256x64_f64/numpy | 133.400 / 168.200 | 136.696 / 253.660 | unavailable | unavailable | unavailable |
| solve_cholesky_128x1_f64/faer | 6.600 / 6.700 | 6.566 / 6.590 | 1 | 1024 | unavailable |
| solve_cholesky_128x1_f64/kernix | 19.000 / 19.200 | 19.480 / 25.759 | 1 | 1024 | unavailable |
| solve_cholesky_128x1_f64/nalgebra | 6.400 / 6.500 | 6.003 / 6.092 | 1 | 1024 | unavailable |
| solve_cholesky_256x64_f64/faer | 321.900 / 361.700 | 326.284 / 338.317 | 1 | 131072 | unavailable |
| solve_cholesky_256x64_f64/kernix | 6849.400 / 6978.500 | 6840.750 / 6886.550 | 1 | 131072 | unavailable |
| solve_cholesky_256x64_f64/nalgebra | 1339.200 / 1386.700 | 1343.850 / 1443.514 | 1 | 131072 | unavailable |
| solve_cholesky_512x128_f64/faer | 2242.200 / 2289.000 | 2244.320 / 2362.880 | 1 | 524288 | unavailable |
| solve_cholesky_512x128_f64/kernix | 97706.800 / 98488.700 | 97523.400 / 99684.500 | 1 | 524288 | unavailable |
| solve_cholesky_512x128_f64/nalgebra | 10323.300 / 10762.700 | 10272.400 / 10633.100 | 1 | 524288 | unavailable |
| solve_lu_256x64_f64/faer | 362.900 / 401.300 | 370.859 / 635.756 | 2 | 262144 | unavailable |
| solve_lu_256x64_f64/kernix | 5824.400 / 6042.600 | 5842.700 / 6390.900 | 2 | 262144 | unavailable |
| solve_lu_256x64_f64/nalgebra | 1176.900 / 1223.100 | 1209.856 / 1981.983 | 1 | 131072 | unavailable |
| tensordot_16x32x8_32x8x16_f64/kernix | 63.900 / 76.800 | 65.428 / 66.389 | 9 | 2166 | unavailable |
| tensordot_16x32x8_32x8x16_f64/numpy | 21.600 / 27.100 | 12.654 / 20.664 | unavailable | unavailable | unavailable |
| training_backward_2x4x8x4_f64/candle | 32.100 / 53.200 | 32.129 / 33.384 | 474 | 8332 | unavailable |
| training_backward_2x4x8x4_f64/kernix | 6.100 / 6.200 | 5.857 / 5.960 | 23 | 4224 | unavailable |
| training_forward_2x4x8x4_f64/candle | 8.100 / 8.300 | 8.189 / 8.406 | 127 | 3912 | unavailable |
| training_forward_2x4x8x4_f64/kernix | 3.500 / 3.700 | 3.662 / 6.765 | 17 | 1032 | unavailable |
| training_record_2x4x8x4_f64/candle | 10.300 / 10.800 | 10.172 / 10.301 | 150 | 10712 | unavailable |
| training_record_2x4x8x4_f64/kernix | 5.000 / 5.100 | 4.804 / 4.834 | 22 | 6832 | unavailable |
| training_sgd_2x4x8x4_f64/kernix | 1.000 / 1.000 | 0.989 / 1.020 | 5 | 688 | unavailable |
| training_shared_backward_64x32_f64/candle | 49.200 / 71.300 | 6.790 / 8.624 | 58 | 100004 | unavailable |
| training_shared_backward_64x32_f64/kernix | 10.200 / 10.300 | 10.371 / 10.993 | 8 | 49768 | unavailable |

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
