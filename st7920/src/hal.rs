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

/// A count-down [`Timer`]
pub trait Timer {
    /// Program the [`Timer`] to run for the given amount of microseconds
    fn program(&mut self, duration: u32);
    /// Returns whether or not the [`Timer`] has expired
    ///
    /// When this function returns `true` it means that the amount of time
    /// that was [`program`]med has elapsed (or that it was never started).
    fn expired(&mut self) -> bool;
    /// Wait until the timer expires
    fn complete(&mut self) {
        while !self.expired() {}
    }
    /// Wait for the given amount of microseconds
    fn delay(&mut self, duration: u32) {
        self.program(duration);
        self.complete();
    }
}

impl<T: Timer> Timer for &mut T {
    fn program(&mut self, duration: u32) {
        T::program(self, duration);
    }

    fn expired(&mut self) -> bool {
        T::expired(self)
    }
}

pub trait HasTimer {
    fn timer(&mut self) -> &mut impl Timer;
}

impl<T: HasTimer> HasTimer for &mut T {
    fn timer(&mut self) -> &mut impl Timer {
        T::timer(self)
    }
}

pub trait Rng {
    fn random(&mut self) -> u32;
}

impl<T: Rng> Rng for &mut T {
    fn random(&mut self) -> u32 {
        T::random(self)
    }
}
