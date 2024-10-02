//! This module defines some traits for handling the ST7920 through a parallel bus interface
//!
//! The parallel interface consists in a data bus, and enable signal and two additional
//! signals to select which _register_ to operate on
//! (check [`Output::select()`] for more informations).
//!
//! With the [`Output`] trait an implementation for the [`Execute`] trait is provided.

use core::ops::BitOr;

use crate::{
    command::{Command, Execute, ExecuteRead},
    hal::{IoPin, OutputPin},
};

/// A parallel bus interface to an ST7920 controlled LCD
pub trait Output {
    /// Set the enable signal high
    fn enable(&mut self);
    /// Set the enable signal low
    fn disable(&mut self);

    /// Send an impulse on the enable pin
    fn latch(&mut self) {
        self.enable();
        self.disable();
    }

    /// Run the given closure while the enable signal is high
    ///
    /// After the closure returns the enable signal is set to low
    fn latched<T>(&mut self, run: impl FnOnce(&mut Self) -> T) -> T {
        self.enable();
        let result = run(self);
        self.disable();
        result
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
    fn select(&mut self, rs: bool, rw: bool);
    /// Select the command register
    fn select_command(&mut self) {
        self.select(false, false);
    }
    /// Select the busy flag and address counter
    fn select_bf_ac(&mut self) {
        self.select(false, true);
    }
    /// Setup the RAM for writing
    fn select_ram_write(&mut self) {
        self.select(true, false);
    }

    /// Write a byte to the data bus
    fn write_u8(&mut self, data: u8);
    /// Write two bytes to the data bus
    fn write_u16(&mut self, data: u16) {
        self.write_u8((data >> 8) as u8);
        self.write_u8((data & 0xFF) as u8);
    }
}

/// A parallel bus interface to an ST7920 controlled LCD that
/// supports reading from the data buffer
pub trait Input: Output {
    /// Setup the RAM for reading
    fn select_ram_read(&mut self) {
        self.select(true, true);
    }

    /// Read a byte from the data bus
    fn read_u8(&mut self) -> u8;
    /// Read two bytes from the data bus
    fn read_u16(&mut self) -> u16 {
        (self.read_u8() as u16) << 8 | self.read_u8() as u16
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

impl<O: OutputPin, Io: OutputPin> Interface4Bit<O, Io> {
    pub fn write_u4(&mut self, nibble: u8) {
        for (i, pin) in self.bus.iter_mut().enumerate() {
            pin.write(nibble & (1 << i) != 0);
        }
        self.latch();
    }
}

impl<O: OutputPin, Io: IoPin> Interface4Bit<O, Io> {
    pub fn read_u4(&mut self) -> u8 {
        self.latched(|this| {
            let [d4, d5, d6, d7] = this.bus.each_mut().map(|pin| pin.read() as u8);
            d7 << 3 | d6 << 2 | d5 << 1 | d4
        })
    }
}

impl<O: OutputPin, Io: OutputPin> Output for Interface4Bit<O, Io> {
    fn enable(&mut self) {
        self.e.write(true);
    }

    fn disable(&mut self) {
        self.e.write(false);
    }

    fn select(&mut self, rs: bool, rw: bool) {
        self.rs.write(rs);
        self.rw.write(rw);
    }

    fn write_u8(&mut self, data: u8) {
        self.write_u4(data >> 4);
        self.write_u4(data & 0xF);
    }
}

impl<O: OutputPin, Io: IoPin> Input for Interface4Bit<O, Io> {
    fn read_u8(&mut self) -> u8 {
        self.read_u4() | self.read_u4() << 4
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

pub type Interface8Bit<Out, InOut> = Interface<Out, InOut, 8>;

impl<O: OutputPin, Io: OutputPin> Output for Interface8Bit<O, Io> {
    fn enable(&mut self) {
        self.e.write(true);
    }

    fn disable(&mut self) {
        self.e.write(false);
    }

    fn select(&mut self, rs: bool, rw: bool) {
        self.rs.write(rs);
        self.rw.write(rw);
    }

    fn write_u8(&mut self, data: u8) {
        for (i, pin) in self.bus.iter_mut().enumerate() {
            pin.write(data & (1 << i) != 0);
        }
        self.latch();
    }
}

impl<O: OutputPin, Io: IoPin> Input for Interface8Bit<O, Io> {
    fn read_u8(&mut self) -> u8 {
        self.latched(|this| {
            this.bus
                .iter_mut()
                .enumerate()
                .map(|(i, pin)| (pin.read() as u8) << i)
                .fold(0, BitOr::bitor)
        })
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

impl<O: Output> Execute for O {
    fn execute(&mut self, command: Command) {
        use Command::*;

        if let Write(data) = command {
            self.select_ram_write();
            self.write_u16(data);
            return;
        }

        self.select_command();

        let [first, second] = command.into_bytes();
        self.write_u8(first);
        if second != 0 {
            self.write_u8(second);
        }
    }
}

impl<I: Input> ExecuteRead for I {
    fn read_bf_ac(&mut self) -> (bool, u8) {
        self.select_bf_ac();
        let read = self.read_u8();
        (read & 0b10000000 != 0, read & 0b01111111)
    }

    fn read(&mut self) -> u16 {
        self.select_ram_read();
        self.read_u16()
    }
}
