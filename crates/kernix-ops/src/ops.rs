// Public CPU numerical operations and their scalar/input capabilities.
mod broadcast;
mod checked;
mod compare;
mod elementwise;
mod error;
mod in_place;
mod indexed;
mod input;
mod linalg;
mod linalg_extra;
mod manipulate;
mod math;
mod math_extra;
mod matmul_element;
#[cfg(all(feature = "std", target_arch = "aarch64", not(miri)))]
mod neon;
mod numeric;
mod ordering;
mod reduce;
mod reduce_extra;
mod scan;
mod stable;
mod stats;
mod structural;

pub use checked::{
    CheckedInteger, checked_add, checked_div, checked_mul, checked_neg, checked_rem, checked_sub,
};
pub use elementwise::{
    add, div, map, mul, neg, rem, sub, try_map, try_map_indexed, try_zip_map, try_zip_map_indexed,
    zip_map,
};
pub use error::OpsError;
pub use in_place::{TensorOutput, add_assign, assign, div_assign, mul_assign, sub_assign, zip_mut};
pub use input::TensorInput;
pub use linalg::{matmul, matmul_reference};
pub use matmul_element::{MatmulElement, MatrixInput};
pub use numeric::{FloatElement, One, Zero};
pub use reduce::{
    max_all, max_axes, mean_all, mean_axes, min_all, min_axes, prod_all, prod_axes, sum_all,
    sum_axes, sum_compensated_all, sum_sequential_all,
};

// High-level elementwise comparisons and Boolean selection.
pub use compare::{
    clamp, equal, greater, greater_equal, less, less_equal, logical_and, logical_not, logical_or,
    logical_xor, not_equal, where_select,
};
// Allocating structural transforms and advanced indexing.
pub use manipulate::{concat, gather_axis, pad_constant, repeat_axis, stack};
// Extended reductions and prefix scans.
pub use reduce_extra::{
    all_all, all_axes, any_all, any_axes, argmax_all, argmax_axis, argmin_all, argmin_axis,
    count_nonzero_all, count_nonzero_axes,
};
pub use scan::{cumprod_axis, cumsum_axis};
// Portable floating-point functions and ML activations.
pub use math::{
    FloatMath, abs, exp, gelu, is_finite, is_nan, leaky_relu, ln, log_softmax, relu, sigmoid, silu,
    softmax, softplus, sqrt, tanh,
};

pub use stats::{std_all, std_axes, variance_all, variance_axes};

pub use math_extra::{
    FloatTrig, atan2, cos, expm1, hypot, log_softmax_online, log_softmax_temperature, log1mexp,
    log1p, logaddexp, logdiffexp, logsigmoid, logsumexp_online_all, logsumexp_online_axis,
    masked_log_softmax, masked_log_softmax_temperature, masked_softmax, masked_softmax_temperature,
    mish, rsqrt, sin, softmax_online, softmax_online_temperature, softmax_temperature,
    softplus_beta,
};

// Extended logical-index operations.
pub use indexed::{masked_select, nonzero, scatter_axis, take_axis};
// Structural transforms and sliding windows.
pub use structural::{chunk_axis, fold_axis, roll_axis, split_axis, tile, unfold_axis};
// Stable total-order sorting and top-k selection.
pub use ordering::{SortDirection, TotalOrder, argsort_axis, sort_axis, topk_axis};
// General contractions and vector/matrix convenience operations.
pub use linalg_extra::{dot, outer, tensordot, trace};
// Stable reductions and explicit dtype conversion contracts.
pub use stable::{cast, l2_norm_all, l2_norm_axis, logsumexp_all, logsumexp_axis, try_cast};

pub(crate) use linalg::dense_matmul;
pub(crate) use stats::{mean_variance_axes, mean_welford_axes};
