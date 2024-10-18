use st7920::{ext::Execute, hal::Timer};

pub fn run<Lcd: Execute>(mut lcd: Lcd, mut delay: impl Timer) -> Result<!, Lcd::Error> {
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

    for scroll in cycleback(0..=0b11111) {
        lcd.scroll_offset(scroll)?;
        delay.delay(200_000);
    }

    unreachable!()
}

fn cycleback<I>(
    i: impl IntoIterator<IntoIter: Clone + DoubleEndedIterator, Item = I>,
) -> impl Iterator<Item = I> {
    let iter = i.into_iter();
    iter.clone().chain(iter.rev()).cycle()
}
