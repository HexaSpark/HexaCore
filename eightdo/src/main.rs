use eightdo::{
    cpu::{ExtendedAddress, Pins, ReadWrite, CPU},
    device::{DeviceResult, Out, RAM, ROM},
};

fn main() {
    let rom = ROM::new_from_file(
        ExtendedAddress::new_16bit_address(0x0000),
        ExtendedAddress::new_16bit_address(0x7FFF),
        "../gen/rom.bin".into(),
    );
    let ram = RAM::new(
        ExtendedAddress::new_16bit_address(0x8000),
        ExtendedAddress::new_18bit_address(0x3FFFE),
    );

    let out = Out::new(ExtendedAddress::new_18bit_address(0x3FFFF));

    let mut pins = Pins::default();
    let mut cpu = CPU::new(None);
    cpu.reset(&mut pins);

    cpu.add_device(ram);
    cpu.add_device(rom);
    cpu.add_device(out);

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
    }
}
