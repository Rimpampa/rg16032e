#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::prelude::*;
use st7920::infallible::*;

#[entry]
fn main() -> ! {
    let (mut lcd, delay, mut rng) = rg16032e::setup();

    lcd.cgram_addr(0);
    for _ in 0..4 {
        lcd.write(0b0011001100110011);
        lcd.write(0b0011001100110011);
        lcd.write(0b1100110011001100);
        lcd.write(0b1100110011001100);
    }
    for _ in 0..8 {
        lcd.write(0b0101010101010101);
        lcd.write(0b1010101010101010);
    }
    for _ in 0..4 {
        lcd.write(0b1001100110011001);
        lcd.write(0b0011001100110011);
        lcd.write(0b0110011001100110);
        lcd.write(0b1100110011001100);
    }
    for _ in 0..4 {
        lcd.write(0b1001100110011001);
        lcd.write(0b0011001100110011);
        lcd.write(0b0110011001100110);
        lcd.write(0b1100110011001100);
    }

    lcd.ddram_addr(0);
    loop {
        lcd.write((rng.random() % 4) as u16 * 2);
        //  0  1  2  3  4  5  6  7  8  9 |  a  b  c  d  e  f
        // 10 11 12 13 14 15 16 17 18 19 | 1a 1b 1c 1d 1e 1f
        match lcd.read_address_counter() {
            0xa => lcd.ddram_addr(0x10),
            0x1a => {
                delay.delay(500.millis());
                lcd.ddram_addr(0x0)
            }
            _ => (),
        }
    }
}
