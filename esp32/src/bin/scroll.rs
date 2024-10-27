#![no_std]
#![no_main]

#[esp_hal::entry]
fn main() -> ! {
    esp32::setup!(p, io, _rng);
    let lcd = esp32::lcd!(p, io.pins);
    examples::scroll::run(lcd, esp_hal::time::now()).unwrap()
}
