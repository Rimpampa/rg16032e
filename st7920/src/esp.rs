use esp_hal::{gpio, peripheral::Peripheral};
use gpio::{AnyFlex as AnyIo, AnyOutput as AnyOut};

use fugit::ExtU32;

use crate::hal::{InputPin, OutputPin, Timer};
use crate::parallel::{Interface, Interface4Bit, Interface8Bit};

pub trait In = Peripheral<P: gpio::InputPin + gpio::CreateErasedPin> + 'static;
pub trait Out = Peripheral<P: gpio::OutputPin + gpio::CreateErasedPin> + 'static;

pub use esp_hal::time::current_time as now;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

pub type Instant = fugit::Instant<u64, 1, 1000000>;

impl Timer for Instant {
    fn program(&mut self, duration: u32) {
        *self = now() + duration.micros()
    }

    fn expired(&mut self) -> bool {
        now() < *self
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

impl OutputPin for AnyOut<'_> {
    fn write(&mut self, level: bool) {
        self.set_level(level.into());
    }
}

impl OutputPin for AnyIo<'_> {
    fn write(&mut self, level: bool) {
        self.set_level(level.into());
    }
}

impl InputPin for AnyIo<'_> {
    fn read(&mut self) -> bool {
        self.is_high()
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

pub fn parallel_4bit<'a>(
    rs: impl Out + 'a,
    rw: impl Out + 'a,
    e: impl Out + 'a,
    db4: impl In + Out + 'a,
    db5: impl In + Out + 'a,
    db6: impl In + Out + 'a,
    db7: impl In + Out + 'a,
) -> Interface4Bit<AnyOut<'a>, AnyIo<'a>> {
    use gpio::Level::Low;
    Interface {
        rs: AnyOut::new(rs, Low),
        rw: AnyOut::new(rw, Low),
        e: AnyOut::new(e, Low),
        bus: [
            AnyIo::new(db4),
            AnyIo::new(db5),
            AnyIo::new(db6),
            AnyIo::new(db7),
        ],
    }
}

pub fn parallel_8bit<'a>(
    rs: impl Out + 'a,
    rw: impl Out + 'a,
    e: impl Out + 'a,
    db0: impl In + Out + 'a,
    db1: impl In + Out + 'a,
    db2: impl In + Out + 'a,
    db3: impl In + Out + 'a,
    db4: impl In + Out + 'a,
    db5: impl In + Out + 'a,
    db6: impl In + Out + 'a,
    db7: impl In + Out + 'a,
) -> Interface8Bit<AnyOut<'a>, AnyIo<'a>> {
    use gpio::Level::Low;
    Interface {
        rs: AnyOut::new(rs, Low),
        rw: AnyOut::new(rw, Low),
        e: AnyOut::new(e, Low),
        bus: [
            AnyIo::new(db0),
            AnyIo::new(db1),
            AnyIo::new(db2),
            AnyIo::new(db3),
            AnyIo::new(db4),
            AnyIo::new(db5),
            AnyIo::new(db6),
            AnyIo::new(db7),
        ],
    }
}
