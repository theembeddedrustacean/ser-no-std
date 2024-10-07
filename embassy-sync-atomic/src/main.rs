#![no_std]
#![no_main]

use core::fmt::Write;
use core::sync::atomic::{AtomicU32, Ordering};
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl, gpio::Io,
    peripherals::Peripherals, prelude::*,
    system::SystemControl, uart::Uart,
};
use heapless::String;

static SHARED: AtomicU32 = AtomicU32::new(0);

#[embassy_executor::task]
async fn async_task() {
    loop {
        // Load value from global context, modify and store
        let shared_var = SHARED.load(Ordering::Relaxed);
        SHARED.store(
            shared_var.wrapping_add(1),
            Ordering::Relaxed,
        );
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
        // Load value from global context
        let shared = SHARED.load(Ordering::Relaxed);
        // Wait 1 second
        Timer::after(Duration::from_millis(1000)).await;
        // Format value for printing
        core::writeln!(&mut msg, "{:02}", shared).unwrap();
        // Transmit Message
        uart.write_bytes(msg.as_bytes()).unwrap();
        msg.clear();
    }
}
