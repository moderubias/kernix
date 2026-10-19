# QA record: Kernix 0.4.0

This release expands general dense algebra and improves CPU fundamentals, while
adding an explicit trainable graph layer. All assertions below distinguish local
verification from remote checks. Historical results remain in previous QA records.

## Local verification

- Workspace default suite: 458 passed before the final checkpoint regression.
- Workspace all features: 460 passed, including 176 doctests, before that regression.
  The added parser regression and facade suite were rerun after the fix; the final
  candidate CI reruns complete default and all-feature suites on all three OSes.
- All eight README/user-guide code blocks passed as downstream doctests. This extra
  check caught a missing residual_norm facade reexport; the export was fixed and
  the documentation check is now a permanent CI step.
- Full no-default workspace suite passed; Rust 1.86 library check passed.
- Strict Clippy (all targets/features), formatting and warnings-as-errors rustdoc passed.
- wasm32-unknown-unknown and i686-unknown-linux-gnu no-default compilation passed.
  aarch64 compilation is covered by CI, not a local runtime claim.
- All four Kiyoshima Open MIT OR Apache-2.0 passports verified. Core package dry-run
  verified 44 files; a dry-run alone does not constitute publication.
- Miri passed all 18 existing storage tests in an isolated harness importing the
  original test file. The pinned old nightly could not build an unrelated zerocopy
  dev dependency's AVX512 code directly, so the harness omitted core dev dependencies.
- Miri passed the three native-wrapper tests (signed strides, invalid lengths,
  empty matrices, native/reference bounds and ordered mode). An additional tiled
  convolution test was subsequently added and passed normal test suites. Interpreter
  checks do not certify every runtime SIMD implementation.

## Numerical and integration checks

LU tests reconstruct P*A=L*U, solve vector/multiple RHS systems, check inverses,
scale-relative pivot policies, singular/nonfinite cases and determinant/logdet.
Cholesky checks positive-definite reconstruction/solves and rejects asymmetry and
nonpositive pivots. QR checks thin reconstruction and orthogonality for tall/wide
matrices, least-squares coefficients and normal residuals. Independent nalgebra
comparisons run before benchmark timing. f32 and strided inputs have explicit tests.

Native GEMM and tiled convolution retain strict error-bound comparisons with ordered
reference paths, including tails, zero sizes, transposes/reversals, groups, padding,
dilation, multiple batches and more than one output tile. Existing finite-difference
convolution and all tape VJPs pass without weakening their tolerances.

Tape tests cover broadcasting, branching, shared layer parameters, constants,
disconnected variables, explicit accumulation, stale/foreign handles, detach, empty
broadcasts and finite/domain failures. Optimizer tests cover oracles, state identity,
transactionality and clipping. Checkpoints roundtrip and resume f32/f64 optimizer
trajectories exactly; malformed/corrupt/truncated inputs and shape/name/dtype mismatches
fail. A new deliberately forged valid-checksum payload exposed acceptance of an
impossible Adam bias power of one; its regression failed first and now passes with
the strict [0,1) power check. Checksums are not authentication.

The synthetic training example ran successfully: categorical loss 0.665737 to
0.008833, with checkpoint save/reload. The temporal example ran five explicit state
updates. Neither example is the user's application or a real-data quality guarantee.

## Performance and remaining work

`benchmarks/results/0.4.0` contains raw batches/environment, source archive/SHA-256,
comparison chart, Markdown and printable PDF. All older reports are retained. One
CPU is pinned on this i3-1215U; frequency and desktop load are uncontrolled. The
reported p95 is a maximum batch mean, not individual frame latency.

Measured median: GEMM128 86.102 us (ndarray 82.070), reused output 82.316 us (81.199),
sum65536 5.538 us (5.607), softmax 77.015 us (69.556), tiled convolution 11.361 us
(manual ndarray loops 53.901), RGB1080-to224 438.953 us (manual ndarray 685.549).
The exact comparison methods are disclosed; ndarray shares the GEMM engine.

Remaining measured gaps: LU solve 26.964 us vs nalgebra 17.685, Cholesky 23.375 vs
10.565, QR 88.613 vs 32.264, reverse-mode dense SGD 11.459 vs manual analytical
derivatives 0.966. Graph validation/ownership adds semantics missing from that manual
baseline. These gaps remain on the general algebra/performance roadmap.

No GPU backend, SVD/eigen, sparse/complex implementation or complete-model 120 Hz
measurement is claimed. No floating algorithm promises exact real arithmetic.
