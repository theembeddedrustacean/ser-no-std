// Build a door lock system using the keypad you built in the GPIO questions, a servo, a red LED, and
// a green LED. The system should prompt the user to enter a 4-digit code using the keypad. Upon
// entering the code, the system should compare it with a code hardcoded in your application. If the
// entered code matches the hardcoded code, rotate the servo 90 degrees, then light up the green LED, indicating
// that the door is unlocked. Otherwise, if the entered code does not match the hardcoded code,
// the red LED should light up, indicating that the door is still locked.

#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_backtrace as _;
use esp_hal::timer::timg::TimerGroup;
use esp_println::println;

#[esp_hal_embassy::main]
async fn main(_spawner: Spawner) {
    let peripherals =
        esp_hal::init(esp_hal::Config::default());

    // Initalize embassy executor
    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_hal_embassy::init(timg0.timer0);

    loop {
        Timer::after(Duration::from_millis(1_000)).await;
        println!("Hello, embassy!");
    }
}
