#![allow(unused_variables)]
#![allow(unused_mut)]

use self::key_maps::KeyMaps;
use crate::{common::Update, common::Update::*};
use bitmaps::{Bitmap, Bits};
use std::collections::{vec_deque::*, HashMap};

pub mod big_machine;
pub mod dynamic_machine;
pub mod key_maps;
pub mod lead_machine;
pub mod mode_machine;
pub mod print_keys;
pub mod runner;

type Sink<TEv> = VecDeque<TEv>;


pub trait Runnable<TEv> {
    fn run(&mut self, ev: TEv, sink: &mut Sink<TEv>) -> ();
}

pub type MachineFac<TEv> = Box<dyn Fn() -> MachineRef<TEv>>;
pub type MachineRef<TEv> = Box<dyn Runnable<TEv>>;

pub trait HasMaps {
    fn maps(&mut self) -> &mut KeyMaps;
}

pub trait CanMask<TEv>: HasMaps {
    fn mask(&mut self, codes: &[u16], sink: &mut Sink<TEv>);
    fn unmask(&mut self, codes: &[u16], sink: &mut Sink<TEv>);
}

pub trait CanEmit<TEv> {
    fn emit(&mut self, ev: TEv, sink: &mut Sink<TEv>);
}


pub fn gather_map<T, T2: Bits>(event: &Update<T>, map: &mut Bitmap<T2>) {
    use super::Movement::*;

    if let Key(code, movement, _) = event {
        match movement {
            Up => map.set(*code as usize, false),
            Down => map.set(*code as usize, true),
        };
    }
}

#[cfg(test)]
mod machines_tests {
    use super::*;
    use crate::fac;
    use velcro::hash_map;

    struct TestMachine {
        count: u16,
    }

    impl Machine<()> for TestMachine {
        fn run(&mut self, ev: (), sink: &mut Sink<()>) -> () {
            for i in 0..self.count {
                sink.extend(Some(()));
            }
        }
    }

    #[test]
    fn machines_run_in_sequence() {
        let mut runner = Runner::new(
            hash_map![
                "1": fac(|| TestMachine { count: 3 }),
                "2": fac(|| TestMachine { count: 4 }),
                "3": fac(|| TestMachine { count: 5 }),
                "4": fac(|| TestMachine { count: 6 }),
                "5": fac(|| TestMachine { count: 7 }),
            ],
            vec!["1", "2", "3", "4", "5"]
        );

        let mut sink = VecDeque::new();
        runner.run((), &mut sink);
        runner.run((), &mut sink);

        assert_eq!(sink.len(), 2 * 3 * 4 * 5 * 6 * 7)
    }
}

use super::Movement::*;

impl<TRaw, TSelf> CanMask<Update<TRaw>> for TSelf
where
    TSelf: HasMaps,
{
    fn mask(&mut self, codes: &[u16], sink: &mut Sink<Update<TRaw>>) {
        for c in codes {
            let maskable = !self.maps().mask.set(*c as usize, true);

            if maskable && self.maps().outp.get(*c as usize) {
                self.emit(Key(*c, Up, None), sink);
            }
        }
    }

    fn unmask(&mut self, codes: &[u16], sink: &mut Sink<Update<TRaw>>) {
        for c in codes {
            let unmaskable = self.maps().mask.set(*c as usize, false);

            if unmaskable && self.maps().inp.get(*c as usize) && !self.maps().outp.get(*c as usize)
            {
                self.emit(Key(*c, Down, None), sink);
            }
        }
    }
}

impl<TRaw, TSelf> CanEmit<Update<TRaw>> for TSelf
where
    TSelf: HasMaps,
{
    fn emit(&mut self, ev: Update<TRaw>, sink: &mut Sink<Update<TRaw>>) {
        gather_map(&ev, &mut self.maps().outp);
        sink.push_back(ev);
    }
}

pub trait LookupFac<T> {
    fn find(&self, tag: &'static str) -> Option<T>;
}

impl<TEv> LookupFac<MachineRef<TEv>> for HashMap<&str, MachineFac<TEv>> {
    fn find(&self, tag: &'static str) -> Option<MachineRef<TEv>> {
        self.get(tag).map(|f| f())
    }
}
