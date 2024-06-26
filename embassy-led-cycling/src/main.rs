/*
Simplified Embedded Rust: ESP Core Library Edition
The Embassy Framework - LED Cycling Application Example
*/

#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use core::sync::atomic::Ordering;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl,
    embassy,
    gpio::{AnyPin, Input, InputOutputAnalogPinType, Output, PullUp, PushPull, IO},
    peripherals::Peripherals,
    prelude::*,
    timer::TimerGroup,
};
use portable_atomic::AtomicU32;

// Global Variable to Control LED Rotation Speed
static BLINK_DELAY: AtomicU32 = AtomicU32::new(200_u32);

#[main]
async fn main(spawner: Spawner) {
    // Take Peripherals
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::max(system.clock_control).freeze();

    // Initalize embassy executor
    let timg0 = TimerGroup::new_async(peripherals.TIMG0, &clocks);
    embassy::init(&clocks, timg0);

    // Acquire Handle to IO
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    // Configure Delay Button to Pull Up input
    let del_but = io.pins.gpio3.into_pull_up_input().degrade();
    // Configure LED Array Pins to Output & Store in Array
    let mut leds: [AnyPin<Output<PushPull>>; 10] = [
        io.pins.gpio1.into_push_pull_output().degrade().into(),
        io.pins.gpio10.into_push_pull_output().degrade().into(),
        io.pins.gpio19.into_push_pull_output().degrade().into(),
        io.pins.gpio18.into_push_pull_output().degrade().into(),
        io.pins.gpio4.into_push_pull_output().degrade().into(),
        io.pins.gpio5.into_push_pull_output().degrade().into(),
        io.pins.gpio6.into_push_pull_output().degrade().into(),
        io.pins.gpio7.into_push_pull_output().degrade().into(),
        io.pins.gpio8.into_push_pull_output().degrade().into(),
        io.pins.gpio9.into_push_pull_output().degrade().into(),
    ];

    // Spawn Button Press Task
    spawner.spawn(press_button(del_but)).unwrap();

    // This line is for Wokwi only so that the console output is formatted correctly
    esp_println::print!("\x1b[20h");

    // Enter Application Loop Blinking on LED at a Time
    loop {
        for led in &mut leds {
            led.set_high();
            Timer::after(Duration::from_millis(
                BLINK_DELAY.load(Ordering::Relaxed) as u64
            ))
            .await;
            led.set_low();
            Timer::after(Duration::from_millis(100)).await;
        }
    }
}

#[embassy_executor::task]
async fn press_button(mut button: AnyPin<Input<PullUp>, InputOutputAnalogPinType>) {
    loop {
        // Wait for Button Press
        button.wait_for_rising_edge().await;
        esp_println::println!("Button Pressed!");
        // Retrieve Delay Global Variable
        let del = BLINK_DELAY.load(Ordering::Relaxed);
        // Adjust Delay Accordingly
        if del <= 50_u32 {
            BLINK_DELAY.store(200_u32, Ordering::Relaxed);
            esp_println::println!("Delay is now 200ms");
        } else {
            BLINK_DELAY.store(del - 50_u32, Ordering::Relaxed);
            esp_println::println!("Delay is now {}ms", del - 50_u32);
        }
    }
}
