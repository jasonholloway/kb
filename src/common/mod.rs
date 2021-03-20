
#[derive(Debug)]
#[derive(Copy, Clone)]
pub enum Update<TRaw> {
    Key(u16, Movement, Option<TRaw>),
    Tick
}

#[derive(Debug)]
#[derive(Copy, Clone)]
pub enum Movement {
    Up,
    Down
}


pub type NextDue = u64;

