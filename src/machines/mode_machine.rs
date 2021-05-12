use super::{Ctx, RunRef, Runnable, Sink, machine::Machine};
use crate::common::{Act::*, Mode, Mode::*};
use crate::common::{MachineEv,Movement::*, Ev,Ev::*};

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

impl<TRaw> Runnable<TRaw,Ev,MachineEv> for ModeMachine
{
    fn run<'a>(&mut self, x: &mut Ctx<TRaw,MachineEv>, ev: (Option<TRaw>,Ev))
    {
        let next = match (self.mode, &ev) {
            (Root, Key(42, Down, _)) => [Then(Mode("MShift"))].iter(),
            (Root, Key(56, Down, _)) => [Then(Mode("MAlt"))].iter(),

            (Mode("MShift"), Key(42, Up, _)) => [Then(Root)].iter(),
            (Mode("MShift"), Key(56, Down, _)) => [Then(Mode("MAltShift"))].iter(),

            (Mode("MAlt"), Key(56, Up, _)) => [Then(Root)].iter(),
            (Mode("MAlt"), Key(42, Down, _)) => [Then(Mode("MAltShift"))].iter(),

            (Mode("MAltShift"), Key(42, Up, _)) => [Then(Mode("MAlt"))].iter(),
            (Mode("MAltShift"), Key(56, Up, _)) => [Then(Mode("MShift"))].iter(),

            (Mode("MAltShift"), Key(36, Down, _)) => [Launch("AltShiftJ")].iter(),

            _ => [].iter(),
        };

        let mut reemit = true;

        for act in next {
            match act {
                Drop => {
                    reemit = false;
                }

                MachineEv(c, m) => {
                    x.emit(Key(*c, *m, None));
                }

                Then(new_mode) => {
                    x.emit(Off(self.mode));
                    x.emit(On(*new_mode));
                    self.mode = *new_mode;

                    println!("\t\t{:?}", *new_mode);
                }

                Mask(c) => {}
                Map(from, to) => {}
                Launch(name) => {
                    //should be registered such that a changing mode will cause it to die...
                    x.spawn(RunRef::new("", Machine::new(ModeMachine::new())))
                }
            }
        }

        if reemit {
            x.emit(ev);
        }
        
    }
}
