# API ergonomics: verified workflows and design decisions

The 0.7 API builds on the 0.6 identity constructors while preserving existing literal,
layout, ownership and device behavior. This audit is grounded in executable workflows,
not proposed syntax alone. Vector/batched matmul differentiation, axis sums/means, signed-step slicing and layer normalization
now have executable VJP coverage; no forward-only example implies other gradients.

## Simple construction, explicit escape hatches

```rust
use kernix::prelude::*;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let weights = Tensor::<f32>::zeros([128, 256])?;
    let biases: Tensor<f32> = Tensor::ones([256])?;
    let matrix = Tensor::from_rows([[1, 2], [3, 4]])?;
    let volume = Tensor::from_values([2, 2, 2], (1..=8).collect())?;
    assert_eq!(weights.shape().dims(), &[128, 256]);
    assert_eq!(biases.storage_slice(), &[1.0; 256]);
    assert_eq!(matrix.get(&[1, 0])?, &3);
    assert_eq!(volume.get(&[1, 1, 1])?, &8);
    Ok(())
}
```

`zeros`/`ones` require the actual Zero/One identity, not Default. Each generates
one value per element without Clone, Copy or arithmetic bounds. Validation and
allocation precede generation. Empty tensors generate no identities; scalars generate
one. C-order/natural alignment are explicit defaults, with `from_vec`, `filled`,
`from_fn` and buffer/layout constructors retaining other policies. These are CPU
constructors; no dtype conversion, transfer, configuration lookup or synchronization
is performed. There is one nonempty element allocation and possible high-rank shape
metadata allocation, not an allocation-free promise. Time is O(numel).

The scalar identity definitions now live in core, with the same methods, numeric
implementations and existing `kernix::Zero`, `kernix::One` and operation-crate paths
reexported. Core acquires no dependency on ops and no new runtime dependency.
Custom types remain supported, including types without Clone or Default:

```rust
use kernix::{Tensor, Zero, One};
struct Scalar(i32);
impl Zero for Scalar { fn zero() -> Self { Self(0) } }
impl One for Scalar { fn one() -> Self { Self(1) } }
let a = Tensor::<Scalar>::zeros([2])?;
let b = Tensor::<Scalar>::ones([])?;
assert_eq!(a.get(&[1])?.0, 0);
assert_eq!(b.get(&[])?.0, 1);
# Ok::<(), kernix::TensorError>(())
```

## Literal compatibility is intentional

```rust
use kernix::prelude::*;
let pixels = tensor![[255u8, 0, 0], [0, 255, 0]]?;
assert_eq!(pixels.shape().dims(), &[2]); // two array-valued pixels
let matrix = matrix![1, 2; 3, 4]?;
assert_eq!(matrix.shape().dims(), &[2, 2]);
# Ok::<(), kernix::TensorError>(())
```

Array-valued scalars are useful for pixels, coordinates and user-defined algebra.
Interpreting their brackets as axes would silently change existing tensor rank,
indexing and scalar type. `tensor!` therefore retains top-level vector semantics;
`matrix![[1,2],[3,4]]` likewise retains its existing one-row array-element meaning.
Use semicolon rows or `from_rows` for scalar matrices. For N dimensions, use checked
`from_values` or the explicit `tensor!(shape = dims; values = values)` form.
Both adopt an existing Vec without copying elements.

List expressions move non-Clone elements once, left to right. Existing repetition
and fill evaluation order is preserved: count/dimensions precede the fill expression.
The tests make these details explicit. Shape/layout/storage failures remain Results;
ragged matrix rows remain compile-time errors. No panic-based shorthand is added.

## Executable workflow suite

Run `cargo test -p kernix --test api_ergonomics --locked` and
`python tools/check_ergonomics.py`. The latter executes a separate no_std library
with default features disabled under a host test allocator. Use `--toolchain 1.86.0`
for MSRV and `--target wasm32-unknown-unknown` for compilation-only target checks.
CI also compiles that consumer for i686 and aarch64.

| Scenario | Executable evidence | Observed cost or boundary |
|---|---|---|
| Vector/matrix arithmetic and broadcasting | `vector_matrix_arithmetic_and_broadcasting` | Free functions compose through Results; each allocating operation owns its output |
| Higher-rank creation and reshape | `higher_rank_creation_and_zero_copy_reshape` | Vec adopted once; reshape_view shares storage, with explicit order/viewability |
| Batched matrix multiplication | `batched_forward_matmul` | Forward workflow plus independently tested batched/vector Tape VJPs in autograd_tensor_tests |
| Statistical reductions | `statistical_reductions` | Axis policy and population correction are explicit |
| Solving a linear system | `linear_system_with_residual` | Root/prelude solve and residual; residual is not a condition estimate |
| Neural forward pass | `small_neural_forward_pass` | Seeded layers compose with ordinary operations; no implicit graph |
| Autodiff training step | `ordinary_autodiff_parameter_update` | Explicit constants, parameter handles, graph lifetime and checked SGD update |
| Transposed/sliced views | `transposed_and_reversed_views_share_storage` | Pointer identity and indexed values verify shared storage |
| Explicit layout/output reuse | `explicit_layout_policy_and_reusable_output` | F→C conversion is a named copy; ordered executor reuses the output allocation |
| no_std + alloc consumer | `tests/fixtures/no_std_workflow.rs`, external checker | Executes on host, compiles on documented targets; final host supplies allocator |

