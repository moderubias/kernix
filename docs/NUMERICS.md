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
small residual and a large solution error. One-norm conditioning and working-precision iterative refinement are available
since 0.8. `condition_number_one` / `reciprocal_condition_number_one` use a globally
scaled LU and every identity-column solve: O(n³), not an inexpensive estimator or
two-norm condition number. Exact singular factors give reciprocal zero; required
unrepresentable solves give NonFinite. Global scaling cannot cover every exponent
range. `solve_refined` uses compensated residuals and a scaled componentwise
backward-error denominator, accepts only improving corrections, and reports
convergence, stagnation or the explicit correction budget. It retains original A
and LU, allocates RHS-sized scratch per attempt, and uses the input dtype. Neither
convergence nor a forward-error bound is promised. Default LU pivot policy applies.
SVD, complex and extended-precision decompositions remain future work.

Cholesky reuses its validated input copy as the C-order lower factor. Small
matrices use right-looking updates with O(n) column workspace; larger ones use
independent accumulation lanes and retry ordered subtraction on nonfinite products
or nonpositive diagonal candidates. This can change rounding, not the finite/SPD
validation policy. QR packs directly into column-major work storage without an
intermediate C-order matrix; empty thin factors return without dimension-sized
loops. Neither decomposition promises bitwise agreement across versions.

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
Small/skinny products, negative/zero-stride rank-two inputs, no_std and other targets
retain matrixmultiply. Since 0.12, positive-stride rank-two views on std/x86-64
can also use the same serial native engine when all dimensions are at least 32
and m*k*n is at least 64³. Borrowed transposes, F-order and gapped inputs need no
materialized tensor copies; native packing workspace can still allocate/cache. Noncontiguous batches retain the checked reference path.
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


## Column-pivoted QR and rank-truncated solving (0.9)

`PivotedQr` returns `A[:, permutation] = Q*R` with thin orthogonal Q and upper
trapezoidal R. Partial pivot-norm estimates are updated after each reflector; scaled norms are
recomputed when cancellation/cumulative loss reaches sqrt(epsilon), or an update
underflows to zero. Every selected reflector uses a fresh scaled norm. The leading rank cutoff is relative to the largest R
diagonal. Its default is epsilon times max(m,n,1); any finite nonnegative explicit
tolerance is accepted. Ratios avoid overflowing a cutoff product; zero tolerance
only drops exact-zero diagonals. This is not an SVD rank, a condition estimator,
a certified strong rank-revealing factorization, or optimal low-rank approximation.

Both solvers operate on the selected rank-truncated matrix, not discarded original
directions. `solve_basic_least_squares` sets free permuted coefficients to zero.
`solve_least_squares` instead returns the minimum Euclidean-norm solution using a
scaled RZ reduction of leading R with implicit right reflectors; no normal equations
or inverse are formed. For full column rank it uses the cheaper triangular path.
For deficient/wide matrices the extra O((n-r)*r²) RZ factorization occurs per call,
with O(n*r) factor storage. It exploits the triangular/trapezoidal structure and
avoids an explicit right basis; rank policy and late RHS scaling are preserved.
The finite RHS is borrowed, and the final permutation/scaling writes directly to
owned C-order output. Intermediates must remain representable; scaling does not
provide arbitrary-exponent guarantees. Existing unpivoted Qr/least_squares still
reject deficient and underdetermined inputs. Neither decomposition records a VJP.


For norms so small that epsilon*norm underflows, QR now constructs dimensionless
Householder vectors from separate scale and unit-norm components. This prevents
subnormal norm rounding from destroying Q orthogonality (f32/f64 regression).
R stays in the input dtype; subnormal R and RHS projections can still lose
information. Relative solve accuracy at those limits requires explicit problem
rescaling; no automatic promotion or arbitrary-exponent guarantee is provided.


## Coalesced contractions, private gradients and parallel tasks (0.10)

C-contiguous tensordot inputs with contracted left suffix and right prefix axes
are read through flat slices, without packing or transposing tensors. The exact
increasing-contraction scalar order and Clone count are retained, with the same
weak scalar bounds. Other axis/layout combinations keep the indexed fallback;
this is a coalesced subset, not a general packed TBLIS implementation.

Backward accumulation reuses C-contiguous buffers owned exclusively by the current
backward invocation. Public retained gradients and primals cannot alias those buffers;
a nonfinite sum discards the local result. Strided accumulation retains its previous
fallback. Repeated backward still returns fresh gradients.

