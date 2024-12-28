#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    gpio::{Input, Level, Output, Pin, Pull},
    prelude::*,
    timer::timg::TimerGroup,
};
use log::info;

extern crate alloc;

#[entry]
fn main() -> ! {
    let peripherals = esp_hal::init({
        let mut config = esp_hal::Config::default();
        config.cpu_clock = CpuClock::max();
        config
    });

    esp_println::logger::init_logger_from_env();
    esp_alloc::heap_allocator!(72 * 1024);

    let mut led = Output::new(peripherals.GPIO8.degrade(), Level::Low);
    let delay = Delay::new();
    loop {
        info!("Hi!");
        delay.delay(500.millis());
        led.toggle();
    }
}
