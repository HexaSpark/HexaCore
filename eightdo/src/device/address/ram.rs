use super::{AddressMappedDevice, DeviceResult};
use crate::cpu::ExtendedAddress;

pub struct RAM {
    start: ExtendedAddress,
    end: ExtendedAddress,
    data: Vec<u8>,
}

impl RAM {
    pub fn new(start: ExtendedAddress, end: ExtendedAddress) -> Self {
        Self {
            start,
            end,
            data: vec![0; (u32::from(end) - u32::from(start)) as usize],
        }
    }
}

impl AddressMappedDevice for RAM {
    fn read(&mut self, address: ExtendedAddress) -> DeviceResult {
        let address_from: u32 = u32::from(address);

        if address_from >= u32::from(self.start) && address_from <= u32::from(self.end) {
            return DeviceResult::Ok(self.data[self.relative(address)]);
        }

        DeviceResult::NotMyAddress
    }

    fn write(&mut self, address: ExtendedAddress, data: u8) -> DeviceResult {
        let address_from: u32 = u32::from(address);

        if address_from >= u32::from(self.start) && address_from <= u32::from(self.end) {
            let index = self.relative(address);
            self.data[index] = data;

            return DeviceResult::Ok(0);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ram_16_bits() {
        let ram = RAM::new(
            ExtendedAddress::new_16bit_address(0x0000),
            ExtendedAddress::new_16bit_address(0xFFFF),
        );

        assert_eq!(ram.size(), 0xFFFF);
        assert_eq!(ram.start(), 0x0000);
        assert_eq!(ram.end(), 0xFFFF);
    }

    #[test]
    fn test_ram_read() {
        let mut ram = RAM::new(
            ExtendedAddress::new_16bit_address(0x0000),
            ExtendedAddress::new_16bit_address(0xFFFF),
        );
        ram.data[0xD007] = 0xCA;

        let result = ram.read(ExtendedAddress::new_16bit_address(0xD007));

        assert_eq!(result, DeviceResult::Ok(0xCA));
    }

    #[test]
    fn test_ram_write() {
        let mut ram = RAM::new(
            ExtendedAddress::new_16bit_address(0x0000),
            ExtendedAddress::new_16bit_address(0xFFFF),
        );

        let result = ram.write(ExtendedAddress::new_16bit_address(0xD007), 0xCA);

        assert_eq!(ram.data[0xD007], 0xCA);
        assert_eq!(result, DeviceResult::Ok(0));
    }

    #[test]
    fn test_ram_out_of_range_read() {
        let ram = RAM::new(
            ExtendedAddress::new_16bit_address(0x0000),
            ExtendedAddress::new_16bit_address(0x7FFF),
        );

        let result = ram.read(ExtendedAddress::new_16bit_address(0xD007));

        assert_eq!(result, DeviceResult::NotMyAddress);
    }

    #[test]
    fn test_ram_out_of_range_write() {
        let mut ram = RAM::new(
            ExtendedAddress::new_16bit_address(0x0000),
            ExtendedAddress::new_16bit_address(0x7FFF),
        );

        let result = ram.write(ExtendedAddress::new_16bit_address(0xD007), 0xCA);

        assert_eq!(result, DeviceResult::NotMyAddress);
    }
}
