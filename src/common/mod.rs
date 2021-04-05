#[derive(Debug, Copy, Clone)]
pub enum Update<TRaw> {
    Key(u16, Movement, Option<TRaw>),
    Tick,
    On(Mode),
    Off(Mode),
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
