//! This module defines some traits for handling the ST7920 through a parallel bus interface
//!
//! The parallel interface consists in a data bus, and enable signal and two additional
//! signals to select which _register_ to operate on
//! (check [`Output::select()`] for more informations).
//!
//! With the [`Output`] trait an implementation for the [`Execute`] trait is provided.

use core::convert::identity;

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

impl<T: Control> Control for &mut T {
    type Error = T::Error;

    fn enable(&mut self) -> Result<(), Self::Error> {
        T::enable(self)
    }
    fn disable(&mut self) -> Result<(), Self::Error> {
        T::disable(self)
    }

    fn select(&mut self, rs: bool, rw: bool) -> Result<(), Self::Error> {
        T::select(self, rs, rw)
    }
}

impl<T: Output> Output for &mut T {
    fn write_u8(&mut self, data: u8) -> Result<(), Self::Error> {
        T::write_u8(self, data)
    }
}

impl<T: Input> Input for &mut T {
    fn read_u8(&mut self) -> Result<u8, Self::Error> {
        T::read_u8(self)
    }
}
