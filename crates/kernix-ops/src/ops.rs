mod broadcast;
mod checked;
mod compare;
mod elementwise;
mod error;
mod in_place;
mod input;
mod linalg;
mod manipulate;
mod math;
mod numeric;
mod reduce;
mod reduce_extra;
mod scan;
mod stats;

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
pub use linalg::matmul;
pub use numeric::{FloatElement, One, Zero};
pub use reduce::{
    max_all, max_axes, mean_all, mean_axes, min_all, min_axes, prod_all, prod_axes, sum_all,
    sum_axes,
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
