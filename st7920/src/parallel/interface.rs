use embedded_hal::digital::OutputPin;
use fugit::ExtU64;

use crate::hal::{Clock, InPin, Instant, IoPin, OutPin};
use crate::{ext, Command, Execute, ExecuteRead, SharedBus};

use super::{Control, Input, Output};

struct Pin<E> {
    e: E,
    end: Instant,
}

pub struct Interface<Out, InOut, Clk, const PINS: usize, const BITS: usize> {
    rs: Out,
    rw: Out,
    pins: [Pin<Out>; PINS],
    bus: [InOut; BITS],
    clk: Clk,
}

impl<O, Io, Clk: Clock, const PINS: usize, const BITS: usize> Interface<O, Io, Clk, PINS, BITS> {
    pub fn new(rs: O, rw: O, e: [O; PINS], bus: [Io; BITS], clk: Clk) -> Self {
        let end = clk.now();
        let pins = e.map(|e| Pin { e, end });
        Self {
            rs,
            rw,
            pins,
            bus,
            clk,
        }
    }
}

impl<O, Io, Clk: Copy, const PINS: usize, const BITS: usize> SharedBus
    for Interface<O, Io, Clk, PINS, BITS>
{
    type Interface<'a>
        = Interface<&'a mut O, &'a mut Io, Clk, 1, BITS>
    where
        O: 'a,
        Io: 'a,
        Clk: 'a;

    fn num(&self) -> usize {
        PINS
    }

    fn get(&mut self, idx: usize) -> Option<Self::Interface<'_>> {
        self.pins.get_mut(idx).map(|Pin { e, end }| Interface {
            rs: &mut self.rs,
            rw: &mut self.rw,
            bus: self.bus.each_mut(),
            clk: self.clk,
            pins: [Pin { e, end: *end }],
        })
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

impl<O: OutputPin, Io, Clk: Clock, const BITS: usize> Control for Interface<O, Io, Clk, 1, BITS> {
    type Error = O::Error;

    fn enable(&mut self) -> Result<(), Self::Error> {
        self.pins[0].e.set_high()
    }

    fn disable(&mut self) -> Result<(), Self::Error> {
        self.pins[0].e.set_low()?;
        Ok(())
    }

    fn select(&mut self, rs: bool, rw: bool) -> Result<(), Self::Error> {
        self.rs.set_state(rs.into())?;
        self.rw.set_state(rw.into())?;
        Ok(())
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

impl<O, Io: OutPin, Clk, const P: usize, const BITS: usize> Interface<O, Io, Clk, P, BITS> {
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

impl<O, Io: IoPin, Clk, const P: usize, const BITS: usize> Interface<O, Io, Clk, P, BITS> {
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

pub type Interface4Bit<Out, InOut, Clock, const PINS: usize> =
    Interface<Out, InOut, Clock, PINS, 4>;

impl<O, Io, Clk> Interface4Bit<O, Io, Clk, 1>
where
    O: OutputPin,
    Io: OutPin<Error = O::Error>,
    Clk: Clock,
{
    pub fn write_u4(&mut self, nibble: u8) -> Result<(), O::Error> {
        self.set_as_output()?;
        self.write_bus(nibble)?;
        self.latch()
    }
}

impl<O, Io, Clk> Interface4Bit<O, Io, Clk, 1>
where
    O: OutputPin,
    Io: IoPin<Error = O::Error>,
    Clk: Clock,
{
    pub fn read_u4(&mut self) -> Result<u8, O::Error> {
        self.set_as_input()?;
        self.try_latched(Self::read_bus)
    }
}

impl<O, Io, Clk> Output for Interface4Bit<O, Io, Clk, 1>
where
    O: OutputPin,
    Io: OutPin<Error = O::Error>,
    Clk: Clock,
{
    fn write_u8(&mut self, data: u8) -> Result<(), Self::Error> {
        self.write_u4(data >> 4)?;
        self.clk.wait(10.micros()); // Enable Cycle Time, min 1800ns
        self.write_u4(data & 0xF)
    }
}

impl<O, Io, Clk> Input for Interface4Bit<O, Io, Clk, 1>
where
    O: OutputPin,
    Io: IoPin<Error = O::Error>,
    Clk: Clock,
{
    fn read_u8(&mut self) -> Result<u8, Self::Error> {
        let h = self.read_u4()?;
        self.clk.wait(10.micros()); // Enable Cycle Time, min 1800ns
        let l = self.read_u4()?;
        Ok(h << 4 | l)
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

pub type Interface8Bit<Out, InOut, Clock, const PINS: usize> =
    Interface<Out, InOut, Clock, PINS, 8>;

impl<O, Io, Clk> Output for Interface8Bit<O, Io, Clk, 1>
where
    O: OutputPin,
    Io: OutPin<Error = O::Error>,
    Clk: Clock,
{
    fn write_u8(&mut self, data: u8) -> Result<(), Self::Error> {
        self.set_as_output()?;
        self.write_bus(data)?;
        self.latch()
    }

    fn write_u16(&mut self, data: u16) -> Result<(), Self::Error> {
        self.write_u8((data >> 8) as u8)?;
        self.clk.wait(10.micros()); // Enable Cycle Time, min 1800ns
        self.write_u8((data & 0xFF) as u8)
    }
}

impl<O, Io, Clk> Input for Interface8Bit<O, Io, Clk, 1>
where
    O: OutputPin,
    Io: IoPin<Error = O::Error>,
    Clk: Clock,
{
    fn read_u8(&mut self) -> Result<u8, Self::Error> {
        self.set_as_input()?;
        self.try_latched(Self::read_bus)
    }

    fn read_u16(&mut self) -> Result<u16, Self::Error> {
        let h = self.read_u8()? as u16;
        self.clk.wait(10.micros()); // Enable Cycle Time, min 1800ns
        let l = self.read_u8()? as u16;
        Ok(h << 8 | l)
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

impl<O, Io, Clk, const BITS: usize> Execute for Interface<O, Io, Clk, 1, BITS>
where
    O: OutputPin,
    Io: OutPin<Error = O::Error>,
    Clk: Clock,
    Self: Output<Error = O::Error>,
{
    type Error = O::Error;

    fn init(&mut self) -> Result<(), Self::Error> {
        crate::init(self, self.clk)
    }

    fn execute(&mut self, command: Command) -> Result<(), Self::Error> {
        self.clk.wait_until(self.pins[0].end);

        if let Command::Write(data) = command {
            self.select_ram_write()?;
            self.write_u16(data)?;
            self.pins[0].end = self.clk.now() + 72.micros();
            return Ok(());
        }

        self.select_command()?;
        self.write_u8(command.into_byte())?;
        self.pins[0].end = self.clk.now() + command.execution_time();
        Ok(())
    }
}

impl<O, Io, Clk, const BITS: usize> ext::Execute for Interface<O, Io, Clk, 1, BITS>
where
    O: OutputPin,
    Io: OutPin<Error = O::Error>,
    Clk: Clock,
    Self: Output<Error = O::Error>,
{
    fn execute_ext(&mut self, command: ext::Command) -> Result<(), Self::Error> {
        self.clk.wait_until(self.pins[0].end);

        self.select_command()?;
        let [first, second] = command.into_bytes();
        self.write_u8(first)?;
        if second != 0 {
            self.write_u8(second)?;
        }
        self.pins[0].end = self.clk.now() + command.execution_time();
        Ok(())
    }
}

impl<O, Io, Clk, const BITS: usize> ExecuteRead for Interface<O, Io, Clk, 1, BITS>
where
    O: OutputPin,
    Io: IoPin<Error = O::Error>,
    Clk: Clock,
    Self: Input<Error = O::Error>,
{
    type Error = Io::Error;

    fn read_bf_ac(&mut self) -> Result<(bool, u8), Self::Error> {
        self.clk.wait_until(self.pins[0].end);

        self.select_bf_ac()?;
        let read = self.read_u8()?;
        Ok((read & 0b10000000 != 0, read & 0b01111111))
    }

    fn read(&mut self) -> Result<u16, Self::Error> {
        self.clk.wait_until(self.pins[0].end);

        self.select_ram_read()?;
        let read = self.read_u16()?;
        self.pins[0].end = self.clk.now() + 72.micros();
        Ok(read)
    }
}


// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

impl<O, Io, Clk, const BITS: usize> Execute for &mut Interface<O, Io, Clk, 1, BITS>
where
    O: OutputPin,
    Io: OutPin<Error = O::Error>,
    Clk: Clock,
    Interface<O, Io, Clk, 1, BITS>: Output<Error = O::Error>,
{
    type Error = O::Error;

    fn init(&mut self) -> Result<(), Self::Error> {
        Interface::init(self)
    }

    fn execute(&mut self, command: Command) -> Result<(), Self::Error> {
        Interface::execute(self, command)
    }
}

impl<O, Io, Clk, const BITS: usize> ext::Execute for &mut Interface<O, Io, Clk, 1, BITS>
where
    O: OutputPin,
    Io: OutPin<Error = O::Error>,
    Clk: Clock,
    Interface<O, Io, Clk, 1, BITS>: Output<Error = O::Error>,
{
    fn execute_ext(&mut self, command: ext::Command) -> Result<(), Self::Error> {
        Interface::execute_ext(self, command)
    }
}

impl<O, Io, Clk, const BITS: usize> ExecuteRead for &mut Interface<O, Io, Clk, 1, BITS>
where
    O: OutputPin,
    Io: IoPin<Error = O::Error>,
    Clk: Clock,
    Interface<O, Io, Clk, 1, BITS>: Input<Error = O::Error>,
{
    type Error = Io::Error;

    fn read_bf_ac(&mut self) -> Result<(bool, u8), Self::Error> {
        Interface::read_bf_ac(self)
    }

    fn read(&mut self) -> Result<u16, Self::Error> {
        Interface::read(self)
    }
}
