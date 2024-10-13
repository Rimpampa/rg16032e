#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::prelude::*;
use st7920::{ext::Execute as ExecuteExt, Execute};

#[entry]
fn main() -> ! {
    let (lcd0, lcd1, delay, _) = rg16032e::setup();
    let mut lcd = rg16032e::Pair(lcd0, lcd1);

    lcd.ddram_addr(0).unwrap();
    for _ in 0..10 {
        lcd.write(u16::from_be_bytes(*b"AA")).unwrap();
    }

    lcd.ddram_addr(0x10).unwrap();
    for _ in 0..10 {
        lcd.write(u16::from_be_bytes(*b"BB")).unwrap();
    }

    lcd.ddram_addr(0x20).unwrap();
    for _ in 0..10 {
        lcd.write(u16::from_be_bytes(*b"CC")).unwrap();
    }

    lcd.ddram_addr(0x30).unwrap();
    for _ in 0..10 {
        lcd.write(u16::from_be_bytes(*b"DD")).unwrap();
    }

    log::info!("DDRAM loaded...");

    lcd.select_extended().unwrap();
    lcd.enable_scroll().unwrap();

    log::info!("Scroll enabled...");

    for scroll in cycleback(0..=0b11111) {
        lcd.scroll_offset(scroll).unwrap();
        delay.delay(200.millis());
    }

    unreachable!()
}

fn cycleback<I>(
    i: impl IntoIterator<IntoIter: Clone + DoubleEndedIterator, Item = I>,
) -> impl Iterator<Item = I> {
    let iter = i.into_iter();
    iter.clone().chain(iter.rev()).cycle()
}
