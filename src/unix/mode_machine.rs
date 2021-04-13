use super::{CanEmit, HasMaps, Runnable, Sink, gather_map, key_maps::KeyMaps, runner::{Ev,Ev::*}};
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

impl HasMaps for ModeMachine {
    fn maps(&mut self) -> &mut KeyMaps {
        &mut self.maps
    }
}

impl<TRaw> Runnable<Update<TRaw>> for ModeMachine
where
    TRaw: Debug,
{
    fn run<'a>(&mut self, ev: Ev<Update<TRaw>>, sink: &'a mut Sink<Ev<Update<TRaw>>>) {

        match &ev {
            Ev(up) => {
                gather_map(&up, &mut self.maps.inp);

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
                            self.emit(Ev(Key(*c, *m, None)), sink);
                        }

                        Then(new_mode) => {
                            self.emit(Ev(Off(self.mode)), sink);
                            self.emit(Ev(On(*new_mode)), sink);
                            self.mode = *new_mode;

                            println!("\t\t{:?}", *new_mode);
                        }

                        Mask(c) => {}
                        Map(from, to) => {}
                        Launch(name) => {}
                    }
                }

                if reemit {
                    self.emit(ev, sink);
                }
            }

            _ => {}
        }
    }
}
