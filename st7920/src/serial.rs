use embedded_hal::spi::SpiBus;

use crate::{
    ext,
    hal::{HasTimer, Timer},
    Command, Execute,
};

fn sync(rw: u8, rs: u8) -> u8 {
    0b11111000 | rw << 2 | rs << 1
}

fn encode_u8(rw: u8, rs: u8, byte: u8) -> [u8; 3] {
    [sync(rw, rs), byte & 0xF0, byte << 4]
}

fn encode_u16(rw: u8, rs: u8, data: u16) -> [u8; 5] {
    let [a, b] = [(data >> 8) as u8, (data & 0xFF) as u8];
    [sync(rw, rs), a & 0xF0, a << 4, b & 0xF0, b << 4]
}

pub struct Interface<Spi, Timer> {
    pub spi: Spi,
    pub timer: Timer,
}

impl<S, T: Timer> HasTimer for Interface<S, T> {
    fn timer(&mut self) -> &mut impl Timer {
        &mut self.timer
    }
}

impl<S: SpiBus, T: Timer> Execute for Interface<S, T> {
    type Error = S::Error;

    fn execute(&mut self, command: Command) -> Result<(), Self::Error> {
        self.timer.complete();

        if let Command::Write(data) = command {
            self.spi.write(&encode_u16(1, 0, data))?;
            self.timer.program(72);
        } else {
            self.spi.write(&encode_u8(0, 0, command.into_byte()))?;
            self.timer.program(command.execution_time());
        }
        Ok(())
    }
}

impl<S: SpiBus, T: Timer> ext::Execute for Interface<S, T> {
    fn execute_ext(&mut self, command: ext::Command) -> Result<(), Self::Error> {
        self.timer.complete();

        let [first, second] = command.into_bytes();
        if second == 0 {
            self.spi.write(&encode_u8(0, 0, first))?;
        } else {
            let data = (first as u16) << 8 | second as u16;
            self.spi.write(&encode_u16(0, 0, data))?;
        }
        self.timer.program(command.execution_time());
        Ok(())
    }
}
