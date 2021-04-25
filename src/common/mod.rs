use crate::machines::{RunRef};

#[derive(Debug)]
pub enum Ev<TRaw> {
    Key(u16, Movement, Option<TRaw>),
    On(Mode),
    Off(Mode),
    Tick,
    Spawn(RunRef<Ev<TRaw>>),
    Die,
    MaskOn(u16),
    MaskOff(u16)
}

#[derive(Debug, Copy, Clone)]
pub enum Movement {
    Up,
    Down,
}

#[derive(Debug, Copy, Clone)]
pub enum Mode {
    Root,
    Mode(&'static str),
}

#[derive(Debug, Copy, Clone)]
pub enum Act {
    Drop,
    Mask(u16),
    Map(u16, u16),
    Emit(u16, Movement),
    Then(Mode),
    Launch(&'static str),
}

pub type NextDue = u64;

// #[derive(Debug)]
// #[derive(Copy, Clone)]
// pub enum Act {
//     Mode(&'static str),
// }
