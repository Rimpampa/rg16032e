//! Super-minimal Hardware Abstraction Layer

/// Generic read pin
pub trait InputPin {
    fn read(&mut self) -> bool;
}

/// Generic output pin
pub trait OutputPin {
    fn write(&mut self, level: bool);
}

/// Generic input/output pin
pub trait IoPin = InputPin + OutputPin;

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
}
