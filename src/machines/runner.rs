use std::collections::VecDeque;

use super::{RunRef, Runnable, Sink};

pub struct Runner<TEv>
{
    active: Vec<RunRef<TEv>>,
    buff1: VecDeque<TEv>,
    buff2: VecDeque<TEv>,
}

impl<TEv> Runner<TEv>
{
    pub fn new(active: Vec<RunRef<TEv>>) -> Runner<TEv> {
        Runner {
            active,
            buff1: VecDeque::new(),
            buff2: VecDeque::new(),
        }
    }
}

impl<TEv> Runnable<TEv> for Runner<TEv>
where
    TEv: std::fmt::Debug,
{
    fn run(&mut self, ev: TEv, sink: &mut Sink<TEv>) {
        let mut input = &mut self.buff1;
        let mut output = &mut self.buff2;

        input.push_back(ev);

        for m in self.active.iter_mut() {
            for e in input.drain(0..) {
                m.inner.run(e, output);
            }

            input.extend(output.drain(0..));
        }

        sink.extend(input.drain(0..));
    }
}
