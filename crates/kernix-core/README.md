# kernix-core

Validated shapes, affine layouts, opaque backend storage and checked tensor views.
Part of [Kernix](https://github.com/moderubias/kernix); add `kernix = "0.2.0"` for
the facade and numerical operations. Core supports `no_std` with `alloc`; no GPU,
BLAS or platform-specific runtime is required. CPU element types must be nonzero-sized.

```rust
use kernix_core::{Order, shape::{Kahen, Kotei}, tensor::Tensor};
fn main() -> Result<(), Box<dyn std::error::Error>> {
let shape = Kotei::<2>::new([2,3])?;
let a = Tensor::from_vec(vec![1,2,3,4,5,6], shape, Order::C)?;
assert_eq!(a.transpose()?.get(&[2,1])?, &6);
assert_eq!(Kahen::from(shape).numel(),6);
Ok(())
}
```

Kahen has dynamic rank, Kotei<N> fixed rank. A scalar has rank zero and one element;
any zero dimension yields zero elements. Nonzero dimension products cannot overflow.
Shape does not know about strides, reshape, dtype or devices. Layout handles signed
element strides, origin, broadcasting and no-copy view reshapes. StorageBuffer does
not require host pointers; HostBuffer is an explicit additional capability.

See https://docs.rs/kernix-core for API contracts, errors and executable examples.
The optional `serde` feature serializes shapes as validated dimension sequences.
Developed with OpenAI Codex (GPT-6 agent), under human direction.

Kiyoshima Open profile: MIT OR Apache-2.0. See LICENSE, LICENSES and KIYOSHIMA.json.
