# Sparse matrices and automatic storage (0.13)

Use `Matrix` when rank-two finite real data should choose Dense or CSR automatically.
Existing `Tensor`/`Kahen`/`Kotei`, macros, dense operations and backend contracts
remain unchanged. A dense affine layout promises ordinary backing storage and
strides; CSR has missing entries and indexed rows. Making CSR silently implement
that contract would break existing consumers or hide materialization.

```rust
use kernix::{Matrix, MatrixStorage, tensor};
let a = Matrix::from_rows([
    [2.0f64, 0.0, 0.0, 0.0],
    [0.0, 3.0, 0.0, 0.0],
])?;
assert_eq!(a.storage_kind(), MatrixStorage::Csr);
let x = tensor![1.0, 2.0, 3.0, 4.0]?;
assert_eq!(a.matvec(&x)?.storage_slice(), &[2.0, 6.0]);
let b = Matrix::from_rows([[1.0], [2.0], [3.0], [4.0]])?;
let result = a.matmul(&b)?.into_dense()?;
assert_eq!(result.storage_slice(), &[2.0, 6.0]);
# Ok::<(), Box<dyn std::error::Error>>(())
```

`Matrix::from_tensor(tensor)` consumes a rank-two owned CPU tensor;
`Matrix::from_values([rows, columns], values)` accepts runtime dimensions.
They inspect finite logical values and exact zeros in one pass. If CSR's
initialized payload is strictly smaller, a second pass copies nonzeros into
fallibly allocated CSR buffers. Otherwise the original dense allocation and
layout are retained. No tolerance removes small nonzeros. Negative zero may become
structural positive zero. Empty matrices remain dense. This selection minimizes
**payload**, not estimated latency, allocator peak memory or process RSS.
Compression temporarily retains the input plus CSR buffers. `Matrix::dense` and
`Matrix::sparse` allow explicit control without copying their buffers.

| Product | Execution | Output |
|---|---|---|
| Dense × Dense | Existing dense GEMM | Dense |
| CSR × Dense | Nonzero row updates | Dense |
| Dense × CSR | Nonzero row traversal | Dense |
| CSR × CSR | Sorted row-stream heap merge | CSR |

Dispatch does not rescan density or convert operands at every multiplication.
Output is not automatically recompressed or densified. Reuse a prepared `Matrix`
when repeating operations; construction and conversion costs are separate.
`matvec` returns an owned rank-one tensor. Mixed multiplication returns an owned
C-order tensor. There is no dtype promotion, device transfer or synchronization.

CSR × CSR can fill in. `matmul_with_limit(&rhs, max_entries)` caps nonzero output
entries, returning `SparseError::FillInLimit` without changing either operand.
The limit does not apply to dense output. Heap scratch is bounded by the largest
left row; it does not allocate a dense buffer of result-column length. Row offsets
and output still allocate. Sorted unique CSR indices permit extremely wide
logical shapes that cannot be materialized as a `Tensor`.

For assembly, use `CsrMatrix::from_triplets([m, n], entries)` or
`Matrix::from_triplets`. Duplicate coordinates are summed in original input order;
raw assembly keeps explicitly stored zeros, including cancellation results.
`from_triplets` explicitly requests sparse storage. `CsrMatrix::from_parts` adopts
canonical offsets/indices/values, validating endpoints, bounds, ordering and
uniqueness without cloning elements. Raw storage supports custom non-Clone values;
COO assembly additionally needs `AddAssign`. Arithmetic uses existing
`FloatMath + MatmulElement` traits, including f32/f64.

```rust
use kernix::{CsrMatrix, tensor};
let a = CsrMatrix::from_triplets([2, 4], vec![(0, 0, 2.0f64), (1, 1, 3.0)])?;
let at = a.transpose(); // Borrowed CSC view: all three buffers are shared.
assert_eq!(at.matvec(&tensor![1.0, 2.0]?)?.storage_slice(), &[2., 6., 0., 0.]);
# Ok::<(), Box<dyn std::error::Error>>(())
```

`CsrMatrix::from_tensor(&input)` explicitly compresses a rank-two `TensorInput`,
including fixed-rank tensors and signed-stride views, without a full dense copy.
`to_dense`, `Matrix::into_dense` and `into_csr` make conversions explicit.
`get(row, column)` returns `None` for a structural zero and an error for an invalid
coordinate. `payload_bytes` counts initialized values, usize indices and offsets;
it excludes unused capacity, object metadata and allocator overhead.

All `Matrix` constructors require finite logical values; sparse arithmetic checks
inputs/results and rejects nonfinite outputs. Raw generic CSR storage can hold
arbitrary values. Sparse execution skips structural zeros and can change floating
point rounding; it promises neither dense bitwise equality nor IEEE NaN/Inf
propagation. Existing `Tensor` numerical policies remain unchanged.

This initial CPU implementation supports rank two and `no_std + alloc`. Sparse
arithmetic is not recorded by `Tape`; explicitly converting to dense starts a new
ordinary dense workflow and does not reconnect a gradient graph. Sparse batched
operations, reductions, reusable output buffers, sparse factorization, iterative
solvers, GPU execution, narrow index types and sparse autodiff are deferred.
Neither sparse storage nor a low density guarantees a speedup: benchmark the actual
shape, nonzero pattern, layout and reuse count.
