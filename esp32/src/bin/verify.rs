#![no_std]
#![no_main]

#[esp_hal::entry]
fn main() -> ! {
    esp32::setup!(p, io, _rng);
    let lcd = esp32::lcd!(p, io.pins);
    examples::verify::run(lcd).unwrap()
}
