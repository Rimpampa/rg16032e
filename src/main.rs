#![no_std]
#![no_main]
#![feature(trait_alias)]
#![feature(associated_type_bounds)]

use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl, delay::Delay, gpio::{AnyOutput, Io, Level}, peripherals::Peripherals, prelude::*, system::SystemControl
};

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = SystemControl::new(peripherals.SYSTEM);

    let clocks = ClockControl::max(system.clock_control).freeze();
    let delay = Delay::new(&clocks);
    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);

    esp_println::logger::init_logger_from_env();

    let mut rs = AnyOutput::new(io.pins.gpio27, Level::Low);
    let mut rw = AnyOutput::new(io.pins.gpio14, Level::Low);
    let mut e = AnyOutput::new(io.pins.gpio13, Level::Low);
    let mut db4 = AnyOutput::new(io.pins.gpio32, Level::Low);
    let mut db5 = AnyOutput::new(io.pins.gpio33, Level::Low);
    let mut db6 = AnyOutput::new(io.pins.gpio25, Level::Low);
    let mut db7 = AnyOutput::new(io.pins.gpio26, Level::Low);

    let mut latch = || {
        e.set_high();
        e.set_low();
        delay.delay(2.millis());
    };

    let mut write = |nibble: u8| {
        let bit = |mask| (nibble & mask) != 0;
        db4.set_level(bit(0b0001).into());
        db5.set_level(bit(0b0010).into());
        db6.set_level(bit(0b0100).into());
        db7.set_level(bit(0b1000).into());
    };

    delay.delay(40.millis());

    rs.set_low();
    rw.set_low();
    delay.delay(10.nanos());

    // Function set
    write(0b0010);
    latch();
    delay.delay(100.micros());

    // Function set
    write(0b0010);
    latch();
    write(0b0000);
    latch();
    delay.delay(100.micros());

    // Display ON/OFF control
    write(0b0000);
    latch();
    write(0b1110);
    latch();
    delay.delay(100.micros());

    // Display clear
    write(0b0000);
    latch();
    write(0b0001);
    latch();
    delay.delay(10.millis());

    // Entry mode set
    write(0b0000);
    latch();
    write(0b0110);
    latch();
    delay.delay(72.micros());

    let mut i = 0u8;
    let mut byte = b'0';
    loop {
        let a = byte;
        byte = match byte {
            b'9' => b'A',
            b'F' => b'0',
            _ => byte + 1,
        };

        let b = byte;
        byte = match byte {
            b'9' => b'A',
            b'F' => b'0',
            _ => byte + 1,
        };

        // Set DDRAM address
        rs.set_low();
        rw.set_low();
        delay.delay(10.nanos());
        write(0b1000 | (i >> 4));
        latch();
        write(i & 0b1111);
        latch();
        delay.delay(72.micros());

        // Write RAM
        rs.set_high();
        rw.set_low();
        delay.delay(10.nanos());
        write(a >> 4);
        latch();
        write(a & 0b1111);
        latch();
        write(b >> 4);
        latch();
        write(b & 0b1111);
        latch();
        delay.delay(72.micros());

        log::info!("Wrote {:?} and {:?} at 0x{i:02x}", a as char, b as char);

        i = i.wrapping_add(1);

        delay.delay(100.millis());
    }
}
