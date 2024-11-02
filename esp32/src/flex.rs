use core::ops::{Deref, DerefMut};
use embedded_hal::digital::{ErrorType, InputPin, OutputPin};
use esp_hal::gpio;
use st7920::hal::{InPin, OutPin};

pub struct Flex<'a>(pub gpio::Flex<'a>);

impl<'a> Deref for Flex<'a> {
    type Target = gpio::Flex<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> ErrorType for Flex<'a> {
    type Error = core::convert::Infallible;
}

impl<'a> OutputPin for Flex<'a> {
    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.0.set_high();
        Ok(())
    }

    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.0.set_low();
        Ok(())
    }
}

impl<'a> InputPin for Flex<'a> {
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        Ok(self.0.is_high())
    }

    fn is_low(&mut self) -> Result<bool, Self::Error> {
        Ok(self.0.is_low())
    }
}

impl<'a> DerefMut for Flex<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl InPin for Flex<'_> {
    fn set_as_input(&mut self) -> Result<(), Self::Error> {
        gpio::Flex::set_as_input(&mut self.0, gpio::Pull::None);
        Ok(())
    }
}

impl OutPin for Flex<'_> {
    fn set_as_output(&mut self) -> Result<(), Self::Error> {
        gpio::Flex::set_as_output(&mut self.0);
        Ok(())
    }
}
