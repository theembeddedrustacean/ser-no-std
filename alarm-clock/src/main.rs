#![no_std]
#![no_main]

use bitmask_enum::bitmask;
use core::cell::RefCell;
use embassy_executor::Spawner;
use embassy_sync::blocking_mutex::{raw::CriticalSectionRawMutex, Mutex};
use embassy_time::{Duration, Instant, Timer};
use esp_backtrace as _;
use esp_hal::i2c::master::{Config as I2cConfig, I2c};
use esp_hal::interrupt::software::SoftwareInterruptControl;
use esp_hal::timer::timg::TimerGroup;
use esp_hal::Blocking;

use esp_backtrace as _;
use esp_hal::gpio::{Input, InputConfig, Level, Output, OutputConfig, Pull};
use esp_hal::ledc::{
    channel, channel::ChannelIFace, timer, timer::TimerIFace, LSGlobalClkSource, Ledc, LowSpeed,
};
use esp_hal::time::Rate;
use esp_println::println;

#[derive(Debug, Clone, Copy)]
enum Button {
    Pressed,
    Released,
}

#[derive(Debug, Clone, Copy)]
enum AlarmSwitch {
    On,
    Off,
}

#[derive(Debug, Clone, Copy)]
enum FormatSwitch {
    H12,
    H24,
}

// Alarm Clock System Events
// Events from the I/O Expander
#[derive(Debug, Clone, Copy)]
pub struct SystemEvents {
    time_but: Button,
    alarm_but: Button,
    hour_but: Button,
    min_but: Button,
    snooze_but: Button,
    alarm_sw: AlarmSwitch,
    fmt_sw: FormatSwitch,
}

// Alarm Clock System States
#[derive(Debug)]
enum State {
    Unarmed,
    Armed,
    Snoozing,
    Alarming,
    SetAlarm,
    SetTime,
}
#[derive(Debug, Clone, Copy)]
struct Time {
    hours: u8,
    minutes: u8,
    seconds: u8,
}

#[derive(Debug, Clone, Copy)]
struct TimeKeeper {
    clocktime: Time,
    alarmtime: Time,
    display_mode: DisplayMode,
}

#[derive(Debug, Clone, Copy)]
enum DisplayMode {
    ShowClock,
    ShowAlarm,
}

impl TimeKeeper {
    const fn new() -> Self {
        TimeKeeper {
            clocktime: Time {
                hours: 0,
                minutes: 0,
                seconds: 0,
            },
            alarmtime: Time {
                hours: 0,
                minutes: 0,
                seconds: 0,
            },
            display_mode: DisplayMode::ShowClock,
        }
    }
    fn alarm_time_match(&self) -> bool {
        self.clocktime.hours == self.alarmtime.hours
            && self.clocktime.minutes == self.alarmtime.minutes
    }
    fn increment_time_hours(&mut self) {
        self.clocktime.hours = (self.clocktime.hours + 1) % 24;
    }
    fn increment_time_minutes(&mut self) {
        self.clocktime.minutes = (self.clocktime.minutes + 1) % 60;
    }
    fn increment_alarm_hours(&mut self) {
        self.alarmtime.hours = (self.alarmtime.hours + 1) % 24;
    }
    fn increment_alarm_minutes(&mut self) {
        self.alarmtime.minutes = (self.alarmtime.minutes + 1) % 60;
    }
    fn show_clock_time(&mut self) {
        self.display_mode = DisplayMode::ShowClock;
    }
    fn show_alarm_time(&mut self) {
        self.display_mode = DisplayMode::ShowAlarm;
    }
    fn tick(&mut self) {
        self.clocktime.seconds += 1;
        if self.clocktime.seconds >= 60 {
            self.clocktime.seconds = 0;
            self.clocktime.minutes += 1;
            if self.clocktime.minutes >= 60 {
                self.clocktime.minutes = 0;
                self.clocktime.hours += 1;
                if self.clocktime.hours >= 24 {
                    self.clocktime.hours = 0;
                }
            }
        }
    }
}

