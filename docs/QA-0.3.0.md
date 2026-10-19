# QA: 0.3.0

423 all-feature tests pass locally. Strict Clippy/rustdoc, no-default tests,
Rust 1.86 and wasm32 compilation pass. CI is required before publication.

New coverage: grouped/dilated/padded convolution finite differences for every
input and weight; strided convolution hand oracles; pooling overlap gradients,
first ties and NaNs; losses and normalization domains; extreme finite mean
cross-entropy; packed/padded/cropped RGB and half-pixel interpolation.
Benchmark correctness checks compare full preprocessing output and reference
convolution output with independently written ndarray implementations before timing.

Fixed Windows configuration handling discovered by 0.2.0 CI: directory reads can
report NotFound. Optional-file loading now first checks metadata and rejects
nonregular paths; read failures after an existing file is identified remain errors.
The existing directory regression test is retained unchanged.

See benchmarks/results/0.3.0 for raw data and printable comparisons. Preprocessing
1920x1080 into 224x224 is timed separately from convolution and frame capture.
This does not establish end-to-end model performance. No GPU or autodiff claim yet.
