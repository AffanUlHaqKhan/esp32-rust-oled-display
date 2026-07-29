//! SSD1306 128x64 OLED driver wrapper.
//!
//! Keeps display bring-up and drawing behind a small API so `main` only wires up
//! peripherals and calls into here. Generic over any `embedded-hal` I2C bus, so it is
//! not tied to a particular chip.

use display_interface::DisplayError;
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    text::Text,
};
use embedded_hal::i2c::I2c;
use ssd1306::mode::{BufferedGraphicsMode, DisplayConfig};
use ssd1306::prelude::I2CInterface;
use ssd1306::rotation::DisplayRotation;
use ssd1306::size::DisplaySize128x64;
use ssd1306::{I2CDisplayInterface, Ssd1306};

/// The concrete display type: 128x64 panel on I2C, in buffered graphics mode.
pub type Display<I2C> =
    Ssd1306<I2CInterface<I2C>, DisplaySize128x64, BufferedGraphicsMode<DisplaySize128x64>>;

/// An initialized SSD1306 panel.
pub struct Oled<I2C> {
    display: Display<I2C>,
}

impl<I2C> Oled<I2C>
where
    I2C: I2c,
{
    /// Takes ownership of the I2C bus, sets up the panel at the default address (0x3C)
    /// and turns it on.
    pub fn new(i2c: I2C) -> Result<Self, DisplayError> {
        let interface = I2CDisplayInterface::new(i2c);
        let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
            .into_buffered_graphics_mode();

        display.init()?;

        Ok(Self { display })
    }

    /// Draws a single line of text at `position` and pushes it to the panel.
    pub fn show_text(&mut self, text: &str, position: Point) -> Result<(), DisplayError> {
        let style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);
        Text::new(text, position, style).draw(&mut self.display)?;
        self.display.flush()
    }

    /// Blanks the panel.
    pub fn clear(&mut self) -> Result<(), DisplayError> {
        self.display.clear_buffer();
        self.display.flush()
    }

    /// Escape hatch for drawing anything `embedded-graphics` can render. The caller is
    /// responsible for calling [`Oled::flush`] afterwards.
    pub fn display(&mut self) -> &mut Display<I2C> {
        &mut self.display
    }

    /// Pushes the frame buffer to the panel.
    pub fn flush(&mut self) -> Result<(), DisplayError> {
        self.display.flush()
    }
}
