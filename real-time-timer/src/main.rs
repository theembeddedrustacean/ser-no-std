/*
Simplified Embedded Rust: ESP Core Library Edition
Programming Timers & Counters - Real-time Timer Application Example
*/

#![no_std]
#![no_main]

use core::cell::{Cell, RefCell};
use critical_section::Mutex;
use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl,
    // interrupt::{self, Priority},
    peripherals::{Peripherals, TIMG0},
    prelude::*,
    timer::{Timer, Timer0, TimerGroup, TimerInterrupts},
};
use esp_println::println;

// Create a Global Variable for a GPIO Peripheral to pass around between threads.
static G_TIMER: Mutex<RefCell<Option<Timer<Timer0<TIMG0>, esp_hal::Blocking>>>> =
    Mutex::new(RefCell::new(None));
// Create a Global Variable for a FLAG to pass around between threads.
static G_FLAG: Mutex<Cell<bool>> = Mutex::new(Cell::new(false));

struct Time {
    seconds: u32,
    minutes: u32,
    hours: u32,
}

// ISR Definition
#[handler]
fn tg0_t0_level() {
    // Start a Critical Section
    critical_section::with(|cs| {
        // Clear Timer Interrupt Pending Flag
        G_TIMER
            .borrow_ref_mut(cs)
            .as_mut()
            .unwrap()
            .clear_interrupt();
        // Re-activate Timer Alarm For Interrupts to Occur again
        G_TIMER
            .borrow_ref_mut(cs)
            .as_mut()
            .unwrap()
            .set_alarm_active(true);
        // Assert G_FLAG indicating a press button happened
        G_FLAG.borrow(cs).set(true);
    });
}

#[entry]
fn main() -> ! {
    // Take Peripherals
    let peripherals = Peripherals::take();

    // Set up system clocks
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    // Instantiate Timer Group 0
    let timer_group0 = TimerGroup::new(
        peripherals.TIMG0,
        &clocks,
        Some(TimerInterrupts {
            timer0_t0: Some(tg0_t0_level),
            ..Default::default()
        }),
    );

    // Instantiate Timer0 in Timer Group 0
    let mut timer0 = timer_group0.timer0;

    // Interrupt Configuration
    // Step 1: Configure timer to trigger an interrupt every second
    // Load count equivalent to 1 second
    timer0.load_alarm_value(40000000);
    // Enable Alarm to generate interrupts
    timer0.set_alarm_active(true);
    // Activate counter
    timer0.set_counter_active(true);
    // Step 2: Start listening for timer events
    timer0.listen();
    // Step 3: Now that input is configured, move the input pin to the global context
    critical_section::with(|cs| G_TIMER.borrow_ref_mut(cs).replace(timer0));

    // Set up a Time struct to keep track of time
    let mut time = Time {
        seconds: 0_u32,
        minutes: 0_u32,
        hours: 0_u32,
    };

    loop {
        critical_section::with(|cs| {
            if G_FLAG.borrow(cs).get() {
                // Clear global flag
                G_FLAG.borrow(cs).set(false);
                // Update Time struct and print
                time.seconds = time.seconds.wrapping_add(1);
                if time.seconds > 59 {
                    time.minutes += 1;
                }
                if time.minutes > 59 {
                    time.hours += 1;
                }
                if time.hours > 23 {
                    time.seconds = 0;
                    time.minutes = 0;
                    time.hours = 0;
                }
                println!(
                    "Elapsed Time {:0>2}:{:0>2}:{:0>2}",
                    time.hours, time.minutes, time.seconds
                );
            }
        });
    }
}
