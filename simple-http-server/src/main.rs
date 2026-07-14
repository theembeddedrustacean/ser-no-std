/*
Simplified Embedded Rust: ESP Core Library Edition
IoT and Networking - Simple HTTP Server Example
*/

#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_net::{
    tcp::TcpSocket, IpListenEndpoint, Runner,
    StackResources,
};
use embassy_time::{Duration, Timer};
use embedded_io_async::Write;
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
use static_cell::StaticCell;

esp_bootloader_esp_idf::esp_app_desc!();

// Permanent memory slot for stack resources, which are used by the network stack.
static STACK_RESOURCES: StaticCell<StackResources<3>> =
    StaticCell::new();

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
    // Note that this is not a static IP configuration, but rather a dynamic one that will be configured using DHCP.
    // You can create a static configuration by using `embassy_net::Config::ipv4_static` instead of `embassy_net::Config::dhcpv4`.
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

    // Create buffers for the TCP socket
    let mut rx_buffer = [0; 1536];
    let mut tx_buffer = [0; 1536];

    // Instantiate a TCP socket with the network stack and buffers
    let mut socket = TcpSocket::new(
        stack,
        &mut rx_buffer,
        &mut tx_buffer,
    );

    // Set a timeout for the socket operations
    socket.set_timeout(Some(
        embassy_time::Duration::from_secs(10),
    ));

    loop {
        // Bind the socket to port 80 (HTTP) & listen for incoming connections
        println!("Wait for connection...");
        let r = socket
            .accept(IpListenEndpoint {
                addr: None,
                port: 80,
            })
            .await;
        println!("Connected...");

        // Handle any errors that may occur during the accept operation
        if let Err(e) = r {
            println!("connect error: {:?}", e);
            continue;
        }

        // Create a buffer to read the incoming HTTP request.
        let mut buffer = [0u8; 1024];
        // Variable to keep track of the current position in the buffer.
        let mut pos = 0;
        loop {
            // Read data from the socket into the buffer, starting at the current position.
            match socket.read(&mut buffer[pos..]).await {
                // If the read operation returns Ok(0), it means the connection has been closed, so we break out of the loop.
                Ok(0) => break,
                // If the read operation returns Ok(len), it means len bytes have been read successfully.
                // We update the position in the buffer and check if we have received the end of the HTTP request (indicated by "\r\n\r\n") or if the buffer is full.
                Ok(len) => {
                    pos += len;
                    let request = core::str::from_utf8(
                        &buffer[..pos],
                    )
                    .unwrap_or("");
                    if request.contains("\r\n\r\n")
                        || pos == buffer.len()
                    {
                        break;
                    }
                }
                Err(_) => break,
            }
        }

        // Write HTTP response to the socket
        let r = socket
            .write_all(
                b"HTTP/1.0 200 OK\r\n\r\n\
            <html>\
                <body>\
                    <h1>Hello World from ESP!</h1>\
                </body>\
            </html>\r\n\
            ",
            )
            .await;

        // Handle any errors that may occur during the write operation
        if let Err(e) = r {
            println!("write error: {:?}", e);
        }

        // Flush the socket to ensure all data is sent
        let r = socket.flush().await;
        if let Err(e) = r {
            println!("flush error: {:?}", e);
        }

        // Wait for a second before closing the socket
        Timer::after(Duration::from_millis(1000)).await;

        // Close the socket and wait for a second before aborting it
        // Closes the write half of the socket
        socket.close();
        Timer::after(Duration::from_millis(1000)).await;

        // Abort the socket to free up resources
        // Closes the read half of the socket and frees up any resources associated with it
        socket.abort();
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
