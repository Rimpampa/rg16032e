use fugit::ExtU64;
use rand_core::RngCore;
use st7920::{ext::Execute as ExecuteExt, hal::sleep, SharedBus};

pub fn run<Lcd, E>(mut lcd: Lcd, mut rng: impl RngCore) -> Result<!, E>
where
    Lcd: SharedBus,
    for<'a> Lcd::Interface<'a>: ExecuteExt<Error = E>,
{
    super::cgram::setup(lcd.get(0).unwrap())?;
    super::scroll::setup(lcd.get(1).unwrap())?;

    let mut counter = 0;
    loop {
        super::cgram::step(lcd.get(0).unwrap(), &mut rng)?;
        for _ in 0..5 {
            super::scroll::step(lcd.get(1).unwrap(), &mut counter)?;
            sleep(150.millis());
        }
    }
}
