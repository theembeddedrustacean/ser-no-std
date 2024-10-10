/*
Simplified Embedded Rust: ESP Core Library Edition
Programming ADCs - Voltmeter Application Example
*/

#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    analog::adc::{Adc, AdcConfig, Attenuation},
    clock::ClockControl,
    delay::Delay,
    gpio::Io,
    peripherals::Peripherals,
    prelude::*,
    system::SystemControl,
};
use esp_println::println;

#[entry]
fn main() -> ! {
    // Take Peripherals & Configure Clocks
    let peripherals = Peripherals::take();
    let system = SystemControl::new(peripherals.SYSTEM);

    let clocks =
        ClockControl::max(system.clock_control).freeze();
    let delay = Delay::new(&clocks);

    // Instantiate and Create Handle for IO
    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);

    // Create handle for ADC configuration parameters
    let mut adc_config = AdcConfig::new();

    // Configure ADC channel
    let mut adc_pin = adc_config.enable_pin(
        io.pins.gpio1,
        Attenuation::Attenuation11dB,
    );

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
