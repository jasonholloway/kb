use std::fmt::Debug;
use crate::common::{CoreEv, Out, Movement::*};
use crate::Action::*;
use super::{Ctx, Runnable};


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

impl<TRaw> Runnable<TRaw,CoreEv,Out> for BigMachine
{
    fn run<'a>(&mut self, x: &mut Ctx<TRaw,Out>, inp: (Option<TRaw>,CoreEv)) -> ()
    {
        use Mode::*;
        use CoreEv::*;

        let (raw, ev) = inp;

        let prev_mode = self.mode;

        let next_mode = match (prev_mode, &ev) {
            (Root, Key(42, Down)) => Shift,
            (Root, Key(56, Down)) => Alt,

            (Shift, Key(42, Up)) => Root,
            (Shift, Key(56, Down)) => AltShift,

            (Alt, Key(56, Up)) => Root,
            (Alt, Key(42, Down)) => AltShift,

            (AltShift, Key(42, Up)) => Alt,
            (AltShift, Key(56, Up)) => Shift,
            (AltShift, Key(36, Down)) => AltShiftJ,
            (AltShift, Key(37, Down)) => AltShiftK,
            (AltShift, Key(57, Down)) => AltShiftSpace,

            (AltShiftSpace, Key(42, Up)) => Root,
            (AltShiftSpace, Key(56, Up)) => Root,
            (AltShiftSpace, Key(57, Up)) => AltShift,

            (AltShiftJ, Key(42, Up)) => Root,
            (AltShiftJ, Key(56, Up)) => Root,
            (AltShiftJ, Key(36, Up)) => AltShift,

            (AltShiftK, Key(42, Up)) => Root,
            (AltShiftK, Key(56, Up)) => Root,
            (AltShiftK, Key(37, Up)) => AltShift,

            _ => prev_mode
        };

        let action = match (prev_mode, next_mode, &ev) {

            (_, AltShiftSpace, Key(57, Down)) => {
                x.mask(&[42, 56]);
                x.key_down(28);
                Take
            },
            (AltShiftSpace, _, Key(57, Up)) => {
                x.key_up(28);
                x.unmask(&[42, 56]);
                Take
            },


            (_, AltShiftJ, Key(36, Down)) => {
                x.mask(&[42, 56]);
                x.key_down(108);
                Take
            },
            (AltShiftJ, _, Key(36, Up)) => {
                x.key_up(108);
                x.unmask(&[42, 56]);
                Take
            },


            (_, AltShiftK, Key(37, Down)) => {
                x.mask(&[42, 56]); //should do this on entry/exit rather than each keypress
                x.key_down(103);
                Take
            },
            (AltShiftK, _, Key(37, Up)) => {
                x.key_up(103);
                x.unmask(&[42, 56]);
                Take
            },

            _ => Skip
        };

        match action {
            Skip => {
                x.emit((raw, Out::Core(ev)))
            },
            Take => {}
        };

        if next_mode != self.mode {
            println!("\t\t{:?}", next_mode);
        }

        self.mode = next_mode;
    }
}

