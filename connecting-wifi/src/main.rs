/*
Simplified Embedded Rust: ESP Core Library Edition
IoT and Networking - Connecting WiFi Example
*/

#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_alloc as _;
use esp_backtrace as _;
use esp_hal::{
    clock::CpuClock,
    interrupt::software::SoftwareInterruptControl,
    timer::timg::TimerGroup,
};
use esp_println::println;
use esp_radio::wifi::{
    sta::StationConfig, AuthenticationMethod, Config,
    ControllerConfig, WifiController,
};

esp_bootloader_esp_idf::esp_app_desc!();

const SSID: &str = "Wokwi-GUEST";
const PASSWORD: &str = "";

#[esp_rtos::main]
async fn main(_spawner: Spawner) -> ! {
    let config = esp_hal::Config::default()
        .with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    // Allocate Heap Memory for the WiFi Stack
    esp_alloc::heap_allocator!(size: 72 * 1024);

    // Initalize embassy executor
    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let sw_int = SoftwareInterruptControl::new(
        peripherals.SW_INTERRUPT,
    );
    esp_rtos::start(
        timg0.timer0,
        sw_int.software_interrupt0,
    );

    // Create Controller Configuration
    let station_config = Config::Station(
        StationConfig::default()
            .with_ssid(SSID)
            .with_password(PASSWORD.into())
            .with_auth_method(AuthenticationMethod::None),
    );

    println!("Starting WiFi");

    // Instantiate the WiFi Controller with the configuration
    let mut controller = WifiController::new(
        peripherals.WIFI,
        ControllerConfig::default()
            .with_initial_config(station_config),
    )
    .unwrap();
    println!("Wifi configured and started!");

    loop {
        println!("About to connect...");
        // Connect to the WiFi network
        match controller.connect_async().await {
            Ok(info) => {
                println!("Wifi connected to {:?}", info);

                // Wait until a disconnect event happens, then print the disconnect info/reason
                let info = controller
                    .wait_for_disconnect_async()
                    .await
                    .ok();
                println!("Disconnected: {:?}", info);
            }
            Err(e) => {
                println!(
                    "Failed to connect to wifi: {e:?}"
                );
            }
        }

        // Wait for 5 seconds before trying to connect again
        Timer::after(Duration::from_millis(5000)).await;
    }
}
