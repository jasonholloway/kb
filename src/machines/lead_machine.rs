use super::{Ctx, Runnable};
use crate::common::{Act::*, CoreEv, Mode, Mode::*, Movement::*, Out};

pub struct LeadMachine {
    mode: Mode,
}

impl LeadMachine {
    pub fn new() -> LeadMachine {
        LeadMachine {
            mode: Root,
        }
    }
}

impl<TRaw> Runnable<TRaw,CoreEv,Out> for LeadMachine
{
    fn run<'a>(&mut self, x: &mut Ctx<TRaw,Out>, (raw, ev): (Option<TRaw>,CoreEv)) -> ()
    {
        use CoreEv::*;

        let next = match (self.mode, &ev) {
            (Root, On(Mode("MAltShift"))) => [Then(Mode("AltShift"))].iter(),

            (Mode("AltShift"), Off(Mode("MAltShift"))) => [Then(Root)].iter(),
            (Mode("AltShift"), Key(36, Down)) => [Then(Mode("AltShiftJ"))].iter(),
            (Mode("AltShift"), Key(37, Down)) => [Then(Mode("AltShiftK"))].iter(),
            (Mode("AltShift"), Key(57, Down)) => [Then(Mode("AltShiftSpace"))].iter(),

            (Mode("AltShiftJ"), Off(Mode("MAltShift"))) => [Emit(108, Up), Then(Root)].iter(),
            (Mode("AltShiftJ"), Key(36, Up)) => {
                [Drop, Emit(108, Up), Then(Mode("AltShift"))].iter()
            }
            (Mode("AltShiftJ"), Key(36, Down)) => [Drop, Emit(108, Down)].iter(),
            (Mode("AltShiftJ"), Key(37, Down)) => {
                [Emit(108, Up), Then(Mode("AltShiftK"))].iter()
            }
            (Mode("AltShiftJ"), Key(57, Down)) => {
                [Emit(108, Up), Then(Mode("AltShiftSpace"))].iter()
            }

            (Mode("AltShiftK"), Off(Mode("MAltShift"))) => [Then(Root)].iter(),
            (Mode("AltShiftK"), Key(37, Up)) => [Then(Mode("AltShift"))].iter(),
            (Mode("AltShiftK"), Key(36, Down)) => [Then(Mode("AltShiftJ"))].iter(),
            (Mode("AltShiftK"), Key(57, Down)) => [Then(Mode("AltShiftSpace"))].iter(),

            (Mode("AltShiftSpace"), Off(Mode("MAltShift"))) => [Then(Root)].iter(),
            (Mode("AltShiftSpace"), Key(57, Up)) => [Then(Mode("AltShift"))].iter(),
            (Mode("AltShiftSpace"), Key(36, Down)) => [Then(Mode("AltShiftJ"))].iter(),
            (Mode("AltShiftSpace"), Key(37, Down)) => [Then(Mode("AltShiftK"))].iter(),

            _ => [].iter(),
        };

        let mut reemit = true;

        for act in next {
            match act {
                Drop => {
                    reemit = false;
                }
                
                Emit(c, m) => {
                    x.emit((None, Out::Core(Key(*c, *m))));
                }

                Then(new_mode) => {
                    x.emit((None, Out::Core(Off(self.mode))));
                    x.emit((None, Out::Core(On(*new_mode))));
                    self.mode = *new_mode;

                    println!("\t\t{:?}", *new_mode);
                }

                Mask(c) => {}
                Map(from, to) => {}
                Launch(name) => {}
            }
        }

        if reemit {
            x.emit((raw, Out::Core(ev)));
        }
    }
}
