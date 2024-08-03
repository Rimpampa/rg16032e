use super::Driver;

use esp_hal::gpio::Level;
use fugit::ExtU64;

pub use esp_hal::time::current_time as now;

pub(super) type Duration = fugit::Duration<u64, 1, 1000000>;
pub(super) type Instant = fugit::Instant<u64, 1, 1000000>;

pub(super) fn sleep(duration: Duration) {
    sleep_until(now() + duration)
}

pub fn sleep_until(end: Instant) {
    while now() < end {}
}

impl Driver {
    // pub(super) fn sleep(&mut self, duration: Duration) {
    //     self.sleep_until = now() + duration;
    // }

    pub(super) fn latched<T>(&mut self, f: impl FnOnce(&mut Self) -> T) -> T {
        // sleep_until(self.sleep_until);
        self.e.set_high();
        let result = f(self);
        self.e.set_low();
        // Enable Cycle Time (Tc) min 1200ns
        sleep(10.micros());
        result
    }

    pub(super) fn latch(&mut self) {
        self.latched(|_| {})
    }

    fn select(&mut self, rs: Level, rw: Level) {
        self.rs.set_level(rs);
        self.rw.set_level(rw);
        // Address Setup Time (Tas) min 10ns
        sleep(1.micros());
    }

    pub(super) fn select_command(&mut self) {
        self.select(Level::Low, Level::Low);
    }

    pub(super) fn select_busy_flag(&mut self) {
        self.select(Level::Low, Level::High);
    }

    pub(super) fn select_ram_read(&mut self) {
        self.select(Level::High, Level::High);
    }

    pub(super) fn select_ram_write(&mut self) {
        self.select(Level::High, Level::Low);
    }

    pub(super) fn write_u4(&mut self, nibble: u8) {
        self.bus.write(nibble);
        self.latch()
    }

    pub(super) fn write_u8(&mut self, byte: u8) {
        self.write_u4(byte >> 4);
        self.write_u4(byte & 0xF);
    }

    pub(super) fn write_u16(&mut self, word: u16) {
        self.write_u8((word >> 8) as u8);
        self.write_u8((word & 0xFF) as u8);
    }

    pub(super) fn write_all_u16(&mut self, words: impl IntoIterator<Item = u16>) {
        words.into_iter().for_each(|word| self.write_u16(word))
    }

    pub(super) fn read_u4(&mut self) -> u8 {
        self.latched(|this| this.bus.read())
    }

    pub(super) fn read_u8(&mut self) -> u8 {
        self.read_u4() << 4 | self.read_u4()
    }

    pub(super) fn read_u16(&mut self) -> u16 {
        (self.read_u8() as u16) << 8 | self.read_u8() as u16
    }
}
