//! Portable neural-network primitives with explicit NCHW geometry.
//!
//! ```
//! use kernix_core::{Order, shape::Kahen, tensor::Tensor};
//! use kernix_ops::neural::{conv2d, Conv2dOptions, avg_pool2d};
//! let image=Tensor::from_vec(vec![1.0f32;16],Kahen::new([1,1,4,4])?,Order::C)?;
//! let filter=Tensor::from_vec(vec![1.0f32;9],Kahen::new([1,1,3,3])?,Order::C)?;
//! let features=conv2d(&image,&filter,Conv2dOptions::default())?;
//! assert_eq!(avg_pool2d(&features,[2,2],[2,2])?.storage_slice(), &[9.0]);
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
use crate::{FloatMath, OpsError, TensorInput};
use alloc::{vec, vec::Vec};
use core::{mem::align_of, ops::AddAssign};
use kernix_core::{
    Order,
    shape::Kahen,
    tensor::{Tensor, TensorBuildError},
};

/// Two-dimensional cross-correlation options (the usual neural-network convolution).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Conv2dOptions {
    /// Vertical/horizontal output step. Both must be positive.
    pub stride: [usize; 2],
    /// Symmetric top/bottom and left/right zero padding.
    pub padding: [usize; 2],
    /// Vertical/horizontal kernel dilation. Both must be positive.
    pub dilation: [usize; 2],
    /// Independent channel groups; positive and divides both channel counts.
    pub groups: usize,
}
impl Default for Conv2dOptions {
    fn default() -> Self {
        Self {
            stride: [1, 1],
            padding: [0, 0],
            dilation: [1, 1],
            groups: 1,
        }
    }
}
fn invalid(operation: &'static str, parameter: &'static str) -> OpsError {
    OpsError::InvalidParameter {
        operation,
        parameter,
    }
}
fn rank4(dims: &[usize], op: &'static str) -> Result<[usize; 4], OpsError> {
    Kahen::new(dims)?;
    dims.try_into().map_err(|_| OpsError::ExactRank {
        operation: op,
        expected: 4,
        actual: dims.len(),
    })
}
fn extent(
    size: usize,
    kernel: usize,
    stride: usize,
    padding: usize,
    dilation: usize,
) -> Result<usize, OpsError> {
    if kernel == 0 || stride == 0 || dilation == 0 {
        return Err(invalid(
            "spatial geometry",
            "kernel, stride and dilation must be positive",
        ));
    }
    let effective = (kernel - 1)
        .checked_mul(dilation)
        .and_then(|x| x.checked_add(1))
        .ok_or(OpsError::DimensionOverflow { axis: 2 })?;
    let padded = padding
        .checked_mul(2)
        .and_then(|p| size.checked_add(p))
        .ok_or(OpsError::DimensionOverflow { axis: 2 })?;
    Ok(if padded < effective {
        0
    } else {
        (padded - effective) / stride + 1
    })
}
struct ConvGeometry {
    input: [usize; 4],
    weight: [usize; 4],
    output: [usize; 4],
    options: Conv2dOptions,
}
impl ConvGeometry {
    fn new(input: &[usize], weight: &[usize], options: Conv2dOptions) -> Result<Self, OpsError> {
        let input = rank4(input, "conv2d")?;
        let weight = rank4(weight, "conv2d weights")?;
        let [n, ci, h, w] = input;
        let [co, cg, kh, kw] = weight;
        let g = options.groups;
        if g == 0 || ci == 0 || co == 0 || ci % g != 0 || co % g != 0 || cg != ci / g {
            return Err(invalid(
                "conv2d",
                "groups and input/output channels must agree",
            ));
        }
        let oh = extent(
            h,
            kh,
            options.stride[0],
            options.padding[0],
            options.dilation[0],
        )?;
        let ow = extent(
            w,
            kw,
            options.stride[1],
            options.padding[1],
            options.dilation[1],
        )?;
        let output = [n, co, oh, ow];
        Kahen::new(output)?;
        Ok(Self {
            input,
            weight,
            output,
            options,
        })
    }
    fn position(&self, oy: usize, ox: usize, ky: usize, kx: usize) -> Option<(usize, usize)> {
        let y = (oy * self.options.stride[0] + ky * self.options.dilation[0])
            .checked_sub(self.options.padding[0])?;
        let x = (ox * self.options.stride[1] + kx * self.options.dilation[1])
            .checked_sub(self.options.padding[1])?;
        (y < self.input[2] && x < self.input[3]).then_some((y, x))
    }
}
fn flatten<E: Into<OpsError>>(err: TensorBuildError<E>) -> OpsError {
    match err {
        TensorBuildError::Tensor(e) => e.into(),
        TensorBuildError::Initialize { error, .. } => error.into(),
    }
}

