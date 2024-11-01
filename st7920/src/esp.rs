use esp_hal::gpio::{Flex, Input, NoPin, Output, PeripheralOutput, Pull};
use esp_hal::spi::master::{Instance, Spi};
use esp_hal::spi::{FullDuplexMode, SpiMode};
use esp_hal::{gpio, peripheral::Peripheral};

use fugit::{ExtU32, RateExtU32};

use crate::hal::{InPin, OutPin, Timer};
use crate::{parallel::interface as parallel, serial};

pub trait In = Peripheral<P: gpio::InputPin> + 'static;
pub trait Out = Peripheral<P: gpio::OutputPin> + 'static;

pub use esp_hal::time::now;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

pub type Instant = fugit::Instant<u64, 1, 1000000>;

impl Timer for Instant {
    fn program(&mut self, duration: u32) {
        *self = now() + duration.micros()
    }

    fn expired(&mut self) -> bool {
        now() > *self
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

impl InPin for Flex<'_> {
    fn set_as_input(&mut self) -> Result<(), Self::Error> {
        self.set_as_input(Pull::None);
        Ok(())
    }
}

impl OutPin for Flex<'_> {
    fn set_as_output(&mut self) -> Result<(), Self::Error> {
        self.set_as_output();
        Ok(())
    }
}

impl InPin for Input<'_> {
    fn set_as_input(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl OutPin for Output<'_> {
    fn set_as_output(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

pub fn parallel_4bit<'a, const NUM: usize>(
    rs: impl Out + 'a,
    rw: impl Out + 'a,
    e: [impl Out + 'a; NUM],
    db4: impl In + Out + 'a,
    db5: impl In + Out + 'a,
    db6: impl In + Out + 'a,
    db7: impl In + Out + 'a,
) -> parallel::Interface4Bit<Output<'a>, Flex<'a>, Instant, NUM> {
    use gpio::Level::Low;
    parallel::Interface {
        rs: Output::new(rs, Low),
        rw: Output::new(rw, Low),
        e: e.map(|e| Output::new(e, Low)),
        bus: [
            Flex::new(db4),
            Flex::new(db5),
            Flex::new(db6),
            Flex::new(db7),
        ],
        timer: now(),
    }
}

pub fn parallel_8bit<'a, const NUM: usize>(
    rs: impl Out + 'a,
    rw: impl Out + 'a,
    e: [impl Out + 'a; NUM],
    db0: impl In + Out + 'a,
    db1: impl In + Out + 'a,
    db2: impl In + Out + 'a,
    db3: impl In + Out + 'a,
    db4: impl In + Out + 'a,
    db5: impl In + Out + 'a,
    db6: impl In + Out + 'a,
    db7: impl In + Out + 'a,
) -> parallel::Interface8Bit<Output<'a>, Flex<'a>, Instant, NUM> {
    use gpio::Level::Low;
    parallel::Interface {
        rs: Output::new(rs, Low),
        rw: Output::new(rw, Low),
        e: e.map(|e| Output::new(e, Low)),
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
        timer: now(),
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

pub fn serial<'a, I: Instance + 'a, const NUM: usize>(
    spi: impl Peripheral<P = I> + 'a,
    mosi: impl Peripheral<P: PeripheralOutput> + 'a,
    sck: impl Peripheral<P: PeripheralOutput> + 'a,
    cs: [impl Out + 'a; NUM],
) -> serial::Interface<Spi<'a, I, FullDuplexMode>, Instant, Output<'a>, NUM> {
    use gpio::Level::Low;
    serial::Interface {
        spi: Spi::new(spi, 530.kHz(), SpiMode::Mode0).with_pins(sck, mosi, NoPin, NoPin),
        timer: now(),
        cs: cs.map(|cs| Output::new(cs, Low)),
    }
}
