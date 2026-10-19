# Kernix capability matrix — 19 October 2026

**Base:** `moderubias/kernix` `master` at `e9d510e4aff8a5b86724d5128d56178c76942b98`, with the `Ops Completeness 03` overlay applied on top.

## Distinguish implementation from production readiness

`Source` means code is present. `Verified` requires this exact repository
version to pass Cargo tests, Clippy, MSRV and feature checks in GitHub Actions.
`Audited` requires Miri/sanitizer/fuzz assessment where applicable. `Fast`
requires documented benchmarks against comparable libraries and workloads.
**This overlay was constructed in a tool environment without Rust toolchain;
source presence and test authorship must NOT be confused with verified build.**

| Area | New in overlay | Existing | Still necessary |
|---|---|---|---|
| Logical indexing | `take_axis`, `scatter_axis`, `masked_select`, `nonzero` | `gather_axis`, checked indexing, views | general N-D indexed scatter, masked assignment, index DSL, tensor scatter reductions |
| Reshaping/manipulation | `split_axis`, `chunk_axis`, `tile`, `roll_axis`, `unfold_axis`, `fold_axis` | transpose, permute, reverse, slice, concat, stack, pad, repeat | zero-copy unfold views, generic reshape planning, crop/warp, multi-axis tiling optimizations |
| Ordering | `sort_axis`, `argsort_axis`, `topk_axis`, float total order | min/max and argmin/argmax | quantile, median, histogram, searchsorted, selection kernels |
| Contraction | `dot`, `outer`, `trace`, `tensordot` | reference `matmul` | einsum parsing/planner, optimized GEMM/BLAS, factorization, solve, SVD/eigensystems |
| Stable math | `logsumexp_all/axis`, `l2_norm_all/axis` | Welford variance/std, softmax/log_softmax | wide reductions, weighted statistics, advanced norms, stable accumulation policies |
| Casting | explicit `cast` / checked `try_cast` | f32/f64 math, primitive integers | first-class f16/bf16, complex, quantization, promotion graph, device dtype support |
| Compute | None | CPU reference kernels | SIMD, threading, kernel fusion, autotuning, GPU execution & transfers |
| ML | None | elementary activations | convolution/pooling, autodiff, optimizers, normalization, losses, attention |
| CV | None | tensor primitives | image I/O/decoding, resize, filters, morphology, color spaces, inference integration |
| Engineering | new regression tests; updated README | CI, MSRV, no_std, Miri workflow | benchmark corpus, actual Miri reports, fuzz campaigns, sanitizer checks, examples |

## Newly implemented contracts

### Indexing

- `take_axis(source, axis, indices_tensor)` replaces the selected axis with
  all dimensions of the unsigned-index tensor, even when it is rank zero.
  Index bounds are checked before producing output, including empty outputs.
- `scatter_axis(source, axis, &[indices], updates)` returns a **copy**;
  duplicate target positions use the **last logical C-order update**.
  Source and update tensors are never mutated.
- `masked_select` accepts a boolean mask that broadcasts *to* the existing
  source shape; it returns a logical-C-order one-dimensional selection.
- `nonzero` returns coordinates `[count, rank]`, including a valid `[1,0]`
  coordinate tensor for a nonzero scalar.

### Structural

- `split_axis` validates exact sum; zero-length partitions are valid.
- `chunk_axis` uses a **maximum chunk size**, not a requested chunk count.
  Empty axes produce an empty list of chunks. `chunk_size == 0` is rejected.
- `tile` repeats **whole blocks**, unlike `repeat_axis`, which duplicates
  individual elements. `repeats` is right-aligned; missing repeats are 1.
- `roll_axis`: positive shift moves elements to higher logical indices,
  including `isize::MIN` as a valid negative shift.
- `unfold_axis` returns an **owned copy** with a window dimension inserted
  after the selected axis. `fold_axis` performs overlap-add and fills
  uncovered output elements with zero. This fold is not a mathematical inverse
  unless the overlap count is normalized separately.

### Ordering and numerical math

- `SortDirection::{Ascending, Descending}`, stable ties, `TotalOrder`:
  integer `Ord`, IEEE `f32/f64::total_cmp` (including NaNs and signed zeros).
- `topk_axis` returns `(values, original_indices)` along an axis, descending;
  it uses a full stable sort (reference implementation), not a partial select.
- `logsumexp_*` use max-shifting, reject NaN/infinity with indexed errors,
  reject empty nontrivial reductions (rather than silently returning NaN).
- `l2_norm_*` use scaled sum-of-squares, accept empty inputs with norm zero,
  and reject nonfinite elements by indexed error.
- `cast` uses Rust's `From`; `try_cast` uses `TryFrom`, no truncation or
  implicit promotion. Narrowing float conversions are **not covered** by these
  functions until explicit round/saturate/truncate semantics are designed.

## Testing requirements

```sh
cargo fmt --all
cargo test --workspace --all-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo check -p kernix-core --no-default-features
cargo check -p kernix-ops --no-default-features
cargo test --workspace --locked
```

Run these on the latest GitHub revision before calling the overlay verified.
For unsafe storage code also run the separate Miri workflow (it is scheduled
weekly, not guaranteed to have run for every commit).

## Remaining architectural obligations

This is an **API extension** rather than the final comprehensive Tensor/ML/CV
specification or a production-readiness certification. In particular:

1. Establish numeric policies for every dtype and op (overflow, NaN, signed
   zeros, zero-sized axes, precision and reproducibility).
2. Produce a complete dtype/device/kernel planner specification, not ad hoc
   placement rules. Preserve `Kahen`/`Kotei` and device-agnostic Layout.
3. Create a reproducible benchmark matrix (contiguous/strided, small/large,
   rank static/dynamic, f32/f64/f16/bf16, single/multi-thread, cold/warm).
4. Establish test oracle comparisons to NumPy/PyTorch reference cases and
   gate new optimized implementations against them.
5. Complete training/inference primitives and practical CV workflows.
