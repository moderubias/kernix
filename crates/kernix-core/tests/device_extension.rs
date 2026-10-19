//! Downstream-style opaque device implementation without HostBuffer.
use kernix_core::{
    Layout, Order,
    device::{Cpu, Device},
    shape::Kahen,
    storage::{Storage, StorageBuffer},
    tensor::Tensor,
};
#[derive(Debug)]
struct Error;
impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("buffer belongs to another context")
    }
}
impl core::error::Error for Error {}
struct Opaque {
    owner: u64,
    data: Vec<f32>,
}
impl StorageBuffer<f32> for Opaque {
    fn len(&self) -> usize {
        self.data.len()
    }
}
struct ExternalDevice(u64);
impl Device<f32> for ExternalDevice {
    type Buffer = Opaque;
    type Error = Error;
    fn name(&self) -> &str {
        "external"
    }
    fn upload(&self, data: Vec<f32>) -> Result<Opaque, Error> {
        Ok(Opaque {
            owner: self.0,
            data,
        })
    }
    fn download(&self, buffer: &Opaque) -> Result<Vec<f32>, Error> {
        if buffer.owner != self.0 {
            Err(Error)
        } else {
            Ok(buffer.data.clone())
        }
    }
    fn synchronize(&self) -> Result<(), Error> {
        Ok(())
    }
}
#[test]
fn opaque_storage_and_transfers_work_without_a_host_pointer() {
    let device = ExternalDevice(1);
    let buffer = device.upload(vec![1.0, 2.0]).unwrap();
    let tensor = Tensor::from_storage(
        Storage::from_buffer(buffer),
        Layout::contiguous(Kahen::new([2]).unwrap(), Order::C).unwrap(),
    )
    .unwrap();
    assert_eq!(tensor.numel(), 2);
    assert_eq!(
        device.download(tensor.storage().buffer()).unwrap(),
        vec![1.0, 2.0]
    );
    assert!(
        ExternalDevice(2)
            .download(tensor.storage().buffer())
            .is_err()
    );
    device.synchronize().unwrap();
    let cpu = Cpu;
    let b = cpu.upload(vec![1u8, 2]).unwrap();
    assert_eq!(cpu.download(&b).unwrap(), vec![1, 2]);
}
