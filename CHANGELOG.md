# Changelog

## 0.1.1

- QA: fix representable temperature-scaled softmax differences lost through
  intermediate subtraction overflow; define the SiLU negative-infinity limit.
- Complete shape removal, const scalar, Vec conversion, Display and validated serde.
- Export portable operations from the facade; collapse the unused traits boundary.
- Document public APIs and add executable examples, strict documentation checks,
  operating-system CI and target compilation checks.
- Install Kiyoshima Open MIT OR Apache-2.0 metadata in all publishable packages.
- Add reproducible, versioned ndarray/NumPy comparisons with raw data and PDF reports.
