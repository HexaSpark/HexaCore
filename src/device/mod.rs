mod address;
mod io;

use crate::cpu::ExtendedAddress;
pub use address::ram::RAM;
pub use address::rom::ROM;

pub use io::out::Out;

#[derive(Debug, PartialEq, Eq)]
pub enum DeviceResult {
    Ok,
    Ok8(u8),
    Ok16(u16),
    NotMyAddress,
    ReadOnly,
    WriteOnly,
    InvalidAddress,
    FileNotFound,
    CannotReadFile,
    NoValidDevice,
}

pub trait AddressMappedDevice {
    fn read(&mut self, address: ExtendedAddress, word: bool) -> DeviceResult;
    fn write(&mut self, address: ExtendedAddress, data: u16, word: bool) -> DeviceResult;
    fn relative(&self, address: ExtendedAddress) -> usize;
    fn size(&self) -> usize;
    fn start(&self) -> ExtendedAddress;
    fn end(&self) -> ExtendedAddress;
}

pub trait IOMappedDevice {
    fn io_address(&self) -> u8;
    fn io_read(&mut self) -> DeviceResult;
    fn io_write(&mut self, data: u8) -> DeviceResult;
    fn io_name(&self) -> &str;
}
