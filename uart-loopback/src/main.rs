/*
Simplified Embedded Rust: ESP Core Library Edition
Programming Serial Communication - Console Printing Application Example
*/

#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    main,
    uart::{ClockSource, Config, DataBits, Parity, StopBits, Uart},
};
use esp_println::println;

esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    // Configure Peripherals and System Clocks
    let peripherals = esp_hal::init(esp_hal::Config::default());

    // Create a Delay abstraction
    let delay = Delay::new();

    // Create a UART Configuration
    let uart_config = Config::default()
        .with_baudrate(115200)
        .with_data_bits(DataBits::_8)
        .with_parity(Parity::None)
        .with_stop_bits(StopBits::_1)
        .with_clock_source(ClockSource::Apb);

    // Instantiate a UART Driver
    let mut loopback = Uart::new(peripherals.UART1, uart_config)
        .unwrap()
        .with_tx(peripherals.GPIO5)
        .with_rx(peripherals.GPIO6);

    // This line is for Wokwi only so that the console
    // output is formatted correctly
    esp_println::print!("\x1b[20h");

    let mut w_letter = 0x61;
    let mut r_letter = [0u8; 1];

    loop {
        // Send a letter over UART
        loopback.write(&[w_letter]).unwrap();
        // Read a letter from UART Buffer
        loopback.read(&mut r_letter).unwrap();
        println!("{}", r_letter[0] as char);
        // Check if the letter is 'z' and reset to 'a' if
        // true
        if w_letter == 0x7A {
            w_letter = 0x61;
        } else {
            w_letter += 1;
        }
        delay.delay_millis(1000u32);
    }
}
