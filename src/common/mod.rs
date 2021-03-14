
use std::io::Error;


pub enum KeyEvent<TRaw> {
    Up(u8, Option<TRaw>),
    Down(u8, Option<TRaw>)
}

pub enum Response {
    Grab,
    Skip
}


pub type Handler<TRaw> = fn(ev: KeyEvent<TRaw>) -> Response;

pub trait Setup : Sized
{
    type TRuntime : Runtime<Self>;
    type TRaw;
    
    fn install(&self, handler: Handler<Self::TRaw>) -> Result<Self::TRuntime, Error>;
}

pub trait Runtime<K : Setup> {
    fn inject(&self, ev: KeyEvent<K::TRaw>) -> ();
}
