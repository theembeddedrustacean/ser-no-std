// Expand the real-time timer example to operate as a stopwatch. Add three input buttons; start, stop,
// and reset.

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