Explicit parallel x86-64/std native dense products with m>=128 and k,n>=64 use
row chunks in multiples of 64, balanced against the private worker count, when
the complete V3 feature check succeeds. This keeps tasks eligible for the native
gemm engine instead of always splitting them below its row threshold. Small,
skinny, ordered, portable and unavailable-feature work retains the 32-row split.
No CPU affinity is forced; performance depends on actual topology and scheduling.
Native worker threads may retain their own packed scratch after first use.


## Blocked Cholesky and explicit execution (0.11)

Cholesky keeps its full finite/symmetry validation and lower-authoritative policy.
Native-capable scalars use a left-looking blocked factorization for n>=192, with
16-column panels and a fixed 64x16-element fallible Schur workspace. Borrowed
signed-stride matrix panels avoid full transposes and additional n-by-n input copies.
Diagonal/nonfinite cancellation retries use the original entry and ordered products.
Smaller/custom portable-capability work retains the existing factorization paths.
Rounding can differ; positive-definiteness has the same finite-dtype limitations.

AArch64 std dense eligible products outside macOS use the gemm-rs NEON table directly, bypassing
its optional AMX selector even when downstream Cargo features unify. Small, strided
and no_std products retain their existing paths. Explicit parallel chunk eligibility
is matched to this engine; the caller still chooses worker count.
macOS retains its previous matrixmultiply default: process-paired measurements
found no stable GEMM128 improvement and a regression for f32 GEMM256 with the fixed
NEON table. The optional vendor executor below remains a separate choice.

The optional accelerate feature exposes kernix::accelerate::AccelerateExecutor.
It requires macOS15+, SDK thread-control declarations and C-contiguous rank-two
f32/f64 host tensors. Construction fails on unsupported targets. It uses the SDK's
new LAPACK/BLAS interface, requests one BLAS thread for each call and restores the
caller's previous thread-local policy. Output is allocated fallibly; no input copy,
device transfer, hidden CPU fallback or autodiff integration is provided. Vendor
workspace is opaque and not included in Rust allocation probes. The portable CPU
remains the default. Dimensions passed to the SDK fit signed32-bit integers; empty
products are handled without BLAS. The optional SDK bridge needs an Apple C toolchain.

## Small dense kernels (0.11.1)

On x86-64 with `std`, runtime AVX2 and FMA support enables a bounded no-packing
path for dense f32/f64 products: rows 4–32, contraction 4–64, columns at most 16
in complete tiles of 16 (f32) or 8 (f64). Four rows and two SIMD vectors per row
reuse each loaded right-hand tile. Remaining rows use smaller specializations;
column tails outside those tiles retain the general engine. Exact input/output
lengths are checked before dispatch. No aligned-input assumption is made.

The native hook writes the existing output and uses no heap scratch, including
first use. Ordinary `matmul` still allocates its result. Contiguous broadcast
batches reuse this path per matrix. This changes fused rounding, not shape,
ownership, dtype or error contracts. Custom scalars, signed-stride views,
`no_std`, unavailable CPU features and explicit ordered execution keep their
existing paths. The default 128×128/256×256 engines are unchanged.


## Reusable solves, compact-WY QR and factor modifications (0.12)

LU, Cholesky and unpivoted QR substitution traverse contiguous RHS rows when
there are more than four columns. Each output retains increasing-k scalar
subtraction, with no additional heap workspace. Four or fewer RHS columns keep
the earlier scalar loop. The output allocation and finite checks remain part of
the public solve operation. This does not change factorization or rank policy.

For native-capable tall/square inputs with m>=192 and n>=128, unpivoted QR groups
32 Householder reflectors as H=I-V*T*V^T. Forward factorization applies H^T;
reverse Q construction applies H. The triangular panel is computed once and
reused. The eventual Q allocation doubles as product scratch; additional panel
workspace is O((m+n)*32). Native GEMM may additionally pack/cache workspace.
Each candidate block update is checked before modifying factor data. Unsupported
or nonfinite native products fall back to individual reflectors. Scaled norms,
subnormal handling, finite validation and existing thin-Q/R shapes are retained.
FMA and blocked arithmetic can change rounding. Pivoted QR is unchanged.

`Cholesky::rank_one_update` and `rank_one_downdate` construct new lower factors
of A+x*x^T and A-x*x^T in O(n²), without materializing A or x*x^T. They allocate
an n×n output and n-element workspace and borrow their finite length-n input.
Update rotations use a scaled hypotenuse; downdates require |x[k]|<L[k,k]
at every pivot and retain the small representable diagonal gap before taking a
square root. A failed downdate leaves both original inputs untouched and reports
the failing pivot or nonfinite intermediate. The methods support signed-stride
vectors, custom LinalgFloat types and no_std+alloc, and do not record gradients.
Reciprocal multiplication in downdates and rotations introduce ordinary rounding;
near-boundary positive-definiteness and accuracy remain conditioning dependent.
