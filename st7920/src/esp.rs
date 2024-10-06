use esp_hal::{gpio, peripheral::Peripheral};
use gpio::{Flex, Output};

use fugit::ExtU32;

use crate::hal::Timer;
use crate::parallel::{Interface, Interface4Bit, Interface8Bit};

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
        now() < *self
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
) -> Interface4Bit<Output<'a>, Flex<'a>> {
    use gpio::Level::Low;
    Interface {
        rs: Output::new(rs, Low),
        rw: Output::new(rw, Low),
        e: Output::new(e, Low),
        bus: [
            Flex::new(db4),
            Flex::new(db5),
            Flex::new(db6),
            Flex::new(db7),
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
) -> Interface8Bit<Output<'a>, Flex<'a>> {
    use gpio::Level::Low;
    Interface {
        rs: Output::new(rs, Low),
        rw: Output::new(rw, Low),
        e: Output::new(e, Low),
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
    }
}
