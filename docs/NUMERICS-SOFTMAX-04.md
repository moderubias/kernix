# Kernix — Numerics, online softmax, and CPU direct-access patch 04

Target: `moderubias/kernix` `master`, verified CI baseline `b208e4231d00635eb4abb5c1647222f5779c43a9` (2026-10-19).

This is **an additive source overlay**, not a release or a validated performance claim. It carries a stdlib-only, idempotent Python patcher to update nine existing Rust files, a new source module, and regression tests. It does not erase modules, change the crate topology, add dependencies, or alter existing `softmax`/`log_softmax` NaN/infinity policies.

## Features and corrections

| Area | API / change | Contract |
|---|---|---|
| Numerics | `softplus` | Uses accurate `log1p(exp(...))` branches to retain small positive outputs for negative logits |
| Numerics | `FloatMath::{ln_1p, exp_m1}` | Default methods preserve third-party trait implementations; `f32`/`f64` use `libm::{log1pf,log1p,expm1f,expm1}` |
| Numerics | `mean_all`, `mean_axes` | Divide each element before accumulation to avoid overflowing a finite output *solely because the original sum overflows*. This is **not** a universal precision guarantee; cancellation and subnormals need richer accumulation policies |
| Numerics | `log1p`, `expm1`, `log1mexp`, `logaddexp`, `logdiffexp` | Log-domain and cancellation-resistant primitives; the real domain applies to `log1mexp` (`x<=0`) and `logdiffexp` (`a>=b`) |
| Activations | `logsigmoid`, `mish`, `rsqrt`, `softplus_beta` | Stable log-sigmoid and generalized softplus; positive, finite beta required |
| Math | `FloatTrig`, `sin`, `cos`, `hypot`, `atan2` | Portable `libm`, NumPy-style broadcasting for binary ops |
| Softmax | `softmax_temperature`, `log_softmax_temperature` | Normalize `(x-max)/temperature`, not `x/temperature`; finite, strictly positive temperature required |
| Attention masks | `masked_softmax`, `masked_log_softmax` | Boolean mask (`true`=include) broadcasts **to** logits, masked values never read; masked probabilities zero, masked log-probabilities `-inf` |
| Attention masks | `masked_softmax_temperature`, `masked_log_softmax_temperature` | Fused mask + scaling; fully masked nonempty fibers return structured `OpsError::InvalidParameter` |
| Online normalization | `softmax_online`, `log_softmax_online`, `softmax_online_temperature` | Online max/exp-sum recurrence: two logical source passes, not a claim of FlashAttention/kernel fusion |
| Online logsumexp | `logsumexp_online_all`, `logsumexp_online_axis` | Online normalization recurrence: **one** logical input pass per reduction group; finite-only matches current `logsumexp_*` contract |
| Direct-access CPU path | `TensorInput::as_c_contiguous_slice` | Optional, object-safe extension with default `None`; built-in owned/borrowed tensors expose contiguous views |
| Linear algebra | `matmul`, `dot` | Direct contiguous slice reads (no per-scalar coordinate and offset validation), with existing indexed fallback for noncontiguous and third-party inputs |
| Elementwise/reductions | `map`, `zip_map`, `add/sub/mul/div/rem`, `sum_all`, `prod_all`, `mean_all` | Contiguous direct reads when semantically equivalent; existing checked logical indexing fallback for broadcasting/strides |
| Shape | `Kahen::{squeeze,unsqueeze,swap_axes}`, `Kotei::swap_axes` | Geometry-preserving helpers; fixed-rank swap does not erase rank; safe singleton squeeze |

### Softmax details

For online softmax on finite logits, maintain running `(m, l)`:

```text
m' = max(m, x)
l' = l * exp((m - m') / temperature) + exp((x - m') / temperature)
```

Start with `(m, l) = (first_value, 1)` to avoid `-inf - -inf`. After one pass,

```text
p[i] = exp((x[i] - m) / temperature) / l
```

Only two logical input passes are needed; old standard `softmax` remains available and unchanged. This implementation is a **general, CPU, checked-index reference kernel**. No GPU tiling, shared-memory staging, tensor-core kernel, packed SIMD, or FlashAttention graph fusion is included.

Masks use **keep semantics**, not additive bias. A `false` location is never read (permitting masked NaNs and infinities), and a fully masked, nonempty reduction group is an explicit error. Unmasked invalid values retain the `NonFiniteInput` policy. Zero-output shapes are valid without evaluating any group.

### Accuracy/compatibility limits

- Extending `FloatMath` with default implementations is source-compatible with existing implementers; third-party implementations should override `ln_1p`/`exp_m1` to obtain full precision.
- The mean correction fixes one specific overflow failure but cannot guarantee correctly rounded means; weighted online/compensated or wider-precision accumulation belongs in a later dtype-policy update.
- The contiguous `matmul` path is **not a blocked GEMM**. Complexity is still O(m*n*k), and no speedup is claimed without measurements.
- Direct access requires the exact logical C-contiguous payload contract. Inconsistent third-party `TensorInput` implementations can return incorrect results (but this API does not grant unchecked access).
- The `softmax_online*` routines require finite logits, matching standard Kernix `softmax`; no change to existing behavior is made.
- Empty reductions with nonempty outputs remain explicit errors.

## Installation (from the repository root)

```bash
cd ~/All_Work_Folders/moderubias/projects/Rust/kernix

git status --short
unzip -o ~/Downloads/kernix-numerics-softmax-04.zip -d .

# Check all source-edit anchors and local changes without modifying tracked files:
python3 tools/kernix-update-04/apply.py --check
# Then apply the patch once (safe to run again):
python3 tools/kernix-update-04/apply.py

cargo fmt --all
cargo test --workspace --all-features --locked
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo check -p kernix-core --no-default-features --locked
cargo check -p kernix-ops --no-default-features --locked
cargo check --workspace --locked

git diff --stat
git status --short
```

The `unzip` step adds new source/tests/docs/tools only. Existing Rust files are modified **only** by the explicit Python command. The patch script preflights all source anchors, refuses to change any target file with uncommitted edits, and recognizes an already-applied patch.

Commit these changes only after tests succeed. Existing GitHub CI success applies to **the baseline**, not to the new overlay. Test/Clippy/Miri/benchmarks must run on the **patched tree** before treating this patch as verified.

## Research and implementation references

- Maxim Milakov, Natalia Gimelshein, *Online normalizer calculation for softmax* (2018): https://arxiv.org/abs/1805.02867
- PyTorch log-softmax documentation (numerical stability relative to `log(softmax)`): https://docs.pytorch.org/docs/stable/generated/torch.nn.LogSoftmax.html
- PyTorch softplus documentation (stability and parameters): https://docs.pytorch.org/docs/stable/generated/torch.nn.Softplus.html
- Kernix source baseline and API documentation: https://github.com/moderubias/kernix/tree/b208e4231d00635eb4abb5c1647222f5779c43a9

This patch provides deterministic reference semantics and test vectors. It does **not** represent a competitive benchmark against `faer`, `ndarray`, BLAS, or accelerated ML frameworks.
