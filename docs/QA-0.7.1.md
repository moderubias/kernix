# Kernix 0.7.1 verification

This patch follows the 0.7 training profile: contiguous layer norm allocated general
reduction states and several full-sized intermediates. Nonempty C-order input/seed
now uses per-group loops and one output buffer, with stack statistics. High-rank
shape metadata may still allocate. The general indexed/strided/empty path remains.
No new API, dtype conversion, dependency, device behavior or unsafe code is added.

The same Welford mean/variance and gradient-mean recurrence, scaled standard
deviation and finite/range errors are retained. Direct/indexed regressions compare
exact tested f32/f64 outputs/VJPs across tail ranks, singleton/high-rank shapes,
constant groups/tiny epsilon, extreme statistics, nonfinite values and empty/scalar
validation. Existing finite differences, independent Candle graphs and actual
batched model training remain release gates. No arbitrary-range accuracy or
universal zero-cost claim follows from removing temporary buffers.

Local checks passed: **516 default, 518 all-feature, 509 no-default** workspace
tests, strict formatting/Clippy/rustdoc and 13 downstream Markdown examples.
Independent Candle/stage tests pass (five normal, five with allocation profiling),
as do strict default/all-feature harness Clippy and eight Python checks. The actual
no_std consumer executes two workflows on Rust 1.86 and compiles on wasm/i686/
aarch64; the workspace MSRV check passes. Three selected consumer Miri cases pass.

Clean measured source `4f60380` has crates tree
`f948102931f8b83beaac963c8017acf3c064998f`; clean before/after status, accuracy
budgets, samples, allocation probes, RSS, actual locks and source archives are retained:

- [Workspace 0.7.1](../benchmarks/results/0.7.1-clean-4f60380/README.md): 50 cases.
- [Exact registry 0.7.0](../benchmarks/results/0.7.0-registry-4f60380/README.md): 50 cases,
  all three packages pinned 0.7.0 with VCS `b98ee37`. Comparator versions match.

For normalized/sliced training, per-call medians in microseconds were:

| Stage | Registry 0.7.0 | Workspace 0.7.1 | Rust allocation calls before/after |
|---|---:|---:|---:|
| Inference | 4.364 | 2.026 | 34 /17 |
| Recording/forward | 4.864 | 2.525 | 39 /22 |
| Backward | 9.065 | 3.026 | 71 /23 |
| Fixed-gradient SGD | 0.481 | 0.536 | 5 /5 |

These are allocation-inclusive CPU stages on one shared machine, not universal
speed rankings or a complete model latency guarantee. SGD was not changed; its
slower observation and every other win/loss remain in the full reports. The timing
binary is uninstrumented and the profiling binary is separate. Private local notes
are absent from both source archives. All three crates are published at 0.7.1 from clean source
`533b6932ff2bca68e943f3b2ae06e2a65b36d936`. Downloaded registry archives contain
48/53/27 files (core/ops/facade); their VCS metadata and source/test/example/README
bytes match. Local research and benchmark files are excluded. License verification
passed. Archive SHA-256:

| Package | SHA-256 |
|---|---|
| kernix-core | `d03ae0d9733ce1a562280990627516a0c5cce0d57af53524f23ca273b317f9a6` |
| kernix-ops | `e0c39f3a4ca3a5b05b366fedbe75334933986b55a2e6860b2f7e527eedf2f5b0` |
| kernix | `7d171bc5fb334c99c1da2e5ba570dd3a2f09119170b4d439779348867679d75f` |

A registry-only renamed `no_std` consumer executes on Rust 1.86 and compiles for
wasm with stable Rust. Release-source [CI](https://github.com/moderubias/kernix/actions/runs/39468319585)
passed all eight jobs, including Windows/macOS/Linux, MSRV and portable checks.
