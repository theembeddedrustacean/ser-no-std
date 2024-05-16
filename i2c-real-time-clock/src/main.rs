/*
Simplified Embedded Rust: ESP Core Library Edition
Programming Serial Communication - I2C Real-time Clock Application Example
*/

#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl, delay::Delay, gpio::IO, i2c::I2C, peripherals::Peripherals, prelude::*,
};
use esp_println::println;
use nobcd::BcdNumber;

const DS1307_ADDR: u8 = 0x68;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();

    let clocks = ClockControl::max(system.clock_control).freeze();
    let delay = Delay::new(&clocks);

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    let mut ds1307 = I2C::new(
        peripherals.I2C0,
        io.pins.gpio3,
        io.pins.gpio2,
        100u32.kHz(),
        &clocks,
        None,
    );

    #[repr(u8)]
    enum DS1307 {
        Seconds,
        Minutes,
        Hours,
        Day,
        Date,
        Month,
        Year,
    }

    enum DAY {
        Sun = 1,
        Mon = 2,
        Tues = 3,
        Wed = 4,
        Thurs = 5,
        Fri = 6,
    }

    struct DateTime {
        sec: u8,
        min: u8,
        hrs: u8,
        day: u8,
        date: u8,
        mnth: u8,
        yr: u8,
    }

    let start_dt = DateTime {
        sec: 0,
        min: 0,
        hrs: 0,
        day: DAY::Fri as u8,
        date: 15,
        mnth: 5,
        yr: 15,
    };

    // Set Time
    // Set Seconds -> Also Activates Oscillator
    let secs: [u8; 1] = BcdNumber::new(start_dt.sec).unwrap().bcd_bytes();
    ds1307
        .write(DS1307_ADDR, &[DS1307::Seconds as u8, secs[0]])
        .unwrap();
    // Set Minutes
    let mins: [u8; 1] = BcdNumber::new(start_dt.min).unwrap().bcd_bytes();
    ds1307
        .write(DS1307_ADDR, &[DS1307::Minutes as u8, mins[0]])
        .unwrap();
    // Set Hours
    let hrs: [u8; 1] = BcdNumber::new(start_dt.hrs).unwrap().bcd_bytes();
    ds1307
        .write(DS1307_ADDR, &[DS1307::Hours as u8, hrs[0]])
        .unwrap();
    // Set Day of Week
    let dow: [u8; 1] = BcdNumber::new(start_dt.day).unwrap().bcd_bytes();
    ds1307
        .write(DS1307_ADDR, &[DS1307::Day as u8, dow[0]])
        .unwrap();
    // Set Day of Month
    let dom: [u8; 1] = BcdNumber::new(start_dt.date).unwrap().bcd_bytes();
    ds1307
        .write(DS1307_ADDR, &[DS1307::Date as u8, dom[0]])
        .unwrap();
    // Set Month
    let mnth: [u8; 1] = BcdNumber::new(start_dt.mnth).unwrap().bcd_bytes();
    ds1307
        .write(DS1307_ADDR, &[DS1307::Month as u8, mnth[0]])
        .unwrap();
    // Set Year
    let yr: [u8; 1] = BcdNumber::new(start_dt.yr).unwrap().bcd_bytes();
    ds1307
        .write(DS1307_ADDR, &[DS1307::Year as u8, yr[0]])
        .unwrap();

    loop {
        // Initialize Array that will buffer data read from the DS1307
        let mut data: [u8; 7] = [0_u8; 7];

        // Provide Starting Address (zero) to Read Data from DS1307
        ds1307.write(DS1307_ADDR, &[0_u8]).unwrap();
        ds1307.read(DS1307_ADDR, &mut data).unwrap();

        println!("{:?}", data);

        let secs = BcdNumber::from_bcd_bytes([data[0] & 0x7f])
            .unwrap()
            .value::<u8>();
        let mins = BcdNumber::from_bcd_bytes([data[1]]).unwrap().value::<u8>();
        let hrs = BcdNumber::from_bcd_bytes([data[2] & 0x3f])
            .unwrap()
            .value::<u8>();
        let dom = BcdNumber::from_bcd_bytes([data[4]]).unwrap().value::<u8>();
        let mnth = BcdNumber::from_bcd_bytes([data[5]]).unwrap().value::<u8>();
        let yr = BcdNumber::from_bcd_bytes([data[6]]).unwrap().value::<u8>();
        let dow = match BcdNumber::from_bcd_bytes([data[3]]).unwrap().value::<u8>() {
            1 => "Sunday",
            2 => "Monday",
            3 => "Tuesday",
            4 => "Wednesday",
            5 => "Thursday",
            6 => "Friday",
            7 => "Saturday",
            _ => "",
        };

        println!(
            "{}, {}/{}/20{}, {:02}:{:02}:{:02}",
            dow, dom, mnth, yr, hrs, mins, secs
        );

        delay.delay_millis(1000_u32);
    }
}
