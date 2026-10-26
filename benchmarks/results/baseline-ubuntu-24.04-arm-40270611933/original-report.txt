# Clean CPU evidence: baseline-ubuntu-24.04-arm-40270611933

Measured source `90cbbff60e5d7458ef9f89918c99a6cca94b51d1`, tree `d806b9704a44b4bf2c4c04e664fe3c60eeebc177`; dirty before/after: false.
Library source: registry 0.11.1. CPU affinity: 0. Dtype explicitly named per workload; row-major inputs.

| Workload / library | Per-call p50 / p95 (us) | Batch-mean p50 / p95 (us) | Rust allocations | Peak additional requested bytes | Process peak RSS (MiB) |
|---|---:|---:|---:|---:|---:|
| cholesky_factor_128_f64/faer | 65.132 / 67.963 | 65.307 / 65.502 | 16 | 148736 | 3.45 |
| cholesky_factor_128_f64/kernix | 277.221 / 280.797 | 276.031 / 278.053 | 1 | 131072 | 3.14 |
| cholesky_factor_128_f64/nalgebra | 128.879 / 131.615 | 130.064 / 131.114 | 1 | 131072 | 2.82 |
| cholesky_factor_128_f64/numpy | 83.691 / 94.001 | 84.901 / 85.379 | unavailable | unavailable | 35.66 |
| cholesky_factor_256_f64/faer | 402.901 / 416.164 | 405.449 / 407.792 | 76 | 592000 | 4.36 |
| cholesky_factor_256_f64/kernix | 767.732 / 776.883 | 766.311 / 772.405 | 49 | 645120 | 4.57 |
| cholesky_factor_256_f64/nalgebra | 1030.499 / 1042.074 | 1033.222 / 1041.601 | 1 | 524288 | 3.58 |
| cholesky_factor_256_f64/numpy | 749.182 / 767.180 | 746.007 / 753.713 | unavailable | unavailable | 38.43 |
| cholesky_factor_512_f64/faer | 2635.678 / 2654.301 | 2643.884 / 2650.893 | 388 | 2297864 | 9.32 |
| cholesky_factor_512_f64/kernix | 4352.098 / 4388.568 | 4320.848 / 4354.194 | 161 | 2269184 | 9.16 |
| cholesky_factor_512_f64/nalgebra | 8613.371 / 8685.046 | 8443.294 / 8648.465 | 1 | 2097152 | 6.58 |
| cholesky_factor_512_f64/numpy | 3859.372 / 3885.235 | 3860.209 / 3871.576 | unavailable | unavailable | 47.92 |
| cholesky_factor_64_f64/faer | 13.248 / 13.287 | 13.282 / 13.317 | 2 | 33280 | 2.92 |
| cholesky_factor_64_f64/kernix | 27.326 / 28.206 | 27.385 / 27.418 | 2 | 33280 | 2.86 |
| cholesky_factor_64_f64/nalgebra | 20.343 / 20.951 | 20.489 / 20.527 | 1 | 32768 | 2.62 |
| cholesky_factor_64_f64/numpy | 18.006 / 18.527 | 18.259 / 18.310 | unavailable | unavailable | 34.80 |
| cholesky_rank_downdate_256_f64/nalgebra | 34.054 / 35.013 | 34.513 / 34.752 | 2 | 526336 | 5.03 |
| cholesky_rank_downdate_256_f64/refactor | 765.573 / 778.139 | 766.001 / 772.577 | 49 | 645120 | 5.08 |
| cholesky_rank_update_256_f64/nalgebra | 33.485 / 34.174 | 33.937 / 34.281 | 2 | 526336 | 5.03 |
| cholesky_rank_update_256_f64/refactor | 762.884 / 775.683 | 763.798 / 769.380 | 49 | 645120 | 5.08 |
| cholesky_rank_update_512_f64/nalgebra | 146.262 / 162.541 | 148.511 / 152.456 | 2 | 2101248 | 12.53 |
| cholesky_rank_update_512_f64/refactor | 4357.818 / 4547.894 | 4347.534 / 4463.395 | 161 | 2269184 | 11.15 |
| create_ones_64x64_f32/candle | 0.344 / 0.352 | 0.350 / 0.351 | 7 | 16696 | 3.03 |
| create_ones_64x64_f32/faer | 0.240 / 0.248 | 0.235 / 0.236 | 1 | 16384 | 2.72 |
| create_ones_64x64_f32/kernix | 0.248 / 0.264 | 0.260 / 0.261 | 1 | 16384 | 2.59 |
| create_ones_64x64_f32/ndarray | 0.208 / 0.216 | 0.217 / 0.217 | 1 | 16384 | 2.59 |
| create_ones_64x64_f32/numpy | 1.520 / 1.560 | 1.591 / 1.598 | unavailable | unavailable | 33.70 |
| create_zeros_64x64_f32/candle | 0.240 / 0.248 | 0.247 / 0.252 | 5 | 16648 | 2.92 |
| create_zeros_64x64_f32/faer | 0.320 / 0.328 | 0.314 / 0.316 | 1 | 16384 | 2.73 |
| create_zeros_64x64_f32/kernix | 0.200 / 0.216 | 0.214 / 0.218 | 1 | 16384 | 2.65 |
| create_zeros_64x64_f32/ndarray | 0.136 / 0.144 | 0.143 / 0.147 | 1 | 16384 | 2.59 |
| create_zeros_64x64_f32/numpy | 0.504 / 0.536 | 0.546 / 0.556 | unavailable | unavailable | 33.69 |
| matmul_128_f32/candle | 85.922 / 87.890 | 86.080 / 86.367 | 10 | 131160 | 3.59 |
| matmul_128_f32/faer | 87.130 / 89.050 | 87.360 / 87.507 | 3 | 196616 | 3.22 |
| matmul_128_f32/kernix | 85.658 / 87.906 | 85.819 / 86.153 | 4 | 131080 | 3.54 |
| matmul_128_f32/nalgebra | 93.434 / 101.913 | 95.105 / 96.306 | 2 | 163840 | 2.91 |
| matmul_128_f32/ndarray | 93.634 / 101.441 | 95.315 / 96.218 | 2 | 163840 | 2.92 |
| matmul_128_f32/numpy | 100.466 / 105.385 | 101.131 / 101.452 | unavailable | unavailable | 35.41 |
| matmul_128_f64/candle | 168.829 / 172.668 | 169.373 / 169.853 | 10 | 262232 | 3.80 |
| matmul_128_f64/faer | 169.260 / 174.477 | 169.828 / 169.997 | 3 | 393224 | 3.53 |
| matmul_128_f64/kernix | 169.069 / 175.348 | 169.634 / 172.341 | 4 | 262152 | 3.78 |
| matmul_128_f64/nalgebra | 210.138 / 221.425 | 210.446 / 211.839 | 2 | 327680 | 2.95 |
| matmul_128_f64/ndarray | 207.458 / 216.354 | 207.163 / 207.943 | 2 | 327680 | 2.95 |
| matmul_128_f64/numpy | 195.170 / 200.851 | 196.726 / 197.623 | unavailable | unavailable | 35.66 |
| matmul_256_f32/candle | 749.902 / 759.701 | 749.164 / 750.721 | 10 | 524376 | 4.31 |
| matmul_256_f32/faer | 675.331 / 680.722 | 671.860 / 679.553 | 3 | 786440 | 4.23 |
| matmul_256_f32/kernix | 661.548 / 666.331 | 660.251 / 662.992 | 4 | 524296 | 4.36 |
| matmul_256_f32/nalgebra | 759.293 / 770.357 | 757.611 / 761.778 | 2 | 589824 | 3.33 |
| matmul_256_f32/ndarray | 760.421 / 770.756 | 759.540 / 762.102 | 2 | 589824 | 3.57 |
| matmul_256_f32/numpy | 722.743 / 730.615 | 722.149 / 725.440 | unavailable | unavailable | 39.56 |
| matmul_256_f64/candle | 1322.999 / 1338.182 | 1322.318 / 1325.526 | 10 | 1048664 | 5.41 |
| matmul_256_f64/faer | 1532.384 / 1546.928 | 1526.163 / 1530.092 | 3 | 1048584 | 5.48 |
| matmul_256_f64/kernix | 1321.711 / 1334.726 | 1322.523 / 1331.475 | 4 | 1048584 | 5.36 |
| matmul_256_f64/nalgebra | 1328.190 / 1341.166 | 1327.027 / 1330.878 | 2 | 1179648 | 4.26 |
| matmul_256_f64/ndarray | 1322.775 / 1333.503 | 1321.132 / 1327.693 | 2 | 1179648 | 4.71 |
| matmul_256_f64/numpy | 1462.838 / 1478.628 | 1464.261 / 1475.038 | unavailable | unavailable | 40.07 |
| matmul_32_f64/candle | 3.448 / 3.488 | 3.463 / 3.499 | 10 | 16976 | 3.20 |
| matmul_32_f64/faer | 3.512 / 3.536 | 3.503 / 3.513 | 3 | 25088 | 2.97 |
| matmul_32_f64/kernix | 3.232 / 3.264 | 3.248 / 3.266 | 3 | 24576 | 2.98 |
| matmul_32_f64/nalgebra | 3.216 / 3.232 | 3.228 / 3.240 | 2 | 24576 | 2.67 |
| matmul_32_f64/ndarray | 3.048 / 3.064 | 3.054 / 3.073 | 2 | 24576 | 2.61 |
| matmul_32_f64/numpy | 8.200 / 8.263 | 8.281 / 8.298 | unavailable | unavailable | 34.36 |
| matmul_64_f32/candle | 11.975 / 12.008 | 12.010 / 12.043 | 10 | 33104 | 3.31 |
| matmul_64_f32/faer | 12.319 / 12.351 | 12.345 / 12.378 | 3 | 49408 | 3.07 |
| matmul_64_f32/kernix | 11.767 / 11.816 | 11.814 / 11.899 | 4 | 33024 | 3.25 |
| matmul_64_f32/nalgebra | 11.280 / 11.311 | 11.304 / 11.375 | 2 | 49152 | 2.76 |
| matmul_64_f32/ndarray | 11.256 / 11.287 | 11.290 / 11.327 | 2 | 49152 | 2.71 |
| matmul_64_f32/numpy | 20.111 / 20.711 | 20.277 / 20.360 | unavailable | unavailable | 34.63 |
| matmul_64_f64/candle | 22.502 / 22.558 | 22.397 / 22.617 | 10 | 65872 | 3.33 |
| matmul_64_f64/faer | 22.654 / 22.702 | 22.671 / 22.715 | 3 | 98560 | 3.05 |
| matmul_64_f64/kernix | 22.239 / 22.318 | 22.286 / 22.326 | 4 | 65792 | 3.32 |
| matmul_64_f64/nalgebra | 22.455 / 22.495 | 22.507 / 22.550 | 2 | 98304 | 2.75 |
| matmul_64_f64/ndarray | 21.782 / 21.815 | 21.838 / 22.040 | 2 | 98304 | 2.71 |
| matmul_64_f64/numpy | 31.054 / 31.766 | 31.219 / 31.358 | unavailable | unavailable | 34.64 |
| matmul_ab_128_f32/kernix | 85.747 / 88.738 | 85.976 / 86.184 | 4 | 131080 | 3.59 |
| matmul_ab_128_f32/numpy | 99.978 / 104.177 | 100.866 / 101.057 | unavailable | unavailable | 35.41 |
| matmul_ab_128_f64/kernix | 168.973 / 175.100 | 169.533 / 169.687 | 4 | 262152 | 3.86 |
| matmul_ab_128_f64/numpy | 195.667 / 203.274 | 197.366 / 198.732 | unavailable | unavailable | 35.66 |
| matmul_batched_4x16x32x16_f32/candle | 2.872 / 2.911 | 2.888 / 2.892 | 25 | 13408 | 3.52 |
| matmul_batched_4x16x32x16_f32/kernix | 2.400 / 2.432 | 2.418 / 2.423 | 7 | 8200 | 2.98 |
| matmul_batched_4x16x32x16_f32/numpy | 5.160 / 5.223 | 5.239 / 5.259 | unavailable | unavailable | 34.34 |
| matmul_batched_4x16x32x16_f64/candle | 4.464 / 4.504 | 4.497 / 4.531 | 25 | 25696 | 3.57 |
| matmul_batched_4x16x32x16_f64/kernix | 3.832 / 3.872 | 3.854 / 3.880 | 7 | 16392 | 3.01 |
| matmul_batched_4x16x32x16_f64/numpy | 12.960 / 13.047 | 13.055 / 13.099 | unavailable | unavailable | 34.37 |
| matmul_sum_backward_32x16x8_f64/candle | 7.343 / 7.567 | 7.331 / 7.373 | 113 | 27980 | 3.71 |
| matmul_sum_backward_32x16x8_f64/kernix | 3.512 / 3.568 | 3.509 / 3.545 | 16 | 18376 | 3.11 |
| matmul_transposed_128_f32/kernix | 92.370 / 102.665 | 94.451 / 95.147 | 3 | 163840 | 3.41 |
| matmul_transposed_128_f32/numpy | 100.249 / 104.121 | 100.923 / 101.264 | unavailable | unavailable | 35.40 |
| matmul_transposed_128_f64/kernix | 215.162 / 231.184 | 217.540 / 219.675 | 3 | 327680 | 3.58 |
| matmul_transposed_128_f64/numpy | 197.507 / 205.043 | 199.641 / 199.902 | unavailable | unavailable | 35.66 |
| min_norm_32x64_rank32_f64/kernix | 96.513 / 100.058 | 96.668 / 96.826 | 20 | 59136 | 3.24 |
| min_norm_32x64_rank32_f64/nalgebra | 218.585 / 222.673 | 218.992 / 219.381 | 15 | 42232 | 2.96 |
| min_norm_32x64_rank32_f64/numpy | 176.300 / 196.171 | 180.190 / 180.866 | unavailable | unavailable | 35.36 |
| min_norm_64x32_rank16_f64/kernix | 72.667 / 75.355 | 72.825 / 72.927 | 20 | 74752 | 3.23 |
| min_norm_64x32_rank16_f64/nalgebra | 117.168 / 119.520 | 117.408 / 117.599 | 15 | 41976 | 2.96 |
| min_norm_64x32_rank16_f64/numpy | 93.754 / 107.249 | 95.573 / 96.258 | unavailable | unavailable | 35.24 |
| parallel_matmul_into_128_f32/kernix | 49.061 / 51.173 | 49.041 / 49.564 | 4 | 65552 | 3.58 |
| parallel_matmul_into_128_f64/kernix | 93.034 / 98.786 | 93.302 / 94.129 | 4 | 131088 | 4.04 |
| parallel_matmul_into_256_f64/kernix | 696.881 / 707.776 | 695.230 / 703.542 | 4 | 524304 | 6.24 |
| parallel_matmul_into_64_f64/kernix | 16.847 / 17.447 | 16.868 / 16.957 | 2 | 98304 | 3.35 |
| pivoted_qr_256x64_f64/faer | 474.328 / 479.615 | 506.608 / 508.774 | 58 | 333312 | 4.03 |
| pivoted_qr_256x64_f64/kernix | 785.043 / 800.066 | 771.055 / 786.661 | 14 | 559104 | 3.60 |
| pivoted_qr_256x64_f64/nalgebra | 1150.730 / 1155.362 | 1150.156 / 1152.283 | 7 | 296960 | 3.30 |
| pivoted_qr_96x32_f64/faer | 62.164 / 64.659 | 62.290 / 63.589 | 32 | 68352 | 3.37 |
| pivoted_qr_96x32_f64/kernix | 88.682 / 90.826 | 88.888 / 88.970 | 14 | 107520 | 2.98 |
| pivoted_qr_96x32_f64/nalgebra | 114.336 / 117.688 | 114.578 / 114.808 | 7 | 58368 | 2.85 |
| qr_1024x256_f64/faer | 20266.282 / 20468.221 | 20220.541 / 20317.391 | 764 | 7135236 | 13.75 |
| qr_1024x256_f64/kernix | 35699.145 / 35929.586 | 35713.855 / 36081.247 | 6 | 6817792 | 14.71 |
| qr_1024x256_f64/nalgebra | 35936.844 / 36048.563 | 35980.896 / 36038.268 | 4 | 4720640 | 9.19 |
| qr_1024x256_f64/numpy | 19667.011 / 19782.484 | 19672.974 / 19790.623 | unavailable | unavailable | 48.90 |
| qr_256x64_f64/faer | 445.986 / 453.305 | 447.389 / 448.087 | 83 | 331776 | 3.79 |
| qr_256x64_f64/kernix | 695.610 / 706.409 | 695.702 / 697.086 | 6 | 426496 | 3.45 |
| qr_256x64_f64/nalgebra | 585.457 / 594.720 | 586.184 / 587.130 | 4 | 295424 | 3.16 |
| qr_256x64_f64/numpy | 562.474 / 583.425 | 560.869 / 569.657 | unavailable | unavailable | 36.04 |
| qr_512x128_f64/faer | 3100.728 / 3129.133 | 3097.956 / 3105.727 | 360 | 1859588 | 5.80 |
| qr_512x128_f64/kernix | 4740.465 / 4771.678 | 4748.048 / 4772.665 | 6 | 1704960 | 5.71 |
| qr_512x128_f64/nalgebra | 4626.744 / 4644.655 | 4621.248 / 4634.154 | 4 | 1180672 | 4.32 |
| qr_512x128_f64/numpy | 4128.250 / 4155.815 | 4117.624 / 4144.955 | unavailable | unavailable | 38.94 |
| qr_96x32_f64/faer | 65.275 / 67.619 | 65.426 / 65.615 | 41 | 67584 | 3.36 |
| qr_96x32_f64/kernix | 71.763 / 73.755 | 71.838 / 71.986 | 6 | 82176 | 2.97 |
| qr_96x32_f64/nalgebra | 61.332 / 63.276 | 61.488 / 61.647 | 4 | 57600 | 2.85 |
| qr_96x32_f64/numpy | 66.099 / 78.027 | 67.500 / 67.945 | unavailable | unavailable | 35.32 |
| softmax_256x64_f64/candle | 110.345 / 112.968 | 110.576 / 110.858 | 37 | 265200 | 4.14 |
| softmax_256x64_f64/kernix | 113.144 / 115.064 | 113.358 / 113.673 | 1 | 131072 | 3.07 |
| softmax_256x64_f64/ndarray | 97.385 / 99.754 | 97.635 / 97.811 | 1 | 131072 | 2.89 |
| softmax_256x64_f64/numpy | 134.623 / 144.214 | 136.173 / 136.416 | unavailable | unavailable | 34.32 |
| solve_cholesky_128x1_f64/faer | 5.919 / 6.288 | 6.006 / 6.026 | 1 | 1024 | 3.56 |
| solve_cholesky_128x1_f64/kernix | 12.624 / 12.743 | 12.820 / 12.849 | 1 | 1024 | 3.02 |
| solve_cholesky_128x1_f64/nalgebra | 5.247 / 5.279 | 5.255 / 5.269 | 1 | 1024 | 2.95 |
| solve_cholesky_256x64_f64/faer | 449.002 / 463.121 | 451.801 / 453.781 | 53 | 327688 | 5.00 |
| solve_cholesky_256x64_f64/kernix | 5108.168 / 5202.994 | 5104.452 / 5174.644 | 1 | 131072 | 4.34 |
| solve_cholesky_256x64_f64/nalgebra | 1275.986 / 1283.425 | 1269.742 / 1274.173 | 1 | 131072 | 4.21 |
| solve_cholesky_512x128_f64/faer | 3191.721 / 3209.015 | 3192.329 / 3210.006 | 205 | 786440 | 9.53 |
| solve_cholesky_512x128_f64/kernix | 46750.296 / 46886.718 | 46680.557 / 46783.174 | 1 | 524288 | 8.53 |
| solve_cholesky_512x128_f64/nalgebra | 12663.051 / 12767.556 | 12711.631 / 12809.353 | 1 | 524288 | 8.72 |
| solve_lu_256x64_f64/faer | 456.217 / 467.456 | 457.779 / 462.583 | 46 | 327688 | 6.07 |
| solve_lu_256x64_f64/kernix | 3429.105 / 3495.349 | 3432.026 / 3483.249 | 2 | 262144 | 4.16 |
| solve_lu_256x64_f64/nalgebra | 1241.292 / 1249.828 | 1241.996 / 1244.963 | 1 | 131072 | 4.22 |
| tensordot_16x32x8_32x8x16_f64/kernix | 46.677 / 48.157 | 46.813 / 47.300 | 9 | 2166 | 2.81 |
| tensordot_16x32x8_32x8x16_f64/numpy | 16.623 / 17.071 | 16.785 / 16.834 | unavailable | unavailable | 34.23 |
| training_backward_2x4x8x4_f64/candle | 21.086 / 21.486 | 21.096 / 21.362 | 476 | 8996 | 4.11 |
| training_backward_2x4x8x4_f64/kernix | 4.207 / 4.344 | 4.244 / 4.260 | 23 | 4224 | 3.21 |
| training_forward_2x4x8x4_f64/candle | 5.287 / 5.376 | 5.315 / 5.352 | 126 | 3912 | 3.99 |
| training_forward_2x4x8x4_f64/kernix | 3.032 / 3.104 | 3.147 / 3.150 | 17 | 1032 | 3.14 |
| training_record_2x4x8x4_f64/candle | 7.608 / 7.824 | 7.417 / 7.624 | 149 | 10712 | 4.01 |
| training_record_2x4x8x4_f64/kernix | 3.847 / 3.919 | 3.881 / 3.890 | 22 | 6832 | 3.15 |
| training_sgd_2x4x8x4_f64/kernix | 0.736 / 0.768 | 0.757 / 0.758 | 5 | 688 | 3.29 |
| training_shared_backward_64x32_f64/candle | 4.968 / 5.032 | 4.977 / 5.049 | 58 | 99996 | 3.71 |
| training_shared_backward_64x32_f64/kernix | 8.367 / 8.416 | 8.392 / 8.422 | 8 | 49768 | 3.06 |

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
