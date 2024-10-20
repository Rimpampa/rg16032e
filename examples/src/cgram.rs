use st7920::{
    hal::{Rng, Timer},
    Execute,
};

pub fn run<Lcd: Execute>(mut lcd: Lcd, mut timer: impl Timer, mut rng: impl Rng) -> Result<!, Lcd::Error> {
    lcd.cgram_addr(0)?;
    for _ in 0..4 {
        lcd.write(0b0011001100110011)?;
        lcd.write(0b0011001100110011)?;
        lcd.write(0b1100110011001100)?;
        lcd.write(0b1100110011001100)?;
    }
    for _ in 0..8 {
        lcd.write(0b0101010101010101)?;
        lcd.write(0b1010101010101010)?;
    }
    for _ in 0..4 {
        lcd.write(0b1001100110011001)?;
        lcd.write(0b0011001100110011)?;
        lcd.write(0b0110011001100110)?;
        lcd.write(0b1100110011001100)?;
    }
    for _ in 0..4 {
        lcd.write(0b1001100110011001)?;
        lcd.write(0b0011001100110011)?;
        lcd.write(0b0110011001100110)?;
        lcd.write(0b1100110011001100)?;
    }

    lcd.ddram_addr(0)?;
    let indices = [0, 0x10].map(|s| s..=s + 0xa).into_iter().flatten();
    for address in indices.cycle() {
        lcd.write((rng.random() % 4) as u16 * 2)?;
        match address {
            0xa => lcd.ddram_addr(0x10)?,
            0x1a => {
                timer.delay(500_000);
                lcd.ddram_addr(0x0)?
            }
            _ => (),
        }
    }

    unreachable!()
}
