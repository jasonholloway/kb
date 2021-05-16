use super::{Ctx, Runnable};
use crate::common::{Act::*, Mode, Mode::*};
use crate::common::{Movement::*,Out,CoreEv,Out::*,CoreEv::*,RunnerEv::*};

pub struct ModeMachine {
    mode: Mode,
}

impl ModeMachine {
    pub fn new() -> ModeMachine {
        ModeMachine {
            mode: Root,
        }
    }
}

impl<TRaw> Runnable<TRaw,CoreEv,Out> for ModeMachine
{
    fn run<'a>(&mut self, x: &mut Ctx<TRaw,Out>, (raw, ev): (Option<TRaw>,CoreEv))
    {
        let next = match (self.mode, &ev) {
            (Root, Key(42, Down)) => [Then(Mode("MShift"))].iter(),
            (Root, Key(56, Down)) => [Then(Mode("MAlt"))].iter(),

            (Mode("MShift"), Key(42, Up)) => [Then(Root)].iter(),
            (Mode("MShift"), Key(56, Down)) => [Then(Mode("MAltShift"))].iter(),

            (Mode("MAlt"), Key(56, Up)) => [Then(Root)].iter(),
            (Mode("MAlt"), Key(42, Down)) => [Then(Mode("MAltShift"))].iter(),

            (Mode("MAltShift"), Key(42, Up)) => [Then(Mode("MAlt"))].iter(),
            (Mode("MAltShift"), Key(56, Up)) => [Then(Mode("MShift"))].iter(),

            (Mode("MAltShift"), Key(36, Down)) => [Launch("AltShiftJ")].iter(),

            _ => [].iter(),
        };

        let mut reemit = true;

        for act in next {
            match act {
                Drop => {
                    reemit = false;
                }

                Emit(c, m) => {
                    x.emit((None, Core(Key(*c, *m))));
                }

                Then(new_mode) => {
                    x.emit((None, Core(Off(self.mode))));
                    x.emit((None, Core(On(*new_mode))));
                    self.mode = *new_mode;

                    println!("\t\t{:?}", *new_mode);
                }

                Mask(c) => {}
                Map(from, to) => {}
                Launch(name) => {
                    //should be registered such that a changing mode will cause it to die...
                    x.emit((None, Runner(Spawn((*name).to_string()))));
                }
            }
        }

        if reemit {
            x.emit((raw, Core(ev)));
        }
        
    }
}
