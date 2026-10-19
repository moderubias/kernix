# Retained CPU benchmark history

All values below are median batch time in microseconds. See each report for the
competitors, exact workload, allocation boundaries and numerical API differences.
Earlier runs were unpinned and lacked source snapshots; 0.4 pins one CPU and includes
an exact source archive. Frequency and other laptop activity are uncontrolled.
These trends cannot establish a controlled speedup or any complete-model FPS.

| Version | GEMM 128 f64 | Sum 65536 f64 | Softmax 256x64 f64 | Conv f32 | RGB 1080p to 224 f32 |
|---|---:|---:|---:|---:|---:|
| [0.1.1](0.1.1/README.md) | 3311.847 | 37.352 | 765.891 | — | — |
| [0.2.0](0.2.0/README.md) | 382.258 | 33.711 | 221.617 | — | — |
| [0.3.0](0.3.0/README.md) | 384.712 | 31.040 | 201.896 | 1202.783 | 1105.560 |
| [0.4.0](0.4.0/README.md) | 86.102 | 5.538 | 77.015 | 11.361 | 438.953 |
| [baseline-0.1.0](baseline-0.1.0/README.md) | 3029.011 | 31.027 | 397.788 | — | — |

Kernix and ndarray now share matrixmultiply for native GEMM. Native FMA/order can
change last bits; ordered matmul, reference convolution and compensated sums are
explicit choices. QR/Cholesky and reverse-mode graph overhead remain optimization
work; the current report preserves these unfavorable comparisons too.
