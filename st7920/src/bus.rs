use core::iter::zip;

use esp_hal::gpio::{AnyFlex, Level, Pull};

use super::{In, Out};

fn write(pin: &mut AnyFlex<'static>, data: u8, bit: u8) {
    pin.set_level(Level::from(data & 1 << bit != 0))
}

fn read(pin: &mut AnyFlex<'static>, bit: u8) -> u8 {
    (bool::from(pin.get_level()) as u8) << bit
}

pub struct Bus {
    pub db4: AnyFlex<'static>,
    pub db5: AnyFlex<'static>,
    pub db6: AnyFlex<'static>,
    pub db7: AnyFlex<'static>,
}

pub macro new($($field:ident : $value:expr),* $(,)?) {
    $crate::bus::Bus {
        $( $field : ::esp_hal::gpio::AnyFlex::new($value), )*
    }
}

impl Bus {
    pub fn new(
        db4: impl In + Out,
        db5: impl In + Out,
        db6: impl In + Out,
        db7: impl In + Out,
    ) -> Self {
        Self {
            db4: AnyFlex::new(db4),
            db5: AnyFlex::new(db5),
            db6: AnyFlex::new(db6),
            db7: AnyFlex::new(db7),
        }
    }

    fn pins(&mut self) -> core::array::IntoIter<&mut AnyFlex<'static>, 4> {
        let Self { db4, db5, db6, db7 } = self;
        [db4, db5, db6, db7].into_iter()
    }

    fn set_as_output(&mut self) {
        self.pins().for_each(|pin| pin.set_as_output());
    }

    fn set_as_input(&mut self) {
        self.pins().for_each(|pin| pin.set_as_input(Pull::None));
    }

    pub fn write(&mut self, nibble: u8) {
        self.set_as_output();
        zip(0u8.., self.pins()).for_each(|(bit, pin)| write(pin, nibble, bit));
    }

    pub fn read(&mut self) -> u8 {
        self.set_as_input();
        zip(0u8.., self.pins()).fold(0, |out, (bit, pin)| out | read(pin, bit))
    }
}
