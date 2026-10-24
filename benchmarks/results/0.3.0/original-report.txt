# Kernix benchmark report: 0.3.0

Measured 2026-10-19T14:42:12Z on 12th Gen Intel(R) Core(TM) i3-1215U.

Rust: rustc 1.99.0 (b940084d7 2026-09-28); NumPy: 2.5.4; one CPU thread; dtype in workload name.

Source: `672e37bfc9b05882096615881d6932082b3a7c46`; working tree dirty: True.

## Comparative CPU results

| Workload | Kernix p50 / p95 (us) | ndarray p50 / p95 (us) | NumPy p50 / p95 (us) |
|---|---:|---:|---:|
| add_65536_f64 | 21.656 / 25.088 | 22.169 / 25.844 | 22.976 / 24.818 |
| conv_1x3x32x32_8x3x3x3_f32 | 1202.783 / 2008.949 | 53.304 / 57.387 | 100.745 / 136.063 |
| matmul_128_f64 | 384.712 / 549.051 | 82.306 / 87.456 | 715.101 / 931.598 |
| matmul_32_f64 | 5.610 / 6.090 | 1.451 / 1.509 | 10.751 / 11.323 |
| matmul_reuse_128_f64 | 339.886 / 455.287 | 85.569 / 90.813 | 697.144 / 770.769 |
| matmul_reuse_32_f64 | 5.222 / 6.918 | 1.420 / 2.571 | 10.623 / 12.158 |
| rgb1080_to224_f32 | 1105.560 / 1253.785 | 691.379 / 716.280 | 4081.478 / 5181.095 |
| softmax_256x64_f64 | 201.896 / 254.720 | 66.690 / 70.915 | 95.585 / 106.062 |
| sum_65536_f64 | 31.040 / 35.041 | 5.696 / 7.661 | 10.432 / 12.625 |

## Method and limits

Operations allocate outputs except scalar sums and explicitly named matmul_reuse cases.
Reuse cases time preallocated output writes in all libraries. Inputs are preallocated and
contiguous. ndarray uses its default matrixmultiply engine (no BLAS feature); NumPy
uses the build recorded in raw.json. BLAS/OMP thread counts are fixed to one.
These are warmed, sequential wall-clock samples on a shared laptop; CPU frequency
and other system load are uncontrolled. p95 is the maximum of 15 batch means,
not a per-request tail-latency guarantee. Different sum orders may round differently.
The ndarray softmax is an explicitly coded stable rowwise comparison, not a native
ndarray softmax API. NumPy timings include Python call overhead. No GPU comparison.
Vision workloads, when present: ndarray uses explicit bilinear/conv loops, while
NumPy uses vectorized bilinear interpolation and sliding-window einsum. No native
convolution/resize API is implied for ndarray. RGB input is 1920x1080 packed u8;
output is 1x3x224x224 f32 (602112 bytes), with no full-resolution float tensor.
No claim that a complete Full HD model reaches 120 FPS follows from these measurements.
raw.json contains all measurements, dependencies and environment metadata.
