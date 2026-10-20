# Kernix0.7.1 verification

This patch follows the0.7 training profile: contiguous layer norm allocated general
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
tests, strict formatting/Clippy/rustdoc and13 downstream Markdown examples.
Independent Candle/stage tests pass (five normal, five with allocation profiling),
as do strict default/all-feature harness Clippy and eight Python checks. The actual
no_std consumer executes two workflows on Rust1.86 and compiles on wasm/i686/
aarch64; the workspace MSRV check passes. Three selected consumer Miri cases pass.

Clean measured source `4f60380` has crates tree
`f948102931f8b83beaac963c8017acf3c064998f`; clean before/after status, accuracy
budgets, samples, allocation probes, RSS, actual locks and source archives are retained:

- [Workspace0.7.1](../benchmarks/results/0.7.1-clean-4f60380/README.md):50 cases.
- [Exact registry0.7.0](../benchmarks/results/0.7.0-registry-4f60380/README.md):50 cases,
  all three packages pinned0.7.0 with VCS `b98ee37`. Comparator versions match.

For normalized/sliced training, per-call medians in microseconds were:

| Stage | Registry0.7.0 | Workspace0.7.1 | Rust allocation calls before/after |
|---|---:|---:|---:|
| Inference | 4.364 | 2.026 | 34 /17 |
| Recording/forward | 4.864 | 2.525 | 39 /22 |
| Backward | 9.065 | 3.026 | 71 /23 |
| Fixed-gradient SGD | 0.481 | 0.536 | 5 /5 |

These are allocation-inclusive CPU stages on one shared machine, not universal
speed rankings or a complete model latency guarantee. SGD was not changed; its
slower observation and every other win/loss remain in the full reports. The timing
binary is uninstrumented and the profiling binary is separate. Private local notes
are absent from both source archives. Published payload/source IDs, registry-only
validation and remote CI are recorded when those gates finish.
