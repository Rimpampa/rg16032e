use core::marker::PhantomData;

use embedded_hal::digital::{ErrorType, InputPin, OutputPin};
use embedded_hal::spi::SpiBus;
use fugit::RateExtU32;
use stm32f4xx_hal::gpio::{self, AnyPin, NoPin, PushPull};
use stm32f4xx_hal::rcc;
use stm32f4xx_hal::spi::{self, Mode, NoMiso, Phase, Polarity, Spi};

use crate::hal::{self, InPin, OutPin};
use crate::{parallel::interface as parallel, serial};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

pub struct Clock<TIM>(PhantomData<TIM>);

impl<TIM> Clone for Clock<TIM> {
    fn clone(&self) -> Self {
        Self(PhantomData)
    }
}

impl<TIM> Copy for Clock<TIM> {}

impl<TIM: HwTimer> Clock<TIM> {
    pub fn new(mut tim: TIM, clocks: &rcc::Clocks) -> Self {
        // SAFETY: copied from stm32f4xx_hal::timer::FTimer::new
        unsafe {
            TIM::enable_unchecked();
            TIM::reset_unchecked();
        }
        const FREQ: u32 = 1_000_000;

        let clk = TIM::timer_clock(clocks);
        assert!(clk.raw() % FREQ == 0);
        let psc = clk.raw() / FREQ;
        let psc = u16::try_from(psc - 1).unwrap();
        tim.set_psc(psc);

        // SAFETY: copied from stm32f4xx_hal::timer::monotonic::FTimer::monotonic
        unsafe { tim.set_arr_max() };

        tim.start();

        Self(PhantomData)
    }
}

impl<TIM: HwTimer> hal::Clock for Clock<TIM> {
    fn now(self) -> hal::Instant {
        // SAFETY: It is safe to read the cnt register, even if the TIM is steal()ed
        // TODO: fact-check this statement
        hal::Instant::from_ticks(unsafe { TIM::read_count() })
    }
}

pub trait HwTimer: rcc::Enable + rcc::Reset + rcc::BusTimerClock {
    fn set_psc(&mut self, value: u16);
    unsafe fn set_arr_max(&mut self);
    fn start(&mut self);
    unsafe fn read_count() -> u64;
}

macro_rules! impl_clocks {
    ($($tim:ty = $bits:ty),* $(,)?) => {
        $(
            impl HwTimer for $tim {
                fn set_psc(&mut self, value: u16) {
                    self.psc().write(|w| w.psc().set(value));
                }

                unsafe fn set_arr_max(&mut self) {
                    self.arr().write(|w| w.bits(<$bits>::MAX as u32))
                }

                fn start(&mut self) {
                    // trigger_update()
                    self.cr1().modify(|_, w| w.urs().set_bit());
                    self.egr().write(|w| w.ug().set_bit());
                    self.cr1().modify(|_, w| w.urs().clear_bit());
                    // start_free(true)
                    self.cr1().modify(|_, w| w.cen().set_bit().udis().bit(false));
                }

                unsafe fn read_count() -> u64 {
                    <$tim>::steal().cnt().read().bits() as $bits as u64
                }
            }
        )*
    };
}

use stm32f4xx_hal::pac;
impl_clocks![
    pac::TIM1 = u16,
    pac::TIM2 = u32,
    pac::TIM3 = u16,
    pac::TIM4 = u16,
    // pac::TIM5 = u32,
    // pac::TIM5 = u16,
    // pac::TIM6 = u16,
    // pac::TIM7 = u16,
    // pac::TIM8 = u16,
    pac::TIM9 = u16,
    pac::TIM10 = u16,
    pac::TIM11 = u16,
    // pac::TIM12 = u16,
    // pac::TIM13 = u16,
    // pac::TIM14 = u16,
];

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

pub fn parallel_4bit<Clk: hal::Clock, const NUM: usize>(
    rs: impl Into<Output>,
    rw: impl Into<Output>,
    e: [impl Into<Output>; NUM],
    db4: impl Into<Input>,
    db5: impl Into<Input>,
    db6: impl Into<Input>,
    db7: impl Into<Input>,
    clock: Clk,
) -> parallel::Interface4Bit<Output, Flex, Clk, NUM> {
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
        clock,
    )
}

pub fn parallel_8bit<Clk: hal::Clock, const NUM: usize>(
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
    clock: Clk,
) -> parallel::Interface8Bit<Output, Flex, Clk, NUM> {
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
        clock,
    )
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

pub fn serial<SPI: spi::Instance<Miso: From<NoPin>>, Clk: hal::Clock, const NUM: usize>(
    spi: SPI,
    mosi: impl Into<SPI::Mosi>,
    sck: impl Into<SPI::Sck>,
    cs: [impl Into<Output>; NUM],
    clocks: &rcc::Clocks,
    clock: Clk,
) -> serial::Interface<impl SpiBus, Clk, Output, NUM> {
    let mode = Mode {
        polarity: Polarity::IdleHigh,
        phase: Phase::CaptureOnSecondTransition,
    };
    serial::Interface::new(
        Spi::new(spi, (sck, NoMiso::new(), mosi), mode, 1.MHz(), clocks),
        clock,
        cs.map(Into::into),
    )
}
