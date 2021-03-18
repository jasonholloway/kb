use std::collections::VecDeque;


#[derive(Debug)]
pub enum Update<TRaw> {
    Key(u16, Movement, Option<TRaw>),
    Tick
}

#[derive(Debug)]
pub enum Movement {
    Up,
    Down
}


pub type NextDue = u64;


pub type Handler<TState, TRaw> = fn(
    state: &mut TState,
    buff: &mut VecDeque<Update<TRaw>>,
    update: Update<TRaw>
) -> NextDue;

