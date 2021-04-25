use crate::common::Ev;

use super::{Runnable, Ctx};

pub struct DynamicMachine {}

impl<TRaw> Runnable<Ev<TRaw>> for DynamicMachine
{
    fn run(&mut self, x: &mut Ctx<Ev<TRaw>>, ev: Ev<TRaw>) -> () {
        x.emit(ev);
    }
}
