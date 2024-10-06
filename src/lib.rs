#![no_std]
#![feature(type_alias_impl_trait)]

use esp_hal::{delay::Delay, gpio::Io, rng::Rng};

use st7920::command::infallible::*;
pub type Driver = st7920::Driver<impl Execute + ExecuteExt + ExecuteRead, impl st7920::hal::Timer>;

pub fn setup() -> (Driver, Delay, Rng) {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    let delay = Delay::new();
    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);
    let rng = Rng::new(peripherals.RNG);

    esp_println::logger::init_logger_from_env();

    let mut lcd = st7920::Driver {
        bus: st7920::esp::parallel_4bit(
            io.pins.gpio32,
            io.pins.gpio33,
            io.pins.gpio25,
            io.pins.gpio26,
            io.pins.gpio27,
            io.pins.gpio14,
            io.pins.gpio13,
        ),
        timer: st7920::esp::now(),
    };

    lcd.init().unwrap();

    (lcd, delay, rng)
}
