use either::Either::{self, Left, Right};
use embedded_hal::{digital::OutputPin, spi::SpiBus};

use crate::{ext, hal, Command, Execute, SharedBus};

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

struct Pin<Cs> {
    cs: Cs,
    end: hal::Instant,
}

pub struct Interface<Spi, Cs, const PINS: usize> {
    spi: Spi,
    pins: [Pin<Cs>; PINS],
}

impl<Spi, Cs, const PINS: usize> Interface<Spi, Cs, PINS> {
    pub fn new(spi: Spi, cs: [Cs; PINS]) -> Self {
        let end = hal::now();
        let pins = cs.map(|cs| Pin { cs, end });
        Self { spi, pins }
    }
}

impl<Spi, Cs, const PINS: usize> SharedBus for Interface<Spi, Cs, PINS> {
    type Interface<'a>
        = Interface<&'a mut Spi, &'a mut Cs, 1>
    where
        Cs: 'a,
        Spi: 'a;

    fn num(&self) -> usize {
        PINS
    }

    fn get(&mut self, idx: usize) -> Option<Self::Interface<'_>> {
        self.pins.get_mut(idx).map(|Pin { cs, end }| Interface {
            spi: &mut self.spi,
            pins: [Pin { cs, end: *end }],
        })
    }
}

impl<Spi, Cs: OutputPin> Interface<Spi, Cs, 1> {
    pub fn transaction<O, E>(
        &mut self,
        duration: hal::Duration,
        run: impl FnOnce(&mut Spi) -> Result<O, E>,
    ) -> Result<O, Either<E, Cs::Error>> {
        hal::sleep_until(self.pins[0].end);

        self.pins[0].cs.set_high().map_err(Right)?;
        let result = run(&mut self.spi);
        // self.clock.delay(1000);
        self.pins[0].cs.set_low().map_err(Right)?;

        self.pins[0].end = hal::now() + duration;
        result.map_err(Left)
    }
}

impl<Spi: SpiBus, Cs: OutputPin> Execute for Interface<Spi, Cs, 1> {
    type Error = Either<Spi::Error, Cs::Error>;

    fn execute(&mut self, command: Command) -> Result<(), Self::Error> {
        self.transaction(command.execution_time(), |spi| match command {
            Command::Write(data) => spi.write(&encode_u16(1, data)),
            _ => spi.write(&encode_u8(0, command.into_byte())),
        })
    }
}

impl<Spi: SpiBus, Cs: OutputPin> ext::Execute for Interface<Spi, Cs, 1> {
    fn execute_ext(&mut self, command: ext::Command) -> Result<(), Self::Error> {
        self.transaction(command.execution_time(), |spi| match command.into_bytes() {
            [data, 0] => spi.write(&encode_u8(0, data)),
            [h, l] => spi.write(&encode_u16(0, (h as u16) << 8 | l as u16)),
        })
    }
}

impl<Spi: SpiBus, Cs: OutputPin> Execute for &mut Interface<Spi, Cs, 1> {
    type Error = Either<Spi::Error, Cs::Error>;

    fn execute(&mut self, command: Command) -> Result<(), Self::Error> {
        Interface::<Spi, Cs, 1>::execute(self, command)
    }
}

impl<Spi: SpiBus, Cs: OutputPin> ext::Execute for &mut Interface<Spi, Cs, 1> {
    fn execute_ext(&mut self, command: ext::Command) -> Result<(), Self::Error> {
        Interface::<Spi, Cs, 1>::execute_ext(self, command)
    }
}
