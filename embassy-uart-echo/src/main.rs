/*
Simplified Embedded Rust: ESP Core Library Edition
The Embassy Framework - UART Echo Application Example
*/

#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex,
    pipe::Pipe,
};
use esp_backtrace as _;
use esp_hal::{
    interrupt::software::SoftwareInterruptControl,
    timer::timg::TimerGroup,
    uart::{
        AtCmdConfig, Config, RxConfig, Uart, UartRx,
        UartTx,
    },
    Async,
};
use esp_println::println;

esp_bootloader_esp_idf::esp_app_desc!();

// Read Buffer Size
const READ_BUF_SIZE: usize = 64;

// End of Transmission Character (Carrige Return -> 13 or 0x0D in ASCII)
const AT_CMD: u8 = 0x0D;

// Declare Pipe sync primitive to share data among Tx and Rx tasks
static DATAPIPE: Pipe<
    CriticalSectionRawMutex,
    READ_BUF_SIZE,
> = Pipe::new();

#[embassy_executor::task]
async fn uart_writer(mut tx: UartTx<'static, Async>) {
    // Declare write buffer to store Tx characters
    let mut wbuf: [u8; READ_BUF_SIZE] =
        [0u8; READ_BUF_SIZE];
    loop {
        // Read characters from pipe into write buffer
        DATAPIPE.read(&mut wbuf).await;
        // Transmit/echo buffer contents over UART
        println!("Sending Letter");
        tx.write_async(&wbuf).await.unwrap();
        // Transmit a new line
        println!("Sending New Line");
        tx.write_async(&[0x0D, 0x0A]).await.unwrap();
        // Flush transmit buffer
        println!("Flushing");
        tx.flush_async().await.unwrap();
    }
}

#[embassy_executor::task]
async fn uart_reader(mut rx: UartRx<'static, Async>) {
    // Declare read buffer to store Rx characters
    let mut rbuf: [u8; READ_BUF_SIZE] =
        [0u8; READ_BUF_SIZE];
    loop {
        // Read characters from UART into read buffer
        let r = rx.read_async(&mut rbuf[0..]).await;
        match r {
            Ok(len) => {
                // If read succeeds then write recieved characters to pipe
                DATAPIPE.write_all(&rbuf[..len]).await;
            }
            Err(e) => {
                println!("RX Error: {:?}", e)
            }
        }
    }
}

#[esp_rtos::main]
async fn main(spawner: Spawner) {
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

    // Instantiate GPIO pins for UART
    let (tx_pin, rx_pin) =
        (peripherals.GPIO21, peripherals.GPIO20);

    // Initialize and configure UART0
    let config = Config::default().with_rx(
        RxConfig::default().with_fifo_full_threshold(
            READ_BUF_SIZE as u16,
        ),
    );
    let mut uart0 = Uart::new(peripherals.UART0, config)
        .unwrap()
        .with_tx(tx_pin)
        .with_rx(rx_pin)
        .into_async();
    uart0.set_at_cmd(
        AtCmdConfig::default().with_cmd_char(AT_CMD),
    );

    // Split UART0 to create seperate Tx and Rx handles
    let (rx, tx) = uart0.split();

    // Spawn Tx and Rx tasks
    spawner.spawn(uart_reader(rx)).ok();
    spawner.spawn(uart_writer(tx)).ok();
}
