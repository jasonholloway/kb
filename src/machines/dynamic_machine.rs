use crate::sink::Sink;

use super::Machine;

pub struct DynamicMachine {}

impl<TEv, TSink> Machine<TEv, TSink> for DynamicMachine
where
    TSink: Sink<TEv>,
{
    fn run(&mut self, ev: TEv, sink: &mut TSink) -> () {
        sink.emit(ev);
    }
}
