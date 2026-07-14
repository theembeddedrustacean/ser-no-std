/*
Simplified Embedded Rust: ESP Core Library Edition
IoT and Networking - Simple HTTP Client Example
*/

#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_net::{
    dns::DnsSocket,
    tcp::client::{TcpClient, TcpClientState},
    Runner, StackResources,
};
use embassy_time::{Duration, Timer};
use esp_alloc as _;
use esp_backtrace as _;
use esp_hal::{
    clock::CpuClock,
    interrupt::software::SoftwareInterruptControl,
    rng::Rng, timer::timg::TimerGroup,
};
use esp_println::println;
use esp_radio::wifi::{
    sta::StationConfig, AuthenticationMethod, Config,
    ControllerConfig, Interface, WifiController,
};
use reqwless::{
    client::HttpClient,
    request::{Method, RequestBuilder},
};
use static_cell::StaticCell;

esp_bootloader_esp_idf::esp_app_desc!();

// Permanent memory slot for stack resources, which are used by the network stack.
static STACK_RESOURCES: StaticCell<StackResources<3>> =
    StaticCell::new();

// Permanent memory slot for the TCP client state, which is used by the HTTP client.
static TCP_STATE: StaticCell<
    TcpClientState<1, 1500, 1500>,
> = StaticCell::new();

const SSID: &str = "Wokwi-GUEST";
const PASSWORD: &str = "";

#[esp_rtos::main]
async fn main(spawner: Spawner) -> ! {
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

    // Create a WiFi device interface
    let wifi_interface =
        esp_radio::wifi::Interface::station();

    println!("Starting WiFi");

    // Instantiate the WiFi controller with the configuration
    let controller = esp_radio::wifi::WifiController::new(
        peripherals.WIFI,
        ControllerConfig::default()
            .with_initial_config(station_config),
    )
    .unwrap();

    println!("WiFi configured and started!");

    // Create a network stack configuration
    let config =
        embassy_net::Config::dhcpv4(Default::default());

    // Create a random seed for the network stack
    let rng = Rng::new();
    let seed =
        (rng.random() as u64) << 32 | rng.random() as u64;

    // Instantiate network stack
    let (stack, runner) = embassy_net::new(
        wifi_interface,
        config,
        STACK_RESOURCES.init(StackResources::new()),
        seed,
    );

    // Spawn the connection task and the network runner task
    spawner.spawn(connection(controller).unwrap());
    spawner.spawn(net_task(runner).unwrap());

    // Wait for the network stack to be configured
    println!("Configuring Network Stack...");
    stack.wait_config_up().await;
    println!("Network Stack Configured");

    // Print the assigned IP address
    if let Some(config) = stack.config_v4() {
        println!("Got IP: {}", config.address);
    }

    // Instantiate TCP client for HTTP client
    let tcp_client = TcpClient::new(
        stack,
        TCP_STATE
            .init(TcpClientState::<1, 1500, 1500>::new()),
    );

    // Instantiate DNS client for HTTP client
    let dns_client = DnsSocket::new(stack);

    loop {
        // Instantiate HTTP client with TCP and DNS clients
        let mut client =
            HttpClient::new(&tcp_client, &dns_client);

        // Buffer for receiving the HTTP response body
        let mut rx_buf = [0u8; 4096];

        println!(
            "Sending HTTP GET request to httpbin.org..."
        );

        // Create an HTTP GET request to httpbin.org
        let request = client
            .request(Method::GET, "http://httpbin.org/get")
            .await
            .unwrap();

        // Add necessary headers to the request
        let mut request = request.headers(&[
            ("Host", "httpbin.org"),
            ("Connection", "close"),
        ]);

        // Send the request and receive the response
        let response =
            request.send(&mut rx_buf).await.unwrap();

        let date = response
            .headers()
            .find(|(name, _)| {
                name.eq_ignore_ascii_case("date")
            })
            .and_then(|(_, value)| {
                core::str::from_utf8(value).ok()
            });

        if let Some(date) = date {
            println!("Date: {}", date);
        } else {
            println!("Date header not found");
        }

        let content_len = response
            .headers()
            .find(|(name, _)| {
                name.eq_ignore_ascii_case("content-length")
            })
            .and_then(|(_, value)| {
                core::str::from_utf8(value).ok()
            });

        if let Some(content_len) = content_len {
            println!(
                "Content Length: {} bytes",
                content_len
            );
        } else {
            println!("Content-Length header not found");
        }

        Timer::after(Duration::from_millis(3000)).await;
    }
}

#[embassy_executor::task]
async fn connection(
    mut controller: WifiController<'static>,
) {
    println!("start connection task");

    loop {
        println!("About to connect...");

        match controller.connect_async().await {
            Ok(info) => {
                println!("WiFi connected to {:?}", info);

                // wait until we're no longer connected
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

        Timer::after(Duration::from_millis(5000)).await
    }
}

#[embassy_executor::task]
async fn net_task(mut runner: Runner<'static, Interface>) {
    runner.run().await
}
