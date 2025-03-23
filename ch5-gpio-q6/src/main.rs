// Simplified Embedded Rust
// Core Library Edition
// CH5-Q6 Wiring Template

// Develop a program to generate a custom LED pattern on
// three GPIO pins. The pattern should repeat indefinitely,
// cycling through turning on and off each LED in sequence
// (e.g., LED1 on, LED2 on, LED3 on, LED1 off, LED2 off,
// LED3 off, repeat).

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
