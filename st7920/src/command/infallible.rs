mod sealed {
    pub trait Infallible: core::fmt::Debug {}
    impl Infallible for core::convert::Infallible {}
}

pub trait Execute: super::Execute<Error: sealed::Infallible> {
    fn execute(&mut self, command: super::Command);
    fn write(&mut self, data: u16);
    fn clear(&mut self);
    fn home(&mut self);
    fn entry_mode(&mut self, increment: bool, shift: bool);
    fn display_on_off(&mut self, display: bool, cursor: bool, blink: bool);
    fn cursor_display_ctrl(&mut self, sc: bool, rl: bool);
    fn select_basic(&mut self);
    fn cgram_addr(&mut self, addr: u8);
    fn ddram_addr(&mut self, addr: u8);
}

pub trait ExecuteExt: super::ext::Execute<Error: sealed::Infallible> {
    fn execute(&mut self, command: super::ext::Command);
    fn stand_by(&mut self);
    fn enable_scroll(&mut self);
    fn enable_cgram(&mut self);
    fn reverse(&mut self, line: u8);
    fn select_extended(&mut self);
    fn select_graphic(&mut self);
    fn scroll_offset(&mut self, offset: u8);
    fn graphic_ram_addr(&mut self, x: u8, y: u8);
}

pub trait ExecuteRead: super::ExecuteRead<Error: sealed::Infallible> {
    fn read(&mut self) -> u16;
    fn read_bf_ac(&mut self) -> (bool, u8);
    fn read_address_counter(&mut self) -> u8;
    fn read_busy_flag(&mut self) -> bool;
}

impl<E: super::Execute<Error: sealed::Infallible>> Execute for E {
    fn execute(&mut self, command: super::Command) {
        super::Execute::execute(self, command).unwrap()
    }

    fn write(&mut self, data: u16) {
        super::Execute::write(self, data).unwrap()
    }

    fn clear(&mut self) {
        super::Execute::clear(self).unwrap()
    }

    fn home(&mut self) {
        super::Execute::home(self).unwrap()
    }

    fn entry_mode(&mut self, increment: bool, shift: bool) {
        super::Execute::entry_mode(self, increment, shift).unwrap()
    }

    fn display_on_off(&mut self, display: bool, cursor: bool, blink: bool) {
        super::Execute::display_on_off(self, display, cursor, blink).unwrap()
    }

    fn cursor_display_ctrl(&mut self, sc: bool, rl: bool) {
        super::Execute::cursor_display_ctrl(self, sc, rl).unwrap()
    }

    fn select_basic(&mut self) {
        super::Execute::select_basic(self).unwrap()
    }

    fn cgram_addr(&mut self, addr: u8) {
        super::Execute::cgram_addr(self, addr).unwrap()
    }

    fn ddram_addr(&mut self, addr: u8) {
        super::Execute::ddram_addr(self, addr).unwrap()
    }
}

impl<E: super::ExecuteRead<Error: sealed::Infallible>> ExecuteRead for E {
    fn read(&mut self) -> u16 {
        super::ExecuteRead::read(self).unwrap()
    }

    fn read_bf_ac(&mut self) -> (bool, u8) {
        super::ExecuteRead::read_bf_ac(self).unwrap()
    }

    fn read_address_counter(&mut self) -> u8 {
        super::ExecuteRead::read_address_counter(self).unwrap()
    }

    fn read_busy_flag(&mut self) -> bool {
        super::ExecuteRead::read_busy_flag(self).unwrap()
    }
}

impl<E: super::ext::Execute<Error: sealed::Infallible>> ExecuteExt for E {
    fn execute(&mut self, command: super::ext::Command) {
        super::ext::Execute::execute_ext(self, command).unwrap()
    }

    fn stand_by(&mut self) {
        super::ext::Execute::stand_by(self).unwrap()
    }

    fn enable_scroll(&mut self) {
        super::ext::Execute::enable_scroll(self).unwrap()
    }

    fn enable_cgram(&mut self) {
        super::ext::Execute::enable_cgram(self).unwrap()
    }

    fn reverse(&mut self, line: u8) {
        super::ext::Execute::reverse(self, line).unwrap()
    }

    fn select_extended(&mut self) {
        super::ext::Execute::select_extended(self).unwrap()
    }

    fn select_graphic(&mut self) {
        super::ext::Execute::select_graphic(self).unwrap()
    }

    fn scroll_offset(&mut self, offset: u8) {
        super::ext::Execute::scroll_offset(self, offset).unwrap()
    }

    fn graphic_ram_addr(&mut self, x: u8, y: u8) {
        super::ext::Execute::graphic_ram_addr(self, x, y).unwrap()
    }
}
