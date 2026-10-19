# QA: 0.1.1

Initial QA preceded planning and implementation. The starting dirty workspace at
b208e42 had 228 passing tests and zero doctests. Newly added extreme-temperature
softmax and SiLU negative-infinity tests both failed before the fixes.

Final local checks on x86_64 Arch Linux, Intel Core i3-1215U:

| Check | Result |
|---|---|
| Workspace all-feature tests | 386 passed, including 148 doctests |
| Format / strict Clippy | Passed |
| Strict rustdoc (`-D warnings`) | Passed |
| Rust 1.86 workspace library build | Passed |
| Facade without default features | Passed |
| wasm32-unknown-unknown without default features | Compilation passed |
| i686-unknown-linux-gnu without default features | Compilation passed |
| Core package verification | Passed |
| Kiyoshima Open MIT OR Apache-2.0 | Root and all three packages verified |

Cross-compilation is not a runtime test. Windows/macOS and additional CPU targets
are configured in CI; this local report does not claim those jobs have run.
Miri has a separate workflow; no local Miri result is claimed for this release.

Fixes preserve existing user changes. Temperature-scaled differences subtract
before division when finite and divide first only when subtraction overflowed.
SiLU explicitly uses its continuous zero limit for negative infinity. Shape removal
revalidates product, and serde calls validated constructors. Property tests cover
broadcast commutativity, insertion/removal and permutation inverses.

Floating-point tests use tolerances. Passing tests are evidence for the covered
contracts, not a proof of universal numerical accuracy or absence of bugs.

See the immutable baseline and 0.1.1 benchmark reports under benchmarks/results.
Results expose significant remaining CPU matmul/softmax performance gaps against
ndarray. No optimized-kernel, complete training stack or 120-FPS model claim is made.
