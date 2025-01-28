use HexaCore::{
    cpu::{EmuOptions, ExtendedAddress, Pins, ReadWrite, CPU},
    device::{DeviceResult, Out, RAM, ROM},
    info::*
};

use std::env;

fn main() {
    // let mut args: Vec<String> = env::args().collect();
    // args.remove(0);

    // if args.len() != 1 {
    //     panic!("Missing ROM file argument.");
    // }

    let inst_info = get_instructions();

    let rom = ROM::new_from_file(
        ExtendedAddress::new_16bit_address(0x00_0000),
        ExtendedAddress::new_16bit_address(0x00_FFFF),
        ("gen/test.bin").into(), // TODO: Change back to &args[0] to return back to normal
    );
    let ram = RAM::new(
        ExtendedAddress::new_ext_address(0x01_0000),
        ExtendedAddress::new_ext_address(0xFF_FFFF),
    );

    let out = Out::new(0xA0);

    let mut pins = Pins::default();
    let mut cpu = CPU::new(inst_info, Some(EmuOptions::new_value(1)));
    cpu.reset(&mut pins); 

    cpu.add_device(ram);
    cpu.add_device(rom);
    cpu.add_io_device(out);

    loop {
        cpu.cycle(&mut pins);

        if pins.bus_enable {
            if pins.rw == ReadWrite::Read {
                let res = cpu.read(pins.address);

                match res {
                    DeviceResult::Ok8(val) => pins.data = val as u16,
                    DeviceResult::Ok16(val) => pins.data = val,
                    _ => panic!("Device Error: {:?}", res),
                }
            } else {
                let res = cpu.write(pins.address, pins.data);

                if let DeviceResult::Ok = res {
                } else {
                    panic!("Device Error: {:?} for address 0x{:06x}", res, u32::from(pins.address));
                }
            }
        }

        if pins.io_enable {
            if pins.io_rw == ReadWrite::Read {
                let res = cpu.read_io(pins.io_address);

                if let DeviceResult::Ok8(val) = res {
                    pins.io_data = val;
                } else {
                    panic!("IO Device Error: {:?}", res);
                }
            } else {
                let res = cpu.write_io(pins.io_address, pins.io_data);

                if let DeviceResult::Ok = res {
                } else {
                    panic!("IO Device Error: {:?}", res);
                }
            }
        }
    }
}
