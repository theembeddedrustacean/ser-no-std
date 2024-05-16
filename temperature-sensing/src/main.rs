/*
Simplified Embedded Rust: ESP Core Library Edition
Programming ADCs - Temperature Sensing Application Example
*/

#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    analog::adc::{AdcConfig, Attenuation, ADC},
    clock::ClockControl,
    delay::Delay,
    gpio::IO,
    peripherals::Peripherals,
    prelude::*,
};
use esp_println::println;
use libm::log;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();

    let clocks = ClockControl::max(system.clock_control).freeze();
    let delay = Delay::new(&clocks);

    // Instantiate and Create Handle for IO
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    // Create handle for ADC configuration parameters
    let mut adc_config = AdcConfig::new();

    // Configure ADC pin
    let mut adc_pin =
        adc_config.enable_pin(io.pins.gpio4.into_analog(), Attenuation::Attenuation11dB);

    // Create ADC Driver
    let mut adc = ADC::new(peripherals.ADC1, adc_config);

    const B: f64 = 3950.0; // B value of the thermistor
    const VMAX: f64 = 4095.0; // Full Range Voltage

    loop {
        // Get ADC reading
        let sample: u16 = adc.read_oneshot(&mut adc_pin).unwrap();
        // For blocking read
        // let sample: u16 = nb::block!(adc.read_oneshot(&mut adc_pin)).unwrap();

        // Convert to temperature
        let temperature = 1. / (log(1. / (VMAX / sample as f64 - 1.)) / B + 1.0 / 298.15) - 273.15;

        // Print the temperature output
        println!("Temperature {:02} Celcius\r", temperature);

        // Wait some time before next measurement
        delay.delay_millis(500_u32);
    }
}
