/*
Simplified Embedded Rust: ESP Core Library Edition
The Embassy Framework - UART Echo Application Example
*/

#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use embassy_executor::Spawner;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, pipe::Pipe};
use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl,
    embassy::{self},
    peripherals::{Peripherals, UART0},
    prelude::*,
    timer::TimerGroup,
    uart::{config::AtCmdConfig, Uart, UartRx, UartTx},
    Async,
};

// Read Buffer Size
const READ_BUF_SIZE: usize = 64;

// End of Transmission Character (Carrige Return -> 13 or 0x0D in ASCII)
const AT_CMD: u8 = 0x0D;

// Declare Pipe sync primitive to share data among Tx and Rx tasks
static DATAPIPE: Pipe<CriticalSectionRawMutex, READ_BUF_SIZE> = Pipe::new();

#[embassy_executor::task]
async fn uart_writer(mut tx: UartTx<'static, UART0, Async>) {
    // Declare write buffer to store Tx characters
    let mut wbuf: [u8; READ_BUF_SIZE] = [0u8; READ_BUF_SIZE];
    loop {
        // Read characters from pipe into write buffer
        DATAPIPE.read(&mut wbuf).await;
        // Transmit/echo buffer contents over UART
        embedded_io_async::Write::write(&mut tx, &wbuf)
            .await
            .unwrap();
        // Transmit a new line
        embedded_io_async::Write::write(&mut tx, &[0x0D, 0x0A])
            .await
            .unwrap();
        // Flush transmit buffer
        embedded_io_async::Write::flush(&mut tx).await.unwrap();
    }
}

#[embassy_executor::task]
async fn uart_reader(mut rx: UartRx<'static, UART0, Async>) {
    // Declare read buffer to store Rx characters
    let mut rbuf: [u8; READ_BUF_SIZE] = [0u8; READ_BUF_SIZE];
    loop {
        // Read characters from UART into read buffer until EOT
        let r = embedded_io_async::Read::read(&mut rx, &mut rbuf[0..]).await;
        match r {
            Ok(len) => {
                // If read succeeds then write recieved characters to pipe
                DATAPIPE.write_all(&rbuf[..len]).await;
            }
            Err(e) => esp_println::println!("RX Error: {:?}", e),
        }
    }
}

#[main]
async fn main(spawner: Spawner) {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    // Initalize embassy executor
    let timg0 = TimerGroup::new_async(peripherals.TIMG0, &clocks);
    embassy::init(&clocks, timg0);

    // Initialize and configure UART0
    let mut uart0 = Uart::new_async(peripherals.UART0, &clocks);
    uart0.set_at_cmd(AtCmdConfig::new(None, None, None, AT_CMD, None));
    uart0
        .set_rx_fifo_full_threshold(READ_BUF_SIZE as u16)
        .unwrap();
    // Split UART0 to create seperate Tx and Rx handles
    let (tx, rx) = uart0.split();

    // Spawn Tx and Rx tasks
    spawner.spawn(uart_reader(rx)).ok();
    spawner.spawn(uart_writer(tx)).ok();
}
