#![no_std]
#![feature(decl_macro)]

// Print panic message to probe console
use defmt_rtt as _;
use panic_probe as _;

pub macro serial_lcd($p:expr, $clocks:expr, $timer:expr) {{
    let gpioa = ::stm32f4xx_hal::gpio::GpioExt::split($p.GPIOA);

    let mut lcd = ::st7920::stm32f4::serial(
        $p.SPI1,
        gpioa.pa7,
        gpioa.pa5,
        [gpioa.pa9.into_push_pull_output()],
        &$clocks,
        $timer,
    );
    ::st7920::Init::init(&mut lcd).unwrap();
    ::defmt::info!("Serial LCD initialized...");

    lcd
}}

pub macro setup($cp:ident, $dp:ident, $clocks:ident, $syst:ident, $tim1:ident) {
    let $cp = ::cortex_m::Peripherals::take().unwrap();
    let $dp = ::stm32f4xx_hal::pac::Peripherals::take().unwrap();

    let rcc = stm32f4xx_hal::rcc::RccExt::constrain($dp.RCC);
    let $clocks = rcc.cfgr.freeze();

    let $syst = ::stm32f4xx_hal::timer::Timer::syst($cp.SYST, &$clocks).counter_us();
    let $tim1 = ::stm32f4xx_hal::timer::TimerExt::counter_us($dp.TIM2, &$clocks);
}