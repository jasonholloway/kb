
#[derive(Debug)]
#[derive(Copy, Clone)]
pub enum Update<TRaw> {
    Key(u16, Movement, Option<TRaw>),
    Tick,
    ModeOn(&'static str),
    ModeOff(&'static str),
    Drop
}

#[derive(Debug)]
#[derive(Copy, Clone)]
pub enum Movement {
    Up,
    Down
}


pub type NextDue = u64;


// #[derive(Debug)]
// #[derive(Copy, Clone)]
// pub enum Act {
//     Mode(&'static str),
// }
