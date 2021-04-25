use std::collections::VecDeque;

use crate::common::Update;
use super::{CanEmit, CanMask, HasMaps, Runnable, key_maps::KeyMaps, runner::Ev};
use Ev::*;
use Update::*;

pub struct Ctx<TRaw> {
    pub maps: KeyMaps,
    pub buff: VecDeque<Ev<Ctx<TRaw>,Update<TRaw>>>
}

pub struct Machine<TRaw,TBody> {
    pub context: Ctx<TRaw>,
    pub body: TBody
}

impl<TOuterCtx, TRaw, TBody> Runnable<TOuterCtx, Ev<Ctx<TRaw>,Update<TRaw>>> for Machine<TRaw,TBody>
where
    TOuterCtx: CanEmit<Ev<Ctx<TRaw>,Update<TRaw>>>,
    TBody: Runnable<Ctx<TRaw>, Ev<Ctx<TRaw>,Update<TRaw>>>
{
    fn run(&mut self, x: &mut TOuterCtx, ev: Ev<Ctx<TRaw>,Update<TRaw>>) -> () {
        if let Ev::Ev(up) = &ev {
            self.context.maps.track_in(up)
        }
        
        self.body.run(&mut self.context, ev);

        //todo: track out too

        x.emit_many(self.context.buff.drain(0..));
    }
}

impl<TRaw,TBody> Machine<TRaw,TBody> {
    pub fn new(body: TBody) -> Machine<TRaw,TBody> {
        Machine {
            context: Ctx {
                maps: KeyMaps::new(),
                buff: VecDeque::new()
            },
            body
        }
    }
}

impl<TRaw> HasMaps for Ctx<TRaw> {
    fn maps(&self) -> &KeyMaps {
        &self.maps
    }
}


impl<TRaw> CanEmit<Ev<Ctx<TRaw>,Update<TRaw>>> for Ctx<TRaw> {

    fn emit(&mut self, ev: Ev<Ctx<TRaw>,Update<TRaw>>) {
        match &ev {
            Ev(up) => {
                self.maps.track_out(&up);
                self.buff.push_back(ev)
            },
            _ => {}
        }
    }

    fn emit_many<T: IntoIterator<Item=Ev<Ctx<TRaw>,Update<TRaw>>>>(&mut self, evs: T) {
        for ev in evs {
            self.emit(ev)
        }
    }
}

impl<TCtx, TRaw> CanMask<Ev<TCtx,Update<TRaw>>> for Ctx<TRaw>
{

    fn mask(&mut self, codes: &[u16]) {
        use super::super::Movement::*;

        for c in codes {
            let maskable = !self.maps.mask.set(*c as usize, true);

            if maskable && self.maps.post.get(*c as usize) {
                self.emit(Ev(Key(*c, Up, None)));
            }
        }
    }

    fn unmask(&mut self, codes: &[u16]) {
        use super::super::Movement::*;

        for c in codes {
            let unmaskable = self.maps.mask.set(*c as usize, false);

            if unmaskable && self.maps.pre.get(*c as usize) && !self.maps.post.get(*c as usize)
            {
                self.emit(Ev(Key(*c, Down, None)));
            }
        }
    }
}

