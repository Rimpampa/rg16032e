#![no_std]
#![no_main]

use core::array;

use esp_backtrace as _;
use esp_hal::{gpio::Io, peripherals::Peripherals, prelude::*};

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);

    esp_println::logger::init_logger_from_env();

    let mut lcd = st7920::Driver::setup(
        io.pins.gpio27,
        io.pins.gpio14,
        io.pins.gpio13,
        st7920::bus::Bus::new(
            io.pins.gpio32,
            io.pins.gpio33,
            io.pins.gpio25,
            io.pins.gpio26,
        ),
    );

    lcd.display_on_off(true, false, true);

    let mut byte = b'0';
    let mut read = false;
    loop {
        let [addr, rest @ ..] = array::from_fn::<_, 10, _>(|_| lcd.read_address_counter().1);
        if rest.iter().any(|a| *a != addr) {
            log::error!("AC READ! {addr} != {rest:?}");

            lcd.init();
            lcd.display_on_off(true, false, true);
            continue;
        }

        let data = (byte as u16) << 8 | byte as u16;
        if read {
            let check = lcd.read();
            if check != data {
                log::error!("RAM W/R! 0x{check:04x} != 0x{data:04x} @ 0x{addr:02x}");

                lcd.init();
                lcd.display_on_off(true, false, true);
                read = false;
                continue;
            }
        } else {
            lcd.write(data);
        }

        let new = lcd.read_address_counter().1;
        if new != addr + 1 {
            log::error!("AC INCREMENT! 0x{new:02x} != {:02x}", addr + 1);

            lcd.init();
            lcd.display_on_off(true, false, true);
            continue;
        }

        //  0  1  2  3  4  5  6  7  8  9 |  a  b  c  d  e  f
        // 10 11 12 13 14 15 16 17 18 19 | 1a 1b 1c 1d 1e 1f
        match addr {
            0x9 => lcd.set_ddram_addr(0x10),
            0x19 => {
                if read {
                    byte = match byte {
                        b'9' => b'A',
                        b'F' => b'0',
                        _ => byte + 1,
                    };
                    log::info!("next");
                } else {
                    log::info!("now read");
                }
                lcd.set_ddram_addr(0x0);
                read = !read;
            }
            _ => (),
        }
    }
}
