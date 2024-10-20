#![no_std]
#![no_main]

#[cortex_m_rt::entry]
fn main() -> ! {
    stm32f4::setup!(_c, p, clocks, timer1, timer2);
    examples::scroll::run(stm32f4::serial_lcd!(p, clocks, timer1), timer2).unwrap()
}
