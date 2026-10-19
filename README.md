# Kernix

**An experimental, type-safe N-dimensional tensor toolkit for Rust.**

Kernix provides a device-independent Shape/Layout model, checked CPU storage,
zero-copy tensor views and an expanding `no_std`-capable collection of CPU
reference operations. The library is under active development; **SIMD/GPU
execution, autodiff and a complete ML/CV stack are not yet implemented**.

## API at a glance

```rust
use kernix::{Kahen, Order, Tensor};
use kernix_ops::{add, softmax};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let shape = Kahen::new([2, 3])?;
    let a = Tensor::from_vec(vec![1.0f32, 2.0, 3.0, 4.0, 5.0, 6.0], shape, Order::C)?;
    let b = Tensor::from_vec(vec![0.5f32, 1.0, -0.5], Kahen::new([3])?, Order::C)?;
    let result = add(&a, &b)?;         // NumPy-style right-aligned broadcasting
    let probabilities = softmax(&result, -1)?;
    assert_eq!(probabilities.shape().dims(), &[2, 3]);
    Ok(())
}
```

The high-level `kernix` crate exports `Kahen`, `Kotei<N>`, `Shape`, `Layout`,
`Storage`, `Tensor` and their errors. Numerical functions currently live in the
separate `kernix-ops` crate; add **both dependencies** when using the example.

`Kahen` (可変, variable rank) and `Kotei` (固定, fixed rank) are intentional
Japanese-inspired type names. Common mathematical functions keep their
industry-standard English names.

## Crates

| Crate | Responsibility |
|---|---|
| `kernix` | Public facade for core types |
| `kernix-core` | Shape, Layout, Storage, Tensor and borrowed views |
| `kernix-ops` | Portable CPU reference operations on logical tensor indices |
| `kernix-traits` | Numeric/algebraic traits |

## Important semantics

- **Storage order differs from logical order.** Tensor operations respect C/F
  layout, transpositions, negative strides and broadcasting strides.
- **Views can be zero-copy.** Structural operations that return owned tensors
  (including `concat`, `scatter_axis`, `tile`, `unfold_axis`) allocate and copy.
- **Native integer arithmetic** (`add`, `matmul`, reductions) retains Rust's
  normal overflow semantics. Use `checked_add/sub/mul/div/rem/neg` where
  fallible checked integer arithmetic is required.
- **Float ordering:** `sort_axis`, `argsort_axis`, `topk_axis` use IEEE total
  ordering for `f32/f64`, including NaN and signed zero; argmin/argmax use
  the separate, documented first-NaN policy.
- `cast` relies on Rust `From` (infallible). `try_cast` uses `TryFrom` and
  returns the first failing logical index. No implicit dtype promotion.
- `kernix-ops` is a **CPU reference layer**, not yet a benchmark-proven
  high-performance kernel engine.

For the complete function inventory and remaining work, see
[`docs/CAPABILITY-MATRIX.md`](docs/CAPABILITY-MATRIX.md).

## Verify locally

```sh
cargo fmt --all -- --check
cargo test --workspace --all-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo check -p kernix-core --no-default-features
cargo check -p kernix-ops --no-default-features
```

The required Rust minimum version is **1.86** (Edition 2024). Core currently
supports `no_std` with `alloc`; this is *not* a claim of allocation-free or
embedded-target compatibility.

## License

Dual-licensed under **MIT OR Apache-2.0**. See [licensing](docs/README-LICENSING.md)
and [KIYOSHIMA.json](KIYOSHIMA.json). The "Kiyoshima Open" profile does not
add restrictions beyond the controlling licenses.
