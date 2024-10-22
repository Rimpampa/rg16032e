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

pub struct Interface<Spi, Timer, Cs, const PINS: usize> {
    pub spi: Spi,
    pub timer: Timer,
    pub cs: [Cs; PINS],
}

pub type SingleInterface<'a, S, T, C> = Interface<&'a mut S, &'a mut T, &'a mut C, 1>;

impl<S, T, C, const P: usize> Interface<S, T, C, P> {
    pub fn get(&mut self, idx: usize) -> Option<Interface<&mut S, &mut T, &mut C, 1>> {
        self.cs.get_mut(idx).map(|cs| Interface {
            spi: &mut self.spi,
            timer: &mut self.timer,
            cs: [cs]
        })
    }
}

impl<S, T: Timer, C: OutputPin> Interface<S, T, C, 1> {
    pub fn transaction<O, E>(&mut self, run: impl FnOnce(&mut S) -> Result<O, E>) -> Result<O, Either<E, C::Error>> {
        self.cs[0].set_high().map_err(Right)?;
        let result = run(&mut self.spi);
        self.timer.delay(1000);
        self.cs[0].set_low().map_err(Right)?;
        result.map_err(Left)
    }
}

impl<S, T: Timer, C, const P: usize> HasTimer for Interface<S, T, C, P> {
    fn timer(&mut self) -> &mut impl Timer {
        &mut self.timer
    }
}

impl<S: SpiBus, T: Timer, C: OutputPin> Execute for Interface<S, T, C, 1> {
    type Error = Either<S::Error, C::Error>;

    fn execute(&mut self, command: Command) -> Result<(), Self::Error> {
        self.timer.complete();

        self.transaction(|spi| match command {
            Command::Write(data) => spi.write(&encode_u16(1, data)),
            _ => spi.write(&encode_u8(0, command.into_byte())),
        })?;
        self.timer.program(command.execution_time());
        Ok(())
    }
}

impl<S: SpiBus, T: Timer, C: OutputPin> ext::Execute for Interface<S, T, C, 1> {
    fn execute_ext(&mut self, command: ext::Command) -> Result<(), Self::Error> {
        self.timer.complete();

        self.transaction(|spi| match command.into_bytes() {
            [data, 0] => spi.write(&encode_u8(0, data)),
            [h, l] => spi.write(&encode_u16(0, (h as u16) << 8 | l as u16)),
        })?;
        self.timer.program(command.execution_time());
        Ok(())
    }
}
