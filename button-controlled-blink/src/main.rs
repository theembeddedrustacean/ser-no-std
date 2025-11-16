/*
Simplified Embedded Rust: ESP Core Library Edition
Programming GPIO - Button Controlled Blinking Application Example
*/

#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::gpio::{DriveMode, Input, InputConfig, Level, Output, OutputConfig, Pull};
use esp_hal::main;

esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    // Take Peripherals
    let peripherals = esp_hal::init(esp_hal::Config::default());

    // Instantiate and Create Handle for LED output & Button
    let led_config = OutputConfig::default().with_drive_mode(DriveMode::PushPull);
    let button_config = InputConfig::default().with_pull(Pull::Up);

    let mut led = Output::new(peripherals.GPIO4, Level::High, led_config);
    let button = Input::new(peripherals.GPIO0, button_config);

    // Create and initialize a delay variable to manage
    // value
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
