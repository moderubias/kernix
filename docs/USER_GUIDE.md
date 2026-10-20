# Working with Kernix

Kernix is a general linear algebra and tensor library. Numerical analysis, data
processing, simulations and neural models use the same validated shape/layout
foundation. This guide follows the public 0.5 API; the linked rustdoc contains
function-specific types, errors and runnable examples. The project is experimental.

## Choose the layer

Use `kernix` for the complete API. Use `kernix-core` for shape, layout, storage and
tensor metadata without numerical algorithms. Use `kernix-ops` for CPU numerical
operations without graph recording or application configuration. All three support
`no_std` with allocation when default features are disabled. Filesystem checkpoints,
TOML loading and worker pools need their corresponding std features.

`use kernix::prelude::*;` imports common types, literals and algebra functions.
`matrix![1,2;3,4]?` creates a row-major matrix, `tensor![1,2]?` creates a vector,
and `full!(0.0f32;2,3)?` fills a tensor. All literals return Results. For runtime
dimensions and an existing Vec, use `Tensor::from_values(dims, values)`; this
adopts the allocation without copying elements.

`Kahen::new([2,3])` validates runtime rank/dimensions; `Kotei::<2>::new([2,3])`
fixes rank at compile time. Empty dimensions describe a scalar with one element.
A zero dimension describes an empty tensor. A zero anywhere makes the product zero,
even if an earlier prefix would overflow. No shape contains strides or a device.

`Tensor::from_vec(values, shape, Order::C)` interprets values in row-major order.
`Order::F` is column-major. Views borrow storage and retain affine strides;
transpose and reverse do not copy elements. `storage_slice()` exposes physical
storage, including gaps. For logical data use iteration, indexed access or
`to_owned_contiguous(Order::C)`. View lifetimes and mutable borrows follow Rust.

## Solve systems and fit observations

```rust
use kernix::prelude::*;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let a=matrix![4.0f64,1.0;1.0,3.0]?;
    let b=tensor![1.0,2.0]?;
    let lu=Lu::factor(&a)?;
    let x=lu.solve(&b)?;
    assert!(residual_norm(&a,&x,&b)?<1e-14);
    let (_sign,_log_absolute_determinant)=lu.slogdet();
    let _positive_definite_solution=Cholesky::factor(&a)?.solve(&b)?;
    let _least_squares=Qr::factor(&a)?.solve_least_squares(&b)?;
    Ok(())
}
```

Prefer a solve to explicitly multiplying by an inverse. Reuse LU/Cholesky factors
for new right-hand sides. RHS `[n]` returns a vector; `[n,k]` returns k solution
columns. LU uses partial pivoting. The default relative cutoff is epsilon times
matrix size, scaled by the largest input entry. `factor_with_tolerance` lets the
caller choose a cutoff; zero only rejects exact zero pivots. This is no condition
estimate. Inspect residuals and the scale of the problem.

Cholesky requires symmetric positive-definite input, checks both triangles and
adds no diagonal regularization. QR uses stable-sign Householder reflectors,
returns thin Q/R, and solves full-column-rank overdetermined least squares.
Rank-deficient and underdetermined solving need a different algorithm and currently
return errors. Empty matrices have explicit valid algebraic identities. All input
values and required decomposition results must be finite. f32 and f64 are supported.

## Execution and resource policy

Default functions operate on CPU data and do not read files or global settings.
f32/f64 contiguous vector/batched and rank-two matrix products use the Rust matrixmultiply engine with checked
slice/stride boundaries. Integer/custom scalar products, vector/batched products
and unsupported fast paths use portable scalar implementations. A downstream scalar
can implement `MatmulElement` without implementing a native kernel. `TensorInput`
adapters may expose validated `MatrixInput` for signed-stride acceleration; the
view must represent the same logical data as indexed access.

