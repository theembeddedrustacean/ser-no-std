/*
Simplified Embedded Rust: ESP Core Library Edition
Programming Serial Communication - Console Printing Application Example
*/

#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl,
    delay::Delay,
    gpio::Io,
    peripherals::Peripherals,
    prelude::*,
    system::SystemControl,
    uart::{config::*, ClockSource, Uart},
};
use esp_println::println;

#[entry]
fn main() -> ! {
    // Configure Peripherals and System Clocks
    let peripherals = Peripherals::take();
    let system = SystemControl::new(peripherals.SYSTEM);
    let clocks =
        ClockControl::boot_defaults(system.clock_control)
            .freeze();

    // Create a Delay abstraction
    let delay = Delay::new(&clocks);

    // Create a UART Configuration
    let uart_config = Config {
        baudrate: 115200,
        data_bits: DataBits::DataBits8,
        parity: Parity::ParityNone,
        stop_bits: StopBits::STOP1,
        clock_source: ClockSource::Apb,
        ..Default::default()
    };

    // Initialize GPIO
    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);

    // Instantiate a UART Driver
    let mut log = Uart::new_with_config(
        peripherals.UART0,
        uart_config,
        &clocks,
        io.pins.gpio43,
        io.pins.gpio44,
    )
    .unwrap();

    // This line is for Wokwi only so that the console output is formatted correctly
    esp_println::print!("\x1b[20h");

    loop {
        println!("esp_println output");
        log.write_bytes("write method output\n".as_bytes())
            .unwrap();
        delay.delay_millis(1000u32);
    }
}
