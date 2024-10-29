use crate::hal;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Command {
    StandBy,
    /// Enable the [`ScrollOffset`](Command::ScrollOffset) command
    ///
    /// Instruction Set: **Extended**
    EnableScroll,
    /// Select the Character Generator RAM (CGRAM)
    ///
    /// Instruction Set: **Extended**
    EnableCgRam,
    /// Reverse the pixels of the given line
    ///
    /// Instruction Set: **Extended**
    ///
    /// Only one line at the time can be reversed.
    /// The first time this command is sent the given line
    /// is reversed, while the second time it returns to normal
    /// (no matter the given address).
    Reverse(u8),
    /// Select the _Extended instruction set_
    SelectExtended,
    /// Select the _Graphic instruction set_
    ///
    /// > When going from the _Basic instruct set_ to the _Graphic_ one,
    /// > the [`SelectExtended`](Command::SelectExtended) command must be run first.
    SelectGraphic,
    /// Set the vertical scroll offset
    ///
    /// Instruction Set: **Extended**
    ///
    /// > Make sure to run the [`SelectScroll`](Command::ScrollOffset) command first.
    ScrollOffset(u8),
    /// Set the Graphic RAM address
    ///
    /// Instruction Set: **Graphic**
    GraphicRamAddr {
        y: u8,
        x: u8,
    },
}

impl Command {
    /// Execution time of the [`Command`] in microseconds
    pub fn execution_time(self) -> hal::Duration {
        use fugit::ExtU64;
        1_600.micros()
    }

    pub fn into_bytes(self) -> [u8; 2] {
        use Command::*;
        let byte = match self {
            StandBy => 0b1,
            EnableScroll => 0b11,
            EnableCgRam => 0b10,
            Reverse(line) => 0b100 | (line & 0b11),
            SelectExtended => 0b100100,
            SelectGraphic => 0b100110,
            ScrollOffset(offset) => 0b1000000 | (offset & 0b11111),
            GraphicRamAddr { y, x } => return [y & 0b1111, x & 0b111111].map(|b| 0b10000000 | b),
        };
        [byte, 0]
    }
}

pub trait Execute: super::Execute {
    fn execute_ext(&mut self, command: Command) -> Result<(), Self::Error>;

    fn stand_by(&mut self) -> Result<(), Self::Error> {
        Execute::execute_ext(self, Command::StandBy)
    }

    fn enable_scroll(&mut self) -> Result<(), Self::Error> {
        Execute::execute_ext(self, Command::EnableScroll)
    }

    fn enable_cgram(&mut self) -> Result<(), Self::Error> {
        Execute::execute_ext(self, Command::EnableCgRam)
    }

    fn reverse(&mut self, line: u8) -> Result<(), Self::Error> {
        Execute::execute_ext(self, Command::Reverse(line))
    }

    fn select_extended(&mut self) -> Result<(), Self::Error> {
        Execute::execute_ext(self, Command::SelectExtended)
    }

    fn select_graphic(&mut self) -> Result<(), Self::Error> {
        Execute::execute_ext(self, Command::SelectGraphic)
    }

    fn scroll_offset(&mut self, offset: u8) -> Result<(), Self::Error> {
        Execute::execute_ext(self, Command::ScrollOffset(offset))
    }

    fn graphic_ram_addr(&mut self, x: u8, y: u8) -> Result<(), Self::Error> {
        Execute::execute_ext(self, Command::GraphicRamAddr { y, x })
    }
}
