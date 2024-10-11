#![no_std]
#![feature(trait_alias)]

pub mod command;
pub mod hal;
pub mod parallel;

#[cfg(feature = "esp")]
pub mod esp;

pub trait Init {
    type Error;

    fn init(&mut self) -> Result<(), Self::Error>;
}

impl<T: command::Execute + hal::HasTimer> Init for T {
    type Error = T::Error;

    fn init(&mut self) -> Result<(), Self::Error> {
        use hal::Timer;
        self.timer().delay(80_000);
        self.select_basic()?;
        self.timer().delay(200);
        self.select_basic()?;
        self.timer().delay(200);
        self.display_on_off(true, false, false)?;
        self.timer().delay(200);
        self.clear()?;
        self.timer().delay(20_000);
        self.entry_mode(true, false)
    }
}