fn offset4(i: [usize; 4], dims: [usize; 4]) -> usize {
    ((i[0] * dims[1] + i[1]) * dims[2] + i[2]) * dims[3] + i[3]
}
fn read4<T: Copy, A: TensorInput<T>>(
    input: &A,
    values: Option<&[T]>,
    i: [usize; 4],
    dims: [usize; 4],
) -> Result<T, OpsError> {
    if let Some(values) = values {
        Ok(values[offset4(i, dims)])
    } else {
        Ok(*input.get(&i)?)
    }
}
/// NCHW cross-correlation with weights `[out_channels, in_channels/groups, kh, kw]`.
///
/// Allocates C-order output. No bias is implicit: broadcast-add `[1,out_channels,1,1]`.
/// Invalid ranks/channel groups/zero kernel or overflowing geometry return errors.
/// An effective kernel larger than the padded input yields an empty spatial axis.
/// Arbitrary strides are respected. Finite contiguous floats use bounded tiled
/// im2col/GEMM with native FMA/order; conv2d_reference keeps scalar order. Scratch
/// storage is O((kernel elements+output channels/groups)*min(output pixels,128)).
/// Cost O(batch*out_channels*oh*ow*in_channels/groups*kh*kw).
///
/// # Examples
///
/// ```
/// use kernix_core::{Order,shape::Kahen,tensor::Tensor};
/// use kernix_ops::neural::*;
/// let a=Tensor::from_vec(vec![1.0f32;16],Kahen::new([1,1,4,4])?,Order::C)?;
/// let w=Tensor::from_vec(vec![1.0f32;4],Kahen::new([1,1,2,2])?,Order::C)?;
/// let out=conv2d(&a,&w,Conv2dOptions::default())?;
/// assert_eq!(out.shape().dims(),&[1,1,3,3]);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn conv2d<T: FloatMath + crate::MatmulElement, A: TensorInput<T>, B: TensorInput<T>>(
    input: &A,
    weight: &B,
    options: Conv2dOptions,
) -> Result<Tensor<T>, OpsError> {
    conv_impl(input, weight, options, true)
}

/// Ordered scalar NCHW convolution with the same geometry as conv2d, without
/// native GEMM/FMA dispatch. Useful for operation-order compatibility and numerical
/// reference checks. See the module example and conv2d options.
pub fn conv2d_reference<
    T: FloatMath + crate::MatmulElement,
    A: TensorInput<T>,
    B: TensorInput<T>,
>(
    input: &A,
    weight: &B,
    options: Conv2dOptions,
) -> Result<Tensor<T>, OpsError> {
    conv_impl(input, weight, options, false)
}

