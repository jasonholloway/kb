use super::{CanEmit, Runnable, key_maps::KeyMaps, runner::{Ev,Ev::*}};
use crate::common::{Act::*, Mode, Mode::*};
use crate::common::{Movement::*, *};
use std::fmt::Debug;

pub struct ModeMachine {
    mode: Mode,
    maps: KeyMaps,
}

impl ModeMachine {
    pub fn new() -> ModeMachine {
        ModeMachine {
            mode: Root,
            maps: KeyMaps::new(),
        }
    }
}


impl<TRaw, TCtx> Runnable<TCtx, Ev<TCtx,Update<TRaw>>> for ModeMachine
where
    TCtx: CanEmit<Ev<TCtx,Update<TRaw>>>,
    TRaw: Debug,
{
    fn run<'a>(&mut self, x: &mut TCtx, ev: Ev<TCtx,Update<TRaw>>) {

        match &ev {
            Ev(up) => {
                self.maps.track_in(&up);

                use Update::*;

                let next = match (self.mode, &up) {
                    (Root, Key(42, Down, _)) => [Then(Mode("MShift"))].iter(),
                    (Root, Key(56, Down, _)) => [Then(Mode("MAlt"))].iter(),

                    (Mode("MShift"), Key(42, Up, _)) => [Then(Root)].iter(),
                    (Mode("MShift"), Key(56, Down, _)) => [Then(Mode("MAltShift"))].iter(),

                    (Mode("MAlt"), Key(56, Up, _)) => [Then(Root)].iter(),
                    (Mode("MAlt"), Key(42, Down, _)) => [Then(Mode("MAltShift"))].iter(),

                    (Mode("MAltShift"), Key(42, Up, _)) => [Then(Mode("MAlt"))].iter(),
                    (Mode("MAltShift"), Key(56, Up, _)) => [Then(Mode("MShift"))].iter(),

                    (Mode("MAltShift"), Key(37, Down, _)) => [Launch("AltShiftJ")].iter(),

                    _ => [].iter(),
                };

                let mut reemit = true;

                for act in next {
                    match act {
                        Drop => {
                            reemit = false;
                        }

                        Emit(c, m) => {
                            x.emit(Ev(Key(*c, *m, None)));
                        }

                        Then(new_mode) => {
                            x.emit(Ev(Off(self.mode)));
                            x.emit(Ev(On(*new_mode)));
                            self.mode = *new_mode;

                            println!("\t\t{:?}", *new_mode);
                        }

                        Mask(c) => {}
                        Map(from, to) => {}
                        Launch(name) => {}
                    }
                }

                if reemit {
                    x.emit(ev);
                }
            }

            _ => {}
        }
    }
}
