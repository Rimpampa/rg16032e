use super::Output;
use embedded_hal::spi::SpiBus;
use fugit::RateExtU32;
use st7920::serial::Interface;
use stm32f4xx_hal::{gpio, rcc, spi::*};

pub fn new<SPI: Instance<Miso: From<gpio::NoPin>>, const NUM: usize>(
    spi: SPI,
    mosi: impl Into<SPI::Mosi>,
    sck: impl Into<SPI::Sck>,
    cs: [impl Into<Output>; NUM],
    clocks: &rcc::Clocks,
) -> Interface<impl SpiBus, Output, NUM> {
    let mode = Mode {
        polarity: Polarity::IdleHigh,
        phase: Phase::CaptureOnSecondTransition,
    };
    Interface::new(
        Spi::new(spi, (sck, NoMiso::new(), mosi), mode, 1.MHz(), clocks),
        cs.map(Into::into),
    )
}
