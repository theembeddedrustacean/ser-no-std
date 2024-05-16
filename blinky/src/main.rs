/*
Simplified Embedded Rust: ESP Core Library Edition
Programming GPIO - Blinky Application Example
*/

#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{clock::ClockControl, delay::Delay, gpio::IO, peripherals::Peripherals, prelude::*};

#[entry]
fn main() -> ! {
    // Take the peripherals
    let peripherals = Peripherals::take();

    // Set up system clocks
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    // Create a delay handle
    let delay = Delay::new(&clocks);

    // Instantiate and Create Handle for IO
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    // Create output pin
    let mut led_pin = io.pins.gpio1.into_push_pull_output();

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
