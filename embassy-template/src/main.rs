#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_backtrace as _;
use esp_hal::{
    interrupt::software::SoftwareInterruptControl,
    timer::timg::TimerGroup,
};

esp_bootloader_esp_idf::esp_app_desc!();

#[embassy_executor::task]
async fn embassy_task() {
    // Task Initializations
    loop {
        // Task Loop Code
        esp_println::println!("Print from an embassy task");
        Timer::after(Duration::from_millis(1_000)).await;
    }
}

#[esp_rtos::main]
async fn main(spawner: Spawner) {
    esp_println::println!("Init!");
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

    spawner.spawn(embassy_task()).unwrap();

    loop {
        // Main loop code
        esp_println::println!("Print from the main task");
        Timer::after(Duration::from_millis(5_000)).await;
    }
}
