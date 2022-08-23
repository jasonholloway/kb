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

use crate::common::{CoreEv, MachineEv, Movement, Out};

use self::key_maps::KeyMaps;


pub struct RunRef<TRaw,TIn,TOut>
{
    tag: &'static str,
    inner: Box<dyn Runnable<TRaw,TIn,TOut>>
}

impl<TRaw,TIn,TOut> PartialEq for RunRef<TRaw,TIn,TOut> {
    fn eq(&self, other: &Self) -> bool {
        false
    }
}

impl<TRaw,TIn,TOut> RunRef<TRaw,TIn,TOut>
{
    pub fn new<TInner: 'static + Runnable<TRaw,TIn,TOut>>(tag: &'static str, inner: TInner) -> RunRef<TRaw,TIn,TOut> {
        RunRef {
            tag,
            inner: Box::new(inner)
        }
    }
}

impl<TRaw,TIn,TOut> std::fmt::Debug for RunRef<TRaw,TIn,TOut> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.tag)
    }
}


pub struct Ctx<TRaw,TOut> {
    pub maps: KeyMaps,
    pub buff: VecDeque<(Option<TRaw>,TOut)>
}

impl<TRaw,TOut> Ctx<TRaw,TOut> {
    pub fn new() -> Ctx<TRaw,TOut> {
        Ctx {
            maps: KeyMaps::new(),
            buff: VecDeque::new()
        }
    }

    pub fn emit(&mut self, ev: (Option<TRaw>,TOut)) {
        self.buff.push_back(ev)
    }

    pub fn emit_many<TIter>(&mut self, evs: TIter)
        where TIter : IntoIterator<Item=(Option<TRaw>,TOut)>
    {
        self.buff.extend(evs)
    }

}

impl<TRaw> Ctx<TRaw,Out> {
    pub fn key_down(&mut self, code: u16) {
        self.emit((None, Out::Core(CoreEv::Key(code, Movement::Down))))
    }

    pub fn key_up(&mut self, code: u16) {
        self.emit((None, Out::Core(CoreEv::Key(code, Movement::Up))))
    }

    pub fn mask<T: IntoIterator<Item=&'static u16>>(&mut self, codes: T) {
        for c in codes {
            self.emit((None, Out::Machine(MachineEv::MaskOn(*c))))
        }
    }

    pub fn unmask<T: IntoIterator<Item=&'static u16>>(&mut self, codes: T) {
        for c in codes {
            self.emit((None, Out::Machine(MachineEv::MaskOff(*c))))
        }
    }
}

// impl<TRaw,TOut> Sink<(Option<TRaw>,TOut)> for Ctx<TRaw,TOut> {

//     // pub fn pass_thru(&mut self, ev: (Option<TRaw>,TOut)) {
//     //     self.buff.push_back(Emit::PassThru(ev))
//     // }

//     // pub fn mask(&mut self, codes: &[u16]) {
//     //     for c in codes {
//     //         self.buff.push_back(Emit::MaskOn(*c))
//     //     }
//     // }

//     // pub fn unmask(&mut self, codes: &[u16]) {
//     //     for c in codes {
//     //         self.buff.push_back(Emit::MaskOff(*c))
//     //     }
//     // }

//     // pub fn spawn(&mut self, runRef: RunRef<TIn,TOut>) {
//     //     self.buff.push_back(Emit::Spawn(runRef))
//     // }
// }





// pub trait Sink<TOut> {
//     fn emit(&mut self, el: TOut) -> ();
// }

// impl<TOut> Sink<TOut> {
//     fn emit_many<T: IntoIterator<Item=TOut>>(&mut self, evs: T) {
//         for ev in evs {
//             self.emit(ev)
//         }
//     }
// }




pub trait Runnable<TRaw,TIn,TOut>
{
    fn run(&mut self, x: &mut Ctx<TRaw,TOut>, ev: (Option<TRaw>,TIn)) -> ();
}




pub trait RunnableEx<TRaw,TIn,TOut> {
    fn run(&mut self, x: &mut Ctx<TRaw,TOut>, ev: TIn) -> ();
}

impl<TRaw,TIn,TOut,TRunnable>  TRunnable
    where TRunnable: Runnable<TRaw,TIn,TOut>
{
    fn run2(&mut self, x: &mut Ctx<TRaw,TOut>, ev: TIn) -> () {
        todo!()
    }
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