`CpuExecutor::matmul_into` accepts reusable C-order rank-two output storage.
`with_ordered_matmul(true)` chooses the increasing-k reference arithmetic.
With the `parallel` feature, `CpuExecutor::new(threads)` creates a private pool;
contiguous rank-two products may distribute rows. An application can create a
`config::Context` explicitly from optional `kernix.toml`:

```toml
[cpu]
threads = 1
ordered_matmul = false
```

Missing optional files use defaults. Typo keys, malformed values, directories and
zero workers are errors. `Context::default()` performs no I/O. There is no hidden
GPU selection. Device storage can be opaque: implement core `Device<T>` for explicit
transfers/synchronization, then operation capabilities such as `MatmulDevice<T>`.
A device address need not implement `HostBuffer` or masquerade as a CPU slice.
Actual CUDA/ROCm/Metal/WebGPU plugins and hardware validation remain future work.

## Record and differentiate

```rust
use kernix::{Kahen,Order,Tensor,autograd::Tape};
fn main()->Result<(),Box<dyn std::error::Error>> {
    let mut tape=Tape::<f64>::new();
    let x=tape.variable(Tensor::from_vec(vec![2.0,3.0],Kahen::new([2])?,Order::C)?)?;
    let square=tape.mul(&x,&x)?;
    let loss=tape.sum(&square)?;
    let gradients=tape.backward(&loss)?;
    assert_eq!(gradients.get(&x)?.unwrap().storage_slice(),&[4.0,6.0]);
    tape.clear();
    assert!(tape.value(&x).is_err());
    Ok(())
}
```

A tape owns values and history. `variable` enables differentiation; `constant`
does not. Variables from another tape or a cleared generation fail. `backward`
requires a rank-zero scalar; `backward_with` takes an exact-shape seed for a vector
Jacobian product. A fresh backward call starts fresh gradients; explicitly call
`Gradients::accumulate` when accumulating batches. Broadcast VJPs reduce expanded
axes to the original shape; shared DAG paths accumulate their contributions.

Available recording operations include add/sub/mul/div, exp/ln/sqrt, relu/tanh/
sigmoid, vector/batched matmul and last-axis linear, all/axis sum/mean, layer norm,
reshape/permutation/signed-step slices, softmax/log-softmax,
convolution, average/max pooling and MSE/categorical/binary-logit losses. General
decomposition derivatives, general integer gather/scatter VJPs and arbitrary callbacks are not recorded.
Domain failures or nonfinite gradients return errors rather than silently updating
parameters. ReLU at zero uses derivative zero; max-pool ties follow the documented
first-winner convention. Normalization has a free inference API, not a tape API.

## Update, save and resume

`training::Parameter` owns finite C-order weights and keeps a stable identity for
optimizer state. `SeededRng` provides reproducible initialization; `Parameter::xavier`
supports dense/convolution fan-in/fan-out. `nn::Linear` and `nn::Conv2d` expose
parameters, direct `forward` inference and `record` calls returning output and
parameter handles. Repeated recording of one parameter shares the first snapshot
on that tape. After updating weights, create or clear the tape before the next step.

Optimizers accept explicit `(parameter, gradient)` pairs. SGD, Adam and decoupled
AdamW validate all candidates before committing an update, avoiding partial changes
when a later gradient is invalid. `clip_grad_norm` supports finite owned C-order
gradients and a positive maximum norm; call it before passing gradients to an
optimizer. No hidden optimizer or training/evaluation global mode exists.

A complete executable example is `crates/kernix/examples/train_patterns.rs`:

```sh
cargo run -p kernix --example train_patterns
```

It builds a small generic convolutional classifier, learns synthetic stripe labels,
checks loss decrease, saves named weights/Adam state and restores them. Its training
accuracy is a software example, not a claim about real data or an application model.

