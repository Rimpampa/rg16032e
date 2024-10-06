#![no_std]
#![feature(trait_alias)]

pub mod command;
pub mod hal;
pub mod parallel;

pub use command::{Execute, ExecuteRead};

#[cfg(feature = "esp")]
pub mod esp;

pub struct Driver<Bus, Timer> {
    pub bus: Bus,
    pub timer: Timer,
}

impl<B: Execute, T: hal::Timer> Driver<B, T> {
    pub fn init(&mut self) -> Result<(), B::Error> {
        self.timer.program(80_000);
        self.bus.select_basic()?;
        self.timer.program(200);
        self.bus.select_basic()?;
        self.timer.program(200);
        self.bus.display_on_off(true, false, false)?;
        self.timer.program(200);
        self.bus.clear()?;
        self.timer.program(20_000);
        self.bus.entry_mode(true, false)
    }
}

impl<B: Execute, T: hal::Timer> Execute for Driver<B, T> {
    type Error = B::Error;

    fn execute(&mut self, command: command::Command) -> Result<(), B::Error> {
        self.timer.complete();
        self.bus.execute(command)?;
        self.timer.program(command.execution_time());
        Ok(())
    }
}

impl<B: ExecuteRead, T: hal::Timer> ExecuteRead for Driver<B, T> {
    type Error = B::Error;

    fn read(&mut self) -> Result<u16, B::Error> {
        self.timer.complete();
        self.bus.read()
    }

    fn read_bf_ac(&mut self) -> Result<(bool, u8), B::Error> {
        self.timer.complete();
        self.bus.read_bf_ac()
    }
}
