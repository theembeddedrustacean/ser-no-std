// Create a program to generate audio tones using PWM for a simple buzzer. A buzzer has two terminals,
// one terminal hooks up to the PWM output and the other to ground. Implement functions to
// play different musical notes by generating corresponding PWM signals with appropriate frequencies.

// Wiring
// The buzzer second terminal (PWM input) is connected to gpio 0.
// The buzzer first terminal is connected to ground.

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
