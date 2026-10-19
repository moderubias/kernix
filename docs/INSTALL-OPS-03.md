# Install Kernix Ops Completeness 03

Base revision: `e9d510e4aff8a5b86724d5128d56178c76942b98`.

**The ZIP contains relative paths rooted at the repository root.** Extract
into the actual project directory, not into `src` or `crates`.

```sh
cd ~/All_Work_Folders/moderubias/projects/Rust/kernix
git status --short
unzip -o ~/Downloads/kernix-ops-completeness-03.zip -d .
cargo fmt --all
cargo test --workspace --all-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo check -p kernix-core --no-default-features
cargo check -p kernix-ops --no-default-features
cargo test --workspace --locked
git diff --check
git diff --stat
```

If the working tree already contains edits, commit or back them up before
extracting because `unzip -o` replaces files by path.

The two existing `kernix-ops` source files replaced here are `src/ops.rs` and
`src/ops/error.rs`; the other source modules and existing tests are left
unchanged. This archive does **not** add or modify dependencies.

The archive was built without a local Rust toolchain. GitHub CI validation
must run after the patch is pushed. Do not treat the presence of tests as
proof that the tests have passed.
