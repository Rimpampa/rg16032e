use stm32f4xx_hal::{
    pac::TIM2 as TIM,
    rcc::{BusTimerClock, Clocks, Enable, Reset},
};

pub fn setup(tim: TIM, clocks: &Clocks) {
    // SAFETY: copied from stm32f4xx_hal::timer::FTimer::new
    unsafe {
        TIM::enable_unchecked();
        TIM::reset_unchecked();
    }
    const FREQ: u32 = 1_000_000;

    let clk = TIM::timer_clock(clocks);
    assert!(clk.raw() % FREQ == 0);
    let psc = clk.raw() / FREQ;
    let psc = u16::try_from(psc - 1).unwrap();
    tim.psc().write(|w| w.psc().set(psc));

    // SAFETY: copied from stm32f4xx_hal::timer::monotonic::FTimer::monotonic
    unsafe { tim.arr().write(|w| w.bits(u32::MAX)) };

    // From trigger_update()
    tim.cr1().modify(|_, w| w.urs().set_bit());
    tim.egr().write(|w| w.ug().set_bit());
    tim.cr1().modify(|_, w| w.urs().clear_bit());
    // From start_free(true)
    tim.cr1().modify(|_, w| w.cen().set_bit().udis().bit(false));
}

#[inline(never)]
#[no_mangle]
unsafe fn _st7920_now() -> st7920::hal::Instant {
    // SAFETY: It is safe to read the cnt register, even if the TIM is steal()ed
    // TODO: fact-check this statement
    let cnt = unsafe { TIM::steal().cnt().read().bits() };
    st7920::hal::Instant::from_ticks(cnt as u64)
}
