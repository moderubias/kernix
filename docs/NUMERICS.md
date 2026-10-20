# Numerical contracts and performance choices

Kernix validates integer shape/layout metadata exactly. Floating tensors approximate
real arithmetic with the caller's dtype. No library can guarantee exact real results
or 100% accuracy for every numerical problem. Errors, tolerances and algorithms are
part of the API contract; reducing test rigor to hide a performance regression is
not an optimization.

## Shapes, indexing and empty values

Dimensions are usize and cannot be negative. `[]` is a scalar with one element;
any zero dimension makes element count zero. Constructors check products with a
zero-first policy. Removing a zero axis can reveal overflow and must revalidate.
Broadcasting takes the non-singleton dimension, so `(0,1)` broadcasts to zero,
not one. `[2,1]` and `[1,3]` broadcast to `[2,3]`, whose element count exceeds both
inputs: the proposed maximum-input-numel property would be incorrect.

Negative axes are normalized with checked bounds. `-rank` is valid for nonzero
rank; more-negative values and scalar indexing fail. Shape operations do not infer
strides. Layout validates affine extents, origins and representability. Host storage
checks alignment and ownership; mutable views/in-place operations require disjointness.

## Arithmetic and reductions

No implicit precision promotion occurs. Native integer arithmetic follows Rust's
scalar overflow behavior; use checked operations for explicit errors. Floating
maps follow documented IEEE domains (for example a negative logarithm is NaN).
Finite-only APIs such as softmax, decompositions and training reject nonfinite data.
Extrema, sorting and pooling document NaN and tie rules rather than guessing.

`sum_all` uses a pairwise contiguous reduction with short unrolled blocks; arbitrary
strides use logical-order traversal. `sum_sequential_all` fixes logical C traversal;
`sum_compensated_all` applies Neumaier compensation to finite inputs and rejects
unrepresentable intermediates. Compensation can recover cancellation such as
`[1e16,1,-1e16]`, but still does not make every sum exact. An exact accumulator or
higher-precision type is future work.

Native f32/f64 rank-two and contiguous vector/batched matmul may use packed SIMD/FMA kernels. Reassociation/FMA
can differ in final bits from increasing-k scalar reference arithmetic. Choose
`matmul_reference` or executor `ordered_matmul=true` when that order is required.
This controls accumulation order rather than guaranteeing architecture-independent
IEEE behavior. Optimized contiguous convolution packs bounded output tiles and uses
native GEMM; `neural::conv2d_reference` retains ordered scalar accumulation.
Noncontiguous batches use the reference path. Convolution input-tile packing uses
at most 128 spatial positions rather than the full image. Native GEMM workspace
is separate; the dense f64 engine can retain a thread-local slab as described below.

With std, elementary float functions use the platform implementation by default.
`portable-math` selects Rust libm even with std; no_std uses libm. All-features tests
therefore exercise portable math; default-feature tests separately exercise std math.
Neither feature enables compiler fast-math. Each backend needs error-bound tests,
not just bitwise comparisons with a different arithmetic order.

## Stable operations and linear systems

Softmax subtracts the row maximum, validates logits and caches exponentials for
normalization. Temperature softmax handles differences before unsafe scaling where
possible. Stable losses avoid unnecessary overflow in logit differences and means.
Variance uses Welford accumulation; L2 norm uses scaled sums of squares.
Layer norm shares one Welford mean/variance state and combines their standard
deviation with sqrt(epsilon) using scaling. Nonfinite/unrepresentable statistics
are errors, including variance overflow that could previously yield finite zeros.
Its explicit input VJP and Tape recording recompute population statistics; trainable
affine scale/bias compose through ordinary broadcast operations. Epsilon is fixed.
Contiguous nonempty groups fuse normalization/VJP loops while retaining the same
scalar recurrence and finite checks; the general indexed path remains available. Extreme
representability limits still produce documented errors or IEEE results.

Pivoted LU solves square systems and defaults to a scale-relative near-singular
pivot cutoff. Cholesky checks symmetry and positive pivots without adding jitter.
Householder QR avoids normal equations for least squares. QR uses four independent
accumulation lanes for reflector dot products, so last bits can differ from 0.4.
Overflowing regrouped dots retry ordered accumulation; overflowing reflector
products use a split update to retain representable extreme results. It skips
identity columns that cannot change while materializing Q. QR rejects deficient or
underdetermined solves; no hidden pseudo-inverse is substituted. Determinant products
may underflow; `slogdet` avoids directly multiplying pivots. Residual norm is an
absolute residual, not a condition estimate. An ill-conditioned matrix can have a
small residual and a large solution error. SVD, condition estimates, iterative
refinement, complex and extended-precision decompositions remain future work.

## Validation and interpreting reports

Tests include scalar oracles, independent nalgebra/ndarray comparisons, reconstruction,
orthogonality, residuals, finite-difference VJPs, broadcasting, extreme logits,
zero sizes, reversed layouts, shared parameters, checkpoint corruption and exact
optimizer resume. Gradient finite differences use f64 with documented bounds.
Storage and safe native wrappers are also exercised through Miri; scalar interpreter
coverage does not certify every architecture-specific assembly path.

Benchmarks retain dtype, thread count, sizes, allocation boundaries, raw batches,
environment/dependency metadata and a source snapshot for new runs. Current Linux
runs pin one allowed CPU. Clean evidence reports use nearest-rank p95 over 256
individual warmed calls, separately from 15 batch means and three first-use samples.
Older batch-only reports used the maximum of 15 means as their reported p95; those
figures are not individual-call latency percentiles. Historical unpinned reports show indicative
trends and must not be treated as controlled causal comparisons. Kernix through
0.7.1 and ndarray share matrixmultiply for native GEMM; 0.7.2 selects gemm-f64 for
eligible dense products as described below. The manual derivative baseline omits the graph
and safety semantics of autodiff; its speed difference is disclosed. No claim of
universal leadership or a complete model's frame rate follows from these tables.


## Dense floating CPU engine selection

<a id="dense-f64-cpu-engine-selection-072"></a>

C-contiguous products with m, k and n at least 64 use serial `gemm-f64` (since
0.7.2) or `gemm-f32` (since 0.7.3) on x86-64 with std and the engine's complete
runtime-detected SIMD feature set. C^T = B^T A^T is passed in column-major
orientation, writing the same C-order output; this does not materialize a transpose.
Small/skinny products, signed-stride rank-two inputs, no_std and other targets
retain matrixmultiply. Noncontiguous batches retain the checked reference path.
Custom scalar implementations and all shape/storage checks remain unchanged.
Native engines may use FMA and reassociation; ordered mode remains available via
`matmul_reference` and `CpuExecutor::with_ordered_matmul(true)`.

These engines are pure Rust and explicitly serial; they add no external BLAS or
implicit thread pool. They can allocate packed scratch and cache an L2-sized slab
in thread-local storage until thread exit. Warmed allocation probes exclude that
first-use allocation and retention; first-use timings and process RSS are recorded
separately. This is an internal heuristic measured on one CPU, not an optimal
threshold or performance guarantee for every x86-64 processor. Miri uses the existing
portable native path; native instructions require execution tests on actual hardware.

No internal dtype promotion is introduced. f32 accuracy checks use a wider f64
scalar oracle constructed from the actual f32 inputs, independently of timing loops.
