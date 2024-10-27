use st7920::{
    ext::Execute as ExecuteExt,
    hal::{Rng, Timer},
    SharedBus,
};

pub fn run<Lcd, T, E>(mut lcd: Lcd, mut delay: impl Timer, mut rng: impl Rng) -> Result<!, E>
where
    Lcd: SharedBus,
    for<'a> Lcd::Interface<'a>: ExecuteExt<Error = E>,
{
    super::cgram::setup(lcd.get(0).unwrap())?;
    super::scroll::setup(lcd.get(1).unwrap())?;

    let mut counter = 0;
    loop {
        super::cgram::step(lcd.get(0).unwrap(), &mut rng)?;
        super::scroll::step(lcd.get(1).unwrap(), &mut counter)?;

        delay.delay(200_000);
    }
}
