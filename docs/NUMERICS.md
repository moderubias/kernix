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

Native f32/f64 rank-two matmul may use packed SIMD/FMA kernels. Reassociation/FMA
can differ in final bits from increasing-k scalar reference arithmetic. Choose
`matmul_reference` or executor `ordered_matmul=true` when that order is required.
This controls accumulation order rather than guaranteeing architecture-independent
IEEE behavior. Optimized contiguous convolution packs bounded output tiles and uses
native GEMM; `neural::conv2d_reference` retains ordered scalar accumulation. Other
layouts use the reference path. Temporary packing is bounded by kernel dimensions,
channels per group and at most 128 spatial positions, rather than the full image.

With std, elementary float functions use the platform implementation by default.
`portable-math` selects Rust libm even with std; no_std uses libm. All-features tests
therefore exercise portable math; default-feature tests separately exercise std math.
Neither feature enables compiler fast-math. Each backend needs error-bound tests,
not just bitwise comparisons with a different arithmetic order.

## Stable operations and linear systems

Softmax subtracts the row maximum, validates logits and caches exponentials for
normalization. Temperature softmax handles differences before unsafe scaling where
possible. Stable losses avoid unnecessary overflow in logit differences and means.
Variance uses Welford accumulation; L2 norm uses scaled sums of squares. Extreme
representability limits still produce documented errors or IEEE results.

Pivoted LU solves square systems and defaults to a scale-relative near-singular
pivot cutoff. Cholesky checks symmetry and positive pivots without adding jitter.
Householder QR avoids normal equations for least squares. QR rejects deficient or
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
runs pin one allowed CPU. p95 in these reports is the maximum of 15 batch means,
not a request-level latency percentile. Historical unpinned reports show indicative
trends and must not be treated as controlled causal comparisons. Kernix and ndarray
share matrixmultiply for native GEMM. The manual derivative baseline omits the graph
and safety semantics of autodiff; its speed difference is disclosed. No claim of
universal leadership or a complete model's frame rate follows from these tables.
