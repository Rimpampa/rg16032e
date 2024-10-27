#![no_std]
#![no_main]

#[cortex_m_rt::entry]
fn main() -> ! {
    stm32f4::setup!(_c, p, clocks, timer, _timer);
    let lcd = stm32f4::lcd!(p, clocks, timer1);
    examples::verify::run(lcd).unwrap()
}
