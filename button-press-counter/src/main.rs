/*
Simplified Embedded Rust: ESP Core Library Edition
Programming GPIO - Button Press Counter Application Example
*/

#![no_std]
#![no_main]

use core::cell::{Cell, RefCell};
use critical_section::Mutex;
use esp_backtrace as _;
use esp_hal::{
    gpio::{Event, Gpio0, Input, PullUp, IO},
    peripherals::Peripherals,
    prelude::*,
};
use esp_println::println;

// Create a Global Variable for a GPIO Peripheral to pass around between threads.
static G_PIN: Mutex<RefCell<Option<Gpio0<Input<PullUp>>>>> = Mutex::new(RefCell::new(None));
// Create a Global Variable for a FLAG to pass around between threads.
static G_FLAG: Mutex<Cell<bool>> = Mutex::new(Cell::new(false));

// ISR Definition
#[handler]
fn gpio() {
    // Start a Critical Section
    critical_section::with(|cs| {
        // Obtain access to Global Button Peripheral and Clear Interrupt Pending Flag
        G_PIN.borrow_ref_mut(cs).as_mut().unwrap().clear_interrupt();
        // Assert G_FLAG indicating a press button happened
        G_FLAG.borrow(cs).set(true);
    });
}

#[entry]
fn main() -> ! {
    // Take Peripherals
    let peripherals = Peripherals::take();

    // Create IO Driver
    let mut io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    // Interrupt Configuration
    // Step 1: Register interrupt handler
    io.set_interrupt_handler(gpio);
    // Step 2: Configure button pin direction
    let mut some_pin = io.pins.gpio0.into_pull_up_input();
    // Step 3: Configure button input to trigger an interrupt on the falling edge and start listening to events
    some_pin.listen(Event::FallingEdge);
    // Step 4: Now that button is configured, move the input pin to the global context
    critical_section::with(|cs| G_PIN.borrow_ref_mut(cs).replace(some_pin));

    // Instantiate variable to keep button count
    let mut count = 0_u32;
    loop {
        critical_section::with(|cs| {
            if G_FLAG.borrow(cs).get() {
                // Clear global flag
                G_FLAG.borrow(cs).set(false);
                // Increment count and print to console
                count += 1;
                println!("Button press count = {}", count);
            }
        });
    }
}
