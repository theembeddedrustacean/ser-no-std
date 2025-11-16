/*
Simplified Embedded Rust: ESP Core Library Edition
Programming ADCs - Voltmeter Application Example
*/

#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    analog::adc::{Adc, AdcConfig, Attenuation},
    delay::Delay,
    main,
};
use esp_println::println;

esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    // Take Peripherals & Configure Clocks
    let peripherals =
        esp_hal::init(esp_hal::Config::default());

    // Create Delay Provider
    let delay = Delay::new();

    // Create handle for ADC configuration parameters
    let mut adc_config = AdcConfig::new();

    // Configure ADC pin
    let mut adc_pin = adc_config
        .enable_pin(peripherals.GPIO0, Attenuation::_11dB);

    // Create ADC Driver
    let mut adc = Adc::new(peripherals.ADC1, adc_config);

    loop {
        // Get ADC Reading
        let sample: u16 =
            adc.read_oneshot(&mut adc_pin).unwrap();

        // Convert to Voltage
        let voltage: u32 = sample as u32 * 3300 / 4095;

        // Print the temperature output
        println!(
            "Raw Reading: {}, Voltage Reading: {}mV",
            sample, voltage
        );

        // Wait half a second before next sample
        delay.delay_millis(500_u32);
    }
}
