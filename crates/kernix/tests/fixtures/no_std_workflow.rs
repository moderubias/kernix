//! Minimal actual no_std consumer, compiled independently with default features off.
#![no_std]
use kernix::{OpsError, prelude::*};

/// Construct, broadcast and multiply using an allocator supplied by the final host.
pub fn workflow() -> Result<Tensor<f32>, OpsError> {
    let a = matrix![1.0f32,2.0;3.0,4.0]?;
    let identity_values = Tensor::<f32>::ones([2, 2])?;
    let zeros = Tensor::<f32>::zeros([2])?;
    let shifted = add(&a, &zeros)?;
    matmul(&shifted, &identity_values)
}

#[cfg(test)]
mod tests {
    #[test]
    fn executes_with_the_host_test_allocator() {
        let out = super::workflow().unwrap();
        assert_eq!(out.storage_slice(), &[3.0, 3.0, 7.0, 7.0]);
    }
}

/// Batched recording, axis means and normalization run without std as well.
pub fn batched_training() -> Result<Tensor<f32>, kernix::autograd::AutogradError> {
    let mut tape = kernix::autograd::Tape::new();
    let x = tape.constant(Tensor::from_values(
        [2, 2, 2],
        (1..=8).map(|x| x as f32).collect(),
    )?)?;
    let w = tape.variable(matrix![1.0f32,0.0;0.0,1.0]?)?;
    let y = tape.matmul(&x, &w)?;
    let norm = tape.layer_norm(&y, 1, 1e-4)?;
    let normalized_loss = tape.sum(&norm)?;
    let mean = tape.mean_axes(&y, &[0], false)?;
    let mean_loss = tape.sum(&mean)?;
    let loss = tape.add(&normalized_loss, &mean_loss)?;
    let tail = tape.slice_axis(&y, 2, 1, 1, 1)?;
    let tail_loss = tape.sum(&tail)?;
    let loss = tape.add(&loss, &tail_loss)?;
    let gradients = tape.backward(&loss)?;
    Ok(gradients.get(&w)?.expect("trainable weight").clone())
}

#[cfg(test)]
mod batched_tests {
    #[test]
    fn executes_batched_recording_with_the_host_allocator() {
        let dx = super::batched_training().unwrap();
        assert_eq!(dx.storage_slice(), &[8.0, 24.0, 10.0, 30.0]);
    }
}
