//! This module defines some traits for handling the ST7920 through a parallel bus interface
//!
//! The parallel interface consists in a data bus, and enable signal and two additional
//! signals to select which _register_ to operate on
//! (check [`Output::select()`] for more informations).
//!
//! With the [`Output`] trait an implementation for the [`Execute`] trait is provided.

use core::convert::identity;

use embedded_hal::digital::PinState::{High, Low};

use crate::{
    command::{ext, Command, Execute, ExecuteRead},
    hal::{IoPin, OutputPin},
};

/// A parallel bus interface to an ST7920 controlled LCD
pub trait Output {
    type Error;

    /// Set the enable signal high
    fn enable(&mut self) -> Result<(), Self::Error>;
    /// Set the enable signal low
    fn disable(&mut self) -> Result<(), Self::Error>;

    /// Send an impulse on the enable pin
    fn latch(&mut self) -> Result<(), Self::Error> {
        self.enable().and_then(|_| self.disable())
    }

    /// Run the given closure while the enable signal is high
    ///
    /// After the closure returns the enable signal is set to low
    fn latched<T>(&mut self, run: impl FnOnce(&mut Self) -> T) -> Result<T, Self::Error> {
        self.enable()?;
        let result = run(self);
        self.disable()?;
        Ok(result)
    }

    fn try_latched<T>(
        &mut self,
        run: impl FnOnce(&mut Self) -> Result<T, Self::Error>,
    ) -> Result<T, Self::Error> {
        self.latched(run).and_then(identity)
    }

    /// Select between the RAM, instruction or busy flag registers
    ///
    /// | rs | rw | function               | Description                   |
    /// |----|----|------------------------|-------------------------------|
    /// | `0`| `0`| [`select_command()`]   | Instruction register          |
    /// | `0`| `1`| [`select_bf_ac()`]     | Busy flag and address counter |
    /// | `1`| `0`| [`select_ram_write()`] | RAM write                     |
    /// | `1`| `1`| [`select_ram_read()`]  | RAM read                      |
    ///
    /// [`select_command()`]: Output::select_command
    /// [`select_bf_ac()`]: Output::select_bf_ac
    /// [`select_ram_write()`]: Output::select_ram_write
    /// [`select_ram_read()`]: Input::select_ram_read
    fn select(&mut self, rs: bool, rw: bool) -> Result<(), Self::Error>;
    /// Select the command register
    fn select_command(&mut self) -> Result<(), Self::Error> {
        self.select(false, false)
    }
    /// Select the busy flag and address counter
    fn select_bf_ac(&mut self) -> Result<(), Self::Error> {
        self.select(false, true)
    }
    /// Setup the RAM for writing
    fn select_ram_write(&mut self) -> Result<(), Self::Error> {
        self.select(true, false)
    }

    /// Write a byte to the data bus
    fn write_u8(&mut self, data: u8) -> Result<(), Self::Error>;
    /// Write two bytes to the data bus
    fn write_u16(&mut self, data: u16) -> Result<(), Self::Error> {
        [data >> 8, data & 0xFF]
            .into_iter()
            .try_for_each(|byte| self.write_u8(byte as u8))
    }
}

/// A parallel bus interface to an ST7920 controlled LCD that
/// supports reading from the data buffer
pub trait Input: Output {
    /// Setup the RAM for reading
    fn select_ram_read(&mut self) -> Result<(), Self::Error> {
        self.select(true, true)
    }