// TCA6424 Addresses
const TCA6424_ADDR: u8 = 0x22; // I2C address of the TCA6424
const IN_PORT0: u8 = 0x80; // Input Port 0 Register
const IN_PORT1: u8 = 0x81; // Input Port 1 Register
#[allow(dead_code)]
const IN_PORT2: u8 = 0x82; // Input Port 2 Register
const OUT_PORT0: u8 = 0x84; // Output Port 0 Register
const OUT_PORT1: u8 = 0x85; // Output Port 1 Register
const OUT_PORT2: u8 = 0x86; // Output Port 2 Register
#[allow(dead_code)]
const POL_INV_PORT0: u8 = 0x88; // Polarity Inversion Port 0 Register
#[allow(dead_code)]
const POL_INV_PORT1: u8 = 0x89; // Polarity Inversion Port 1 Register
#[allow(dead_code)]
const POL_INV_PORT2: u8 = 0x8A; // Polarity Inversion Port 2 Register
const CONFIG_PORT0: u8 = 0x8C; // Configuration Port 0 Register
#[allow(dead_code)]
const CONFIG_PORT1: u8 = 0x8D; // Configuration Port 1 Register
#[allow(dead_code)]
const CONFIG_PORT2: u8 = 0x8E; // Configuration Port 2 Register

// RTC Address
const RTC_ADDR: u8 = 0x68;

// I/O Expander Pin Definitions
#[bitmask(u8)]
pub enum IoExpPort0 {
    AlarmSwitchOn,  // Port 0 Pin 0
    AlarmSwitchOff, // Port 0 Pin 1
    P02,            // Port 0 Pin 2 (Unused)
    P03,            // Port 0 Pin 3 (Unused)
    AlarmButton,    // Port 0 Pin 4
    TimeButton,     // Port 0 Pin 5
    MinuteButton,   // Port 0 Pin 6
    HourButton,     // Port 0 Pin 7
}

#[bitmask(u8)]
pub enum IoExpPort1 {
    Digit1,         // Port 1 Pin 0
    Digit2,         // Port 1 Pin 1
    Digit3,         // Port 1 Pin 2
    Digit4,         // Port 1 Pin 3
    Led,            // Port 1 Pin 4
    PmLed,          // Port 1 Pin 5
    FormatSwitch12, // Port 1 Pin 6
    FormatSwitch24, // Port 1 Pin 7
}

#[bitmask(u8)]
pub enum IoExpPort2 {
    SegA, // Port 2 Pin 0
    SegB, // Port 2 Pin 1
    SegC, // Port 2 Pin 2
    SegD, // Port 2 Pin 3
    SegE, // Port 2 Pin 4
    SegF, // Port 2 Pin 5
    Dp,   // Port 2 Pin 6
    SegG, // Port 2 Pin 7
}

// Direction Configuration for TCA6424
const PORT0_DIR: u8 = 0xFF;
const PORT1_DIR: u8 = 0xC0;
const PORT2_DIR: u8 = 0x00;

// Mutex to Share same I2C singleton across threads
static SHARED_I2C: Mutex<CriticalSectionRawMutex, RefCell<Option<I2c<'static, Blocking>>>> =
    Mutex::new(RefCell::new(None));

esp_bootloader_esp_idf::esp_app_desc!();

// Global Event & Action Channels Shared Between Tasks)
static SHARED_SYS_EVENTS: Mutex<CriticalSectionRawMutex, RefCell<Option<SystemEvents>>> =
    Mutex::new(RefCell::new(None));
static SHARED_TIMEKEEPER: Mutex<CriticalSectionRawMutex, RefCell<TimeKeeper>> =
    Mutex::new(RefCell::new(TimeKeeper::new()));

pub struct Rtc3888<'a> {
    i2c: &'a Mutex<CriticalSectionRawMutex, RefCell<Option<I2c<'static, Blocking>>>>,
    address: u8,
}

