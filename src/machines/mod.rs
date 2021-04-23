#![allow(unused_variables)]
#![allow(unused_mut)]

use self::{key_maps::KeyMaps, runner::{Ev,Ev::*}};
use crate::{common::Update, common::Update::*};
use bitmaps::{Bitmap, Bits};
use std::collections::{vec_deque::*, HashMap};

pub mod big_machine;
pub mod dynamic_machine;
pub mod key_maps;
pub mod lead_machine;
pub mod mode_machine;
pub mod print_keys;


pub mod runner;


pub struct RunRef<TUp> {
    tag: &'static str,
    inner: Box<dyn Runnable<TUp>>
}

impl<TUp> RunRef<TUp> {
    pub fn new<TInner: 'static + Runnable<TUp>>(tag: &'static str, inner: TInner) -> RunRef<TUp> {
        RunRef {
            tag,
            inner: Box::new(inner)
        }
    }
}

impl<TUp> std::fmt::Debug for RunRef<TUp> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.tag)
    }
}




type Sink<TEv> = VecDeque<TEv>;


pub trait Runnable<TUp> {
    fn run(&mut self, ev: Ev<TUp>, sink: &mut Sink<Ev<TUp>>) -> ();
}

pub type MachineFac<TEv> = Box<dyn Fn() -> MachineRef<TEv>>;
pub type MachineRef<TEv> = Box<dyn Runnable<TEv>>;

pub trait HasMaps {
    fn maps(&mut self) -> &mut KeyMaps;
}

pub trait CanMask<TEv>: HasMaps {
    fn mask(&mut self, codes: &[u16], sink: &mut Sink<TEv>);
    fn unmask(&mut self, codes: &[u16], sink: &mut Sink<TEv>);
}

pub trait CanEmit<TEv> {
    fn emit(&mut self, ev: TEv, sink: &mut Sink<TEv>);
}


pub fn gather_map<T, T2: Bits>(event: &Update<T>, map: &mut Bitmap<T2>) {
    use super::Movement::*;

    if let Key(code, movement, _) = event {
        match movement {
            Up => map.set(*code as usize, false),
            Down => map.set(*code as usize, true),
        };
    }
}

use super::Movement::*;

impl<TRaw, TSelf> CanMask<Ev<Update<TRaw>>> for TSelf
where
    TSelf: HasMaps,
{
    fn mask(&mut self, codes: &[u16], sink: &mut Sink<Ev<Update<TRaw>>>) {
        for c in codes {
            let maskable = !self.maps().mask.set(*c as usize, true);

            if maskable && self.maps().outp.get(*c as usize) {
                self.emit(Ev(Key(*c, Up, None)), sink);
            }
        }
    }

    fn unmask(&mut self, codes: &[u16], sink: &mut Sink<Ev<Update<TRaw>>>) {
        for c in codes {
            let unmaskable = self.maps().mask.set(*c as usize, false);

            if unmaskable && self.maps().inp.get(*c as usize) && !self.maps().outp.get(*c as usize)
            {
                self.emit(Ev(Key(*c, Down, None)), sink);
            }
        }
    }
}


impl<TRaw, TSelf> CanEmit<Ev<Update<TRaw>>> for TSelf
where
    TSelf: HasMaps,
{
    fn emit(&mut self, ev: Ev<Update<TRaw>>, sink: &mut Sink<Ev<Update<TRaw>>>) {
        match &ev {
            Ev::Ev(up) => {
                gather_map(&up, &mut self.maps().outp);
                sink.push_back(ev)
            },
            _ => {}
        }
    }
}

pub trait LookupFac<T> {
    fn find(&self, tag: &'static str) -> Option<T>;
}

impl<TEv> LookupFac<MachineRef<TEv>> for HashMap<&str, MachineFac<TEv>> {
    fn find(&self, tag: &'static str) -> Option<MachineRef<TEv>> {
        self.get(tag).map(|f| f())
    }
}
