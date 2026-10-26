# Clean CPU evidence: baseline-macos-latest-40270611933

Measured source `90cbbff60e5d7458ef9f89918c99a6cca94b51d1`, tree `d806b9704a44b4bf2c4c04e664fe3c60eeebc177`; dirty before/after: false.
Library source: registry 0.11.1. CPU affinity: not pinned. Dtype explicitly named per workload; row-major inputs.

| Workload / library | Per-call p50 / p95 (us) | Batch-mean p50 / p95 (us) | Rust allocations | Peak additional requested bytes | Process peak RSS (MiB) |
|---|---:|---:|---:|---:|---:|
| cholesky_factor_128_f64/faer | 44.000 / 44.292 | 44.256 / 46.893 | 16 | 149504 | unavailable |
| cholesky_factor_128_f64/kernix | 202.500 / 227.834 | 204.084 / 207.293 | 1 | 131072 | unavailable |
| cholesky_factor_128_f64/nalgebra | 98.833 / 111.416 | 99.265 / 105.479 | 1 | 131072 | unavailable |
| cholesky_factor_128_f64/numpy | 47.166 / 51.833 | 47.543 / 51.788 | unavailable | unavailable | unavailable |
| cholesky_factor_256_f64/faer | 278.916 / 410.167 | 288.752 / 329.769 | 76 | 592384 | unavailable |
| cholesky_factor_256_f64/kernix | 621.250 / 687.292 | 632.031 / 737.545 | 49 | 645120 | unavailable |
| cholesky_factor_256_f64/nalgebra | 731.375 / 861.583 | 904.154 / 1905.014 | 1 | 524288 | unavailable |
| cholesky_factor_256_f64/numpy | 343.083 / 549.500 | 361.545 / 460.716 | unavailable | unavailable | unavailable |
| cholesky_factor_512_f64/faer | 1614.792 / 1641.458 | 1600.399 / 1609.804 | 388 | 2625544 | unavailable |
| cholesky_factor_512_f64/kernix | 3011.042 / 3344.791 | 3009.469 / 3059.750 | 161 | 2269184 | unavailable |
| cholesky_factor_512_f64/nalgebra | 5012.125 / 5257.833 | 5136.875 / 6211.125 | 1 | 2097152 | unavailable |
| cholesky_factor_512_f64/numpy | 2371.250 / 2590.167 | 2370.000 / 2472.742 | unavailable | unavailable | unavailable |
| cholesky_factor_64_f64/faer | 9.959 / 10.125 | 10.153 / 10.255 | 2 | 33280 | unavailable |
| cholesky_factor_64_f64/kernix | 21.917 / 56.083 | 23.064 / 24.480 | 2 | 33280 | unavailable |
| cholesky_factor_64_f64/nalgebra | 21.500 / 26.958 | 25.746 / 41.842 | 1 | 32768 | unavailable |
| cholesky_factor_64_f64/numpy | 13.750 / 14.250 | 17.255 / 17.824 | unavailable | unavailable | unavailable |
| cholesky_rank_downdate_256_f64/nalgebra | 23.708 / 29.625 | 26.064 / 26.788 | 2 | 526336 | unavailable |
| cholesky_rank_downdate_256_f64/refactor | 571.333 / 585.583 | 572.699 / 1472.507 | 49 | 645120 | unavailable |
| cholesky_rank_update_256_f64/nalgebra | 28.875 / 32.000 | 30.719 / 38.219 | 2 | 526336 | unavailable |
| cholesky_rank_update_256_f64/refactor | 700.625 / 937.875 | 709.269 / 866.854 | 49 | 645120 | unavailable |
| cholesky_rank_update_512_f64/nalgebra | 89.667 / 95.750 | 88.484 / 89.913 | 2 | 2101248 | unavailable |
| cholesky_rank_update_512_f64/refactor | 3465.667 / 4516.958 | 3225.021 / 3465.445 | 161 | 2269184 | unavailable |
| create_ones_64x64_f32/candle | 0.500 / 0.792 | 0.463 / 0.964 | 7 | 16696 | unavailable |
| create_ones_64x64_f32/faer | 0.292 / 0.334 | 0.312 / 0.322 | 1 | 16384 | unavailable |
| create_ones_64x64_f32/kernix | 0.750 / 0.834 | 0.394 / 0.497 | 1 | 16384 | unavailable |
| create_ones_64x64_f32/ndarray | 0.334 / 0.500 | 0.445 / 0.734 | 1 | 16384 | unavailable |
| create_ones_64x64_f32/numpy | 1.125 / 1.417 | 1.854 / 11.251 | unavailable | unavailable | unavailable |
| create_zeros_64x64_f32/candle | 0.375 / 0.417 | 0.444 / 1.003 | 5 | 16648 | unavailable |
| create_zeros_64x64_f32/faer | 0.292 / 0.375 | 0.418 / 0.528 | 1 | 16384 | unavailable |
| create_zeros_64x64_f32/kernix | 0.292 / 0.333 | 0.284 / 0.301 | 1 | 16384 | unavailable |
| create_zeros_64x64_f32/ndarray | 0.375 / 0.583 | 0.310 / 0.343 | 1 | 16384 | unavailable |
| create_zeros_64x64_f32/numpy | 0.459 / 0.500 | 0.519 / 0.571 | unavailable | unavailable | unavailable |
| matmul_128_f32/candle | 56.959 / 66.666 | 59.350 / 63.373 | 10 | 327768 | unavailable |
| matmul_128_f32/faer | 51.750 / 51.875 | 51.915 / 63.401 | 3 | 393224 | unavailable |
| matmul_128_f32/kernix | 49.292 / 50.750 | 50.016 / 50.840 | 3 | 163840 | unavailable |
| matmul_128_f32/kernix-accelerate | 6.667 / 8.208 | 6.679 / 7.114 | 1 | 65536 | unavailable |
| matmul_128_f32/nalgebra | 47.750 / 49.458 | 52.344 / 55.117 | 2 | 163840 | unavailable |
| matmul_128_f32/ndarray | 47.792 / 57.791 | 47.965 / 48.119 | 2 | 163840 | unavailable |
| matmul_128_f32/numpy | 6.000 / 6.459 | 6.789 / 14.689 | unavailable | unavailable | unavailable |
| matmul_128_f64/candle | 94.959 / 98.042 | 95.229 / 95.980 | 10 | 655448 | unavailable |
| matmul_128_f64/faer | 117.708 / 314.083 | 115.555 / 152.430 | 3 | 786440 | unavailable |
| matmul_128_f64/kernix | 95.625 / 97.708 | 95.988 / 98.601 | 3 | 327680 | unavailable |
| matmul_128_f64/kernix-accelerate | 16.083 / 16.625 | 15.848 / 16.251 | 1 | 131072 | unavailable |
| matmul_128_f64/nalgebra | 96.750 / 97.167 | 97.080 / 98.029 | 2 | 327680 | unavailable |
| matmul_128_f64/ndarray | 94.167 / 96.042 | 95.033 / 105.070 | 2 | 327680 | unavailable |
| matmul_128_f64/numpy | 15.042 / 15.750 | 15.374 / 15.962 | unavailable | unavailable | unavailable |
| matmul_256_f32/candle | 362.500 / 380.250 | 370.269 / 372.606 | 10 | 786520 | unavailable |
| matmul_256_f32/faer | 385.292 / 411.625 | 389.542 / 396.984 | 3 | 1048584 | unavailable |
| matmul_256_f32/kernix | 360.500 / 376.084 | 361.479 / 363.165 | 3 | 589824 | unavailable |
| matmul_256_f32/kernix-accelerate | 36.125 / 38.792 | 36.903 / 39.048 | 1 | 262144 | unavailable |
| matmul_256_f32/nalgebra | 405.917 / 507.667 | 401.227 / 568.322 | 2 | 589824 | unavailable |
| matmul_256_f32/ndarray | 393.417 / 439.292 | 392.319 / 409.865 | 2 | 589824 | unavailable |
| matmul_256_f32/numpy | 36.500 / 150.500 | 44.197 / 260.283 | unavailable | unavailable | unavailable |
| matmul_256_f64/candle | 891.708 / 1857.875 | 905.049 / 1162.921 | 10 | 1572952 | unavailable |
| matmul_256_f64/faer | 746.459 / 765.667 | 749.673 / 767.467 | 3 | 1572872 | unavailable |
| matmul_256_f64/kernix | 726.291 / 740.417 | 729.137 / 738.312 | 3 | 1179648 | unavailable |
| matmul_256_f64/kernix-accelerate | 120.083 / 130.125 | 115.331 / 130.777 | 1 | 524288 | unavailable |
| matmul_256_f64/nalgebra | 812.166 / 928.959 | 836.597 / 1281.047 | 2 | 1179648 | unavailable |
| matmul_256_f64/ndarray | 737.417 / 748.875 | 737.943 / 755.202 | 2 | 1179648 | unavailable |
| matmul_256_f64/numpy | 99.333 / 100.334 | 99.911 / 101.569 | unavailable | unavailable | unavailable |
| matmul_32_f64/candle | 2.375 / 2.416 | 2.268 / 2.395 | 10 | 18512 | unavailable |
| matmul_32_f64/faer | 3.416 / 19.583 | 2.752 / 2.850 | 3 | 26624 | unavailable |
| matmul_32_f64/kernix | 2.166 / 2.208 | 2.101 / 2.374 | 3 | 24576 | unavailable |
| matmul_32_f64/nalgebra | 1.958 / 1.959 | 1.959 / 2.088 | 2 | 24576 | unavailable |
| matmul_32_f64/ndarray | 1.917 / 1.917 | 1.885 / 1.925 | 2 | 24576 | unavailable |
| matmul_32_f64/numpy | 1.375 / 1.542 | 1.644 / 2.554 | unavailable | unavailable | unavailable |
| matmul_64_f32/candle | 7.333 / 7.625 | 7.324 / 7.393 | 10 | 33872 | unavailable |
| matmul_64_f32/faer | 8.375 / 11.416 | 8.465 / 14.334 | 3 | 50176 | unavailable |
| matmul_64_f32/kernix | 6.750 / 7.792 | 6.749 / 6.968 | 3 | 49152 | unavailable |
| matmul_64_f32/nalgebra | 7.083 / 8.709 | 6.893 / 9.546 | 2 | 49152 | unavailable |
| matmul_64_f32/ndarray | 7.000 / 9.125 | 7.782 / 13.889 | 2 | 49152 | unavailable |
| matmul_64_f32/numpy | 1.916 / 2.084 | 2.038 / 2.896 | unavailable | unavailable | unavailable |
| matmul_64_f64/candle | 13.542 / 13.667 | 13.709 / 14.623 | 10 | 66640 | unavailable |
| matmul_64_f64/faer | 18.042 / 23.333 | 16.592 / 17.518 | 3 | 99328 | unavailable |
| matmul_64_f64/kernix | 12.958 / 13.000 | 13.004 / 13.168 | 3 | 98304 | unavailable |
| matmul_64_f64/nalgebra | 12.584 / 12.625 | 12.798 / 13.177 | 2 | 98304 | unavailable |
| matmul_64_f64/ndarray | 12.375 / 12.792 | 12.662 / 13.144 | 2 | 98304 | unavailable |
| matmul_64_f64/numpy | 3.000 / 3.333 | 3.198 / 3.210 | unavailable | unavailable | unavailable |
| matmul_ab_128_f32/kernix | 52.917 / 65.209 | 55.966 / 63.999 | 3 | 163840 | unavailable |
| matmul_ab_128_f32/numpy | 5.250 / 5.542 | 5.303 / 5.643 | unavailable | unavailable | unavailable |
| matmul_ab_128_f64/kernix | 95.792 / 111.125 | 97.359 / 106.943 | 3 | 327680 | unavailable |
| matmul_ab_128_f64/numpy | 16.584 / 18.042 | 15.165 / 16.436 | unavailable | unavailable | unavailable |
| matmul_batched_4x16x32x16_f32/candle | 3.250 / 3.625 | 2.689 / 2.990 | 25 | 14944 | unavailable |
| matmul_batched_4x16x32x16_f32/kernix | 1.958 / 2.375 | 2.014 / 2.057 | 7 | 8200 | unavailable |
| matmul_batched_4x16x32x16_f32/numpy | 2.042 / 2.584 | 2.233 / 2.310 | unavailable | unavailable | unavailable |
| matmul_batched_4x16x32x16_f64/candle | 3.750 / 4.792 | 3.714 / 4.061 | 25 | 27232 | unavailable |
| matmul_batched_4x16x32x16_f64/kernix | 2.750 / 2.875 | 2.992 / 4.536 | 7 | 16392 | unavailable |
| matmul_batched_4x16x32x16_f64/numpy | 1.875 / 2.083 | 2.235 / 5.591 | unavailable | unavailable | unavailable |
| matmul_sum_backward_32x16x8_f64/candle | 7.125 / 8.750 | 7.065 / 14.935 | 113 | 27980 | unavailable |
| matmul_sum_backward_32x16x8_f64/kernix | 3.083 / 3.875 | 3.169 / 3.346 | 16 | 18376 | unavailable |
| matmul_transposed_128_f32/kernix | 47.792 / 49.708 | 47.988 / 48.309 | 3 | 163840 | unavailable |
| matmul_transposed_128_f32/numpy | 4.417 / 5.250 | 4.630 / 4.676 | unavailable | unavailable | unavailable |
| matmul_transposed_128_f64/kernix | 122.209 / 302.167 | 148.347 / 317.241 | 3 | 327680 | unavailable |
| matmul_transposed_128_f64/numpy | 13.833 / 14.750 | 15.122 / 21.723 | unavailable | unavailable | unavailable |
| min_norm_32x64_rank32_f64/kernix | 60.625 / 71.333 | 60.840 / 61.008 | 20 | 59136 | unavailable |
| min_norm_32x64_rank32_f64/nalgebra | 142.375 / 150.709 | 143.162 / 146.183 | 15 | 42232 | unavailable |
| min_norm_32x64_rank32_f64/numpy | 136.459 / 146.125 | 134.329 / 135.225 | unavailable | unavailable | unavailable |
| min_norm_64x32_rank16_f64/kernix | 45.667 / 47.792 | 45.838 / 47.143 | 20 | 74752 | unavailable |
| min_norm_64x32_rank16_f64/nalgebra | 72.625 / 83.458 | 76.665 / 125.898 | 15 | 41976 | unavailable |
| min_norm_64x32_rank16_f64/numpy | 86.792 / 101.042 | 87.490 / 93.567 | unavailable | unavailable | unavailable |
| parallel_matmul_into_128_f32/kernix | 46.792 / 51.833 | 43.919 / 44.776 | 2 | 196608 | unavailable |
| parallel_matmul_into_128_f64/kernix | 65.375 / 76.375 | 70.089 / 71.605 | 2 | 393216 | unavailable |
| parallel_matmul_into_256_f64/kernix | 403.792 / 414.959 | 405.737 / 408.013 | 2 | 1310720 | unavailable |
| parallel_matmul_into_64_f64/kernix | 19.500 / 30.375 | 30.258 / 35.492 | 2 | 49152 | unavailable |
| pivoted_qr_256x64_f64/faer | 357.125 / 574.583 | 374.005 / 525.125 | 58 | 336904 | unavailable |
| pivoted_qr_256x64_f64/kernix | 605.625 / 667.125 | 601.382 / 627.839 | 14 | 559104 | unavailable |
| pivoted_qr_256x64_f64/nalgebra | 1003.917 / 1055.291 | 1007.321 / 1036.896 | 7 | 296960 | unavailable |
| pivoted_qr_96x32_f64/faer | 42.792 / 52.209 | 38.394 / 39.400 | 32 | 94728 | unavailable |
| pivoted_qr_96x32_f64/kernix | 67.584 / 115.500 | 71.049 / 75.360 | 14 | 107520 | unavailable |
| pivoted_qr_96x32_f64/nalgebra | 93.083 / 98.375 | 93.535 / 97.441 | 7 | 58368 | unavailable |
| qr_1024x256_f64/faer | 15215.375 / 25582.042 | 19454.875 / 24730.542 | 764 | 9166852 | unavailable |
| qr_1024x256_f64/kernix | 33113.250 / 44868.792 | 30561.166 / 32451.833 | 6 | 6817792 | unavailable |
| qr_1024x256_f64/nalgebra | 24295.583 / 27500.250 | 26069.000 / 26767.208 | 4 | 4720640 | unavailable |
| qr_1024x256_f64/numpy | 13464.750 / 18346.875 | 13322.750 / 13844.292 | unavailable | unavailable | unavailable |
| qr_256x64_f64/faer | 356.750 / 900.291 | 329.554 / 362.500 | 83 | 335880 | unavailable |
| qr_256x64_f64/kernix | 516.250 / 602.500 | 608.684 / 1084.188 | 6 | 426496 | unavailable |
| qr_256x64_f64/nalgebra | 392.875 / 723.250 | 443.422 / 597.078 | 4 | 295424 | unavailable |
| qr_256x64_f64/numpy | 369.084 / 400.000 | 344.054 / 345.667 | unavailable | unavailable | unavailable |
| qr_512x128_f64/faer | 1959.125 / 4015.667 | 1882.188 / 3187.031 | 360 | 3334148 | unavailable |
| qr_512x128_f64/kernix | 4014.292 / 4573.958 | 4051.583 / 4425.180 | 6 | 1704960 | unavailable |
| qr_512x128_f64/nalgebra | 2785.125 / 2857.458 | 2784.729 / 2806.354 | 4 | 1180672 | unavailable |
| qr_512x128_f64/numpy | 2049.125 / 2101.500 | 2053.475 / 2141.475 | unavailable | unavailable | unavailable |
| qr_96x32_f64/faer | 43.000 / 43.250 | 43.147 / 43.385 | 41 | 94216 | unavailable |
| qr_96x32_f64/kernix | 53.042 / 53.250 | 53.413 / 54.376 | 6 | 82176 | unavailable |
| qr_96x32_f64/nalgebra | 39.209 / 48.500 | 40.081 / 41.077 | 4 | 57600 | unavailable |
| qr_96x32_f64/numpy | 49.125 / 65.625 | 49.366 / 53.802 | unavailable | unavailable | unavailable |
| softmax_256x64_f64/candle | 74.792 / 270.083 | 76.561 / 85.346 | 37 | 265200 | unavailable |
| softmax_256x64_f64/kernix | 84.125 / 253.917 | 105.519 / 357.522 | 1 | 131072 | unavailable |
| softmax_256x64_f64/ndarray | 71.667 / 76.333 | 71.629 / 73.385 | 1 | 131072 | unavailable |
| softmax_256x64_f64/numpy | 86.959 / 114.334 | 84.464 / 99.225 | unavailable | unavailable | unavailable |
| solve_cholesky_128x1_f64/faer | 4.417 / 4.750 | 4.414 / 4.680 | 1 | 1024 | unavailable |
| solve_cholesky_128x1_f64/kernix | 18.625 / 20.166 | 17.927 / 18.416 | 1 | 1024 | unavailable |
| solve_cholesky_128x1_f64/nalgebra | 4.792 / 5.959 | 5.147 / 5.517 | 1 | 1024 | unavailable |
| solve_cholesky_256x64_f64/faer | 319.625 / 357.917 | 324.233 / 386.620 | 53 | 786440 | unavailable |
| solve_cholesky_256x64_f64/kernix | 3459.083 / 3604.500 | 3462.653 / 3624.639 | 1 | 131072 | unavailable |
| solve_cholesky_256x64_f64/nalgebra | 1071.292 / 1175.708 | 1048.325 / 1079.971 | 1 | 131072 | unavailable |
| solve_cholesky_512x128_f64/faer | 2576.125 / 4177.667 | 2571.615 / 5121.333 | 205 | 1572872 | unavailable |
| solve_cholesky_512x128_f64/kernix | 36528.084 / 46379.334 | 36303.542 / 38117.292 | 1 | 524288 | unavailable |
| solve_cholesky_512x128_f64/nalgebra | 7621.667 / 9815.625 | 9083.083 / 17466.709 | 1 | 524288 | unavailable |
| solve_lu_256x64_f64/faer | 301.958 / 353.917 | 323.402 / 777.321 | 46 | 786440 | unavailable |
| solve_lu_256x64_f64/kernix | 3778.625 / 4591.083 | 4512.306 / 5616.458 | 2 | 262144 | unavailable |
| solve_lu_256x64_f64/nalgebra | 822.458 / 839.334 | 828.003 / 1011.400 | 1 | 131072 | unavailable |
| tensordot_16x32x8_32x8x16_f64/kernix | 64.459 / 77.416 | 66.281 / 116.970 | 9 | 2166 | unavailable |
| tensordot_16x32x8_32x8x16_f64/numpy | 4.875 / 5.416 | 5.013 / 5.308 | unavailable | unavailable | unavailable |
| training_backward_2x4x8x4_f64/candle | 23.375 / 33.083 | 18.774 / 20.971 | 476 | 21284 | unavailable |
| training_backward_2x4x8x4_f64/kernix | 3.250 / 3.292 | 2.829 / 3.190 | 23 | 4224 | unavailable |
| training_forward_2x4x8x4_f64/candle | 5.875 / 7.292 | 5.466 / 7.275 | 126 | 9568 | unavailable |
| training_forward_2x4x8x4_f64/kernix | 2.167 / 2.209 | 1.946 / 2.208 | 17 | 1032 | unavailable |
| training_record_2x4x8x4_f64/candle | 6.333 / 8.083 | 5.778 / 6.286 | 149 | 11544 | unavailable |
| training_record_2x4x8x4_f64/kernix | 3.250 / 4.083 | 3.186 / 3.630 | 22 | 6832 | unavailable |
| training_sgd_2x4x8x4_f64/kernix | 0.791 / 1.000 | 0.885 / 1.147 | 5 | 688 | unavailable |
| training_shared_backward_64x32_f64/candle | 3.750 / 3.834 | 3.551 / 3.759 | 58 | 99996 | unavailable |
| training_shared_backward_64x32_f64/kernix | 6.000 / 7.583 | 6.442 / 6.725 | 8 | 49768 | unavailable |

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