`Checkpoint::capture` takes unique names and an optional Adam instance. `to_bytes`
and `from_bytes` work without std. `save` writes a sibling temporary file, synchronizes
it and renames it; filesystem/platform durability still depends on the filesystem.
`load` uses a 256 MiB default cap; `load_with_limit` sets an explicit cap. The format
records version, f32/f64 dtype, dimensions, names, values and optional optimizer
state. A checksum detects accidental corruption; it is not an authenticity signature.
Wrong dtype, malformed lengths, duplicate names, nonfinite weights, truncation,
unsupported versions and incompatible restore shapes fail. Restore validates the
whole requested parameter set before modifying it. Weight-only restore can be used
for inference; full restore reproduces the optimizer trajectory tested by QA.

## Compose temporal models and visual inputs

`crates/kernix/examples/temporal_state.rs` demonstrates a state tensor passed from
one observation to the next. A model can concatenate visual features, map features
and other observations, update hidden state and produce independent output heads.
Reset state between unrelated sequences. During training, retain a tape for the
chosen unroll window; use `detach` to carry a value without prior history when
truncating backpropagation. Graph growth is explicit and consumes memory.

`vision::RgbFrame` borrows platform-independent RGB8 bytes, accepts padded rows,
crops without copying and resizes with half-pixel bilinear interpolation. Conversion
to normalized NCHW f32 is fused, so resizing a Full HD source to a small model input
does not allocate a Full HD floating tensor. Frame capture, decoding, labels, model
architecture and application decisions remain the caller's responsibility.

120 Hz means 8.333 ms for capture, queueing, preprocessing, inference, state handling
and outputs together. Measure the chosen model with warmed real frame sequences,
allocation counts and frame latency tails. Microbenchmark batch means do not establish
an end-to-end FPS guarantee. CPU performance, including graph overhead and unoptimized
fallbacks, remains active development; retained reports disclose both wins and gaps.


## Batched and normalized training

`Tape::matmul` uses the same vector/batch shape rules as `matmul`, including scalar
dot outputs and empty batches. Broadcast contributions are summed back into the
original operand shapes. `Linear::record` accepts `[...,in]` like inference; it
preserves shared parameter leaves across multiple calls. Native contractions may
change floating accumulation order. No higher-order graph is retained.

```rust
use kernix::{Tensor, autograd::Tape};
let mut tape = Tape::<f64>::new();
let x = tape.variable(Tensor::from_values([2, 2, 3], (0..12).map(|i| i as f64 / 10.0).collect())?)?;
let w = tape.variable(Tensor::ones([3, 4])?)?;
let y = tape.matmul(&x, &w)?;
let normalized = tape.layer_norm(&y, 1, 1e-4)?;
let rows = tape.mean_axes(&normalized, &[-1], false)?;
let loss = tape.sum(&rows)?;
let gradients = tape.backward(&loss)?;
assert_eq!(gradients.get(&w)?.unwrap().shape().dims(), &[3, 4]);
# Ok::<(), Box<dyn std::error::Error>>(())
```

Normalization recomputes population statistics for its backward pass; it allocates
scratch and output. Positive finite epsilon is fixed, not differentiated. Compose
`tape.mul`/`add` with trainable scale/bias for affine normalization. Constant groups
are centered with the same Welford mean used by variance. Nonfinite data and
unrepresentable statistics/derivatives return errors. In particular, overflowing
variance is rejected instead of producing spurious finite zeros; extreme values
are not a promise of arbitrary-range exact arithmetic.

The external no_std workflow also executes batched gradients, axis means and layer
norm under a host allocator. The standalone benchmark harness validates independent
Candle graphs and separately measures inference, eager graph recording/forward,
fresh backward and fixed-gradient transactional SGD. See [benchmark boundaries](BENCHMARKING.md).


`Tape::slice_axis` keeps the existing `(axis, start, len, step)` convention,
including negative steps. It copies selected values into the owned graph; its VJP
scatters into an input-shaped zero buffer and accumulates overlapping branches.
Plain `Tensor::slice_axis` remains a zero-copy view for inference. General integer
indexing and duplicate scatter-write semantics are separate future work.