impl<'a> Rtc3888<'a> {
    fn new(
        i2c: &'a Mutex<CriticalSectionRawMutex, RefCell<Option<I2c<'static, Blocking>>>>,
        address: u8,
    ) -> Self {
        Rtc3888 { i2c, address }
    }
    fn write_time(&mut self, time: Time) -> Result<(), esp_hal::i2c::master::Error> {
        let seconds_bcd = ((time.seconds / 10) << 4) | (time.seconds % 10);
        let minutes_bcd = ((time.minutes / 10) << 4) | (time.minutes % 10);
        let hours_bcd = ((time.hours / 10) << 4) | (time.hours % 10);

        self.i2c.lock(|i2c| {
            let mut i2c = i2c.borrow_mut();
            let i2c = i2c.as_mut().expect("I2C not initialized");

            // Write time to RTC starting at register 0x00
            i2c.write(self.address, &[0x00, seconds_bcd, minutes_bcd, hours_bcd])
        })?;
        Ok(())
    }
    fn read_time(&mut self) -> Result<Time, esp_hal::i2c::master::Error> {
        let mut buf = [0u8; 3];
        self.i2c.lock(|i2c| {
            let mut i2c = i2c.borrow_mut();
            let i2c = i2c.as_mut().expect("I2C not initialized");

            // Read time from RTC starting at register 0x00
            i2c.write_read(self.address, &[0x00], &mut buf)
        })?;

        let seconds_bcd = buf[0];
        let minutes_bcd = buf[1];
        let hours_bcd = buf[2];

        let seconds = ((seconds_bcd >> 4) * 10) + (seconds_bcd & 0x0F);
        let minutes = ((minutes_bcd >> 4) * 10) + (minutes_bcd & 0x0F);
        let hours = ((hours_bcd >> 4) * 10) + (hours_bcd & 0x0F);

        Ok(Time {
            hours,
            minutes,
            seconds,
        })
    }
}

pub struct SevenSegmentDriver<'a> {
    i2c: &'a Mutex<CriticalSectionRawMutex, RefCell<Option<I2c<'static, Blocking>>>>,
}

impl<'a> SevenSegmentDriver<'a> {
    pub fn new(
        i2c: &'a Mutex<CriticalSectionRawMutex, RefCell<Option<I2c<'static, Blocking>>>>,
    ) -> Self {
        let driver = SevenSegmentDriver { i2c };
        driver
    }
    pub fn display_digit(
        &mut self,
        digit_index: u8, // 0-3 (Digit 1, 2, 3, 4)
        value: u8,       // 0-9 (the number to show)
    ) -> Result<(), esp_hal::i2c::master::Error> {
        let digit_pins = [
            IoExpPort1::Digit1.bits(),
            IoExpPort1::Digit2.bits(),
            IoExpPort1::Digit3.bits(),
            IoExpPort1::Digit4.bits(),
        ];

        // Mask to clear all digit pins (pins 0-3)
        let all_digits_mask = 0b0000_1111;
        let mut segments = digit_to_segments(value); // No colon here

        // Set colon bit
        segments |= IoExpPort2::Dp.bits();

        self.i2c.lock(|i2c| {
            let mut i2c = i2c.borrow_mut();
            let i2c = i2c.as_mut().expect("I2C not initialized");

            // 1. Read OUT_PORT1
            let mut rbuf = [0u8];
            i2c.write_read(TCA6424_ADDR, &[OUT_PORT1], &mut rbuf)
                .unwrap();
            let current_port1 = rbuf[0];

            // 2. Blanking: Deactivate all digits (set bits 0-3 high, preserve LEDs/switches)
            let leds_state = current_port1 & !all_digits_mask;
            let blank_state = leds_state | all_digits_mask; // All digits high (off)
            i2c.write(TCA6424_ADDR, &[OUT_PORT1, blank_state]).unwrap();

            // 3. Write Segments to OUT_PORT2
            i2c.write(TCA6424_ADDR, &[OUT_PORT2, segments]).unwrap();

            // 4. Activate selected digit (set its bit low, others high)
            let new_digit_state = all_digits_mask & !digit_pins[digit_index as usize];
            let new_port1 = leds_state | new_digit_state;
            i2c.write(TCA6424_ADDR, &[OUT_PORT1, new_port1]).unwrap();
        });
        Ok(())
    }
}

