// Simplified Embedded Rust
// Core Library Edition
// CH6-Q4 Wiring Template

// Wokwi has an analog joystick component that provides a range of analog values depending on the
// direction. Create an application that prints out the position of the joystick.

// Hint:
// Heres a link to the datasheet to help understand how it works
// https://components101.com/modules/joystick-module
// You can also click on the question mark icon when selecting the joystick for the Wokwi documentation

#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    gpio::{Io, Level, Output},
    main,
};
use esp_println::println;

#[main]
fn main() -> ! {
    // Take the peripherals
    let peripherals =
        esp_hal::init(esp_hal::Config::default());

    // Create a delay handle
    let delay = Delay::new();

    println!("Hello world!");

    loop {
        println!("Loop...");
        delay.delay_millis(500u32);
    }
}
