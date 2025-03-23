// Refactor the LED fading code to control the position of a servo motor connected to a pin using PWM.
// Implement functions to move the servo motor smoothly from its minimum position to its maximum
// position and back again over a specified duration. Assume access to PWM functions in your library.
// You can rotate the servo motor shaft from 0𝑜 to 180𝑜 by varying the PWM pulse width from 1 ms to
// 2 ms for a 50 Hz (20 ms period) signal. This corresponds to a 5-10% duty cycle on a 50 Hz waveform.

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
