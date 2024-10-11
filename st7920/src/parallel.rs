//! This module defines some traits for handling the ST7920 through a parallel bus interface
//!
//! The parallel interface consists in a data bus, and enable signal and two additional
//! signals to select which _register_ to operate on
//! (check [`Output::select()`] for more informations).
//!
//! With the [`Output`] trait an implementation for the [`Execute`] trait is provided.

use core::convert::identity;

use crate::command::{ext, Command, Execute, ExecuteRead};
use crate::hal::{HasTimer, Timer};

pub mod interface;
pub use interface::{Interface4Bit, Interface8Bit};

/// A parallel bus interface to an ST7920 controlled LCD
pub trait Control {
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
    /// Setup the RAM for reading
    fn select_ram_read(&mut self) -> Result<(), Self::Error> {
        self.select(true, true)
    }
}

/// A parallel bus interface to an ST7920 controlled LCD
pub trait Output: Control {
    /// Write a byte to the data bus
    fn write_u8(&mut self, data: u8) -> Result<(), Self::Error>;
    /// Write two bytes to the data bus
    fn write_u16(&mut self, data: u16) -> Result<(), Self::Error> {
        self.write_u8((data >> 8) as u8)?;
        self.write_u8((data & 0xFF) as u8)
    }
}

/// A parallel bus interface to an ST7920 controlled LCD that
/// supports reading from the data buffer
pub trait Input: Control {
    /// Read a byte from the data bus
    fn read_u8(&mut self) -> Result<u8, Self::Error>;
    /// Read two bytes from the data bus
    fn read_u16(&mut self) -> Result<u16, Self::Error> {
        Ok((self.read_u8()? as u16) << 8 | self.read_u8()? as u16)
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

impl<O: Output + HasTimer> Execute for O {
    type Error = O::Error;

    fn execute(&mut self, command: Command) -> Result<(), Self::Error> {
        self.timer().complete();

        if let Command::Write(data) = command {
            self.select_ram_write()?;
            self.write_u16(data)?;
            self.timer().program(72);
            return Ok(());
        }

        self.select_command()?;
        self.write_u8(command.into_byte())?;
        self.timer().program(command.execution_time());
        Ok(())
    }
}

impl<O: Output + HasTimer> ext::Execute for O {
    fn execute_ext(&mut self, command: ext::Command) -> Result<(), Self::Error> {
        self.timer().complete();

        self.select_command()?;
        let [first, second] = command.into_bytes();
        self.write_u8(first)?;
        if second != 0 {
            self.write_u8(second)?;
        }
        self.timer().program(command.execution_time());
        Ok(())
    }
}

impl<I: Input + HasTimer> ExecuteRead for I {
    type Error = I::Error;

    fn read_bf_ac(&mut self) -> Result<(bool, u8), Self::Error> {
        self.timer().complete();

        self.select_bf_ac()?;
        let read = self.read_u8()?;
        Ok((read & 0b10000000 != 0, read & 0b01111111))
    }

    fn read(&mut self) -> Result<u16, Self::Error> {
        self.timer().complete();

        self.select_ram_read()?;
        let read = self.read_u16()?;
        self.timer().program(72);
        Ok(read)
    }
}
