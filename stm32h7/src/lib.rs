#![no_std]
#![feature(decl_macro)]

// Print panic message to probe console
use defmt_rtt as _;
use panic_probe as _;

#[cfg(all(feature = "serial", not(feature = "two-displays")))]
pub macro lcd($p:expr, $prec:expr, $clocks:expr, $timer:expr) {{
    let gpiof = ::stm32h7xx_hal::gpio::GpioExt::split($p.GPIOF, $prec.GPIOF);

// SPI5 PF7=SCK PF9/11=MOSI
    let mut lcd = ::st7920::stm32h7::serial(
        $p.SPI5,
        gpiof.pf9.into_alternate(),
        gpiof.pf7.into_alternate(),
        [gpiof.pf8.into_push_pull_output()],
        $prec.SPI5,
        &$clocks,
        $timer,
    );
    ::st7920::Init::init(&mut lcd).unwrap();
    ::defmt::info!("Serial LCD initialized...");

    lcd
}}

#[cfg(all(feature = "serial", feature = "two-displays"))]
pub macro lcd($p:expr, $prec:expr, $clocks:expr, $timer:expr) {{
    let gpiof = ::stm32h7xx_hal::gpio::GpioExt::split($p.GPIOF, $prec.GPIOF);

    let mut lcd = ::st7920::stm32h7::serial(
        $p.SPI5,
        gpiof.pf9.into_alternate(),
        gpiof.pf7.into_alternate(),
        [
            gpiof.pf8.into_push_pull_output().erase(),
            gpioc.pf0.into_push_pull_output().erase(),
        ],
        $prec.SPI5,
        &$clocks,
        $timer,
    );
    ::st7920::Init::init(&mut ::st7920::SharedBus::get(&mut lcd, 0).unwrap()).unwrap();
    ::st7920::Init::init(&mut ::st7920::SharedBus::get(&mut lcd, 1).unwrap()).unwrap();
    ::defmt::info!("Serial LCD initialized...");

    lcd
}}

pub macro setup($cp:ident, $dp:ident, $prec:ident, $clocks:ident, $tim1:ident, $tim2:ident) {
    use ::stm32h7xx_hal::prelude::*; // TODO: use fugit

    let $cp = ::cortex_m::Peripherals::take().unwrap();
    let $dp = ::stm32h7xx_hal::pac::Peripherals::take().unwrap();
    
    let pwr = ::stm32h7xx_hal::pwr::PwrExt::constrain($dp.PWR).freeze();

    let rcc = ::stm32h7xx_hal::rcc::RccExt::constrain($dp.RCC);
    let ccdr = rcc.sys_ck(100.MHz()).freeze(pwr, &$dp.SYSCFG);

    let $prec = ccdr.peripheral;
    let $clocks = ccdr.clocks;
    
    let tim2 = ::stm32h7xx_hal::timer::TimerExt::tick_timer($dp.TIM2, 1.MHz(), $prec.TIM2, &$clocks);
    // let $tim2 = ::stm32h7xx_hal::timer::TimerExt::tick_timer($dp.TIM5, 1.MHz(), $prec.TIM5, &$clocks);

    let $tim1 = (&tim2, 0);
    let $tim2 = (&tim2, 0);
}
