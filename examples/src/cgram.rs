use fugit::ExtU64;
use rand_core::RngCore;
use st7920::{hal::sleep, Execute};

pub fn run<Lcd, E>(mut lcd: Lcd, mut rng: impl RngCore) -> Result<!, E>
where
    for<'a> &'a mut Lcd: Execute<Error = E>,
{
    setup(&mut lcd)?;

    loop {
        step(&mut lcd, &mut rng)?;
        sleep(500.millis());
    }
}

pub fn setup<Lcd: Execute>(mut lcd: Lcd) -> Result<(), Lcd::Error> {
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

    Ok(())
}

pub fn step<Lcd: Execute>(mut lcd: Lcd, mut rng: impl RngCore) -> Result<(), Lcd::Error> {
    lcd.ddram_addr(0)?;
    for _ in 0..=0xa {
        lcd.write((rng.next_u32() % 4) as u16 * 2)?;
    }
    lcd.ddram_addr(0x10)?;
    for _ in 0..=0xa {
        lcd.write((rng.next_u32() % 4) as u16 * 2)?;
    }
    Ok(())
}
