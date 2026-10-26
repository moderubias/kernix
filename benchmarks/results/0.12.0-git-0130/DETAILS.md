# Clean CPU evidence: 0.12.0-git-0130

Measured source `7c11b2165b0cb7b523db4d938a36f1396396a138`, tree `cc7ac522fccee7e655adb7ca8d3644d95344fcb1`; dirty before/after: false.
Library source: git v0.12.0 90cbbff60e5d7458ef9f89918c99a6cca94b51d1. CPU affinity: 0. Dtype explicitly named per workload; row-major inputs.

| Workload / library | Per-call p50 / p95 (us) | Batch-mean p50 / p95 (us) | Rust allocations | Peak additional requested bytes | Process peak RSS (MiB) |
|---|---:|---:|---:|---:|---:|
| cholesky_factor_128_f64/faer | 38.593 / 42.530 | 39.181 / 42.346 | 2 | 132096 | 5.09 |
| cholesky_factor_128_f64/kernix | 113.963 / 123.568 | 117.908 / 123.834 | 1 | 131072 | 4.59 |
| cholesky_factor_128_f64/nalgebra | 100.243 / 114.547 | 99.520 / 120.587 | 1 | 131072 | 4.19 |
| cholesky_factor_128_f64/numpy | 72.999 / 79.188 | 73.364 / 74.091 | unavailable | unavailable | 37.63 |
| cholesky_factor_256_f64/faer | 208.438 / 219.393 | 213.798 / 227.725 | 2 | 526336 | 5.86 |
| cholesky_factor_256_f64/kernix | 506.734 / 537.680 | 511.785 / 520.437 | 49 | 645120 | 6.05 |
| cholesky_factor_256_f64/nalgebra | 690.745 / 706.056 | 684.155 / 695.914 | 1 | 524288 | 5.00 |
| cholesky_factor_256_f64/numpy | 530.914 / 559.632 | 528.263 / 570.626 | unavailable | unavailable | 41.65 |
| cholesky_factor_512_f64/faer | 1620.073 / 1795.341 | 1674.671 / 1809.862 | 2 | 2101248 | 9.38 |
| cholesky_factor_512_f64/kernix | 3081.611 / 3356.157 | 3065.559 / 3444.928 | 161 | 2269184 | 10.69 |
| cholesky_factor_512_f64/nalgebra | 5827.295 / 6467.950 | 5921.382 / 6176.960 | 1 | 2097152 | 7.98 |
| cholesky_factor_512_f64/numpy | 3303.614 / 5008.344 | 3628.041 / 6258.008 | unavailable | unavailable | 53.53 |
| cholesky_factor_64_f64/faer | 8.612 / 8.754 | 8.382 / 11.391 | 2 | 33280 | 4.68 |
| cholesky_factor_64_f64/kernix | 20.570 / 23.433 | 21.348 / 21.960 | 2 | 33280 | 4.41 |
| cholesky_factor_64_f64/nalgebra | 14.425 / 14.483 | 14.691 / 15.259 | 1 | 32768 | 3.98 |
| cholesky_factor_64_f64/numpy | 11.142 / 12.358 | 11.572 / 12.510 | unavailable | unavailable | 36.95 |
| cholesky_rank_downdate_256_f64/kernix | 120.339 / 130.741 | 122.089 / 125.732 | 3 | 526336 | 5.96 |
| cholesky_rank_downdate_256_f64/nalgebra | 24.519 / 25.691 | 26.248 / 27.641 | 2 | 526336 | 6.46 |
| cholesky_rank_downdate_256_f64/refactor | 537.424 / 555.709 | 536.224 / 543.224 | 49 | 645120 | 6.72 |
| cholesky_rank_update_256_f64/kernix | 121.709 / 132.114 | 124.813 / 127.638 | 3 | 526336 | 6.00 |
| cholesky_rank_update_256_f64/nalgebra | 25.787 / 26.869 | 26.467 / 27.984 | 2 | 526336 | 6.48 |
| cholesky_rank_update_256_f64/refactor | 527.645 / 558.928 | 530.078 / 565.264 | 49 | 645120 | 6.74 |
| cholesky_rank_update_512_f64/kernix | 759.186 / 808.231 | 735.242 / 790.634 | 3 | 2101248 | 10.48 |
| cholesky_rank_update_512_f64/nalgebra | 151.980 / 179.864 | 156.859 / 183.677 | 2 | 2101248 | 14.05 |
| cholesky_rank_update_512_f64/refactor | 2961.108 / 3258.511 | 2952.182 / 5739.237 | 161 | 2269184 | 12.68 |
| create_ones_64x64_f32/candle | 0.255 / 0.268 | 0.256 / 0.262 | 7 | 16696 | 4.83 |
| create_ones_64x64_f32/faer | 0.199 / 0.203 | 0.186 / 0.212 | 1 | 16384 | 4.37 |
| create_ones_64x64_f32/kernix | 0.184 / 0.202 | 0.198 / 0.212 | 1 | 16384 | 3.65 |
| create_ones_64x64_f32/ndarray | 0.147 / 0.149 | 0.149 / 0.155 | 1 | 16384 | 4.03 |
| create_ones_64x64_f32/numpy | 1.342 / 2.781 | 1.388 / 1.417 | unavailable | unavailable | 35.97 |
| create_zeros_64x64_f32/candle | 0.157 / 0.159 | 0.161 / 0.172 | 5 | 16648 | 4.50 |
| create_zeros_64x64_f32/faer | 0.164 / 0.175 | 0.192 / 0.195 | 1 | 16384 | 4.50 |
| create_zeros_64x64_f32/kernix | 0.146 / 0.147 | 0.147 / 0.158 | 1 | 16384 | 3.97 |
| create_zeros_64x64_f32/ndarray | 0.092 / 0.093 | 0.093 / 0.099 | 1 | 16384 | 4.11 |
| create_zeros_64x64_f32/numpy | 0.402 / 0.435 | 0.436 / 0.461 | unavailable | unavailable | 36.01 |
| matmul_128_f32/candle | 40.092 / 41.610 | 40.325 / 41.926 | 9 | 65816 | 5.37 |
| matmul_128_f32/faer | 43.291 / 46.521 | 43.849 / 44.705 | 1 | 65536 | 4.83 |
| matmul_128_f32/kernix | 39.675 / 41.564 | 40.612 / 44.067 | 3 | 65544 | 4.58 |
| matmul_128_f32/nalgebra | 47.902 / 56.591 | 50.207 / 53.244 | 2 | 163840 | 4.30 |
| matmul_128_f32/ndarray | 51.522 / 58.055 | 50.594 / 52.442 | 2 | 163840 | 4.45 |
| matmul_128_f32/numpy | 40.024 / 43.615 | 41.387 / 42.103 | unavailable | unavailable | 37.61 |
| matmul_128_f64/candle | 75.702 / 80.266 | 77.155 / 78.532 | 9 | 131352 | 5.43 |
| matmul_128_f64/faer | 95.298 / 99.367 | 93.916 / 99.691 | 1 | 131072 | 5.02 |
| matmul_128_f64/kernix | 75.604 / 79.653 | 76.239 / 77.099 | 3 | 131080 | 5.02 |
| matmul_128_f64/nalgebra | 114.111 / 128.296 | 116.507 / 120.351 | 2 | 327680 | 4.12 |
| matmul_128_f64/ndarray | 117.788 / 125.771 | 119.623 / 124.139 | 2 | 327680 | 4.70 |
| matmul_128_f64/numpy | 81.491 / 89.167 | 82.816 / 87.464 | unavailable | unavailable | 37.76 |
| matmul_256_f32/candle | 291.844 / 307.555 | 289.704 / 294.964 | 9 | 262424 | 5.99 |
| matmul_256_f32/faer | 345.546 / 355.576 | 350.590 / 399.421 | 1 | 262144 | 5.43 |
| matmul_256_f32/kernix | 294.171 / 302.507 | 296.828 / 309.540 | 3 | 262152 | 5.50 |
| matmul_256_f32/nalgebra | 510.560 / 569.774 | 402.026 / 423.181 | 2 | 589824 | 4.72 |
| matmul_256_f32/ndarray | 395.069 / 430.988 | 401.770 / 417.827 | 2 | 589824 | 5.17 |
| matmul_256_f32/numpy | 293.135 / 302.236 | 294.852 / 300.287 | unavailable | unavailable | 41.77 |
| matmul_256_f64/candle | 577.512 / 635.850 | 582.883 / 612.233 | 9 | 524568 | 7.00 |
| matmul_256_f64/faer | 747.438 / 770.990 | 753.514 / 769.368 | 1 | 524288 | 6.00 |
| matmul_256_f64/kernix | 578.704 / 611.138 | 589.286 / 612.844 | 3 | 524296 | 6.21 |
| matmul_256_f64/nalgebra | 665.307 / 680.277 | 667.694 / 686.169 | 2 | 1179648 | 5.84 |
| matmul_256_f64/ndarray | 668.339 / 696.501 | 679.922 / 873.047 | 2 | 1179648 | 6.09 |
| matmul_256_f64/numpy | 658.499 / 700.631 | 668.548 / 672.962 | unavailable | unavailable | 42.72 |
| matmul_32_f64/candle | 1.513 / 1.547 | 1.592 / 1.770 | 9 | 8592 | 4.98 |
| matmul_32_f64/faer | 1.662 / 1.739 | 1.711 / 1.762 | 1 | 8192 | 4.60 |
| matmul_32_f64/kernix | 1.611 / 1.635 | 1.658 / 1.698 | 3 | 24576 | 4.36 |
| matmul_32_f64/nalgebra | 1.535 / 1.594 | 1.557 / 1.594 | 2 | 24576 | 4.16 |
| matmul_32_f64/ndarray | 1.425 / 1.446 | 1.497 / 1.545 | 2 | 24576 | 4.18 |
| matmul_32_f64/numpy | 2.736 / 2.786 | 2.868 / 3.682 | unavailable | unavailable | 36.55 |
| matmul_64_f32/candle | 4.955 / 5.112 | 5.134 / 5.253 | 9 | 16664 | 5.25 |
| matmul_64_f32/faer | 4.971 / 5.039 | 5.158 / 5.229 | 1 | 16384 | 4.43 |
| matmul_64_f32/kernix | 5.110 / 5.208 | 5.243 / 5.427 | 3 | 16544 | 4.79 |
| matmul_64_f32/nalgebra | 6.076 / 6.122 | 6.115 / 6.146 | 2 | 49152 | 4.23 |
| matmul_64_f32/ndarray | 5.698 / 6.147 | 5.943 / 6.050 | 2 | 49152 | 4.24 |
| matmul_64_f32/numpy | 6.074 / 6.259 | 6.315 / 8.295 | unavailable | unavailable | 36.80 |
| matmul_64_f64/candle | 10.447 / 11.917 | 10.962 / 11.278 | 9 | 33048 | 5.04 |
| matmul_64_f64/faer | 11.092 / 11.333 | 11.304 / 11.666 | 1 | 32768 | 4.73 |
| matmul_64_f64/kernix | 9.836 / 10.112 | 10.319 / 11.426 | 3 | 32928 | 4.65 |
| matmul_64_f64/nalgebra | 11.542 / 11.632 | 11.674 / 11.957 | 2 | 98304 | 4.33 |
| matmul_64_f64/ndarray | 11.248 / 12.926 | 11.406 / 11.824 | 2 | 98304 | 3.98 |
| matmul_64_f64/numpy | 12.287 / 13.666 | 12.746 / 13.195 | unavailable | unavailable | 36.87 |
| matmul_ab_128_f32/kernix | 35.942 / 38.148 | 37.086 / 37.359 | 3 | 65544 | 4.92 |
| matmul_ab_128_f32/numpy | 38.065 / 41.453 | 40.009 / 41.370 | unavailable | unavailable | 37.42 |
| matmul_ab_128_f64/kernix | 74.251 / 79.569 | 75.457 / 106.386 | 3 | 131080 | 5.11 |
| matmul_ab_128_f64/numpy | 78.009 / 81.789 | 80.554 / 81.369 | unavailable | unavailable | 37.85 |
| matmul_batched_4x16x32x16_f32/candle | 1.545 / 1.608 | 1.564 / 1.590 | 25 | 13216 | 5.73 |
| matmul_batched_4x16x32x16_f32/kernix | 1.057 / 1.132 | 1.078 / 1.090 | 3 | 4104 | 4.45 |
| matmul_batched_4x16x32x16_f32/numpy | 2.262 / 2.332 | 2.251 / 2.514 | unavailable | unavailable | 36.47 |
| matmul_batched_4x16x32x16_f64/candle | 2.049 / 2.115 | 2.135 / 2.232 | 25 | 25504 | 5.44 |
| matmul_batched_4x16x32x16_f64/kernix | 1.873 / 1.880 | 1.897 / 1.939 | 3 | 8200 | 4.45 |
| matmul_batched_4x16x32x16_f64/numpy | 3.054 / 3.125 | 3.240 / 3.313 | unavailable | unavailable | 36.67 |
| matmul_sum_backward_32x16x8_f64/candle | 4.397 / 4.884 | 4.428 / 4.776 | 113 | 27988 | 5.43 |
| matmul_sum_backward_32x16x8_f64/kernix | 2.369 / 2.431 | 2.453 / 2.536 | 15 | 18376 | 4.54 |
| matmul_transposed_128_f32/kernix | 49.227 / 62.834 | 51.063 / 55.105 | 3 | 163840 | 4.92 |
| matmul_transposed_128_f32/numpy | 37.672 / 41.581 | 40.165 / 80.805 | unavailable | unavailable | 37.44 |
| matmul_transposed_128_f64/kernix | 124.804 / 131.374 | 121.597 / 127.836 | 3 | 327680 | 5.14 |
| matmul_transposed_128_f64/numpy | 84.445 / 97.864 | 85.573 / 88.499 | unavailable | unavailable | 37.75 |
| min_norm_32x64_rank32_f64/kernix | 61.999 / 65.504 | 62.665 / 65.591 | 20 | 59136 | 4.83 |
| min_norm_32x64_rank32_f64/nalgebra | 132.504 / 139.762 | 132.899 / 134.284 | 15 | 42232 | 4.32 |
| min_norm_32x64_rank32_f64/numpy | 132.454 / 141.256 | 133.403 / 135.339 | unavailable | unavailable | 37.54 |
| min_norm_64x32_rank16_f64/kernix | 46.498 / 51.052 | 46.222 / 47.903 | 20 | 74752 | 4.89 |
| min_norm_64x32_rank16_f64/nalgebra | 76.373 / 91.216 | 77.353 / 82.320 | 15 | 41976 | 4.35 |
| min_norm_64x32_rank16_f64/numpy | 67.917 / 73.090 | 67.399 / 71.144 | unavailable | unavailable | 37.58 |
| parallel_matmul_into_128_f32/kernix | 39.928 / 43.246 | 22.561 / 58.999 | 2 | 16 | 5.27 |
| parallel_matmul_into_128_f64/kernix | 40.966 / 44.654 | 44.092 / 88.213 | 2 | 8 | 5.51 |
| parallel_matmul_into_256_f64/kernix | 292.801 / 322.131 | 295.139 / 445.546 | 2 | 16 | 7.16 |
| parallel_matmul_into_64_f64/kernix | 11.917 / 20.997 | 12.119 / 15.556 | 2 | 49152 | 4.69 |
| pivoted_qr_256x64_f64/faer | 494.897 / 542.976 | 502.011 / 518.319 | 10 | 333312 | 5.40 |
| pivoted_qr_256x64_f64/kernix | 600.564 / 633.046 | 601.611 / 611.651 | 14 | 559104 | 4.98 |
| pivoted_qr_256x64_f64/nalgebra | 838.764 / 903.223 | 800.693 / 815.438 | 7 | 296960 | 4.76 |
| pivoted_qr_96x32_f64/faer | 77.223 / 83.286 | 81.059 / 92.126 | 10 | 68352 | 5.05 |
| pivoted_qr_96x32_f64/kernix | 57.371 / 62.199 | 58.545 / 59.541 | 14 | 107520 | 4.68 |
| pivoted_qr_96x32_f64/nalgebra | 75.452 / 82.050 | 77.376 / 78.808 | 7 | 58368 | 4.22 |
| qr_1024x256_f64/faer | 12763.563 / 14042.113 | 13084.087 / 14307.396 | 7 | 5308416 | 12.06 |
| qr_1024x256_f64/kernix | 15264.449 / 17475.654 | 15623.750 / 17172.148 | 82 | 7145480 | 17.55 |
| qr_1024x256_f64/nalgebra | 27508.871 / 28640.715 | 27707.888 / 28239.676 | 4 | 4720640 | 10.51 |
| qr_1024x256_f64/numpy | 13352.622 / 14958.522 | 13273.363 / 14812.540 | unavailable | unavailable | 53.08 |
| qr_256x64_f64/faer | 421.185 / 441.288 | 413.805 / 612.053 | 7 | 331776 | 5.22 |
| qr_256x64_f64/kernix | 495.985 / 520.125 | 494.891 / 536.953 | 6 | 426496 | 4.82 |
| qr_256x64_f64/nalgebra | 384.992 / 396.894 | 383.713 / 401.089 | 4 | 295424 | 4.43 |
| qr_256x64_f64/numpy | 357.744 / 372.502 | 356.609 / 513.535 | unavailable | unavailable | 38.54 |
| qr_512x128_f64/faer | 2082.605 / 3211.381 | 2100.011 / 2163.381 | 7 | 1343488 | 6.70 |
| qr_512x128_f64/kernix | 2682.241 / 3348.879 | 2704.547 / 2818.825 | 49 | 2294792 | 8.30 |
| qr_512x128_f64/nalgebra | 3066.934 / 3315.421 | 3082.398 / 3236.110 | 4 | 1180672 | 5.62 |
| qr_512x128_f64/numpy | 2454.159 / 2613.276 | 2443.493 / 2633.986 | unavailable | unavailable | 41.79 |
| qr_96x32_f64/faer | 67.808 / 72.484 | 68.668 / 71.119 | 7 | 67584 | 5.18 |
| qr_96x32_f64/kernix | 48.770 / 53.653 | 49.922 / 51.355 | 6 | 82176 | 3.95 |
| qr_96x32_f64/nalgebra | 37.835 / 40.519 | 38.078 / 38.577 | 4 | 57600 | 4.18 |
| qr_96x32_f64/numpy | 41.910 / 60.757 | 42.611 / 46.020 | unavailable | unavailable | 37.73 |
| softmax_256x64_f64/candle | 84.045 / 88.681 | 83.019 / 85.687 | 37 | 265200 | 6.07 |
| softmax_256x64_f64/kernix | 89.846 / 94.602 | 89.431 / 119.442 | 1 | 131072 | 4.50 |
| softmax_256x64_f64/ndarray | 74.404 / 79.572 | 74.718 / 76.360 | 1 | 131072 | 4.38 |
| softmax_256x64_f64/numpy | 101.847 / 112.456 | 103.156 / 111.742 | unavailable | unavailable | 36.48 |
| solve_cholesky_128x1_f64/faer | 3.900 / 5.051 | 4.053 / 4.120 | 1 | 1024 | 5.30 |
| solve_cholesky_128x1_f64/kernix | 10.548 / 10.848 | 10.761 / 14.268 | 1 | 1024 | 4.58 |
| solve_cholesky_128x1_f64/nalgebra | 4.103 / 4.207 | 4.182 / 4.224 | 1 | 1024 | 4.52 |
| solve_cholesky_256x64_f64/faer | 227.765 / 237.020 | 224.996 / 230.473 | 1 | 131072 | 6.85 |
| solve_cholesky_256x64_f64/kernix | 1093.900 / 1151.923 | 1091.275 / 1118.908 | 1 | 131072 | 5.90 |
| solve_cholesky_256x64_f64/nalgebra | 883.708 / 936.679 | 884.718 / 924.896 | 1 | 131072 | 5.75 |
| solve_cholesky_512x128_f64/faer | 1561.877 / 1665.148 | 1585.136 / 2567.160 | 1 | 524288 | 11.02 |
| solve_cholesky_512x128_f64/kernix | 8968.499 / 9730.846 | 9218.882 / 13199.541 | 1 | 524288 | 10.06 |
| solve_cholesky_512x128_f64/nalgebra | 7264.539 / 7851.198 | 7257.440 / 8075.569 | 1 | 524288 | 10.27 |
| solve_lu_256x64_f64/faer | 241.498 / 255.535 | 241.991 / 247.206 | 2 | 262144 | 6.66 |
| solve_lu_256x64_f64/kernix | 1079.062 / 1179.013 | 1100.160 / 1453.127 | 2 | 262144 | 5.65 |
| solve_lu_256x64_f64/nalgebra | 846.889 / 897.356 | 842.227 / 878.555 | 1 | 131072 | 5.63 |
| sparse_auto_512_f64/kernix-dense | 79.129 / 86.402 | 80.928 / 84.253 | 1 | 2097152 | 15.98 |
| sparse_auto_512_f64/numpy | 79.647 / 90.915 | 82.744 / 89.396 | unavailable | unavailable | 63.01 |
| sparse_auto_512_f64/scipy | 873.223 / 968.029 | 876.276 / 904.452 | unavailable | unavailable | 62.92 |
| sparse_matmul_512x32_f32/faer | 39.736 / 43.318 | 39.366 / 40.122 | 1 | 65536 | 5.66 |
| sparse_matmul_512x32_f32/kernix-dense | 206.305 / 234.870 | 210.318 / 220.899 | 3 | 163840 | 6.88 |
| sparse_matmul_512x32_f32/numpy | 172.879 / 186.683 | 172.990 / 178.742 | unavailable | unavailable | 54.32 |
| sparse_matmul_512x32_f32/scipy | 14.246 / 15.429 | 15.126 / 17.248 | unavailable | unavailable | 53.82 |
| sparse_matmul_512x32_f64/faer | 42.102 / 47.066 | 42.581 / 43.794 | 1 | 131072 | 6.75 |
| sparse_matmul_512x32_f64/kernix-dense | 405.344 / 439.350 | 408.857 / 430.320 | 3 | 327680 | 9.02 |
| sparse_matmul_512x32_f64/numpy | 381.985 / 404.572 | 388.373 / 398.597 | unavailable | unavailable | 55.82 |
| sparse_matmul_512x32_f64/scipy | 20.475 / 24.540 | 20.801 / 21.890 | unavailable | unavailable | 54.49 |
| sparse_matvec_2048_f64/faer | 4.948 / 4.963 | 4.959 / 5.052 | 1 | 16384 | 13.02 |
| sparse_matvec_2048_f64/kernix-dense | 3230.535 / 4711.761 | 3336.953 / 3953.524 | 3 | 155648 | 44.97 |
| sparse_matvec_2048_f64/numpy | 384.024 / 609.870 | 376.602 / 464.735 | unavailable | unavailable | 60.70 |
| sparse_matvec_2048_f64/scipy | 7.133 / 7.382 | 7.298 / 7.431 | unavailable | unavailable | 61.02 |
| sparse_product_256_f64/kernix-dense | 574.436 / 937.731 | 571.791 / 583.865 | 3 | 524296 | 7.87 |
| sparse_product_256_f64/numpy | 603.759 / 652.491 | 609.221 / 629.271 | unavailable | unavailable | 56.24 |
| sparse_product_256_f64/scipy | 43.248 / 46.176 | 43.512 / 44.281 | unavailable | unavailable | 54.62 |
| tensordot_16x32x8_32x8x16_f64/kernix | 37.974 / 39.289 | 37.419 / 38.488 | 9 | 2166 | 4.18 |
| tensordot_16x32x8_32x8x16_f64/numpy | 8.023 / 8.850 | 8.257 / 8.547 | unavailable | unavailable | 36.50 |
| training_backward_2x4x8x4_f64/candle | 14.785 / 15.121 | 15.081 / 16.363 | 472 | 8332 | 5.91 |
| training_backward_2x4x8x4_f64/kernix | 3.337 / 3.416 | 3.490 / 3.630 | 23 | 4224 | 4.86 |
| training_forward_2x4x8x4_f64/candle | 3.702 / 3.847 | 3.791 / 3.963 | 126 | 3912 | 5.53 |
| training_forward_2x4x8x4_f64/kernix | 2.346 / 2.426 | 2.302 / 2.471 | 17 | 1032 | 5.01 |
| training_record_2x4x8x4_f64/candle | 5.252 / 6.061 | 5.243 / 5.278 | 149 | 10712 | 6.45 |
| training_record_2x4x8x4_f64/kernix | 2.665 / 2.726 | 2.766 / 2.824 | 22 | 6832 | 4.66 |
| training_sgd_2x4x8x4_f64/kernix | 0.527 / 0.558 | 0.538 / 0.553 | 5 | 688 | 5.16 |
| training_shared_backward_64x32_f64/candle | 3.934 / 4.118 | 4.006 / 4.441 | 58 | 100004 | 5.47 |
| training_shared_backward_64x32_f64/kernix | 6.630 / 6.789 | 6.301 / 6.603 | 8 | 49768 | 4.69 |

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
