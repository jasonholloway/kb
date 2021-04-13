use super::{Runnable, Sink, runner::Ev};

pub struct DynamicMachine {}

impl<TUp> Runnable<TUp> for DynamicMachine
{
    fn run(&mut self, ev: Ev<TUp>, sink: &mut Sink<Ev<TUp>>) -> () {
        sink.push_back(ev);
    }
}
