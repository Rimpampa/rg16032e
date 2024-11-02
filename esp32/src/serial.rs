use super::Out;
use esp_hal::{gpio::*, peripheral::Peripheral, spi::master::*, spi::*};
use fugit::RateExtU32;
use st7920::serial::Interface;

pub fn new<'a, I: Instance + 'a, const NUM: usize>(
    spi: impl Peripheral<P = I> + 'a,
    mosi: impl Peripheral<P: PeripheralOutput> + 'a,
    sck: impl Peripheral<P: PeripheralOutput> + 'a,
    cs: [impl Out + 'a; NUM],
) -> Interface<Spi<'a, I, FullDuplexMode>, Output<'a>, NUM> {
    Interface::new(
        Spi::new(spi, 530.kHz(), SpiMode::Mode0).with_pins(sck, mosi, NoPin, NoPin),
        cs.map(|cs| Output::new(cs, Level::Low)),
    )
}
