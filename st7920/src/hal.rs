//! Super-minimal Hardware Abstraction Layer

pub use embedded_hal::digital::{ErrorType, InputPin, OutputPin};

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

/// A [`StopWatch`] measures time (in microseconds) since when it's started
///
/// When it's [`start`]ed the time starts counting up and
/// the stopwatch keeps track of how much has [`elapsed`] since then.
pub trait StopWatch {
    /// Start measuring time from now
    fn start(&mut self);
    /// Return the number of microseconds elapsed since the last call to [`start()`]
    ///
    /// If [`start()`] was never called the return value is implementation defined
    /// and could be any.
    fn elapsed(&mut self) -> u32;

    /// Waits until the given duration of time has elapsed since the
    /// last call to [`start()`]
    fn wait(&mut self, duration: u32) {
        while self.elapsed() < duration {}
    }
}

pub type Instant = fugit::Instant<u64, 1, 1_000_000>;
pub type Duration = fugit::Duration<u64, 1, 1_000_000>;

pub trait Clock: Copy {
    fn now(self) -> Instant;

    fn wait_until(self, end: Instant) {
        while self.now() < end {}
    }

    fn wait(self, duration: impl Into<Duration>) {
        self.wait_until(self.now() + duration.into());
    }
}
