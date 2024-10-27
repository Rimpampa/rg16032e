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
    stm32h7::setup!(_c, p, prec, clocks, timer1, timer2);
    let lcd = stm32h7::lcd!(p, prec, clocks, timer1);
    examples::cgram::run(lcd, timer2, FakeRandom(0)).unwrap()
}
