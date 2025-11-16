/*
Simplified Embedded Rust: ESP Core Library Edition
Programming PWM - LED Fading Application Example
*/

#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    ledc::{
        channel, channel::ChannelIFace, timer,
        timer::TimerIFace, LSGlobalClkSource, Ledc,
        LowSpeed,
    },
    main,
    time::Rate,
};

esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    // Take Peripherals and Configure System Clocks
    let peripherals =
        esp_hal::init(esp_hal::Config::default());

    // Instantiate delay abstraction
    let delay = Delay::new();

    // Instantiate GPIO Pin to be used for LEDC peripheral
    let led = peripherals.GPIO7;

    // Create LEDC instance with low speed global clock
    let mut ledc = Ledc::new(peripherals.LEDC);
    ledc.set_global_slow_clock(LSGlobalClkSource::APBClk);

    // Configure LEDC timer
    let mut timer =
        ledc.timer::<LowSpeed>(timer::Number::Timer0);
    timer
        .configure(timer::config::Config {
            duty: timer::config::Duty::Duty14Bit,
            clock_source: timer::LSClockSource::APBClk,
            frequency: Rate::from_khz(1u32),
        })
        .unwrap();

    // Configure LEDC Channel Attaching Timer and Pin
    let mut channel =
        ledc.channel(channel::Number::Channel0, led);
    channel
        .configure(channel::config::Config {
            timer: &timer,
            duty_pct: 0,
            drive_mode: esp_hal::gpio::DriveMode::PushPull,
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
