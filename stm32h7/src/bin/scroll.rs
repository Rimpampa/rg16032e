#![no_std]
#![no_main]

#[cortex_m_rt::entry]
fn main() -> ! {
    stm32h7::setup!(_c, p, clocks, timer1, timer2);
    let lcd = stm32h7::lcd!(p, clocks, timer1);
    examples::scroll::run(lcd, timer2).unwrap()
}
