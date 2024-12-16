mod address;
mod io;

use crate::cpu::ExtendedAddress;
pub use address::ram::RAM;
pub use address::rom::ROM;

pub use io::out::Out;

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

pub trait IOMappedDevice {
    fn address(&self) -> u8;
    fn read(&self) -> DeviceResult;
    fn write(&mut self, data: u8) -> DeviceResult;
    fn name(&self) -> &str;
}
