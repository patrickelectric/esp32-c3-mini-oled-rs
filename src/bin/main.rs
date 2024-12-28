#![no_std]
#![no_main]

use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Line, PrimitiveStyle, PrimitiveStyleBuilder, Rectangle},
};
use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    gpio::{Level, Output, Pin},
    i2c,
    prelude::*,
};
use log::info;
use sh1106::{prelude::*, Builder};

extern crate alloc;

#[entry]
fn main() -> ! {
    let peripherals = esp_hal::init({
        let mut config = esp_hal::Config::default();
        config.cpu_clock = CpuClock::max();
        config
    });

    esp_println::logger::init_logger_from_env();
    esp_alloc::heap_allocator!(102 * 1024);

    // Display
    let i2c = i2c::master::I2c::new(
        peripherals.I2C0,
        i2c::master::Config {
            frequency: 400.kHz(),
            timeout: Some(10),
        },
    )
    .with_scl(peripherals.GPIO6.degrade())
    .with_sda(peripherals.GPIO5.degrade());
    // There is no 72x40 constructor, the screen is mapped in the middle of the 128x64 pixel buffer of the SH1106 controller
    let mut display: GraphicsMode<_> = Builder::new()
        .with_size(DisplaySize::Display128x64)
        .connect_i2c(i2c)
        .into();
    display.init().unwrap();

    let style = PrimitiveStyleBuilder::new()
        .stroke_width(1)
        .stroke_color(BinaryColor::On)
        .build();

    // Correct display size
    let real_display_size = Size::new(72, 40); // real size
    let real_display_size_as_point = Point::new(
        real_display_size.width as i32,
        real_display_size.height as i32,
    ) - Point::new(1, 1);
    let offset = (display.size() - real_display_size) / 2;
    let offset = Point::new(offset.width as i32, offset.height as i32);
    let final_point = offset + real_display_size_as_point;
    Rectangle::with_corners(offset, final_point)
        .into_styled(style)
        .draw(&mut display)
        .unwrap();
    Line::new(offset, final_point)
        .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
        .draw(&mut display)
        .unwrap();

    Line::new(
        Point::new(final_point.x, offset.y),
        Point::new(offset.x, final_point.y),
    )
    .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
    .draw(&mut display)
    .unwrap();
    display.flush().unwrap();

    let mut led = Output::new(peripherals.GPIO8.degrade(), Level::Low);
    let delay = Delay::new();
    loop {
        info!("Hi!");
        delay.delay(1000.millis());
        led.toggle();
    }
}
