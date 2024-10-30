use fugit::ExtU64;
use st7920::{ext::Execute, hal::sleep};

pub fn run<Lcd: Execute>(mut lcd: Lcd) -> Result<!, Lcd::Error> {
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
        sleep(1.secs());
        lcd.reverse(i)?;
    }

    unreachable!()
}
