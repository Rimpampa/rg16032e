#![no_std]
#![feature(decl_macro)]

// Print panic message to probe console
use defmt_rtt as _;
use panic_probe as _;

mod flex;
mod now;
pub mod parallel;
pub mod serial;

use stm32f4xx_hal::gpio;

type Input = gpio::AnyPin<gpio::Input>;
type Output = gpio::AnyPin<gpio::Output<gpio::PushPull>>;

#[cfg(all(feature = "serial", not(feature = "two-displays")))]
pub macro lcd($p:expr, $clocks:expr) {{
    let gpioa = ::stm32f4xx_hal::gpio::GpioExt::split($p.GPIOA);

    let mut lcd = $crate::serial::new(
        $p.SPI1,
        gpioa.pa7,
        gpioa.pa5,
        [gpioa.pa9.into_push_pull_output()],
        &$clocks,
    );
    ::st7920::Execute::init(&mut lcd).unwrap();
    ::defmt::info!("Serial LCD initialized...");

    lcd
}}

#[cfg(all(feature = "serial", feature = "two-displays"))]
pub macro lcd($p:expr, $clocks:expr) {{
    let gpioa = ::stm32f4xx_hal::gpio::GpioExt::split($p.GPIOA);

    let mut lcd = $crate::serial::new(
        $p.SPI1,
        gpioa.pa7,
        gpioa.pa5,
        [
            gpioa.pa9.into_push_pull_output().erase(),
            gpioa.pa6.into_push_pull_output().erase(),
        ],
        &$clocks,
    );
    ::st7920::Execute::init(&mut ::st7920::SharedBus::get(&mut lcd, 0).unwrap()).unwrap();
    ::st7920::Execute::init(&mut ::st7920::SharedBus::get(&mut lcd, 1).unwrap()).unwrap();
    ::defmt::info!("Serial LCD initialized...");

    lcd
}}

pub macro setup($cp:ident, $dp:ident, $clocks:ident, $rng:ident) {
    let $cp = ::cortex_m::Peripherals::take().unwrap();
    let $dp = ::stm32f4xx_hal::pac::Peripherals::take().unwrap();

    let rcc = stm32f4xx_hal::rcc::RccExt::constrain($dp.RCC);
    let $clocks = rcc.cfgr.freeze();

    $crate::now::setup($dp.TIM2, &$clocks);

    let $rng = ::rand_mt::Mt::new_unseeded();
}
