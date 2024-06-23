//! Prints "Hello, world!" on the host console using semihosting

#![no_main]
#![no_std]

use core::panic::PanicInfo;

use m_bus_parser::MbusData;

use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};

#[panic_handler]
#[no_mangle]
fn my_panic_handler(info: &PanicInfo) -> ! {
    hprintln!("Oh noes, panic {:?} :(", info);
    loop {}
}
#[entry]
fn main() -> ! {
    hprintln!("Let's parse some data!");
    let example = [
        0x68, 0x3C, 0x3C, 0x68, 0x08, 0x08, 0x72, 0x78, 0x03, 0x49, 0x11, 0x77, 0x04, 0x0E, 0x16,
        0x0A, 0x00, 0x00, 0x00, 0x0C, 0x78, 0x78, 0x03, 0x49, 0x11, 0x04, 0x13, 0x31, 0xD4, 0x00,
        0x00, 0x42, 0x6C, 0x00, 0x00, 0x44, 0x13, 0x00, 0x00, 0x00, 0x00, 0x04, 0x6D, 0x0B, 0x0B,
        0xCD, 0x13, 0x02, 0x27, 0x00, 0x00, 0x09, 0xFD, 0x0E, 0x02, 0x09, 0xFD, 0x0F, 0x06, 0x0F,
        0x00, 0x01, 0x75, 0x13, 0xD3, 0x16,
    ];
    if let Ok(mbus_data) = MbusData::try_from(example.as_slice()) {
        hprintln!("Parsed data!");
        hprintln!("{:?}", mbus_data);
    } else {
        hprintln!("Failed to parse data!");
    }

    hprintln!("Finished Parsing!");
    debug::exit(debug::EXIT_SUCCESS);

    loop {}
}
