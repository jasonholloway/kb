use std::fmt::Debug;
use crate::{common::{Movement::*, *}, sink::Sink};
use super::{CanEmit, HasMaps, Machine, key_maps::KeyMaps, gather_map};



pub struct ModeMachine {
    mode: &'static str,
    maps: KeyMaps,
}

impl ModeMachine {
    pub fn new() -> ModeMachine {
        ModeMachine {
            mode: ".",
            maps: KeyMaps::new(),
        }
    }
}

impl HasMaps for ModeMachine {
    fn maps(&mut self) -> &mut KeyMaps {
        &mut self.maps
    }
}


pub enum Act {
    Mode(&'static str)
}



impl<TRaw, TSink> Machine<Update<TRaw>, TSink>
    for ModeMachine
where
    TRaw: Debug,
    TSink: Sink<Update<TRaw>> {

    fn run<'a>(&mut self, ev: Update<TRaw>, sink: &'a mut TSink) -> () {
        use Update::*;
        use Act::*;
        
        gather_map(&ev, &mut self.maps.inp);

        let prev_mode = self.mode;

        let next = match (prev_mode, &ev) {
            (".", Key(42, Down, _)) => [Mode("Shift")].iter(),
            (".", Key(56, Down, _)) => [Mode("Alt")].iter(),

            ("Shift", Key(42, Up, _)) => [Mode(".")].iter(),
            ("Shift", Key(56, Down, _)) => [Mode("AltShift")].iter(),

            ("Alt", Key(56, Up, _)) => [Mode(".")].iter(),
            ("Alt", Key(42, Down, _)) => [Mode("AltShift")].iter(),

            ("AltShift", Key(42, Up, _)) => [Mode("Alt")].iter(),
            ("AltShift", Key(56, Up, _)) => [Mode("Shift")].iter(),

            _ => [].iter()
        };

        for act in next {
            match act {
                Mode(m) => {
                    println!("\t\t{:?}", m);
                    self.mode = m;
                }
            }
        }

        //but we want to transmit ticks as well...

        if let Key(_, _, raw) = &ev {
            match raw {
                Some(_) => self.emit(ev, sink),
                None => {}
            }
        }
    }
}


