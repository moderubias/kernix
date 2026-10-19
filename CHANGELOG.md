# Changelog

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