fn conv_impl<T: FloatMath + crate::MatmulElement, A: TensorInput<T>, B: TensorInput<T>>(
    input: &A,
    weight: &B,
    options: Conv2dOptions,
    native: bool,
) -> Result<Tensor<T>, OpsError> {
    let g = ConvGeometry::new(input.dims(), weight.dims(), options)?;
    let ni = Kahen::new(g.input)?.numel();
    let nw = Kahen::new(g.weight)?.numel();
    let iv = input.as_c_contiguous_slice().filter(|v| v.len() == ni);
    let wv = weight.as_c_contiguous_slice().filter(|v| v.len() == nw);

    if native
        && T::HAS_NATIVE_MATMUL
        && iv.is_some_and(|v| v.iter().all(|x| FloatMath::is_finite(*x)))
        && wv.is_some_and(|v| v.iter().all(|x| FloatMath::is_finite(*x)))
    {
        let input = iv.expect("contiguous capability checked");
        let weights = wv.expect("contiguous capability checked");
        let mut output =
            Tensor::filled(Kahen::new(g.output)?, Order::C, align_of::<T>(), T::zero())?;
        if output.is_empty() {
            return Ok(output);
        }
        let spatial = g.output[2] * g.output[3];
        let block = spatial.min(128);
        let co = g.weight[0] / options.groups;
        let kernel = Kahen::new(&g.weight[1..])?.numel();
        let mut columns = Tensor::filled(
            Kahen::new([kernel, block])?,
            Order::C,
            align_of::<T>(),
            T::zero(),
        )?;
        let mut product = Tensor::filled(
            Kahen::new([co, block])?,
            Order::C,
            align_of::<T>(),
            T::zero(),
        )?;
        let dst = output.storage_slice_mut();
        for batch in 0..g.output[0] {
            for group in 0..options.groups {
                for start in (0..spatial).step_by(block) {
                    let count = (spatial - start).min(block);
                    let col = &mut columns.storage_slice_mut()[..kernel * count];
                    for ci in 0..g.weight[1] {
                        for ky in 0..g.weight[2] {
                            for kx in 0..g.weight[3] {
                                let row = (ci * g.weight[2] + ky) * g.weight[3] + kx;
                                let target = &mut col[row * count..(row + 1) * count];
                                if options == Conv2dOptions::default() {
                                    let mut p = 0;
                                    while p < count {
                                        let linear = start + p;
                                        let y = linear / g.output[3];
                                        let x = linear % g.output[3];
                                        let run = (g.output[3] - x).min(count - p);
                                        let from =
                                            ((batch * g.input[1] + ci) * g.input[2] + y + ky)
                                                * g.input[3]
                                                + x
                                                + kx;
                                        target[p..p + run]
                                            .copy_from_slice(&input[from..from + run]);
                                        p += run;
                                    }
                                } else {
                                    for (p, value) in target.iter_mut().enumerate() {
                                        let linear = start + p;
                                        *value = if let Some((y, x)) = g.position(
                                            linear / g.output[3],
                                            linear % g.output[3],
                                            ky,
                                            kx,
                                        ) {
                                            input[offset4(
                                                [batch, group * g.weight[1] + ci, y, x],
                                                g.input,
                                            )]
                                        } else {
                                            T::zero()
                                        };
                                    }
                                }
                            }
                        }
                    }
                    let weight = &weights[group * co * kernel..(group + 1) * co * kernel];
                    let values = &mut product.storage_slice_mut()[..co * count];
                    if !T::try_dense_matmul(weight, col, values, co, kernel, count) {
                        crate::ops::dense_matmul(weight, col, values, co, kernel, count, false);
                    }
                    for channel in 0..co {
                        let base = ((batch * g.weight[0] + group * co + channel) * spatial) + start;
                        dst[base..base + count]
                            .copy_from_slice(&values[channel * count..(channel + 1) * count]);
                    }
                }
            }
        }
        return Ok(output);
    }
    if options == Conv2dOptions::default() {
        if let (Some(input), Some(weight)) = (iv, wv) {
            let mut output =
                Tensor::filled(Kahen::new(g.output)?, Order::C, align_of::<T>(), T::zero())?;
            if output.is_empty() {
                return Ok(output);
            }
            let [n, co, oh, ow] = g.output;
            let ci = g.input[1];
            let h = g.input[2];
            let w = g.input[3];
            let kh = g.weight[2];
            let kw = g.weight[3];
            let values = output.storage_slice_mut();
            for batch in 0..n {
                for channel in 0..co {
                    for y in 0..oh {
                        for x in 0..ow {
                            let mut sum = T::zero();
                            for inner in 0..ci {
                                for ky in 0..kh {
                                    let ai = ((batch * ci + inner) * h + y + ky) * w + x;
                                    let wi = ((channel * ci + inner) * kh + ky) * kw;
                                    for (&a, &b) in
                                        input[ai..ai + kw].iter().zip(&weight[wi..wi + kw])
                                    {
                                        sum += a * b;
                                    }
                                }
                            }
                            values[((batch * co + channel) * oh + y) * ow + x] = sum;
                        }
                    }
                }
            }
            return Ok(output);
        }
    }
    Tensor::try_from_fn(Kahen::new(g.output)?, Order::C, align_of::<T>(), |i| {
        let [n, co, oy, ox] = [i[0], i[1], i[2], i[3]];
        let group = co / (g.weight[0] / options.groups);
        let mut sum = T::zero();
        for ci in 0..g.weight[1] {
            for ky in 0..g.weight[2] {
                for kx in 0..g.weight[3] {
                    if let Some((y, x)) = g.position(oy, ox, ky, kx) {
                        sum += read4(input, iv, [n, group * g.weight[1] + ci, y, x], g.input)?
                            * read4(weight, wv, [co, ci, ky, kx], g.weight)?;
                    }
                }
            }
        }
        Ok::<T, OpsError>(sum)
    })
    .map_err(flatten)
}
/// Vector-Jacobian product of conv2d, returning (input_gradient, weight_gradient).
///
/// Gradient dimensions must exactly equal forward output geometry. Results are
/// fresh zero-initialized C-order tensors; overlapping windows accumulate. Options
/// and strides follow conv2d. Cost equals the reference forward contraction order.
/// See the [module example](self) for the forward geometry.
///
/// # Examples
///
/// ```
/// use kernix_core::{Order,shape::Kahen,tensor::Tensor};
/// use kernix_ops::neural::*;
/// let a=Tensor::from_vec(vec![1.0f32;16],Kahen::new([1,1,4,4])?,Order::C)?;
/// let w=Tensor::from_vec(vec![1.0f32;4],Kahen::new([1,1,2,2])?,Order::C)?;
/// let g=conv2d(&a,&w,Conv2dOptions::default())?;
/// let (dx,dw)=conv2d_backward(&a,&w,&g,Conv2dOptions::default())?;
/// assert_eq!(dx.shape().dims(),a.shape().dims());
/// assert_eq!(dw.shape().dims(),w.shape().dims());
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn conv2d_backward<
    T: FloatMath + AddAssign,
    A: TensorInput<T>,
    B: TensorInput<T>,
    G: TensorInput<T>,
