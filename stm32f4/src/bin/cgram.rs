#![no_std]
#![no_main]

#[cortex_m_rt::entry]
fn main() -> ! {
    stm32f4::setup!(_c, p, clocks, clock, rng);
    let lcd = stm32f4::lcd!(p, clocks, clock);
    examples::cgram::run(lcd, clock, rng).unwrap()
}