fn digit_to_segments(digit: u8) -> u8 {
    let segments = match digit {
        0 => {
            IoExpPort2::SegA.bits()
                | IoExpPort2::SegB.bits()
                | IoExpPort2::SegC.bits()
                | IoExpPort2::SegD.bits()
                | IoExpPort2::SegE.bits()
                | IoExpPort2::SegF.bits()
        }
        1 => IoExpPort2::SegB.bits() | IoExpPort2::SegC.bits(),
        2 => {
            IoExpPort2::SegA.bits()
                | IoExpPort2::SegB.bits()
                | IoExpPort2::SegD.bits()
                | IoExpPort2::SegE.bits()
                | IoExpPort2::SegG.bits()
        }
        3 => {
            IoExpPort2::SegA.bits()
                | IoExpPort2::SegB.bits()
                | IoExpPort2::SegC.bits()
                | IoExpPort2::SegD.bits()
                | IoExpPort2::SegG.bits()
        }
        4 => {
            IoExpPort2::SegB.bits()
                | IoExpPort2::SegC.bits()
                | IoExpPort2::SegF.bits()
                | IoExpPort2::SegG.bits()
        }
        5 => {
            IoExpPort2::SegA.bits()
                | IoExpPort2::SegC.bits()
                | IoExpPort2::SegD.bits()
                | IoExpPort2::SegF.bits()
                | IoExpPort2::SegG.bits()
        }
        6 => {
            IoExpPort2::SegA.bits()
                | IoExpPort2::SegC.bits()
                | IoExpPort2::SegD.bits()
                | IoExpPort2::SegE.bits()
                | IoExpPort2::SegF.bits()
                | IoExpPort2::SegG.bits()
        }
        7 => IoExpPort2::SegA.bits() | IoExpPort2::SegB.bits() | IoExpPort2::SegC.bits(),
        8 => {
            IoExpPort2::SegA.bits()
                | IoExpPort2::SegB.bits()
                | IoExpPort2::SegC.bits()
                | IoExpPort2::SegD.bits()
                | IoExpPort2::SegE.bits()
                | IoExpPort2::SegF.bits()
                | IoExpPort2::SegG.bits()
        }
        9 => {
            IoExpPort2::SegA.bits()
                | IoExpPort2::SegB.bits()
                | IoExpPort2::SegC.bits()
                | IoExpPort2::SegD.bits()
                | IoExpPort2::SegF.bits()
                | IoExpPort2::SegG.bits()
        }
        _ => 0,
    };
    segments
}

pub struct ExpanderLedsDriver<'a> {
    i2c: &'a Mutex<CriticalSectionRawMutex, RefCell<Option<I2c<'static, Blocking>>>>,
}

impl<'a> ExpanderLedsDriver<'a> {
    pub fn new(
        i2c: &'a Mutex<CriticalSectionRawMutex, RefCell<Option<I2c<'static, Blocking>>>>,
    ) -> Self {
        let driver = ExpanderLedsDriver { i2c };
        driver
    }

    pub fn pm_led_on(&mut self) -> Result<(), esp_hal::i2c::master::Error> {
        self.i2c.lock(|i2c| {
            let mut i2c = i2c.borrow_mut();
            let i2c = i2c.as_mut().expect("I2C not initialized");

            // 1. READ
            let mut rbuf = [0u8];
            i2c.write_read(TCA6424_ADDR, &[OUT_PORT1], &mut rbuf)
                .unwrap();
            let current = rbuf[0];

            // 2. MODIFY
            let new = current | IoExpPort1::PmLed.bits();

            // 3. WRITE
            i2c.write(TCA6424_ADDR, &[OUT_PORT1, new])
        })?;
        Ok(())
    }

    pub fn pm_led_off(&mut self) -> Result<(), esp_hal::i2c::master::Error> {
        self.i2c.lock(|i2c| {
            let mut i2c = i2c.borrow_mut();
            let i2c = i2c.as_mut().expect("I2C not initialized");

            // 1. READ
            let mut rbuf = [0u8];
            i2c.write_read(TCA6424_ADDR, &[OUT_PORT1], &mut rbuf)
                .unwrap();
            let current = rbuf[0];

            // 2. MODIFY
            let new = current & !IoExpPort1::PmLed.bits(); // Use AND NOT to clear the bit

            // 3. WRITE
            i2c.write(TCA6424_ADDR, &[OUT_PORT1, new])
        })?;
        Ok(())
    }

    pub fn led_on(&mut self) -> Result<(), esp_hal::i2c::master::Error> {
        self.i2c.lock(|i2c| {
            let mut i2c = i2c.borrow_mut();
            let i2c = i2c.as_mut().expect("I2C not initialized");

            // 1. READ
            let mut rbuf = [0u8];
            i2c.write_read(TCA6424_ADDR, &[OUT_PORT1], &mut rbuf)
                .unwrap();
            let current = rbuf[0];

            // 2. MODIFY
            let new = current | IoExpPort1::Led.bits();

            // 3. WRITE
            i2c.write(TCA6424_ADDR, &[OUT_PORT1, new])
        })?;
        Ok(())
    }

