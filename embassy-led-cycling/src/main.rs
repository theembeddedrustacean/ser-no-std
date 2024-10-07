/*
Simplified Embedded Rust: ESP Core Library Edition
The Embassy Framework - LED Cycling Application Example
*/

#![no_std]
#![no_main]

use core::sync::atomic::Ordering;
use embassy_executor::Spawner;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::mutex::Mutex;
use embassy_time::{Duration, Timer};
use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl,
    gpio::{AnyInput, AnyOutput, Io, Level, Pull},
    peripherals::Peripherals,
    system::SystemControl,
    timer::timg::TimerGroup,
};
use portable_atomic::AtomicU32;

// Global Variable to Control LED Rotation Speed
static BLINK_DELAY: AtomicU32 = AtomicU32::new(200_u32);

type ButtonType = Mutex<
    CriticalSectionRawMutex,
    Option<AnyInput<'static>>,
>;
static BUTTON: ButtonType = Mutex::new(None);

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    // Take Peripherals
    let peripherals = Peripherals::take();
    let system = SystemControl::new(peripherals.SYSTEM);
    let clocks =
        ClockControl::max(system.clock_control).freeze();

    // Initalize embassy executor
    let timg0 = TimerGroup::new(peripherals.TIMG0, &clocks);
    esp_hal_embassy::init(&clocks, timg0.timer0);

    // Acquire Handle to IO
    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);
    // Configure Delay Button to Pull Up input
    let del_but = AnyInput::new(io.pins.gpio3, Pull::Up);
    // Inner scope is so that once the mutex is written to, the MutexGuard is dropped, thus the
    // Mutex is released
    {
        *(BUTTON.lock().await) = Some(del_but);
    }
    // Configure LED Array Pins to Output & Store in Array
    let mut leds: [AnyOutput; 10] = [
        AnyOutput::new(io.pins.gpio1, Level::Low),
        AnyOutput::new(io.pins.gpio10, Level::Low),
        AnyOutput::new(io.pins.gpio19, Level::Low),
        AnyOutput::new(io.pins.gpio18, Level::Low),
        AnyOutput::new(io.pins.gpio4, Level::Low),
        AnyOutput::new(io.pins.gpio5, Level::Low),
        AnyOutput::new(io.pins.gpio6, Level::Low),
        AnyOutput::new(io.pins.gpio7, Level::Low),
        AnyOutput::new(io.pins.gpio8, Level::Low),
        AnyOutput::new(io.pins.gpio9, Level::Low),
    ];

    // Spawn Button Press Task
    spawner.spawn(press_button(&BUTTON)).unwrap();

    // This line is for Wokwi only so that the console output is formatted correctly
    esp_println::print!("\x1b[20h");

    // Enter Application Loop Blinking on LED at a Time
    loop {
        for led in &mut leds {
            led.set_high();
            Timer::after(Duration::from_millis(
                BLINK_DELAY.load(Ordering::Relaxed) as u64,
            ))
            .await;
            led.set_low();
            Timer::after(Duration::from_millis(100)).await;
        }
    }
}

#[embassy_executor::task]
async fn press_button(button: &'static ButtonType) {
    loop {
        // Wait for Button Press
        {
            let mut button_unlocked = button.lock().await;
            if let Some(button_ref) =
                button_unlocked.as_mut()
            {
                button_ref.wait_for_rising_edge().await;
                esp_println::println!("Button Pressed!");
            }
        }
        // Retrieve Delay Global Variable
        let del = BLINK_DELAY.load(Ordering::Relaxed);
        // Adjust Delay Accordingly
        if del <= 50_u32 {
            BLINK_DELAY.store(200_u32, Ordering::Relaxed);
            esp_println::println!("Delay is now 200ms");
        } else {
            BLINK_DELAY
                .store(del - 50_u32, Ordering::Relaxed);
            esp_println::println!(
                "Delay is now {}ms",
                del - 50_u32
            );
        }
    }
}
