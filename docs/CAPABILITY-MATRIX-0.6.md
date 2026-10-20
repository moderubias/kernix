# Kernix capability matrix — 0.6 series

Version 0.6 adds numeric identity constructors, executable workflow checks and
clean evidence tooling to the 0.5 foundation. It adds no new derivatives. `Implemented` means
an available documented API; `partial` identifies its boundary. Compilation alone
is not hardware runtime validation. The [0.5 snapshot](CAPABILITY-MATRIX-0.5.md) and archived 0.4 table are historical.

| Area | Status | Available contract | Remaining boundary |
|---|---|---|---|
| Shape | Implemented | Kahen, Kotei fixed rank, checked scalar/empty/overflow/axis/broadcast/serde semantics | Shapes contain no storage, strides, dtype or device |
| Layout and CPU storage | Implemented | C/F, signed/zero strides, origin, owned/aligned/borrowed memory, checked views | Mutable transforms need proven nonaliasing |
| Tensor construction | Implemented | Fallible tensor!/matrix!/full!/shape!, from_values/from_rows/full/zeros/ones, explicit lower-level constructors | CPU-owned C-order literals; no implicit dtype/device conversion |
| Numerical operations | Substantial | Arithmetic, comparisons/masks, reductions/stats/scans, structural/index/sort operations, contractions | No einsum planner, FFT or complete multidimensional gather/scatter DSL |
| Dense algebra | Partial | f32/f64 pivoted LU, Cholesky, thin Householder QR, solve, full-column-rank least squares, inverse, determinant/slogdet | No pivoted QR, SVD/pinv/condition estimate, rank-deficient/underdetermined solve, eigen/Schur, complex or sparse solvers |
| CPU execution | Partial | Checked native rank-two f32/f64 GEMM, signed-stride inputs, tiled convolution, reusable GEMM output, private parallel pools | Vector/batched products and many operations retain serial/reference paths; no blanket speed ranking |
| Numeric policies | Implemented, scoped | Explicit nonfinite/domain rules, sequential/compensated reductions, ordered matmul/conv, portable-math | Finite precision; no universal bitwise reproducibility or exact accumulator |
| Reverse-mode tape | Partial | Explicit first-order graph, finite checks, broadcast arithmetic, selected math/reductions/transforms, rank-two matmul, softmax, conv/pool/loss VJPs | No batched/vector matmul, layer-norm or decomposition VJP; no higher-order differentiation or arbitrary callback recording |
| Dense layer recording | Partial | Linear forward accepts leading axes; record composes permute/matmul/add | record requires rank-two input |
| Training/checkpoints | Implemented, scoped | Explicit parameters, seeded Xavier, SGD/Adam/AdamW, clipping, bounded named f32/f64/Adam checkpoint | Single-owner Rc graph/identity; no general shared concurrent trainer or common interchange format |
| Vision/neural inference | Partial | Borrowed RGB8 crop/resize/normalize, grouped NCHW conv, pool, dense, normalization and losses | No decoding/capture, attention, embeddings or general model stack |
| Backend extension | Partial | Opaque Device buffers, explicit transfers/sync, MatmulDevice capability, CPU executor, external-device conformance test | No shipped accelerator implementation, async event contract or GPU hardware evidence |
| Dtypes | Partial | Generic scalar traits where documented; most numerical/training APIs focus on f32/f64 | No complete complex, low-precision, quantized, mixed-accumulator/promotion architecture |
| Configuration | Implemented | Optional explicitly loaded strict TOML, local defaults/pools, no ambient math I/O | No implicit global runtime selection |
| Portability | Partial verification | no_std + alloc foundation; Linux/Windows/macOS source CI; MSRV 1.86; wasm/i686/aarch64 compile checks | Other-target compilation is not runtime execution; no accelerator claim |
| Performance evidence | Improved | Clean-commit/registry provenance, separate call/batch distributions, faer/Candle/OpenBLAS comparisons, allocator probes and process RSS | Shared-machine noise; no pure-kernel or end-to-end model guarantee; validation contracts differ by library |

The [operation-by-operation VJP inventory](AUTOGRAD-COVERAGE.md) classifies the
numerical surface and every Tape recording method. Its JSON and generated table
are checked against source exports in CI. `supported` is scoped to documented
finite inputs and first-order gradients; unsupported operations never silently detach.

See [numerical contracts](NUMERICS.md), [user guide](USER_GUIDE.md),
[0.6.0 release QA](QA-0.6.0.md), and [benchmark methodology](BENCHMARKING.md).
No unspecified complete model's 120-Hz budget follows from these capabilities.
