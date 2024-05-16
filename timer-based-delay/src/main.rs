/*
Simplified Embedded Rust: ESP Core Library Edition
Programming Timers & Counters - Timer-Based Delay Application Example
*/

#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl, gpio::IO, peripherals::Peripherals, prelude::*, timer::TimerGroup,
};

#[entry]
fn main() -> ! {
    // Take the peripherals
    let peripherals = Peripherals::take();

    // Set up system clocks
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    // Instantiate and Create Handle for IO
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    // Instantiate Output Pin for LED Control
    let mut led_pin = io.pins.gpio0.into_push_pull_output();

    // Instantiate Timer Group 0
    let timer_group0 = TimerGroup::new(peripherals.TIMG0, &clocks, None);

    // Instantiate Timer0 in Timer Group 0
    let mut timer0 = timer_group0.timer0;

    // Reset Counter to Zero
    timer0.reset_counter();

    // Activate Counter to Start Counting
    timer0.set_counter_active(true);

    loop {
        // Check if Timer Reached or Exceeded 1 second
        // TIMG Clock is 80 MHz Divided by 2 resulting in 40 Million Counts per Second
        // Reaching a Count of 40 Millon Means 1 Second has Elapsed
        if timer0.now() >= 40000000 {
            // Toggle LED
            led_pin.toggle();
            // Reset Counter
            timer0.reset_counter();
        }
    }
}
