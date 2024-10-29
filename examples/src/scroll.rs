use st7920::{ext::Execute, hal::Clock};
use fugit::ExtU64;

pub fn run<Lcd, E>(mut lcd: Lcd, clock: impl Clock) -> Result<!, E>
where
    for<'a> &'a mut Lcd: Execute<Error = E>,
{
    setup(&mut lcd)?;

    let mut counter = 0;
    loop {
        step(&mut lcd, &mut counter)?;
        clock.wait(200.millis());
    }
}

pub fn setup<Lcd: Execute>(mut lcd: Lcd) -> Result<(), Lcd::Error> {
    lcd.ddram_addr(0)?;
    for _ in 0..10 {
        lcd.write(u16::from_be_bytes(*b"AA"))?;
    }

    lcd.ddram_addr(0x10)?;
    for _ in 0..10 {
        lcd.write(u16::from_be_bytes(*b"BB"))?;
    }

    lcd.ddram_addr(0x20)?;
    for _ in 0..10 {
        lcd.write(u16::from_be_bytes(*b"CC"))?;
    }

    lcd.ddram_addr(0x30)?;
    for _ in 0..10 {
        lcd.write(u16::from_be_bytes(*b"DD"))?;
    }

    log::info!("DDRAM loaded...");

    lcd.select_extended()?;
    lcd.enable_scroll()?;

    log::info!("Scroll enabled...");

    Ok(())
}

pub fn step<Lcd: Execute>(mut lcd: Lcd, counter: &mut u8) -> Result<(), Lcd::Error> {
    lcd.scroll_offset(*counter & 0b011111)?;

    match *counter & 0b100000 {
        0b100000 => match *counter {
            0b100000 => *counter = 1,
            _ => *counter -= 1,
        },
        _ => match *counter {
            0b011111 => *counter = 0b111110,
            _ => *counter += 1,
        }
    }

    Ok(())
}
