#![no_std]
#![feature(type_alias_impl_trait)]
#![feature(trait_alias)]
#![feature(decl_macro)]

use esp_backtrace as _;

pub macro parallel_lcd($pins:expr) {{
    let mut lcd = ::st7920::esp::parallel_4bit(
        $pins.gpio32,
        $pins.gpio33,
        $pins.gpio25,
        $pins.gpio26,
        $pins.gpio27,
        $pins.gpio14,
        $pins.gpio13,
    );
    ::st7920::Init::init(&mut lcd).unwrap();
    ::log::info!("Parallel LCD initialized...");

    lcd
}}

pub macro serial_lcd($peripherals:expr, $pins:expr) {{
    let mut lcd = ::st7920::esp::serial($peripherals.SPI2, $pins.gpio16, $pins.gpio4, $pins.gpio17);
    ::st7920::Init::init(&mut lcd).unwrap();
    ::log::info!("Serial LCD initialized...");

    lcd
}}

pub macro setup($peripherals:ident, $io:ident, $rng:ident) {
    let $peripherals = ::esp_hal::init(esp_hal::Config::default());
    let $io = ::esp_hal::gpio::Io::new($peripherals.GPIO, $peripherals.IO_MUX);
    let $rng = ::esp_hal::rng::Rng::new($peripherals.RNG);
}
