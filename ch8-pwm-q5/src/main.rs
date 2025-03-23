// Refactor the LED fading code such that the level of fading can be controlled by an input potentiometer
// (same as the one used in the ADC questions).

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
