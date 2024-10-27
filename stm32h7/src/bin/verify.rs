#![no_std]
#![no_main]

#[cortex_m_rt::entry]
fn main() -> ! {
    stm32h7::setup!(_c, p, clocks, timer, _timer);
    let lcd = stm32h7::lcd!(p, clocks, timer1);
    examples::verify::run(lcd).unwrap()
}
