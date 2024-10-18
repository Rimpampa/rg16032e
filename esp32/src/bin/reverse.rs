#![no_std]
#![no_main]

#[esp_hal::entry]
fn main() -> ! {
    esp32::setup!(_p, io, _rng);
    examples::reverse::run(esp32::parallel_lcd!(io.pins), esp_hal::time::now()).unwrap()
}
