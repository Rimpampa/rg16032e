use embedded_hal::digital::{ErrorType, OutputPin};
use embedded_hal::spi::SpiBus;
use fugit::RateExtU32;

use stm32h7xx_hal::gpio::{self, ErasedPin, PushPull};
use stm32h7xx_hal::pac::{TIM2, TIM5};
use stm32h7xx_hal::{rcc::CoreClocks, timer};

use stm32h7xx_hal::spi::{self, Mode, NoMiso, Phase, Polarity};

use crate::hal::Timer;
use crate::serial;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

impl Timer for (&timer::Timer<TIM2>, u32) {
    fn program(&mut self, duration: u32) {
        self.1 = self.0.counter() + duration;
    }

    fn expired(&mut self) -> bool {
        self.0.counter() > self.1
    }
}

impl Timer for (&timer::Timer<TIM5>, u32) {
    fn program(&mut self, duration: u32) {
        self.1 = self.0.counter() + duration;
    }

    fn expired(&mut self) -> bool {
        self.0.counter() > self.1
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

type Output = ErasedPin<gpio::Output<PushPull>>;

impl ErrorType for SpiWrap<Output> {
    type Error = <Output as embedded_hal_v1::digital::v2::OutputPin>::Error;
}

impl OutputPin for SpiWrap<Output> {
    fn set_low(&mut self) -> Result<(), Self::Error> {
        embedded_hal_v1::digital::v2::OutputPin::set_low(&mut self.0)
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        embedded_hal_v1::digital::v2::OutputPin::set_high(&mut self.0)
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[derive(Debug)]
pub struct SpiWrap<S>(S);

impl embedded_hal::spi::Error for SpiWrap<spi::Error> {
    fn kind(&self) -> embedded_hal::spi::ErrorKind {
        match self.0 {
            spi::Error::Overrun => embedded_hal::spi::ErrorKind::Overrun,
            spi::Error::ModeFault => embedded_hal::spi::ErrorKind::ModeFault,
            _ => embedded_hal::spi::ErrorKind::Other,
        }
    }
} 

impl<S: embedded_hal_v1::spi::FullDuplex<u8, Error = spi::Error>> embedded_hal::spi::ErrorType for SpiWrap<S> {
    type Error = SpiWrap<spi::Error>;
}

impl<S: embedded_hal_v1::spi::FullDuplex<u8, Error = spi::Error>> SpiBus for SpiWrap<S> {
    fn read(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        for word in words {
            *word = nb::block!(self.0.read()).map_err(SpiWrap)?;
        }
        Ok(())
    }

    fn write(&mut self, words: &[u8]) -> Result<(), Self::Error> {
        for word in words {
            nb::block!(self.0.send(*word)).map_err(SpiWrap)?;
        }
        Ok(())
    }

    fn transfer(&mut self, read: &mut [u8], write: &[u8]) -> Result<(), Self::Error> {
        let mut read = read.iter_mut();
        let mut write = write.iter();

        loop {
            let write = write.next();
            let end = write.is_none();
            let write = write.unwrap_or(&0x00);
            nb::block!(self.0.send(*write)).map_err(SpiWrap)?;
            match read.next() {
                Some(read) => *read = nb::block!(self.0.read()).map_err(SpiWrap)?,
                None if end => break,
                None => (),
            }
        }
        Ok(())
    }

    fn transfer_in_place(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        for word in words {
            nb::block!(self.0.send(*word)).map_err(SpiWrap)?;
            *word = nb::block!(self.0.read()).map_err(SpiWrap)?;
        }
        Ok(())
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

pub fn serial<SPI: spi::SpiExt<SPI, u8>, T: Timer, const NUM: usize>(
    spi: SPI,
    mosi: impl spi::PinMosi<SPI>,
    sck: impl spi::PinSck<SPI>,
    cs: [impl Into<Output>; NUM],
    prec: SPI::Rec,
    clocks: &CoreClocks,
    timer: T,
) -> serial::Interface<impl SpiBus, T, SpiWrap<Output>, NUM>
where
    NoMiso: spi::PinMiso<SPI>,
    spi::Spi<SPI, spi::Enabled>: embedded_hal_v1::spi::FullDuplex<u8, Error = spi::Error>,
{
    let mode = Mode {
        polarity: Polarity::IdleHigh,
        phase: Phase::CaptureOnSecondTransition,
    };
    serial::Interface {
        spi: SpiWrap(spi.spi((sck, NoMiso, mosi), mode, 1.MHz(), prec, clocks)),
        timer,
        cs: cs.map(Into::into).map(SpiWrap),
    }
}
