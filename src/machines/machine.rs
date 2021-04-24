use crate::common::Update;
use super::{CanEmit, CanMask, HasMaps, Runnable, Sink, key_maps::KeyMaps, runner::Ev};
use Ev::*;
use Update::*;

pub struct Ctx {
    pub maps: KeyMaps,
}

pub struct Machine<TBody> {
    pub context: Ctx,
    pub body: TBody
}

impl<TRaw, TBody> Runnable<(), Ev<Ctx,Update<TRaw>>> for Machine<TBody>
    where TBody : Runnable<Ctx, Ev<Ctx,Update<TRaw>>>
{
    fn run(&mut self, x: &mut (), ev: Ev<Ctx,Update<TRaw>>, sink: &mut Sink<Ev<Ctx,Update<TRaw>>>) -> () {
        if let Ev::Ev(up) = &ev {
            self.context.maps.track_in(up)
        }
        
        self.body.run(&mut self.context, ev, sink)

        //and to track out as well, involving looping through our own private buffer?
        //or give the body a special interface
    }
}

impl<TBody> Machine<TBody> {
    pub fn new(body: TBody) -> Machine<TBody> {
        Machine {
            context: Ctx {
                maps: KeyMaps::new(),
            },
            body
        }
    }
}

impl HasMaps for Ctx {
    fn maps(&self) -> &KeyMaps {
        &self.maps
    }
}


impl<TCtx,TRaw> CanEmit<Ev<TCtx,Update<TRaw>>> for Ctx {

    fn emit(&mut self, ev: Ev<TCtx,Update<TRaw>>, sink: &mut Sink<Ev<TCtx,Update<TRaw>>>) {
        match &ev {
            Ev(up) => {
                self.maps.track_out(&up);
                sink.push_back(ev)
            },
            _ => {}
        }
    }
}

impl<TCtx, TRaw> CanMask<Ev<TCtx,Update<TRaw>>> for Ctx
{

    fn mask(&mut self, codes: &[u16], sink: &mut Sink<Ev<TCtx,Update<TRaw>>>) {
        use super::super::Movement::*;

        for c in codes {
            let maskable = !self.maps.mask.set(*c as usize, true);

            if maskable && self.maps.outp.get(*c as usize) {
                self.emit(Ev(Key(*c, Up, None)), sink);
            }
        }
    }

    fn unmask(&mut self, codes: &[u16], sink: &mut Sink<Ev<TCtx,Update<TRaw>>>) {
        use super::super::Movement::*;

        for c in codes {
            let unmaskable = self.maps.mask.set(*c as usize, false);

            if unmaskable && self.maps.inp.get(*c as usize) && !self.maps.outp.get(*c as usize)
            {
                self.emit(Ev(Key(*c, Down, None)), sink);
            }
        }
    }
}

