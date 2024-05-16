/*
Simplified Embedded Rust: ESP Core Library Edition
The Embassy Framework - Blinky Application Example
*/

#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl,
    embassy::{self},
    gpio::IO,
    peripherals::Peripherals,
    prelude::*,
    timer::TimerGroup,
};

#[main]
async fn main(_spawner: Spawner) {
    // Take peripherals & Configure System Clocks
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    // Initalize embassy executor
    let timg0 = TimerGroup::new_async(peripherals.TIMG0, &clocks);
    embassy::init(&clocks, timg0);

    // Setup and Configure LED Output Pin
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let mut led = io.pins.gpio1.into_push_pull_output();

    loop {
        // Turn on LED
        led.set_high();
        // Wait for 1 second
        Timer::after(Duration::from_millis(1_000)).await;
        // Turn off LED
        led.set_low();
        // Wait for 1 second
        Timer::after(Duration::from_millis(1_000)).await;
    }
}
