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

use core::fmt::Write;
use embedded_graphics::prelude::Point;
use heapless::String;

use esp32_rust_oled_display::oled::Oled;
use esp32_rust_oled_display::sonar::Sonar;

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

    let mut oled = Oled::new(i2c).expect("Display init error");
    let mut sonar = Sonar::new(peripherals.GPIO6, peripherals.GPIO7);

    // oled.show_text("Hello Strangerssss !!!", Point::new(10, 32))
    //     .expect("Draw error");

    loop {
        let mut line: String<24> = String::new();
        match sonar.measure_mm() {
            Ok(mm) => {
                let _ = write!(line, "Value: {} mm", mm);
            }
            Err(e) => {
                error!("sonar: {:?}", e);
                let _ = write!(line, "Value: ---");
            }
        }

        oled.clear().expect("Clear error");
        oled.show_text(&line, Point::new(10, 32))
            .expect("Draw error");
        // delay.delay_millis(100);
        let delay_start = Instant::now();
        while delay_start.elapsed() < Duration::from_millis(100) {}
    }
}
