#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::prelude::*;
use st7920::ext::Execute as ExtExecute;
use st7920::*;

#[entry]
fn main() -> ! {
    let (lcd0, lcd1, delay, _) = rg16032e::setup();
    let mut lcd = rg16032e::Pair(lcd0, lcd1);

    lcd.ddram_addr(0).unwrap();
    for _ in 0..10 {
        lcd.write(u16::from_be_bytes(*b"~*")).unwrap();
    }

    lcd.ddram_addr(0x10).unwrap();
    for _ in 0..10 {
        lcd.write(0).unwrap();
    }

    lcd.cgram_addr(0).unwrap();
    for _ in 0..4 {
        lcd.write(0b0011000000110000).unwrap();
        lcd.write(0b1111000011110000).unwrap();
        lcd.write(0b1100001111000011).unwrap();
        lcd.write(0b0000001100000011).unwrap();
    }

    lcd.select_extended().unwrap();

    for i in (0..2).cycle() {
        log::info!("REVERSE {i}");
        lcd.reverse(i).unwrap();
        delay.delay(1000.millis());
        lcd.reverse(i).unwrap();
    }

    unreachable!()
}
