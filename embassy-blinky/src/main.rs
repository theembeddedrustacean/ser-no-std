/*
Simplified Embedded Rust: ESP Core Library Edition
The Embassy Framework - Blinky Application Example
*/

#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl,
    gpio::{Io, Level, Output},
    peripherals::Peripherals,
    system::SystemControl,
    timer::timg::TimerGroup,
};

#[esp_hal_embassy::main]
async fn main(_spawner: Spawner) {
    // Take peripherals & Configure System Clocks
    let peripherals = Peripherals::take();
    let system = SystemControl::new(peripherals.SYSTEM);
    let clocks =
        ClockControl::boot_defaults(system.clock_control)
            .freeze();

    // Initalize embassy executor
    let timg0 = TimerGroup::new(peripherals.TIMG0, &clocks);
    esp_hal_embassy::init(&clocks, timg0.timer0);

    // Setup and Configure LED Output Pin
    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);
    let mut led = Output::new(io.pins.gpio1, Level::High);

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
