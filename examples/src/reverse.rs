use st7920::{ext::Execute, hal::Timer};

pub fn run<Lcd: Execute>(mut lcd: Lcd, mut delay: impl Timer) -> Result<!, Lcd::Error> {
    lcd.ddram_addr(0)?;
    for _ in 0..10 {
        lcd.write(u16::from_be_bytes(*b"~*"))?;
    }

    lcd.ddram_addr(0x10)?;
    for _ in 0..10 {
        lcd.write(0)?;
    }

    lcd.cgram_addr(0)?;
    for _ in 0..4 {
        lcd.write(0b0011000000110000)?;
        lcd.write(0b1111000011110000)?;
        lcd.write(0b1100001111000011)?;
        lcd.write(0b0000001100000011)?;
    }

    lcd.select_extended()?;

    for i in (0..2).cycle() {
        log::info!("REVERSE {i}");
        lcd.reverse(i)?;
        delay.delay(1000_000);
        lcd.reverse(i)?;
    }

    unreachable!()
}
