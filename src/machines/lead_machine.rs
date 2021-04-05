use super::{gather_map, key_maps::KeyMaps, CanEmit, HasMaps, Machine};
use crate::common::{Act::*, Mode, Mode::*};
use crate::{
    common::{Movement::*, *},
    sink::Sink,
};
use std::fmt::Debug;

pub struct LeadMachine {
    mode: Mode,
    maps: KeyMaps,
}

impl LeadMachine {
    pub fn new() -> LeadMachine {
        LeadMachine {
            mode: Root,
            maps: KeyMaps::new(),
        }
    }
}

impl HasMaps for LeadMachine {
    fn maps(&mut self) -> &mut KeyMaps {
        &mut self.maps
    }
}

impl<TRaw, TSink> Machine<Update<TRaw>, TSink> for LeadMachine
where
    TRaw: Debug,
    TSink: Sink<Update<TRaw>>,
{
    fn run<'a>(&mut self, ev: Update<TRaw>, sink: &'a mut TSink) -> () {
        use Update::*;

        gather_map(&ev, &mut self.maps.inp);

        let next = match (self.mode, &ev) {
            (Root, On(Mode("MAltShift"))) => [Then(Mode("AltShift"))].iter(),

            (Mode("AltShift"), Off(Mode("MAltShift"))) => [Then(Root)].iter(),
            (Mode("AltShift"), Key(36, Down, _)) => [Then(Mode("AltShiftJ"))].iter(),
            (Mode("AltShift"), Key(37, Down, _)) => [Then(Mode("AltShiftK"))].iter(),
            (Mode("AltShift"), Key(57, Down, _)) => [Then(Mode("AltShiftSpace"))].iter(),

            (Mode("AltShiftJ"), Off(Mode("MAltShift"))) => [Emit(108, Up), Then(Root)].iter(),
            (Mode("AltShiftJ"), Key(36, Up, _)) => {
                [Drop, Emit(108, Up), Then(Mode("AltShift"))].iter()
            }
            (Mode("AltShiftJ"), Key(36, Down, _)) => [Drop, Emit(108, Down)].iter(),
            (Mode("AltShiftJ"), Key(37, Down, _)) => {
                [Emit(108, Up), Then(Mode("AltShiftK"))].iter()
            }
            (Mode("AltShiftJ"), Key(57, Down, _)) => {
                [Emit(108, Up), Then(Mode("AltShiftSpace"))].iter()
            }

            (Mode("AltShiftK"), Off(Mode("MAltShift"))) => [Then(Root)].iter(),
            (Mode("AltShiftK"), Key(37, Up, _)) => [Then(Mode("AltShift"))].iter(),
            (Mode("AltShiftK"), Key(36, Down, _)) => [Then(Mode("AltShiftJ"))].iter(),
            (Mode("AltShiftK"), Key(57, Down, _)) => [Then(Mode("AltShiftSpace"))].iter(),

            (Mode("AltShiftSpace"), Off(Mode("MAltShift"))) => [Then(Root)].iter(),
            (Mode("AltShiftSpace"), Key(57, Up, _)) => [Then(Mode("AltShift"))].iter(),
            (Mode("AltShiftSpace"), Key(36, Down, _)) => [Then(Mode("AltShiftJ"))].iter(),
            (Mode("AltShiftSpace"), Key(37, Down, _)) => [Then(Mode("AltShiftK"))].iter(),

            _ => [].iter(),
        };

        let mut reemit = true;

        for act in next {
            match act {
                Drop => {
                    reemit = false;
                }

                Emit(c, m) => {
                    self.emit(Key(*c, *m, None), sink);
                }

                Then(new_mode) => {
                    self.emit(Off(self.mode), sink);
                    self.emit(On(*new_mode), sink);
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
}
