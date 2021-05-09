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

use crate::common::{Emit, Ev};

use self::key_maps::KeyMaps;


pub struct RunRef<TRaw> {
    tag: &'static str,
    inner: Box<dyn Runnable<TRaw>>
}

impl<TRaw> PartialEq for RunRef<TRaw> {
    fn eq(&self, other: &Self) -> bool {
        false
    }
}

impl<TRaw> RunRef<TRaw> {
    pub fn new<TInner: 'static + Runnable<TRaw>>(tag: &'static str, inner: TInner) -> RunRef<TRaw> {
        RunRef {
            tag,
            inner: Box::new(inner)
        }
    }
}

impl<TRaw> std::fmt::Debug for RunRef<TRaw> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.tag)
    }
}


pub struct Ctx<TRaw> {
    pub maps: KeyMaps,
    pub buff: VecDeque<Emit<TRaw>>
}

impl<TRaw> Ctx<TRaw> {
    pub fn new() -> Ctx<TRaw> {
        Ctx {
            maps: KeyMaps::new(),
            buff: VecDeque::new()
        }
    }
}

impl<TRaw> Ctx<TRaw> {
    pub fn emit(&mut self, ev: Ev<TRaw>) {
        self.buff.push_back(Emit::Emit(ev))
    }

    pub fn emit_many<T: IntoIterator<Item=Ev<TRaw>>>(&mut self, evs: T) {
        for ev in evs {
            self.emit(ev)
        }
    }

    pub fn pass_thru(&mut self, ev: Ev<TRaw>) {
        self.buff.push_back(Emit::PassThru(ev))
    }

    pub fn mask(&mut self, codes: &[u16]) {
        for c in codes {
            self.buff.push_back(Emit::MaskOn(*c))
        }
    }

    pub fn unmask(&mut self, codes: &[u16]) {
        for c in codes {
            self.buff.push_back(Emit::MaskOff(*c))
        }
    }

    pub fn spawn(&mut self, runRef: RunRef<TRaw>) {
        self.buff.push_back(Emit::Spawn(runRef))
    }
}




pub trait Runnable<TRaw> {
    fn run(&mut self, x: &mut Ctx<TRaw>, ev: Ev<TRaw>) -> ();
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

