/*
Simplified Embedded Rust: ESP Core Library Edition
Programming GPIO - Button Controlled Blinking Application Example
*/

#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    gpio::{Input, Io, Level, Output, Pull},
    prelude::*,
};

#[entry]
fn main() -> ! {
    // Take Peripherals
    let peripherals =
        esp_hal::init(esp_hal::Config::default());

    // Instantiate and Create Handle for IO
    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);

    // Instantiate and Create Handle for LED output & Button
    // Input
    let mut led = Output::new(io.pins.gpio4, Level::High);
    let button = Input::new(io.pins.gpio0, Pull::Up);

    // Create and initialize a delay variable to manage
    // delay loop
    let mut blinkdelay = 1_000_000_u32;

    // Initialize LED to on or off
    led.set_low();

    // Application Loop
    loop {
        for _i in 1..blinkdelay {
            // Check if button got pressed
            if button.is_low() {
                // If button pressed decrease the delay
                // value
                blinkdelay = blinkdelay - 2_5000_u32;
                // If updated delay value reaches zero then
                // reset it back to starting value
                if blinkdelay < 2_5000 {
                    blinkdelay = 1_000_000_u32;
                }
            }
        }
        // Toggle LED
        led.toggle();
    }
}
