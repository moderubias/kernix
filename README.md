# Kernix

Kernix is an experimental Rust library for building N-dimensional tensor and numerical computing systems.

The project is under active development. Its current foundation focuses on shape representation, validation, and broadcasting.

## Current functionality

- `Kahen` — dynamic-rank shapes.
- `Kotei<N>` — compile-time fixed-rank shapes.
- Checked dimension and element-count validation.
- Axis normalization and dimension access.
- Shape permutation and singleton-axis transformations.
- Broadcasting between dynamic- and fixed-rank shapes.
- Explicit shape errors.

Tensor storage, numerical operations, and execution backends are planned but are not yet provided by the public API.

## Workspace

| Crate | Purpose |
|---|---|
| `kernix` | Public-facing API |
| `kernix-core` | Core shape primitives and invariants |
| `kernix-ops` | Numerical and tensor operations (under development) |
| `kernix-traits` | Numerical and algebraic traits |

## Compatibility

Rust 1.86 or later, Edition 2024.

## Licensing

Kernix uses **Kiyoshima Open**, a project-specific licensing profile.

The controlling software license is **MIT OR Apache-2.0**, at your option. Kiyoshima Open does not introduce additional license restrictions.

See [Licensing](docs/README-LICENSING.md) and [KIYOSHIMA.json](KIYOSHIMA.json).

Copyright (c) 2026 Akayo Kiyoshima.
