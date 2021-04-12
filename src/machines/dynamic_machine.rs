use super::{Runnable, Sink};

pub struct DynamicMachine {}

impl<TEv> Runnable<TEv> for DynamicMachine
{
    fn run(&mut self, ev: TEv, sink: &mut Sink<TEv>) -> () {
        sink.push_back(ev);
    }
}
