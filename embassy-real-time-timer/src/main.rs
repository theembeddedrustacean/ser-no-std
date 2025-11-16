/*
Simplified Embedded Rust: ESP Core Library Edition
The Embassy Framework - Real-time Timer Application Example
*/

#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_backtrace as _;
use esp_hal::{
    interrupt::software::SoftwareInterruptControl,
    timer::timg::TimerGroup,
};
use esp_println::println;

esp_bootloader_esp_idf::esp_app_desc!();

struct Time {
    seconds: u32,
    minutes: u32,
    hours: u32,
}

#[esp_rtos::main]
async fn main(_spawner: Spawner) {
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

    // This line is for Wokwi only so that the console output is formatted correctly
    esp_println::print!("\x1b[20h");

    // Set up a Time struct to keep track of time
    let mut time = Time {
        seconds: 0_u32,
        minutes: 0_u32,
        hours: 0_u32,
    };

    loop {
        // Wait for 1 second
        Timer::after(Duration::from_millis(1_000)).await;
        // Update and Print Timer Struct
        time.seconds = time.seconds.wrapping_add(1);
        if time.seconds > 59 {
            time.minutes += 1;
        }
        if time.minutes > 59 {
            time.hours += 1;
        }
        if time.hours > 23 {
            time.seconds = 0;
            time.minutes = 0;
            time.hours = 0;
        }
        println!(
            "Elapsed Time {:0>2}:{:0>2}:{:0>2}",
            time.hours, time.minutes, time.seconds
        );
    }
}
