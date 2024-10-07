#![no_std]
#![no_main]

use core::fmt::Write;
use embassy_executor::Spawner;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Channel;
use embassy_time::{Duration, Timer};
use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl, gpio::Io,
    peripherals::Peripherals, prelude::*,
    system::SystemControl, uart::Uart,
};
use heapless::String;

//Declare a channel of 2 u32s
static SHARED: Channel<CriticalSectionRawMutex, u32, 2> =
    Channel::new();

#[embassy_executor::task]
async fn async_task_one() {
    loop {
        SHARED.send(1).await;
        Timer::after(Duration::from_millis(500)).await;
    }
}

#[embassy_executor::task]
async fn async_task_two() {
    loop {
        SHARED.send(2).await;
        Timer::after(Duration::from_millis(1000)).await;
    }
}

#[main]
async fn main(spawner: Spawner) {
    // Initialize and create handle for devicer peripherals
    let peripherals = Peripherals::take();
    let system = SystemControl::new(peripherals.SYSTEM);
    let clocks =
        ClockControl::max(system.clock_control).freeze();
    // Configure UART
    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);
    let mut uart = Uart::new_async(
        peripherals.UART0,
        &clocks,
        io.pins.gpio21,
        io.pins.gpio20,
    )
    .unwrap();
    // Create empty String for message
    let mut msg: String<16> = String::new();
    // Spawn async blinking task
    spawner.spawn(async_task_one()).unwrap();
    spawner.spawn(async_task_two()).unwrap();

    loop {
        let val = SHARED.receive().await;
        core::writeln!(&mut msg, "{:02}", val).unwrap();
        // Transmit Message
        uart.write_bytes(msg.as_bytes()).unwrap();
        msg.clear();
    }
}
