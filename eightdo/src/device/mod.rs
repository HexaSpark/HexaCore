pub mod out;
pub mod ram;
pub mod rom;

use crate::cpu::ExtendedAddress;
pub use out::Out;
pub use ram::RAM;
pub use rom::ROM;

#[derive(Debug, PartialEq, Eq)]
pub enum DeviceResult {
    Ok(u8),
    NotMyAddress,
    ReadOnly,
    WriteOnly,
    InvalidAddress,
    FileNotFound,
    CannotReadFile,
    NoValidDevice,
}

pub trait AddressMappedDevice {
    fn read(&self, address: ExtendedAddress) -> DeviceResult;
    fn write(&mut self, address: ExtendedAddress, data: u8) -> DeviceResult;
    fn relative(&self, address: ExtendedAddress) -> usize;
    fn size(&self) -> usize;
    fn start(&self) -> ExtendedAddress;
    fn end(&self) -> ExtendedAddress;
}
