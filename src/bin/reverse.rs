#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::prelude::*;

#[entry]
fn main() -> ! {
    let (mut lcd, delay, _) = rg16032e::setup();

    lcd.set_ddram_addr(0);
    for _ in 0..10 {
        lcd.write(u16::from_be_bytes(*b"~*"));
    }

    lcd.set_ddram_addr(0x10);
    for _ in 0..10 {
        lcd.write(0);
    }

    lcd.set_cgram_addr(0);
    for _ in 0..4 {
        lcd.write(0b0011000000110000);
        lcd.write(0b1111000011110000);
        lcd.write(0b1100001111000011);
        lcd.write(0b0000001100000011);
    }

    lcd.set_extended();

    for i in (0..2).cycle() {
        log::info!("REVERSE {i}");
        lcd.reverse(i);
        delay.delay(1000.millis());
        lcd.reverse(i);
    }

    unreachable!()
}
