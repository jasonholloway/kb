use crate::common::{CoreEv, Out};
use super::{Ctx, Runnable};

pub struct DynamicMachine {}

impl<TRaw> Runnable<TRaw,CoreEv,Out> for DynamicMachine
{
    fn run(&mut self, x: &mut Ctx<TRaw,Out>, (raw, ev): (Option<TRaw>,CoreEv)) -> ()
    {
        x.emit((raw, Out::Core(ev)));
    }
}