    pub fn led_off(&mut self) -> Result<(), esp_hal::i2c::master::Error> {
        self.i2c.lock(|i2c| {
            let mut i2c = i2c.borrow_mut();
            let i2c = i2c.as_mut().expect("I2C not initialized");

            // 1. READ
            let mut rbuf = [0u8];
            i2c.write_read(TCA6424_ADDR, &[OUT_PORT1], &mut rbuf)
                .unwrap();
            let current = rbuf[0];

            // 2. MODIFY
            let new = current & !IoExpPort1::Led.bits();

            // 3. WRITE
            i2c.write(TCA6424_ADDR, &[OUT_PORT1, new])
        })?;
        Ok(())
    }
}

pub struct ExpanderInputsDriver<'a> {
    i2c: &'a Mutex<CriticalSectionRawMutex, RefCell<Option<I2c<'static, Blocking>>>>,
}

impl<'a> ExpanderInputsDriver<'a> {
    pub fn new(
        i2c: &'a Mutex<CriticalSectionRawMutex, RefCell<Option<I2c<'static, Blocking>>>>,
    ) -> Self {
        let driver = ExpanderInputsDriver { i2c };
        driver
    }

    fn init(&mut self) -> Result<(), esp_hal::i2c::master::Error> {
        self.i2c.lock(|i2c| {
            let mut i2c = i2c.borrow_mut();
            let i2c = i2c.as_mut().expect("I2C not initialized");

            // 1. Set the initial output state
            i2c.write(TCA6424_ADDR, &[OUT_PORT0, 0, 0x0F, 0])?;

            // 2. Configure the directions
            i2c.write(
                TCA6424_ADDR,
                &[CONFIG_PORT0, PORT0_DIR, PORT1_DIR, PORT2_DIR],
            )
        })?;

        Ok(())
    }

    pub fn update_events(&mut self) -> SystemEvents {
        let mut ports = [0u8; 2];
        self.i2c.lock(|i2c| {
            let mut i2c = i2c.borrow_mut();
            let i2c = i2c.as_mut().expect("I2C not initialized");
            let mut buf = [0u8];

            for (i, reg) in [IN_PORT0, IN_PORT1].iter().enumerate() {
                i2c.write_read(TCA6424_ADDR, &[*reg], &mut buf).unwrap();
                ports[i] = buf[0];
            }
        });

        let events = SystemEvents {
            time_but: if ports[0] & IoExpPort0::TimeButton.bits() == 0 {
                Button::Pressed
            } else {
                Button::Released
            },
            alarm_but: if ports[0] & IoExpPort0::AlarmButton.bits() == 0 {
                Button::Pressed
            } else {
                Button::Released
            },
            hour_but: if ports[0] & IoExpPort0::HourButton.bits() == 0 {
                Button::Pressed
            } else {
                Button::Released
            },
            min_but: if ports[0] & IoExpPort0::MinuteButton.bits() == 0 {
                Button::Pressed
            } else {
                Button::Released
            },
            // Snooze button handled in separate GPIO driver
            snooze_but: Button::Released,
            alarm_sw: if ports[0] & IoExpPort0::AlarmSwitchOn.bits() == 0 {
                AlarmSwitch::On
            } else {
                AlarmSwitch::Off
            },
            fmt_sw: if ports[1] & IoExpPort1::FormatSwitch12.bits() == 0 {
                FormatSwitch::H12
            } else {
                FormatSwitch::H24
            },
        };
        events
    }
}

