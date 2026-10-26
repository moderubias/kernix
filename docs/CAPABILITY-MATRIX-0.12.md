# Kernix capability matrix — 0.12 series

0.12 preserves existing public signatures and adds owned Cholesky rank-one
updates/downdates. Multiple-RHS solves reuse contiguous rows; larger unpivoted QR
uses compact-WY panels. These are scoped algorithms with explicit allocation,
finite checks and hardware evidence, not a universal speed ranking. Sparse
COO/CSR/CSC, sparse kernels and GPU execution are not implemented.

| Area | Status | Available contract | Remaining boundary |
|---|---|---|---|
| Shape | Implemented | Kahen, Kotei fixed rank, checked scalar/empty/overflow/axis/broadcast/serde semantics | Shapes contain no storage, strides, dtype or device |
| Layout and CPU storage | Implemented | C/F, signed/zero strides, origin, owned/aligned/borrowed memory, checked views | Mutable transforms need proven nonaliasing |
| Tensor construction | Implemented | Fallible tensor!/matrix!/full!/shape!, from_values/from_rows/full/zeros/ones, explicit lower-level constructors | CPU-owned C-order literals; no implicit dtype/device conversion |
| Numerical operations | Substantial | Arithmetic, comparisons/masks, reductions/stats/scans, structural/index/sort operations, contractions | No einsum planner, FFT or complete multidimensional gather/scatter DSL |
| Dense algebra | Partial | f32/f64 pivoted LU, Cholesky, thin Householder QR, solve, full-column-rank least squares, inverse, determinant/slogdet, one-norm conditioning, refinement diagnostics, pivoted QR, rank-truncated minimum-norm/basic solves, O(n²) owned Cholesky rank-one update/downdate | No SVD/pinv/two-norm condition, eigen/Schur, complex or sparse solvers |
| CPU execution | Partial | Checked native rank-two f32/f64 GEMM, signed-stride rank-two inputs, contiguous vector/batched products, tiled convolution, reusable GEMM output, private parallel pools, blocked Cholesky/QR, eligible bounded no-packing kernels and positive-stride panels | Noncontiguous batches and many operations retain serial/reference paths; no blanket speed ranking |
| Numeric policies | Implemented, scoped | Explicit nonfinite/domain rules, sequential/compensated reductions, ordered matmul/conv, portable-math | Finite precision; no universal bitwise reproducibility or exact accumulator |
| Reverse-mode tape | Partial | Explicit first-order graph, finite checks, broadcast arithmetic, selected math, axis sum/mean, reshape/permutation/slice, vector/batched matmul, layer norm, softmax, conv/pool/loss VJPs | No decomposition or general integer gather/scatter VJP; no higher-order differentiation or arbitrary callback recording |
| Dense layer recording | Partial | Linear forward and recording accept vectors and arbitrary leading axes; record composes permute/matmul/add | Parameter shape contracts and first-order CPU tape; no implicit concurrent graph |
| Training/checkpoints | Implemented, scoped | Explicit parameters, seeded Xavier, SGD/Adam/AdamW, clipping, bounded named f32/f64/Adam checkpoint | Single-owner Rc graph/identity; no general shared concurrent trainer or common interchange format |
| Vision/neural inference | Partial | Borrowed RGB8 crop/resize/normalize, grouped NCHW conv, pool, dense, normalization and losses | No decoding/capture, attention, embeddings or general model stack |
| Backend extension | Partial | Opaque Device buffers, explicit transfers/sync, MatmulDevice capability, CPU executor, external-device conformance test; optional explicit single-thread AccelerateExecutor on macOS 15+ for C-order rank-two f32/f64 | No shipped accelerator implementation, async event contract or GPU hardware evidence |
| Dtypes | Partial | Generic scalar traits where documented; most numerical/training APIs focus on f32/f64 | No complete complex, low-precision, quantized, mixed-accumulator/promotion architecture |
| Configuration | Implemented | Optional explicitly loaded strict TOML, local defaults/pools, no ambient math I/O | No implicit global runtime selection |
| Portability | Partial verification | no_std + alloc foundation; Linux x86/ARM, Windows and macOS runtime CI; MSRV 1.86; wasm/i686/aarch64 compile checks | Other-target compilation is not runtime execution; no accelerator claim |
| Performance evidence | Improved | Clean-commit/registry provenance, separate call/batch distributions, fresh-process version pairs, faer/Candle/OpenBLAS comparisons, stage-specific training, allocator probes and process RSS | Shared-machine noise; no pure-kernel or end-to-end model guarantee; validation contracts differ by library |

The [operation-by-operation VJP inventory](AUTOGRAD-COVERAGE.md) classifies the
numerical surface and every Tape recording method. Its JSON and generated table
are checked against source exports in CI. `supported` is scoped to documented
finite inputs and first-order gradients; unsupported operations never silently detach.

See [numerical contracts](NUMERICS.md), [user guide](USER_GUIDE.md),
[0.12 verification](QA-0.12.0.md), and [benchmark methodology](BENCHMARKING.md).
No unspecified complete model's 120-Hz budget follows from these capabilities.
