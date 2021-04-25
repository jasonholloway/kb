#![allow(unused_variables)]
#![allow(unused_mut)]

#[cfg(test)]
mod test;

pub mod runner;
pub mod key_maps;
pub mod machine;

pub mod big_machine;
pub mod dynamic_machine;
pub mod lead_machine;
pub mod mode_machine;
pub mod print_keys;


use self::key_maps::KeyMaps;
use std::collections::vec_deque::*;


pub struct RunRef<TCtx, TEv> {
    tag: &'static str,
    inner: Box<dyn Runnable<TCtx, TEv>>
}

impl<TCtx, TEv> RunRef<TCtx,TEv> {
    pub fn new<TInner: 'static + Runnable<TCtx, TEv>>(tag: &'static str, inner: TInner) -> RunRef<TCtx, TEv> {
        RunRef {
            tag,
            inner: Box::new(inner)
        }
    }
}

impl<TCtx, TEv> std::fmt::Debug for RunRef<TCtx,TEv> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.tag)
    }
}




type Sink<TEv> = VecDeque<TEv>;


pub trait Runnable<TCtx, TEv> {
    fn run(&mut self, ctx: &mut TCtx, ev: TEv) -> ();
}



pub trait HasMaps {
    fn maps(&self) -> &KeyMaps;
}

pub trait CanEmit<TEv> {
    fn emit(&mut self, ev: TEv);
    fn emit_many<T: IntoIterator<Item=TEv>>(&mut self, evs: T);
}

pub trait CanMask<TEv> {
    fn mask(&mut self, codes: &[u16]);
    fn unmask(&mut self, codes: &[u16]);
}

