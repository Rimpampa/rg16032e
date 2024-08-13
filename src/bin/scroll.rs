#![no_std]
#![no_main]
#![feature(associated_type_bounds)]

use esp_backtrace as _;
use esp_hal::prelude::*;

#[entry]
fn main() -> ! {
    let (mut lcd, delay, _) = rg16032e::setup();

    lcd.set_ddram_addr(0);
    for _ in 0..10 {
        lcd.write(u16::from_be_bytes(*b"AA"));
    }

    lcd.set_ddram_addr(0x10);
    for _ in 0..10 {
        lcd.write(u16::from_be_bytes(*b"BB"));
    }

    lcd.set_ddram_addr(0x20);
    for _ in 0..10 {
        lcd.write(u16::from_be_bytes(*b"CC"));
    }

    lcd.set_ddram_addr(0x30);
    for _ in 0..10 {
        lcd.write(u16::from_be_bytes(*b"DD"));
    }

    lcd.set_extended();
    lcd.select_scroll();

    for scroll in cycleback(0..=0b11111) {
        lcd.set_scroll_addr(scroll);
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
