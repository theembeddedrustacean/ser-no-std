/*
Simplified Embedded Rust: ESP Core Library Edition
Programming PWM - LED Fading Application Example
*/

#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl,
    delay::Delay,
    gpio::IO,
    ledc::{channel, timer, LSGlobalClkSource, LowSpeed, LEDC},
    peripherals::Peripherals,
    prelude::*,
};

#[entry]
fn main() -> ! {
    // Take Peripherals and Configure System Clocks
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::max(system.clock_control).freeze();

    // Instantiate delay abstraction
    let delay = Delay::new(&clocks);

    // Configure GPIO Pin to be used for LEDC peripheral
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let led = io.pins.gpio7;

    // Create LEDC instance with low speed global clock
    let mut ledc = LEDC::new(peripherals.LEDC, &clocks);
    ledc.set_global_slow_clock(LSGlobalClkSource::APBClk);

    // Configure LEDC timer
    let mut timer = ledc.get_timer::<LowSpeed>(timer::Number::Timer0);
    timer
        .configure(timer::config::Config {
            duty: timer::config::Duty::Duty14Bit,
            clock_source: timer::LSClockSource::APBClk,
            frequency: 1u32.kHz(),
        })
        .unwrap();

    // Configure LEDC Channel Attaching Timer and Pin
    let mut channel = ledc.get_channel(channel::Number::Channel0, led);
    channel
        .configure(channel::config::Config {
            timer: &timer,
            duty_pct: 0,
            pin_config: channel::config::PinConfig::PushPull,
        })
        .unwrap();

    // Set the PWM Max Duty Cycle
    let max_duty = 100_u8;
    // Set the PWM Min Duty Cycle
    let min_duty = 0_u8;

    loop {
        // Sweep from 0% Duty to Maximum Duty (100%)
        for duty in min_duty..max_duty {
            // Set Duty
            channel.set_duty(duty).unwrap();
            // Delay to create fading effect
            delay.delay_millis(10_u32);
        }

        // Sweep from Maximum Duty (100%) to 0% Duty
        for duty in (min_duty..max_duty).rev() {
            // Set Duty
            channel.set_duty(duty).unwrap();
            // Delay to create fading effect
            delay.delay_millis(10_u32);
        }
    }
}
