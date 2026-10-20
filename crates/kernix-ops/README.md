# kernix-ops

Portable CPU tensor operations for [Kernix](https://github.com/moderubias/kernix).
Use `kernix = "0.4.0"` for the complete public facade, or add `kernix-ops` and
`kernix-core` directly. The crate supports `no_std` with `alloc`; the `std` feature
is enabled by default. No BLAS, GPU driver or system math library is required.

```rust
use kernix_core::{Order, shape::Kahen, tensor::Tensor};
use kernix_ops::softmax;
fn main() -> Result<(), Box<dyn std::error::Error>> {
let logits = Tensor::from_vec(vec![1.0f64, 2.0, 3.0], Kahen::new([3])?, Order::C)?;
let probabilities = softmax(&logits, -1)?;
assert!((probabilities.storage_slice().iter().sum::<f64>() - 1.0).abs() < 1e-14);
Ok(())
}
```

Inputs use logical indexing, so strided, reversed, C/F-order and broadcast views
have the same mathematical contract. Outputs allocate C-contiguous CPU storage.
Native arithmetic follows Rust scalar semantics; use `checked_*` for checked
integers. Finite-only reductions/softmax return indexed errors on nonfinite data.
Floating precision is finite, and no implicit dtype promotion occurs.

The API includes arithmetic/maps, reductions/statistics, batched matmul,
contractions, structural transforms, indexing, sorting and stable activations.
See https://docs.rs/kernix-ops for contracts and runnable function examples.
Dense f32/f64 GEMM uses checked native Rust kernels; ordered reference
matmul and compensated sums remain explicit choices. `linalg` provides pivoted LU,
Cholesky, Householder QR, solves, inverse/determinants and least squares. `neural`
provides convolution/pooling/losses; `vision` provides borrowed RGB preprocessing.
The facade adds explicit autodiff, trainable layers, optimizers and checkpoints.
There is no GPU backend. Native kernels can change floating accumulation order.
Large dense f32/f64 products on x86-64/std may use serial gemm engines with runtime
SIMD checks. Packing can allocate and retain thread-local scratch until thread exit;
ordered matmul avoids native packing. See the [numerical contracts](https://github.com/moderubias/kernix/blob/main/docs/NUMERICS.md)
for selection, rounding and memory costs.
Developed with OpenAI Codex (GPT-6 agent), under human direction.

Kiyoshima Open profile: MIT OR Apache-2.0. See LICENSE, LICENSES and KIYOSHIMA.json.
