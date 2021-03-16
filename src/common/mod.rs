
use std::io::Error;


pub enum KeyEvent<TRaw> {
    Up(u8, Option<TRaw>),
    Down(u8, Option<TRaw>)
}

pub enum Update<TRaw> {
    Key(KeyEvent<TRaw>),
    Tick
}

pub enum Response {
    Grab,
    Skip
}


pub type Handler<TState, TRaw> = fn(state: TState, update: Update<TRaw>) -> Response;

pub trait Setup : Sized
{
    type TRuntime : Runtime<Self>;
    type TRaw;
    
    fn install<TState>(&self, state: TState, handler: Handler<TState, Self::TRaw>) -> Result<Self::TRuntime, Error>;
}

pub trait Runtime<K : Setup> {
    fn inject(&self, ev: KeyEvent<K::TRaw>) -> ();
}
