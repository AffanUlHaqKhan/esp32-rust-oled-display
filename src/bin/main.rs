#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use esp_hal::clock::CpuClock;
use esp_hal::i2c::master::{Config as I2cConfig, I2c};
use esp_hal::main;
use esp_hal::time::{Duration, Instant, Rate};
use log::error;

use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    text::Text,
};
use ssd1306::mode::DisplayConfig;
use ssd1306::rotation::DisplayRotation;
use ssd1306::size::DisplaySize128x64;
use ssd1306::{I2CDisplayInterface, Ssd1306};

#[panic_handler]
fn panic(panic_info: &core::panic::PanicInfo) -> ! {
    error!("{}", panic_info);
    loop {}
}

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[main]
fn main() -> ! {
    // generator version: 1.3.0
    // generator parameters: --chip esp32c6 -o log -o vscode

    esp_println::logger::init_logger_from_env();

    let peripherals = esp_hal::init(esp_hal::Config::default().with_cpu_clock(CpuClock::max()));

    // Set up the pin out for OLED against ESP32-C6
    let config = I2cConfig::default().with_frequency(Rate::from_khz(400));
    let i2c = I2c::new(peripherals.I2C0, config)
        .expect("I2C init error")
        .with_sda(peripherals.GPIO21)
        .with_scl(peripherals.GPIO22);

    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();

    display.init().expect("Display init error");

    let style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);
    Text::new("Hello Stranger !!!", Point::new(10, 32), style)
        .draw(&mut display)
        .expect("Draw error");

    display.flush().expect("Flush error");

    loop {
        let delay_start = Instant::now();
        while delay_start.elapsed() < Duration::from_millis(1000) {}
    }
}
