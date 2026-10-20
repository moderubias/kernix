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

Clean before/after evidence and publication/remote-CI checks are recorded after
their corresponding gates finish.
