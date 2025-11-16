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
    gpio::{
        Input, InputConfig, Level, Output, OutputConfig,
        Pull,
    },
    interrupt::software::SoftwareInterruptControl,
    timer::timg::TimerGroup,
};
use portable_atomic::AtomicU32;

esp_bootloader_esp_idf::esp_app_desc!();

// Global Variable to Control LED Rotation Speed
static BLINK_DELAY: AtomicU32 = AtomicU32::new(200_u32);

type ButtonType =
    Mutex<CriticalSectionRawMutex, Option<Input<'static>>>;
static BUTTON: ButtonType = Mutex::new(None);

#[esp_rtos::main]
async fn main(spawner: Spawner) {
    // Take Peripherals
    let peripherals =
        esp_hal::init(esp_hal::Config::default());

    // Initalize embassy executor
    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let sw_int = SoftwareInterruptControl::new(
        peripherals.SW_INTERRUPT,
    );
    esp_rtos::start(
        timg0.timer0,
        sw_int.software_interrupt0,
    );

    // Configure Delay Button to Pull Up input
    let del_but_config =
        InputConfig::default().with_pull(Pull::Up);
    let del_but =
        Input::new(peripherals.GPIO3, del_but_config);
    // Inner scope is so that once the mutex is written to, the MutexGuard is dropped, thus the
    // Mutex is released
    {
        *(BUTTON.lock().await) = Some(del_but);
    }
    // Configure LED Array Pins to Output & Store in Array
    let led_array_config = OutputConfig::default();
    let mut leds: [Output; 10] = [
        Output::new(
            peripherals.GPIO1,
            Level::Low,
            led_array_config,
        ),
        Output::new(
            peripherals.GPIO10,
            Level::Low,
            led_array_config,
        ),
        Output::new(
            peripherals.GPIO19,
            Level::Low,
            led_array_config,
        ),
        Output::new(
            peripherals.GPIO18,
            Level::Low,
            led_array_config,
        ),
        Output::new(
            peripherals.GPIO4,
            Level::Low,
            led_array_config,
        ),
        Output::new(
            peripherals.GPIO5,
            Level::Low,
            led_array_config,
        ),
        Output::new(
            peripherals.GPIO6,
            Level::Low,
            led_array_config,
        ),
        Output::new(
            peripherals.GPIO7,
            Level::Low,
            led_array_config,
        ),
        Output::new(
            peripherals.GPIO8,
            Level::Low,
            led_array_config,
        ),
        Output::new(
            peripherals.GPIO9,
            Level::Low,
            led_array_config,
        ),
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
