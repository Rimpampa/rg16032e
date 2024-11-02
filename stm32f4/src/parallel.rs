use super::{flex::Flex, Input, Output};
use st7920::parallel::interface::*;

pub fn new_4bit<const NUM: usize>(
    rs: impl Into<Output>,
    rw: impl Into<Output>,
    e: [impl Into<Output>; NUM],
    db4: impl Into<Input>,
    db5: impl Into<Input>,
    db6: impl Into<Input>,
    db7: impl Into<Input>,
) -> Interface4Bit<Output, Flex, NUM> {
    Interface::new(
        rs.into(),
        rw.into(),
        e.map(Into::into),
        [
            Flex::new(db4),
            Flex::new(db5),
            Flex::new(db6),
            Flex::new(db7),
        ],
    )
}

pub fn new_8bit<const NUM: usize>(
    rs: impl Into<Output>,
    rw: impl Into<Output>,
    e: [impl Into<Output>; NUM],
    db0: impl Into<Input>,
    db1: impl Into<Input>,
    db2: impl Into<Input>,
    db3: impl Into<Input>,
    db4: impl Into<Input>,
    db5: impl Into<Input>,
    db6: impl Into<Input>,
    db7: impl Into<Input>,
) -> Interface8Bit<Output, Flex, NUM> {
    Interface::new(
        rs.into(),
        rw.into(),
        e.map(Into::into),
        [
            Flex::new(db0),
            Flex::new(db1),
            Flex::new(db2),
            Flex::new(db3),
            Flex::new(db4),
            Flex::new(db5),
            Flex::new(db6),
            Flex::new(db7),
        ],
    )
}