>(
    input: &A,
    weight: &B,
    gradient: &G,
    options: Conv2dOptions,
) -> Result<(Tensor<T>, Tensor<T>), OpsError> {
    let g = ConvGeometry::new(input.dims(), weight.dims(), options)?;
    let ni = Kahen::new(g.input)?.numel();
    let nw = Kahen::new(g.weight)?.numel();
    let iv = input.as_c_contiguous_slice().filter(|v| v.len() == ni);
    let wv = weight.as_c_contiguous_slice().filter(|v| v.len() == nw);

    if gradient.dims() != g.output {
        return Err(invalid(
            "conv2d_backward",
            "gradient shape differs from forward output",
        ));
    }
    let mut dx = Tensor::filled(Kahen::new(g.input)?, Order::C, align_of::<T>(), T::zero())?;
    let mut dw = Tensor::filled(Kahen::new(g.weight)?, Order::C, align_of::<T>(), T::zero())?;
    if gradient.dims().contains(&0) {
        return Ok((dx, dw));
    }
    for n in 0..g.output[0] {
        for co in 0..g.output[1] {
            for oy in 0..g.output[2] {
                for ox in 0..g.output[3] {
                    let dy = *gradient.get(&[n, co, oy, ox])?;
                    let group = co / (g.weight[0] / options.groups);
                    for ci in 0..g.weight[1] {
                        for ky in 0..g.weight[2] {
                            for kx in 0..g.weight[3] {
                                if let Some((y, x)) = g.position(oy, ox, ky, kx) {
                                    let ai = [n, group * g.weight[1] + ci, y, x];
                                    let wi = [co, ci, ky, kx];
                                    *dx.get_mut(&ai)? += dy * read4(weight, wv, wi, g.weight)?;
                                    *dw.get_mut(&wi)? += dy * read4(input, iv, ai, g.input)?;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    Ok((dx, dw))
}
fn pool_dims(
    dims: &[usize],
    window: [usize; 2],
    stride: [usize; 2],
) -> Result<[usize; 4], OpsError> {
    let [n, c, h, w] = rank4(dims, "pool2d")?;
    window[0]
        .checked_mul(window[1])
        .ok_or(OpsError::ReductionCountOverflow)?;
    Ok([
        n,
        c,
        extent(h, window[0], stride[0], 0, 1)?,
        extent(w, window[1], stride[1], 0, 1)?,
    ])
}
fn pool<T: FloatMath, A: TensorInput<T>>(
    input: &A,
    window: [usize; 2],
    stride: [usize; 2],
    maximum: bool,
) -> Result<Tensor<T>, OpsError> {
    let dims = pool_dims(input.dims(), window, stride)?;
    Tensor::try_from_fn(Kahen::new(dims)?, Order::C, align_of::<T>(), |i| {
        let mut sum = T::zero();
        let mut best = None;
        for y in 0..window[0] {
            for x in 0..window[1] {
                let value =
                    *input.get(&[i[0], i[1], i[2] * stride[0] + y, i[3] * stride[1] + x])?;
                if maximum {
                    if best.is_none_or(|b| {
                        !FloatMath::is_nan(b) && (FloatMath::is_nan(value) || value > b)
                    }) {
                        best = Some(value);
                    }
                } else {
                    sum = sum + value;
                }
            }
        }
        Ok::<T, OpsError>(if maximum {
            best.expect("positive window")
        } else {
            sum / T::from_count(window[0] * window[1])
        })
    })
    .map_err(flatten)
}
/// NCHW average pooling without padding, using positive window and stride.
/// Incomplete trailing windows are omitted. Allocates a C-order result. Invalid
/// ranks/zero geometry/overflow return errors; oversized windows yield empty axes.
/// See the [module example](self) for a 2x2 reduction.
///
/// # Examples
///
/// ```
/// use kernix_core::{Order,shape::Kahen,tensor::Tensor};
/// use kernix_ops::neural::*;
/// let a=Tensor::from_vec(vec![1.0f32;16],Kahen::new([1,1,4,4])?,Order::C)?;
/// let out=avg_pool2d(&a,[2,2],[2,2])?;
/// assert_eq!(out.storage_slice(),&[1.0;4]);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn avg_pool2d<T: FloatMath, A: TensorInput<T>>(
    input: &A,
    window: [usize; 2],
    stride: [usize; 2],
) -> Result<Tensor<T>, OpsError> {
    pool(input, window, stride, false)
}
/// NCHW maximum pooling without padding. First ties and first NaNs win in
/// row-major window order. Geometry/allocation follows avg_pool2d.
///
/// # Examples
///
/// ```
/// use kernix_core::{Order,shape::Kahen,tensor::Tensor};
/// use kernix_ops::neural::*;
/// let a=Tensor::from_vec(vec![1.0f32;16],Kahen::new([1,1,4,4])?,Order::C)?;
/// let out=max_pool2d(&a,[2,2],[2,2])?;
/// assert_eq!(out.storage_slice(),&[1.0;4]);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn max_pool2d<T: FloatMath, A: TensorInput<T>>(
    input: &A,
    window: [usize; 2],
    stride: [usize; 2],
) -> Result<Tensor<T>, OpsError> {
    pool(input, window, stride, true)
}
/// Average-pooling VJP for an original NCHW shape. Overlapping window contributions
/// add; uncovered input elements receive zero. Reject mismatched gradient geometry.
///
/// # Examples
///
/// ```
/// use kernix_core::{Order,shape::Kahen,tensor::Tensor};
/// use kernix_ops::neural::*;
/// let a=Tensor::from_vec(vec![1.0f32;16],Kahen::new([1,1,4,4])?,Order::C)?;
/// let g=avg_pool2d(&a,[2,2],[2,2])?;
/// let dx=avg_pool2d_backward(a.shape().dims(),&g,[2,2],[2,2])?;
/// assert_eq!(dx.storage_slice(),&[0.25;16]);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn avg_pool2d_backward<T: FloatMath + AddAssign, G: TensorInput<T>>(
    input_dims: impl AsRef<[usize]>,
    gradient: &G,
    window: [usize; 2],
    stride: [usize; 2],
) -> Result<Tensor<T>, OpsError> {
    let dims = input_dims.as_ref();
    let out = pool_dims(dims, window, stride)?;
    if gradient.dims() != out {
        return Err(invalid(
            "avg_pool2d_backward",
            "gradient shape differs from forward output",
        ));
    }
    let mut dx = Tensor::filled(Kahen::new(dims)?, Order::C, align_of::<T>(), T::zero())?;
    if out.contains(&0) {
        return Ok(dx);
    }
    for n in 0..out[0] {
        for c in 0..out[1] {
            for y in 0..out[2] {
                for x in 0..out[3] {
                    let dy = *gradient.get(&[n, c, y, x])? / T::from_count(window[0] * window[1]);
                    for ky in 0..window[0] {
                        for kx in 0..window[1] {
                            *dx.get_mut(&[n, c, y * stride[0] + ky, x * stride[1] + kx])? += dy;
                        }
                    }
                }
            }
        }
    }
    Ok(dx)
}
/// Maximum-pooling VJP. Route each gradient to the first maximum (first NaN
/// takes priority), and sum overlapping contributions. Input values are required
/// to select winners; mismatched output gradients return an error.
///
/// # Examples
///
/// ```
/// use kernix_core::{Order,shape::Kahen,tensor::Tensor};
/// use kernix_ops::neural::*;
/// let a=Tensor::from_vec(vec![1.0f32;16],Kahen::new([1,1,4,4])?,Order::C)?;
/// let g=max_pool2d(&a,[2,2],[2,2])?;
/// let dx=max_pool2d_backward(&a,&g,[2,2],[2,2])?;
/// assert_eq!(dx.storage_slice()[0],1.0);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn max_pool2d_backward<T: FloatMath + AddAssign, A: TensorInput<T>, G: TensorInput<T>>(
    input: &A,
    gradient: &G,
    window: [usize; 2],
    stride: [usize; 2],
) -> Result<Tensor<T>, OpsError> {
    let out = pool_dims(input.dims(), window, stride)?;
    if gradient.dims() != out {
        return Err(invalid(
            "max_pool2d_backward",
            "gradient shape differs from forward output",
        ));
    }
    let mut dx = Tensor::filled(
        Kahen::new(input.dims())?,
        Order::C,
        align_of::<T>(),
        T::zero(),
    )?;
    if out.contains(&0) {
        return Ok(dx);
    }
    for n in 0..out[0] {
        for c in 0..out[1] {
            for y in 0..out[2] {
                for x in 0..out[3] {
                    let mut best = None;
                    let mut winner = [0; 4];
                    for ky in 0..window[0] {
                        for kx in 0..window[1] {
                            let idx = [n, c, y * stride[0] + ky, x * stride[1] + kx];
                            let v = *input.get(&idx)?;
                            if best.is_none_or(|b| {
                                !FloatMath::is_nan(b) && (FloatMath::is_nan(v) || v > b)
                            }) {
                                best = Some(v);
                                winner = idx;
                            }
                        }
                    }
                    *dx.get_mut(&winner)? += *gradient.get(&[n, c, y, x])?;
                }
            }
        }
    }
    Ok(dx)
}
/// Dense affine transform of the last input axis; weights use `[out,in]`, bias `[out]`.
/// Leading batch/sequence axes are retained. No implicit dtype conversion occurs.
/// Invalid dimensions or access/allocation errors fail explicitly. Allocates C-order.
///
/// # Examples
///
/// ```
/// use kernix_core::{Order,shape::Kahen,tensor::Tensor};
/// use kernix_ops::neural::*;
/// let a=Tensor::from_vec(vec![1.0f32;16],Kahen::new([1,1,4,4])?,Order::C)?;
/// let weights=Tensor::from_vec(vec![1.0f32;8],Kahen::new([2,4])?,Order::C)?;
/// let out=linear(&a,&weights,None)?;
/// assert_eq!(out.shape().dims(),&[1,1,4,2]);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn linear<T: FloatMath + AddAssign, A: TensorInput<T>, W: TensorInput<T>>(
    input: &A,
    weight: &W,
    bias: Option<&dyn TensorInput<T>>,
) -> Result<Tensor<T>, OpsError> {
    let dims = input.dims();
    let w = weight.dims();
    Kahen::new(dims)?;
    Kahen::new(w)?;
    if dims.is_empty() || w.len() != 2 || dims[dims.len() - 1] != w[1] {
        return Err(invalid(
            "linear",
            "input [...,in] and weight [out,in] required",
        ));
    }
    if bias.is_some_and(|b| b.dims() != [w[0]]) {
        return Err(invalid("linear", "bias must have shape [out]"));
    }
    let mut out = dims.to_vec();
    let last = out.len() - 1;
    out[last] = w[0];
    let mut from = vec![0; dims.len()];
    Tensor::try_from_fn(Kahen::new(out)?, Order::C, align_of::<T>(), |i| {
        from.copy_from_slice(i);
        let co = i[last];
        let mut value = if let Some(b) = bias {
            *b.get(&[co])?
        } else {
            T::zero()
        };
        for k in 0..w[1] {
            from[last] = k;
            value += *input.get(&from)? * *weight.get(&[co, k])?;
        }
        Ok::<T, OpsError>(value)
    })
    .map_err(flatten)
}
/// Mean categorical cross-entropy of finite logits `[...,classes]` and flattened
/// integer labels (one per leading-index row). Uses shifted log-domain sums, avoiding log(0)
/// from underflowed probabilities. Mean scaling precedes overflowing differences.
/// Empty batches/classes, invalid labels or unrepresentable mean losses fail.
///
/// # Examples
///
/// ```
/// use kernix_core::{Order,shape::Kahen,tensor::Tensor};
/// use kernix_ops::neural::*;
/// let a=Tensor::from_vec(vec![1.0f32;16],Kahen::new([1,1,4,4])?,Order::C)?;
/// let logits=a.reshape_view([4,4],Order::C)?;
/// let loss=cross_entropy_with_logits(&logits,&[0,1,2,3])?;
/// assert!((loss-4.0f32.ln()).abs()<1e-6);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn cross_entropy_with_logits<T: FloatMath, A: TensorInput<T>>(
    logits: &A,
    targets: &[usize],
) -> Result<T, OpsError> {
    let dims = logits.dims();
    if dims.is_empty() || dims[dims.len() - 1] == 0 {
        return Err(invalid(
            "cross_entropy_with_logits",
            "nonempty class axis required",
        ));
    }
    let classes = dims[dims.len() - 1];
    let rows = Kahen::new(&dims[..dims.len() - 1])?.numel();
    if rows == 0 || rows != targets.len() {
        return Err(invalid(
            "cross_entropy_with_logits",
            "one target per nonempty logit row required",
        ));
    }
    for &target in targets {
        if target >= classes {
            return Err(OpsError::IndexOutOfBounds {
                axis: dims.len() - 1,
                index: target,
                dim: classes,
            });
        }
    }
    let mut loss = T::zero();
    let count = T::from_count(rows);
    let mut index = vec![0; dims.len()];
    for (row, &target) in targets.iter().enumerate() {
        let mut linear = row;
        for axis in (0..dims.len() - 1).rev() {
            index[axis] = linear % dims[axis];
            linear /= dims[axis];
        }
        let mut maximum = None;
        for class in 0..classes {
            index[dims.len() - 1] = class;
            let value = *logits.get(&index)?;
            if !FloatMath::is_finite(value) {
                return Err(OpsError::NonFiniteInput {
                    operation: "cross_entropy_with_logits",
                    indices: index.clone(),
                });
            }
            if maximum.is_none_or(|m| value > m) {
                maximum = Some(value);
            }
        }
        let maximum = maximum.expect("positive class count");
        let mut exp_sum = T::zero();
        for class in 0..classes {
            index[dims.len() - 1] = class;
            exp_sum = exp_sum + FloatMath::exp(*logits.get(&index)? - maximum);
        }
        index[dims.len() - 1] = target;
        let chosen = *logits.get(&index)?;
        let difference = maximum - chosen;
        let scaled = if FloatMath::is_finite(difference) {
            difference / count
        } else {
            maximum / count - chosen / count
        };
        loss = loss + scaled + FloatMath::ln(exp_sum) / count;
    }
    if !FloatMath::is_finite(loss) {
        return Err(invalid(
            "cross_entropy_with_logits",
            "mean loss outside element range",
        ));
    }

    Ok(loss)
}
/// Mean squared error over identically shaped tensors. Empty inputs fail;
/// nonfinite operands or unrepresentable squared differences return an error.
///
/// # Examples
///
/// ```
/// use kernix_core::{Order,shape::Kahen,tensor::Tensor};
/// use kernix_ops::neural::*;
/// let a=Tensor::from_vec(vec![1.0f32;16],Kahen::new([1,1,4,4])?,Order::C)?;
/// assert_eq!(mse_loss(&a,&a)?,0.0);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn mse_loss<T: FloatMath, A: TensorInput<T>, B: TensorInput<T>>(
    prediction: &A,
    target: &B,
) -> Result<T, OpsError> {
    if prediction.dims() != target.dims() {
        return Err(invalid(
            "mse_loss",
            "prediction and target shapes must match",
        ));
    }
    let delta = crate::zip_map(prediction, target, |a, b| *a - *b)?;
    if delta.numel() == 0 {
        return Err(OpsError::EmptyReduction);
    }
    let mut mean = T::zero();
    for (i, &v) in delta.storage_slice().iter().enumerate() {
        let sq = v * v;
        if !FloatMath::is_finite(sq) {
            return Err(OpsError::NonFiniteInput {
                operation: "mse_loss",
                indices: vec![i],
            });
        }
        mean = mean + sq / T::from_count(delta.numel());
    }
    Ok(mean)
}
/// Mean binary cross-entropy from finite logits and targets in `[0,1]`, of equal
/// shapes. Uses stable softplus branches; no probabilities are materialized.
/// Empty input, invalid targets or nonfinite logits return an error.
///
/// # Examples
///
/// ```
/// use kernix_core::{Order,shape::Kahen,tensor::Tensor};
/// use kernix_ops::neural::*;
/// let a=Tensor::from_vec(vec![1.0f32;16],Kahen::new([1,1,4,4])?,Order::C)?;
/// let loss=binary_cross_entropy_with_logits(&a,&a)?;
/// assert!((loss-(-1.0f32).exp().ln_1p()).abs()<1e-6);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn binary_cross_entropy_with_logits<T: FloatMath, A: TensorInput<T>, B: TensorInput<T>>(
    logits: &A,
    targets: &B,
) -> Result<T, OpsError> {
    if logits.dims() != targets.dims() {
        return Err(invalid(
            "binary_cross_entropy_with_logits",
            "logits and target shapes must match",
        ));
    }
    let losses = crate::try_zip_map(logits, targets, |x, y| {
        if !FloatMath::is_finite(*x) || !FloatMath::is_finite(*y) || *y < T::zero() || *y > T::one()
        {
            return Err(invalid(
                "binary_cross_entropy_with_logits",
                "finite logits and targets in `[0,1]` required",
            ));
        }
        let high = if *x > T::zero() { *x } else { T::zero() };
        Ok(high - *x * *y + FloatMath::ln_1p(FloatMath::exp(-FloatMath::abs(*x))))
    })?;
    if losses.numel() == 0 {
        return Err(OpsError::EmptyReduction);
    }
    let mut mean = T::zero();
    for &v in losses.storage_slice() {
        mean = mean + v / T::from_count(losses.numel());
    }
    Ok(mean)
}
/// Normalize the last normalized_rank axes independently for each leading group,
/// using population Welford variance and positive finite epsilon. Allocates C-order;
/// Contiguous nonempty input uses one output buffer and per-group stack statistics;
/// strided/empty inputs retain the general checked composition. No learned scale/bias
/// is implicit. Invalid rank/epsilon/reduction, nonfinite data
/// or unrepresentable statistics/results return errors. Variance overflow is rejected
/// rather than silently normalizing finite inputs to zero. A scaled combination of
/// sqrt(variance) and sqrt(epsilon) avoids overflow in variance + epsilon.
///
/// # Examples
///
/// ```
/// use kernix_core::{Order,shape::Kahen,tensor::Tensor};
/// use kernix_ops::neural::*;
/// let a=Tensor::from_vec(vec![1.0f32;16],Kahen::new([1,1,4,4])?,Order::C)?;
/// let out=layer_norm(&a,2,1e-5)?;
/// assert_eq!(out.storage_slice(),&[0.0;16]);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn layer_norm<T: FloatMath + AddAssign, A: TensorInput<T>>(
    input: &A,
    normalized_rank: usize,
    epsilon: T,
) -> Result<Tensor<T>, OpsError> {
    validate_layer_norm(input.dims().len(), normalized_rank, epsilon)?;
    if let Some(data) = input.as_c_contiguous_slice() {
        let shape = Kahen::new(input.dims())?;
        if !data.is_empty() && data.len() == shape.numel() {
            let width = input.dims()[input.dims().len() - normalized_rank..]
                .iter()
                .product();
            return layer_norm_dense(data, None, shape, width, epsilon);
        }
    }
    Ok(layer_norm_parts(input, normalized_rank, epsilon)?.0)
}

