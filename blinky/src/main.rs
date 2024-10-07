/*
Simplified Embedded Rust: ESP Core Library Edition
Programming GPIO - Blinky Application Example
*/

#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl,
    delay::Delay,
    gpio::{Io, Level, Output},
    peripherals::Peripherals,
    prelude::*,
    system::SystemControl,
};

#[entry]
fn main() -> ! {
    // Take the peripherals
    let peripherals = Peripherals::take();

    // Set up system clocks
    let system = SystemControl::new(peripherals.SYSTEM);
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    // Create a delay handle
    let delay = Delay::new(&clocks);

    // Instantiate and Create Handle for IO
    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);

    // Create output pin
    let mut led_pin = Output::new(io.pins.gpio1, Level::Low);

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
