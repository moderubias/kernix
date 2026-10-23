//! Explicit reverse-mode differentiation: own the tape, values and gradient lifetime.
//!
//! ```
//! use kernix::{autograd::Tape, Kahen, Order, Tensor};
//! let mut tape=Tape::<f64>::new();
//! let x=tape.variable(Tensor::from_vec(vec![2.0,3.0],Kahen::new([2])?,Order::C)?)?;
//! let square=tape.mul(&x,&x)?;
//! let loss=tape.sum(&square)?;
//! let gradients=tape.backward(&loss)?;
//! assert_eq!(gradients.get(&x)?.unwrap().storage_slice(),&[4.0,6.0]);
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
use crate::{
    FloatMath, Kahen, OpsError, Order, ShapeError, Tensor, TensorError, neural::Conv2dOptions,
};
use alloc::{rc::Rc, vec, vec::Vec};
use core::{mem::align_of, ops::AddAssign};
use thiserror::Error;

/// Floating dtype supported by portable training (f32/f64 implementations provided).
/// Arithmetic preserves the selected dtype; conversion is explicit.
pub trait TrainFloat: FloatMath + crate::MatmulElement + AddAssign + Send + Sync + 'static {
    /// Checkpoint dtype identifier: one for f32, two for f64.
    const DTYPE: u8;
    /// Convert from f64 using this dtype's normal rounding rules.
    fn from_f64(value: f64) -> Self;
    /// Widen to f64; f32 values are represented exactly.
    fn to_f64(self) -> f64;
}
impl TrainFloat for f32 {
    const DTYPE: u8 = 1;
    fn from_f64(value: f64) -> Self {
        value as f32
    }
    fn to_f64(self) -> f64 {
        self as f64
    }
}
impl TrainFloat for f64 {
    const DTYPE: u8 = 2;
    fn from_f64(value: f64) -> Self {
        value
    }
    fn to_f64(self) -> f64 {
        self
    }
}

