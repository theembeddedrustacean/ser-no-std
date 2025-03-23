// Simplified Embedded Rust
// Core Library Edition
// CH5-Q5 Wiring Template

// In the Wokwi components list there exists a PIR motion
// sensor component that simulates motion. The PIR sensor
// has three pins; a power, ground, and digital output
// (OUT). Triggering the sensor will drive the OUT pin high
// for 5 seconds, and then go low again. The sensor will
// ignore any further input for the next 1.2 seconds, and
// then start sensing for motion again. Create a project
// that upon detecting movement turns on a red LED.

#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{delay::Delay, main};
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
        delay.delay_millis(500);
    }
}
