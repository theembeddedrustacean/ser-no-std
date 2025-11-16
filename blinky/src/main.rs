/*
Simplified Embedded Rust: ESP Core Library Edition
Programming GPIO - Blinky Application Example
*/

#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    gpio::{DriveMode, DriveStrength, Level, Output, OutputConfig, Pull},
    main,
};

esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    // Take the peripherals
    let peripherals = esp_hal::init(esp_hal::Config::default());

    // Create a delay handle
    let delay = Delay::new();

    // Create output pin configuration
    let led_pin_conf = OutputConfig::default()
        .with_drive_mode(DriveMode::PushPull)
        .with_drive_strength(DriveStrength::_10mA)
        .with_pull(Pull::None);

    // Create output pin
    let mut led_pin = Output::new(peripherals.GPIO1, Level::Low, led_pin_conf);

    loop {
        // Turn on LED
        led_pin.set_high();
        // Wait for 1 second
        delay.delay_millis(1000u32);
        // Turn off LED
        led_pin.set_low();
        // Wait for 1 second
        delay.delay_millis(1000u32);
    }
}
