# Kernix: functional audit and Ops Expansion 02

Baseline: `moderubias/kernix`, `master` at `77237b0b56434f3b13f7bae188dcfe6c051554e8` (2026-10-19), after the prior Hardening 01 patch.

**Status:** Source additions and structural/static checks prepared. **NOT built with Cargo in the artifact environment** (no Rust toolchain). This document is not an assertion that the code compiles, passes Clippy, or is production ready. Run the exact checks below before committing.

## Added public CPU operations

| Group | Public API | Contract |
|---|---|---|
| Comparison | `equal`, `not_equal`, `less`, `less_equal`, `greater`, `greater_equal` | NumPy-style right-aligned broadcasting; IEEE unordered comparisons for float NaN |
| Boolean | `logical_not`, `logical_and`, `logical_or`, `logical_xor` | Operations on `bool` tensors; broadcasting for binary ops |
| Conditional | `where_select` | Three-way broadcasting of condition and two values; fresh output, clones only selected values |
| Bounding | `clamp` | Inclusive scalar bounds, reject inverted/NaN bounds; NaN inputs propagate |
| Structural | `concat`, `stack` | Borrowed heterogeneous host inputs via `&dyn TensorInput<T>`; allocate C-order output |
| Indexing | `gather_axis` | Position vector along one axis, duplicate indices permitted, always bounds-checked |
| Repetition/padding | `repeat_axis`, `pad_constant` | Checked shape arithmetic; no input mutation |
| Boolean reductions | `any_all`, `all_all`, `any_axes`, `all_axes` | False/true identities for empty reductions respectively |
| Count | `count_nonzero_all`, `count_nonzero_axes` | Value != zero; IEEE NaN considered nonzero |
| Arg reductions | `argmin_all`, `argmax_all`, `argmin_axis`, `argmax_axis` | Flat C logical index for `_all`, axis index for `_axis`; first tie wins; first NaN wins |
| Scans | `cumsum_axis`, `cumprod_axis` | Inclusive logical-order scans along axis |
| Float elementary | `exp`, `ln`, `sqrt`, `tanh`, `abs`, `is_nan`, `is_finite` | `f32`/`f64`, portable `libm` backed, `no_std` |
| Activations | `relu`, `leaky_relu`, `sigmoid`, `silu`, `gelu`, `softplus` | Numerically stabilized where applicable |
| Normalization | `softmax`, `log_softmax` | Shift by max, single axis, reject NaN/Inf logits with indexed error |
| Statistics | `variance_all`, `variance_axes`, `std_all`, `std_axes` | Welford recurrence, `correction=0/1`; explicit degrees-of-freedom errors |
| Convenience | `kernix_ops::prelude` | Common traits and tensor types; mathematical functions deliberately not wildcard imported |

**Core arithmetic contract remains unchanged:** original `add/sub/mul/div` use native Rust operators (including integer overflow and division rules). Use the Hardening 01 `checked_*` variants for checked integer arithmetic.

**Error behavior:** All new allocation-producing ops return `Result<... , OpsError>`. Invalid axis, unmatched shape, invalid bounds, arithmetic shape overflow, invalid degrees of freedom, and non-finite softmax logits have explicit error variants. The new operations are **reference CPU kernels**, not accelerated. `gather_axis` and `pad_constant` copy data by `Clone`; views retain zero-copy semantics in `kernix-core`.

## Installing overlay without deleting existing files

From the **repository root** (not your home directory):

```bash
cd ~/All_Work_Folders/moderubias/projects/Rust/kernix
git status --short
unzip -o ~/Downloads/kernix-ops-expansion-02.zip -d .
# New `libm` dependency requires Cargo.lock regeneration once:
cargo check -p kernix-ops
cargo fmt --all
cargo test --workspace --all-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo check -p kernix-core --no-default-features
cargo check -p kernix-ops --no-default-features
cargo test --workspace --locked
git diff --stat
```

Before unzipping, ensure local changes are saved/committed; `-o` overwrites matching paths. This overlay touches `crates/kernix-ops` and adds this documentation only; existing Shape, Layout, Storage and Tensor source files are not replaced. The feature `parallel` is still not an optimized execution implementation.

Once tests pass, stage and review:

```bash
git add Cargo.lock crates/kernix-ops docs/OPS-EXPANSION-2026-10-19.md
git diff --cached --check
git diff --cached --stat
git commit -m "feat(ops): add tensor manipulation, comparisons, statistics and activations"
git push origin master
```

## Functionality that remains necessary, not implemented here

This is **not a complete ML/CV ecosystem**. The current architectural scope still needs:

- Tensor data types and conversions: f16/bf16, complex, quantized; dtype promotions/casting policies; device semantics.
- Higher indexing: advanced `gather` with tensor indices, batched `scatter`, mask indexing, slicing DSL, non-overlap-safe assignment.
- Structural ops: `split`, `chunk`, `tile`, `roll`, `flip`, `unfold/fold`, `einsum`, sparse data structures.
- General reductions: weighted reductions, product/mean overflow policies, stable logsumexp, quantiles, sort/argsort/top-k, norms, histograms.
- Linear algebra: optimized GEMM/batched matmul, factorization/solve/eigensystems/SVD, complex support.
- ML kernels: conv1d/2d/3d, pooling, attention, embedding, normalization, dropout, loss functions, backward/autodiff.
- CV: image dtype/layout conversions, resize/interpolation, color transforms, filters, morphology, warping, codecs, inference adaptors.
- Acceleration: optimized contiguous and strided kernel execution, SIMD, multithreading, fusion, GPU backend and transfers.
- Deployment: ONNX/model I/O, stable public facade, examples, versioned compatibility policy, profiling, benchmark baselines.
- Safety: Miri and sanitizers on memory-managing parts, fuzz/property tests for aliasing, failure injection, release+debug numerics.

**Performance claims require real benchmarks** against comparable libraries on the same hardware, shapes, strides, dtypes, thread counts and output semantics. This patch increases breadth and test coverage, not measured throughput.
