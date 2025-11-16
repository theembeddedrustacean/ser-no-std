/*
Simplified Embedded Rust: ESP Core Library Edition
Programming Serial Communication - I2C Scanner Application Example
*/

#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    i2c::master::{Config, I2c},
    main,
    time::Rate,
};
use esp_println::println;

esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    // Take Peripherals and Setup System Clocks
    let peripherals = esp_hal::init(esp_hal::Config::default());

    // Initialize and configure I2C0
    let mut i2c0 = I2c::new(
        peripherals.I2C0,
        Config::default().with_frequency(Rate::from_khz(100)),
    )
    .unwrap()
    .with_scl(peripherals.GPIO2)
    .with_sda(peripherals.GPIO3);

    // Start Scan at Address 1 going up to 127
    for addr in 1..=127 {
        println!("Scanning Address {}", addr as u8);

        // Scan Address
        let res = i2c0.read(addr as u8, &mut [0]);

        // Check and Print Result
        match res {
            Ok(_) => println!("Device Found at Address {}", addr as u8),
            Err(_) => println!("No Device Found"),
        }
    }

    // Loop Forever
    loop {}
}