The Rust cases are in [api_ergonomics.rs](../crates/kernix/tests/api_ergonomics.rs).
[Identity tests](../crates/kernix-core/tests/identity_constructors.rs) additionally
check scalar/empty/high-rank dimensions, invalid storage extents, zero-sized types,
non-Clone custom identities and generation counts before/after validation. Existing
literal tests retain move/evaluation/inference and compile-fail coverage.

## Decisions and deferred proposals

| Proposal | Classification | Concrete problem and decision | Costs / architectural assessment |
|---|---|---|---|
| zeros/ones associated constructors | Additive; implemented | IDE-discoverable numeric initialization without shape/order/alignment boilerplate | Checked C-order allocation, no Clone/Default/arithmetic bounds; explicit lower-level alternatives remain |
| Share Zero/One definitions through core | Compatibility-sensitive; implemented with legacy reexports | Constructors must not depend on ops or invent duplicate identity traits | Existing paths/custom implementations are tested; no bound strengthening or extra dependency |
| Reinterpret nested tensor! arrays | Breaking; not implemented | Attractive scalar-axis syntax conflicts with valid existing array-valued elements | Requires deliberate migration/version approval; no such migration is approved |
| Separate explicit ND literal macro | Additive proposal; deferred | Could reduce flattening ceremony for hand-written N-D data | Needs rank/leaf grammar, rectangularity diagnostics, empty cases and array-valued leaf policy; from_values already covers runtime workflows |
| More arithmetic macros/builders | Deferred | No executable workflow currently needs another syntax layer | Would expand permanent API and obscure fallibility or allocation boundaries without demonstrated benefit |
| Arithmetic operators | Deferred design | Could improve formulas, but failures must compose predictably | Panicking operators are not acceptable shorthand; Result-valued operators/expression wrappers need usability and overhead evaluation |
| Fluent operation extension trait | Additive proposal; deferred | May improve IDE discovery for repeated chains | Must live above core and show benefit over existing free functions; broad aliases alone do not justify it |
| Automatic reshape copying | Compatibility-sensitive; not implemented | A single name could hide view failures | Existing reshape_view fails explicitly; to_owned_contiguous exposes copying and selected traversal order |
| Broader gradient/Linear recording surface | Additive M1 work; implemented in 0.7 | Vector/batched matmul, axis sum/mean, signed-step slices, layer norm and leading-axis Linear recording remove training shape asymmetry | Finite differences, Candle graphs, empty/shared-DAG tests and normalized batched training pass; operations remain explicit and first-order |

Relevant implementation boundaries are [construction](../crates/kernix-core/src/tensor/create.rs),
[owned/view APIs](../crates/kernix-core/src/tensor/owned.rs),
[scalar capabilities](../crates/kernix-core/src/scalar.rs),
[CPU output reuse](../crates/kernix-ops/src/execution.rs),
[Tape](../crates/kernix/src/autograd.rs) and [layers](../crates/kernix/src/nn.rs).
No existing good API is rewritten solely to make a formula look shorter.


The [tensor training regressions](../crates/kernix/tests/autograd_tensor_tests.rs)
exercise affine normalization, signed reductions, shared Linear parameters, repeated
backward, empty contractions and a normalized two-layer batched model. Normalization
now rejects nonfinite/unrepresentable statistics that could formerly produce zeros
or NaNs; this is an explicit numerical validation correction. General integer indexing VJPs,
operator syntax and ND literal grammar remain deferred, not reported as implemented.

## Additive storage-aware matrix workflow (0.13)

`Matrix` wraps rank-two finite real matrices with automatic Dense/CSR selection.
It keeps the existing dense affine `Tensor` contract intact; CSR cannot expose
ordinary strides/HostBuffer without hiding a conversion. Construction scans once
for selection and again only when compressing. Explicit CSR assembly avoids the
dense input. Multiplication dispatches on stored representation without conversion.
The permanent surface addresses real sparse memory and arithmetic costs rather
than adding synonyms for dense operations. Tests exercise all four storage products,
views, ownership, empty/resource failures and a downstream no_std consumer.

Classification: additive `Matrix`/`CsrMatrix`/`CscView`; compatibility-sensitive
implicit changes to Tensor storage and numerical nonfinite semantics are rejected.
Deferred: N-D sparse TensorInput, automatic timing policy, batch/sparse autodiff
and index-width abstraction pending actual workloads and backend contracts.
See [contracts](SPARSE.md) and executable `examples/sparse.rs`.
