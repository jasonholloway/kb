use super::{CanEmit, Runnable, runner::Ev};

pub struct DynamicMachine {}

impl<TCtx, TUp> Runnable<TCtx, Ev<TCtx,TUp>> for DynamicMachine
    where TCtx: CanEmit<Ev<TCtx,TUp>>
{
    fn run(&mut self, x: &mut TCtx, ev: Ev<TCtx,TUp>) -> () {
        x.emit(ev);
    }
}
