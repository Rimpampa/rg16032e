#![no_std]
#![no_main]

#[esp_hal::entry]
fn main() -> ! {
    esp32::setup!(p, io, rng);
    let lcd = esp32::lcd!(p, io.pins);
    examples::cgram::run(lcd, esp_hal::time::now(), rng).unwrap()
}
