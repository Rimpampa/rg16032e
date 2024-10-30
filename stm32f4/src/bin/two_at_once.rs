#![no_std]
#![no_main]

#[cortex_m_rt::entry]
fn main() -> ! {
    stm32f4::setup!(_c, p, clocks, rng);
    let lcd = stm32f4::lcd!(p, clocks);
    examples::two_at_once::run(lcd, rng).unwrap()
}
