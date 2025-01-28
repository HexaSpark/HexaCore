use super::{DeviceResult, IOMappedDevice};

enum OutMode {
    WaitingForMode,
    Int,
    Char,
}

use OutMode::*;

pub struct Out {
    address: u8,
    mode: OutMode,
}

impl Out {
    pub fn new(address: u8) -> Self {
        Self {
            address,
            mode: OutMode::WaitingForMode,
        }
    }
}

impl IOMappedDevice for Out {
    fn io_address(&self) -> u8 {
        self.address
    }

    fn io_read(&mut self) -> DeviceResult {
        DeviceResult::WriteOnly
    }

    fn io_write(&mut self, data: u8) -> DeviceResult {
        match self.mode {
            WaitingForMode => {
                if data == 0 {
                    self.mode = Int;
                } else {
                    self.mode = Char;
                }

                DeviceResult::Ok
            }
            Int => {
                println!("0x{:02x}", data);
                self.mode = WaitingForMode;
                DeviceResult::Ok
            }
            Char => {
                print!("{}", data as char);
                self.mode = WaitingForMode;
                DeviceResult::Ok
            }
        }
    }

    fn io_name(&self) -> &str {
        "Out"
    }
}
