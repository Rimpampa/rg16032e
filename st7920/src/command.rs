pub mod ext;
pub mod infallible;

fn bit<T: Into<u8>>(v: T, bit: u8) -> u8 {
    v.into() << bit
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Command {
    /// Write into the currently selected RAM
    Write(u16),
    /// Clear the contents of the display.
    ///
    /// Instruction Set: **Basic**
    ///
    /// This command sets all the DDRAM to `0x20` (ASCII space),
    /// resets the Address Counter to 0, and resets the
    /// currently applied display shift.
    ///
    /// This command also resets the entry mode, as if
    /// this command was sent:
    /// [`EntryMode { increment: true, shift: false }`](Command::EntryMode).
    Clear,
    /// Reset the display shift and the Address Counter
    ///
    /// Instruction Set: **Basic**
    Home,
    /// Chose the behaviour after a read or write operation
    ///
    /// Instruction Set: **Basic**
    EntryMode {
        /// Whether the Address Counter is incrmented (`true`)
        /// or decremented (`false`) after each read or write operation.
        increment: bool,
        /// Whether the whole DDRAM display contents is shifted in
        /// the direction of the Address Counter (+1 = right, -1 = false).
        shift: bool,
    },
    /// Enable or disable different parts of the LCD controller
    ///
    /// Instruction Set: **Basic**
    DisplayOnOff {
        display: bool,
        cursor: bool,
        blink: bool,
    },
    /// Move either the cursor or the entire display contents by 1
    /// to the left or right
    ///
    /// Instruction Set: **Basic**
    CursorDisplayCtrl {
        /// Shift by 1 the display (`true`) or the cursor (`false`)
        ///
        /// When the display shifts the cursor follows the shift
        /// direction and the Address Counter is left the same
        sc: bool,
        /// Right (`true`) or left (`false`)
        rl: bool,
    },
    /// Select the _Basic instruction set_
    SelectBasic,
    /// Set the Character Generator RAM (CGRAM) address
    ///
    /// Instruction Set: **Basic** (also **Extended**
    /// when [`EnableCgRam`](Command::EnableCgRam) is sent)
    ///
    /// After sending this command every read and write operation
    /// happens on the CGRAM
    CgRamAddr(u8),
    /// Set the Display Data RAM (DDRAM) address
    ///
    /// Instruction Set: **Basic**
    ///
    /// After sending this command every read and write operation
    /// happens on the DDRAM
    DdRamAddr(u8),
}

impl Command {
    /// Execution time of the [`Command`] in microseconds
    pub fn execution_time(self) -> u32 {
        let Self::Clear = self else { return 72 };
        1_600
    }

    pub fn into_byte(self) -> u8 {
        use Command::*;
        match self {
            Write(_) => unreachable!(),
            Clear => 0b1,
            Home => 0b10,
            EntryMode {
                increment: i,
                shift: s,
            } => 0b0100 | bit(i, 1) | bit(s, 0),
            DisplayOnOff {
                display: d,
                cursor: c,
                blink: b,
            } => 0b1000 | bit(d, 2) | bit(c, 1) | bit(b, 0),
            CursorDisplayCtrl { sc, rl } => 0b10000 | bit(sc, 3) | bit(rl, 2),
            SelectBasic => 0b100000,
            CgRamAddr(addr) => 0b01000000 | (addr & 0b0111111),
            DdRamAddr(addr) => 0b10000000 | (addr & 0b1111111),
        }
    }
}

pub trait Execute {
    type Error;

    fn execute(&mut self, command: Command) -> Result<(), Self::Error>;

    fn write(&mut self, data: u16) -> Result<(), Self::Error> {
        self.execute(Command::Write(data))
    }

    fn clear(&mut self) -> Result<(), Self::Error> {
        self.execute(Command::Clear)
    }

    fn home(&mut self) -> Result<(), Self::Error> {
        self.execute(Command::Home)
    }

    fn entry_mode(&mut self, increment: bool, shift: bool) -> Result<(), Self::Error> {
        self.execute(Command::EntryMode { increment, shift })
    }

    fn display_on_off(
        &mut self,
        display: bool,
        cursor: bool,
        blink: bool,
    ) -> Result<(), Self::Error> {
        self.execute(Command::DisplayOnOff {
            display,
            cursor,
            blink,
        })
    }

    fn cursor_display_ctrl(&mut self, sc: bool, rl: bool) -> Result<(), Self::Error> {
        self.execute(Command::CursorDisplayCtrl { sc, rl })
    }

    fn select_basic(&mut self) -> Result<(), Self::Error> {
        self.execute(Command::SelectBasic)
    }

    fn cgram_addr(&mut self, addr: u8) -> Result<(), Self::Error> {
        self.execute(Command::CgRamAddr(addr))
    }

    fn ddram_addr(&mut self, addr: u8) -> Result<(), Self::Error> {
        self.execute(Command::DdRamAddr(addr))
    }
}

pub trait ExecuteRead {
    type Error;

    fn read(&mut self) -> Result<u16, Self::Error>;

    fn read_bf_ac(&mut self) -> Result<(bool, u8), Self::Error>;
    fn read_address_counter(&mut self) -> Result<u8, Self::Error> {
        Ok(self.read_bf_ac()?.1)
    }
    fn read_busy_flag(&mut self) -> Result<bool, Self::Error> {
        Ok(self.read_bf_ac()?.0)
    }
}
