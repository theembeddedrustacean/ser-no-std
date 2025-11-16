#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{delay::Delay, main};
use esp_println::println;

esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    // Instantiate delay provider
    let delay = Delay::new();
    // Print "Hello world!" every 500ms
    loop {
        println!("Hello world!");
        delay.delay_millis(500u32);
    }
}
