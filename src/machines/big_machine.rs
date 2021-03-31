use std::fmt::Debug;
use crate::{common::{Movement::*, *}, sink::Sink};
use Update::*;
use crate::Action::*;
use super::{CanEmit, CanMask, HasMaps, Machine, key_maps::KeyMaps, gather_map};


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

impl<TRaw, TSink> Machine<Update<TRaw>, TSink>
    for BigMachine
where
    TRaw: Debug,
    TSink: Sink<Update<TRaw>> {

    fn run<'a>(&mut self, ev: Update<TRaw>, sink: &'a mut TSink) -> () {
        use Mode::*;
        
        gather_map(&ev, &mut self.maps.inp);

        let prev_mode = self.mode;

        let next_mode = match (prev_mode, &ev) {
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

        let action = match (prev_mode, next_mode, &ev) {

            (_, AltShiftSpace, Key(57, Down, _)) => {
                self.mask(&[42, 56], sink);
                self.emit(Key(28, Down, None), sink);
                Take
            },
            (AltShiftSpace, _, Key(57, Up, _)) => {
                self.emit(Key(28, Up, None), sink);
                self.unmask(&[42, 56], sink);
                Take
            },


            (_, AltShiftJ, Key(36, Down, _)) => {
                self.mask(&[42, 56], sink);
                self.emit(Key(108, Down, None), sink);
                Take
            },
            (AltShiftJ, _, Key(36, Up, _)) => {
                self.emit(Key(108, Up, None), sink);
                self.unmask(&[42, 56], sink);
                Take
            },


            (_, AltShiftK, Key(37, Down, _)) => {
                self.mask(&[42, 56], sink); //should do this on entry/exit rather than each keypress
                self.emit(Key(103, Down, None), sink);
                Take
            },
            (AltShiftK, _, Key(37, Up, _)) => {
                self.emit(Key(103, Up, None), sink);
                self.unmask(&[42, 56], sink);
                Take
            },

            _ => Skip
        };

        match action {
            Skip => {
                if let Key(_, _, raw) = &ev {
                    match raw {
                        Some(_) => self.emit(ev, sink),
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
}




impl<TRaw, TSink, TSelf> CanMask<Update<TRaw>, TSink>
    for TSelf
where
    TSelf: HasMaps,
    TSink: Sink<Update<TRaw>> {
    
    fn mask(&mut self, codes: &[u16], sink: &mut TSink) {
        for c in codes {
            let maskable = !self.maps().mask.set(*c as usize, true);

            if maskable && self.maps().outp.get(*c as usize) {
                self.emit(Key(*c, Up, None), sink);
            }
        }
    }

    fn unmask(&mut self, codes: &[u16], sink: &mut TSink) {
        for c in codes {
            let unmaskable = self.maps().mask.set(*c as usize, false);

            if unmaskable && self.maps().inp.get(*c as usize) && !self.maps().outp.get(*c as usize) {
                self.emit(Key(*c, Down, None), sink);
            }
        }
    }
}

impl<TRaw, TSink, TSelf> CanEmit<Update<TRaw>, TSink>
    for TSelf
where
    TSelf: HasMaps,
    TSink: Sink<Update<TRaw>> {

    fn emit(&mut self, ev: Update<TRaw>, sink: &mut TSink) {
        gather_map(&ev, &mut self.maps().outp);
        sink.emit(ev);
    }
}
