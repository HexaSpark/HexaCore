use super::{AddressMappedDevice, DeviceResult};
use crate::cpu::ExtendedAddress;

pub struct Out {
    address: ExtendedAddress,
}

impl Out {
    pub fn new(address: ExtendedAddress) -> Self {
        Self { address }
    }
}

impl AddressMappedDevice for Out {
    fn read(&self, address: ExtendedAddress) -> DeviceResult {
        let address_from: u32 = u32::from(address);

        if address_from == u32::from(self.address) {
            return DeviceResult::WriteOnly;
        }

        DeviceResult::NotMyAddress
    }

    fn write(&mut self, address: ExtendedAddress, data: u8) -> DeviceResult {
        let address_from: u32 = u32::from(address);

        if address_from == u32::from(self.address) {
            println!("0x{:02x}", data);

            return DeviceResult::Ok(0);
        }

        DeviceResult::NotMyAddress
    }

    fn relative(&self, _address: ExtendedAddress) -> usize {
        0
    }

    fn size(&self) -> usize {
        1
    }

    fn start(&self) -> ExtendedAddress {
        self.address
    }

    fn end(&self) -> ExtendedAddress {
        self.address
    }
}
