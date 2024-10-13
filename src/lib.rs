#![no_std]
#![feature(type_alias_impl_trait)]

use core::{convert::Infallible, fmt::Debug};

use either::Either;
use esp_hal::{delay::Delay, gpio::Io, rng::Rng};

use st7920::{ext, infallible, Command, Execute, Init};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

pub struct Pair<A, B>(pub A, pub B);

impl<A: Init, B: Init> Init for Pair<A, B> {
    type Error = Either<A::Error, B::Error>;

    fn init(&mut self) -> Result<(), Self::Error> {
        self.0.init().map_err(Either::Left)?;
        self.1.init().map_err(Either::Right)?;
        Ok(())
    }
}

impl<A: Execute, B: Execute> Execute for Pair<A, B> {
    type Error = Either<A::Error, B::Error>;

    fn execute(&mut self, command: Command) -> Result<(), Self::Error> {
        self.0.execute(command).map_err(Either::Left)?;
        self.1.execute(command).map_err(Either::Right)?;
        Ok(())
    }
}

impl<A: ext::Execute, B: ext::Execute> ext::Execute for Pair<A, B> {
    fn execute_ext(&mut self, command: ext::Command) -> Result<(), Self::Error> {
        self.0.execute_ext(command).map_err(Either::Left)?;
        self.1.execute_ext(command).map_err(Either::Right)?;
        Ok(())
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

pub type Parallel =
    impl Init<Error = Infallible> + infallible::ExecuteExt + infallible::ExecuteRead;
pub type Serial = impl Init<Error: Debug> + ext::Execute<Error: Debug>;

pub fn setup() -> (Parallel, Serial, Delay, Rng) {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    let delay = Delay::new();
    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);
    let rng = Rng::new(peripherals.RNG);

    esp_println::logger::init_logger_from_env();

    let mut lcd_0 = st7920::esp::parallel_4bit(
        io.pins.gpio32,
        io.pins.gpio33,
        io.pins.gpio25,
        io.pins.gpio26,
        io.pins.gpio27,
        io.pins.gpio14,
        io.pins.gpio13,
    );
    lcd_0.init().unwrap();
    log::info!("LCD0 initialized...");

    let mut lcd_1 = st7920::esp::serial(peripherals.SPI2, io.pins.gpio17, io.pins.gpio16);
    lcd_1.init().unwrap();

    log::info!("LCD1 initialized...");

    (lcd_0, lcd_1, delay, rng)
}
