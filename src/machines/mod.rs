#![allow(unused_variables)]
#![allow(unused_mut)]

use std::collections::vec_deque::*;
use bitmaps::{Bitmap,Bits};
use crate::{common::Update, common::Update::*, sink::*};
use self::key_maps::KeyMaps;

pub mod key_maps;
pub mod print_keys;
pub mod mode_machine;
pub mod big_machine;


pub trait Machine<TEv, TSink: Sink<TEv>> {
    fn run(&mut self, ev: TEv, sink: &mut TSink) -> ();
}

pub trait HasMaps {
    fn maps(&mut self) -> &mut KeyMaps;
}

pub trait CanMask<TEv, TSink: Sink<TEv>> : HasMaps {
    fn mask(&mut self, codes: &[u16], sink: &mut TSink);
    fn unmask(&mut self, codes: &[u16], sink: &mut TSink);
}

pub trait CanEmit<TEv, TSink: Sink<TEv>> {
    fn emit(&mut self, ev: TEv, sink: &mut TSink);
}






pub trait Runnable<TEv> {
    fn run<TSink: Sink<TEv>>(&mut self, ev: TEv, sink: &mut TSink) -> ();
}

pub struct Runner<TEv> {
    machines: Vec<Box<dyn Machine<TEv, VecDeque<TEv>>>>,
    buff1: VecDeque<TEv>,
    buff2: VecDeque<TEv>
}

impl<'a, TEv> Runner<TEv> {
    pub fn new(machines: Vec<Box<dyn Machine<TEv, VecDeque<TEv>>>>) -> Runner<TEv> {
        Runner {
            machines,
            buff1: VecDeque::new(),
            buff2: VecDeque::new()
        }
    }
}

impl<TEv: std::fmt::Debug> Runnable<TEv> for Runner<TEv> {
    
    fn run<TSink: Sink<TEv>>(&mut self, ev: TEv, sink: &mut TSink) -> () {
        let mut input = &mut self.buff1;
        let mut output = &mut self.buff2;

        input.push_back(ev);
        
        for m in self.machines.iter_mut() {
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
        count: u16
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
        let mut runner = Runner::new(vec!(
            Box::from(TestMachine { count: 3 }),
            Box::from(TestMachine { count: 4 }),
            Box::from(TestMachine { count: 5 }),
            Box::from(TestMachine { count: 6 }),
            Box::from(TestMachine { count: 7 }),
        ));

        let mut sink = VecDeque::new();
        runner.run((), &mut sink);
        runner.run((), &mut sink);
        
        assert_eq!(sink.len(), 2 * 3 * 4 * 5 * 6 * 7)
    }

}


use super::Movement::*;

impl<TRaw, TSink, TSelf> CanMask<Update<TRaw>, TSink>
    for TSelf
where
    TSelf: HasMaps,
    TSink: Sink<Update<TRaw>> {
    
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

            if unmaskable && self.maps().inp.get(*c as usize) && !self.maps().outp.get(*c as usize) {
                self.emit(Key(*c, Down, None), sink);
            }
        }
    }
}

impl<TRaw, TSink, TSelf> CanEmit<Update<TRaw>, TSink>
    for TSelf
where
    TSelf: HasMaps,
    TSink: Sink<Update<TRaw>> {

    fn emit(&mut self, ev: Update<TRaw>, sink: &mut TSink) {
        gather_map(&ev, &mut self.maps().outp);
        sink.emit(ev);
    }
}
