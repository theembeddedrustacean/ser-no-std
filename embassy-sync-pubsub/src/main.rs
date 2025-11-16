#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::pubsub::PubSubChannel;
use embassy_time::{Duration, Timer};
use esp_backtrace as _;
use esp_hal::{
    interrupt::software::SoftwareInterruptControl,
    timer::timg::TimerGroup,
};
use esp_println::println;

esp_bootloader_esp_idf::esp_app_desc!();

//Declare a pubsub channel with a capcity of 2 and 1 subscriber and 2 publishers
static SHARED: PubSubChannel<
    CriticalSectionRawMutex,
    u32,
    2,
    2,
    2,
> = PubSubChannel::new();

#[embassy_executor::task]
async fn async_task_one() {
    let pub1 = SHARED.publisher().unwrap();
    loop {
        pub1.publish_immediate(1);
        Timer::after(Duration::from_millis(500)).await;
    }
}

#[embassy_executor::task]
async fn async_task_two() {
    let pub2 = SHARED.publisher().unwrap();
    loop {
        pub2.publish_immediate(2);
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
    spawner.spawn(async_task_one()).unwrap();
    spawner.spawn(async_task_two()).unwrap();

    let mut sub = SHARED.subscriber().unwrap();

    loop {
        let val = sub.next_message_pure().await;
        // Print Message
        println!("{}", val);
    }
}
