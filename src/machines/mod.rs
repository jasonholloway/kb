#![allow(unused_variables)]
#![allow(unused_mut)]

use std::collections::vec_deque::*;
use crate::sink::*;

pub mod big_machine;


pub trait Runnable<TEv> {
    fn run<TSink: Sink<TEv>>(&mut self, ev: TEv, sink: &mut TSink) -> ();
}

pub trait Machine<TEv, TSink: Sink<TEv>> {
    fn run(&mut self, ev: TEv, sink: &mut TSink) -> ();
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
