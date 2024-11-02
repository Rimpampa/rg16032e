use super::{Input, Output};
use embedded_hal::digital::{ErrorType, InputPin, OutputPin};
use st7920::hal::{InPin, OutPin};

pub enum Flex {
    Input(Input),
    Output(Output),
    None,
}

impl Flex {
    pub fn new(pin: impl Into<Input>) -> Self {
        let mut pin: Output = pin.into().into_mode();
        pin.set_low();
        Self::Output(pin)
    }

    fn take(&mut self) -> Self {
        core::mem::replace(self, Self::None)
    }
}

impl ErrorType for Flex {
    type Error = core::convert::Infallible;
}

impl InputPin for Flex {
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        self.set_as_input()?;
        let Self::Input(pin) = self else {
            unreachable!()
        };
        pin.is_high()
    }

    fn is_low(&mut self) -> Result<bool, Self::Error> {
        self.set_as_input()?;
        let Self::Input(pin) = self else {
            unreachable!()
        };
        pin.is_low()
    }
}

impl InPin for Flex {
    fn set_as_input(&mut self) -> Result<(), Self::Error> {
        *self = match self.take() {
            pin @ Self::Input(_) => pin,
            Self::Output(pin) => Self::Input(pin.into_mode()),
            Self::None => unreachable!(),
        };
        Ok(())
    }
}

impl OutputPin for Flex {
    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.set_as_output()?;
        let Self::Output(pin) = self else {
            unreachable!()
        };
        pin.set_low();
        Ok(())
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.set_as_output()?;
        let Self::Output(pin) = self else {
            unreachable!()
        };
        pin.set_high();
        Ok(())
    }
}

impl OutPin for Flex {
    fn set_as_output(&mut self) -> Result<(), Self::Error> {
        *self = match self.take() {
            pin @ Self::Output(_) => pin,
            Self::Input(pin) => Self::Output(pin.into_mode()),
            Self::None => unreachable!(),
        };
        Ok(())
    }
}
