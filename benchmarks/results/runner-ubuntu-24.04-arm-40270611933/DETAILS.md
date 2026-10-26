# Clean CPU evidence: runner-ubuntu-24.04-arm-40270611933

Measured source `90cbbff60e5d7458ef9f89918c99a6cca94b51d1`, tree `d806b9704a44b4bf2c4c04e664fe3c60eeebc177`; dirty before/after: false.
Library source: workspace 90cbbff60e5d7458ef9f89918c99a6cca94b51d1. CPU affinity: 0. Dtype explicitly named per workload; row-major inputs.

| Workload / library | Per-call p50 / p95 (us) | Batch-mean p50 / p95 (us) | Rust allocations | Peak additional requested bytes | Process peak RSS (MiB) |
|---|---:|---:|---:|---:|---:|
| cholesky_factor_128_f64/faer | 65.236 / 67.339 | 65.381 / 65.588 | 16 | 148736 | 3.50 |
| cholesky_factor_128_f64/kernix | 279.326 / 282.621 | 278.608 / 280.169 | 1 | 131072 | 3.27 |
| cholesky_factor_128_f64/nalgebra | 128.896 / 131.615 | 130.179 / 130.496 | 1 | 131072 | 2.89 |
| cholesky_factor_128_f64/numpy | 82.282 / 94.354 | 83.548 / 83.989 | unavailable | unavailable | 35.64 |
| cholesky_factor_256_f64/faer | 403.316 / 413.772 | 404.837 / 405.596 | 76 | 592000 | 4.41 |
| cholesky_factor_256_f64/kernix | 763.797 / 772.428 | 762.113 / 768.352 | 49 | 645120 | 4.63 |
| cholesky_factor_256_f64/nalgebra | 1029.890 / 1037.562 | 1035.284 / 1039.755 | 1 | 524288 | 3.62 |
| cholesky_factor_256_f64/numpy | 748.773 / 767.604 | 747.419 / 753.844 | unavailable | unavailable | 38.41 |
| cholesky_factor_512_f64/faer | 2639.439 / 2658.733 | 2631.555 / 2641.704 | 388 | 2297864 | 9.38 |
| cholesky_factor_512_f64/kernix | 4320.437 / 4358.203 | 4312.769 / 4360.104 | 161 | 2269184 | 9.18 |
| cholesky_factor_512_f64/nalgebra | 8407.403 / 8484.901 | 8128.773 / 8496.972 | 1 | 2097152 | 6.63 |
| cholesky_factor_512_f64/numpy | 3804.216 / 3826.470 | 3799.067 / 3820.098 | unavailable | unavailable | 47.91 |
| cholesky_factor_64_f64/faer | 13.247 / 13.287 | 13.286 / 13.320 | 2 | 33280 | 3.04 |
| cholesky_factor_64_f64/kernix | 27.374 / 28.166 | 27.432 / 27.502 | 2 | 33280 | 3.04 |
| cholesky_factor_64_f64/nalgebra | 20.334 / 21.030 | 20.500 / 20.530 | 1 | 32768 | 2.70 |
| cholesky_factor_64_f64/numpy | 17.983 / 18.695 | 18.191 / 18.251 | unavailable | unavailable | 34.79 |
| cholesky_rank_downdate_256_f64/kernix | 151.558 / 159.910 | 154.061 / 154.331 | 3 | 526336 | 4.40 |
| cholesky_rank_downdate_256_f64/nalgebra | 34.157 / 34.982 | 34.545 / 34.941 | 2 | 526336 | 5.03 |
| cholesky_rank_downdate_256_f64/refactor | 764.757 / 778.628 | 764.882 / 770.077 | 49 | 645120 | 5.08 |
| cholesky_rank_update_256_f64/kernix | 151.678 / 163.269 | 154.103 / 154.592 | 3 | 526336 | 4.40 |
| cholesky_rank_update_256_f64/nalgebra | 33.845 / 34.549 | 34.371 / 34.718 | 2 | 526336 | 5.03 |
| cholesky_rank_update_256_f64/refactor | 765.988 / 775.692 | 764.642 / 767.128 | 49 | 645120 | 5.07 |
| cholesky_rank_update_512_f64/kernix | 654.660 / 668.027 | 669.229 / 677.618 | 3 | 2101248 | 8.91 |
| cholesky_rank_update_512_f64/nalgebra | 145.559 / 159.790 | 147.169 / 150.861 | 2 | 2101248 | 12.53 |
| cholesky_rank_update_512_f64/refactor | 4323.053 / 4359.962 | 4313.645 / 4335.185 | 161 | 2269184 | 11.13 |
| create_ones_64x64_f32/candle | 0.344 / 0.352 | 0.349 / 0.350 | 7 | 16696 | 3.17 |
| create_ones_64x64_f32/faer | 0.240 / 0.248 | 0.230 / 0.230 | 1 | 16384 | 2.79 |
| create_ones_64x64_f32/kernix | 0.256 / 0.256 | 0.259 / 0.260 | 1 | 16384 | 2.65 |
| create_ones_64x64_f32/ndarray | 0.208 / 0.216 | 0.216 / 0.216 | 1 | 16384 | 2.65 |
| create_ones_64x64_f32/numpy | 1.520 / 1.568 | 1.591 / 1.601 | unavailable | unavailable | 33.68 |
| create_zeros_64x64_f32/candle | 0.232 / 0.248 | 0.251 / 0.254 | 5 | 16648 | 3.05 |
| create_zeros_64x64_f32/faer | 0.320 / 0.328 | 0.314 / 0.322 | 1 | 16384 | 2.79 |
| create_zeros_64x64_f32/kernix | 0.208 / 0.224 | 0.213 / 0.220 | 1 | 16384 | 2.71 |
| create_zeros_64x64_f32/ndarray | 0.136 / 0.144 | 0.149 / 0.157 | 1 | 16384 | 2.63 |
| create_zeros_64x64_f32/numpy | 0.512 / 0.544 | 0.551 / 0.568 | unavailable | unavailable | 33.66 |
| matmul_128_f32/candle | 85.930 / 88.066 | 86.085 / 86.356 | 10 | 131160 | 3.78 |
| matmul_128_f32/faer | 87.146 / 89.682 | 87.360 / 87.461 | 3 | 196616 | 3.22 |
| matmul_128_f32/kernix | 85.586 / 87.778 | 85.563 / 86.280 | 4 | 131080 | 3.52 |
| matmul_128_f32/nalgebra | 93.250 / 101.369 | 94.794 / 95.161 | 2 | 163840 | 2.98 |
| matmul_128_f32/ndarray | 94.098 / 102.321 | 95.118 / 95.643 | 2 | 163840 | 2.96 |
| matmul_128_f32/numpy | 99.953 / 102.313 | 100.331 / 100.571 | unavailable | unavailable | 35.37 |
| matmul_128_f64/candle | 168.820 / 171.917 | 169.285 / 169.445 | 10 | 262232 | 4.05 |
| matmul_128_f64/faer | 168.933 / 175.357 | 169.521 / 169.665 | 3 | 393224 | 3.59 |
| matmul_128_f64/kernix | 168.285 / 171.452 | 168.690 / 169.095 | 4 | 262152 | 3.79 |
| matmul_128_f64/nalgebra | 207.994 / 215.866 | 207.952 / 208.621 | 2 | 327680 | 2.95 |
| matmul_128_f64/ndarray | 210.954 / 223.688 | 210.658 / 211.721 | 2 | 327680 | 3.01 |
| matmul_128_f64/numpy | 195.723 / 202.682 | 197.536 / 198.330 | unavailable | unavailable | 35.63 |
| matmul_256_f32/candle | 753.950 / 762.701 | 752.338 / 756.333 | 10 | 524376 | 4.43 |
| matmul_256_f32/faer | 673.002 / 678.611 | 671.958 / 676.676 | 3 | 786440 | 4.22 |
| matmul_256_f32/kernix | 663.100 / 669.603 | 660.235 / 662.040 | 4 | 524296 | 4.36 |
| matmul_256_f32/nalgebra | 756.726 / 771.588 | 752.954 / 763.269 | 2 | 589824 | 3.33 |
| matmul_256_f32/ndarray | 757.653 / 767.644 | 756.107 / 758.920 | 2 | 589824 | 3.46 |
| matmul_256_f32/numpy | 724.352 / 733.191 | 725.071 / 728.997 | unavailable | unavailable | 39.55 |
| matmul_256_f64/candle | 1321.199 / 1334.798 | 1321.808 / 1325.037 | 10 | 1048664 | 5.66 |
| matmul_256_f64/faer | 1534.937 / 1551.383 | 1536.019 / 1545.070 | 3 | 1048584 | 5.35 |
| matmul_256_f64/kernix | 1320.480 / 1331.310 | 1319.289 / 1321.992 | 4 | 1048584 | 5.36 |
| matmul_256_f64/nalgebra | 1332.286 / 1342.990 | 1329.221 / 1333.856 | 2 | 1179648 | 4.33 |
| matmul_256_f64/ndarray | 1323.982 / 1337.782 | 1322.145 / 1332.793 | 2 | 1179648 | 4.77 |
| matmul_256_f64/numpy | 1469.973 / 1482.301 | 1467.691 / 1471.800 | unavailable | unavailable | 40.03 |
| matmul_32_f64/candle | 3.424 / 3.464 | 3.450 / 3.456 | 10 | 16976 | 3.46 |
| matmul_32_f64/faer | 3.504 / 3.535 | 3.489 / 3.504 | 3 | 25088 | 3.03 |
| matmul_32_f64/kernix | 3.248 / 3.280 | 3.258 / 3.262 | 3 | 24576 | 2.93 |
| matmul_32_f64/nalgebra | 3.224 / 3.248 | 3.238 / 3.241 | 2 | 24576 | 2.73 |
| matmul_32_f64/ndarray | 3.048 / 3.080 | 3.056 / 3.076 | 2 | 24576 | 2.67 |
| matmul_32_f64/numpy | 8.199 / 8.336 | 8.283 / 8.308 | unavailable | unavailable | 34.34 |
| matmul_64_f32/candle | 11.952 / 11.992 | 11.996 / 12.045 | 10 | 33104 | 3.51 |
| matmul_64_f32/faer | 12.287 / 12.311 | 12.311 / 12.343 | 3 | 49408 | 3.07 |
| matmul_64_f32/kernix | 11.744 / 11.791 | 11.781 / 11.838 | 4 | 33024 | 3.27 |
| matmul_64_f32/nalgebra | 11.263 / 11.288 | 11.293 / 11.346 | 2 | 49152 | 2.80 |
| matmul_64_f32/ndarray | 11.264 / 11.287 | 11.276 / 11.314 | 2 | 49152 | 2.78 |
| matmul_64_f32/numpy | 20.095 / 20.687 | 20.265 / 20.310 | unavailable | unavailable | 34.57 |
| matmul_64_f64/candle | 22.590 / 22.639 | 22.636 / 22.696 | 10 | 65872 | 3.59 |
| matmul_64_f64/faer | 22.615 / 22.679 | 22.681 / 22.743 | 3 | 98560 | 3.11 |
| matmul_64_f64/kernix | 22.167 / 22.247 | 22.237 / 22.252 | 4 | 65792 | 3.32 |
| matmul_64_f64/nalgebra | 22.455 / 22.510 | 22.520 / 23.087 | 2 | 98304 | 2.82 |
| matmul_64_f64/ndarray | 21.815 / 21.871 | 21.863 / 21.872 | 2 | 98304 | 2.79 |
| matmul_64_f64/numpy | 31.046 / 31.782 | 31.254 / 31.312 | unavailable | unavailable | 34.62 |
| matmul_ab_128_f32/kernix | 86.258 / 89.106 | 86.494 / 86.724 | 4 | 131080 | 3.60 |
| matmul_ab_128_f32/numpy | 99.857 / 103.321 | 100.386 / 100.582 | unavailable | unavailable | 35.39 |
| matmul_ab_128_f64/kernix | 168.484 / 174.020 | 169.214 / 169.672 | 4 | 262152 | 3.86 |
| matmul_ab_128_f64/numpy | 195.419 / 202.866 | 197.053 / 197.631 | unavailable | unavailable | 35.64 |
| matmul_batched_4x16x32x16_f32/candle | 2.880 / 2.912 | 2.892 / 2.900 | 25 | 13408 | 3.65 |
| matmul_batched_4x16x32x16_f32/kernix | 2.384 / 2.416 | 2.402 / 2.410 | 7 | 8200 | 2.97 |
| matmul_batched_4x16x32x16_f32/numpy | 5.160 / 5.200 | 5.242 / 5.253 | unavailable | unavailable | 34.34 |
| matmul_batched_4x16x32x16_f64/candle | 4.432 / 4.480 | 4.461 / 4.509 | 25 | 25696 | 3.75 |
| matmul_batched_4x16x32x16_f64/kernix | 3.855 / 3.896 | 3.870 / 3.884 | 7 | 16392 | 3.01 |
| matmul_batched_4x16x32x16_f64/numpy | 12.991 / 13.063 | 13.086 / 13.192 | unavailable | unavailable | 34.34 |
| matmul_sum_backward_32x16x8_f64/candle | 7.183 / 7.447 | 7.210 / 7.278 | 113 | 27980 | 3.90 |
| matmul_sum_backward_32x16x8_f64/kernix | 3.519 / 3.592 | 3.515 / 3.538 | 16 | 18376 | 3.04 |
| matmul_transposed_128_f32/kernix | 92.713 / 101.401 | 94.908 / 95.280 | 3 | 163840 | 3.39 |
| matmul_transposed_128_f32/numpy | 100.370 / 105.145 | 101.074 / 101.252 | unavailable | unavailable | 35.38 |
| matmul_transposed_128_f64/kernix | 212.186 / 226.896 | 213.099 / 213.648 | 3 | 327680 | 3.45 |
| matmul_transposed_128_f64/numpy | 197.098 / 204.650 | 198.856 / 199.230 | unavailable | unavailable | 35.64 |
| min_norm_32x64_rank32_f64/kernix | 96.250 / 99.041 | 96.352 / 96.505 | 20 | 59136 | 3.18 |
| min_norm_32x64_rank32_f64/nalgebra | 218.081 / 220.482 | 218.450 / 218.958 | 15 | 42232 | 3.02 |
| min_norm_32x64_rank32_f64/numpy | 176.948 / 198.050 | 180.122 / 181.067 | unavailable | unavailable | 35.35 |
| min_norm_64x32_rank16_f64/kernix | 72.267 / 74.739 | 72.471 / 72.606 | 20 | 74752 | 3.18 |
| min_norm_64x32_rank16_f64/nalgebra | 117.024 / 120.008 | 117.251 / 117.392 | 15 | 41976 | 3.02 |
| min_norm_64x32_rank16_f64/numpy | 93.650 / 109.313 | 95.369 / 96.441 | unavailable | unavailable | 35.22 |
| parallel_matmul_into_128_f32/kernix | 49.044 / 51.189 | 49.207 / 49.461 | 4 | 65552 | 3.88 |
| parallel_matmul_into_128_f64/kernix | 92.914 / 95.346 | 91.465 / 93.288 | 4 | 131088 | 4.18 |
| parallel_matmul_into_256_f64/kernix | 698.185 / 712.328 | 701.034 / 726.305 | 4 | 524304 | 6.37 |
| parallel_matmul_into_64_f64/kernix | 16.798 / 17.215 | 16.857 / 16.904 | 2 | 98304 | 3.35 |
| pivoted_qr_256x64_f64/faer | 474.448 / 486.224 | 506.477 / 509.228 | 58 | 333312 | 4.08 |
| pivoted_qr_256x64_f64/kernix | 783.100 / 793.570 | 768.142 / 783.238 | 14 | 559104 | 3.61 |
| pivoted_qr_256x64_f64/nalgebra | 1150.907 / 1154.514 | 1150.218 / 1151.371 | 7 | 296960 | 3.36 |
| pivoted_qr_96x32_f64/faer | 62.236 / 64.275 | 62.436 / 62.470 | 32 | 68352 | 3.43 |
| pivoted_qr_96x32_f64/kernix | 88.378 / 90.626 | 88.527 / 88.608 | 14 | 107520 | 3.12 |
| pivoted_qr_96x32_f64/nalgebra | 114.272 / 116.472 | 114.504 / 114.591 | 7 | 58368 | 2.93 |
| qr_1024x256_f64/faer | 20071.459 / 20232.160 | 20082.786 / 20188.539 | 764 | 7135236 | 13.62 |
| qr_1024x256_f64/kernix | 23959.756 / 24217.235 | 23858.299 / 24072.740 | 51 | 7276544 | 14.83 |
| qr_1024x256_f64/nalgebra | 35803.543 / 35939.859 | 35773.841 / 36002.761 | 4 | 4720640 | 9.20 |
| qr_1024x256_f64/numpy | 19573.491 / 19690.763 | 19612.520 / 19672.852 | unavailable | unavailable | 48.88 |
| qr_256x64_f64/faer | 451.250 / 458.809 | 452.539 / 454.090 | 83 | 331776 | 3.79 |
| qr_256x64_f64/kernix | 693.442 / 704.440 | 692.337 / 695.225 | 6 | 426496 | 3.45 |
| qr_256x64_f64/nalgebra | 586.225 / 595.152 | 587.389 / 589.728 | 4 | 295424 | 3.23 |
| qr_256x64_f64/numpy | 565.426 / 584.496 | 563.502 / 570.598 | unavailable | unavailable | 36.07 |
| qr_512x128_f64/faer | 3095.064 / 3113.495 | 3104.955 / 3122.420 | 360 | 1859588 | 5.68 |
| qr_512x128_f64/kernix | 4122.523 / 4171.271 | 4132.642 / 4151.393 | 31 | 2098176 | 6.21 |
| qr_512x128_f64/nalgebra | 4608.794 / 4629.360 | 4605.508 / 4611.644 | 4 | 1180672 | 4.33 |
| qr_512x128_f64/numpy | 4138.346 / 4164.631 | 4137.839 / 4155.115 | unavailable | unavailable | 38.93 |
| qr_96x32_f64/faer | 65.452 / 67.515 | 65.576 / 65.649 | 41 | 67584 | 3.36 |
| qr_96x32_f64/kernix | 73.411 / 75.339 | 73.487 / 74.600 | 6 | 82176 | 3.04 |
| qr_96x32_f64/nalgebra | 61.308 / 63.180 | 61.461 / 61.556 | 4 | 57600 | 2.92 |
| qr_96x32_f64/numpy | 66.027 / 82.162 | 67.665 / 67.945 | unavailable | unavailable | 35.31 |
| softmax_256x64_f64/candle | 110.384 / 112.600 | 110.601 / 110.861 | 37 | 265200 | 4.27 |
| softmax_256x64_f64/kernix | 113.008 / 115.057 | 113.248 / 113.438 | 1 | 131072 | 3.07 |
| softmax_256x64_f64/ndarray | 97.465 / 99.249 | 97.696 / 97.869 | 1 | 131072 | 3.00 |
| softmax_256x64_f64/numpy | 134.095 / 141.302 | 135.215 / 135.698 | unavailable | unavailable | 34.31 |
| solve_cholesky_128x1_f64/faer | 5.896 / 6.271 | 5.999 / 6.011 | 1 | 1024 | 3.62 |
| solve_cholesky_128x1_f64/kernix | 12.839 / 13.199 | 12.867 / 12.937 | 1 | 1024 | 3.14 |
| solve_cholesky_128x1_f64/nalgebra | 5.224 / 5.256 | 5.246 / 5.259 | 1 | 1024 | 3.02 |
| solve_cholesky_256x64_f64/faer | 448.842 / 460.089 | 450.861 / 452.788 | 53 | 327688 | 5.04 |
| solve_cholesky_256x64_f64/kernix | 1718.605 / 1724.860 | 1718.812 / 1721.597 | 1 | 131072 | 4.34 |
| solve_cholesky_256x64_f64/nalgebra | 1276.226 / 1283.321 | 1267.954 / 1278.678 | 1 | 131072 | 4.27 |
| solve_cholesky_512x128_f64/faer | 3181.250 / 3200.025 | 3180.474 / 3188.032 | 205 | 786440 | 9.59 |
| solve_cholesky_512x128_f64/kernix | 13442.704 / 13669.321 | 13432.704 / 13496.748 | 1 | 524288 | 8.46 |
| solve_cholesky_512x128_f64/nalgebra | 12652.581 / 12763.141 | 12717.249 / 12847.160 | 1 | 524288 | 8.78 |
| solve_lu_256x64_f64/faer | 454.778 / 466.745 | 456.074 / 462.459 | 46 | 327688 | 6.14 |
| solve_lu_256x64_f64/kernix | 1652.017 / 1658.880 | 1654.896 / 1657.390 | 2 | 262144 | 4.28 |
| solve_lu_256x64_f64/nalgebra | 1228.773 / 1236.509 | 1229.039 / 1232.517 | 1 | 131072 | 4.27 |
| tensordot_16x32x8_32x8x16_f64/kernix | 46.733 / 48.581 | 46.807 / 46.912 | 9 | 2166 | 2.89 |
| tensordot_16x32x8_32x8x16_f64/numpy | 16.431 / 16.799 | 16.670 / 16.789 | unavailable | unavailable | 34.23 |
| training_backward_2x4x8x4_f64/candle | 21.199 / 21.534 | 21.229 / 21.292 | 476 | 8996 | 4.29 |
| training_backward_2x4x8x4_f64/kernix | 4.183 / 4.296 | 4.249 / 4.256 | 23 | 4224 | 3.28 |
| training_forward_2x4x8x4_f64/candle | 5.280 / 5.368 | 5.292 / 5.324 | 126 | 3912 | 4.18 |
| training_forward_2x4x8x4_f64/kernix | 3.048 / 3.112 | 3.162 / 3.173 | 17 | 1032 | 3.27 |
| training_record_2x4x8x4_f64/candle | 7.607 / 7.799 | 7.515 / 7.618 | 149 | 10712 | 4.20 |
| training_record_2x4x8x4_f64/kernix | 3.848 / 3.911 | 3.885 / 3.905 | 22 | 6832 | 3.28 |
| training_sgd_2x4x8x4_f64/kernix | 0.744 / 0.776 | 0.757 / 0.760 | 5 | 688 | 3.40 |
| training_shared_backward_64x32_f64/candle | 4.991 / 5.048 | 4.969 / 5.033 | 58 | 99996 | 3.92 |
| training_shared_backward_64x32_f64/kernix | 8.408 / 8.463 | 8.444 / 8.465 | 8 | 49768 | 3.23 |

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