#[esp_rtos::main]
async fn main(spawner: Spawner) {
    esp_println::logger::init_logger_from_env();
    let peripherals = esp_hal::init(esp_hal::Config::default());

    let sw_int = SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);
    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_rtos::start(timg0.timer0, sw_int.software_interrupt0);

    //----------- Device Driver Configuration -----------//
    // Alarm LED Output Driver
    let mut alarm_led = Output::new(peripherals.GPIO3, Level::Low, OutputConfig::default());

    // Snooze Button Input Driver
    let snooze_button = Input::new(
        peripherals.GPIO5,
        InputConfig::default().with_pull(Pull::Up),
    );

    // RTC and I/O Expander I2C Bus Driver
    // I/O Expander Contains Buttons, Switches and 7-Segment Display

    // Master I2C Configuration
    // Bus Addresses:
    // RTC -> 0x68
    // I/O Expander -> 0x22
    // Pins:
    // SCL -> GPIO7
    // SDA -> GPIO6
    let i2c0 = I2c::new(
        peripherals.I2C0,
        I2cConfig::default().with_frequency(Rate::from_khz(100)),
    )
    .unwrap()
    .with_scl(peripherals.GPIO7)
    .with_sda(peripherals.GPIO6);

    // Move I2C to shared state to share among tasks
    SHARED_I2C.lock(|i2c| i2c.borrow_mut().replace(i2c0));

    // RTC Driver
    let mut rtc = Rtc3888::new(&SHARED_I2C, RTC_ADDR);
    // Read RTC Time from TimeKeeper on Startup
    SHARED_TIMEKEEPER.lock(|rc| {
        let mut tk = rc.borrow_mut();
        if let Ok(current_time) = rtc.read_time() {
            tk.clocktime = current_time;
        }
    });
    println!(
        "RTC Initialized: {}:{}",
        rtc.read_time().unwrap().hours,
        rtc.read_time().unwrap().minutes
    );

    // Buzzer Output PWM Driver
    // Configure PWM
    // Buzzer connected to GPIO2
    // Create LEDC instance with low speed global clock
    let mut buzz = Ledc::new(peripherals.LEDC);
    buzz.set_global_slow_clock(LSGlobalClkSource::APBClk);

    // Configure LEDC timer
    let mut timer = buzz.timer::<LowSpeed>(timer::Number::Timer0);

    timer
        .configure(timer::config::Config {
            duty: timer::config::Duty::Duty14Bit,
            clock_source: timer::LSClockSource::APBClk,
            frequency: Rate::from_hz(2700u32),
        })
        .unwrap();

    // Configure LEDC Channel Attaching Timer and Pin
    let mut buzz_channel = buzz.channel(channel::Number::Channel0, peripherals.GPIO4);

    buzz_channel
        .configure(channel::config::Config {
            timer: &timer,
            duty_pct: 0,
            drive_mode: esp_hal::gpio::DriveMode::PushPull,
        })
        .unwrap();

    //----------- Task Spawning -----------//
    // Display Update Task
    // No need to pass anything as I2C is already a shared resource
    spawner.spawn(display_update()).ok();
    // Timekeeper Task
    // No need to pass anything as TimeKeeper struct is already a shared resource
    spawner.spawn(timekeeper_task()).ok();
    // Event Polling Task
    spawner.spawn(event_handler_loop(snooze_button)).ok();

    // Variable to hold current system state
    let mut state = State::Unarmed;

    loop {
        match state {
            State::Unarmed => {
                alarm_led.set_low();
                buzz_channel.set_duty(0u8).unwrap();
                let time = SHARED_TIMEKEEPER.lock(|rc| {
                    let mut tk = rc.borrow_mut();
                    tk.show_clock_time();
                    tk.clocktime
                });
                rtc.write_time(time).unwrap();
                println!(
                    "RTC Updated: {}:{}",
                    rtc.read_time().unwrap().hours,
                    rtc.read_time().unwrap().minutes
                );
                state = unarmed_state().await;
            }
            State::Armed => {
                alarm_led.set_high();
                state = armed_state().await;
            }
            State::SetTime => {
                SHARED_TIMEKEEPER.lock(|rc| {
                    let mut tk = rc.borrow_mut();
                    tk.show_clock_time();
                });
                state = set_time_state().await;
            }
            State::SetAlarm => {
                SHARED_TIMEKEEPER.lock(|rc| {
                    let mut tk = rc.borrow_mut();
                    tk.show_alarm_time();
                });
                state = set_alarm_state().await;
            }
            State::Snoozing => {
                buzz_channel.set_duty(0u8).unwrap();
                state = snoozing_state().await;
            }
            State::Alarming => {
                buzz_channel.set_duty(50u8).unwrap();
                state = alarming_state().await;
            }
        }
    }
}

