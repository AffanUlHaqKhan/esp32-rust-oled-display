use esp_hal::gpio::{Input, InputConfig, Level, Output, OutputConfig, Pull};
use esp_hal::time::{Duration, Instant};

// Longest echo pulse we accept (~4 m round trip).
const ECHO_TIMEOUT: Duration = Duration::from_micros(38_000);
/// How long to wait for the burst to even start.
const ECHO_START_TIMEOUT: Duration = Duration::from_micros(5_000);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SonarError {
    /// ECHO never went high — check wiring / 5 V supply.
    NoResponse,
    /// ECHO stayed high past max range — nothing in front of it.
    OutOfRange,
}

pub struct Sonar<'d> {
    trig: Output<'d>,
    echo: Input<'d>,
}

fn delay_micros(us: u64) {
    let start = Instant::now();
    while start.elapsed() < Duration::from_micros(us) {}
}


impl<'d> Sonar<'d> {

    pub fn new(
        trig_pin: impl esp_hal::gpio::OutputPin + 'd,
        echo_pin: impl esp_hal::gpio::InputPin + 'd,
    ) -> Self {
        Self{
            trig: Output::new(trig_pin, Level::Low, OutputConfig::default()),
            echo: Input::new(echo_pin, InputConfig::default().with_pull(Pull::None))
        }
    }


    pub fn measure_mm(&mut self) -> Result<u32, SonarError> {
        self.trig.set_low();
        delay_micros(2);

        self.trig.set_high();
        delay_micros(10);
        self.trig.set_low();

        self.wait_for_echo_start()?;
        let pulse = self.measure_echo_high()?;
        Ok(Self::pulse_to_mm(pulse.as_micros()))
    }


    fn wait_for_echo_start(&mut self) -> Result<(), SonarError> {
        let start = Instant::now();
        while !self.echo.is_high(){
            if start.elapsed() > ECHO_START_TIMEOUT { return Err(SonarError::NoResponse) }
        }
        Ok(())
    }


    fn measure_echo_high(&mut self) -> Result<Duration, SonarError> {
        let rise = Instant::now();
        while self.echo.is_high() {
               if rise.elapsed() > ECHO_TIMEOUT { return Err(SonarError::OutOfRange) }
           }
           Ok(rise.elapsed())
    }

    fn pulse_to_mm(pulse_us: u64) -> u32 {
        // Multiply before dividing: `343 / 2000` alone truncates to 0 in integer math.
        let mm = pulse_us * 343 / 2000;
        mm as u32
    }
}