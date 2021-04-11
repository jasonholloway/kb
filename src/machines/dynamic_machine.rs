use super::{Machine, Sink};

pub struct DynamicMachine {}

impl<TEv> Machine<TEv> for DynamicMachine
{
    fn run(&mut self, ev: TEv, sink: &mut Sink<TEv>) -> () {
        sink.push_back(ev);
    }
}
