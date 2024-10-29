#![no_std]
#![no_main]

#[cortex_m_rt::entry]
fn main() -> ! {
    stm32f4::setup!(_c, p, clocks, clock, _rng);
    let lcd = stm32f4::lcd!(p, clocks, clock);
    examples::scroll::run(lcd, clock).unwrap()
}