#[embassy_executor::task]
async fn event_handler_loop(snooze_button: Input<'static>) {
    // Instantiate and Initialize Expander Input Driver
    let mut expander_inputs = ExpanderInputsDriver::new(&SHARED_I2C);
    expander_inputs.init().unwrap();

    loop {
        // Read I/O Expander Inputs
        let mut io_events = expander_inputs.update_events();
        // Read Snooze Button State
        if snooze_button.is_low() {
            io_events.snooze_but = Button::Pressed;
        } else {
            io_events.snooze_but = Button::Released;
        }

        // Update Shared Events in Global Context
        SHARED_SYS_EVENTS.lock(|events_ref| {
            let mut events = events_ref.borrow_mut();
            *events = Some(io_events);
        });

        // Poll Inputs every 5 ms
        Timer::after(Duration::from_millis(5)).await;
    }
}

#[embassy_executor::task]
async fn display_update() {
    // Instantiate 7-Segment Display Driver
    let mut seg_driver = SevenSegmentDriver::new(&SHARED_I2C);
    // PM LED Driver
    let mut expander_leds = ExpanderLedsDriver::new(&SHARED_I2C);

    loop {
        // Read current display time from TimeKeeper
        let (hours, minutes) = SHARED_TIMEKEEPER.lock(|rc| {
            let tk = rc.borrow();
            match tk.display_mode {
                DisplayMode::ShowClock => (tk.clocktime.hours, tk.clocktime.minutes),
                DisplayMode::ShowAlarm => (tk.alarmtime.hours, tk.alarmtime.minutes),
            }
        });

        // Read Current Format Switch State
        let mut display_hours = hours;
        let events = SHARED_SYS_EVENTS.lock(|events_ref| {
            let events = events_ref.borrow();
            *events
        });
        if let Some(event) = events {
            match event.fmt_sw {
                FormatSwitch::H24 => {
                    // 24-hour format
                    expander_leds.pm_led_off().unwrap();
                }
                FormatSwitch::H12 => {
                    // 12-hour format
                    // Convert to 12-hour format
                    if hours == 0 {
                        display_hours = 12; // Midnight
                        expander_leds.pm_led_off().unwrap();
                    } else if hours == 12 {
                        display_hours = 12; // Noon
                        expander_leds.pm_led_on().unwrap();
                    } else if hours > 12 {
                        display_hours = hours - 12;
                        // Turn on PM LED if hours >= 12
                        expander_leds.pm_led_on().unwrap();
                    } else {
                        // AM time
                        expander_leds.pm_led_off().unwrap();
                    }
                }
            }
        }

        // Create an array of the 4 digits to display
        let digits = [
            display_hours / 10,
            display_hours % 10,
            minutes / 10,
            minutes % 10,
        ];
        for (digit_idx, digit_val) in digits.iter().enumerate() {
            // Update One digit at a time
            seg_driver
                .display_digit(digit_idx as u8, *digit_val)
                .unwrap();
            // Wait 5ms. This gives a ~50Hz refresh rate (4 digits * 5ms = 20ms)
            Timer::after(Duration::from_millis(5)).await;
        }
    }
}

#[embassy_executor::task]
async fn timekeeper_task() {
    loop {
        SHARED_TIMEKEEPER.lock(|rc| {
            let mut tk = rc.borrow_mut();
            tk.tick();
            // Debug Print Current Time
            // Update Display Time to Clock Time
            // tk.show_clock_time();
            // println!(
            //     "Time: {:02}:{:02}:{:02}",
            //     tk.clocktime.hours, tk.clocktime.minutes, tk.clocktime.seconds
            // );
        });
        Timer::after(Duration::from_secs(1)).await;
    }
}

async fn unarmed_state() -> State {
    loop {
        let events = SHARED_SYS_EVENTS.lock(|events_ref| {
            let events = events_ref.borrow();
            *events
        });
        if let Some(event) = events {
            match event {
                SystemEvents {
                    alarm_sw: AlarmSwitch::On,
                    ..
                } => {
                    return State::Armed;
                }
                SystemEvents {
                    alarm_but: Button::Pressed,
                    ..
                } => {
                    return State::SetAlarm;
                }
                SystemEvents {
                    time_but: Button::Pressed,
                    ..
                } => {
                    return State::SetTime;
                }
                _ => {}
            }
        }
        // Evaluate State every 1 ms
        Timer::after(Duration::from_millis(100)).await;
    }
}

