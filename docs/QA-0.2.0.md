# QA: 0.2.0

401 all-feature tests passed, including strict configuration, external opaque device
transfers, contiguous/indexed kernel bit equality and reusable output allocation tests.
Default-free tests, strict Clippy/rustdoc/format checks, Rust 1.86 library compilation,
wasm32 and i686 checks passed. See immutable benchmark results for measurements.

The row/inner/column GEMM traversal preserves increasing-k scalar accumulation;
parallel contexts distribute independent rows without changing those scalar sums.
The rowwise softmax path agrees exactly with checked logical indexing for tested
finite inputs and reports the same coordinates for nonfinite data.

The reusable-output path retains the physical allocation and validates geometry
before writes. Rust scalar panics can still leave partial output; this is documented.

Configuration is explicit: missing optional files choose defaults, malformed files
and unknown keys fail. CPU thread counts above one require the parallel feature.
Opaque-device tests implement transfers outside the core module without HostBuffer
and reject a buffer owned by another device. No GPU implementation is claimed.

Measured serial matmul 128x128 p50: 382 us vs 3312 us in 0.1.1; ndarray 89 us.
Shared-machine frequency/load are uncontrolled; timings are not a universal promise.
Reports include allocation and reuse cases, all raw samples and printable PDF.
