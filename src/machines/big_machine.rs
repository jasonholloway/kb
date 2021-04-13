use std::fmt::Debug;
use crate::common::{Movement::*, Update, Update::*};
use crate::Action::*;
use super::{CanEmit, CanMask, HasMaps, Runnable, Sink, gather_map, key_maps::KeyMaps, runner::{Ev,Ev::*}};


#[derive(PartialEq, Copy, Clone, Debug)]
enum Mode {
    Root,
    Shift,
    Alt,
    AltShift,
    AltShiftSpace,
    AltShiftJ,
    AltShiftK
}


pub struct BigMachine {
    mode: Mode,
    maps: KeyMaps,
}

impl BigMachine {
    pub fn new() -> BigMachine {
        BigMachine {
            mode: Mode::Root,
            maps: KeyMaps::new(),
        }
    }
}

impl HasMaps for BigMachine {
    fn maps(&mut self) -> &mut KeyMaps {
        &mut self.maps
    }
}

impl<TRaw> Runnable<Update<TRaw>>
    for BigMachine
where
    TRaw: Debug,
{
    fn run<'a>(&mut self, ev: Ev<Update<TRaw>>, sink: &'a mut Sink<Ev<Update<TRaw>>>) -> () {
        use Mode::*;

        match ev {
            Ev(up) => {
                gather_map(&up, &mut self.maps.inp);

                let prev_mode = self.mode;

                let next_mode = match (prev_mode, &up) {
                    (Root, Key(42, Down, _)) => Shift,
                    (Root, Key(56, Down, _)) => Alt,

                    (Shift, Key(42, Up, _)) => Root,
                    (Shift, Key(56, Down, _)) => AltShift,

                    (Alt, Key(56, Up, _)) => Root,
                    (Alt, Key(42, Down, _)) => AltShift,

                    (AltShift, Key(42, Up, _)) => Alt,
                    (AltShift, Key(56, Up, _)) => Shift,
                    (AltShift, Key(36, Down, _)) => AltShiftJ,
                    (AltShift, Key(37, Down, _)) => AltShiftK,
                    (AltShift, Key(57, Down, _)) => AltShiftSpace,

                    (AltShiftSpace, Key(42, Up, _)) => Root,
                    (AltShiftSpace, Key(56, Up, _)) => Root,
                    (AltShiftSpace, Key(57, Up, _)) => AltShift,

                    (AltShiftJ, Key(42, Up, _)) => Root,
                    (AltShiftJ, Key(56, Up, _)) => Root,
                    (AltShiftJ, Key(36, Up, _)) => AltShift,

                    (AltShiftK, Key(42, Up, _)) => Root,
                    (AltShiftK, Key(56, Up, _)) => Root,
                    (AltShiftK, Key(37, Up, _)) => AltShift,

                    _ => prev_mode
                };

                let action = match (prev_mode, next_mode, &up) {

                    (_, AltShiftSpace, Key(57, Down, _)) => {
                        self.mask(&[42, 56], sink);
                        self.emit(Ev(Key(28, Down, None)), sink);
                        Take
                    },
                    (AltShiftSpace, _, Key(57, Up, _)) => {
                        self.emit(Ev(Key(28, Up, None)), sink);
                        self.unmask(&[42, 56], sink);
                        Take
                    },


                    (_, AltShiftJ, Key(36, Down, _)) => {
                        self.mask(&[42, 56], sink);
                        self.emit(Ev(Key(108, Down, None)), sink);
                        Take
                    },
                    (AltShiftJ, _, Key(36, Up, _)) => {
                        self.emit(Ev(Key(108, Up, None)), sink);
                        self.unmask(&[42, 56], sink);
                        Take
                    },


                    (_, AltShiftK, Key(37, Down, _)) => {
                        self.mask(&[42, 56], sink); //should do this on entry/exit rather than each keypress
                        self.emit(Ev(Key(103, Down, None)), sink);
                        Take
                    },
                    (AltShiftK, _, Key(37, Up, _)) => {
                        self.emit(Ev(Key(103, Up, None)), sink);
                        self.unmask(&[42, 56], sink);
                        Take
                    },

                    _ => Skip
                };

                match action {
                    Skip => {
                        if let Key(_, _, raw) = &up {
                            match raw {
                                Some(_) => self.emit(Ev(up), sink),
                                None => {}
                            }
                        }
                    },
                    Take => {}
                };

                if next_mode != self.mode {
                    println!("\t\t{:?}", next_mode);
                }

                self.mode = next_mode;
            }

            _ => {}
        }
        
    }
}

