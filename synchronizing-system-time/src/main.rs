/*
Simplified Embedded Rust: ESP Core Library Edition
IoT and Networking - NTP Synchronization Example
*/

#![no_std]
#![no_main]

use core::net::{IpAddr, SocketAddr};

use embassy_executor::Spawner;
use embassy_net::{
    dns::DnsQueryType,
    udp::{PacketMetadata, UdpSocket},
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

use sntpc::{get_time, NtpContext, NtpTimestampGenerator};
use sntpc_net_embassy::UdpSocketWrapper;
use static_cell::StaticCell;

esp_bootloader_esp_idf::esp_app_desc!();

// Permanent memory slot for stack resources, which are used by the network stack.
static STACK_RESOURCES: StaticCell<StackResources<3>> =
    StaticCell::new();

const SSID: &str = "Wokwi-GUEST";
const PASSWORD: &str = "";
const NTP_SERVER: &str = "pool.ntp.org";

#[derive(Clone, Copy)]
struct Timestamp {
    current_time_us: u64,
}

impl NtpTimestampGenerator for Timestamp {
    fn init(&mut self) {
        self.current_time_us = 0;
    }

    fn timestamp_sec(&self) -> u64 {
        self.current_time_us / 1_000_000
    }

    fn timestamp_subsec_micros(&self) -> u32 {
        (self.current_time_us % 1_000_000) as u32
    }
}

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

    // Query the DNS server for the NTP server's IP address
    let ntp_addrs = stack
        .dns_query(NTP_SERVER, DnsQueryType::A)
        .await
        .unwrap();

    if ntp_addrs.is_empty() {
        panic!("Failed to resolve DNS. Empty result");
    }

    // Create buffers for the UDP socket
    let mut rx_meta = [PacketMetadata::EMPTY; 16];
    let mut rx_buffer = [0; 4096];
    let mut tx_meta = [PacketMetadata::EMPTY; 16];
    let mut tx_buffer = [0; 4096];

    // Instantiate a UDP socket with the network stack and buffers
    let mut socket = UdpSocket::new(
        stack,
        &mut rx_meta,
        &mut rx_buffer,
        &mut tx_meta,
        &mut tx_buffer,
    );

    // Bind the socket to port 123 (NTP) for sending and receiving NTP packets
    socket.bind(123).unwrap();

    // Wrap the UDP socket in a UdpSocketWrapper to implement the required traits for SNTP
    let socket = UdpSocketWrapper::new(socket);

    loop {
        // Convert the first resolved NTP server address to an IpAddr
        let addr: IpAddr = ntp_addrs[0].into();

        // Get the current time from the NTP server using the SNTP client
        let result = get_time(
            SocketAddr::from((addr, 123)),
            &socket,
            NtpContext::new(Timestamp {
                current_time_us: 0,
            }),
        )
        .await;

        match result {
            Ok(timestamp) => {
                println!(
                    "Current time: {} seconds, {} microseconds",
                    timestamp.sec(),
                    timestamp.sec_fraction()
                );
            }
            Err(e) => {
                println!("Failed to get time from NTP server: {:?}", e);
            }
        }

        Timer::after(Duration::from_secs(5)).await;
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
