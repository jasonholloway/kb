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


use std::collections::vec_deque::*;

use crate::common::Ev;

use self::key_maps::KeyMaps;


pub struct RunRef<TEv> {
    tag: &'static str,
    inner: Box<dyn Runnable<TEv>>
}

impl<TEv> RunRef<TEv> {
    pub fn new<TInner: 'static + Runnable<TEv>>(tag: &'static str, inner: TInner) -> RunRef<TEv> {
        RunRef {
            tag,
            inner: Box::new(inner)
        }
    }
}

impl<TEv> std::fmt::Debug for RunRef<TEv> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.tag)
    }
}



pub struct Ctx<TEv> {
    pub maps: KeyMaps,
    pub buff: VecDeque<TEv>
}

impl<TRaw> Ctx<TRaw> {
    pub fn new() -> Ctx<TRaw> {
        Ctx {
            maps: KeyMaps::new(),
            buff: VecDeque::new()
        }
    }
}

impl<TRaw> Ctx<Ev<TRaw>> {
    pub fn emit(&mut self, ev: Ev<TRaw>) {
        self.buff.push_back(ev)
    }

    pub fn emit_many<T: IntoIterator<Item=Ev<TRaw>>>(&mut self, evs: T) {
        self.buff.extend(evs)
    }

    pub fn mask(&mut self, codes: &[u16]) {
        for c in codes {
            self.emit(Ev::MaskOn(*c));
        }
    }

    pub fn unmask(&mut self, codes: &[u16]) {
        for c in codes {
            self.emit(Ev::MaskOff(*c));
        }
    }
}




pub trait Runnable<TEv> {
    fn run(&mut self, x: &mut Ctx<TEv>, ev: TEv) -> ();
}



// pub trait HasMaps {
//     fn maps(&self) -> &KeyMaps;
// }

// pub trait CanEmit<TEv> {
//     fn emit(&mut self, ev: TEv);
//     fn emit_many<T: IntoIterator<Item=TEv>>(&mut self, evs: T);
// }

// pub trait CanMask<TEv> {
//     fn mask(&mut self, codes: &[u16]);
//     fn unmask(&mut self, codes: &[u16]);
// }

