#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::mutex::Mutex;
use embassy_time::{Duration, Timer};
use esp_backtrace as _;
use esp_hal::{prelude::*, timer::timg::TimerGroup};
use esp_println::println;

static SHARED: Mutex<CriticalSectionRawMutex, u32> =
    Mutex::new(0);

#[embassy_executor::task]
async fn async_task() {
    loop {
        {
            let mut shared = SHARED.lock().await;
            *shared = shared.wrapping_add(1);
            Timer::after(Duration::from_millis(1000)).await;
        }
        Timer::after(Duration::from_millis(1000)).await;
    }
}

#[main]
async fn main(spawner: Spawner) {
    // Initialize and create handle for devicer peripherals
    let peripherals =
        esp_hal::init(esp_hal::Config::default());

    // Initalize embassy executor
    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_hal_embassy::init(timg0.timer0);

    // Spawn async blinking task
    spawner.spawn(async_task()).unwrap();

    loop {
        // Wait 1 second
        Timer::after(Duration::from_millis(1000)).await;
        // Obtain updated value from global context
        let shared = SHARED.lock().await;
        // Print Message
        println!("{}", shared);
    }
}