async fn armed_state() -> State {
    loop {
        // Get Events
        let events = SHARED_SYS_EVENTS.lock(|events_ref| {
            let events = events_ref.borrow();
            *events
        });
        // Check if current time matches alarm time
        let alarm_match = SHARED_TIMEKEEPER.lock(|rc| {
            let tk = rc.borrow();
            tk.alarm_time_match()
        });
        if alarm_match {
            return State::Alarming;
        }
        if let Some(event) = events {
            match event {
                SystemEvents {
                    alarm_sw: AlarmSwitch::Off,
                    ..
                } => {
                    return State::Unarmed;
                }
                SystemEvents {
                    alarm_but: Button::Pressed,
                    ..
                } => {
                    return State::SetAlarm;
                }
                SystemEvents {
                    time_but: Button::Pressed,
                    ..
                } => {
                    return State::SetTime;
                }
                _ => {}
            }
        }
        // Evaluate State every 1 ms
        Timer::after(Duration::from_millis(100)).await;
    }
}

async fn set_time_state() -> State {
    loop {
        let events = SHARED_SYS_EVENTS.lock(|events_ref| {
            let events = events_ref.borrow();
            *events
        });
        if let Some(event) = events {
            match event {
                SystemEvents {
                    hour_but: Button::Pressed,
                    ..
                } => {
                    SHARED_TIMEKEEPER.lock(|rc| {
                        let mut tk = rc.borrow_mut();
                        tk.increment_time_hours();
                    });
                }
                SystemEvents {
                    min_but: Button::Pressed,
                    ..
                } => {
                    SHARED_TIMEKEEPER.lock(|rc| {
                        let mut tk = rc.borrow_mut();
                        tk.increment_time_minutes();
                    });
                }
                SystemEvents {
                    time_but: Button::Released,
                    ..
                } => {
                    return State::Unarmed;
                }
                _ => {}
            }
        }
        // Poll for events every 80 ms
        // This also acts as a debounce delay for buttons
        Timer::after(Duration::from_millis(100)).await;
    }
}

async fn set_alarm_state() -> State {
    loop {
        let events = SHARED_SYS_EVENTS.lock(|events_ref| {
            let events = events_ref.borrow();
            *events
        });
        if let Some(event) = events {
            match event {
                SystemEvents {
                    hour_but: Button::Pressed,
                    ..
                } => {
                    SHARED_TIMEKEEPER.lock(|rc| {
                        let mut tk = rc.borrow_mut();
                        tk.increment_alarm_hours();
                    });
                }
                SystemEvents {
                    min_but: Button::Pressed,
                    ..
                } => {
                    SHARED_TIMEKEEPER.lock(|rc| {
                        let mut tk = rc.borrow_mut();
                        tk.increment_alarm_minutes();
                    });
                }
                SystemEvents {
                    alarm_but: Button::Released,
                    ..
                } => {
                    return State::Unarmed;
                }
                _ => {}
            }
        }
        // Evaluate State every 1 ms
        Timer::after(Duration::from_millis(100)).await;
    }
}

async fn alarming_state() -> State {
    loop {
        let events = SHARED_SYS_EVENTS.lock(|events_ref| {
            let events = events_ref.borrow();
            *events
        });
        if let Some(event) = events {
            match event {
                SystemEvents {
                    snooze_but: Button::Pressed,
                    ..
                } => {
                    return State::Snoozing;
                }
                SystemEvents {
                    alarm_sw: AlarmSwitch::Off,
                    ..
                } => {
                    return State::Unarmed;
                }
                _ => {}
            }
        }
        // Evaluate State every 1 ms
        Timer::after(Duration::from_millis(100)).await;
    }
}

async fn snoozing_state() -> State {
    let snoozing_start_instant = Instant::now();
    loop {
        let events = SHARED_SYS_EVENTS.lock(|events_ref| {
            let events = events_ref.borrow();
            *events
        });
        if let Some(event) = events {
            match event {
                SystemEvents {
                    alarm_sw: AlarmSwitch::Off,
                    ..
                } => {
                    return State::Unarmed;
                }
                _ => {}
            }
        }
        if snoozing_start_instant.elapsed() >= Duration::from_secs(5 * 60) {
            return State::Alarming;
        }
        // Evaluate State every 1 ms
        Timer::after(Duration::from_millis(100)).await;
    }
}
