#![allow(unused_variables)]
#![allow(unused_mut)]


enum Event<TRaw> {
    Empty,
    Raw(TRaw)
}

use std::collections::vec_deque::*;
type Sink<TRaw> = VecDeque<Event<TRaw>>;


trait Machine<TRaw> {
    fn run(&mut self, ev: Event<TRaw>, sink: &mut Sink<TRaw>) -> ();
}






struct Runner<TRaw> {
    machines: Vec<(Box<dyn Machine<TRaw>>, VecDeque<Event<TRaw>>)>,
    buff1: VecDeque<Event<TRaw>>,
    buff2: VecDeque<Event<TRaw>>
}

impl<'a, TRaw> Runner<TRaw> {

    fn new(machines: Vec<(Box<dyn Machine<TRaw>>, VecDeque<Event<TRaw>>)>) -> Runner<TRaw> {
        Runner {
            machines,
            buff1: VecDeque::new(),
            buff2: VecDeque::new()
        }
    }
    
    fn run(&mut self, ev: Event<TRaw>, sink: &mut Sink<TRaw>) -> () {
        let mut buff1 = &mut self.buff1;
        let mut buff2 = &mut self.buff2;

        buff1.push_back(ev);
        
        for (m, _) in self.machines.iter_mut() {
            buff2.extend(buff1.drain(0..));
            
            for e in buff2.drain(0..) {
                m.run(e, buff1);
            }
        }

        sink.extend(buff1.drain(0..));
    }
}


#[cfg(test)]
mod machines_tests {
    use super::*;

    struct TestMachine {
        count: u16
    }

    impl Machine<()> for TestMachine {
        fn run(&mut self, ev: Event<()>, sink: &mut Sink<()>) -> () {
            for i in 0..self.count {
                sink.push_back(Event::Empty);
            }
        }
    }
    

    #[test]
    fn machines_run_in_sequence() {
        let mut m1 = TestMachine { count: 3 };
        let mut m2 = TestMachine { count: 4 };
        let mut m3 = TestMachine { count: 5 };
        let mut m4 = TestMachine { count: 6 };
        let mut m5 = TestMachine { count: 7 };

        let mut runner = Runner::new(vec!(
            (Box::from(m1), VecDeque::new()),
            (Box::from(m2), VecDeque::new()),
            (Box::from(m3), VecDeque::new()),
            (Box::from(m4), VecDeque::new()),
            (Box::from(m5), VecDeque::new())
        ));

        let mut sink = Sink::new();
        runner.run(Event::Empty, &mut sink);
        runner.run(Event::Empty, &mut sink);
        
        assert_eq!(sink.len(), 2 * 3 * 4 * 5 * 6 * 7)
    }

}
