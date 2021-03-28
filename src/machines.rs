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
    input_buff: VecDeque<Event<TRaw>>,
    machines: Vec<(Box<dyn Machine<TRaw>>, VecDeque<Event<TRaw>>)>
}

impl<'a, TRaw> Runner<TRaw> {

    fn new(machines: Vec<(Box<dyn Machine<TRaw>>, VecDeque<Event<TRaw>>)>) -> Runner<TRaw> {
        Runner {
            input_buff: VecDeque::new(),
            machines
        }
    }
    
    fn run(&mut self, ev: Event<TRaw>, sink: &mut Sink<TRaw>) -> () {
        let mut buff = &mut self.input_buff;

        buff.push_back(ev);
        
        for (m, buff2) in self.machines.iter_mut() {

            for e in buff.drain(0..) {
                m.run(e, buff2);
            }

            for e in buff2.drain(0..) {
                buff.push_back(e);
            }
        }

        for e in buff.drain(0..) {
            sink.push_back(e);
        }
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

        let mut runner = Runner::new(vec!(
            (Box::from(m1), VecDeque::new()),
            (Box::from(m2), VecDeque::new())
        ));

        let mut sink = Sink::new();
        runner.run(Event::Empty, &mut sink);
        runner.run(Event::Empty, &mut sink);
        
        assert_eq!(sink.len(), 24)
    }

}
