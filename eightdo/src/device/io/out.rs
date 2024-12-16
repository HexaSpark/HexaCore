use super::{IOMappedDevice, DeviceResult};

enum OutMode {
    WaitingForMode,
    Int,
    Char
}

use OutMode::*;

pub struct Out {
    address: u8,
    mode: OutMode
}

impl Out {
    pub fn new(address: u8) -> Self {
        Self { address, mode: OutMode::WaitingForMode }
    }
}

impl IOMappedDevice for Out {
    fn address(&self) -> u8 {
        self.address
    }

    fn read(&self) -> DeviceResult {
        DeviceResult::WriteOnly
    }

    fn write(&mut self, data: u8) -> DeviceResult {
        match self.mode {
            WaitingForMode => {
                if data == 0 {
                    self.mode = Int;
                } else {
                    self.mode = Char;
                }

                DeviceResult::Ok(0)
            },
            Int => {
                println!("0x{:02x}", data);
                self.mode = WaitingForMode;
                DeviceResult::Ok(0)
            },
            Char => {
                print!("{}", data as char);
                self.mode = WaitingForMode;
                DeviceResult::Ok(0)
            }
        }
    }

    fn name(&self) -> &str {
        "Out"
    }
}
