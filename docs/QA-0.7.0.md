# Kernix 0.7.0 verification

## Scope and compatibility

The candidate extends first-order CPU training with vector/broadcast-batched matmul,
signed-axis sum/mean, signed-step axis slices and population layer norm. Linear
recording accepts vectors and arbitrary leading axes. Existing literals, scalar
identity traits, ownership/device boundaries and runtime dependencies are preserved.
No higher-order, decomposition or general integer-index derivative is claimed.

Matmul promotes/transposes borrowed views and skips constant operand derivatives.
A shared right matrix/vector contracts leading rows directly, avoiding a full
per-batch weight-gradient temporary. Contiguous floating batches dispatch checked
native kernels per matrix. Impossible output capacities after zero-length
contractions now return resource errors rather than Vec capacity panics.

Layer norm uses one Welford mean/variance state, avoiding spurious offsets in
constant groups with tiny epsilon. A scaled standard-deviation combination avoids
variance+epsilon overflow. Nonfinite/unrepresentable statistics are now errors,
including cases that formerly produced zeros/NaNs. This is a deliberate numerical
validation correction in0.7; it is not an arbitrary-range statistics guarantee.
Affine gradients compose through broadcast mul/add; epsilon is fixed.

## Verification before measurement

Workspace tests passed: **514 default, 516 all-feature, 507 no-default**, including
rustdoc. Strict fmt, Clippy and rustdoc passed. Thirteen executable Markdown examples
pass. An actual external no_std library executes two workflows on Rust1.86,
including batched gradients, axis means, normalization and slice VJPs; it compiles
for wasm32, i686 and aarch64. The workspace MSRV1.86 check also passes.

Fourteen new tensor-training integration cases cover finite differences, scalar
seeds, empty/broadcast/strided geometry, shared DAGs, repeated backward, custom
seed errors, shared Linear parameters, normalization affine derivatives and slice
scatter. A normalized two-layer batched model reduces loss without custom VJPs.
Native batches are checked against ordered reference products for f32/f64,
with integer/strided fallback and impossible output-capacity regressions.

Three independent Candle graph tests compare matmul, affine normalization and axis
reduction VJPs. The stage harness validates every loss against a scalar implementation
and every weight/bias derivative against central differences. Five harness tests pass
normally and five with allocation profiling (serial execution); strict default and
all-feature harness Clippy pass. Eight Python inventory/evidence tests pass. The
source-checked inventory now classifies174 numerical entries and28 Tape methods.

An isolated minimal consumer passed three Miri tests for tiny native/broadcast
products, shared-weight/vector VJPs, normalization and reverse-slice scatter. This
is selected interpreter coverage, not certification of every native assembly path.

## Evidence and release gates

Clean source `aa7347a201ce9294b02990b7598af146470bca65` and crates tree
`fa1808e9b63894b529e661372b801b02750a5d25` were measured with clean status
before/after. Retained reports:

- [Workspace0.7](../benchmarks/results/0.7.0-clean-aa7347a/README.md): 50 validated cases.
- [Registry0.6 baseline](../benchmarks/results/0.6.0-registry-aa7347a/README.md):
  43 cases; all three packages pinned0.6.0, registry VCS `56e71f0`. Unsupported
  new Tape stages are listed as unavailable, never replaced by manual derivatives.

Comparator lockfiles match: faer0.24.4, Candle0.11.0, ndarray0.17.2,
nalgebra0.35.0 and matrixmultiply0.3.11. NumPy uses verified OpenBLAS and
thread/affinity controls. Both source archives exclude private web/planning files;
raw observations, accuracy/resource budgets and source/binary hashes are retained.

Broadcast-batched GEMM `[4,16,32] @ [1,32,16]` measured 2.060us per-call median
versus 280.496us in registry0.6 (about136x for this particular prior reference path).
Current Candle measured1.909us. Rank-two graph/backward measured2.403us versus
3.084us in0.6. For the normalized/sliced training case, Kernix/Candle medians were
3.809/3.282us inference, 4.646/4.756us recording/forward, and8.668/13.309us backward.
Kernix fixed-gradient SGD measured0.483us. These allocation-inclusive shared-machine
observations do not establish a universal ranking; all comparator wins/losses remain.

Publication/source IDs, remote CI and registry-only verification follow their
respective release gates. Stages separate inference,
eager recording/forward, backward and fixed-gradient transactional SGD. Graph
allocation counts include owned values/scratch, not container-only attribution;
no subtraction of medians is used to claim a graph-only time. Legacy0.6 Tape lacks
these operations and omits unsupported stages explicitly. Kernel-only/full-application
latency and accelerator hardware remain unmeasured.
