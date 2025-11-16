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
    gpio::{Level, Output, OutputConfig},
    interrupt::software::SoftwareInterruptControl,
    timer::timg::TimerGroup,
};

esp_bootloader_esp_idf::esp_app_desc!();

#[esp_rtos::main]
async fn main(_spawner: Spawner) {
    // Take peripherals & Configure System Clocks
    let peripherals =
        esp_hal::init(esp_hal::Config::default());

    // Initalize embassy executor
    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let sw_int = SoftwareInterruptControl::new(
        peripherals.SW_INTERRUPT,
    );
    esp_rtos::start(
        timg0.timer0,
        sw_int.software_interrupt0,
    );

    // Setup and Configure LED Output Pin
    let led_config = OutputConfig::default();
    let mut led = Output::new(
        peripherals.GPIO1,
        Level::High,
        led_config,
    );

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
