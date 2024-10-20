use either::Either::{self, Left, Right};
use embedded_hal::{digital::OutputPin, spi::SpiBus};

use crate::{
    ext,
    hal::{HasTimer, Timer},
    Command, Execute,
};

fn sync(rs: u8) -> u8 {
    0b11111000 | rs << 1
}

fn encode_u8(rs: u8, byte: u8) -> [u8; 3] {
    [sync(rs), byte & 0xF0, byte << 4]
}

fn encode_u16(rs: u8, data: u16) -> [u8; 5] {
    let [a, b] = [(data >> 8) as u8, (data & 0xFF) as u8];
    [sync(rs), a & 0xF0, a << 4, b & 0xF0, b << 4]
}

pub struct Interface<Spi, Timer, Cs> {
    pub spi: Spi,
    pub timer: Timer,
    pub cs: Cs,
}

impl<S, T: Timer, C> HasTimer for Interface<S, T, C> {
    fn timer(&mut self) -> &mut impl Timer {
        &mut self.timer
    }
}

impl<S: SpiBus, T: Timer, C: OutputPin> Execute for Interface<S, T, C> {
    type Error = Either<S::Error, C::Error>;

    fn execute(&mut self, command: Command) -> Result<(), Self::Error> {
        self.timer.complete();

        self.cs.set_high().map_err(Right)?;
        match command {
            Command::Write(data) => self.spi.write(&encode_u16(1, data)),
            _ => self.spi.write(&encode_u8(0, command.into_byte())),
        }
        .map_err(Left)?;
        self.cs.set_low().map_err(Right)?;
        self.timer.program(command.execution_time());
        Ok(())
    }
}

impl<S: SpiBus, T: Timer, C: OutputPin> ext::Execute for Interface<S, T, C> {
    fn execute_ext(&mut self, command: ext::Command) -> Result<(), Self::Error> {
        self.timer.complete();

        self.cs.set_high().map_err(Right)?;
        match command.into_bytes() {
            [data, 0] => self.spi.write(&encode_u8(0, data)),
            [h, l] => self.spi.write(&encode_u16(0, (h as u16) << 8 | l as u16)),
        }
        .map_err(Left)?;
        self.cs.set_low().map_err(Right)?;
        self.timer.program(command.execution_time());
        Ok(())
    }
}
