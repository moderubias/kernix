# Kernix capability matrix (0.4.0)

| Area | Available | Further work |
|---|---|---|
| Shape | Kahen, Kotei, checked transforms, broadcasting, serde | Additional generic views as needed |
| Layout/storage | C/F order, signed/zero strides, owned/aligned CPU and borrowed views | Device pools, asynchronous lifetimes |
| General tensor operations | Arithmetic, masks, indexing, structural operations, reductions/statistics, sorting, contractions | Planned einsum optimizer and richer dtype support |
| Dense algebra | f32/f64 pivoted LU, Cholesky, thin Householder QR, solve/least squares, inverse, det/slogdet | SVD, eigen, condition estimates, complex, sparse |
| CPU speed | Native packed rank-two GEMM, signed strides, bounded tiled conv, pairwise sums, cached softmax | Batched/vector kernels, broader fusion, factorization blocking, graph overhead |
| Numerical choices | Ordered matmul/reference conv, compensated/sequential sums, portable-math | Exact accumulators and extended precision |
| Devices | Opaque buffers, explicit transfers/sync, operation capability traits, CPU executor | Actual CUDA/ROCm/Metal/WebGPU crates and hardware QA |
| Configuration | Explicit optional strict TOML, defaults, private worker pools | Backend-specific policies when implemented |
| Neural/vision | Grouped conv and VJPs, pool and VJPs, dense/norm/losses, borrowed RGB resize | Attention, embedding, normalization VJP, codecs via adapters |
| Autodiff | Explicit reverse-mode tape, broadcasting, rank-two matmul, conv/pool/losses, detach, shared DAG parameters | Batched matmul VJP, decomposition VJPs, graph optimization |
| Training | Parameters, seeded Xavier, SGD, Adam/AdamW, clipping, versioned named checkpoints | Additional optimizers and model serialization formats |
| Temporal models | Explicit state tensors and truncated graph composition example | Specialized GRU/LSTM conveniences |
| Portability | no_std+alloc library, no external BLAS/driver; OS and target CI | Actual runtime testing on additional hardware |

The library serves general linear algebra. Synthetic vision training is a composition
example, not the project boundary or an implemented game model. An 8.333 ms complete
frame budget remains an application benchmark to establish, not an API guarantee.
