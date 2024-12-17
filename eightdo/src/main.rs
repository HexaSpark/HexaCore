use eightdo::{
    cpu::{EmuOptions, ExtendedAddress, Pins, ReadWrite, CPU},
    device::{DeviceResult, Out, RAM, ROM},
};

use std::env;

fn main() {let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    if args.len() != 1 {
        panic!("Missing ROM file argument.");
    }
    
    let rom = ROM::new_from_file(
        ExtendedAddress::new_16bit_address(0x0000),
        ExtendedAddress::new_16bit_address(0x7FFF),
        (&args[0]).into(),
    );
    let ram = RAM::new(
        ExtendedAddress::new_16bit_address(0x8000),
        ExtendedAddress::new_18bit_address(0x3FFFF),
    );

    let out = Out::new(0xA0);

    let mut pins = Pins::default();
    let mut cpu = CPU::new(Some(EmuOptions::new_value(1)));
    cpu.reset(&mut pins);

    cpu.add_device(ram);
    cpu.add_device(rom);
    cpu.add_io_device(out);

    loop {
        cpu.cycle(&mut pins);

        if pins.bus_enable {
            if pins.rw == ReadWrite::Read {
                let res = cpu.read(pins.address);
                if let DeviceResult::Ok(val) = res {
                    pins.data = val
                } else {
                    panic!("Device Error: {:?}", res);
                }
            } else {
                let res = cpu.write(pins.address, pins.data);

                if let DeviceResult::Ok(_) = res {
                } else {
                    panic!("Device Error: {:?}", res);
                }
            }
        }

        if pins.io_enable {
            if pins.io_rw == ReadWrite::Read {
                let res = cpu.read_io(pins.io_address);

                if let DeviceResult::Ok(val) = res {
                    pins.io_data = val;
                } else {
                    panic!("IO Device Error: {:?}", res);
                }
            } else {
                let res = cpu.write_io(pins.io_address, pins.io_data);

                if let DeviceResult::Ok(_) = res {} else {
                    panic!("IO Device Error: {:?}", res);
                }
            }
        }
    }
}
