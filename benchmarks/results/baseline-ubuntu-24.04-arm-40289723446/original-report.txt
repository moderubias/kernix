# Clean CPU evidence: baseline-ubuntu-24.04-arm-40289723446

Measured source `7c11b2165b0cb7b523db4d938a36f1396396a138`, tree `cc7ac522fccee7e655adb7ca8d3644d95344fcb1`; dirty before/after: false.
Library source: git v0.12.0 90cbbff60e5d7458ef9f89918c99a6cca94b51d1. CPU affinity: 0. Dtype explicitly named per workload; row-major inputs.

| Workload / library | Per-call p50 / p95 (us) | Batch-mean p50 / p95 (us) | Rust allocations | Peak additional requested bytes | Process peak RSS (MiB) |
|---|---:|---:|---:|---:|---:|
| cholesky_factor_128_f64/faer | 65.093 / 67.429 | 65.245 / 65.373 | 16 | 148736 | 3.39 |
| cholesky_factor_128_f64/kernix | 276.018 / 281.049 | 275.864 / 278.938 | 1 | 131072 | 3.14 |
| cholesky_factor_128_f64/nalgebra | 130.272 / 134.128 | 130.980 / 131.470 | 1 | 131072 | 2.82 |
| cholesky_factor_128_f64/numpy | 85.684 / 101.730 | 87.686 / 88.328 | unavailable | unavailable | 35.91 |
| cholesky_factor_256_f64/faer | 407.338 / 421.361 | 410.486 / 411.599 | 76 | 592000 | 4.30 |
| cholesky_factor_256_f64/kernix | 766.542 / 777.422 | 766.308 / 770.673 | 49 | 645120 | 4.51 |
| cholesky_factor_256_f64/nalgebra | 1033.901 / 1044.373 | 1042.672 / 1055.885 | 1 | 524288 | 3.58 |
| cholesky_factor_256_f64/numpy | 748.579 / 766.122 | 745.988 / 750.005 | unavailable | unavailable | 38.73 |
| cholesky_factor_512_f64/faer | 2643.417 / 2666.561 | 2651.677 / 2684.423 | 388 | 2297864 | 9.30 |
| cholesky_factor_512_f64/kernix | 4334.758 / 4374.980 | 4333.459 / 4351.114 | 161 | 2269184 | 9.06 |
| cholesky_factor_512_f64/nalgebra | 8698.559 / 8759.416 | 8569.681 / 8677.303 | 1 | 2097152 | 6.58 |
| cholesky_factor_512_f64/numpy | 3850.626 / 3869.608 | 3852.575 / 3861.852 | unavailable | unavailable | 48.27 |
| cholesky_factor_64_f64/faer | 13.223 / 13.255 | 13.261 / 13.293 | 2 | 33280 | 2.85 |
| cholesky_factor_64_f64/kernix | 27.311 / 28.159 | 27.351 / 27.402 | 2 | 33280 | 2.84 |
| cholesky_factor_64_f64/nalgebra | 20.830 / 20.871 | 20.670 / 20.700 | 1 | 32768 | 2.64 |
| cholesky_factor_64_f64/numpy | 17.695 / 18.359 | 17.958 / 18.056 | unavailable | unavailable | 35.12 |
| cholesky_rank_downdate_256_f64/kernix | 150.079 / 159.256 | 153.061 / 153.647 | 3 | 526336 | 4.40 |
| cholesky_rank_downdate_256_f64/nalgebra | 33.982 / 35.038 | 34.316 / 34.653 | 2 | 526336 | 5.14 |
| cholesky_rank_downdate_256_f64/refactor | 767.346 / 777.922 | 767.279 / 771.985 | 49 | 645120 | 5.08 |
| cholesky_rank_update_256_f64/kernix | 151.832 / 160.575 | 154.231 / 154.627 | 3 | 526336 | 4.40 |
| cholesky_rank_update_256_f64/nalgebra | 34.007 / 35.078 | 34.196 / 34.653 | 2 | 526336 | 5.16 |
| cholesky_rank_update_256_f64/refactor | 771.917 / 783.133 | 772.962 / 823.015 | 49 | 645120 | 5.07 |
| cholesky_rank_update_512_f64/kernix | 651.662 / 668.676 | 659.742 / 664.999 | 3 | 2101248 | 8.91 |
| cholesky_rank_update_512_f64/nalgebra | 147.704 / 161.775 | 150.080 / 153.049 | 2 | 2101248 | 12.64 |
| cholesky_rank_update_512_f64/refactor | 4349.557 / 4379.267 | 4333.253 / 4362.868 | 161 | 2269184 | 11.16 |
| create_ones_64x64_f32/candle | 0.344 / 0.360 | 0.355 / 0.364 | 7 | 16696 | 3.23 |
| create_ones_64x64_f32/faer | 0.240 / 0.248 | 0.233 / 0.234 | 1 | 16384 | 2.73 |
| create_ones_64x64_f32/kernix | 0.256 / 0.272 | 0.261 / 0.264 | 1 | 16384 | 2.59 |
| create_ones_64x64_f32/ndarray | 0.208 / 0.216 | 0.216 / 0.220 | 1 | 16384 | 2.57 |
| create_ones_64x64_f32/numpy | 1.544 / 1.584 | 1.605 / 1.611 | unavailable | unavailable | 34.00 |
| create_zeros_64x64_f32/candle | 0.240 / 0.248 | 0.245 / 0.248 | 5 | 16648 | 3.09 |
| create_zeros_64x64_f32/faer | 0.320 / 0.328 | 0.312 / 0.319 | 1 | 16384 | 2.72 |
| create_zeros_64x64_f32/kernix | 0.200 / 0.216 | 0.222 / 0.227 | 1 | 16384 | 2.71 |
| create_zeros_64x64_f32/ndarray | 0.136 / 0.144 | 0.141 / 0.148 | 1 | 16384 | 2.59 |
| create_zeros_64x64_f32/numpy | 0.520 / 0.544 | 0.562 / 0.572 | unavailable | unavailable | 34.01 |
| matmul_128_f32/candle | 85.851 / 88.291 | 85.945 / 86.120 | 10 | 131160 | 3.72 |
| matmul_128_f32/faer | 87.148 / 89.531 | 87.297 / 87.911 | 3 | 196616 | 3.22 |
| matmul_128_f32/kernix | 85.643 / 88.187 | 85.735 / 85.861 | 4 | 131080 | 3.60 |
| matmul_128_f32/nalgebra | 93.811 / 104.115 | 95.399 / 98.833 | 2 | 163840 | 2.92 |
| matmul_128_f32/ndarray | 93.427 / 104.026 | 95.433 / 95.833 | 2 | 163840 | 2.92 |
| matmul_128_f32/numpy | 99.786 / 102.802 | 100.228 / 100.430 | unavailable | unavailable | 35.77 |
| matmul_128_f64/candle | 169.255 / 173.638 | 169.898 / 170.167 | 10 | 262232 | 4.05 |
| matmul_128_f64/faer | 168.974 / 174.597 | 169.480 / 169.607 | 3 | 393224 | 3.60 |
| matmul_128_f64/kernix | 168.207 / 171.142 | 168.687 / 168.922 | 4 | 262152 | 3.85 |
| matmul_128_f64/nalgebra | 209.517 / 222.492 | 209.440 / 209.785 | 2 | 327680 | 3.07 |
| matmul_128_f64/ndarray | 209.459 / 223.810 | 209.340 / 209.910 | 2 | 327680 | 3.08 |
| matmul_128_f64/numpy | 194.213 / 199.837 | 195.351 / 195.835 | unavailable | unavailable | 35.90 |
| matmul_256_f32/candle | 766.159 / 778.151 | 763.721 / 765.711 | 10 | 524376 | 4.55 |
| matmul_256_f32/faer | 676.696 / 682.831 | 674.905 / 681.948 | 3 | 786440 | 4.21 |
| matmul_256_f32/kernix | 662.297 / 667.057 | 659.195 / 660.783 | 4 | 524296 | 4.42 |
| matmul_256_f32/nalgebra | 762.742 / 774.757 | 760.278 / 762.665 | 2 | 589824 | 3.45 |
| matmul_256_f32/ndarray | 758.012 / 770.260 | 758.261 / 760.213 | 2 | 589824 | 3.58 |
| matmul_256_f32/numpy | 724.790 / 733.486 | 724.401 / 727.916 | unavailable | unavailable | 39.88 |
| matmul_256_f64/candle | 1322.686 / 1334.309 | 1326.716 / 1329.105 | 10 | 1048664 | 5.66 |
| matmul_256_f64/faer | 1540.097 / 1554.785 | 1538.527 / 1543.966 | 3 | 1048584 | 5.48 |
| matmul_256_f64/kernix | 1323.125 / 1336.597 | 1321.809 / 1323.589 | 4 | 1048584 | 5.42 |
| matmul_256_f64/nalgebra | 1331.869 / 1344.677 | 1333.255 / 1342.632 | 2 | 1179648 | 4.39 |
| matmul_256_f64/ndarray | 1323.957 / 1337.261 | 1323.566 / 1348.163 | 2 | 1179648 | 4.82 |
| matmul_256_f64/numpy | 1470.099 / 1484.065 | 1472.327 / 1480.261 | unavailable | unavailable | 40.37 |
| matmul_32_f64/candle | 3.440 / 3.472 | 3.455 / 3.460 | 10 | 16976 | 3.38 |
| matmul_32_f64/faer | 3.496 / 3.520 | 3.488 / 3.505 | 3 | 25088 | 2.97 |
| matmul_32_f64/kernix | 3.240 / 3.271 | 3.250 / 3.264 | 3 | 24576 | 3.00 |
| matmul_32_f64/nalgebra | 3.216 / 3.232 | 3.231 / 3.243 | 2 | 24576 | 2.79 |
| matmul_32_f64/ndarray | 3.047 / 3.056 | 3.051 / 3.068 | 2 | 24576 | 2.74 |
| matmul_32_f64/numpy | 8.216 / 8.256 | 8.305 / 8.327 | unavailable | unavailable | 34.67 |
| matmul_64_f32/candle | 11.952 / 11.999 | 11.996 / 12.026 | 10 | 33104 | 3.38 |
| matmul_64_f32/faer | 12.303 / 12.327 | 12.330 / 12.353 | 3 | 49408 | 2.99 |
| matmul_64_f32/kernix | 11.767 / 11.808 | 11.801 / 12.131 | 4 | 33024 | 3.27 |
| matmul_64_f32/nalgebra | 11.263 / 11.288 | 11.298 / 11.308 | 2 | 49152 | 2.76 |
| matmul_64_f32/ndarray | 11.271 / 11.304 | 11.289 / 11.309 | 2 | 49152 | 2.70 |
| matmul_64_f32/numpy | 20.103 / 20.534 | 20.263 / 20.291 | unavailable | unavailable | 34.90 |
| matmul_64_f64/candle | 22.583 / 22.655 | 22.376 / 22.703 | 10 | 65872 | 3.53 |
| matmul_64_f64/faer | 22.639 / 22.735 | 22.678 / 22.746 | 3 | 98560 | 3.05 |
| matmul_64_f64/kernix | 22.159 / 22.246 | 22.214 / 22.226 | 4 | 65792 | 3.32 |
| matmul_64_f64/nalgebra | 22.463 / 22.503 | 22.519 / 22.610 | 2 | 98304 | 2.88 |
| matmul_64_f64/ndarray | 21.783 / 21.847 | 21.847 / 21.862 | 2 | 98304 | 2.85 |
| matmul_64_f64/numpy | 31.031 / 31.534 | 31.208 / 31.286 | unavailable | unavailable | 34.94 |
| matmul_ab_128_f32/kernix | 85.651 / 88.195 | 85.832 / 85.939 | 4 | 131080 | 3.65 |
| matmul_ab_128_f32/numpy | 100.115 / 103.378 | 100.552 / 100.865 | unavailable | unavailable | 35.65 |
| matmul_ab_128_f64/kernix | 168.343 / 173.127 | 169.102 / 169.283 | 4 | 262152 | 3.86 |
| matmul_ab_128_f64/numpy | 195.437 / 201.388 | 196.910 / 197.435 | unavailable | unavailable | 35.91 |
| matmul_batched_4x16x32x16_f32/candle | 2.888 / 2.920 | 2.901 / 2.912 | 25 | 13408 | 3.59 |
| matmul_batched_4x16x32x16_f32/kernix | 2.392 / 2.424 | 2.414 / 2.420 | 7 | 8200 | 2.98 |
| matmul_batched_4x16x32x16_f32/numpy | 5.152 / 5.208 | 5.233 / 5.248 | unavailable | unavailable | 34.65 |
| matmul_batched_4x16x32x16_f64/candle | 4.423 / 4.464 | 4.445 / 4.494 | 25 | 25696 | 3.57 |
| matmul_batched_4x16x32x16_f64/kernix | 3.824 / 3.856 | 3.847 / 3.855 | 7 | 16392 | 3.00 |
| matmul_batched_4x16x32x16_f64/numpy | 12.991 / 13.056 | 13.089 / 13.107 | unavailable | unavailable | 34.68 |
| matmul_sum_backward_32x16x8_f64/candle | 7.216 / 7.408 | 7.226 / 7.248 | 113 | 27980 | 3.78 |
| matmul_sum_backward_32x16x8_f64/kernix | 3.503 / 3.560 | 3.507 / 3.523 | 16 | 18376 | 3.04 |
| matmul_transposed_128_f32/kernix | 92.707 / 101.810 | 94.762 / 95.599 | 3 | 163840 | 3.33 |
| matmul_transposed_128_f32/numpy | 100.266 / 104.081 | 100.899 / 101.361 | unavailable | unavailable | 35.64 |
| matmul_transposed_128_f64/kernix | 210.676 / 229.291 | 213.433 / 215.027 | 3 | 327680 | 3.58 |
| matmul_transposed_128_f64/numpy | 196.260 / 203.427 | 197.655 / 198.139 | unavailable | unavailable | 35.90 |
| min_norm_32x64_rank32_f64/kernix | 96.490 / 99.266 | 96.608 / 97.532 | 20 | 59136 | 3.29 |
| min_norm_32x64_rank32_f64/nalgebra | 218.373 / 220.581 | 218.729 / 218.848 | 15 | 42232 | 3.02 |
| min_norm_32x64_rank32_f64/numpy | 175.525 / 186.732 | 177.975 / 178.983 | unavailable | unavailable | 35.67 |
| min_norm_64x32_rank16_f64/kernix | 72.388 / 74.996 | 72.544 / 74.237 | 20 | 74752 | 3.31 |
| min_norm_64x32_rank16_f64/nalgebra | 116.985 / 119.202 | 117.124 / 117.281 | 15 | 41976 | 3.02 |
| min_norm_64x32_rank16_f64/numpy | 93.627 / 105.370 | 95.275 / 95.780 | unavailable | unavailable | 35.55 |
| parallel_matmul_into_128_f32/kernix | 48.558 / 50.997 | 50.250 / 50.964 | 4 | 65552 | 3.82 |
| parallel_matmul_into_128_f64/kernix | 93.139 / 98.019 | 93.792 / 96.070 | 4 | 131088 | 4.03 |
| parallel_matmul_into_256_f64/kernix | 695.435 / 709.682 | 696.427 / 703.775 | 4 | 524304 | 6.23 |
| parallel_matmul_into_64_f64/kernix | 16.439 / 16.775 | 17.167 / 18.535 | 2 | 98304 | 3.29 |
| pivoted_qr_256x64_f64/faer | 475.285 / 481.044 | 512.179 / 516.401 | 58 | 333312 | 4.08 |
| pivoted_qr_256x64_f64/kernix | 785.744 / 803.478 | 771.817 / 785.989 | 14 | 559104 | 3.61 |
| pivoted_qr_256x64_f64/nalgebra | 1150.615 / 1154.311 | 1150.484 / 1166.003 | 7 | 296960 | 3.36 |
| pivoted_qr_96x32_f64/faer | 62.156 / 64.484 | 62.279 / 62.417 | 32 | 68352 | 3.44 |
| pivoted_qr_96x32_f64/kernix | 88.219 / 90.627 | 88.411 / 88.525 | 14 | 107520 | 3.05 |
| pivoted_qr_96x32_f64/nalgebra | 114.258 / 116.442 | 114.503 / 114.818 | 7 | 58368 | 2.93 |
| qr_1024x256_f64/faer | 20384.433 / 20597.334 | 20391.808 / 20502.971 | 764 | 7135236 | 13.88 |
| qr_1024x256_f64/kernix | 24329.143 / 24545.507 | 24327.127 / 24558.562 | 51 | 7276544 | 15.08 |
| qr_1024x256_f64/nalgebra | 36130.138 / 36270.520 | 36158.241 / 36354.094 | 4 | 4720640 | 9.33 |
| qr_1024x256_f64/numpy | 19828.189 / 19971.551 | 19792.880 / 19921.465 | unavailable | unavailable | 49.22 |
| qr_256x64_f64/faer | 446.936 / 451.776 | 447.665 / 451.838 | 83 | 331776 | 3.92 |
| qr_256x64_f64/kernix | 692.115 / 705.162 | 693.122 / 694.861 | 6 | 426496 | 3.70 |
| qr_256x64_f64/nalgebra | 591.094 / 602.662 | 590.835 / 592.230 | 4 | 295424 | 3.23 |
| qr_256x64_f64/numpy | 568.024 / 586.135 | 563.369 / 568.046 | unavailable | unavailable | 36.36 |
| qr_512x128_f64/faer | 3128.496 / 3148.335 | 3123.543 / 3131.660 | 360 | 1859588 | 5.93 |
| qr_512x128_f64/kernix | 4140.201 / 4185.326 | 4132.540 / 4162.741 | 31 | 2098176 | 6.45 |
| qr_512x128_f64/nalgebra | 4623.818 / 4644.169 | 4618.144 / 4623.317 | 4 | 1180672 | 4.45 |
| qr_512x128_f64/numpy | 4138.682 / 4169.480 | 4147.865 / 4166.838 | unavailable | unavailable | 39.26 |
| qr_96x32_f64/faer | 65.140 / 67.733 | 65.309 / 65.408 | 41 | 67584 | 3.49 |
| qr_96x32_f64/kernix | 73.299 / 75.515 | 73.460 / 74.845 | 6 | 82176 | 3.10 |
| qr_96x32_f64/nalgebra | 61.309 / 63.228 | 61.469 / 61.609 | 4 | 57600 | 2.91 |
| qr_96x32_f64/numpy | 66.372 / 88.571 | 68.056 / 68.429 | unavailable | unavailable | 35.65 |
| softmax_256x64_f64/candle | 110.298 / 114.465 | 110.588 / 110.679 | 37 | 265200 | 4.28 |
| softmax_256x64_f64/kernix | 113.186 / 116.169 | 113.561 / 113.715 | 1 | 131072 | 3.07 |
| softmax_256x64_f64/ndarray | 97.291 / 99.123 | 97.529 / 97.693 | 1 | 131072 | 2.95 |
| softmax_256x64_f64/numpy | 134.585 / 144.744 | 136.348 / 137.051 | unavailable | unavailable | 34.65 |
| solve_cholesky_128x1_f64/faer | 5.888 / 6.264 | 5.992 / 6.009 | 1 | 1024 | 3.49 |
| solve_cholesky_128x1_f64/kernix | 12.704 / 12.823 | 12.444 / 12.599 | 1 | 1024 | 3.08 |
| solve_cholesky_128x1_f64/nalgebra | 5.232 / 5.263 | 5.257 / 5.383 | 1 | 1024 | 3.02 |
| solve_cholesky_256x64_f64/faer | 449.476 / 461.867 | 451.813 / 453.059 | 53 | 327688 | 5.00 |
| solve_cholesky_256x64_f64/kernix | 1718.431 / 1724.623 | 1718.982 / 1724.703 | 1 | 131072 | 4.34 |
| solve_cholesky_256x64_f64/nalgebra | 1293.099 / 1313.129 | 1290.574 / 1299.395 | 1 | 131072 | 4.27 |
| solve_cholesky_512x128_f64/faer | 3194.277 / 3209.963 | 3205.358 / 3214.481 | 205 | 786440 | 9.53 |
| solve_cholesky_512x128_f64/kernix | 13545.170 / 13615.886 | 13541.322 / 13815.180 | 1 | 524288 | 8.53 |
| solve_cholesky_512x128_f64/nalgebra | 12891.520 / 13011.305 | 12902.128 / 13021.817 | 1 | 524288 | 8.78 |
| solve_lu_256x64_f64/faer | 455.592 / 465.824 | 456.660 / 461.663 | 46 | 327688 | 6.14 |
| solve_lu_256x64_f64/kernix | 1654.184 / 1662.168 | 1656.870 / 1659.600 | 2 | 262144 | 4.28 |
| solve_lu_256x64_f64/nalgebra | 1231.346 / 1239.522 | 1231.662 / 1233.859 | 1 | 131072 | 4.21 |
| sparse_auto_512_f64/kernix-dense | 54.277 / 67.293 | 55.064 / 55.708 | 1 | 2097152 | 14.78 |
| sparse_auto_512_f64/numpy | 55.437 / 71.708 | 56.791 / 57.172 | unavailable | unavailable | 60.80 |
| sparse_auto_512_f64/scipy | 755.079 / 779.077 | 750.872 / 757.213 | unavailable | unavailable | 63.06 |
| sparse_matmul_512x32_f32/faer | 65.740 / 67.693 | 64.113 / 65.938 | 1 | 65536 | 4.02 |
| sparse_matmul_512x32_f32/kernix-dense | 381.241 / 395.424 | 384.892 / 385.683 | 3 | 163840 | 5.32 |
| sparse_matmul_512x32_f32/numpy | 510.943 / 523.358 | 512.020 / 513.766 | unavailable | unavailable | 52.35 |
| sparse_matmul_512x32_f32/scipy | 15.407 / 15.767 | 15.583 / 15.645 | unavailable | unavailable | 51.64 |
| sparse_matmul_512x32_f64/faer | 67.549 / 69.741 | 65.928 / 67.746 | 1 | 131072 | 5.16 |
| sparse_matmul_512x32_f64/kernix-dense | 783.359 / 801.486 | 784.810 / 793.127 | 3 | 327680 | 7.65 |
| sparse_matmul_512x32_f64/numpy | 1070.132 / 1082.444 | 1071.078 / 1084.058 | unavailable | unavailable | 53.42 |
| sparse_matmul_512x32_f64/scipy | 23.031 / 24.615 | 23.377 / 23.496 | unavailable | unavailable | 52.63 |
| sparse_matvec_2048_f64/faer | 8.008 / 8.072 | 8.029 / 8.067 | 1 | 16384 | 11.32 |
| sparse_matvec_2048_f64/kernix-dense | 2947.906 / 3010.902 | 2955.677 / 3030.085 | 3 | 155648 | 43.44 |
| sparse_matvec_2048_f64/numpy | 810.426 / 819.921 | 809.139 / 810.262 | unavailable | unavailable | 81.13 |
| sparse_matvec_2048_f64/scipy | 9.791 / 9.944 | 9.909 / 10.188 | unavailable | unavailable | 81.41 |
| sparse_product_256_f64/kernix-dense | 1324.937 / 1340.064 | 1323.382 / 1330.783 | 4 | 1048584 | 6.39 |
| sparse_product_256_f64/numpy | 1476.689 / 1494.520 | 1473.491 / 1481.878 | unavailable | unavailable | 53.86 |
| sparse_product_256_f64/scipy | 47.413 / 56.389 | 47.831 / 48.343 | unavailable | unavailable | 52.68 |
| tensordot_16x32x8_32x8x16_f64/kernix | 46.765 / 48.549 | 46.851 / 47.037 | 9 | 2166 | 2.83 |
| tensordot_16x32x8_32x8x16_f64/numpy | 16.463 / 16.831 | 16.610 / 16.673 | unavailable | unavailable | 34.54 |
| training_backward_2x4x8x4_f64/candle | 21.230 / 21.551 | 21.151 / 21.277 | 476 | 8996 | 4.22 |
| training_backward_2x4x8x4_f64/kernix | 4.168 / 4.280 | 4.204 / 4.233 | 23 | 4224 | 3.27 |
| training_forward_2x4x8x4_f64/candle | 5.352 / 5.479 | 5.363 / 5.389 | 126 | 3912 | 4.12 |
| training_forward_2x4x8x4_f64/kernix | 3.039 / 3.119 | 3.067 / 3.087 | 17 | 1032 | 3.21 |
| training_record_2x4x8x4_f64/candle | 7.615 / 7.783 | 7.456 / 7.664 | 149 | 10712 | 4.13 |
| training_record_2x4x8x4_f64/kernix | 3.808 / 3.872 | 3.829 / 3.837 | 22 | 6832 | 3.21 |
| training_sgd_2x4x8x4_f64/kernix | 0.728 / 0.760 | 0.742 / 0.748 | 5 | 688 | 3.33 |
| training_shared_backward_64x32_f64/candle | 4.991 / 5.048 | 4.956 / 5.029 | 58 | 99996 | 3.73 |
| training_shared_backward_64x32_f64/kernix | 8.383 / 8.447 | 8.411 / 8.542 | 8 | 49768 | 3.25 |

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
