/*
Simplified Embedded Rust: ESP Core Library Edition
Programming Timers & Counters - Timer-Based Delay Application Example
*/

#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl,
    gpio::{Io, Level, Output},
    peripherals::Peripherals,
    prelude::*,
    system::SystemControl,
    timer::timg::TimerGroup,
};

#[entry]
fn main() -> ! {
    // Take the peripherals
    let peripherals = Peripherals::take();

    // Set up system clocks
    let system = SystemControl::new(peripherals.SYSTEM);
    let clocks =
        ClockControl::boot_defaults(system.clock_control)
            .freeze();

    // Instantiate and Create Handle for IO
    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);

    // Instantiate Output Pin for LED Control
    let mut led_pin =
        Output::new(io.pins.gpio0, Level::Low);

    // Instantiate Timer Group 0
    let timer_group0 =
        TimerGroup::new(peripherals.TIMG0, &clocks);

    // Instantiate Timer0 in Timer Group 0
    let timer0 = timer_group0.timer0;

    // Capture Start Time
    let mut start = timer0.now();

    // Activate Counter to Start Counting
    timer0.start();

    loop {
        // Check if Timer Reached or Exceeded 1 second
        if timer0
            .now()
            .checked_duration_since(start)
            .unwrap()
            .to_secs()
            >= 1
        {
            // Toggle LED
            led_pin.toggle();
            // Reset Counter
            start = timer0.now();
        }
    }
}
