#![no_std]
#![no_main]

use core::array;

use esp_backtrace as _;
use esp_hal::prelude::*;
use st7920::*;

#[entry]
fn main() -> ! {
    let (mut lcd, ..) = rg16032e::setup();

    lcd.display_on_off(true, false, true);

    let mut byte = b'0';
    let mut read = false;
    loop {
        let [addr, rest @ ..] = array::from_fn::<_, 10, _>(|_| lcd.read_address_counter());
        if rest.iter().any(|a| *a != addr) {
            log::error!("AC READ! {addr} != {rest:?}");

            lcd.init().unwrap();
            lcd.display_on_off(true, false, true);
            continue;
        }

        let data = (byte as u16) << 8 | byte as u16;
        if read {
            let check = lcd.read();
            if check != data {
                log::error!("RAM W/R! 0x{check:04x} != 0x{data:04x} @ 0x{addr:02x}");

                lcd.init().unwrap();
                lcd.display_on_off(true, false, true);
                read = false;
                continue;
            }
        } else {
            lcd.write(data);
        }

        let new = lcd.read_address_counter();
        if new != addr + 1 {
            log::error!("AC INCREMENT! 0x{new:02x} != {:02x}", addr + 1);

            lcd.init().unwrap();
            lcd.display_on_off(true, false, true);
            continue;
        }

        //  0  1  2  3  4  5  6  7  8  9 |  a  b  c  d  e  f
        // 10 11 12 13 14 15 16 17 18 19 | 1a 1b 1c 1d 1e 1f
        match addr {
            0x9 => lcd.ddram_addr(0x10),
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
                lcd.ddram_addr(0x0);
                read = !read;
            }
            _ => (),
        }
    }
}
