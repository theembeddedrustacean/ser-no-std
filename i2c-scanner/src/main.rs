/*
Simplified Embedded Rust: ESP Core Library Edition
Programming Serial Communication - I2C Scanner Application Example
*/

#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{gpio::Io, i2c::I2c, prelude::*};
use esp_println::println;

#[entry]
fn main() -> ! {
    // Take Peripherals and Setup System Clocks
    let peripherals =
        esp_hal::init(esp_hal::Config::default());

    // Create IO Driver
    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);

    // Initialize and configure I2C0
    let mut i2c0 = I2c::new(
        peripherals.I2C0,
        io.pins.gpio3,
        io.pins.gpio2,
        100u32.kHz(),
    );

    // Start Scan at Address 1 going up to 127
    for addr in 1..=127 {
        println!("Scanning Address {}", addr as u8);

        // Scan Address
        let res = i2c0.read(addr as u8, &mut [0]);

        // Check and Print Result
        match res {
            Ok(_) => println!(
                "Device Found at Address {}",
                addr as u8
            ),
            Err(_) => println!("No Device Found"),
        }
    }

    // Loop Forever
    loop {}
}
