//! Small composable layers with shared inference and explicit-training parameters.
//!
//! ```
//! use kernix::{nn::Linear,training::SeededRng,autograd::Tape,Kahen,Order,Tensor};
//! let layer=Linear::<f32>::new(3,2,&mut SeededRng::new(7))?;
//! let input=Tensor::from_vec(vec![1.0,2.0,3.0],Kahen::new([1,3])?,Order::C)?;
//! assert_eq!(layer.forward(&input)?.shape().dims(),&[1,2]);
//! let mut tape=Tape::new();let input=tape.constant(input)?;
//! let recorded=layer.record(&mut tape,&input)?;
//! let loss=tape.sum(&recorded.output)?;
//! let gradients=tape.backward(&loss)?;
//! assert!(gradients.get(&recorded.weight)?.is_some());
//! # Ok::<(),Box<dyn std::error::Error>>(())
//! ```
use crate::{
    Kahen, OpsError, Order, Tensor, TensorInput,
    autograd::{AutogradError, Tape, TrainFloat, Variable},
    neural::Conv2dOptions,
    training::{Parameter, SeededRng, TrainingError},
};
use core::mem::align_of;

/// Inference capability for a layer consuming/producing CPU tensors. Custom layers
/// can compose plain operations; graph recording remains an explicit separate API.
pub trait Module<T> {
    /// Allocate an inference output without recording an automatic-differentiation graph.
    fn forward(&self, input: &Tensor<T>) -> Result<Tensor<T>, OpsError>;
}
/// Recorded layer result and graph handles for its weight/bias parameter snapshots.
/// Use these handles with Gradients::get, then update the corresponding layer parameters.
#[derive(Debug, Clone)]
pub struct RecordedLayer {
    /// Output handle for subsequent differentiable operations.
    pub output: Variable,
    /// Trainable weight leaf on this tape.
    pub weight: Variable,
    /// Trainable bias leaf on this tape.
    pub bias: Variable,
}
/// Last-axis dense affine layer with `[out,in]` weights and `[out]` bias.
/// Constructors use seeded Xavier weights and zero bias. Inference and differentiable
/// recording accept vectors and arbitrary leading batch/sequence axes.
pub struct Linear<T: TrainFloat> {
    /// Trainable `[out,in]` weight matrix; replacement must remain compatible with inputs.
    pub weight: Parameter<T>,
    /// Trainable `[out]` bias, initialized to zero.
    pub bias: Parameter<T>,
}
impl<T: TrainFloat> Linear<T> {
    /// Initialize positive input/output feature counts. See the module example for
    /// the same layer used in inference and training.
    pub fn new(input: usize, output: usize, rng: &mut SeededRng) -> Result<Self, TrainingError> {
        if input == 0 || output == 0 {
            return Err(TrainingError::Invalid(
                "linear feature counts must be positive",
            ));
        }
        Ok(Self {
            weight: Parameter::xavier([output, input], rng)?,
            bias: Parameter::new(Tensor::filled(
                Kahen::new([output])?,
                Order::C,
                align_of::<T>(),
                T::zero(),
            )?)?,
        })
    }
    /// Compute inference values with optional leading batch/sequence axes, allocating
    /// no gradient graph. Errors follow neural::linear.
    pub fn forward<A: TensorInput<T>>(&self, input: &A) -> Result<Tensor<T>, OpsError> {
        if input.dims().len() == 2 {
            let output = crate::matmul(input, &self.weight.value().transpose()?)?;
            crate::add(&output, self.bias.value())
        } else {
            crate::neural::linear(input, self.weight.value(), Some(self.bias.value()))
        }
    }
    /// Snapshot parameters onto a tape and record last-axis dense computation.
    /// Returned leaves identify derivatives for this particular forward pass.
    pub fn record(
        &self,
        tape: &mut Tape<T>,
        input: &Variable,
    ) -> Result<RecordedLayer, AutogradError> {
        let weight = self.weight.track(tape)?;
        let bias = self.bias.track(tape)?;
        let output = tape.linear(input, &weight, Some(&bias))?;
        Ok(RecordedLayer {
            output,
            weight,
            bias,
        })
    }
}
impl<T: TrainFloat> Module<T> for Linear<T> {
    fn forward(&self, input: &Tensor<T>) -> Result<Tensor<T>, OpsError> {
        Linear::forward(self, input)
    }
}
/// NCHW cross-correlation layer with explicit geometry, seeded Xavier weights and
/// broadcast bias `[1,out,1,1]`. Grouped, padded and dilated inference/training agree.
pub struct Conv2d<T: TrainFloat> {
    /// Trainable `[out,in/groups,kh,kw]` filters.
    pub weight: Parameter<T>,
    /// Trainable `[1,out,1,1]` bias, initialized to zero.
    pub bias: Parameter<T>,
    options: Conv2dOptions,
}
impl<T: TrainFloat> Conv2d<T> {
    /// Construct positive compatible channels/kernel/groups. Spatial extent and
    /// address arithmetic are checked when a concrete input is supplied.
    pub fn new(
        input: usize,
        output: usize,
        kernel: [usize; 2],
        options: Conv2dOptions,
        rng: &mut SeededRng,
    ) -> Result<Self, TrainingError> {
        if input == 0
            || output == 0
            || options.groups == 0
            || input % options.groups != 0
            || output % options.groups != 0
            || kernel.contains(&0)
            || options.stride.contains(&0)
            || options.dilation.contains(&0)
        {
            return Err(TrainingError::Invalid(
                "invalid Conv2d channels/groups/kernel/step",
            ));
        }
        Ok(Self {
            weight: Parameter::xavier([output, input / options.groups, kernel[0], kernel[1]], rng)?,
            bias: Parameter::new(Tensor::filled(
                Kahen::new([1, output, 1, 1])?,
                Order::C,
                align_of::<T>(),
                T::zero(),
            )?)?,
            options,
        })
    }
    /// Inspect immutable stride/padding/dilation/groups for this layer.
    pub fn options(&self) -> Conv2dOptions {
        self.options
    }
    /// Run NCHW convolution and broadcast bias without a graph. Allocates output
    /// and a convolution intermediate; no hidden device transfer occurs.
    pub fn forward<A: TensorInput<T>>(&self, input: &A) -> Result<Tensor<T>, OpsError> {
        let output = crate::neural::conv2d(input, self.weight.value(), self.options)?;
        crate::add(&output, self.bias.value())
    }
    /// Snapshot weights/bias onto a tape and record differentiable NCHW convolution.
    /// The returned parameter handles belong to this tape and forward pass.
    pub fn record(
        &self,
        tape: &mut Tape<T>,
        input: &Variable,
    ) -> Result<RecordedLayer, AutogradError> {
        let weight = self.weight.track(tape)?;
        let bias = self.bias.track(tape)?;
        let features = tape.conv2d(input, &weight, self.options)?;
        let output = tape.add(&features, &bias)?;
        Ok(RecordedLayer {
            output,
            weight,
            bias,
        })
    }
}
impl<T: TrainFloat> Module<T> for Conv2d<T> {
    fn forward(&self, input: &Tensor<T>) -> Result<Tensor<T>, OpsError> {
        Conv2d::forward(self, input)
    }
}
