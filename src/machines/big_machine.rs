use std::fmt::Debug;
use crate::common::{MachineEv, Ev, Ev::*, Movement::*};
use crate::Action::*;
use super::{Ctx, Runnable, Sink};


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
    mode: Mode
}


impl BigMachine {
    pub fn new() -> BigMachine {
        BigMachine {
            mode: Mode::Root,
        }
    }
}

impl<TRaw> Runnable<TRaw,Ev,MachineEv> for BigMachine
{
    fn run<'a>(&mut self, x: &mut Ctx<TRaw,MachineEv>, ev: (Option<TRaw>,Ev)) -> ()
    {
        use Mode::*;

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
                x.mask(&[42, 56]);
                x.emit(Key(28, Down, None));
                Take
            },
            (AltShiftSpace, _, Key(57, Up, _)) => {
                x.emit(Key(28, Up, None));
                x.unmask(&[42, 56]);
                Take
            },


            (_, AltShiftJ, Key(36, Down, _)) => {
                x.mask(&[42, 56]);
                x.emit(Key(108, Down, None));
                Take
            },
            (AltShiftJ, _, Key(36, Up, _)) => {
                x.emit(Key(108, Up, None));
                x.unmask(&[42, 56]);
                Take
            },


            (_, AltShiftK, Key(37, Down, _)) => {
                x.mask(&[42, 56]); //should do this on entry/exit rather than each keypress
                x.emit(Key(103, Down, None));
                Take
            },
            (AltShiftK, _, Key(37, Up, _)) => {
                x.emit(Key(103, Up, None));
                x.unmask(&[42, 56]);
                Take
            },

            _ => Skip
        };

        match action {
            Skip => {
                if let Key(_, _, raw) = &ev {
                    match raw {
                        Some(_) => x.emit(ev),
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