    /// Read a byte from the data bus
    fn read_u8(&mut self) -> Result<u8, Self::Error>;
    /// Read two bytes from the data bus
    fn read_u16(&mut self) -> Result<u16, Self::Error> {
        let [h, l] = [self.read_u8()?, self.read_u8()?].map(u16::from);
        Ok((h as u16) << 8 | l as u16)
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

pub struct Interface<Out, InOut, const BITS: usize> {
    pub rs: Out,
    pub rw: Out,
    pub e: Out,
    pub bus: [InOut; BITS],
}

pub type Interface4Bit<Out, InOut> = Interface<Out, InOut, 4>;

impl<O, Io, E> Interface4Bit<O, Io>
where
    O: OutputPin<Error = E>,
    Io: OutputPin<Error = E>,
{
    pub fn write_u4(&mut self, nibble: u8) -> Result<(), E> {
        for (i, pin) in self.bus.iter_mut().enumerate() {
            let level = nibble & (1 << i) != 0;
            pin.set_state(level.into())?;
        }
        self.latch()
    }
}

impl<O, Io, E> Interface4Bit<O, Io>
where
    O: OutputPin<Error = E>,
    Io: IoPin<E>,
{
    pub fn read_u4(&mut self) -> Result<u8, E> {
        self.try_latched(|this| {
            let [d4, d5, d6, d7] = this.bus.each_mut().map(|pin| Ok(pin.is_high()? as u8));
            Ok(d7? << 3 | d6? << 2 | d5? << 1 | d4?)
        })
    }
}

impl<O, Io, E> Output for Interface4Bit<O, Io>
where
    O: OutputPin<Error = E>,
    Io: OutputPin<Error = E>,
{
    type Error = E;

    fn enable(&mut self) -> Result<(), Self::Error> {
        self.e.set_state(High)
    }

    fn disable(&mut self) -> Result<(), Self::Error> {
        self.e.set_state(Low)
    }

    fn select(&mut self, rs: bool, rw: bool) -> Result<(), Self::Error> {
        self.rs.set_state(rs.into())?;
        self.rw.set_state(rw.into())
    }

    fn write_u8(&mut self, data: u8) -> Result<(), Self::Error> {
        self.write_u4(data >> 4)?;
        self.write_u4(data & 0xF)
    }
}

impl<O, Io, E> Input for Interface4Bit<O, Io>
where
    O: OutputPin<Error = E>,
    Io: IoPin<E>,
{
    fn read_u8(&mut self) -> Result<u8, Self::Error> {
        Ok(self.read_u4()? | self.read_u4()? << 4)
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

pub type Interface8Bit<Out, InOut> = Interface<Out, InOut, 8>;

impl<O, Io, E> Output for Interface8Bit<O, Io>
where
    O: OutputPin<Error = E>,
    Io: OutputPin<Error = E>,
{
    type Error = E;

    fn enable(&mut self) -> Result<(), Self::Error> {
        self.e.set_state(High)
    }

    fn disable(&mut self) -> Result<(), Self::Error> {
        self.e.set_state(Low)
    }

    fn select(&mut self, rs: bool, rw: bool) -> Result<(), Self::Error> {
        self.rs.set_state(rs.into())?;
        self.rw.set_state(rw.into())
    }

    fn write_u8(&mut self, data: u8) -> Result<(), Self::Error> {
        for (i, pin) in self.bus.iter_mut().enumerate() {
            let level = data & (1 << i) != 0;
            pin.set_state(level.into())?
        }
        self.latch()
    }
}

impl<O, Io, E> Input for Interface8Bit<O, Io>
where
    O: OutputPin<Error = E>,
    Io: IoPin<E>,
{
    fn read_u8(&mut self) -> Result<u8, Self::Error> {
        self.try_latched(|this| {
            this.bus
                .iter_mut()
                .rev()
                .try_fold(0, |out, pin| Ok(out << 1 | pin.is_high()? as u8))
        })
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

impl<O: Output> Execute for O {
    type Error = O::Error;

    fn execute(&mut self, command: Command) -> Result<(), Self::Error> {
        if let Command::Write(data) = command {
            self.select_ram_write()?;
            return self.write_u16(data);
        }

        self.select_command()?;

        self.write_u8(command.into_byte())?;
        Ok(())
    }
}

impl<O: Output> ext::Execute for O {
    fn execute_ext(&mut self, command: ext::Command) -> Result<(), Self::Error> {
        self.select_command()?;

        let [first, second] = command.into_bytes();
        self.write_u8(first)?;
        if second != 0 {
            self.write_u8(second)?;
        }
        Ok(())
    }
}

impl<I: Input> ExecuteRead for I {
    type Error = I::Error;

    fn read_bf_ac(&mut self) -> Result<(bool, u8), Self::Error> {
        self.select_bf_ac()?;
        let read = self.read_u8()?;
        Ok((read & 0b10000000 != 0, read & 0b01111111))
    }

    fn read(&mut self) -> Result<u16, Self::Error> {
        self.select_ram_read()?;
        self.read_u16()
    }
}
