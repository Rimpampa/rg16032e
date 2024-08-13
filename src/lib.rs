#![no_std]

use esp_hal::{
    clock::ClockControl, delay::Delay, gpio::Io, peripherals::Peripherals, rng::Rng,
    system::SystemControl,
};
use st7920::Driver;

pub fn setup() -> (Driver, Delay, Rng) {
    let peripherals = Peripherals::take();
    let system = SystemControl::new(peripherals.SYSTEM);

    let clocks = ClockControl::max(system.clock_control).freeze();
    let delay = Delay::new(&clocks);
    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);
    let rng = Rng::new(peripherals.RNG);

    esp_println::logger::init_logger_from_env();

    let lcd = Driver::setup(
        io.pins.gpio32,
        io.pins.gpio33,
        io.pins.gpio25,
        ::st7920::bus::new!(
            db4: io.pins.gpio26,
            db5: io.pins.gpio27,
            db6: io.pins.gpio14,
            db7: io.pins.gpio13,
        ),
    );

    (lcd, delay, rng)
}
