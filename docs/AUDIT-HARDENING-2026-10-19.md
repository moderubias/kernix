# Kernix — API/Correctness Hardening 01

Baseline inspected: `moderubias/kernix`, branch `master`, commit
`2cd8954d0333df790aacfea3861ef237c14eda55` (2026-10-19).

This is a **selective overlay patch**: it overwrites only the listed files.
It does not delete or rename modules and does not claim to finish the entire
ML/CV ecosystem. Existing APIs `Kahen`, `Kotei`, `Tensor`, `Layout`, `add`,
`matmul`, etc. remain available.

## Changes

1. **Checked integer arithmetic** in `kernix-ops`:
   - `checked_add`, `checked_sub`, `checked_mul`, `checked_div`, `checked_rem`, `checked_neg`;
   - supports all builtin signed and unsigned integer types, broadcast operands,
     arbitrary valid host tensor layouts;
   - returns `OpsError::Arithmetic { operation, indices }` instead of a native
     arithmetic panic/wrap for failing elements;
   - output indices refer to the *broadcasted* tensor, not storage coordinates.
   - **Existing** generic `add` / `div` / etc. still use native Rust operator semantics;
     this patch does **not** silently change them to checked operations.

2. **Fallible transforms** in `kernix-ops`: `try_map`, `try_map_indexed`,
   `try_zip_map`, `try_zip_map_indexed`. They propagate `OpsError` from a
   callback without exporting a partly initialized tensor. Output remains
   freshly allocated and C-contiguous; reading respects strided inputs.

3. **NaN policy correction** for `min_all`, `max_all`, `min_axes`, `max_axes`:
   - an IEEE float NaN anywhere in the reduced group propagates to its result;
   - an empty full reduction still yields `None`;
   - comparisons between *distinct* incomparable values of an arbitrary
     `PartialOrd` type retain the previous first-wins behavior.
   - **Behavior change** from the previous position-dependent skip-NaN policy.
     Revisit this policy before `1.0` if `nanmin`/`nanmax` are later introduced.

4. **Real no_std feature propagation**:
   - `kernix-ops` now depends on `kernix-core` with `default-features = false`;
   - `kernix-ops/std` still explicitly enables the core's `std` feature;
   - the existing `cargo check -p kernix-ops --no-default-features`
     can now check the intended dependency configuration.

5. **Main `kernix` facade**:
   - reexports existing `layout`, `storage`, `tensor` modules and their main
     types, in addition to its existing shape API;
   - it does **not** introduce a dependency on unpublished `kernix-ops`,
     preserving the current facade's independent packaging.

6. **CI**: formatting, full-feature tests, strict Clippy, no-default-features
   checks, MSRV (`1.86`) check, and a scheduled/manual nightly Miri workflow.

7. **Regression tests**: 14 operation cases plus 2 facade cases.

## Installation

From the repository root:

```bash
cd ~/All_Work_Folders/moderubias/projects/Rust/kernix
git status --short
unzip -o ~/Downloads/kernix-hardening-overlay.zip -d .
cargo fmt --all
cargo test --workspace --all-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo check -p kernix-core --no-default-features
cargo check -p kernix-ops --no-default-features
git diff --stat
```

Save or commit any existing local work **before** unzipping; `unzip -o`
overwrites the files provided in this overlay. It leaves all other files intact.

## Verification status

- Source review against GitHub master: **performed**.
- Python/static checks for archive structure and metadata: **performed**.
- `cargo test`/Clippy/Rustfmt in the artifact environment: **not available**.
- Miri: **not run**; new workflow permits running it in GitHub Actions.
- Passing tests before the patch do not prove the new code correct; run the
  validation commands after applying this patch and resolve any failures.

## Still outside this patch

The current Ops API remains a CPU reference layer, **not** a complete ML/CV
framework or an accelerated kernel execution engine. Critical missing areas
include numerical policies across all dtypes, optimized SIMD/BLAS kernels,
GPU backends, broader indexing/gather/scatter, reductions/scan/statistics,
random, complex and low-precision dtype support, signal processing and FFT,
convolutions, ML activations, autodiff, optimizers, ONNX/model I/O,
image codecs/transforms, performance benchmarks, and rigorous `unsafe`
verification. They require separate specifications and tested implementations,
not placeholder methods.
