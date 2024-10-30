//! Super-minimal Hardware Abstraction Layer

pub use embedded_hal::digital::{ErrorType, InputPin, OutputPin};

pub type Instant = fugit::Instant<u64, 1, 1_000_000>;
pub type Duration = fugit::Duration<u64, 1, 1_000_000>;

extern "Rust" {
    fn _st7920_now() -> Instant;
}

pub fn now() -> Instant {
    unsafe { _st7920_now() }
}

pub fn sleep_until(end: Instant) {
    while now() < end {}
}

pub fn sleep(duration: impl Into<Duration>) {
    sleep_until(now() + duration.into());
}

/// Generic output pin
pub trait OutPin: ErrorType + OutputPin {
    fn set_as_output(&mut self) -> Result<(), Self::Error>;
}

/// Generic input pin
pub trait InPin: ErrorType + InputPin {
    fn set_as_input(&mut self) -> Result<(), Self::Error>;
}

/// Generic input/output pin
pub trait IoPin = InPin + OutPin;
