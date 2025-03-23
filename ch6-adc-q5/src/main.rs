// Simplified Embedded Rust
// Core Library Edition
// CH6-Q5 Wiring Template

// Using the LED bar and the rotating potentiometer, create an application that replicates a volume
// control dial. Meaning as the potentiometer dial is rotated clockwise, an increasing amount of LEDs
// light up indicating higher volume. Conversely, as the dial is rotated counter-clockwise, LEDs are
// turned off.

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
