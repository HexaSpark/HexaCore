use std::io::Read;

use super::{AddressMappedDevice, DeviceResult};
use crate::cpu::ExtendedAddress;

pub struct ROM {
    start: ExtendedAddress,
    end: ExtendedAddress,
    data: Vec<u8>,
}

impl ROM {
    pub fn new(start: ExtendedAddress, end: ExtendedAddress, mut data: Vec<u8>) -> Self {
        let size = u32::from(end) - u32::from(start);

        if data.len() != size as usize {
            data.resize(size as usize, 0);
        }

        Self { start, end, data }
    }

    pub fn new_from_file(start: ExtendedAddress, end: ExtendedAddress, file: String) -> Self {
        let size = u32::from(end) - u32::from(start);
        let mut data: Vec<u8> = vec![];
        let mut file_o = std::fs::File::open(file).unwrap();
        let bytes_read = file_o.read_to_end(&mut data).unwrap();

        if bytes_read != size as usize {
            data.resize(size as usize, 0);
        }

        Self { start, end, data }
    }
}

impl AddressMappedDevice for ROM {
    fn read(&mut self, address: ExtendedAddress) -> DeviceResult {
        let address_from: u32 = u32::from(address);

        if address_from >= u32::from(self.start) && address_from <= u32::from(self.end) {
            return DeviceResult::Ok(self.data[self.relative(address)]);
        }

        DeviceResult::NotMyAddress
    }

    fn write(&mut self, address: ExtendedAddress, _data: u8) -> DeviceResult {
        let address_from: u32 = u32::from(address);

        if address_from >= u32::from(self.start) && address_from <= u32::from(self.end) {
            return DeviceResult::ReadOnly;
        }

        DeviceResult::NotMyAddress
    }

    fn relative(&self, address: ExtendedAddress) -> usize {
        (u32::from(address) - u32::from(self.start)) as usize
    }

    fn size(&self) -> usize {
        (u32::from(self.end) - u32::from(self.start)) as usize
    }

    fn start(&self) -> ExtendedAddress {
        self.start
    }

    fn end(&self) -> ExtendedAddress {
        self.end
    }
}
