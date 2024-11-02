use super::{flex::Flex as MyFlex, In, Out};
use esp_hal::gpio::*;
use st7920::parallel::interface::*;

pub fn new_4bit<'a, const NUM: usize>(
    rs: impl Out + 'a,
    rw: impl Out + 'a,
    e: [impl Out + 'a; NUM],
    db4: impl In + Out + 'a,
    db5: impl In + Out + 'a,
    db6: impl In + Out + 'a,
    db7: impl In + Out + 'a,
) -> Interface4Bit<Output<'a>, MyFlex<'a>, NUM> {
    Interface::new(
        Output::new(rs, Level::Low),
        Output::new(rw, Level::Low),
        e.map(|e| Output::new(e, Level::Low)),
        [
            Flex::new(db4),
            Flex::new(db5),
            Flex::new(db6),
            Flex::new(db7),
        ]
        .map(MyFlex),
    )
}

pub fn new_8bit<'a, const NUM: usize>(
    rs: impl Out + 'a,
    rw: impl Out + 'a,
    e: [impl Out + 'a; NUM],
    db0: impl In + Out + 'a,
    db1: impl In + Out + 'a,
    db2: impl In + Out + 'a,
    db3: impl In + Out + 'a,
    db4: impl In + Out + 'a,
    db5: impl In + Out + 'a,
    db6: impl In + Out + 'a,
    db7: impl In + Out + 'a,
) -> Interface8Bit<Output<'a>, MyFlex<'a>, NUM> {
    Interface::new(
        Output::new(rs, Level::Low),
        Output::new(rw, Level::Low),
        e.map(|e| Output::new(e, Level::Low)),
        [
            Flex::new(db0),
            Flex::new(db1),
            Flex::new(db2),
            Flex::new(db3),
            Flex::new(db4),
            Flex::new(db5),
            Flex::new(db6),
            Flex::new(db7),
        ]
        .map(MyFlex),
    )
}
