use embedded_hal::digital::OutputPin;

use crate::{
    hal::{HasTimer, InPin, IoPin, OutPin, Timer},
    SharedBus,
};

use super::{Control, Input, Output};

pub struct Interface<Out, InOut, Timer, const PINS: usize, const BITS: usize> {
    pub rs: Out,
    pub rw: Out,
    pub e: [Out; PINS],
    pub bus: [InOut; BITS],
    pub timer: Timer,
}

impl<O, Io, T, const P: usize, const B: usize> SharedBus for Interface<O, Io, T, P, B> {
    type Interface<'a>
        = Interface<&'a mut O, &'a mut Io, &'a mut T, 1, B>
    where
        O: 'a,
        Io: 'a,
        T: 'a;

    fn num(&self) -> usize {
        P
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

impl<O, Io, T: Timer, const P: usize, const B: usize> HasTimer for Interface<O, Io, T, P, B> {
    fn timer(&mut self) -> &mut impl Timer {
        &mut self.timer
    }
}

impl<O: OutputPin, Io, T: Timer, const B: usize> Control for Interface<O, Io, T, 1, B> {
    type Error = O::Error;

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

impl<O, Io: OutPin, T, const P: usize, const B: usize> Interface<O, Io, T, P, B> {
    pub fn set_as_output(&mut self) -> Result<(), Io::Error> {
        self.bus.iter_mut().try_for_each(OutPin::set_as_output)
    }

    fn write_bus(&mut self, data: u8) -> Result<(), Io::Error> {
        for (i, pin) in self.bus.iter_mut().enumerate() {
            let level = data & (1 << i) != 0;
            pin.set_state(level.into())?
        }
        Ok(())
    }
}

impl<O, Io: IoPin, T, const P: usize, const B: usize> Interface<O, Io, T, P, B> {
    pub fn set_as_input(&mut self) -> Result<(), Io::Error> {
        self.bus.iter_mut().try_for_each(InPin::set_as_input)
    }

    fn read_bus(&mut self) -> Result<u8, Io::Error> {
        self.bus
            .iter_mut()
            .rev()
            .try_fold(0, |out, pin| Ok(out << 1 | pin.is_high()? as u8))
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

pub type Interface4Bit<Out, InOut, Timer, const PINS: usize> =
    Interface<Out, InOut, Timer, PINS, 4>;

impl<O, Io, T> Interface4Bit<O, Io, T, 1>
where
    O: OutputPin,
    Io: OutPin<Error = O::Error>,
    T: Timer,
{
    pub fn write_u4(&mut self, nibble: u8) -> Result<(), O::Error> {
        self.timer.complete();

        self.set_as_output()?;
        self.write_bus(nibble)?;
        self.latch()
    }
}

impl<O, Io, T> Interface4Bit<O, Io, T, 1>
where
    O: OutputPin,
    Io: IoPin<Error = O::Error>,
    T: Timer,
{
    pub fn read_u4(&mut self) -> Result<u8, O::Error> {
        self.timer.complete();

        self.set_as_input()?;
        self.try_latched(Self::read_bus)
    }
}

impl<O, Io, T> Output for Interface4Bit<O, Io, T, 1>
where
    O: OutputPin,
    Io: OutPin<Error = O::Error>,
    T: Timer,
{
    fn write_u8(&mut self, data: u8) -> Result<(), Self::Error> {
        self.write_u4(data >> 4)?;
        self.write_u4(data & 0xF)
    }
}

impl<O, Io, T> Input for Interface4Bit<O, Io, T, 1>
where
    O: OutputPin,
    Io: IoPin<Error = O::Error>,
    T: Timer,
{
    fn read_u8(&mut self) -> Result<u8, Self::Error> {
        let data = self.read_u4()? << 4 | self.read_u4()?;
        Ok(data)
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

pub type Interface8Bit<Out, InOut, Timer, const PINS: usize> =
    Interface<Out, InOut, Timer, PINS, 8>;

impl<O, Io, T> Output for Interface8Bit<O, Io, T, 1>
where
    O: OutputPin,
    Io: OutPin<Error = O::Error>,
    T: Timer,
{
    fn write_u8(&mut self, data: u8) -> Result<(), Self::Error> {
        self.timer.complete();

        self.set_as_output()?;
        self.write_bus(data)?;
        self.latch()
    }
}

impl<O, Io, T> Input for Interface8Bit<O, Io, T, 1>
where
    O: OutputPin,
    Io: IoPin<Error = O::Error>,
    T: Timer,
{
    fn read_u8(&mut self) -> Result<u8, Self::Error> {
        self.timer.complete();

        self.set_as_input()?;
        self.try_latched(Self::read_bus)
    }
}