fn validate_layer_norm<T: FloatMath>(
    rank: usize,
    normalized_rank: usize,
    epsilon: T,
) -> Result<(), OpsError> {
    if normalized_rank == 0
        || normalized_rank > rank
        || !FloatMath::is_finite(epsilon)
        || epsilon <= T::zero()
    {
        return Err(invalid(
            "layer_norm",
            "normalized_rank in 1..=rank and positive finite epsilon required",
        ));
    }
    Ok(())
}

fn layer_norm_stddev<T: FloatMath>(variance: T, epsilon_root: T) -> Result<T, OpsError> {
    if !FloatMath::is_finite(variance) {
        return Err(invalid(
            "layer_norm",
            "finite representable statistics required",
        ));
    }
    let variance_root = FloatMath::sqrt(variance);
    let (high, low) = if variance_root > epsilon_root {
        (variance_root, epsilon_root)
    } else {
        (epsilon_root, variance_root)
    };
    let ratio = low / high;
    let denominator = high * FloatMath::sqrt(T::one() + ratio * ratio);
    if !FloatMath::is_finite(denominator) || denominator <= T::zero() {
        return Err(invalid(
            "layer_norm",
            "positive representable standard deviation required",
        ));
    }
    Ok(denominator)
}

// Nonempty contiguous groups share the reference recurrence and scalar operation
// order. Only output storage is allocated; statistics live on the stack per group.
fn layer_norm_dense<T: FloatMath>(
    data: &[T],
    gradient: Option<&[T]>,
    shape: Kahen,
    width: usize,
    epsilon: T,
) -> Result<Tensor<T>, OpsError> {
    let mut output = Tensor::zeros(shape.dims())?;
    let epsilon_root = FloatMath::sqrt(epsilon);
    for (group, (row, out)) in data
        .chunks_exact(width)
        .zip(output.storage_slice_mut().chunks_exact_mut(width))
        .enumerate()
    {
        let mut mean = T::zero();
        let mut m2 = T::zero();
        for (i, &x) in row.iter().enumerate() {
            let delta = x - mean;
            mean = mean + delta / T::from_count(i + 1);
            let delta2 = x - mean;
            m2 = m2 + delta * delta2;
        }
        let variance = m2 / T::from_count(width);
        let variance = if variance < T::zero() {
            T::zero()
        } else {
            variance
        };
        let stddev = layer_norm_stddev(variance, epsilon_root)?;
        if let Some(gradient) = gradient {
            let g = &gradient[group * width..(group + 1) * width];
            let mut mean_g = T::zero();
            let mut mean_gy = T::zero();
            for (i, (&x, &g)) in row.iter().zip(g).enumerate() {
                let centered = x - mean;
                let y = centered / stddev;
                if !FloatMath::is_finite(centered) || !FloatMath::is_finite(y) {
                    return Err(invalid(
                        "layer_norm",
                        "finite representable input and output required",
                    ));
                }
                let gy = g * y;
                mean_g = mean_g + (g - mean_g) / T::from_count(i + 1);
                mean_gy = mean_gy + (gy - mean_gy) / T::from_count(i + 1);
            }
            for ((&x, &g), out) in row.iter().zip(g).zip(out) {
                let y = (x - mean) / stddev;
                let value = ((g - mean_g) - y * mean_gy) / stddev;
                if !FloatMath::is_finite(value) {
                    return Err(invalid(
                        "layer_norm_backward",
                        "finite representable derivative required",
                    ));
                }
                *out = value;
            }
        } else {
            for (&x, out) in row.iter().zip(out) {
                let centered = x - mean;
                let value = centered / stddev;
                if !FloatMath::is_finite(centered) || !FloatMath::is_finite(value) {
                    return Err(invalid(
                        "layer_norm",
                        "finite representable input and output required",
                    ));
                }
                *out = value;
            }
        }
    }
    Ok(output)
}

