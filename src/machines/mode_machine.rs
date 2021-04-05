use super::{gather_map, key_maps::KeyMaps, CanEmit, HasMaps, Machine};
use crate::common::{Act::*, Mode, Mode::*};
use crate::{
    common::{Movement::*, *},
    sink::Sink,
};
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

impl<TRaw, TSink> Machine<Update<TRaw>, TSink> for ModeMachine
where
    TRaw: Debug,
    TSink: Sink<Update<TRaw>>,
{
    fn run<'a>(&mut self, ev: Update<TRaw>, sink: &'a mut TSink) -> () {
        use Update::*;

        gather_map(&ev, &mut self.maps.inp);

        let next = match (self.mode, &ev) {
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