/// Differentiation, validation or underlying tensor/kernel failure.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum AutogradError {
    /// A kernel rejected its operands or parameters.
    #[error(transparent)]
    Ops(#[from] OpsError),
    /// Tensor storage/layout validation failed.
    #[error(transparent)]
    Tensor(#[from] TensorError),
    /// Shape validation failed.
    #[error(transparent)]
    Shape(#[from] ShapeError),
    /// Handle belongs to another tape, a cleared tape or another graph snapshot.
    #[error("variable or gradients belong to another tape generation")]
    ForeignVariable,
    /// A value or derivative exceeded the finite element domain.
    #[error("nonfinite value or gradient at node {node}")]
    NonFinite {
        /// Node index at which validation failed.
        node: usize,
    },
    /// The default backward seed requires a rank-zero scalar.
    #[error("backward requires a rank-zero loss; use backward_with for a tensor output")]
    ExpectedScalar,
    /// Seed or accumulation gradients do not match the expected geometry/snapshot.
    #[error("gradient shape or graph snapshot mismatch")]
    GradientShape,
}
/// Cheap graph handle. Values and derivatives remain owned by Tape/Gradients.
/// Handles from other tapes or a cleared generation are rejected, including after
/// an old tape is dropped (the ownership token remains alive in the handle).
#[derive(Debug, Clone)]
pub struct Variable {
    index: usize,
    owner: Rc<()>,
}
#[derive(Clone)]
enum Op<T> {
    Leaf,
    Add(usize, usize),
    Sub(usize, usize),
    Mul(usize, usize),
    Div(usize, usize),
    Matmul(usize, usize),
    Sum(usize, bool),
    ReduceAxes(usize, Vec<usize>, bool),
    Reshape(usize),
    Permute(usize, Vec<usize>),
    Slice(usize, usize, usize, usize, isize),
    Relu(usize),
    Tanh(usize),
    Sigmoid(usize),
    Exp(usize),
    Ln(usize),
    Sqrt(usize),
    Softmax(usize, isize, bool),
    LayerNorm(usize, usize, T),
    Conv(usize, usize, Conv2dOptions),
    Pool(usize, [usize; 2], [usize; 2], bool),
    CrossEntropy(usize, Vec<usize>),
    Mse(usize, usize),
    Bce(usize, usize),
}
struct Node<T> {
    value: Tensor<T>,
    op: Op<T>,
    requires_grad: bool,
}
/// Application-owned reverse-mode graph. Values are materialized CPU tensors.
/// No global graph, implicit thread-local state, unsafe code or retained optimizer
/// references are used. Drop/clear releases the graph. This first version supports
/// first-order derivatives; use plain operations for inference without any tape.
pub struct Tape<T: TrainFloat> {
    nodes: Vec<Node<T>>,
    owner: Rc<()>,
    parameters: Vec<(Rc<()>, usize)>,
}
impl<T: TrainFloat> Default for Tape<T> {
    fn default() -> Self {
        Self::new()
    }
}
macro_rules! binary {
    ($name:ident,$op:ident) => {
        #[doc=concat!("Record broadcasting `",stringify!($name),"` and its VJP. Inputs must belong to this tape.\n\nSee the module example; replace `tape.mul(&x,&x)` with this operation.\nResults and gradients are finite-checked; scalar arithmetic retains dtype rounding.")]
        pub fn $name(&mut self,left:&Variable,right:&Variable)->Result<Variable,AutogradError>{
            let a=self.index(left)?;let b=self.index(right)?;
            let value=crate::$name(&self.nodes[a].value,&self.nodes[b].value)?;
            self.record(value,Op::$op(a,b),self.nodes[a].requires_grad||self.nodes[b].requires_grad)
        }
    };
}
macro_rules! unary {
    ($name:ident,$op:ident) => {
        #[doc=concat!("Record elementwise `",stringify!($name),"` and its VJP; see the module's explicit tape example.\nNonfinite forward results or derivatives are errors. ReLU uses derivative zero at zero.")]
        pub fn $name(&mut self,input:&Variable)->Result<Variable,AutogradError>{
            let a=self.index(input)?;let value=crate::$name(&self.nodes[a].value)?;
            self.record(value,Op::$op(a),self.nodes[a].requires_grad)
        }
    };
}
impl<T: TrainFloat> Tape<T> {
    /// Start an empty graph with a fresh ownership generation; performs no allocation
    /// for tensor data. See the module example for a complete differentiation cycle.
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            owner: Rc::new(()),
            parameters: Vec::new(),
        }
    }
    /// Number of stored values, including constants and intermediate operations.
    pub fn len(&self) -> usize {
        self.nodes.len()
    }
    /// Whether this graph stores no values.
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }
    /// Release graph data and invalidate all existing handles for this tape. Already
    /// returned Gradients remain readable through their own generation handles.
    pub fn clear(&mut self) {
        self.nodes.clear();
        self.parameters.clear();
        self.owner = Rc::new(());
    }
    /// Add a finite-valued trainable leaf. Takes ownership, materializing noncanonical
    /// logical layouts into C order. No other operation can mutate this stored value.
    pub fn variable(&mut self, value: Tensor<T>) -> Result<Variable, AutogradError> {
        self.record(value, Op::Leaf, true)
    }
    /// Add a finite-valued constant. Its derivatives are not retained, even if it
    /// contributes to a trainable descendant's forward computation.
    pub fn constant(&mut self, value: Tensor<T>) -> Result<Variable, AutogradError> {
        self.record(value, Op::Leaf, false)
    }
    /// Borrow an immutable value after validating the handle's graph generation.
    pub fn value(&self, var: &Variable) -> Result<&Tensor<T>, AutogradError> {
        Ok(&self.nodes[self.index(var)?].value)
    }
    /// Copy a value out of the graph, breaking gradient history. Use as a constant
    /// on the next tape for truncated recurrent sequences, or keep as inference state.
    pub fn detach(&self, var: &Variable) -> Result<Tensor<T>, AutogradError> {
        Ok(self.value(var)?.clone())
    }
    pub(crate) fn track_parameter(
        &mut self,
        owner: &Rc<()>,
        value: &Tensor<T>,
    ) -> Result<Variable, AutogradError> {
        if let Some((_, index)) = self.parameters.iter().find(|(id, _)| Rc::ptr_eq(id, owner)) {
            return Ok(Variable {
                index: *index,
                owner: self.owner.clone(),
            });
        }
        let variable = self.variable(value.clone())?;
        self.parameters.push((owner.clone(), variable.index));
        Ok(variable)
    }
    fn index(&self, v: &Variable) -> Result<usize, AutogradError> {
        if Rc::ptr_eq(&self.owner, &v.owner) && v.index < self.nodes.len() {
            Ok(v.index)
        } else {
            Err(AutogradError::ForeignVariable)
        }
    }
    fn record(
        &mut self,
        value: Tensor<T>,
        op: Op<T>,
        requires_grad: bool,
    ) -> Result<Variable, AutogradError> {
        let value = if value.layout().is_c_contiguous()
            && value.origin() == 0
            && value.storage_len() == value.numel()
        {
            value
        } else {
            value.to_owned_contiguous(Order::C)?
        };
        finite(&value, self.nodes.len())?;
        let index = self.nodes.len();
        self.nodes.push(Node {
            value,
            op,
            requires_grad,
        });
        Ok(Variable {
            index,
            owner: self.owner.clone(),
        })
    }
    binary!(add, Add);
    binary!(sub, Sub);
    binary!(mul, Mul);
    binary!(div, Div);
    unary!(relu, Relu);
    unary!(tanh, Tanh);
    unary!(sigmoid, Sigmoid);
    unary!(exp, Exp);
    unary!(ln, Ln);
    unary!(sqrt, Sqrt);
    /// Record vector, matrix or batched matmul with the same geometry as [`crate::matmul`].
    /// Batch axes broadcast right-aligned. VJPs sum broadcast contributions back to
    /// each operand, including empty batches; vector dot products produce scalars.
    /// Scalars are invalid operands. Gradients allocate CPU outputs, without copying
    /// operands for their virtual singleton axes or transposed contraction views.
    pub fn matmul(&mut self, left: &Variable, right: &Variable) -> Result<Variable, AutogradError> {
        let a = self.index(left)?;
        let b = self.index(right)?;
        let value = crate::matmul(&self.nodes[a].value, &self.nodes[b].value)?;
        self.record(
            value,
            Op::Matmul(a, b),
            self.nodes[a].requires_grad || self.nodes[b].requires_grad,
        )
    }
    /// Record a last-axis dense transform of `[...,in]` input with `[out,in]`
    /// weights, including vector inputs and empty leading batch axes. An optional
    /// bias broadcasts to the product (conventionally `[out]`), preserving the
    /// existing permute/matmul/add composition and shared parameter identity.
    pub fn linear(
        &mut self,
        input: &Variable,
        weight: &Variable,
        bias: Option<&Variable>,
    ) -> Result<Variable, AutogradError> {
        let w = self.permute(weight, &[1, 0])?;
        let value = self.matmul(input, &w)?;
        if let Some(bias) = bias {
            self.add(&value, bias)
        } else {
            Ok(value)
        }
    }
    /// Sum every element into a rank-zero value. Empty input has sum zero and
    /// an empty input gradient. See the module example for branching accumulation.
    pub fn sum(&mut self, input: &Variable) -> Result<Variable, AutogradError> {
        self.reduce(input, false)
    }
    /// Mean every element into a rank-zero value. Empty inputs return EmptyReduction.
    pub fn mean(&mut self, input: &Variable) -> Result<Variable, AutogradError> {
        self.reduce(input, true)
    }
    /// Record sum over signed axes, with optional retained singleton dimensions.
    /// Empty axes are the identity; empty reduced extents contribute zero.
    /// Invalid/duplicate axes fail as in [`crate::sum_axes`]. The VJP broadcasts
    /// its exact-shape seed to the input, allocating one input-shaped derivative.
    pub fn sum_axes(
        &mut self,
        input: &Variable,
        axes: &[isize],
        keepdims: bool,
    ) -> Result<Variable, AutogradError> {
        self.reduce_axes(input, axes, keepdims, false)
    }
    /// Record population mean over signed axes, retaining dimensions if requested.
    /// Empty reductions fail when output contains elements; empty outputs are valid.
    /// VJPs preserve dtype and scale each derivative by the reduced element count.
    pub fn mean_axes(
        &mut self,
        input: &Variable,
        axes: &[isize],
        keepdims: bool,
    ) -> Result<Variable, AutogradError> {
        self.reduce_axes(input, axes, keepdims, true)
    }
    fn reduce_axes(
        &mut self,
        input: &Variable,
        axes: &[isize],
        keepdims: bool,
        mean: bool,
    ) -> Result<Variable, AutogradError> {
        let a = self.index(input)?;
        let value = if mean {
            crate::mean_axes(&self.nodes[a].value, axes, keepdims)?
        } else {
            crate::sum_axes(&self.nodes[a].value, axes, keepdims)?
        };
        // The forward operation has already validated signed axes and duplicates.
        let rank = self.nodes[a].value.rank();
        let axes = axes
            .iter()
            .map(|&axis| {
                if axis < 0 {
                    rank.checked_add_signed(axis).expect("validated axis")
                } else {
                    axis as usize
                }
            })
            .collect();
        self.record(
            value,
            Op::ReduceAxes(a, axes, mean),
            self.nodes[a].requires_grad,
        )
    }
    fn reduce(&mut self, input: &Variable, mean: bool) -> Result<Variable, AutogradError> {
        let a = self.index(input)?;
        let v = if mean {
            crate::mean_all(&self.nodes[a].value)?
        } else {
            crate::sum_all(&self.nodes[a].value)?
        };
        self.record(
            Tensor::from_vec(vec![v], Kahen::scalar(), Order::C)?,
            Op::Sum(a, mean),
            self.nodes[a].requires_grad,
        )
    }
    /// Record a same-numel reshape. Values are materialized in logical C order;
    /// this changes graph tensor geometry, not the pure Shape API.
    pub fn reshape(
        &mut self,
        input: &Variable,
        dims: impl AsRef<[usize]>,
    ) -> Result<Variable, AutogradError> {
        let a = self.index(input)?;
        let value = self.nodes[a]
            .value
            .reshape_view(dims, Order::C)?
            .to_owned_contiguous(Order::C)?;
        self.record(value, Op::Reshape(a), self.nodes[a].requires_grad)
    }
    /// Record a complete axis permutation and its inverse-gradient permutation.
    /// Invalid or duplicate axes return a shape/layout error.
    pub fn permute(&mut self, input: &Variable, axes: &[usize]) -> Result<Variable, AutogradError> {
        let a = self.index(input)?;
        let value = self.nodes[a]
            .value
            .permute(axes)?
            .to_owned_contiguous(Order::C)?;
        self.record(
            value,
            Op::Permute(a, axes.to_vec()),
            self.nodes[a].requires_grad,
        )
    }
    /// Record an axis slice with the same start/length/nonzero signed-step contract
    /// as [`Tensor::slice_axis`]. Copies selected values into the owned tape; use a
    /// plain Tensor view for zero-copy inference. The VJP scatters the seed into a
    /// zero-filled input-shaped buffer, leaving unselected elements at zero.
    pub fn slice_axis(
        &mut self,
        input: &Variable,
        axis: usize,
        start: usize,
        len: usize,
        step: isize,
    ) -> Result<Variable, AutogradError> {
        let a = self.index(input)?;
        let value = self.nodes[a]
            .value
            .slice_axis(axis, start, len, step)?
            .to_owned_contiguous(Order::C)?;
        self.record(
            value,
            Op::Slice(a, axis, start, len, step),
            self.nodes[a].requires_grad,
        )
    }
    /// Record finite-input softmax on a signed axis. Its VJP uses the full
    /// Jacobian-vector identity, not independent sigmoid derivatives.
    pub fn softmax(&mut self, input: &Variable, axis: isize) -> Result<Variable, AutogradError> {
        self.normalized(input, axis, false)
    }
    /// Record log-softmax with a stable VJP, rejecting unrepresentable log values.
    pub fn log_softmax(
        &mut self,
        input: &Variable,
        axis: isize,
    ) -> Result<Variable, AutogradError> {
        self.normalized(input, axis, true)
    }
    fn normalized(
        &mut self,
        input: &Variable,
        axis: isize,
        log: bool,
    ) -> Result<Variable, AutogradError> {
        let a = self.index(input)?;
        let value = if log {
            crate::log_softmax(&self.nodes[a].value, axis)?
        } else {
            crate::softmax(&self.nodes[a].value, axis)?
        };
        self.record(
            value,
            Op::Softmax(a, axis, log),
            self.nodes[a].requires_grad,
        )
    }
    /// Record population layer normalization over the last `normalized_rank` axes.
    /// Epsilon is positive, finite and fixed (not differentiated). Input/statistics
    /// and derivatives must be finite/representable; see [`crate::neural::layer_norm`].
    /// Backward recomputes statistics, allocating CPU scratch. Compose `mul`/`add`
    /// with trainable broadcast scale/bias for affine normalization.
    pub fn layer_norm(
        &mut self,
        input: &Variable,
        normalized_rank: usize,
        epsilon: T,
    ) -> Result<Variable, AutogradError> {
        let a = self.index(input)?;
        let value = crate::neural::layer_norm(&self.nodes[a].value, normalized_rank, epsilon)?;
        self.record(
            value,
            Op::LayerNorm(a, normalized_rank, epsilon),
            self.nodes[a].requires_grad,
        )
    }
    /// Record grouped/dilated NCHW cross-correlation, differentiating both input
    /// and weights. Add a broadcast bias separately. See neural::conv2d geometry.
    pub fn conv2d(
        &mut self,
        input: &Variable,
        weight: &Variable,
        options: Conv2dOptions,
    ) -> Result<Variable, AutogradError> {
        let a = self.index(input)?;
        let b = self.index(weight)?;
        let value = crate::neural::conv2d(&self.nodes[a].value, &self.nodes[b].value, options)?;
        self.record(
            value,
            Op::Conv(a, b, options),
            self.nodes[a].requires_grad || self.nodes[b].requires_grad,
        )
    }
    /// Record NCHW average pooling without padding, summing overlapping VJP contributions.
    pub fn avg_pool2d(
        &mut self,
        input: &Variable,
        window: [usize; 2],
        stride: [usize; 2],
    ) -> Result<Variable, AutogradError> {
        self.pool(input, window, stride, false)
    }
    /// Record NCHW maximum pooling. First ties/NaNs select the VJP winner; tape
    /// inputs themselves must be finite. ReLU/sqrt/domain conventions are explicit.
    pub fn max_pool2d(
        &mut self,
        input: &Variable,
        window: [usize; 2],
        stride: [usize; 2],
    ) -> Result<Variable, AutogradError> {
        self.pool(input, window, stride, true)
    }
    fn pool(
        &mut self,
        input: &Variable,
        w: [usize; 2],
        s: [usize; 2],
        max: bool,
    ) -> Result<Variable, AutogradError> {
        let a = self.index(input)?;
        let value = if max {
            crate::neural::max_pool2d(&self.nodes[a].value, w, s)?
        } else {
            crate::neural::avg_pool2d(&self.nodes[a].value, w, s)?
        };
        self.record(value, Op::Pool(a, w, s, max), self.nodes[a].requires_grad)
    }
    /// Record mean categorical cross-entropy of logits `[...,classes]` and integer
    /// labels. Labels are copied into the graph; no target derivative is defined.
    pub fn cross_entropy(
        &mut self,
        logits: &Variable,
        labels: &[usize],
    ) -> Result<Variable, AutogradError> {
        let a = self.index(logits)?;
        let loss = crate::neural::cross_entropy_with_logits(&self.nodes[a].value, labels)?;
        self.record(
            Tensor::from_vec(vec![loss], Kahen::scalar(), Order::C)?,
            Op::CrossEntropy(a, labels.to_vec()),
            self.nodes[a].requires_grad,
        )
    }
    /// Record mean squared error of equal-shaped tensors. Both operands are
    /// differentiable when trainable; constants do not retain gradients.
    pub fn mse(
        &mut self,
        prediction: &Variable,
        target: &Variable,
    ) -> Result<Variable, AutogradError> {
        let a = self.index(prediction)?;
        let b = self.index(target)?;
        let loss = crate::neural::mse_loss(&self.nodes[a].value, &self.nodes[b].value)?;
        self.record(
            Tensor::from_vec(vec![loss], Kahen::scalar(), Order::C)?,
            Op::Mse(a, b),
            self.nodes[a].requires_grad || self.nodes[b].requires_grad,
        )
    }
    /// Record stable binary cross-entropy with logits. Targets in `[0,1]` can be
    /// trainable, in which case their derivative is -logit/count.
    pub fn binary_cross_entropy(
        &mut self,
        logits: &Variable,
        target: &Variable,
    ) -> Result<Variable, AutogradError> {
        let a = self.index(logits)?;
        let b = self.index(target)?;
        let loss = crate::neural::binary_cross_entropy_with_logits(
            &self.nodes[a].value,
            &self.nodes[b].value,
        )?;
        self.record(
            Tensor::from_vec(vec![loss], Kahen::scalar(), Order::C)?,
            Op::Bce(a, b),
            self.nodes[a].requires_grad || self.nodes[b].requires_grad,
        )
    }
    /// Differentiate a scalar with seed one, returning fresh gradients. Repeated
    /// calls do not implicitly accumulate. Use Gradients::accumulate explicitly.
    pub fn backward(&self, loss: &Variable) -> Result<Gradients<T>, AutogradError> {
        let index = self.index(loss)?;
        if !self.nodes[index].value.shape().is_scalar() {
            return Err(AutogradError::ExpectedScalar);
        }
        self.backward_with(
            loss,
            &Tensor::from_vec(vec![T::one()], Kahen::scalar(), Order::C)?,
        )
    }
    /// Compute a vector-Jacobian product with an exact-shape finite seed. Branches
    /// accumulate by parent node; constants and disconnected leaves have no gradient.
    /// No higher-order graph is retained. Singular/nonfinite derivatives are errors.
    pub fn backward_with(
        &self,
        output: &Variable,
        seed: &Tensor<T>,
    ) -> Result<Gradients<T>, AutogradError> {
        let root = self.index(output)?;
        if seed.shape().dims() != self.nodes[root].value.shape().dims() {
            return Err(AutogradError::GradientShape);
        }
        finite(seed, root)?;
        let mut values = vec![None; self.nodes.len()];
        if self.nodes[root].requires_grad {
            values[root] = Some(seed.to_owned_contiguous(Order::C)?);
        }
        for index in (0..=root).rev() {
            let (parent_grads, current) = values.split_at_mut(index);
            let Some(g) = current[0].as_ref() else {
                continue;
            };
            let node = &self.nodes[index];
            let value = |i: usize| &self.nodes[i].value;
            let mut send =
                |i: usize, gradient: Tensor<T>| accumulate(&self.nodes, parent_grads, i, gradient);
            match &node.op {
                Op::Leaf => (),
                Op::Add(a, b) => {
                    send(*a, g.clone())?;
                    send(*b, g.clone())?;
                }
                Op::Sub(a, b) => {
                    send(*a, g.clone())?;
                    send(*b, crate::neg(g)?)?;
                }
                Op::Mul(a, b) => {
                    send(*a, crate::mul(g, value(*b))?)?;
                    send(*b, crate::mul(g, value(*a))?)?;
                }
                Op::Div(a, b) => {
                    let da = crate::div(g, value(*b))?;
                    let db = crate::neg(&crate::mul(&da, &node.value)?)?;
                    send(*a, da)?;
                    send(*b, db)?;
                }
                Op::Matmul(a, b) => {
                    let av = value(*a);
                    let bv = value(*b);
                    let left_vector = av.rank() == 1;
                    let right_vector = bv.rank() == 1;
                    // A shared right matrix/vector contracts all leading input rows
                    // directly. This avoids allocating a per-batch weight derivative
                    // and then reducing it; all three reshapes borrow existing data.
                    let k = av.shape().dims()[av.rank() - 1];
                    let n = if right_vector {
                        1
                    } else {
                        bv.shape().dims()[bv.rank() - 1]
                    };
                    if av.rank() > 2 && bv.rank() <= 2 && k > 0 && n > 0 {
                        let rows = av.numel() / k;
                        let left = av.reshape_view([rows, k], Order::C)?;
                        let right = bv.reshape_view([k, n], Order::C)?;
                        let seed = g.reshape_view([rows, n], Order::C)?;
                        if self.nodes[*a].requires_grad {
                            let dx = crate::matmul(&seed, &right.swap_axes(0, 1)?)?;
                            send(*a, reshape_owned(dx, av.shape().dims())?)?;
                        }
                        if self.nodes[*b].requires_grad {
                            let dw = crate::matmul(&left.swap_axes(0, 1)?, &seed)?;
                            send(*b, reshape_owned(dw, bv.shape().dims())?)?;
                        }
                        continue;
                    }
                    let left = if left_vector {
                        av.reshape_view([1, av.numel()], Order::C)?
                    } else {
                        av.view()?
                    };
                    let right = if right_vector {
                        bv.reshape_view([bv.numel(), 1], Order::C)?
                    } else {
                        bv.view()?
                    };
                    let seed = if left_vector || right_vector {
                        let batch_rank =
                            g.rank() - usize::from(!left_vector) - usize::from(!right_vector);
                        let mut dims = g.shape().dims()[..batch_rank].to_vec();
                        dims.push(left.shape().dims()[left.rank() - 2]);
                        dims.push(right.shape().dims()[right.rank() - 1]);
                        g.reshape_view(dims, Order::C)?
                    } else {
                        g.view()?
                    };
                    if self.nodes[*a].requires_grad {
                        let rank = right.rank();
                        let mut dx = crate::matmul(&seed, &right.swap_axes(rank - 2, rank - 1)?)?;
                        if left_vector {
                            let mut dims = dx.shape().dims().to_vec();
                            dims.remove(dims.len() - 2);
                            dx = reshape_owned(dx, dims)?;
                        }
                        send(*a, dx)?;
                    }
                    if self.nodes[*b].requires_grad {
                        let rank = left.rank();
                        let mut dw = crate::matmul(&left.swap_axes(rank - 2, rank - 1)?, &seed)?;
                        if right_vector {
                            let mut dims = dw.shape().dims().to_vec();
                            dims.pop();
                            dw = reshape_owned(dw, dims)?;
                        }
                        send(*b, dw)?;
                    }
                }
                Op::ReduceAxes(a, axes, mean) => {
                    let mut dims = value(*a).shape().dims().to_vec();
                    for &axis in axes {
                        dims[axis] = 1;
                    }
                    let seed = g.reshape_view(dims, Order::C)?;
                    let expanded = seed.broadcast_to(value(*a).shape().dims())?;
                    let count = if *mean && !value(*a).is_empty() {
                        // A nonempty validated tensor bounds every subproduct.
                        T::from_count(
                            axes.iter()
                                .map(|&axis| value(*a).shape().dims()[axis])
                                .product(),
                        )
                    } else {
                        T::one()
                    };
                    send(*a, crate::map(&expanded, |x| *x / count)?)?;
                }
                Op::Sum(a, mean) => {
                    let count = if *mean {
                        T::from_count(value(*a).numel())
                    } else {
                        T::one()
                    };
                    send(
                        *a,
                        Tensor::filled(
                            value(*a).shape().clone(),
                            Order::C,
                            align_of::<T>(),
                            g.storage_slice()[0] / count,
                        )?,
                    )?;
                }
                Op::Reshape(a) => send(
                    *a,
                    g.reshape_view(value(*a).shape().dims(), Order::C)?
                        .to_owned_contiguous(Order::C)?,
                )?,
                Op::Permute(a, axes) => {
                    let mut inv = vec![0; axes.len()];
                    for (i, &axis) in axes.iter().enumerate() {
                        inv[axis] = i;
                    }
                    send(*a, g.permute(inv)?.to_owned_contiguous(Order::C)?)?;
                }
                Op::Slice(a, axis, start, len, step) => {
                    let mut dx = Tensor::zeros(value(*a).shape().dims())?;
                    {
                        let mut destination =
                            dx.view_mut()?.slice_axis(*axis, *start, *len, *step)?;
                        crate::assign(&mut destination, g)?;
                    }
                    send(*a, dx)?;
                }
                Op::Relu(a) => send(
                    *a,
                    crate::zip_map(
                        g,
                        value(*a),
                        |g, x| if *x > T::zero() { *g } else { T::zero() },
                    )?,
                )?,
                Op::Tanh(a) => send(
                    *a,
                    crate::zip_map(g, &node.value, |g, y| *g * (T::one() - *y * *y))?,
                )?,
                Op::Sigmoid(a) => send(
                    *a,
                    crate::zip_map(g, &node.value, |g, y| *g * *y * (T::one() - *y))?,
                )?,
                Op::Exp(a) => send(*a, crate::mul(g, &node.value)?)?,
                Op::Ln(a) => send(*a, crate::div(g, value(*a))?)?,
                Op::Sqrt(a) => send(
                    *a,
                    crate::zip_map(g, &node.value, |g, y| (*g / T::from_count(2)) / *y)?,
                )?,
                Op::Softmax(a, axis, log) => {
                    let gradient = if *log {
                        let sum = crate::sum_axes(g, &[*axis], true)?;
                        crate::sub(g, &crate::mul(&crate::exp(&node.value)?, &sum)?)?
                    } else {
                        let dot = crate::sum_axes(&crate::mul(g, &node.value)?, &[*axis], true)?;
                        crate::mul(&node.value, &crate::sub(g, &dot)?)?
                    };
                    send(*a, gradient)?;
                }
                Op::LayerNorm(a, rank, epsilon) => {
                    send(
                        *a,
                        crate::neural::layer_norm_backward(value(*a), g, *rank, *epsilon)?,
                    )?;
                }
                Op::Conv(a, b, options) => {
                    let (dx, dw) =
                        crate::neural::conv2d_backward(value(*a), value(*b), g, *options)?;
                    send(*a, dx)?;
                    send(*b, dw)?;
                }
                Op::Pool(a, w, s, max) => {
                    let dx = if *max {
                        crate::neural::max_pool2d_backward(value(*a), g, *w, *s)?
                    } else {
                        crate::neural::avg_pool2d_backward(value(*a).shape().dims(), g, *w, *s)?
                    };
                    send(*a, dx)?;
                }
                Op::CrossEntropy(a, labels) => {
                    let mut dx = crate::softmax(value(*a), -1)?;
                    let classes = value(*a).shape().dims()[value(*a).rank() - 1];
                    for (row, &target) in labels.iter().enumerate() {
                        dx.storage_slice_mut()[row * classes + target] =
                            dx.storage_slice()[row * classes + target] - T::one();
                    }
                    let scale = g.storage_slice()[0] / T::from_count(labels.len());
                    for x in dx.storage_slice_mut() {
                        *x = *x * scale;
                    }
                    send(*a, dx)?;
                }
                Op::Mse(a, b) => {
                    let scale = (g.storage_slice()[0] / T::from_count(value(*a).numel()))
                        * T::from_count(2);
                    let dx = crate::zip_map(value(*a), value(*b), |a, b| (*a - *b) * scale)?;
                    let dt = crate::neg(&dx)?;
                    send(*a, dx)?;
                    send(*b, dt)?;
                }
                Op::Bce(a, b) => {
                    let scale = g.storage_slice()[0] / T::from_count(value(*a).numel());
                    let dx =
                        crate::map(&crate::sub(&crate::sigmoid(value(*a))?, value(*b))?, |x| {
                            *x * scale
                        })?;
                    let dt = crate::map(value(*a), |x| -*x * scale)?;
                    send(*a, dx)?;
                    send(*b, dt)?;
                }
            }
        }
        Ok(Gradients {
            values,
            owner: self.owner.clone(),
        })
    }
}
fn finite<T: TrainFloat>(value: &Tensor<T>, node: usize) -> Result<(), AutogradError> {
    if value
        .storage_slice()
        .iter()
        .any(|x| !FloatMath::is_finite(*x))
    {
        Err(AutogradError::NonFinite { node })
    } else {
        Ok(())
    }
}
// Reuse a derivative's owned contiguous buffer when removing virtual axes or
// reversing a broadcast reduction. Layout validation remains checked.
fn reshape_owned<T>(
    mut value: Tensor<T>,
    dims: impl AsRef<[usize]>,
) -> Result<Tensor<T>, AutogradError> {
    let layout = value
        .layout()
        .reshape_view(dims, Order::C)
        .map_err(TensorError::from)?;
    value.set_layout(layout)?;
    Ok(value)
}
fn accumulate<T: TrainFloat>(
    nodes: &[Node<T>],
    gradients: &mut [Option<Tensor<T>>],
    index: usize,
    gradient: Tensor<T>,
) -> Result<(), AutogradError> {
    if !nodes[index].requires_grad {
        return Ok(());
    }
    let dims = nodes[index].value.shape().dims();
    let gradient = if gradient.shape().dims() == dims {
        gradient
    } else {
        let gd = gradient.shape().dims();
        if gd.len() < dims.len() {
            return Err(AutogradError::GradientShape);
        }
        let lead = gd.len() - dims.len();
        if gd[lead..]
            .iter()
            .zip(dims)
            .any(|(&from, &to)| from != to && to != 1)
        {
            return Err(AutogradError::GradientShape);
        }
        let axes: Vec<isize> = (0..gd.len())
            .filter(|&axis| axis < lead || dims[axis - lead] == 1 && gd[axis] != 1)
            .map(|x| x as isize)
            .collect();
        reshape_owned(crate::sum_axes(&gradient, &axes, true)?, dims)?
    };
    finite(&gradient, index)?;
    // These buffers belong exclusively to this backward invocation. Mutating one
    // cannot alter a primal, a retained Gradients value, or a later backward call.
    // On error the entire local result is discarded. Keep the strided fallback.
    if let Some(old) = &mut gradients[index] {
        if let (Ok(left), Ok(right)) = (
            old.as_contiguous_mut_slice(Order::C),
            gradient.as_contiguous_slice(Order::C),
        ) {
            for (old, &value) in left.iter_mut().zip(right) {
                // Preserve Add semantics for custom scalars (AddAssign can differ).
                *old = core::ops::Add::add(*old, value);
            }
            finite(old, index)?;
            return Ok(());
        }
    }
    let combined = if let Some(old) = &gradients[index] {
        crate::add(old, &gradient)?
    } else {
        gradient
    };
    finite(&combined, index)?;
    gradients[index] = Some(combined);
    Ok(())
}
/// Owned first-order derivatives for a specific graph generation/snapshot.
/// Separate backward calls start fresh, so training loops cannot accidentally
/// retain stale accumulated derivatives through hidden tape state.
pub struct Gradients<T: TrainFloat> {
    values: Vec<Option<Tensor<T>>>,
    owner: Rc<()>,
}
impl<T: TrainFloat> Gradients<T> {
    /// Borrow a derivative, or None for constants/disconnected values. Reject
    /// handles from another graph generation or later than this snapshot.
    pub fn get(&self, variable: &Variable) -> Result<Option<&Tensor<T>>, AutogradError> {
        if !Rc::ptr_eq(&self.owner, &variable.owner) || variable.index >= self.values.len() {
            return Err(AutogradError::ForeignVariable);
        }
        Ok(self.values[variable.index].as_ref())
    }
    /// Explicitly add another backward result from the same graph snapshot.
    /// Validate all sums before replacing this result, so failure is transactional.
    pub fn accumulate(&mut self, other: &Self) -> Result<(), AutogradError> {
        if !Rc::ptr_eq(&self.owner, &other.owner) || self.values.len() != other.values.len() {
            return Err(AutogradError::GradientShape);
        }
        let mut values = Vec::with_capacity(self.values.len());
        for (index, (a, b)) in self.values.iter().zip(&other.values).enumerate() {
            let v = match (a, b) {
                (Some(a), Some(b)) => Some(crate::add(a, b)?),
                (Some(a), None) => Some(a.clone()),
                (None, Some(b)) => Some(b.clone()),
                _ => None,
            };
            if let Some(v) = &v {
                finite(v, index)?;
            }
            values.push(v);
        }
        self.values = values;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unbroadcast_rejects_incompatible_equal_numel_axes() {
        let nodes = [Node {
            value: Tensor::<f64>::ones([2, 3]).unwrap(),
            op: Op::Leaf,
            requires_grad: true,
        }];
        let mut gradients = [None];
        assert!(matches!(
            accumulate(&nodes, &mut gradients, 0, Tensor::ones([1, 6]).unwrap()),
            Err(AutogradError::GradientShape)
        ));
        assert!(gradients[0].is_none());
        accumulate(&nodes, &mut gradients, 0, Tensor::ones([4, 2, 3]).unwrap()).unwrap();
        assert_eq!(gradients[0].as_ref().unwrap().storage_slice(), &[4.0; 6]);
    }
}