fn layer_norm_parts<T: FloatMath + AddAssign, A: TensorInput<T>>(
    input: &A,
    normalized_rank: usize,
    epsilon: T,
) -> Result<(Tensor<T>, Tensor<T>), OpsError> {
    let rank = input.dims().len();
    validate_layer_norm(rank, normalized_rank, epsilon)?;
    let axes: Vec<isize> = ((rank - normalized_rank)..rank)
        .map(|x| x as isize)
        .collect();
    let (mean, variance) = crate::ops::mean_variance_axes(input, &axes, true)?;
    let epsilon_root = FloatMath::sqrt(epsilon);
    let stddev = crate::try_map(&variance, |v| layer_norm_stddev(*v, epsilon_root))?;
    let centered = crate::sub(input, &mean)?;
    let normalized = crate::try_zip_map(&centered, &stddev, |x, std| {
        let value = *x / *std;
        if !FloatMath::is_finite(*x) || !FloatMath::is_finite(value) {
            return Err(invalid(
                "layer_norm",
                "finite representable input and output required",
            ));
        }
        Ok(value)
    })?;
    Ok((normalized, stddev))
}

/// Input VJP of [`layer_norm`] with an exact-shape finite upstream gradient.
/// Recomputes the forward population statistics. Contiguous nonempty inputs/seeds
/// allocate one C-order output; other layouts use general scratch/output;
/// no graph or higher-order derivative is recorded. For each normalized group,
/// computes `(g - mean(g) - normalized * mean(g * normalized)) / stddev`.
/// Invalid geometry/epsilon, empty nonempty-output reductions and unrepresentable
/// statistics or derivatives fail explicitly. Scale/bias gradients can be composed
/// with ordinary broadcast multiply/add (or their Tape counterparts).
///
/// ```
/// use kernix_core::tensor::Tensor;
/// let x = Tensor::from_values([2], vec![1.0f64, 3.0])?;
/// let g = Tensor::ones([2])?;
/// let dx = kernix_ops::neural::layer_norm_backward(&x, &g, 1, 1e-5)?;
/// assert_eq!(dx.storage_slice(), &[0.0, 0.0]);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn layer_norm_backward<T: FloatMath + AddAssign, A: TensorInput<T>, G: TensorInput<T>>(
    input: &A,
    gradient: &G,
    normalized_rank: usize,
    epsilon: T,
) -> Result<Tensor<T>, OpsError> {
    if input.dims() != gradient.dims() {
        return Err(invalid(
            "layer_norm_backward",
            "gradient must match input shape exactly",
        ));
    }
    validate_layer_norm(input.dims().len(), normalized_rank, epsilon)?;
    if let (Some(data), Some(gradient)) = (
        input.as_c_contiguous_slice(),
        gradient.as_c_contiguous_slice(),
    ) {
        let shape = Kahen::new(input.dims())?;
        if !data.is_empty() && data.len() == shape.numel() && gradient.len() == data.len() {
            let width = input.dims()[input.dims().len() - normalized_rank..]
                .iter()
                .product();
            return layer_norm_dense(data, Some(gradient), shape, width, epsilon);
        }
    }
    let (normalized, stddev) = layer_norm_parts(input, normalized_rank, epsilon)?;
    let axes: Vec<isize> = ((input.dims().len() - normalized_rank)..input.dims().len())
        .map(|x| x as isize)
        .collect();
    let mean_g = crate::ops::mean_welford_axes(gradient, &axes, true)?;
    let mean_gy = crate::ops::mean_welford_axes(&crate::mul(gradient, &normalized)?, &axes, true)?;
    let centered_g = crate::sub(gradient, &mean_g)?;
    let numerator = crate::sub(&centered_g, &crate::mul(&normalized, &mean_gy)?)?;
    crate::try_zip_map(&numerator, &stddev, |g, std| {
        let value = *g / *std;
        if !FloatMath::is_finite(value) {
            return Err(invalid(
                "layer_norm_backward",
                "finite representable derivative required",
            ));
        }
        Ok(value)
    })
}
