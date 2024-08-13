use super::{Driver, Mode};

pub trait Exec<Command> {
    fn exec(&mut self, command: Command);

    fn exec_stateless(&mut self, command: Command);

    fn exec_all(&mut self, iter: impl IntoIterator<Item = Command>) {
        iter.into_iter().for_each(|command| self.exec(command))
    }
}

fn bit(level: bool, bit: u8) -> u8 {
    (level as u8) << bit
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Basic {
    Clear,
    Home,
    EntryMode { i_d: bool, s: bool },
    DisplayOnOff { d: bool, c: bool, b: bool },
    CursorDisplayControl { s_c: bool, r_l: bool },
    Set,
    CgRamAddr(u8),
    DdRamAddr(u8),
}

impl Exec<Basic> for Driver {
    fn exec(&mut self, command: Basic) {
        use Basic::*;

        self.wait_busy();

        // NOTE:
        // Specs says that in the same FUNCTION SET command you can
        // change only one of DL, RE and G bits, so when switching from
        // graphic mode to basic instruction set, first change to extended
        // instruction set
        // RE=1 G=1 ---> RE=1 G=0 ---> RE=0 G=0

        if self.mode == Mode::Graphic {
            self.exec_stateless(Extended::Set);
            self.wait_busy();
        }

        if self.mode != Mode::Basic && command != Set {
            self.exec_stateless(Set);
            self.wait_busy();
        }

        self.mode = Mode::Basic;

        self.exec_stateless(command);
    }

    fn exec_stateless(&mut self, command: Basic) {
        use Basic::*;
        self.select_command();
        self.write_u8(match command {
            Clear => 0b00000001,
            Home => 0b00000010,
            EntryMode { i_d, s } => 0b00000100 | bit(i_d, 1) | bit(s, 0),
            DisplayOnOff { d, c, b } => 0b00001000 | bit(d, 2) | bit(c, 1) | bit(b, 0),
            CursorDisplayControl { s_c, r_l } => 0b00010000 | bit(s_c, 3) | bit(r_l, 2),
            Set => 0b00100000,
            CgRamAddr(addr) => 0b01000000 | (addr & 0b00111111),
            DdRamAddr(addr) => 0b10000000 | (addr & 0b01111111),
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Extended {
    StandBy,
    SelectScroll,
    SelectCgRam,
    Reverse(u8),
    Set,
    SetGraphic,
    ScrollAddr(u8),
    GraphicRamAddr { y: u8, x: u8 },
}

impl Exec<Extended> for Driver {
    fn exec(&mut self, command: Extended) {
        use Extended::*;

        self.wait_busy();

        // NOTE:
        // Specs says that in the same FUNCTION SET command you can
        // change only one of DL, RE and G bits, so when switching from
        // basic instruction set to graphic, first change to extended
        // instruction set
        // RE=0 G=0 ---> RE=1 G=0 ---> RE=1 G=1

        if self.mode == Mode::Basic {
            self.mode = match command {
                SetGraphic => Mode::Graphic,
                Set => Mode::Extended,
                _ => {
                    self.exec_stateless(Set);
                    self.wait_busy();
                    Mode::Extended
                }
            }
        }
        self.exec_stateless(command);
    }

    fn exec_stateless(&mut self, command: Extended) {
        use Extended::*;
        self.select_command();
        self.write_u8(match command {
            StandBy => 0b00000001,
            SelectScroll => 0b00000011,
            SelectCgRam => 0b00000010,
            Reverse(addr) => 0b00000100 | (addr & 0b00000011),
            Set => 0b00100100,
            SetGraphic => 0b00100110,
            ScrollAddr(addr) => 0b01000000 | (addr & 0b00011111),
            GraphicRamAddr { y, x } => {
                self.write_u8(0b10000000 | (y & 0b1111));
                self.write_u8(0b10000000 | (x & 0b111111));
                return;
            }
        })
    }
}
