use crate::machines::{RunRef};

#[derive(Debug,PartialEq)]
pub enum Ev<TRaw> {
    Key(u16, Movement, Option<TRaw>),
    On(Mode),
    Off(Mode),
    Tick
}

pub enum Emit<TRaw> {
    Emit(Ev<TRaw>),
    Now(Mode),
    Die,
    MaskOn(u16),
    MaskOff(u16),
    PassThru(Ev<TRaw>),
    Spawn(RunRef<TRaw>),
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Movement {
    Up,
    Down,
}

#[derive(Debug, Copy, Clone, PartialEq)]
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

// impl<TRaw> Emit<TRaw> {
//     pub fn ev(self) -> Ev<TRaw> {
//         match self {
//             Emit::Emit(ev) => ev,
//             Emit::PassThru(ev) => ev
//         }
//     }
// }
