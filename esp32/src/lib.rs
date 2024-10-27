#![no_std]
#![feature(type_alias_impl_trait)]
#![feature(trait_alias)]
#![feature(decl_macro)]

use esp_backtrace as _;

#[cfg(all(feature = "parallel", not(feature = "two-displays")))]
pub macro lcd($peripherals:expr, $pins:expr) {{
    let mut lcd = ::st7920::esp::parallel_4bit(
        $pins.gpio32,
        $pins.gpio33,
        [$pins.gpio25],
        $pins.gpio26,
        $pins.gpio27,
        $pins.gpio14,
        $pins.gpio13,
    );
    ::st7920::Init::init(&mut lcd).unwrap();
    ::log::info!("Parallel LCD initialized...");

    lcd
}}

#[cfg(all(feature = "serial", not(feature = "two-displays")))]
pub macro lcd($peripherals:expr, $pins:expr) {{
    let mut lcd = ::st7920::esp::serial(
        $peripherals.SPI2,
        $pins.gpio26,
        $pins.gpio27,
        [$pins.gpio14]
    );
    ::st7920::Init::init(&mut lcd).unwrap();
    ::log::info!("Serial LCD initialized...");

    lcd
}}

#[cfg(all(feature = "serial", feature = "two-displays"))]
pub macro lcd($peripherals:expr, $pins:expr) {{
    let mut lcd = ::st7920::esp::serial(
        $peripherals.SPI2,
        $pins.gpio26,
        $pins.gpio27,
        [
            ::esp_hal::gpio::Pin::degrade($pins.gpio14),
            ::esp_hal::gpio::Pin::degrade($pins.gpio32),
        ]
    );
    ::st7920::Init::init(&mut ::st7920::SharedBus::get(&mut lcd, 0).unwrap()).unwrap();
    ::st7920::Init::init(&mut ::st7920::SharedBus::get(&mut lcd, 1).unwrap()).unwrap();
    ::log::info!("Serial LCD initialized...");

    lcd
}}

pub macro setup($peripherals:ident, $io:ident, $rng:ident) {
    let $peripherals = ::esp_hal::init(esp_hal::Config::default());
    let $io = ::esp_hal::gpio::Io::new($peripherals.GPIO, $peripherals.IO_MUX);
    let $rng = ::esp_hal::rng::Rng::new($peripherals.RNG);
}
