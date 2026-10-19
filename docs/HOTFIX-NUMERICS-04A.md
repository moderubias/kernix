# Kernix numerics & softmax 04a — test-oracle and Clippy hotfix

**Target:** Kernix `master` (`b208e423`) **after** applying `kernix-numerics-softmax-04.zip`.

This is a narrow correction to overlay 04, not an architectural change.

## Fixes

- Remove the unused `elementwise::map` import in `crates/kernix-ops/src/ops/reduce.rs`.
- In `mean_all`, replace two unnecessary `clone()` calls on `FloatElement: Copy` with dereferences, removing `clippy::clone_on_copy` warnings.
- Correct the `noncontiguous_matrix_falls_back_to_checked_indexing` **test's expected values**, not the `matmul` implementation. Reversing the columns of `[[1,2,3],[4,5,6]]` gives `[[3,2,1],[6,5,4]]`; multiplying by `[[7,8],[9,10],[11,12]]` yields `[[50,56],[131,146]]`.

The original `matmul` output was correct. Do not change the algorithm to make it return `[46,52,127,142]`.

## Apply

```sh
cd ~/All_Work_Folders/moderubias/projects/Rust/kernix
git status --short
unzip -o ~/Downloads/kernix-numerics-hotfix-04a.zip -d .
python3 tools/kernix-update-04/fix_04a.py --check
python3 tools/kernix-update-04/fix_04a.py
cargo fmt --all
cargo test --workspace --all-features --locked
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo check -p kernix-core --no-default-features --locked
cargo check -p kernix-ops --no-default-features --locked
git diff --stat
git status --short
```

`--check` performs non-mutating anchor validation. The patch fails closed if it detects unexpected source and can be re-run safely after successful installation. The script changes only two existing files. Existing user modifications outside those exact fragments are preserved.

`zsh: command not found: #` comes from interactive zsh when `INTERACTIVE_COMMENTS` is disabled; omit comment-only lines or enable `setopt interactivecomments`.

## Verification

- Confirmed the expected matrix product independently using direct scalar arithmetic.
- Checked both generated patch variants and repeated application on representative source fixtures.
- The container used to prepare this archive has no Rust toolchain. Cargo, Clippy, and Miri results for this hotfix must be checked locally.
