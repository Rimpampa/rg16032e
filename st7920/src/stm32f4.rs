use embedded_hal::digital::{ErrorType, InputPin, OutputPin};
use embedded_hal::spi::SpiBus;
use fugit::RateExtU32;
use stm32f4xx_hal::gpio::{self, AnyPin, NoPin, PushPull};
use stm32f4xx_hal::rcc;
use stm32f4xx_hal::spi::{self, Mode, NoMiso, Phase, Polarity, Spi};

use crate::hal::{InPin, OutPin};
use crate::{parallel::interface as parallel, serial};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

// default mode
type Input = AnyPin<gpio::Input>;
type Output = AnyPin<gpio::Output<PushPull>>;

pub enum Flex {
    Input(Input),
    Output(Output),
    None,
}

impl Flex {
    pub fn new(pin: impl Into<Input>) -> Self {
        let mut pin: Output = pin.into().into_mode();
        pin.set_low();
        Self::Output(pin)
    }

    fn take(&mut self) -> Self {
        core::mem::replace(self, Self::None)
    }
}

impl ErrorType for Flex {
    type Error = core::convert::Infallible;
}

impl InputPin for Flex {
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        self.set_as_input()?;
        let Self::Input(pin) = self else {
            unreachable!()
        };
        pin.is_high()
    }

    fn is_low(&mut self) -> Result<bool, Self::Error> {
        self.set_as_input()?;
        let Self::Input(pin) = self else {
            unreachable!()
        };
        pin.is_low()
    }
}

impl InPin for Flex {
    fn set_as_input(&mut self) -> Result<(), Self::Error> {
        *self = match self.take() {
            pin @ Self::Input(_) => pin,
            Self::Output(pin) => Self::Input(pin.into_mode()),
            Self::None => unreachable!(),
        };
        Ok(())
    }
}

impl OutputPin for Flex {
    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.set_as_output()?;
        let Self::Output(pin) = self else {
            unreachable!()
        };
        pin.set_low();
        Ok(())
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.set_as_output()?;
        let Self::Output(pin) = self else {
            unreachable!()
        };
        pin.set_high();
        Ok(())
    }
}

impl OutPin for Flex {
    fn set_as_output(&mut self) -> Result<(), Self::Error> {
        *self = match self.take() {
            pin @ Self::Output(_) => pin,
            Self::Input(pin) => Self::Output(pin.into_mode()),
            Self::None => unreachable!(),
        };
        Ok(())
    }
}

impl InPin for Input {
    fn set_as_input(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl OutPin for Output {
    fn set_as_output(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

pub fn parallel_4bit<const NUM: usize>(
    rs: impl Into<Output>,
    rw: impl Into<Output>,
    e: [impl Into<Output>; NUM],
    db4: impl Into<Input>,
    db5: impl Into<Input>,
    db6: impl Into<Input>,
    db7: impl Into<Input>,
) -> parallel::Interface4Bit<Output, Flex, NUM> {
    parallel::Interface::new(
        rs.into(),
        rw.into(),
        e.map(Into::into),
        [
            Flex::new(db4),
            Flex::new(db5),
            Flex::new(db6),
            Flex::new(db7),
        ],
    )
}

pub fn parallel_8bit<const NUM: usize>(
    rs: impl Into<Output>,
    rw: impl Into<Output>,
    e: [impl Into<Output>; NUM],
    db0: impl Into<Input>,
    db1: impl Into<Input>,
    db2: impl Into<Input>,
    db3: impl Into<Input>,
    db4: impl Into<Input>,
    db5: impl Into<Input>,
    db6: impl Into<Input>,
    db7: impl Into<Input>,
) -> parallel::Interface8Bit<Output, Flex, NUM> {
    parallel::Interface::new(
        rs.into(),
        rw.into(),
        e.map(Into::into),
        [
            Flex::new(db0),
            Flex::new(db1),
            Flex::new(db2),
            Flex::new(db3),
            Flex::new(db4),
            Flex::new(db5),
            Flex::new(db6),
            Flex::new(db7),
        ],
    )
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

pub fn serial<SPI: spi::Instance<Miso: From<NoPin>>, const NUM: usize>(
    spi: SPI,
    mosi: impl Into<SPI::Mosi>,
    sck: impl Into<SPI::Sck>,
    cs: [impl Into<Output>; NUM],
    clocks: &rcc::Clocks,
) -> serial::Interface<impl SpiBus, Output, NUM> {
    let mode = Mode {
        polarity: Polarity::IdleHigh,
        phase: Phase::CaptureOnSecondTransition,
    };
    serial::Interface::new(
        Spi::new(spi, (sck, NoMiso::new(), mosi), mode, 1.MHz(), clocks),
        cs.map(Into::into),
    )
}
