# Kernix

Kernix is an experimental Rust library for linear algebra, N-dimensional tensors
and extensible computation. Its foundation separates validated shapes, affine
layouts, storage ownership and numerical operations. The default implementation
runs on the CPU and needs no BLAS installation, GPU driver or configuration file.

The project is being developed with **OpenAI Codex (GPT-6 agent)**, under human
direction and review. Generated code is subject to the same tests and numerical
checks as other contributions. This is early software, with an evolving API.

## Install

Install stable Rust and Cargo from [rustup](https://rustup.rs), then:

```sh
cargo new tensor-demo
cd tensor-demo
cargo add kernix
```

```toml
[dependencies]
kernix = "0.10.0"
```

The declared minimum Rust version is 1.86, using Edition 2024. Stable Rust is
recommended. A C/C++ compiler or native linear algebra runtime is not needed by
Kernix itself; the Rust target still needs its normal linker/system SDK.

| System | Setup |
|---|---|
| Linux (Arch) | `sudo pacman -S rustup base-devel`, then `rustup default stable` |
| Linux (Debian/Ubuntu) | Install `build-essential`, then Rust with rustup |
| Windows | Install rustup and Visual Studio Build Tools with Desktop development with C++ / Windows SDK for the default MSVC target |
| macOS | Install Command Line Tools (`xcode-select --install`), then rustup |
| Other Rust targets | Use the target toolchain/linker; core and ops support `no_std` with an allocator |

Linux x86_64 is verified locally. Release 0.6.0 CI passed on Windows, macOS and Linux.
WebAssembly, ARM and 32-bit availability depends on their Rust toolchain, allocator
and selected features; a portable API is not evidence that every target was tested.
No CUDA, ROCm, SIMD-specific or OS-specific library is a mandatory dependency.
Optional future backends must not prevent use of the portable CPU path.

For an allocator-equipped `no_std` application:

```toml
kernix = { version = "0.10.0", default-features = false }
```

`no_std` does not mean allocation-free. Shapes above rank six, tensor outputs and
many operations allocate. CPU storage rejects zero-sized element types.

## Solve a linear system

```rust
use kernix::prelude::*;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let a = matrix![4.0f64, 1.0; 1.0, 3.0]?;
    let b = tensor![1.0, 2.0]?;
    let x = solve(&a, &b)?;
    assert!(residual_norm(&a, &x, &b)? < 1e-14);
    Ok(())
}
```

`Lu` reuses partial-pivoted factors for multiple solves, inverse, determinant and
signed log determinant. `Cholesky` factors symmetric positive-definite systems.
`Qr` exposes thin Householder factors and full-column-rank least squares without
forming normal equations. Both f32 and f64 support arbitrary input strides.
Pivot tolerances are explicit rejection policies; a small residual does not prove
small solution error for an ill-conditioned matrix. SVD, eigenproblems, sparse and
complex algebra are planned extensions, rather than available APIs.

## First tensor computation

```rust
use kernix::{prelude::*, softmax};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let a = matrix![1.0f32, 2.0, 3.0; 4.0, 5.0, 6.0]?;
    let bias = tensor![0.5, 1.0, -0.5]?;
    let logits = add(&a, &bias)?;
    let probabilities = softmax(&logits, -1)?;
    assert_eq!(probabilities.shape().dims(), &[2, 3]);
    assert_eq!(a.get(&[1, 2])?, &6.0);
    Ok(())
}
```

Use `kernix::ops::add` or the root `kernix::add`; both are the same function.
The [API reference](https://docs.rs/kernix) describes individual functions and
contains executable examples. See also [core](https://docs.rs/kernix-core) and
[operations](https://docs.rs/kernix-ops).

## Tensor literals

Start with `use kernix::prelude::*;`. Literals return `Result`, so use `?` to
propagate invalid shapes, storage extents and other construction errors.

```rust
use kernix::prelude::*;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let vector = tensor![1, 2, 3]?;
    let repeated = tensor![7; 4]?;
    let matrix = matrix![1, 2; 3, 4]?;
    let zeros = Tensor::<f32>::zeros([2, 3])?;
    let ones = Tensor::<f32>::ones([3])?;
    let scalar = full!(42;)?;
    let shape = shape![2, 3]?;
    let volume = tensor!(shape = [1, 2, 2]; values = vec![1, 2, 3, 4])?;
    assert_eq!(matrix.get(&[1, 0])?, &3);
    assert_eq!(volume.shape().dims(), &[1, 2, 2]);
    Ok(())
}
```

`matrix!` checks equal row lengths at compile time. List literals move elements
without requiring Clone; `full!` and repetition clone one value. All tensor literals
create owned C-order CPU storage. The explicit `values` form adopts its Vec without
copying elements. Functions `Tensor::from_values(dims, values)`,
`Tensor::from_rows(rows)` and `Tensor::full(dims, value)` provide the same behavior
for adapters and programmatic dimensions. For column-major data, explicit alignment,
fixed rank or device storage, use the existing constructors and capability traits.
The macros also work with `no_std` plus an allocator and no caller-side `alloc` import.

See the [0.9 capability matrix](https://github.com/moderubias/kernix/blob/main/docs/CAPABILITY-MATRIX-0.9.md)
and [forward/VJP coverage](https://github.com/moderubias/kernix/blob/main/docs/AUTOGRAD-COVERAGE.md)
for available APIs and their exact limits.

`Tensor::zeros` and `Tensor::ones` generate numeric identities without Clone or
Default bounds. Existing `kernix::Zero`/`One` implementations work; core-only
adapters can implement `kernix_core::scalar::{Zero, One}`. Validation precedes element
generation. These constructors allocate C-order CPU storage; explicit order/alignment
remain available through `from_vec`, `filled` and `from_fn`.

Nested arrays remain scalar elements: `tensor![[1, 2], [3, 4]]?` has shape `[2]`
and element type `[i32; 2]`. For a scalar matrix, use `matrix![1, 2; 3, 4]?` or
`Tensor::from_rows([[1, 2], [3, 4]])?`. Higher-rank construction uses explicit
`Tensor::from_values(dims, values)`. No existing literal silently flattens array elements.
See the [executable ergonomics suite and design audit](https://github.com/moderubias/kernix/blob/main/docs/API-ERGONOMICS.md).

Training uses the same vector/batched matmul geometry: `Tape::matmul` and
`Linear::record` accept leading batch/sequence axes. `Tape::sum_axes`, `mean_axes`
`layer_norm` and signed-step `slice_axis` compose with broadcast scale/bias and ordinary optimizers.
Recording is explicit and first-order; unsupported operations never silently detach.
See [the training guide](https://github.com/moderubias/kernix/blob/main/docs/USER_GUIDE.md#batched-and-normalized-training).

## Shapes

`Kahen` means variable in Japanese: its rank is determined at runtime.
`Kotei<N>` means fixed: its **rank**, not its dimension sizes, is fixed at compile
time. Method names and documentation are English. Both types validate dimensions
at construction and expose only immutable dimension slices.

```rust
use kernix::{Kahen, Kotei};
fn main() -> Result<(), Box<dyn std::error::Error>> {
let image = Kotei::<4>::new([1, 3, 224, 224])?;
let dynamic = Kahen::from(image);
assert_eq!(dynamic.dim(-1)?, 224);
assert_eq!(dynamic.unsqueeze(0)?.squeeze_axis(0)?, dynamic);
assert_eq!(Kahen::scalar().numel(), 1);
assert_eq!(Kahen::new([usize::MAX, 2, 0])?.numel(), 0);
assert_eq!(Kahen::new([2, 1])?.broadcast(&Kahen::new([1, 3])?)?.dims(), &[2, 3]);
Ok(())
}
```

Shapes know nothing about dtype, device, strides or memory order. Shape transforms
include insertion/removal, squeezing, permutation and broadcasting. `remove_axis`
can change element count, while `squeeze_axis` requires a singleton. Removing a
zero axis can expose overflow and therefore returns an error. A scalar has rank
zero and one element; a shape with any zero dimension has zero elements.

Broadcasting aligns trailing axes: equal sizes are retained, and a singleton
takes the other size (including zero). `can_broadcast_to` is directional; it does
not ask whether two shapes merely have a common broadcast result.

With `features = ["serde"]`, shapes serialize as dimension sequences and validate
rank/overflow during deserialization. Invalid metadata cannot bypass constructors.

## Layouts and views

`Order::C` is row-major; `Order::F` is column-major. Strides are **element** strides,
may be negative or zero, and include an origin within storage. Tensor operations
read logical elements, respecting permutations, gaps, reverse slices and broadcast
views. Inputs never silently change dtype or device.

```rust
use kernix::{Kahen, Order, Tensor};
fn main() -> Result<(), Box<dyn std::error::Error>> {
let tensor = Tensor::from_vec(vec![1, 2, 3, 4, 5, 6], Kahen::new([2, 3])?, Order::C)?;
let transposed = tensor.transpose()?; // owns metadata, borrows the same elements
assert_eq!(transposed.shape().dims(), &[3, 2]);
assert_eq!(transposed.get(&[2, 1])?, &6);
let copy = transposed.to_owned_contiguous(Order::C)?;
assert_eq!(copy.storage_slice(), &[1, 4, 2, 5, 3, 6]);
Ok(())
}
```

`reshape_view` belongs to Layout/Tensor and fails when stride chunks cannot
represent the requested traversal without copying. `storage_slice` exposes the
physical backing, which may include gaps and extra elements. Use logical iteration
or `to_owned_contiguous` when you need logical contents. Mutable views hold an
exclusive borrow; in-place numerical operations reject unproven disjointness.

## Operations and numerical behavior

The portable CPU API includes:

- Elementwise arithmetic, checked integer arithmetic, user-defined maps, comparisons,
  Boolean masks and selection, casts and in-place updates.
- Sum/product/mean/extrema, axis reductions, argmin/argmax, nonzero counts, cumulative
  sums/products, Welford variance/std, scaled L2 norms and stable log-sum-exp.
- Vector products, batched/vector/matrix `matmul`, `tensordot`, outer products and trace.
- Concatenation, stacking, gather/scatter, padding, repeat/tile, splits, rolling,
  sliding windows and overlap-add folding.
- Stable sorting, argsort, top-k, elementary math and activations, temperature/masked
  softmax and online normalization variants.

Allocating operations return C-order CPU tensors. Free functions accept owned or
borrowed tensors through `TensorInput`, including fixed-rank and strided inputs.
Scalar arithmetic follows the element type: native integer operations can panic on
overflow in debug or wrap in release. Use `checked_*` for checked integer errors.
Floating operations round according to the dtype; no implicit dtype promotion,
fast-math policy or promise of exact real arithmetic is made.

Softmax/log-sum-exp reject nonfinite logits with coordinates. Elementwise functions
follow their documented IEEE domain behavior; for example `ln(-1)` is NaN.
Extrema and arg-extrema have documented NaN/tie policies. Float sorting uses IEEE
total ordering, which includes NaNs and signed zeros. Reduction order can differ
from NumPy/BLAS and produce different last bits. Numerical tests use explicit
tolerances rather than claiming 100% mathematical precision.

## Architecture and extension

| Crate | Responsibility |
|---|---|
| `kernix` | User-facing entry point, core types and numerical functions |
| `kernix-core` | Shapes, affine layouts, opaque storage and tensors |
| `kernix-ops` | Portable CPU implementations and host tensor contracts |

`StorageBuffer<T>` exposes element count without requiring a host pointer.
`HostBuffer<T>` is an additional capability for memory safely borrowable on CPU.
An accelerator can supply opaque storage without teaching Shape about GPUs or
pretending a device pointer is a CPU slice. Device transfers and matrix multiplication are separate capabilities: `Device<T>`
provides opaque storage and explicit upload/download/synchronization;
`MatmulDevice<T>` adds execution. `CpuExecutor` implements both. Future GPU
backends implement their own capabilities without requiring `HostBuffer`.

`CpuExecutor::matmul_into` writes contiguous rank-two results into an existing
allocation. The allocating API retains the general strided/batched fallback.
f32/f64 GEMM, including contiguous vector/batched products, uses checked native Rust kernels, including signed
strides. These Rust engines require no external BLAS. Native packing can allocate;
large dense x86-64/std products may retain thread-local scratch until thread exit.
Output-buffer reuse does not make native execution scratch-free. Native SIMD/FMA and packed
summation can change last bits. `matmul_reference` or
`CpuExecutor::default().with_ordered_matmul(true)` selects increasing-k scalar order.
`sum_all` uses pairwise contiguous reduction; `sum_sequential_all` fixes logical order;
`sum_compensated_all` uses Neumaier compensation for finite inputs.
`portable-math` selects libm even with std; otherwise std uses the platform math
implementation. Neither setting promises bitwise identity on every architecture.

This release includes CPU decompositions, reverse-mode differentiation, trainable
layers and checked checkpoints. GPU execution, SVD, eigen and sparse algebra remain
future work. Applications need not create a configuration file. With the default `config`
feature, an application can explicitly load optional `kernix.toml`:

```toml
[cpu]
threads = 1
ordered_matmul = false
```

```rust
use kernix::config::Context;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let context = Context::from_file("kernix.toml")?;
    assert!(context.cpu().threads() >= 1);
    // Use context.cpu().matmul(&a, &b) to apply this scheduling policy.
    Ok(())
}
```

A missing optional file uses defaults. Malformed TOML, unknown keys and zero workers
are errors. `Context::default()` never reads files. Multiple workers require
`features = ["parallel"]`; each context owns a private pool and changes no global
settings. Parallel matmul currently accelerates only contiguous rank-two inputs.
Free functions use a serial execution policy and do not consult ambient
configuration. Native SIMD/FMA selection and platform math can affect rounding. `config` requires `std`; disable default features
for `no_std` use.

## Vision and neural building blocks

`kernix::neural` provides grouped NCHW convolution with stride/padding/dilation,
input/weight VJPs, average/maximum pooling and VJPs, dense affine transforms,
layer normalization and mean classification/regression losses. Convolution uses
cross-correlation (no kernel flip), as is usual in neural networks. Weights are
`[out_channels,in_channels/groups,kh,kw]`; biases can be broadcast-added separately.

`kernix::vision::RgbFrame` borrows packed RGB8 bytes or padded rows. A crop is a
borrowed view, and bilinear resize fuses RGB normalization into a C-contiguous
`[1,3,height,width]` f32 tensor. It uses half-pixel sampling with edge clamping.
There is no full-resolution floating intermediate. Capture and decoding remain
application choices, allowing the same interface on different operating systems.

```rust
use kernix::vision::RgbFrame;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pixels = vec![128u8; 1920 * 1080 * 3]; // synthetic RGB frame
    let frame = RgbFrame::new(&pixels, 1920, 1080)?;
    let input = frame.resize_nchw([224,224], [0.0;3], [1.0;3])?;
    assert_eq!(input.shape().dims(), &[1,3,224,224]);
    Ok(())
}
```

Losses accept finite data with documented domains: integer labels for categorical
cross-entropy, targets in `[0,1]` for binary cross-entropy with logits, and identical
shapes for MSE. Cross-entropy scales before otherwise overflowing logit differences
when the final mean remains representable. Pooling ties/NaNs use first-winner rules.
See the neural API examples and gradient checks for exact contracts. These are
building blocks; model architecture, training data and application behavior belong
to the caller.

## Training and explicit temporal state

`autograd::Tape` records supported operations with explicit graph ownership.
`training::Parameter`, seeded Xavier initialization, SGD, Adam/AdamW and finite-gradient
clipping provide training primitives. `nn::Linear` and `nn::Conv2d` support both direct
inference and recording. Shared parameters record once per tape; clear or create a
new tape after updating weights. Checkpoints preserve named weights and optional
Adam state in a versioned, bounded, checked f32/f64 format.

```sh
cargo run -p kernix --example train_patterns
cargo run -p kernix --example temporal_state
```

The first example learns synthetic stripe classes, then saves and restores a
checkpoint. The second carries an explicit hidden-state tensor between observations.
They illustrate composition rather than prescribing an application model.
Read the [user guide](https://github.com/moderubias/kernix/blob/main/docs/USER_GUIDE.md)
and [numerical contracts](https://github.com/moderubias/kernix/blob/main/docs/NUMERICS.md)
for graph lifetime, gradients, serialization, precision and performance choices.
Not every free numerical operation has an autodiff counterpart; unsupported operations
are explicit, and solvers are currently inference-only. Graph recording validates
and owns tensors, so it has overhead compared with hand-written derivatives.

## Tests, benchmarks and development

```sh
git clone https://github.com/moderubias/kernix.git
cd kernix
cargo fmt --all -- --check
cargo test --workspace --locked
cargo test --workspace --all-features --locked
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps
cargo check -p kernix --no-default-features --locked
```

Every public API is subject to `missing_docs = deny`. Doctests compile and execute
examples. Property tests check shape invariants; regression tests cover layouts,
aliasing, integer errors, nonfinite math and extreme softmax inputs. Unsafe aligned
storage has a Miri CI workflow; a configured workflow does not certify an unrun test.

Comparative benchmarks require Python 3 + NumPy + Matplotlib for measurement and report
generation; none is a runtime dependency. On Arch: install `python-numpy python-matplotlib`; on other systems install equivalent tools.

```sh
python benchmarks/evidence.py my-unique-run --require-openblas
```

Reports include a compact comparison table, colored PNG/PDF charts and a standalone
HTML view with filtering and expandable details. Pair versions with
`python benchmarks/report.py <current-run> --baseline <baseline-run>`.

The evidence runner requires a clean commit and saves uniquely named local results
with source/tree IDs, lockfiles and hashes. It records separate per-call and
batch-mean distributions, first-use observations, allocation counts outside timing
and process peak RSS. It compares matching CPU work with ndarray, nalgebra, faer,
Candle and an explicitly selected NumPy backend. See the
[methodology](https://github.com/moderubias/kernix/blob/main/docs/BENCHMARKING.md)
for boundaries and commands. Official reports are retained in
[`benchmarks/results`](benchmarks/results); `benchmarks/run.py` reproduces the older
batch-only reports. A shared laptop benchmark is not a universal performance guarantee
or evidence that an unspecified complete model reaches 120 FPS.

## License

Kiyoshima Licensing **Open profile**, **MIT OR Apache-2.0**, at your option.
[LICENSE](LICENSE) declares the license; complete texts are in [LICENSES](LICENSES).
[KIYOSHIMA.json](KIYOSHIMA.json) records the machine-readable passport. The profile
adds no restrictions to the controlling standard licenses. Verify with
`klicense verify-project .` using the Kiyoshima licensing utility.
