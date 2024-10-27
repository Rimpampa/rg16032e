#![no_std]
#![no_main]

use st7920::hal::Rng;

struct FakeRandom(u32);

impl Rng for FakeRandom {
    fn random(&mut self) -> u32 {
        self.0 += 1;
        self.0
    }
}

#[cortex_m_rt::entry]
fn main() -> ! {
    stm32f4::setup!(_c, p, clocks, timer1, timer2);
    examples::cgram::run(stm32f4::serial_lcd!(p, clocks, timer1), timer2, FakeRandom(0)).unwrap()
}
