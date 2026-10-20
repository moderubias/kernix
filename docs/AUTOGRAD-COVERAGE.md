# Forward and reverse-mode coverage (0.5 series)

Generated from `OPERATION-SUPPORT.json`; run `python tools/check_operation_support.py`.

Scope: exported CPU numerical free functions, dense factor methods, RGB frame
methods and the Tensor reshape/permutation counterparts of Tape operations.
Shape/layout/storage lifecycle APIs, configuration, optimizers and checkpoints
are outside this numerical inventory. Every public Tape method is classified;
control/lifetime/backward methods are explicitly excluded from recording operations.

`supported` means a first-order Tape VJP for the documented domain; `partial`
means a narrower domain than the forward API. `unsupported` means no Tape entry
point, including free explicit backward helpers (no higher-order recording).
`not_applicable` identifies inspection/construction or discrete-data helpers.
No operation silently detaches: unsupported recording requires explicit caller code.

| Forward operation | VJP | Tape method | Contract / limitation |
|---|---|---|---|
| `Tensor::permute` | supported | `permute` | Complete valid axis permutation; Tape materializes values and records the inverse VJP. |
| `Tensor::reshape_view` | supported | `reshape` | Same-numel reshape; Tape materializes logical C order. |
| `abs` | unsupported | — | No Tape recording API. |
| `add` | supported | `add` | Finite f32/f64 inputs and gradients; first-order reverse mode. |
| `add_assign` | unsupported | — | No Tape recording API. |
| `all_all` | unsupported | — | No Tape recording API. |
| `all_axes` | unsupported | — | No Tape recording API. |
| `any_all` | unsupported | — | No Tape recording API. |
| `any_axes` | unsupported | — | No Tape recording API. |
| `argmax_all` | unsupported | — | No Tape recording API. |
| `argmax_axis` | unsupported | — | No Tape recording API. |
| `argmin_all` | unsupported | — | No Tape recording API. |
| `argmin_axis` | unsupported | — | No Tape recording API. |
| `argsort_axis` | unsupported | — | No Tape recording API. |
| `assign` | unsupported | — | No Tape recording API. |
| `atan2` | unsupported | — | No Tape recording API. |
| `cast` | unsupported | — | No Tape recording API. |
| `checked_add` | unsupported | — | No Tape recording API. |
| `checked_div` | unsupported | — | No Tape recording API. |
| `checked_mul` | unsupported | — | No Tape recording API. |
| `checked_neg` | unsupported | — | No Tape recording API. |
| `checked_rem` | unsupported | — | No Tape recording API. |
| `checked_sub` | unsupported | — | No Tape recording API. |
| `chunk_axis` | unsupported | — | No Tape recording API. |
| `clamp` | unsupported | — | No Tape recording API. |
| `concat` | unsupported | — | No Tape recording API. |
| `cos` | unsupported | — | No Tape recording API. |
| `count_nonzero_all` | unsupported | — | No Tape recording API. |
| `count_nonzero_axes` | unsupported | — | No Tape recording API. |
| `cumprod_axis` | unsupported | — | No Tape recording API. |
| `cumsum_axis` | unsupported | — | No Tape recording API. |
| `div` | supported | `div` | Finite f32/f64 inputs and gradients; first-order reverse mode. |
| `div_assign` | unsupported | — | No Tape recording API. |
| `dot` | unsupported | — | No Tape recording API. |
| `equal` | unsupported | — | No Tape recording API. |
| `exp` | supported | `exp` | Finite f32/f64 inputs and gradients; first-order reverse mode. |
| `expm1` | unsupported | — | No Tape recording API. |
| `fold_axis` | unsupported | — | No Tape recording API. |
| `gather_axis` | unsupported | — | No Tape recording API. |
| `gelu` | unsupported | — | No Tape recording API. |
| `greater` | unsupported | — | No Tape recording API. |
| `greater_equal` | unsupported | — | No Tape recording API. |
| `hypot` | unsupported | — | No Tape recording API. |
| `is_finite` | unsupported | — | No Tape recording API. |
| `is_nan` | unsupported | — | No Tape recording API. |
| `l2_norm_all` | unsupported | — | No Tape recording API. |
| `l2_norm_axis` | unsupported | — | No Tape recording API. |
| `leaky_relu` | unsupported | — | No Tape recording API. |
| `less` | unsupported | — | No Tape recording API. |
| `less_equal` | unsupported | — | No Tape recording API. |
| `linalg::Cholesky::factor` | unsupported | — | No Tape recording API. |
| `linalg::Cholesky::lower` | not_applicable | — | Construction/inspection or discrete RGB8 preprocessing; no recorded variable. |
| `linalg::Cholesky::solve` | unsupported | — | No Tape recording API. |
| `linalg::Lu::determinant` | unsupported | — | No Tape recording API. |
| `linalg::Lu::factor` | unsupported | — | No Tape recording API. |
| `linalg::Lu::factor_with_tolerance` | unsupported | — | No Tape recording API. |
| `linalg::Lu::inverse` | unsupported | — | No Tape recording API. |
| `linalg::Lu::packed` | not_applicable | — | Construction/inspection or discrete RGB8 preprocessing; no recorded variable. |
| `linalg::Lu::permutation` | not_applicable | — | Construction/inspection or discrete RGB8 preprocessing; no recorded variable. |
| `linalg::Lu::slogdet` | unsupported | — | No Tape recording API. |
| `linalg::Lu::solve` | unsupported | — | No Tape recording API. |
| `linalg::Qr::factor` | unsupported | — | No Tape recording API. |
| `linalg::Qr::q` | not_applicable | — | Construction/inspection or discrete RGB8 preprocessing; no recorded variable. |
| `linalg::Qr::r` | not_applicable | — | Construction/inspection or discrete RGB8 preprocessing; no recorded variable. |
| `linalg::Qr::solve_least_squares` | unsupported | — | No Tape recording API. |
| `linalg::determinant` | unsupported | — | No Tape recording API. |
| `linalg::eye` | not_applicable | — | Construction/inspection or discrete RGB8 preprocessing; no recorded variable. |
| `linalg::inverse` | unsupported | — | No Tape recording API. |
| `linalg::least_squares` | unsupported | — | No Tape recording API. |
| `linalg::residual_norm` | unsupported | — | No Tape recording API. |
| `linalg::solve` | unsupported | — | No Tape recording API. |
| `ln` | supported | `ln` | Finite f32/f64 inputs and gradients; first-order reverse mode. |
| `log1mexp` | unsupported | — | No Tape recording API. |
| `log1p` | unsupported | — | No Tape recording API. |
| `log_softmax` | supported | `log_softmax` | Signed axis; finite logits and representable results. |
| `log_softmax_online` | unsupported | — | No Tape recording API. |
| `log_softmax_temperature` | unsupported | — | No Tape recording API. |
| `logaddexp` | unsupported | — | No Tape recording API. |
| `logdiffexp` | unsupported | — | No Tape recording API. |
| `logical_and` | unsupported | — | No Tape recording API. |
| `logical_not` | unsupported | — | No Tape recording API. |
| `logical_or` | unsupported | — | No Tape recording API. |
| `logical_xor` | unsupported | — | No Tape recording API. |
| `logsigmoid` | unsupported | — | No Tape recording API. |
| `logsumexp_all` | unsupported | — | No Tape recording API. |
| `logsumexp_axis` | unsupported | — | No Tape recording API. |
| `logsumexp_online_all` | unsupported | — | No Tape recording API. |
| `logsumexp_online_axis` | unsupported | — | No Tape recording API. |
| `map` | unsupported | — | No Tape recording API. |
| `masked_log_softmax` | unsupported | — | No Tape recording API. |
| `masked_log_softmax_temperature` | unsupported | — | No Tape recording API. |
| `masked_select` | unsupported | — | No Tape recording API. |
| `masked_softmax` | unsupported | — | No Tape recording API. |
| `masked_softmax_temperature` | unsupported | — | No Tape recording API. |
| `matmul` | partial | `matmul` | Rank-two operands only; no vector or broadcast-batch VJP. |
| `matmul_reference` | unsupported | — | No Tape recording API. |
| `max_all` | unsupported | — | No Tape recording API. |
| `max_axes` | unsupported | — | No Tape recording API. |
| `mean_all` | supported | `mean` | All elements; empty mean returns EmptyReduction. |
| `mean_axes` | unsupported | — | No Tape recording API. |
| `min_all` | unsupported | — | No Tape recording API. |
| `min_axes` | unsupported | — | No Tape recording API. |
| `mish` | unsupported | — | No Tape recording API. |
| `mul` | supported | `mul` | Finite f32/f64 inputs and gradients; first-order reverse mode. |
| `mul_assign` | unsupported | — | No Tape recording API. |
| `neg` | unsupported | — | No Tape recording API. |
| `neural::avg_pool2d` | supported | `avg_pool2d` | NCHW geometry; pooling ties follow the documented first-winner rule. |
| `neural::avg_pool2d_backward` | unsupported | — | Explicit free VJP helper; not recorded for higher-order differentiation. |
| `neural::binary_cross_entropy_with_logits` | supported | `binary_cross_entropy` | Documented finite target/logit domain; labels are not differentiated. |
| `neural::conv2d` | supported | `conv2d` | NCHW geometry; pooling ties follow the documented first-winner rule. |
| `neural::conv2d_backward` | unsupported | — | Explicit free VJP helper; not recorded for higher-order differentiation. |
| `neural::conv2d_reference` | unsupported | — | No Tape recording API. |
| `neural::cross_entropy_with_logits` | supported | `cross_entropy` | Documented finite target/logit domain; labels are not differentiated. |
| `neural::layer_norm` | unsupported | — | Forward normalization only; no Tape VJP for input or affine parameters. |
| `neural::linear` | partial | `linear` | Rank-two input only; forward accepts arbitrary leading axes. |
| `neural::max_pool2d` | supported | `max_pool2d` | NCHW geometry; pooling ties follow the documented first-winner rule. |
| `neural::max_pool2d_backward` | unsupported | — | Explicit free VJP helper; not recorded for higher-order differentiation. |
| `neural::mse_loss` | supported | `mse` | Documented finite target/logit domain; labels are not differentiated. |
| `nonzero` | unsupported | — | No Tape recording API. |
| `not_equal` | unsupported | — | No Tape recording API. |
| `outer` | unsupported | — | No Tape recording API. |
| `pad_constant` | unsupported | — | No Tape recording API. |
| `prod_all` | unsupported | — | No Tape recording API. |
| `prod_axes` | unsupported | — | No Tape recording API. |
| `relu` | supported | `relu` | Finite f32/f64 inputs and gradients; first-order reverse mode. |
| `rem` | unsupported | — | No Tape recording API. |
| `repeat_axis` | unsupported | — | No Tape recording API. |
| `roll_axis` | unsupported | — | No Tape recording API. |
| `rsqrt` | unsupported | — | No Tape recording API. |
| `scatter_axis` | unsupported | — | No Tape recording API. |
| `sigmoid` | supported | `sigmoid` | Finite f32/f64 inputs and gradients; first-order reverse mode. |
| `silu` | unsupported | — | No Tape recording API. |
| `sin` | unsupported | — | No Tape recording API. |
| `softmax` | supported | `softmax` | Signed axis; finite logits and representable results. |
| `softmax_online` | unsupported | — | No Tape recording API. |
| `softmax_online_temperature` | unsupported | — | No Tape recording API. |
| `softmax_temperature` | unsupported | — | No Tape recording API. |
| `softplus` | unsupported | — | No Tape recording API. |
| `softplus_beta` | unsupported | — | No Tape recording API. |
| `sort_axis` | unsupported | — | No Tape recording API. |
| `split_axis` | unsupported | — | No Tape recording API. |
| `sqrt` | supported | `sqrt` | Finite f32/f64 inputs and gradients; first-order reverse mode. |
| `stack` | unsupported | — | No Tape recording API. |
| `std_all` | unsupported | — | No Tape recording API. |
| `std_axes` | unsupported | — | No Tape recording API. |
| `sub` | supported | `sub` | Finite f32/f64 inputs and gradients; first-order reverse mode. |
| `sub_assign` | unsupported | — | No Tape recording API. |
| `sum_all` | supported | `sum` | All elements; empty sum has an empty input gradient. |
| `sum_axes` | unsupported | — | No Tape recording API. |
| `sum_compensated_all` | unsupported | — | No Tape recording API. |
| `sum_sequential_all` | unsupported | — | No Tape recording API. |
| `take_axis` | unsupported | — | No Tape recording API. |
| `tanh` | supported | `tanh` | Finite f32/f64 inputs and gradients; first-order reverse mode. |
| `tensordot` | unsupported | — | No Tape recording API. |
| `tile` | unsupported | — | No Tape recording API. |
| `topk_axis` | unsupported | — | No Tape recording API. |
| `trace` | unsupported | — | No Tape recording API. |
| `try_cast` | unsupported | — | No Tape recording API. |
| `try_map` | unsupported | — | No Tape recording API. |
| `try_map_indexed` | unsupported | — | No Tape recording API. |
| `try_zip_map` | unsupported | — | No Tape recording API. |
| `try_zip_map_indexed` | unsupported | — | No Tape recording API. |
| `unfold_axis` | unsupported | — | No Tape recording API. |
| `variance_all` | unsupported | — | No Tape recording API. |
| `variance_axes` | unsupported | — | No Tape recording API. |
| `vision::RgbFrame::crop` | not_applicable | — | Construction/inspection or discrete RGB8 preprocessing; no recorded variable. |
| `vision::RgbFrame::from_rows` | not_applicable | — | Construction/inspection or discrete RGB8 preprocessing; no recorded variable. |
| `vision::RgbFrame::new` | not_applicable | — | Construction/inspection or discrete RGB8 preprocessing; no recorded variable. |
| `vision::RgbFrame::resize_nchw` | not_applicable | — | Construction/inspection or discrete RGB8 preprocessing; no recorded variable. |
| `where_select` | unsupported | — | No Tape recording API. |
| `zip_map` | unsupported | — | No Tape recording API. |
| `zip_mut` | unsupported | — | No Tape recording API. |

The checker detects missing/new exports, stale Tape method names, duplicate rows,
unclassified recording methods and missing regression-test evidence. It does not
prove derivative correctness; numerical tests and explicit domain review do that.
