use super::{Runnable, Sink, runner::Ev};

pub struct DynamicMachine {}

impl<TCtx, TUp> Runnable<TCtx, Ev<TCtx,TUp>> for DynamicMachine
{
    fn run(&mut self, x: &mut TCtx, ev: Ev<TCtx,TUp>, sink: &mut Sink<Ev<TCtx,TUp>>) -> () {
        sink.push_back(ev);
    }
}
