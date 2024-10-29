#![no_std]
#![feature(decl_macro)]

// Print panic message to probe console
use defmt_rtt as _;
use panic_probe as _;

#[cfg(all(feature = "serial", not(feature = "two-displays")))]
pub macro lcd($p:expr, $clocks:expr, $clock:expr) {{
    let gpioa = ::stm32f4xx_hal::gpio::GpioExt::split($p.GPIOA);

    let mut lcd = ::st7920::stm32f4::serial(
        $p.SPI1,
        gpioa.pa7,
        gpioa.pa5,
        [gpioa.pa9.into_push_pull_output()],
        &$clocks,
        $clock,
    );
    ::st7920::Execute::init(&mut lcd).unwrap();
    ::defmt::info!("Serial LCD initialized...");

    lcd
}}

#[cfg(all(feature = "serial", feature = "two-displays"))]
pub macro lcd($p:expr, $clocks:expr, $clock:expr) {{
    let gpioa = ::stm32f4xx_hal::gpio::GpioExt::split($p.GPIOA);

    let mut lcd = ::st7920::stm32f4::serial(
        $p.SPI1,
        gpioa.pa7,
        gpioa.pa5,
        [
            gpioa.pa9.into_push_pull_output().erase(),
            gpioa.pa6.into_push_pull_output().erase(),
        ],
        &$clocks,
        $clock,
    );
    ::st7920::Execute::init(&mut ::st7920::SharedBus::get(&mut lcd, 0).unwrap()).unwrap();
    ::st7920::Execute::init(&mut ::st7920::SharedBus::get(&mut lcd, 1).unwrap()).unwrap();
    ::defmt::info!("Serial LCD initialized...");

    lcd
}}

pub macro setup($cp:ident, $dp:ident, $clocks:ident, $clock:ident, $rng:ident) {
    let $cp = ::cortex_m::Peripherals::take().unwrap();
    let $dp = ::stm32f4xx_hal::pac::Peripherals::take().unwrap();

    let rcc = stm32f4xx_hal::rcc::RccExt::constrain($dp.RCC);
    let $clocks = rcc.cfgr.freeze();

    let $clock = ::st7920::stm32f4::Clock::new($dp.TIM2, &$clocks);

    let $rng = ::rand_mt::Mt::new_unseeded();
}
