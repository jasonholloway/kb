#![allow(unused_variables)]
#![allow(unused_mut)]

use self::key_maps::KeyMaps;
use crate::{common::Update, common::Update::*, sink::*};
use bitmaps::{Bitmap, Bits};
use std::collections::{vec_deque::*, HashMap};

pub mod dynamic_machine;
pub mod key_maps;
pub mod lead_machine;
pub mod mode_machine;
pub mod print_keys;

pub trait Machine<TEv, TSink: Sink<TEv>> {
    fn run(&mut self, ev: TEv, sink: &mut TSink) -> ();
}

pub type MachineRef<TEv> = Box<dyn Machine<TEv, VecDeque<TEv>>>;

pub type MachineFac<TEv> = Box<dyn Fn() -> MachineRef<TEv>>;

pub trait HasMaps {
    fn maps(&mut self) -> &mut KeyMaps;
}

pub trait CanMask<TEv, TSink: Sink<TEv>>: HasMaps {
    fn mask(&mut self, codes: &[u16], sink: &mut TSink);
    fn unmask(&mut self, codes: &[u16], sink: &mut TSink);
}

pub trait CanEmit<TEv, TSink: Sink<TEv>> {
    fn emit(&mut self, ev: TEv, sink: &mut TSink);
}

pub trait Runnable<TEv> {
    fn run<TSink: Sink<TEv>>(&mut self, ev: TEv, sink: &mut TSink) -> ();
}

pub struct Runner<TEv, TLookup>
where
    TLookup: LookupFac<MachineRef<TEv>>,
{
    lookup: TLookup,
    active: Vec<MachineRef<TEv>>,
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
        let active = initial
            .into_iter()
            .flat_map(|s| lookup.find(s))
            .collect::<Vec<_>>();

        Runner {
            lookup,
            active,
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
    fn run<TSink: Sink<TEv>>(&mut self, ev: TEv, sink: &mut TSink) -> () {
        let mut input = &mut self.buff1;
        let mut output = &mut self.buff2;

        input.push_back(ev);

        for m in self.active.iter_mut() {
            for e in input.drain(0..) {
                m.run(e, output);
            }

            input.extend(output.drain(0..));
        }

        sink.emit_many(input.drain(0..));
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

    impl<TSink: Sink<()>> Machine<(), TSink> for TestMachine {
        fn run(&mut self, ev: (), sink: &mut TSink) -> () {
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

impl<TRaw, TSink, TSelf> CanMask<Update<TRaw>, TSink> for TSelf
where
    TSelf: HasMaps,
    TSink: Sink<Update<TRaw>>,
{
    fn mask(&mut self, codes: &[u16], sink: &mut TSink) {
        for c in codes {
            let maskable = !self.maps().mask.set(*c as usize, true);

            if maskable && self.maps().outp.get(*c as usize) {
                self.emit(Key(*c, Up, None), sink);
            }
        }
    }

    fn unmask(&mut self, codes: &[u16], sink: &mut TSink) {
        for c in codes {
            let unmaskable = self.maps().mask.set(*c as usize, false);

            if unmaskable && self.maps().inp.get(*c as usize) && !self.maps().outp.get(*c as usize)
            {
                self.emit(Key(*c, Down, None), sink);
            }
        }
    }
}

impl<TRaw, TSink, TSelf> CanEmit<Update<TRaw>, TSink> for TSelf
where
    TSelf: HasMaps,
    TSink: Sink<Update<TRaw>>,
{
    fn emit(&mut self, ev: Update<TRaw>, sink: &mut TSink) {
        gather_map(&ev, &mut self.maps().outp);
        sink.emit(ev);
    }
}

pub trait LookupFac<T> {
    fn find(&self, tag: &'static str) -> Option<T>;
}

impl<T, TFn> LookupFac<T> for HashMap<&str, TFn>
where
    TFn: Fn() -> T,
{
    fn find(&self, tag: &'static str) -> Option<T> {
        self.get(tag).map(|f| f())
    }
}
