#[cfg(test)]
#[path = "./runner_test.rs"]
mod runner_test;

use std::collections::VecDeque;

use super::{Ctx, RunRef, Runnable};
use crate::common::{MachineEv, MachineEv::*, Ev};

pub struct Runner<TRaw>
{
    pending: VecDeque<RunRef<TRaw,Ev,MachineEv>>,
    seen: VecDeque<RunRef<TRaw,Ev,MachineEv>>,
    buff1: VecDeque<(Option<TRaw>,Ev)>,
    buff2: VecDeque<(Option<TRaw>,Ev)>,
    context: Ctx<TRaw,MachineEv>
}

impl<TRaw> Runner<TRaw>
{
    pub fn new(active: Vec<RunRef<TRaw,Ev,MachineEv>>) -> Runner<TRaw> {
        Runner {
            pending: VecDeque::from(active),
            seen: VecDeque::new(),
            buff1: VecDeque::new(),
            buff2: VecDeque::new(),
            context: Ctx::new()
        }
    }
}

impl<TRaw> Runnable<TRaw,Ev,MachineEv> for Runner<TRaw>
{
    fn run(&mut self, x: &mut Ctx<TRaw,MachineEv>, ev: (Option<TRaw>,Ev))
    {
        let mut buff1 = &mut self.buff1;
        let mut buff2 = &mut self.buff2;
        let mut pending = &mut self.pending;
        let mut seen = &mut self.seen;

        if pending.is_empty() {
            return;
        }

        buff1.push_back(ev);

        while let Some(mut m) = pending.pop_front() {

            let mut requeue = true;

            for e1 in buff1.drain(0..) {
                m.inner.run(&mut self.context, e1);

                for (d, e2) in self.context.buff.drain(0..) {
                    // let e2 = emit;
                    match e2 {
                        MachineEv(ev) => {
                            buff2.push_back((d, ev));
                        },
                        Spawn(name) => {
                            pending.push_front(name);
                        },
                        Die => {
                            requeue = false;
                        },
                        _ => {}
                    }
                }
            }

            buff1.extend(buff2.drain(0..));

            if requeue {
                seen.push_back(m);
            }
        }

        pending.extend(seen.drain(0..));

        x.emit_many(buff1.drain(0..));
    }
}
