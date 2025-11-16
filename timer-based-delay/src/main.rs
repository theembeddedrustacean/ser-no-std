/*
Simplified Embedded Rust: ESP Core Library Edition
Programming Timers & Counters - Timer-Based Delay Application Example
*/

#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    gpio::{Level, Output, OutputConfig},
    main,
    timer::timg::TimerGroup,
    timer::Timer,
};

esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    // Take the peripherals
    let peripherals =
        esp_hal::init(esp_hal::Config::default());

    // Instantiate & Configure Output Pin for LED Control
    let led_pin_config = OutputConfig::default();
    let mut led_pin = Output::new(
        peripherals.GPIO0,
        Level::Low,
        led_pin_config,
    );

    // Instantiate Timer Group 0
    let timer_group0 = TimerGroup::new(peripherals.TIMG0);

    // Instantiate Timer0 in Timer Group 0
    let timer0 = timer_group0.timer0;

    // Capture Start Time
    let mut start = timer0.now();

    // Activate Counter to Start Counting
    timer0.start();

    loop {
        // Check if Timer Reached or Exceeded 1 second
        if start.elapsed().as_secs() >= 1 {
            // Toggle LED
            led_pin.toggle();
            // Reset Counter
            start = timer0.now();
        }
    }
}
