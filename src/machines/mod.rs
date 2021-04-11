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

type Sink<TEv> = VecDeque<TEv>;

pub trait Machine<TEv> {
    fn run(&mut self, ev: TEv, sink: &mut Sink<TEv>) -> ();
}

pub type MachineFac<TEv> = Box<dyn Fn() -> MachineRef<TEv>>;
pub type MachineRef<TEv> = Box<dyn Machine<TEv>>;

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

pub trait Runnable<TEv> {
    fn run(&mut self, ev: TEv, sink: &mut Sink<TEv>) -> ();
}

pub struct Runner<TEv, TLookup>
where
    TLookup: LookupFac<MachineRef<TEv>>,
{
    active: Vec<MachineRef<TEv>>,
    lookup: TLookup,
    buff1: VecDeque<TEv>,
    buff2: VecDeque<TEv>,
}

impl<'a, TEv, TLookup> Runner<TEv, TLookup>
where
    TLookup: LookupFac<MachineRef<TEv>>,
{
    pub fn new<TTags: IntoIterator<Item = &'static str>>(
        lookup: TLookup,
        initial: TTags,
    ) -> Runner<TEv, TLookup> {
        Runner {
            active: initial
                .into_iter()
                .flat_map(|s| lookup.find(s))
                .collect::<Vec<_>>(),
            lookup,
            buff1: VecDeque::new(),
            buff2: VecDeque::new(),
        }
    }
}

impl<TEv, TLookup> Runnable<TEv> for Runner<TEv, TLookup>
where
    TEv: std::fmt::Debug,
    TLookup: LookupFac<MachineRef<TEv>>,
{
    fn run(&mut self, ev: TEv, sink: &mut Sink<TEv>) -> () {
        let mut input = &mut self.buff1;
        let mut output = &mut self.buff2;

        input.push_back(ev);

        for m in self.active.iter_mut() {
            for e in input.drain(0..) {
                m.run(e, output);
            }

            input.extend(output.drain(0..));
        }

        sink.extend(input.drain(0..));
    }
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
            vec![
                Box::from(TestMachine { count: 3 }),
                Box::from(TestMachine { count: 4 }),
                Box::from(TestMachine { count: 5 }),
                Box::from(TestMachine { count: 6 }),
                Box::from(TestMachine { count: 7 }),
            ],
            [""],
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
