#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::signal::Signal;
use embassy_time::{Duration, Timer};
use esp_backtrace as _;
use esp_hal::{
    interrupt::software::SoftwareInterruptControl,
    timer::timg::TimerGroup,
};
use esp_println::println;

esp_bootloader_esp_idf::esp_app_desc!();

static SHARED: Signal<CriticalSectionRawMutex, u32> =
    Signal::new();

#[embassy_executor::task]
async fn async_task() {
    loop {
        SHARED.signal(5);
        Timer::after(Duration::from_millis(1000)).await;
    }
}

#[esp_rtos::main]
async fn main(spawner: Spawner) {
    // Initialize and create handle for devicer peripherals
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
    // Spawn async blinking task
    spawner.spawn(async_task()).unwrap();

    loop {
        let val = SHARED.wait().await;
        // Print Message
        println!("{}", val);
    }
}
