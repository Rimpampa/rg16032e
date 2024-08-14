#![no_std]
#![feature(trait_alias)]
#![feature(associated_type_bounds)]

use esp_hal::{gpio, peripheral::Peripheral};
use gpio::{AnyOutput, CreateErasedPin, InputPin, Level, OutputPin};

use fugit::ExtU64;

pub mod bus;
mod command;
mod ll;

use command::{Basic, Exec, Extended};

pub trait In = Peripheral<P: InputPin + CreateErasedPin> + 'static;
pub trait Out = Peripheral<P: OutputPin + CreateErasedPin> + 'static;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Mode {
    Basic,
    Extended,
    Graphic,
}

pub struct Driver {
    rs: AnyOutput<'static>,
    rw: AnyOutput<'static>,
    e: AnyOutput<'static>,
    bus: bus::Bus,
    mode: Mode,
    // sleep_until: ll::Instant,
}

impl Driver {
    pub fn setup(rs: impl Out, rw: impl Out, e: impl Out, bus: bus::Bus) -> Self {
        let mut this = Self {
            rs: AnyOutput::new(rs, Level::Low),
            rw: AnyOutput::new(rw, Level::Low),
            e: AnyOutput::new(e, Level::Low),
            bus,
            mode: Mode::Basic,
            // sleep_until: ll::now(),
        };
        this.init();
        this
    }

    pub fn init(&mut self) {
        ll::sleep(80.millis());
        self.exec_stateless(Basic::Set);
        ll::sleep(200.micros());
        self.exec_stateless(Basic::Set);
        ll::sleep(200.micros());
        self.exec_stateless(Basic::DisplayOnOff {
            d: true,
            c: false,
            b: false,
        });
        ll::sleep(200.micros());
        self.exec_stateless(Basic::Clear);
        ll::sleep(20.millis());
        self.exec_stateless(Basic::EntryMode {
            i_d: true,
            s: false,
        });
    }

    pub fn read_address_counter(&mut self) -> (bool, u8) {
        self.select_busy_flag();
        let read = self.read_u8();
        ((read >> 7) != 0, read & 0b01111111)
    }

    pub fn wait_busy(&mut self) {
        let end = ll::now() + 1.millis();
        while self.read_address_counter().0 && ll::now() < end {}
    }

    pub fn clear(&mut self) {
        self.exec(Basic::Clear)
    }

    pub fn home(&mut self) {
        self.exec(Basic::Home)
    }

    pub fn entry_mode(&mut self, increment: bool, shift: bool) {
        self.exec(Basic::EntryMode {
            i_d: increment,
            s: shift,
        })
    }

    pub fn display_on_off(&mut self, display: bool, cursor: bool, blink: bool) {
        self.exec(Basic::DisplayOnOff {
            d: display,
            c: cursor,
            b: blink,
        })
    }

    pub fn cursor_display_control(&mut self, s_c: bool, r_l: bool) {
        self.exec(Basic::CursorDisplayControl { s_c, r_l })
    }

    pub fn set_cgram_addr(&mut self, addr: u8) {
        self.exec(Basic::CgRamAddr(addr))
    }

    pub fn set_ddram_addr(&mut self, addr: u8) {
        self.exec(Basic::DdRamAddr(addr))
    }

    pub fn stand_by(&mut self) {
        self.exec(Extended::StandBy)
    }

    pub fn select_scroll(&mut self) {
        self.exec(Extended::SelectScroll)
    }

    pub fn select_cgram(&mut self) {
        self.exec(Extended::SelectCgRam)
    }

    pub fn reverse(&mut self, line: u8) {
        self.exec(Extended::Reverse(line))
    }

    pub fn set_basic(&mut self) {
        self.exec(Basic::Set)
    }

    pub fn set_extended(&mut self) {
        self.exec(Extended::Set)
    }

    pub fn set_graphic(&mut self) {
        self.exec(Extended::SetGraphic)
    }

    pub fn set_scroll_addr(&mut self, scroll: u8) {
        self.exec(Extended::ScrollAddr(scroll))
    }

    pub fn set_graphic_ram_addr(&mut self, x: u8, y: u8) {
        self.exec(Extended::GraphicRamAddr { x, y })
    }

    pub fn write(&mut self, data: u16) {
        self.select_ram_write();
        self.write_u16(data);
    }

    pub fn read(&mut self) -> u16 {
        self.select_ram_read();
        self.read_u16()
    }
}
