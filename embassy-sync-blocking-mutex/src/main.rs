#![no_std]
#![no_main]

use core::cell::RefCell;
use core::fmt::Write;
use embassy_executor::Spawner;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::blocking_mutex::Mutex;
use embassy_time::{Duration, Timer};
use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl, gpio::Io,
    peripherals::Peripherals, prelude::*,
    system::SystemControl, uart::Uart,
};
use heapless::String;

static SHARED: Mutex<
    CriticalSectionRawMutex,
    RefCell<u32>,
> = Mutex::new(RefCell::new(0));

#[embassy_executor::task]
async fn async_task() {
    loop {
        // Load value from global context, modify and store
        SHARED.lock(|f| {
            let val = f.borrow_mut().wrapping_add(1);
            f.replace(val);
        });
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
    //Configure UART
    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);
    let mut uart = Uart::new_async(
        peripherals.UART0,
        &clocks,
        io.pins.gpio21,
        io.pins.gpio20,
    )
    .unwrap();
    // Create empty String for message
    let mut msg: String<8> = String::new();
    // Spawn async blinking task
    spawner.spawn(async_task()).unwrap();

    loop {
        // Wait 1 second
        Timer::after(Duration::from_millis(1000)).await;
        // Obtain updated value from global context
        let shared =
            SHARED.lock(|f| f.clone().into_inner());
        core::writeln!(&mut msg, "{:02}", shared).unwrap();
        // Transmit Message
        uart.write_bytes(msg.as_bytes()).unwrap();
        msg.clear();
    }
}
