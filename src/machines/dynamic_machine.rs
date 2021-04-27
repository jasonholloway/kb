use crate::common::Ev;

use super::{Runnable, Ctx};

pub struct DynamicMachine {}

impl<TRaw> Runnable<TRaw> for DynamicMachine
{
    fn run(&mut self, x: &mut Ctx<TRaw>, ev: Ev<TRaw>) -> () {
        x.emit(ev);
    }
}
