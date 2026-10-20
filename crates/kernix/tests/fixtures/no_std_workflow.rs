//! Minimal actual no_std consumer, compiled independently with default features off.
#![no_std]
use kernix::{prelude::*, OpsError};

/// Construct, broadcast and multiply using an allocator supplied by the final host.
pub fn workflow() -> Result<Tensor<f32>,OpsError> {
    let a=matrix![1.0f32,2.0;3.0,4.0]?;
    let identity_values=Tensor::<f32>::ones([2,2])?;
    let zeros=Tensor::<f32>::zeros([2])?;
    let shifted=add(&a,&zeros)?;
    matmul(&shifted,&identity_values)
}

#[cfg(test)]
mod tests {
    #[test]
    fn executes_with_the_host_test_allocator() {
        let out=super::workflow().unwrap();
        assert_eq!(out.storage_slice(),&[3.0,3.0,7.0,7.0]);
    }
}
