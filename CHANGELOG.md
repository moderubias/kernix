# Changelog

## 0.5.0

- Add fallible `tensor!`, `matrix!`, `full!` and `shape!` literals and a focused
  `prelude` for concise numerical code, including no_std + alloc applications.
- Add C-order `Tensor::from_values`, `Tensor::from_rows` and `Tensor::full`;
  existing Vec allocations are adopted without copying elements. Matrix literals
  reject ragged rows at compile time and accept elements without Clone or Copy.
- Reduce Householder QR work when materializing Q and use independent dot-product
  lanes plus a single-product reflector update with extreme-range fallbacks.
  Last-bit rounding may change; reconstruction/orthogonality/least-squares bounds
  and nonfinite rejection remain checked.
- Add independent QR oracle/property tests and comparative tensor-construction
  benchmarks; simplify the getting-started examples without changing backend traits.

## 0.4.0

- Add general dense algebra: reusable pivoted LU/Cholesky/Householder QR,
  square and least-squares solves, inverse, determinant/slogdet and residuals.
- Accelerate f32/f64 rank-two GEMM, including signed strides, through checked
  matrixmultiply kernels; retain explicit ordered reference arithmetic.
- Add pairwise/ordered/compensated sum policies, cached softmax exponentials,
  std/libm math selection, bounded tiled GEMM convolution and hoisted RGB coordinates.
- Add explicit reverse-mode tapes, broadcast VJPs, shared parameters and detach.
- Add trainable dense/convolution layers, seeded Xavier, SGD, Adam/AdamW and clipping.
- Add checked versioned named weight/optimizer checkpoints with exact resume tests.
- Add generic synthetic training and temporal-state examples, detailed user/numerics
  guides, retained nalgebra/ndarray/NumPy comparisons and exact source snapshots.
- Native kernels can round differently from ordered scalar arithmetic; choose the
  documented reference APIs when accumulation order matters. Decompositions are
  inference APIs; SVD, sparse/complex algebra and GPU execution remain future work.

## 0.3.0

- Add NCHW grouped/dilated convolution and explicit input/weight gradients.
- Add average/max pooling and VJPs, affine transforms, layer norm and stable losses.
- Add borrowed/cropped RGB frames and fused bilinear resize/normalization.
- Fix optional config paths on Windows by distinguishing directories from absent files.
- Add Full HD preprocessing and convolution comparison reports.

## 0.2.0

- Add explicit Device transfers, MatmulDevice capability and private CPU executors.
- Implement strict optional kernix.toml loading with defaults, no ambient global state.
- Improve contiguous matmul locality and rowwise softmax while preserving scalar order.
- Add allocation-reusing matmul_into and serial/parallel conformance tests.
- Retain comparative reports against ndarray/NumPy for both allocation policies.

## 0.1.1

- QA: fix representable temperature-scaled softmax differences lost through
  intermediate subtraction overflow; define the SiLU negative-infinity limit.
- Complete shape removal, const scalar, Vec conversion, Display and validated serde.
- Export portable operations from the facade; collapse the unused traits boundary.
- Document public APIs and add executable examples, strict documentation checks,
  operating-system CI and target compilation checks.
- Install Kiyoshima Open MIT OR Apache-2.0 metadata in all publishable packages.
- Add reproducible, versioned ndarray/NumPy comparisons with raw data and PDF reports.
