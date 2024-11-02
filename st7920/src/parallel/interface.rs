use crate::{
    hal::{HasTimer, InPin, OutPin, Timer},
    SharedBus,
};

use embedded_hal::digital::{InputPin, OutputPin};

use super::{Control, Input, Output};

pub trait Config {
    type Error;
    type Out: OutPin<Error = Self::Error>;
    type InOut: OutPin<Error = Self::Error>;
    type Timer: Timer;
}

pub struct Interface<C: Config, const PINS: usize, const BITS: usize> {
    pub rs: C::Out,
    pub rw: C::Out,
    pub e: [C::Out; PINS],
    pub bus: [C::InOut; BITS],
    pub timer: C::Timer,
}

impl<C: Config, const PINS: usize, const BITS: usize> SharedBus for Interface<C, PINS, BITS>
where
    for<'a> &'a mut C: Config<
        Error = C::Error,
        Out = &'a mut C::Out,
        InOut = &'a mut C::InOut,
        Timer = &'a mut C::Timer,
    >,
{
    type Interface<'a>
        = Interface<&'a mut C, 1, BITS>
    where
        C: 'a;

    fn num(&self) -> usize {
        PINS
    }

    fn get(&mut self, idx: usize) -> Option<Self::Interface<'_>> {
        self.e.get_mut(idx).map(|e| Interface {
            rs: &mut self.rs,
            rw: &mut self.rw,
            bus: self.bus.each_mut(),
            timer: &mut self.timer,
            e: [e],
        })
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

impl<C: Config, const PINS: usize, const BITS: usize> HasTimer for Interface<C, PINS, BITS> {
    fn timer(&mut self) -> &mut impl Timer {
        &mut self.timer
    }
}

impl<C: Config, const B: usize> Control for Interface<C, 1, B> {
    type Error = C::Error;

    fn enable(&mut self) -> Result<(), Self::Error> {
        self.timer.complete();
        self.e[0].set_high()
    }

    fn disable(&mut self) -> Result<(), Self::Error> {
        self.e[0].set_low()?;
        self.timer.program(10); // Enable Cycle Time, min 1800ns
        Ok(())
    }

    fn select(&mut self, rs: bool, rw: bool) -> Result<(), Self::Error> {
        self.rs.set_state(rs.into())?;
        self.rw.set_state(rw.into())?;
        Ok(())
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

impl<C: Config, const PINS: usize, const BITS: usize> Interface<C, PINS, BITS> {
    pub fn set_as_output(&mut self) -> Result<(), C::Error> {
        self.bus.iter_mut().try_for_each(OutPin::set_as_output)
    }

    fn write_bus(&mut self, data: u8) -> Result<(), C::Error> {
        for (i, pin) in self.bus.iter_mut().enumerate() {
            let level = data & (1 << i) != 0;
            pin.set_state(level.into())?
        }
        Ok(())
    }
}

impl<C: Config, const PINS: usize, const BITS: usize> Interface<C, PINS, BITS>
where
    C::InOut: InPin<Error = C::Error>,
{
    pub fn set_as_input(&mut self) -> Result<(), C::Error> {
        self.bus.iter_mut().try_for_each(InPin::set_as_input)
    }

    fn read_bus(&mut self) -> Result<u8, C::Error> {
        self.bus
            .iter_mut()
            .rev()
            .try_fold(0, |out, pin| Ok(out << 1 | pin.is_high()? as u8))
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

pub type Interface4Bit<C, const PINS: usize> = Interface<C, PINS, 4>;

impl<C: Config> Interface4Bit<C, 1> {
    pub fn write_u4(&mut self, nibble: u8) -> Result<(), C::Error> {
        self.timer.complete();

        self.set_as_output()?;
        self.write_bus(nibble)?;
        self.latch()
    }
}

impl<C: Config<InOut: InPin<Error = C::Error>>> Interface4Bit<C, 1> {
    pub fn read_u4(&mut self) -> Result<u8, C::Error> {
        self.timer.complete();

        self.set_as_input()?;
        self.try_latched(Self::read_bus)
    }
}

impl<C: Config> Output for Interface4Bit<C, 1> {
    fn write_u8(&mut self, data: u8) -> Result<(), Self::Error> {
        self.write_u4(data >> 4)?;
        self.write_u4(data & 0xF)
    }
}

impl<C: Config<InOut: InPin<Error = C::Error>>> Input for Interface4Bit<C, 1> {
    fn read_u8(&mut self) -> Result<u8, Self::Error> {
        let data = self.read_u4()? << 4 | self.read_u4()?;
        Ok(data)
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

pub type Interface8Bit<C, const PINS: usize> = Interface<C, PINS, 8>;

impl<C: Config> Output for Interface8Bit<C, 1> {
    fn write_u8(&mut self, data: u8) -> Result<(), Self::Error> {
        self.timer.complete();

        self.set_as_output()?;
        self.write_bus(data)?;
        self.latch()
    }
}

impl<C: Config<InOut: InPin<Error = C::Error>>> Input for Interface8Bit<C, 1> {
    fn read_u8(&mut self) -> Result<u8, Self::Error> {
        self.timer.complete();

        self.set_as_input()?;
        self.try_latched(Self::read_bus)
    }
}
