use crate::common::{MachineEv, Ev};

use super::{Ctx, Runnable, Sink};

pub struct DynamicMachine {}

impl<TRaw> Runnable<TRaw,Ev,MachineEv> for DynamicMachine
{
    fn run(&mut self, x: &mut Ctx<TRaw,MachineEv>, ev: (Option<TRaw>,Ev)) -> ()
    {
        x.emit(ev);
    }
}
