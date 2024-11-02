use embedded_hal::digital::{ErrorType, InputPin, OutputPin};
use embedded_hal::spi::SpiBus;
use fugit::{ExtU32, RateExtU32};
use stm32f4xx_hal::gpio::{self, AnyPin, NoPin, PushPull};
use stm32f4xx_hal::rcc::Clocks;
use stm32f4xx_hal::spi::{self, Mode, NoMiso, Phase, Polarity, Spi};
use stm32f4xx_hal::timer::{self, CounterUs, SysCounterUs};

use crate::hal::{InPin, OutPin, Timer};
use crate::{parallel::interface as parallel, serial};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

impl Timer for SysCounterUs {
    fn program(&mut self, duration: u32) {
        self.start(duration.micros()).unwrap()
    }

    fn expired(&mut self) -> bool {
        self.wait().is_ok()
    }
}

impl<TIM: timer::Instance> Timer for CounterUs<TIM> {
    fn program(&mut self, duration: u32) {
        self.start(duration.micros()).unwrap()
    }

    fn expired(&mut self) -> bool {
        self.wait().is_ok()
    }
}

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

pub struct ParallelConfig<T>(core::marker::PhantomData<T>);

impl<T: Timer> parallel::Config for ParallelConfig<T> {
    type Error = core::convert::Infallible;
    type Out = Output;
    type InOut = Flex;
    type Timer = T;
}

pub fn parallel_4bit<T: Timer, const NUM: usize>(
    rs: impl Into<Output>,
    rw: impl Into<Output>,
    e: [impl Into<Output>; NUM],
    db4: impl Into<Input>,
    db5: impl Into<Input>,
    db6: impl Into<Input>,
    db7: impl Into<Input>,
    timer: T,
) -> parallel::Interface4Bit<ParallelConfig<T>, NUM> {
    parallel::Interface {
        rs: rs.into(),
        rw: rw.into(),
        e: e.map(Into::into),
        bus: [
            Flex::new(db4),
            Flex::new(db5),
            Flex::new(db6),
            Flex::new(db7),
        ],
        timer,
    }
}

pub fn parallel_8bit<T: Timer, const NUM: usize>(
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
    timer: T,
) -> parallel::Interface8Bit<ParallelConfig<T>, NUM> {
    parallel::Interface {
        rs: rs.into(),
        rw: rw.into(),
        e: e.map(Into::into),
        bus: [
            Flex::new(db0),
            Flex::new(db1),
            Flex::new(db2),
            Flex::new(db3),
            Flex::new(db4),
            Flex::new(db5),
            Flex::new(db6),
            Flex::new(db7),
        ],
        timer,
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

pub fn serial<SPI: spi::Instance<Miso: From<NoPin>>, T: Timer, const NUM: usize>(
    spi: SPI,
    mosi: impl Into<SPI::Mosi>,
    sck: impl Into<SPI::Sck>,
    cs: [impl Into<Output>; NUM],
    clocks: &Clocks,
    timer: T,
) -> serial::Interface<impl SpiBus, T, Output, NUM> {
    let mode = Mode {
        polarity: Polarity::IdleHigh,
        phase: Phase::CaptureOnSecondTransition,
    };
    serial::Interface {
        spi: Spi::new(spi, (sck, NoMiso::new(), mosi), mode, 1.MHz(), clocks),
        timer,
        cs: cs.map(Into::into),
    }
}
